//! Architectural-truth round-trip tests for the schema-derived
//! `meta-signal-agent` contract. Each request and reply variant round-trips
//! through the `signal_frame` envelope (rkyv) and through NOTA text.

use meta_signal_agent::{
    ApiKeyHandle, ConfigureProvider, DefaultProviderSet, EndpointUrl, Frame, FrameBody, Input,
    Lifecycle, LifecycleState, ModelName, OperationKind, OrderRejection, OrderRejectionReason,
    Output, ProviderConfiguration, ProviderConfigured, ProviderName, ProviderRetired,
    RejectionDetail, RequestUnimplemented, RetireProvider, SetDefaultProvider, Start, Stop,
    UnimplementedReason,
};
use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, SessionEpoch, SubReply,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn deepseek() -> ProviderConfiguration {
    ProviderConfiguration {
        name: ProviderName::new("deepseek".to_owned()),
        endpoint: EndpointUrl::new("https://api.deepseek.com/v1".to_owned()),
        default_model: ModelName::new("deepseek-v4-flash".to_owned()),
        api_key_handle: ApiKeyHandle::new("DEEPSEEK_API_KEY".to_owned()),
    }
}

fn round_trip_request(request: Input) -> Input {
    let expected = request.clone();
    let frame = request.into_frame(exchange());
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => {
            assert_eq!(request.payloads().head(), &expected);
            request.payloads().head().clone()
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: Output) -> Output {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    assert_eq!(encoded, expected);
    let recovered = NotaSource::new(&encoded)
        .parse::<T>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
}

#[test]
fn every_request_round_trips_through_frame() {
    let requests = [
        Input::ConfigureProvider(ConfigureProvider::new(deepseek())),
        Input::RetireProvider(RetireProvider::new(ProviderName::new("mimo".to_owned()))),
        Input::SetDefaultProvider(SetDefaultProvider::new(ProviderName::new(
            "deepseek".to_owned(),
        ))),
        Input::Start(Start {}),
        Input::Stop(Stop {}),
    ];
    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn every_reply_round_trips_through_frame() {
    let replies = [
        Output::ProviderConfigured(ProviderConfigured::new(ProviderName::new(
            "deepseek".to_owned(),
        ))),
        Output::ProviderRetired(ProviderRetired::new(ProviderName::new("mimo".to_owned()))),
        Output::DefaultProviderSet(DefaultProviderSet::new(ProviderName::new(
            "deepseek".to_owned(),
        ))),
        Output::Started(Lifecycle::new(LifecycleState::Started)),
        Output::Stopped(Lifecycle::new(LifecycleState::Stopped)),
        Output::OrderRejected(OrderRejection {
            reason: OrderRejectionReason::ProviderUnknown,
            detail: RejectionDetail::new("no such provider".to_owned()),
        }),
        Output::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::SetDefaultProvider,
            reason: UnimplementedReason::NotInPrototypeScope,
        }),
    ];
    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn input_exposes_contract_owned_operation_kind() {
    assert_eq!(
        Input::ConfigureProvider(ConfigureProvider::new(deepseek())).operation_kind(),
        OperationKind::ConfigureProvider
    );
    assert_eq!(Input::Stop(Stop {}).operation_kind(), OperationKind::Stop);
}

#[test]
fn provider_configuration_round_trips_through_nota_text_with_key_handle_only() {
    round_trip_nota(
        Input::ConfigureProvider(ConfigureProvider::new(deepseek())),
        "(ConfigureProvider (deepseek https://api.deepseek.com/v1 deepseek-v4-flash DEEPSEEK_API_KEY))",
    );
}

#[test]
fn order_rejection_round_trips_through_nota_text() {
    round_trip_nota(
        Output::OrderRejected(OrderRejection {
            reason: OrderRejectionReason::KeyHandleMissing,
            detail: RejectionDetail::new("env var not set".to_owned()),
        }),
        "(OrderRejected (KeyHandleMissing [env var not set]))",
    );
}
