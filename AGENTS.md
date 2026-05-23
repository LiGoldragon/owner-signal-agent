# owner-signal-persona-agent agent notes

Read `~/primary/AGENTS.md` first. This repository is the owner-only typed
contract for privileged `persona-agent` policy commands. Keep daemon behavior,
actors, storage, process spawning, backend routing logic, and text parsing out
of this crate.

Before changing the contract surface, read:

- `~/primary/skills/component-triad.md`
- `~/primary/skills/contract-repo.md`
- `~/primary/skills/naming.md`
- `~/primary/reports/designer/309-design-agent-component-abstraction.md`
- `~/primary/reports/designer/310-meta-overhaul-booking-roadmap.md` §5 and §9

The ordinary `signal-persona-agent` contract is not present yet. Shared nouns
in this crate (`AgentIdentifier`, `AgentBackend`, `LaneName`, and
`BackendConfiguration`) are compatibility placeholders for that future ordinary
contract; factor them to the shared owner/ordinary surface when
`signal-persona-agent` lands.
