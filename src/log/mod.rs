use {
    anyhow::Result,
    std::str::FromStr,
    tracing::Level,
    tracing_subscriber::filter::{
        Directive,
        EnvFilter,
    },
    tracing_subscriber::fmt::{
        format::{
            DefaultFields,
            FmtSpan,
        },
        SubscriberBuilder,
    },
    tracing_subscriber::layer::Layered,
    tracing_subscriber::Registry,
};

pub type SubscriberType = SubscriberBuilder<
    DefaultFields,
    tracing_subscriber::fmt::format::Format,
    tracing_subscriber::reload::Layer<
        EnvFilter,
        Layered<tracing_subscriber::fmt::Layer<Registry>, Registry>,
    >,
>;

pub fn init_tracer(log_level: String) -> Result<SubscriberType> {
    let filter = EnvFilter::from_default_env()
        .add_directive(Directive::from(Level::from_str(&log_level)?));

    let subscriber = tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ACTIVE)
        .with_thread_ids(true)
        .with_level(true)
        .with_env_filter(filter)
        .with_filter_reloading();

    Ok(subscriber)
}
