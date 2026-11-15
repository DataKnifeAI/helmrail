# CRD sketch (provisional)

API group: `helmrail.dataknife.ai`  
Version: `v1alpha1`

Names and fields will move during M1–M2; this is the contract for review.

## AgentClass

Reusable preset. Instances should stay thin.

```yaml
apiVersion: helmrail.dataknife.ai/v1alpha1
kind: AgentClass
metadata:
  name: hermes-cluster-vllm
spec:
  runtime: hermes          # hermes | cursor | custom
  image:
    repository: nousresearch/hermes-agent   # adapter default if omitted
    tag: latest
  model:
    provider: vllm         # maps to Hermes custom/vllm
    baseUrl: http://vllm.inference.svc.cluster.local:8000/v1
    name: hermes3:8b
  sandboxRef: restricted
  dependencyRefs:
    - cluster-vllm-egress
  common:
    approvalsMode: smart   # typed common options (expand over time)
    gateway:
      apiServer: true
  # escape hatch
  raw: {}
```

## SandboxProfile

```yaml
apiVersion: helmrail.dataknife.ai/v1alpha1
kind: SandboxProfile
metadata:
  name: restricted
spec:
  networkPolicy:
    enabled: true
    allowDNS: true
    # default: no blanket 0.0.0.0/0 on 443 — declare intents
    egress:
      - name: https-apis
        ports: [{ port: 443, protocol: TCP }]
        toCIDRs: ["0.0.0.0/0"]
      - name: cluster-vllm
        ports: [{ port: 8000, protocol: TCP }]
        toNamespace: inference
  runtimeClassName: gvisor   # optional
  pod:
    seccompProfile: RuntimeDefault
    # main container hardening depends on runtime image realities
```

## DependencyBundle

Declares *what* to attach; operator materializes *how*.

```yaml
apiVersion: helmrail.dataknife.ai/v1alpha1
kind: DependencyBundle
metadata:
  name: cluster-vllm-egress
spec:
  # env-only dependency (no sidecar)
  env:
    - name: OPENAI_BASE_URL
      valueFrom:
        configMapKeyRef: { name: vllm-endpoints, key: url }
  # later: sidecars, volumes, externalService refs
```

## Agent

Long-lived member of the fleet.

```yaml
apiVersion: helmrail.dataknife.ai/v1alpha1
kind: Agent
metadata:
  name: devops-1
spec:
  classRef: hermes-cluster-vllm
  # sparse overrides only
  model:
    name: hermes3:8b
  suspend: false
```

## AgentFleet

Desired pool / director policy.

```yaml
apiVersion: helmrail.dataknife.ai/v1alpha1
kind: AgentFleet
metadata:
  name: devops
spec:
  classRef: hermes-cluster-vllm
  replicas: 3
  # later: autoscaling, max ephemeral workers, queues
```

## AgentTask

On-demand work → spawn or assign a worker.

```yaml
apiVersion: helmrail.dataknife.ai/v1alpha1
kind: AgentTask
metadata:
  name: review-pr-123
spec:
  classRef: hermes-cluster-vllm
  mode: ephemeral          # ephemeral | assign
  ttlSecondsAfterFinished: 600
  prompt: |
    Review PR #123 in org/repo and comment.
  # later: result sink, callbacks, parent agent ref
```

## Status conventions (for Fleetshade)

All Helmrail CRs should expose:

- `status.phase`: Pending | Running | Succeeded | Failed | Suspended
- `status.conditions[]` with `Available` / `Degraded`
- Labels: `helmrail.dataknife.ai/class`, `helmrail.dataknife.ai/fleet`, `helmrail.dataknife.ai/runtime`

## Mapping from hermes-agent-operator

| hermeum | Helmrail |
|---------|-----------|
| `HermesAgent` + `hermes.config.raw.model` | `AgentClass.spec.model` + hermes adapter |
| `security.networkPolicy` | `SandboxProfile` |
| `searxng` / `camofox` | `DependencyBundle` (sidecar kind) |
| `profiles` | still single-pod multi-persona — use multiple `Agent`s for true sub-agents |
| *(missing)* | `AgentFleet`, `AgentTask` |
