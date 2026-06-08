# meta-signal-agent agent notes

Read `~/primary/AGENTS.md` first. This repository is the meta typed
contract for privileged `agent` policy commands. Keep daemon behavior,
actors, storage, process spawning, backend routing logic, and text parsing out
of this crate.

Before changing the contract surface, read:

- `~/primary/skills/component-triad.md`
- `~/primary/skills/contract-repo.md`
- `~/primary/skills/naming.md`
- `~/primary/reports/designer/309-design-agent-component-abstraction.md`
- `~/primary/reports/designer/310-meta-overhaul-booking-roadmap.md` §5 and §9

The ordinary `signal-agent` contract exists beside this meta contract. Shared
nouns in this crate (`AgentIdentifier`, `AgentBackend`, `LaneName`, and
`BackendConfiguration`) stay compatible with that ordinary surface; factor them
to a shared meta/ordinary surface when the contract shape calls for it.
