# Architecture

## Problem

We want an **agent ecosystem**, not a single-agent installer:

- Deploy a **fleet** of agents
- Attach **relevant dependencies** (inference, search, browser, secrets, volumes)
- **Customize** where capabilities come from (cluster vLLM, Enodios host, cloud APIs)
- **Direct** work: a controller that can spin **sub-agents on demand**
- Prefer **good sandboxing** and **typed common options** on the CRD

Existing pieces cover slices of this:

| Component | Covers | Does not cover |
|-----------|--------|----------------|
| Enodios | Local Hermes ↔ vLLM on a GPU host | Fleet, k8s lifecycle, tasks |
| Dioptra | Observability / status board | Create or direct agents |
| hermes-agent-operator | One Hermes StatefulSet + config.raw | Fleet director, typed model/sandbox, ephemeral workers |

**Nauarchos** is the missing command layer.

## Design principles

1. **Two planes**
   - *Runtime plane* — how an agent pod runs (image, sandbox, mounts, NetworkPolicy)
   - *Command plane* — which agents exist, what work they take, when workers spawn
2. **Classes over snowflakes** — `AgentClass` holds 80% of config; instances stay thin
3. **Dependencies are resources** — declare bundles; the operator wires them (don’t hide sidecars only in raw YAML)
4. **Sandbox is a profile** — named, reusable, not ad-hoc annotations
5. **Escape hatch last** — typed fields first; `raw` / unstructured overlays for power users
6. **Runtime adapters** — Hermes is the first adapter; Cursor/Coder and custom images come later
7. **Observe via Dioptra** — Nauarchos emits status; Dioptra remains the board

## Logical components

```
                    ┌─────────────────────┐
                    │  AgentFleet / API   │  desired capacity, policies
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
        AgentTask         Agent (durable)   Agent (ephemeral)
              │                │                │
              └────────────────┼────────────────┘
                               ▼
                         AgentClass
                     (model, sandbox, deps)
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
      DependencyBundle   SandboxProfile    RuntimeAdapter
      (vLLM URL,         (NP, RuntimeClass, (hermes | cursor |
       SearXNG, …)        egress, …)         custom)
```

## Cluster shape (target)

- **Nauarchos operator** Deployment (Rust, `kube-rs`)
- CRDs in group `nauarchos.dataknife.ai` (provisional)
- Per-agent: Pod/Job/StatefulSet (adapter-chosen), ServiceAccount, NetworkPolicy, Secrets refs
- Optional: director Service (HTTP/gRPC) for non-GitOps task submission later

## Inference & dependencies

Customization of “where it comes from”:

```yaml
# conceptual — see docs/CRDS.md
model:
  provider: custom   # vllm | openai | anthropic | …
  baseUrl: http://vllm.inference.svc:8000/v1
  name: hermes3:8b
dependencies:
  - name: cluster-vllm      # DependencyBundle
  - name: searxng-sidecar
```

Bundles can mean: env injection only, sidecar, external Service reference, or PVC.

## Sandboxing model

Aligned with Hermes’ own guidance: **OS/cluster isolation is the real boundary**.

Nauarchos owns:

- NetworkPolicy templates (DNS + allowlisted CIDRs/namespaces/ports — including internal vLLM)
- RuntimeClass (gVisor / Kata when available)
- Dropped capabilities / non-root where the image allows
- Optional nested terminal backend for tool execution (later)

Hermes in-process approvals remain available via adapter config, but are not the primary control.

## What we deliberately defer

- Multi-cluster federation
- Full multi-tenant billing/quotas (start with namespace tenancy)
- Replacing Enodios or Dioptra
- Forking hermes-agent-operator on day one (adapt or shell out later if needed)

## Relation to a Rust rewrite of hermeum

Rewriting hermes-agent-operator in Rust is **out of scope**. If Hermes lifecycle is painful, add a thin `runtime: hermes` adapter that either:

1. Creates child `HermesAgent` CRs (reuse hermeum), or
2. Owns a minimal Hermes Pod spec itself

Prefer (1) until we outgrow it.
