// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn resource_not_found_exception_correct_errors(
    mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
) -> crate::types::error::builders::ResourceNotFoundExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.resource_id.is_none() {
        builder.resource_id = Some(Default::default())
    }
    if builder.resource_type.is_none() {
        builder.resource_type = Some(Default::default())
    }
    builder
}

pub(crate) fn access_denied_exception_correct_errors(
    mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
) -> crate::types::error::builders::AccessDeniedExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn internal_server_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
) -> crate::types::error::builders::InternalServerExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn throttling_exception_correct_errors(
    mut builder: crate::types::error::builders::ThrottlingExceptionBuilder,
) -> crate::types::error::builders::ThrottlingExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_correct_errors(
    mut builder: crate::types::error::builders::ValidationExceptionBuilder,
) -> crate::types::error::builders::ValidationExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn get_search_job_output_output_correct_errors(
    mut builder: crate::operation::get_search_job::builders::GetSearchJobOutputBuilder,
) -> crate::operation::get_search_job::builders::GetSearchJobOutputBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::SearchJobState>().ok()
    }
    if builder.search_scope.is_none() {
        builder.search_scope = {
            let builder = crate::types::builders::SearchScopeBuilder::default();
            crate::serde_util::search_scope_correct_errors(builder).build().ok()
        }
    }
    if builder.item_filters.is_none() {
        builder.item_filters = {
            let builder = crate::types::builders::ItemFiltersBuilder::default();
            Some(builder.build())
        }
    }
    if builder.creation_time.is_none() {
        builder.creation_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.search_job_identifier.is_none() {
        builder.search_job_identifier = Some(Default::default())
    }
    if builder.search_job_arn.is_none() {
        builder.search_job_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn get_search_result_export_job_output_output_correct_errors(
    mut builder: crate::operation::get_search_result_export_job::builders::GetSearchResultExportJobOutputBuilder,
) -> crate::operation::get_search_result_export_job::builders::GetSearchResultExportJobOutputBuilder {
    if builder.export_job_identifier.is_none() {
        builder.export_job_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn list_search_job_backups_output_output_correct_errors(
    mut builder: crate::operation::list_search_job_backups::builders::ListSearchJobBackupsOutputBuilder,
) -> crate::operation::list_search_job_backups::builders::ListSearchJobBackupsOutputBuilder {
    if builder.results.is_none() {
        builder.results = Some(Default::default())
    }
    builder
}

pub(crate) fn list_search_job_results_output_output_correct_errors(
    mut builder: crate::operation::list_search_job_results::builders::ListSearchJobResultsOutputBuilder,
) -> crate::operation::list_search_job_results::builders::ListSearchJobResultsOutputBuilder {
    if builder.results.is_none() {
        builder.results = Some(Default::default())
    }
    builder
}

pub(crate) fn list_search_jobs_output_output_correct_errors(
    mut builder: crate::operation::list_search_jobs::builders::ListSearchJobsOutputBuilder,
) -> crate::operation::list_search_jobs::builders::ListSearchJobsOutputBuilder {
    if builder.search_jobs.is_none() {
        builder.search_jobs = Some(Default::default())
    }
    builder
}

pub(crate) fn service_quota_exceeded_exception_correct_errors(
    mut builder: crate::types::error::builders::ServiceQuotaExceededExceptionBuilder,
) -> crate::types::error::builders::ServiceQuotaExceededExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.resource_id.is_none() {
        builder.resource_id = Some(Default::default())
    }
    if builder.resource_type.is_none() {
        builder.resource_type = Some(Default::default())
    }
    if builder.service_code.is_none() {
        builder.service_code = Some(Default::default())
    }
    if builder.quota_code.is_none() {
        builder.quota_code = Some(Default::default())
    }
    builder
}

pub(crate) fn list_search_result_export_jobs_output_output_correct_errors(
    mut builder: crate::operation::list_search_result_export_jobs::builders::ListSearchResultExportJobsOutputBuilder,
) -> crate::operation::list_search_result_export_jobs::builders::ListSearchResultExportJobsOutputBuilder {
    if builder.export_jobs.is_none() {
        builder.export_jobs = Some(Default::default())
    }
    builder
}

pub(crate) fn conflict_exception_correct_errors(
    mut builder: crate::types::error::builders::ConflictExceptionBuilder,
) -> crate::types::error::builders::ConflictExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.resource_id.is_none() {
        builder.resource_id = Some(Default::default())
    }
    if builder.resource_type.is_none() {
        builder.resource_type = Some(Default::default())
    }
    builder
}

pub(crate) fn start_search_result_export_job_output_output_correct_errors(
    mut builder: crate::operation::start_search_result_export_job::builders::StartSearchResultExportJobOutputBuilder,
) -> crate::operation::start_search_result_export_job::builders::StartSearchResultExportJobOutputBuilder {
    if builder.export_job_identifier.is_none() {
        builder.export_job_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn search_scope_correct_errors(mut builder: crate::types::builders::SearchScopeBuilder) -> crate::types::builders::SearchScopeBuilder {
    if builder.backup_resource_types.is_none() {
        builder.backup_resource_types = Some(Default::default())
    }
    builder
}

pub(crate) fn export_job_summary_correct_errors(
    mut builder: crate::types::builders::ExportJobSummaryBuilder,
) -> crate::types::builders::ExportJobSummaryBuilder {
    if builder.export_job_identifier.is_none() {
        builder.export_job_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn s3_export_specification_correct_errors(
    mut builder: crate::types::builders::S3ExportSpecificationBuilder,
) -> crate::types::builders::S3ExportSpecificationBuilder {
    if builder.destination_bucket.is_none() {
        builder.destination_bucket = Some(Default::default())
    }
    builder
}

pub(crate) fn long_condition_correct_errors(
    mut builder: crate::types::builders::LongConditionBuilder,
) -> crate::types::builders::LongConditionBuilder {
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn string_condition_correct_errors(
    mut builder: crate::types::builders::StringConditionBuilder,
) -> crate::types::builders::StringConditionBuilder {
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn time_condition_correct_errors(
    mut builder: crate::types::builders::TimeConditionBuilder,
) -> crate::types::builders::TimeConditionBuilder {
    if builder.value.is_none() {
        builder.value = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}
