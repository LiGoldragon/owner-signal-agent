# INTENT — owner-signal-agent

*The owner-only wire contract for privileged `agent` policy. Defines the
typed request/reply channel that `orchestrate` uses to spawn and retire
agent runs, set lane default-backend policy, mutate backend configuration,
and toggle per-lane routing through the agent front door.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this owner-only `owner-signal-agent`
contract. Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `agent/INTENT.md`. The ordinary router-facing
contract stays in `signal-agent/INTENT.md`.

## Why this repo exists

`owner-signal-agent` is the **owner-only authority contract** for the `agent`
component. It carries orchestrate-to-agent authority orders. Ordinary
router-facing operations stay in `signal-agent`; runtime actors, backend process
supervision, sockets, redb tables, and routing decisions live in `agent`.

The three-repo split is not bureaucracy: owner-only operations live in this
distinct repo so a security-sensitive edit is obvious from which repo it lands
in, and clients that do not need owner authority do not depend on it at all
(per `primary/skills/component-triad.md` §"Why the contract is a separate repo").

## The channel shape

The owner channel (`signal_channel! { channel Owner { ... } }`) carries:

- **Requests:** `SpawnAgent`, `RetireAgent`, `SetBackendPolicy`,
  `MutateBackendConfiguration`, `RouteThroughAgent`
- **Replies:** the closed `Reply` enumeration matching those operations
- **Observations:** the observer stream types emitted by `signal_channel!`

The shared nouns are deliberately small and stay locally compatible with the
ordinary contract: `AgentIdentifier` names one agent run; `AgentBackend` is the
closed backend set (`Claude`, `Codex`, `Gemini`, `Pi`, `OpenCode`, `Fixture`);
`LaneName` names the lane whose default backend or route is changing; and
`BackendConfiguration` carries the backend endpoint, availability, optional
model, thinking level, and extension set.

## Constraints

- Owner-only mutating authority enters through this crate, not the ordinary
  `signal-agent` contract.
- Wire operations are contract-local owner verbs, not Sema class wrappers. There
  is no public `Mutate` / `Match` tag on the wire.
- Wire enums are closed. No `Unknown` escape hatch.
- Every identifier is spelled as a full English word —
  `MutateBackendConfiguration`, never the abbreviated `Config` form.
- Shared nouns stay compatible with the ordinary `signal-agent` contract until a
  shared-surface or re-export decision lands.
- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses. No Kameo, Tokio, redb, sockets, process spawning, or daemon policy
  logic.
- Every operation and reply round-trips through both rkyv frames and NOTA text.

## Non-ownership

This crate does not own:

- the `agent` daemon runtime, actors, or backend process supervision;
- redb tables or any durable state;
- socket binding, transport, or routing decisions;
- lowering from contract operations to Sema effects;
- ordinary router-facing traffic (lives in `signal-agent`).

## See also

- `ARCHITECTURE.md` — contract surface, records, and closed-enum discipline.
- `../agent/INTENT.md` — daemon-side intent (backends, supervision, routing).
- `../signal-agent/INTENT.md` — ordinary router-facing contract.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
