# INTENT — meta-signal-agent

*The owner-only meta policy wire contract for the `agent` LLM-call component.
Companion to `ARCHITECTURE.md` and `Cargo.toml`.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `meta-signal-agent` contract.
Workspace-shape intent stays in `primary/INTENT.md`. Component daemon intent
stays in `agent/INTENT.md`. The ordinary call contract stays in
`signal-agent/INTENT.md`.

## Why this repo exists

`meta-signal-agent` is the **meta authority contract** for the `agent`
component — the security-sensitive door. It carries the owner orders that
configure LLM providers and drive the daemon's lifecycle. The three-repo split
is not bureaucracy: meta operations live in this distinct repo so a
security-sensitive edit is obvious from which repo it lands in, and clients that
do not need owner authority do not depend on it at all
(`primary/skills/component-triad.md` §"Why the contract is a separate repo").

## Stated psyche intent

The psyche authorized this build and made two shaping decisions, captured in
Spirit:

- The `agent` component is an LLM-API-call component, not an agent-harness front
  door; harness backends are deferred. (Spirit `iucr`, Decision.)
- LLM providers are modeled as a generic OpenAI-compatible API with endpoint,
  model, and key as configuration, so adding a provider is configuration rather
  than a contract change. (Spirit `f8k7`, Decision.)

The second decision is this contract's whole reason for existing in its current
shape: `ConfigureProvider` carries a `ProviderConfiguration` (endpoint + default
model + key handle). Adding DeepSeek, MiMo, Kimi, GLM, or MiniMax is a message,
never a new type. DeepSeek and MiMo are the first two configured providers.

## The channel shape

- **Requests:** `ConfigureProvider`, `RetireProvider`, `SetDefaultProvider`,
  `Start`, `Stop`.
- **Replies:** `ProviderConfigured`, `ProviderRetired`, `DefaultProviderSet`,
  `Started(Lifecycle)`, `Stopped(Lifecycle)`, `OrderRejected(OrderRejection)`,
  `RequestUnimplemented`.

## Constraints

- Meta mutating authority enters through this crate, not the ordinary
  `signal-agent` contract.
- The API-key handle is an environment-variable name the daemon resolves; the
  secret value never crosses this wire (`primary/skills/secrets.md`).
- Wire enums are closed. No `Unknown` escape hatch. No concrete provider name is
  a variant.
- Identifiers are full English words.
- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses. No Kameo, Tokio, redb, sockets, process spawning, or daemon policy
  logic.

## Non-ownership

This crate does not own:

- the `agent` daemon runtime, the provider registry, or the HTTPS call;
- redb tables or any durable state;
- socket binding or transport;
- ordinary peer-callable call traffic (lives in `signal-agent`).

## What this is NOT

- Not an agent-spawn / backend-policy / route-toggle contract. The earlier
  harness-shaped framing is discarded; this is provider configuration and
  lifecycle only.

## See also

- `ARCHITECTURE.md` — contract surface, records, closed-enum discipline.
- `../agent/INTENT.md` — daemon-side intent (provider registry, the call path).
- `../signal-agent/INTENT.md` — ordinary call contract.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
