use opentelemetry::runtime::TokioCurrentThread;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
use tokio::task::JoinHandle;
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

// /// Compose multiple layers into a `tracing`'s subscriber.
// ///
// /// # Implementation Notes
// ///
// /// We are using `impl Subscriber` as return type to avoid having to spell out the actual
// /// type of the returned subscriber, which is indeed quite complex.
// pub fn get_subscriber<'a>(
//     name: String,
//     env_filter: String,
//     collector_endpoint: Option<String>,
// ) -> impl Subscriber + Sync + Send {
//     global::set_text_map_propagator(TraceContextPropagator::new());

//     let pp = match collector_endpoint {
//         Some(cep) => opentelemetry_zipkin::new_pipeline().with_collector_endpoint(cep),
//         _ => opentelemetry_zipkin::new_pipeline(),
//     };

//     let tracer = pp
//         .with_service_name(name)
//         .install_batch(TokioCurrentThread)
//         .expect("Failed to install OpenTelemetry tracer.");

//     let env_filter =
//         EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

//     let fmt_layer = tracing_subscriber::fmt::layer();

//     Registry::default()
//         .with(env_filter)
//         .with(fmt_layer)
//         .with(tracing_opentelemetry::layer().with_tracer(tracer))
// }

// /// Register a subscriber as global default to process span data.
// ///
// /// It should only be called once!
// pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
//     LogTracer::init().expect("Failed to set logger");
//     set_global_default(subscriber).expect("Failed to set subscriber");
// }

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to spell out the actual
/// type of the returned subscriber, which is indeed quite complex.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
    collector_endpoint: Option<String>,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    global::set_text_map_propagator(TraceContextPropagator::new());

    let pp = match collector_endpoint {
        Some(cep) => opentelemetry_zipkin::new_pipeline().with_collector_endpoint(cep),
        _ => opentelemetry_zipkin::new_pipeline(),
    };

    let tracer = pp
        .with_service_name(name.clone())
        .install_batch(TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.");

    let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(tracing_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
