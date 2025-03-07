// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_application_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_application::UpdateApplicationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.display_name {
        object.key("DisplayName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.icon_s3_location {
        #[allow(unused_mut)]
        let mut object_5 = object.key("IconS3Location").start_object();
        crate::protocol_serde::shape_s3_location::ser_s3_location(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.launch_path {
        object.key("LaunchPath").string(var_6.as_str());
    }
    if let Some(var_7) = &input.working_directory {
        object.key("WorkingDirectory").string(var_7.as_str());
    }
    if let Some(var_8) = &input.launch_parameters {
        object.key("LaunchParameters").string(var_8.as_str());
    }
    if let Some(var_9) = &input.app_block_arn {
        object.key("AppBlockArn").string(var_9.as_str());
    }
    if let Some(var_10) = &input.attributes_to_delete {
        let mut array_11 = object.key("AttributesToDelete").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    Ok(())
}
