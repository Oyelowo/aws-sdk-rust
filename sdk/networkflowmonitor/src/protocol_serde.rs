// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_create_monitor;

pub(crate) mod shape_create_scope;

pub(crate) mod shape_delete_monitor;

pub(crate) mod shape_delete_scope;

pub(crate) mod shape_get_monitor;

pub(crate) mod shape_get_query_results_monitor_top_contributors;

pub(crate) mod shape_get_query_results_workload_insights_top_contributors;

pub(crate) mod shape_get_query_results_workload_insights_top_contributors_data;

pub(crate) mod shape_get_query_status_monitor_top_contributors;

pub(crate) mod shape_get_query_status_workload_insights_top_contributors;

pub(crate) mod shape_get_query_status_workload_insights_top_contributors_data;

pub(crate) mod shape_get_scope;

pub(crate) mod shape_list_monitors;

pub(crate) mod shape_list_scopes;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_start_query_monitor_top_contributors;

pub(crate) mod shape_start_query_workload_insights_top_contributors;

pub(crate) mod shape_start_query_workload_insights_top_contributors_data;

pub(crate) mod shape_stop_query_monitor_top_contributors;

pub(crate) mod shape_stop_query_workload_insights_top_contributors;

pub(crate) mod shape_stop_query_workload_insights_top_contributors_data;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_monitor;

pub(crate) mod shape_update_scope;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_monitor_input;

pub(crate) mod shape_create_scope_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_query_monitor_top_contributors_input;

pub(crate) mod shape_start_query_workload_insights_top_contributors_data_input;

pub(crate) mod shape_start_query_workload_insights_top_contributors_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_monitor_input;

pub(crate) mod shape_update_scope_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_monitor_list;

pub(crate) mod shape_monitor_local_resource;

pub(crate) mod shape_monitor_local_resources;

pub(crate) mod shape_monitor_remote_resource;

pub(crate) mod shape_monitor_remote_resources;

pub(crate) mod shape_monitor_top_contributors_row_list;

pub(crate) mod shape_scope_summary_list;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_target_resource;

pub(crate) mod shape_target_resource_list;

pub(crate) mod shape_workload_insights_top_contributors_data_points;

pub(crate) mod shape_workload_insights_top_contributors_row_list;

pub(crate) mod shape_monitor_summary;

pub(crate) mod shape_monitor_top_contributors_row;

pub(crate) mod shape_scope_summary;

pub(crate) mod shape_target_identifier;

pub(crate) mod shape_workload_insights_top_contributors_data_point;

pub(crate) mod shape_workload_insights_top_contributors_row;

pub(crate) mod shape_kubernetes_metadata;

pub(crate) mod shape_target_id;

pub(crate) mod shape_traversed_constructs_list;

pub(crate) mod shape_workload_insights_top_contributors_timestamps_list;

pub(crate) mod shape_workload_insights_top_contributors_values_list;

pub(crate) mod shape_traversed_component;
