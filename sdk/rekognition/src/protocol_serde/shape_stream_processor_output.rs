// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stream_processor_output(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::StreamProcessorOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.kinesis_data_stream {
        #[allow(unused_mut)]
        let mut object_2 = object.key("KinesisDataStream").start_object();
        crate::protocol_serde::shape_kinesis_data_stream::ser_kinesis_data_stream(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.s3_destination {
        #[allow(unused_mut)]
        let mut object_4 = object.key("S3Destination").start_object();
        crate::protocol_serde::shape_s3_destination::ser_s3_destination(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_stream_processor_output<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::StreamProcessorOutput>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::StreamProcessorOutputBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "KinesisDataStream" => {
                            builder =
                                builder.set_kinesis_data_stream(crate::protocol_serde::shape_kinesis_data_stream::de_kinesis_data_stream(tokens)?);
                        }
                        "S3Destination" => {
                            builder = builder.set_s3_destination(crate::protocol_serde::shape_s3_destination::de_s3_destination(tokens)?);
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
