// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_extend_license_consumption_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::extend_license_consumption::ExtendLicenseConsumptionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.license_consumption_token {
        object.key("LicenseConsumptionToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.dry_run {
        object.key("DryRun").boolean(*var_2);
    }
    Ok(())
}
