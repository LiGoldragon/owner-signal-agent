# owner-signal-persona-agent — architecture

*OwnerSignal contract for privileged Persona agent policy commands.*

## Role

`owner-signal-persona-agent` is the owner-only Signal surface for
`persona-agent`. It carries orchestrate-to-agent authority orders that spawn and
retire agent runs, set lane default-backend policy, mutate backend
configuration, and toggle per-lane routing through the new agent front door.

The ordinary router-facing contract lives in `signal-persona-agent`. At this
repo's creation that ordinary contract is not present, so this crate carries
local compatible definitions for the shared nouns that both contracts need:
`AgentIdentifier`, `AgentBackend`, `LaneName`, and `BackendConfiguration`. When
`signal-persona-agent` lands, those nouns are factored to a shared surface or
one contract re-exports the other according to the then-current design.

This repo implements bead `primary-gvgj.2` from the agent-component wave in
`reports/designer/309-design-agent-component-abstraction.md` and is sequenced by
`reports/designer/310-meta-overhaul-booking-roadmap.md` §5 and §9.

## Boundary

This crate owns the typed wire vocabulary for owner-only policy. It does not
own daemon actors, backend process supervision, redb tables, socket listeners,
routing decisions, or lowering to Sema effects. Runtime interpretation belongs
in `persona-agent`.

## Contract surface

The crate declares one `signal_channel!` at the crate root:

```rust
signal_channel! {
    channel Owner {
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

- Owner-only mutating authority enters through this crate, not the ordinary
  agent contract.
- Wire operations are contract-local owner verbs, not Sema class wrappers.
- Wire enums are closed. There is no `Unknown` escape hatch.
- Shared nouns remain compatible with the future ordinary contract.
- Round-trip tests cover rkyv frame encoding and NOTA text encoding.
- Contract code contains no Kameo, Tokio, redb, sockets, process spawning, or
  daemon policy logic.

## Code map

```text
src/lib.rs              — owner request/reply records and signal_channel! invocation
examples/canonical.nota — canonical NOTA examples for public owner values
tests/round_trip.rs     — rkyv frame, NOTA, and operation-kind witnesses
```
