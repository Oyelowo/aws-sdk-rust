// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_tape_with_barcode_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_tape_with_barcode::CreateTapeWithBarcodeInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.gateway_arn {
        object.key("GatewayARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tape_size_in_bytes {
        object.key("TapeSizeInBytes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.tape_barcode {
        object.key("TapeBarcode").string(var_3.as_str());
    }
    if let Some(var_4) = &input.kms_encrypted {
        object.key("KMSEncrypted").boolean(*var_4);
    }
    if let Some(var_5) = &input.kms_key {
        object.key("KMSKey").string(var_5.as_str());
    }
    if let Some(var_6) = &input.pool_id {
        object.key("PoolId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.worm {
        object.key("Worm").boolean(*var_7);
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    Ok(())
}
