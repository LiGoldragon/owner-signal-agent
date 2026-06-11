# meta-signal-agent agent notes

Read `~/primary/AGENTS.md` first, then this repo's `INTENT.md` and
`ARCHITECTURE.md`. This repository is the owner-only meta wire contract for the
`agent` LLM-call component: provider configuration and lifecycle. Keep daemon
behaviour, actors, storage, process spawning, the provider registry, and text
parsing out of this crate.

`agent` makes OpenAI-compatible provider HTTP API calls; it is NOT an agent
harness (psyche Spirit `iucr`, `f8k7`). A provider is a generic
OpenAI-compatible API (endpoint + model + typed secret-source reference);
adding one is a `ConfigureProvider` message, never a contract change. The
secret source is an Environment, Gopass, or File reference — the secret value
never crosses the wire.

Before changing the contract surface, read:

- `~/primary/skills/component-triad.md` §"Two authority tiers"
- `~/primary/skills/contract-repo.md`
- `~/primary/skills/secrets.md`
- `~/primary/skills/naming.md`

Edit `schema/lib.schema` and regenerate
(`META_SIGNAL_AGENT_UPDATE_SCHEMA_ARTIFACTS=1 cargo build`); never hand-edit
`src/schema/lib.rs`.
