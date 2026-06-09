# meta-signal-agent

Owner-only meta policy Signal contract for the `agent` LLM-call component.

This crate owns the privileged orders that configure LLM providers and drive the
daemon's lifecycle: `ConfigureProvider` (endpoint + model + key handle),
`RetireProvider`, `SetDefaultProvider`, `Start`, `Stop`. Ordinary peer-callable
call traffic belongs in `signal-agent`.

A provider is a generic OpenAI-compatible API; adding DeepSeek, MiMo, Kimi, GLM,
or MiniMax is a `ConfigureProvider` message, never a contract change. The
API-key handle is an environment-variable name the daemon resolves — the secret
value never crosses the wire. `schema/lib.schema` is the source of truth; read
`ARCHITECTURE.md` and `INTENT.md`.
