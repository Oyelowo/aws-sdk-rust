// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_change(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Change,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("ChangeType").string(input.change_type.as_str());
    }
    if let Some(var_1) = &input.entity {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Entity").start_object();
        crate::protocol_serde::shape_entity::ser_entity(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.entity_tags {
        let mut array_4 = object.key("EntityTags").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.details {
        object.key("Details").string(var_7.as_str());
    }
    if let Some(var_8) = &input.details_document {
        object.key("DetailsDocument").document(var_8);
    }
    if let Some(var_9) = &input.change_name {
        object.key("ChangeName").string(var_9.as_str());
    }
    Ok(())
}
