// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_managed_prefix_list_input_input_input(
    input: &crate::operation::modify_managed_prefix_list::ModifyManagedPrefixListInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyManagedPrefixList", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PrefixListId");
    if let Some(var_4) = &input.prefix_list_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("CurrentVersion");
    if let Some(var_6) = &input.current_version {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("PrefixListName");
    if let Some(var_8) = &input.prefix_list_name {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("AddEntry");
    if let Some(var_10) = &input.add_entries {
        if !var_10.is_empty() {
            let mut list_12 = scope_9.start_list(true, None);
            for item_11 in var_10 {
                #[allow(unused_mut)]
                let mut entry_13 = list_12.entry();
                crate::protocol_serde::shape_add_prefix_list_entry::ser_add_prefix_list_entry(entry_13, item_11)?;
            }
            list_12.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("RemoveEntry");
    if let Some(var_15) = &input.remove_entries {
        if !var_15.is_empty() {
            let mut list_17 = scope_14.start_list(true, None);
            for item_16 in var_15 {
                #[allow(unused_mut)]
                let mut entry_18 = list_17.entry();
                crate::protocol_serde::shape_remove_prefix_list_entry::ser_remove_prefix_list_entry(entry_18, item_16)?;
            }
            list_17.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("MaxEntries");
    if let Some(var_20) = &input.max_entries {
        scope_19.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
