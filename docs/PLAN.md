# Plan & milestones

## Codename

**Helmrail** — control plane rail for the agent fleet.

Fits DataKnife Greek naming:

- **Coldberth** — road / local Hermes+vLLM
- **Fleetshade** — survey / observe the fleet
- **Helmrail** — command / deploy and direct the fleet

## Why Rust (and when not to)

**Use Rust** for this project:

- Learning goal is explicit and valid
- `kube-rs` is production-capable for CRD controllers
- Long-lived operator benefits from predictable memory behavior
- CRD types + async reconcile map cleanly to Rust

**Do not** choose Rust expecting it to unlock fleet features — those are product design. If delivery pressure ever outweighs learning, a Go sidecar/adapter is fine; the CRD API stays the contract.

## Success criteria (north star)

1. Apply an `AgentClass` + `AgentFleet` and get N sandboxed agent pods
2. Common options (model endpoint, sandbox profile, dependency bundles) are **typed CRD fields**
3. Submit an `AgentTask` and get an **ephemeral worker** that runs and terminates
4. Point model traffic at **in-cluster or network vLLM** without editing opaque blobs
5. Fleetshade can read Helmrail status (labels/conditions) without a special protocol v1

## Milestones

### M0 — Bootstrap (this PR / repo)

- [x] Repository, Apache-2.0 license, README
- [x] Architecture + CRD sketch
- [x] Rust workspace skeleton (`helmrail-crd`, `helmrail-operator`)
- [x] Makefile + CI (lint/test workflows, `make ci`)

### M1 — CRDs + empty reconciler

- [ ] Generate/install CRDs (`AgentClass`, `Agent`, `SandboxProfile`, `DependencyBundle`)
- [ ] Operator watches resources; status `Phase=Pending` stub
- [ ] `cargo test` + kind/k3d smoke (apply CRDs)

### M2 — Durable agent from class

- [ ] `Agent` references `AgentClass`
- [ ] Create ServiceAccount + NetworkPolicy from `SandboxProfile`
- [ ] Create Pod/Deployment for `runtime: custom` (echo/sleep image first)
- [ ] Model env injection from typed `model.*` fields

### M3 — Hermes adapter (reuse, don’t rewrite)

- [ ] `runtime: hermes` creates/owns a child `HermesAgent` **or** minimal Hermes Pod
- [ ] Wire `baseUrl` to cluster vLLM Service
- [ ] Document Coldberth as out-of-cluster inference option

### M4 — Fleet + on-demand tasks

- [ ] `AgentFleet` desired replicas / pool
- [ ] `AgentTask` spawns ephemeral Job/Pod worker from class
- [ ] TTL, retries, result status on the task
- [ ] Basic RBAC so a director SA can create tasks only

### M5 — Dependency bundles + UX

- [ ] Bundles: secret/env, sidecar (SearXNG-like), external service ref
- [ ] Example presets: `hermes-vllm-cluster`, `hermes-cloud-api`
- [ ] Helm chart or kustomize install
- [ ] Fleetshade integration notes (label contract)

### Later

- Cursor/Coder workspace adapter
- Admission validation webhook
- gVisor/Kata RuntimeClass profiles
- Director HTTP API (non-GitOps task submit)
- Multi-namespace tenancy controls

## Non-goals (near term)

- Rust rewrite of hermeum
- Replacing Coldberth install scripts
- Building Fleetshade inside this repo
- Perfect multi-tenant SaaS

## Review checklist

When reviewing this bootstrap, confirm:

1. Codename and positioning vs Coldberth / Fleetshade / hermeum
2. CRD split (class / agent / fleet / task / sandbox / deps)
3. Hermes via adapter vs owning Pods day one
4. Rust learning pace vs M1–M2 scope
5. API group name: `helmrail.dataknife.ai` (provisional)
