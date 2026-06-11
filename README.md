# meta-signal-agent

Owner-only meta policy Signal contract for the `agent` LLM-call component.

This crate owns the privileged orders that configure LLM providers and drive the
daemon's lifecycle: `ConfigureProvider` (endpoint + model + typed secret source),
`RetireProvider`, `SetDefaultProvider`, `Start`, `Stop`. Ordinary peer-callable
call traffic belongs in `signal-agent`.

A provider is a generic OpenAI-compatible API; adding DeepSeek, MiMo, Kimi, GLM,
or MiniMax is a `ConfigureProvider` message, never a contract change. The
secret source is a daemon-resolved backend reference — the secret value never
crosses the wire. `schema/lib.schema` is the source of truth; read
`ARCHITECTURE.md` and `INTENT.md`.
