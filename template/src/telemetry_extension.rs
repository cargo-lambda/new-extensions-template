use lambda_extension::{tracing, LambdaTelemetry, LambdaTelemetryRecord, Error};

/// Process telemetry from the Lambda extension API.
///
/// This function is called when the Lambda extension receives telemetry from the Lambda API.
pub(crate) async fn telemetry_processor(events: Vec<LambdaTelemetry>) -> Result<(), Error> {
    for event in events {
        match event.record {
            LambdaTelemetryRecord::Function(record) => {
                tracing::info!(telemetry_type = "function", ?record, "received function telemetry");
            }
            _ignore_other => {},
        }
    }

    Ok(())
}