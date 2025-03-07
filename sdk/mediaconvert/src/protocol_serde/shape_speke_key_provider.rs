// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_speke_key_provider(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SpekeKeyProvider,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.certificate_arn {
        object.key("certificateArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.encryption_contract_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("encryptionContractConfiguration").start_object();
        crate::protocol_serde::shape_encryption_contract_configuration::ser_encryption_contract_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.resource_id {
        object.key("resourceId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.system_ids {
        let mut array_6 = object.key("systemIds").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.url {
        object.key("url").string(var_8.as_str());
    }
    Ok(())
}

pub(crate) fn de_speke_key_provider<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::SpekeKeyProvider>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SpekeKeyProviderBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "certificateArn" => {
                            builder = builder.set_certificate_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "encryptionContractConfiguration" => {
                            builder = builder.set_encryption_contract_configuration(
                                crate::protocol_serde::shape_encryption_contract_configuration::de_encryption_contract_configuration(tokens)?,
                            );
                        }
                        "resourceId" => {
                            builder = builder.set_resource_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "systemIds" => {
                            builder = builder.set_system_ids(
                                    crate::protocol_serde::shape_list_of_string_pattern09a_faf809a_faf409a_faf409a_faf409a_faf12::de_list_of_string_pattern09a_faf809a_faf409a_faf409a_faf409a_faf12(tokens)?
                                );
                        }
                        "url" => {
                            builder = builder.set_url(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
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
