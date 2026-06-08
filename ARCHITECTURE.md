# meta-signal-agent — architecture

*Meta signal contract for privileged agent policy commands.*

## Role

`meta-signal-agent` is the meta Signal surface for
`agent`. It carries orchestrate-to-agent authority orders that spawn and
retire agent runs, set lane default-backend policy, mutate backend
configuration, and toggle per-lane routing through the new agent front door.

The ordinary router-facing contract lives in `signal-agent`. This crate carries
local compatible definitions for the shared nouns that both contracts need:
`AgentIdentifier`, `AgentBackend`, `LaneName`, and `BackendConfiguration`.
Those nouns stay aligned until a shared surface or re-export decision lands.

This repo implements bead `primary-gvgj.2` from the agent-component wave in
`reports/designer/309-design-agent-component-abstraction.md` and is sequenced by
`reports/designer/310-meta-overhaul-booking-roadmap.md` §5 and §9.

## Boundary

This crate owns the typed wire vocabulary for meta policy. It does not
own daemon actors, backend process supervision, redb tables, socket listeners,
routing decisions, or lowering to Sema effects. Runtime interpretation belongs
in `agent`.

## Contract surface

The crate declares one `signal_channel!` at the crate root:

```rust
signal_channel! {
    channel MetaAgent {
        operation SpawnAgent(SpawnAgent),
        operation RetireAgent(RetireAgent),
        operation SetBackendPolicy(SetBackendPolicy),
        operation MutateBackendConfiguration(MutateBackendConfiguration),
        operation RouteThroughAgent(RouteThroughAgent),
    }
    reply Reply { ... }
    observable { ... }
}
```

The macro emits `Operation`, `OperationKind`, `Reply`, `Frame`, `FrameBody`,
`Request`, `RequestBuilder`, observer stream types, and the NOTA codec for the
public payloads.

## Records

The local compatibility nouns are deliberately small:

- `AgentIdentifier` names one agent run.
- `AgentBackend` is the closed backend set: `Claude`, `Codex`, `Gemini`, `Pi`,
  `OpenCode`, `Fixture`.
- `LaneName` names the lane whose default backend or route is changing.
- `BackendConfiguration` carries the backend endpoint, availability, optional
  model, thinking level, and extension set.

`MutateBackendConfiguration` uses the full English word because workspace
naming forbids the abbreviated `Config` form.

## Invariants

- Meta mutating authority enters through this crate, not the ordinary
  agent contract.
- Wire operations are contract-local meta verbs, not Sema class wrappers.
- Wire enums are closed. There is no `Unknown` escape hatch.
- Shared nouns remain compatible with the ordinary contract.
- Round-trip tests cover rkyv frame encoding and NOTA text encoding.
- Contract code contains no Kameo, Tokio, redb, sockets, process spawning, or
  daemon policy logic.

## Code map

```text
src/lib.rs              — meta request/reply records and signal_channel! invocation
examples/canonical.nota — canonical NOTA examples for public meta values
tests/round_trip.rs     — rkyv frame, NOTA, and operation-kind witnesses
```
