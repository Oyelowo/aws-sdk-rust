// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_collector_health_check<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CollectorHealthCheck>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CollectorHealthCheckBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "CollectorStatus" => {
                            builder = builder.set_collector_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::CollectorStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "LocalCollectorS3Access" => {
                            builder =
                                builder.set_local_collector_s3_access(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "WebCollectorS3Access" => {
                            builder = builder.set_web_collector_s3_access(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "WebCollectorGrantedRoleBasedAccess" => {
                            builder = builder.set_web_collector_granted_role_based_access(
                                ::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?,
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
