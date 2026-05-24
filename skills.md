# skills — owner-signal-agent

Read this before editing the owner-only agent contract.

## Required context

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/naming.md`
- `~/primary/skills/nota-design.md`
- `~/primary/skills/rust-discipline.md`
- this repo's `ARCHITECTURE.md`
- `~/primary/reports/designer/309-design-agent-component-abstraction.md`
- `~/primary/reports/designer/310-meta-overhaul-booking-roadmap.md` §5 and §9

## Boundary

This crate owns privileged owner-to-agent policy vocabulary. It has no runtime,
no actors, no sockets, no storage, and no backend process logic.

## Invariants

- Agent lifecycle and backend policy orders live here, not in the ordinary
  `signal-agent` contract.
- Every request variant declares a contract-local Signal root verb through
  `signal_channel!`.
- Shared agent nouns stay aligned with the ordinary contract.
- Runtime interpretation stays in `agent`.
