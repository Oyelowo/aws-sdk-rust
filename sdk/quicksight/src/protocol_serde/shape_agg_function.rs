// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_agg_function(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AggFunction,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.aggregation {
        object.key("Aggregation").string(var_1.as_str());
    }
    if let Some(var_2) = &input.aggregation_function_parameters {
        #[allow(unused_mut)]
        let mut object_3 = object.key("AggregationFunctionParameters").start_object();
        for (key_4, value_5) in var_2 {
            {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.period {
        object.key("Period").string(var_6.as_str());
    }
    if let Some(var_7) = &input.period_field {
        object.key("PeriodField").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_agg_function<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AggFunction>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AggFunctionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Aggregation" => {
                            builder = builder.set_aggregation(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AggType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "AggregationFunctionParameters" => {
                            builder = builder.set_aggregation_function_parameters(
                                crate::protocol_serde::shape_agg_function_param_map::de_agg_function_param_map(tokens)?,
                            );
                        }
                        "Period" => {
                            builder = builder.set_period(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::TopicTimeGranularity::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "PeriodField" => {
                            builder = builder.set_period_field(
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
