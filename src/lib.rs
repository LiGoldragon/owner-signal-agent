//! OwnerSignal contract for privileged `agent` policy commands.
//!
//! Ordinary router-facing agent traffic belongs in `signal-agent`.
//! This crate carries owner-only orders for spawning and retiring agents,
//! lane backend policy, backend configuration, and staged routing through
//! the agent front door.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_sema::SemaObservation;

pub use signal_frame::{
    ExchangeFrameBody as FrameExchangeFrameBody, HandshakeReply, HandshakeRequest, ProtocolVersion,
    Request as FrameRequest, SIGNAL_FRAME_PROTOCOL_VERSION,
};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct AgentIdentifier(String);

impl AgentIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct LaneName(String);

impl LaneName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct WirePath(String);

impl WirePath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct ModelName(String);

impl ModelName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct ExtensionName(String);

impl ExtensionName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum AgentBackend {
    Claude,
    Codex,
    Gemini,
    Pi,
    OpenCode,
    Fixture,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ThinkingLevel {
    Disabled,
    Low,
    Medium,
    High,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum BackendEndpoint {
    UnixSocket(WirePath),
    InternalFixture,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum BackendQuarantineReason {
    HealthCheckFailed,
    OwnerDisabled,
    BackendUnavailable,
    UnsafeConfiguration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct BackendQuarantine {
    pub reason: BackendQuarantineReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum BackendAvailability {
    Enabled,
    Disabled,
    Quarantined(BackendQuarantine),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct BackendConfiguration {
    pub endpoint: BackendEndpoint,
    pub availability: BackendAvailability,
    pub model: Option<ModelName>,
    pub thinking_level: ThinkingLevel,
    pub extensions: Vec<ExtensionName>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SpawnAgent {
    pub agent: AgentIdentifier,
    pub backend: AgentBackend,
    pub lane: LaneName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RetirementReason {
    OwnerRequested,
    LaneRetired,
    BackendUnavailable,
    Superseded,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RetireAgent {
    pub agent: AgentIdentifier,
    pub reason: RetirementReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SetBackendPolicy {
    pub lane: LaneName,
    pub default_backend: AgentBackend,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MutateBackendConfiguration {
    pub backend: AgentBackend,
    pub configuration: BackendConfiguration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RouteThroughAgent {
    pub lane: LaneName,
    pub enabled: bool,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AgentSpawned {
    pub agent: AgentIdentifier,
    pub backend: AgentBackend,
    pub lane: LaneName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AgentRetired {
    pub agent: AgentIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct BackendPolicySet {
    pub lane: LaneName,
    pub default_backend: AgentBackend,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct BackendConfigurationMutated {
    pub backend: AgentBackend,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AgentRouteSet {
    pub lane: LaneName,
    pub enabled: bool,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RejectionReason {
    AgentAlreadyActive,
    AgentMissing,
    BackendUnavailable,
    BackendQuarantined,
    LanePolicyConflict,
    LegacyRouteUnavailable,
    PolicyStoreUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OrderRejected {
    pub reason: RejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
    PolicyStoreUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel Owner {
        operation SpawnAgent(SpawnAgent),
        operation RetireAgent(RetireAgent),
        operation SetBackendPolicy(SetBackendPolicy),
        operation MutateBackendConfiguration(MutateBackendConfiguration),
        operation RouteThroughAgent(RouteThroughAgent),
    }
    reply Reply {
        AgentSpawned(AgentSpawned),
        AgentRetired(AgentRetired),
        BackendPolicySet(BackendPolicySet),
        BackendConfigurationMutated(BackendConfigurationMutated),
        AgentRouteSet(AgentRouteSet),
        OrderRejected(OrderRejected),
        RequestUnimplemented(RequestUnimplemented),
    }
    observable {
        filter default;
        operation_event OperationReceived;
        effect_event EffectEmitted;
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OperationReceived {
    pub operation: OperationKind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EffectEmitted {
    pub observation: SemaObservation,
}

impl From<SpawnAgent> for Operation {
    fn from(payload: SpawnAgent) -> Self {
        Self::SpawnAgent(payload)
    }
}

impl From<RetireAgent> for Operation {
    fn from(payload: RetireAgent) -> Self {
        Self::RetireAgent(payload)
    }
}

impl From<SetBackendPolicy> for Operation {
    fn from(payload: SetBackendPolicy) -> Self {
        Self::SetBackendPolicy(payload)
    }
}

impl From<MutateBackendConfiguration> for Operation {
    fn from(payload: MutateBackendConfiguration) -> Self {
        Self::MutateBackendConfiguration(payload)
    }
}

impl From<RouteThroughAgent> for Operation {
    fn from(payload: RouteThroughAgent) -> Self {
        Self::RouteThroughAgent(payload)
    }
}
