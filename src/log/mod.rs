use anyhow::Result;
use opentelemetry::{
    global,
    sdk::{
        propagation::TraceContextPropagator,
        trace::{
            self,
            IdGenerator,
            Sampler,
        },
        Resource,
    },
    KeyValue,
};
use opentelemetry_otlp::{
    OtlpTracePipeline,
    TonicExporterBuilder,
    WithExportConfig,
};
use std::str::FromStr;
use tracing::{
    subscriber,
    Level,
};
use tracing_subscriber::{
    filter::{
        Directive,
        EnvFilter,
    },
    fmt::format::FmtSpan,
    layer::{
        Layered,
        SubscriberExt,
    },
    Registry,
};
use url::Url;

pub type HandleType = tracing_subscriber::reload::Handle<
    EnvFilter,
    Layered<tracing_subscriber::fmt::Layer<Registry>, Registry>,
>;

fn init_otlp_exporter(otel_addr: String) -> TonicExporterBuilder {
    let addr = Url::parse(&otel_addr)
        .unwrap_or_else(|_| {
            tracing::error!(
                "Cannot parse otel http address string. Using the default \
                 value",
            );
            Url::from_str("http://127.0.0.1:4317").unwrap()
        })
        .to_string();

    opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(addr)
        .with_timeout(std::time::Duration::from_secs(3))
}

fn init_builder(pipeline: TonicExporterBuilder) -> OtlpTracePipeline {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(pipeline)
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(IdGenerator::default())
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "holdmybackup",
                )])),
        )
}
pub fn init_tracer(
    log_level: String,
    otel_addr: String,
) -> Result<HandleType> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = init_otlp_exporter(otel_addr);
    let builder = init_builder(exporter);

    let tracer = builder
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Cannot build OTEL tracer");

    let filter = EnvFilter::from_default_env()
        .add_directive(Directive::from(Level::from_str(&log_level)?));

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ACTIVE)
        .with_thread_ids(true)
        .with_level(true)
        .with_env_filter(filter)
        .with_filter_reloading();

    let handle = subscriber.reload_handle();
    let subscriber = subscriber.finish();

    match subscriber::set_global_default(subscriber.with(telemetry)) {
        Ok(_) => tracing::debug!("Global subscriber's set."),
        Err(e) => {
            tracing::error!("Cannot set global subscriber: {}", e.to_string());
            std::process::exit(1);
        }
    }

    Ok(handle)
}
