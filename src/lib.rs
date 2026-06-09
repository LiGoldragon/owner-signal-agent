//! Schema-derived meta policy Signal contract for the `agent` LLM-call component.
//!
//! This is the owner-only authority surface. The deploy/bootstrap tool (and the
//! gated-Spirit owner) configures providers and drives lifecycle here; ordinary
//! peer-callable call traffic belongs in `signal-agent`.
//!
//! Provider model (psyche f8k7): a provider is a GENERIC OpenAI-compatible API —
//! `ProviderConfiguration` carries an endpoint URL, a default model, and an
//! API-key HANDLE (an environment-variable name the daemon resolves; the secret
//! value never crosses this wire). Adding DeepSeek, MiMo, Kimi, GLM, or MiniMax
//! is a `ConfigureProvider` message, never a contract change.
//!
//! `schema/lib.schema` is the source of truth. The checked-in `src/schema/lib.rs`
//! is a freshness-checked schema-rust-next artifact, not handwritten vocabulary.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

impl Input {
    pub fn operation_kind(&self) -> OperationKind {
        match self {
            Self::ConfigureProvider(_) => OperationKind::ConfigureProvider,
            Self::RetireProvider(_) => OperationKind::RetireProvider,
            Self::SetDefaultProvider(_) => OperationKind::SetDefaultProvider,
            Self::Start(_) => OperationKind::Start,
            Self::Stop(_) => OperationKind::Stop,
        }
    }
}
