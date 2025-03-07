// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_library_item_metadata_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_library_item_metadata::UpdateLibraryItemMetadataInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.is_verified {
        object.key("isVerified").boolean(*var_1);
    }
    if let Some(var_2) = &input.library_item_id {
        object.key("libraryItemId").string(var_2.as_str());
    }
    Ok(())
}
