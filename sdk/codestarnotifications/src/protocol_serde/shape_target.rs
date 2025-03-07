// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_target(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Target,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.target_type {
        object.key("TargetType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_address {
        object.key("TargetAddress").string(var_2.as_str());
    }
    Ok(())
}
