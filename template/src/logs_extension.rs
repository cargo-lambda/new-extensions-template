use lambda_extension::{tracing, LambdaLog, LambdaLogRecord, Error};

/// Process logs from the Lambda extension API.
///
/// This function is called when the Lambda extension receives logs from the Lambda API.
/// The Lambda API sends logs in batches, so this function processes a batch of logs at a time.
/// These logs can come from the function the extension is attached to,
/// or from another extension that has also been added to the same Lambda function.
pub(crate) async fn logs_processor(logs: Vec<LambdaLog>) -> Result<(), Error> {
    for log in logs {
        match log.record {
            LambdaLogRecord::Function(record) => {
                tracing::info!(log_type = "function", ?record, "received function logs");
            }
            LambdaLogRecord::Extension(record) => {
                tracing::info!(log_type = "extension", ?record, "received extension logs");
            },
            _ignore_other => {},
        }
    }

    Ok(())
}