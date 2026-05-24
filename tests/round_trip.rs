use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use owner_signal_agent::{
    AgentBackend, AgentIdentifier, AgentRetired, AgentRouteSet, AgentSpawned, BackendAvailability,
    BackendConfiguration, BackendConfigurationMutated, BackendEndpoint, BackendPolicySet,
    ExtensionName, Frame, FrameBody, LaneName, ModelName, MutateBackendConfiguration, Operation,
    OperationKind, OrderRejected, RejectionReason, Reply as AgentReply, RequestUnimplemented,
    RetireAgent, RetirementReason, RouteThroughAgent, SetBackendPolicy, SpawnAgent, ThinkingLevel,
    UnimplementedReason, WirePath,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, SignalOperationHeads, SubReply,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn agent() -> AgentIdentifier {
    AgentIdentifier::new("agent-alpha")
}

fn lane() -> LaneName {
    LaneName::new("designer")
}

fn backend_configuration() -> BackendConfiguration {
    BackendConfiguration {
        endpoint: BackendEndpoint::UnixSocket(WirePath::new("claude-socket")),
        availability: BackendAvailability::Enabled,
        model: Some(ModelName::new("claude-sonnet")),
        thinking_level: ThinkingLevel::High,
        extensions: vec![ExtensionName::new("filesystem")],
    }
}

fn completed_reply(payload: AgentReply) -> FrameReply<AgentReply> {
    FrameReply::committed(NonEmpty::single(SubReply::Ok(payload)))
}

fn round_trip_operation(operation: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: operation.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode operation");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode operation");

    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request, got {other:?}"),
    }
}

fn round_trip_reply(reply: AgentReply) -> AgentReply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: completed_reply(reply.clone()),
    });
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");

    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

fn round_trip_nota<Value>(value: Value, expected: &str)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota text");
    let encoded = encoder.into_string();
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = Value::decode(&mut decoder).expect("decode nota text");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn owner_agent_operations_round_trip_through_length_prefixed_frames() {
    let operations = vec![
        Operation::SpawnAgent(SpawnAgent {
            agent: agent(),
            backend: AgentBackend::Claude,
            lane: lane(),
        }),
        Operation::RetireAgent(RetireAgent {
            agent: agent(),
            reason: RetirementReason::OwnerRequested,
        }),
        Operation::SetBackendPolicy(SetBackendPolicy {
            lane: lane(),
            default_backend: AgentBackend::Claude,
        }),
        Operation::MutateBackendConfiguration(MutateBackendConfiguration {
            backend: AgentBackend::Claude,
            configuration: backend_configuration(),
        }),
        Operation::RouteThroughAgent(RouteThroughAgent {
            lane: lane(),
            enabled: true,
        }),
    ];

    for operation in operations {
        assert_eq!(round_trip_operation(operation.clone()), operation);
    }
}

#[test]
fn owner_agent_replies_round_trip_through_length_prefixed_frames() {
    let replies = vec![
        AgentReply::AgentSpawned(AgentSpawned {
            agent: agent(),
            backend: AgentBackend::Claude,
            lane: lane(),
        }),
        AgentReply::AgentRetired(AgentRetired { agent: agent() }),
        AgentReply::BackendPolicySet(BackendPolicySet {
            lane: lane(),
            default_backend: AgentBackend::Claude,
        }),
        AgentReply::BackendConfigurationMutated(BackendConfigurationMutated {
            backend: AgentBackend::Claude,
        }),
        AgentReply::AgentRouteSet(AgentRouteSet {
            lane: lane(),
            enabled: true,
        }),
        AgentReply::OrderRejected(OrderRejected {
            reason: RejectionReason::BackendUnavailable,
        }),
        AgentReply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn owner_agent_operation_kind_is_generated_by_macro() {
    let cases = vec![
        (
            Operation::SpawnAgent(SpawnAgent {
                agent: agent(),
                backend: AgentBackend::Claude,
                lane: lane(),
            }),
            OperationKind::SpawnAgent,
        ),
        (
            Operation::RetireAgent(RetireAgent {
                agent: agent(),
                reason: RetirementReason::OwnerRequested,
            }),
            OperationKind::RetireAgent,
        ),
        (
            Operation::SetBackendPolicy(SetBackendPolicy {
                lane: lane(),
                default_backend: AgentBackend::Claude,
            }),
            OperationKind::SetBackendPolicy,
        ),
        (
            Operation::MutateBackendConfiguration(MutateBackendConfiguration {
                backend: AgentBackend::Claude,
                configuration: backend_configuration(),
            }),
            OperationKind::MutateBackendConfiguration,
        ),
        (
            Operation::RouteThroughAgent(RouteThroughAgent {
                lane: lane(),
                enabled: true,
            }),
            OperationKind::RouteThroughAgent,
        ),
    ];

    for (operation, expected_kind) in cases {
        assert_eq!(operation.kind(), expected_kind);
    }
}

#[test]
fn owner_agent_domain_operations_are_contract_local_heads() {
    for expected in [
        "SpawnAgent",
        "RetireAgent",
        "SetBackendPolicy",
        "MutateBackendConfiguration",
        "RouteThroughAgent",
    ] {
        assert!(Operation::HEADS.contains(&expected));
    }
}

#[test]
fn owner_agent_nota_text_shape_stays_canonical() {
    round_trip_nota(
        Operation::SpawnAgent(SpawnAgent {
            agent: agent(),
            backend: AgentBackend::Claude,
            lane: lane(),
        }),
        "(SpawnAgent (agent-alpha Claude designer))",
    );
    round_trip_nota(
        Operation::RetireAgent(RetireAgent {
            agent: agent(),
            reason: RetirementReason::OwnerRequested,
        }),
        "(RetireAgent (agent-alpha OwnerRequested))",
    );
    round_trip_nota(
        Operation::SetBackendPolicy(SetBackendPolicy {
            lane: lane(),
            default_backend: AgentBackend::Claude,
        }),
        "(SetBackendPolicy (designer Claude))",
    );
    round_trip_nota(
        Operation::MutateBackendConfiguration(MutateBackendConfiguration {
            backend: AgentBackend::Claude,
            configuration: backend_configuration(),
        }),
        "(MutateBackendConfiguration (Claude ((UnixSocket claude-socket) Enabled (Some claude-sonnet) High [filesystem])))",
    );
    round_trip_nota(
        Operation::RouteThroughAgent(RouteThroughAgent {
            lane: lane(),
            enabled: true,
        }),
        "(RouteThroughAgent (designer True))",
    );
    round_trip_nota(
        AgentReply::AgentSpawned(AgentSpawned {
            agent: agent(),
            backend: AgentBackend::Claude,
            lane: lane(),
        }),
        "(AgentSpawned (agent-alpha Claude designer))",
    );
    round_trip_nota(
        AgentReply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
        "(RequestUnimplemented (NotBuiltYet))",
    );
}
