// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelExportTask`](crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`task_identifier(impl Into<String>)`](crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder::task_identifier) / [`set_task_identifier(Option<String>)`](crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder::set_task_identifier):<br>required: **true**<br><p>The unique identifier of the export task.</p><br>
    /// - On success, responds with [`CancelExportTaskOutput`](crate::operation::cancel_export_task::CancelExportTaskOutput) with field(s):
    ///   - [`graph_id(String)`](crate::operation::cancel_export_task::CancelExportTaskOutput::graph_id): <p>The source graph identifier of the cancelled export task.</p>
    ///   - [`role_arn(String)`](crate::operation::cancel_export_task::CancelExportTaskOutput::role_arn): <p>The ARN of the IAM role that will allow the exporting of data to the destination.</p>
    ///   - [`task_id(String)`](crate::operation::cancel_export_task::CancelExportTaskOutput::task_id): <p>The unique identifier of the export task.</p>
    ///   - [`status(ExportTaskStatus)`](crate::operation::cancel_export_task::CancelExportTaskOutput::status): <p>The current status of the export task. The status is <code>CANCELLING</code> when the export task is cancelled.</p>
    ///   - [`format(ExportFormat)`](crate::operation::cancel_export_task::CancelExportTaskOutput::format): <p>The format of the cancelled export task.</p>
    ///   - [`destination(String)`](crate::operation::cancel_export_task::CancelExportTaskOutput::destination): <p>The Amazon S3 URI of the cancelled export task where data will be exported to.</p>
    ///   - [`kms_key_identifier(String)`](crate::operation::cancel_export_task::CancelExportTaskOutput::kms_key_identifier): <p>The KMS key identifier of the cancelled export task.</p>
    ///   - [`parquet_type(Option<ParquetType>)`](crate::operation::cancel_export_task::CancelExportTaskOutput::parquet_type): <p>The parquet type of the cancelled export task.</p>
    ///   - [`status_reason(Option<String>)`](crate::operation::cancel_export_task::CancelExportTaskOutput::status_reason): <p>The reason that the export task has this status value.</p>
    /// - On failure, responds with [`SdkError<CancelExportTaskError>`](crate::operation::cancel_export_task::CancelExportTaskError)
    pub fn cancel_export_task(&self) -> crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder {
        crate::operation::cancel_export_task::builders::CancelExportTaskFluentBuilder::new(self.handle.clone())
    }
}
