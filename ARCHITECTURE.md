# meta-signal-agent — Architecture

`meta-signal-agent` is the owner-only meta policy Signal contract for the
`agent` LLM-call component — the security-sensitive door in the `agent` triad
(`agent` runtime, `signal-agent` ordinary contract, `meta-signal-agent` meta
policy contract). Mutating authority lives here, not in the ordinary contract.

It is a schema-derived `WireContract` crate: `schema/lib.schema` is the source
of truth; `schema-rust-next`'s `ContractCrateBuild` emits the freshness-checked
`src/schema/lib.rs`. No engine traits, runtime, actors, or `tokio`.

## Scope — provider configuration + lifecycle, not harness policy

Per psyche intent (Spirit `f8k7`, `iucr`), `agent` makes provider HTTP API
calls in an OpenAI-compatible style. This contract carries the owner orders that
configure providers and drive the daemon's lifecycle. The earlier
spawn-agent / retire-agent / backend-policy / route-toggle framing (an
agent-harness shape) is discarded.

## The provider model is the load-bearing decision

A provider is a **generic OpenAI-compatible API**: a `ProviderConfiguration`
carries a `ProviderName`, an `EndpointUrl`, a default `ModelName`, and a typed
`SecretSource`. The secret source is a daemon-resolved backend reference
(`Environment`, `Gopass`, or `File`) — the secret value never crosses this wire
(`primary/skills/secrets.md`: an agent never sees a secret value). Adding
DeepSeek, MiMo, Kimi, GLM, or MiniMax is a `ConfigureProvider` message, never a
new variant or contract change.

## Contract surface

- `ConfigureProvider(ConfigureProvider)` — add or update a provider
  (endpoint + model + key handle). Reply: `ProviderConfigured(ProviderConfigured)`.
- `RetireProvider(RetireProvider)` — remove a provider by name. Reply:
  `ProviderRetired(ProviderRetired)`.
- `SetDefaultProvider(SetDefaultProvider)` — set the provider used when a
  `Call` names none. Reply: `DefaultProviderSet(DefaultProviderSet)`.
- `Start(Start)` / `Stop(Stop)` — lifecycle. Reply: `Started(Lifecycle)` /
  `Stopped(Lifecycle)`.

`OrderRejected(OrderRejection)` carries a closed `OrderRejectionReason`;
`RequestUnimplemented` is the skeleton-honesty reply.

## Records

- `ProviderConfiguration` — the "add a provider = configuration" record:
  `name`, `endpoint`, `default_model`, `secret_source`. No per-provider type.
- `SecretSource` — closed backend reference enum:
  `Environment(EnvironmentSecret)`, `Gopass(GopassSecret)`, `File(FileSecret)`.
- `ProviderName`, `EndpointUrl`, `ModelName`, `EnvironmentVariable`,
  `GopassPath`, `SecretFilePath` — string newtypes.
- `Lifecycle` over a closed `LifecycleState` (`Started` / `Stopped`).
- `OrderRejectionReason` (closed): `ProviderUnknown`,
  `ProviderAlreadyConfigured`, `EndpointInvalid`, `KeyHandleMissing`,
  `PolicyStoreUnavailable`.

## Invariants

- Meta mutating authority enters through this crate, not the ordinary contract.
- The secret value never appears: only the typed secret-source reference crosses
  the wire.
- Wire operations are contract-local meta verbs, not Sema class wrappers.
- Wire enums are closed. No `Unknown` escape hatch. No concrete provider name
  is a variant.
- Identifiers are full English words.
- Contract code contains no Kameo, Tokio, redb, sockets, process spawning, or
  daemon policy logic.
- Every operation and reply has an rkyv and NOTA round-trip witness.

## Code map

```text
schema/lib.schema        the source of truth (schema-rust-next grammar)
src/schema/lib.rs        freshness-checked schema-rust-next artifact (generated)
src/lib.rs               module entry + hand-written methods on emitted nouns
build.rs                 ContractCrateBuild -> WireContract emission
examples/canonical.nota  one canonical NOTA example per operation/reply
tests/round_trip.rs      rkyv frame, NOTA, and operation-kind witnesses
```

## See also

- `/home/li/primary/skills/component-triad.md` §"Two authority tiers".
- `/home/li/primary/skills/contract-repo.md`
- `/home/li/primary/skills/secrets.md` — key handles, never secret values.
- `../agent/ARCHITECTURE.md` — daemon-side provider registry and call path.
- `../signal-agent/ARCHITECTURE.md` — the ordinary call contract.
