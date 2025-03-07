// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_network_insights_access_scope_input_input_input(
    input: &crate::operation::create_network_insights_access_scope::CreateNetworkInsightsAccessScopeInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateNetworkInsightsAccessScope", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("MatchPath");
    if let Some(var_2) = &input.match_paths {
        if !var_2.is_empty() {
            let mut list_4 = scope_1.start_list(true, Some("item"));
            for item_3 in var_2 {
                #[allow(unused_mut)]
                let mut entry_5 = list_4.entry();
                crate::protocol_serde::shape_access_scope_path_request::ser_access_scope_path_request(entry_5, item_3)?;
            }
            list_4.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("ExcludePath");
    if let Some(var_7) = &input.exclude_paths {
        if !var_7.is_empty() {
            let mut list_9 = scope_6.start_list(true, Some("item"));
            for item_8 in var_7 {
                #[allow(unused_mut)]
                let mut entry_10 = list_9.entry();
                crate::protocol_serde::shape_access_scope_path_request::ser_access_scope_path_request(entry_10, item_8)?;
            }
            list_9.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("ClientToken");
    if let Some(var_12) = &input.client_token {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("TagSpecification");
    if let Some(var_14) = &input.tag_specifications {
        if !var_14.is_empty() {
            let mut list_16 = scope_13.start_list(true, Some("item"));
            for item_15 in var_14 {
                #[allow(unused_mut)]
                let mut entry_17 = list_16.entry();
                crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_17, item_15)?;
            }
            list_16.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("DryRun");
    if let Some(var_19) = &input.dry_run {
        scope_18.boolean(*var_19);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
