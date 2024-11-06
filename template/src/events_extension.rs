use lambda_extension::{tracing, LambdaEvent, NextEvent, Error};

/// Process events from the Lambda extension API.
///
/// This function is called when the Lambda extension receives an event from the Lambda API.
pub(crate) async fn events_processor(event: LambdaEvent) -> Result<(), Error> {
    match event.next {
        NextEvent::Shutdown(event) => {
            tracing::info!(event_type = "shutdown", ?event, "shutting down function");
        }
        NextEvent::Invoke(event) => {
            tracing::info!(event_type = "invoke", ?event, "invoking function");
        }
    }
    Ok(())
}