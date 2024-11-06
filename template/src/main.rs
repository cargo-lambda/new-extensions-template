{%- if logs or telemetry -%}
use lambda_extension::{service_fn, tracing, Extension, Error, SharedService};
{%- else -%}
use lambda_extension::{service_fn, tracing, Extension, Error};
{%- endif %}

{% if events -%}
mod events_extension;
use events_extension::events_processor;
{%- endif %}

{% if logs -%}
mod logs_extension;
use logs_extension::logs_processor;
{%- elsif telemetry -%}
mod telemetry_extension;
use telemetry_extension::telemetry_processor;
{%- endif %}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    Extension::new()
        {%- if events %}
        .with_events_processor(service_fn(events_processor))
        {%- endif -%}
        {%- if logs %}
        .with_logs_processor(SharedService::new(service_fn(logs_processor)))
        {%- endif -%}
        {%- if telemetry %}
        .with_telemetry_processor(SharedService::new(service_fn(telemetry_processor)))
        {%- endif %}
        .run()
        .await
}
