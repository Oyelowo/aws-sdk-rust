// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_fsx_update_protocol_smb(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FsxUpdateProtocolSmb,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.domain {
        object.key("Domain").string(var_1.as_str());
    }
    if let Some(var_2) = &input.mount_options {
        #[allow(unused_mut)]
        let mut object_3 = object.key("MountOptions").start_object();
        crate::protocol_serde::shape_smb_mount_options::ser_smb_mount_options(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.password {
        object.key("Password").string(var_4.as_str());
    }
    if let Some(var_5) = &input.user {
        object.key("User").string(var_5.as_str());
    }
    Ok(())
}
