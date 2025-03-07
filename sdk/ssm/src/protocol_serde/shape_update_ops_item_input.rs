// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_ops_item_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_ops_item::UpdateOpsItemInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.operational_data {
        #[allow(unused_mut)]
        let mut object_3 = object.key("OperationalData").start_object();
        for (key_4, value_5) in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_6 = object_3.key(key_4.as_str()).start_object();
                crate::protocol_serde::shape_ops_item_data_value::ser_ops_item_data_value(&mut object_6, value_5)?;
                object_6.finish();
            }
        }
        object_3.finish();
    }
    if let Some(var_7) = &input.operational_data_to_delete {
        let mut array_8 = object.key("OperationalDataToDelete").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.notifications {
        let mut array_11 = object.key("Notifications").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_ops_item_notification::ser_ops_item_notification(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.priority {
        object.key("Priority").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.related_ops_items {
        let mut array_16 = object.key("RelatedOpsItems").start_array();
        for item_17 in var_15 {
            {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::protocol_serde::shape_related_ops_item::ser_related_ops_item(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if let Some(var_19) = &input.status {
        object.key("Status").string(var_19.as_str());
    }
    if let Some(var_20) = &input.ops_item_id {
        object.key("OpsItemId").string(var_20.as_str());
    }
    if let Some(var_21) = &input.title {
        object.key("Title").string(var_21.as_str());
    }
    if let Some(var_22) = &input.category {
        object.key("Category").string(var_22.as_str());
    }
    if let Some(var_23) = &input.severity {
        object.key("Severity").string(var_23.as_str());
    }
    if let Some(var_24) = &input.actual_start_time {
        object
            .key("ActualStartTime")
            .date_time(var_24, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_25) = &input.actual_end_time {
        object
            .key("ActualEndTime")
            .date_time(var_25, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_26) = &input.planned_start_time {
        object
            .key("PlannedStartTime")
            .date_time(var_26, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_27) = &input.planned_end_time {
        object
            .key("PlannedEndTime")
            .date_time(var_27, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_28) = &input.ops_item_arn {
        object.key("OpsItemArn").string(var_28.as_str());
    }
    Ok(())
}
