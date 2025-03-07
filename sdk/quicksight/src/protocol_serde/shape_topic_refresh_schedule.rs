// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_topic_refresh_schedule(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TopicRefreshSchedule,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("IsEnabled").boolean(input.is_enabled);
    }
    {
        object.key("BasedOnSpiceSchedule").boolean(input.based_on_spice_schedule);
    }
    if let Some(var_1) = &input.starting_at {
        object
            .key("StartingAt")
            .date_time(var_1, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_2) = &input.timezone {
        object.key("Timezone").string(var_2.as_str());
    }
    if let Some(var_3) = &input.repeat_at {
        object.key("RepeatAt").string(var_3.as_str());
    }
    if let Some(var_4) = &input.topic_schedule_type {
        object.key("TopicScheduleType").string(var_4.as_str());
    }
    Ok(())
}

pub(crate) fn de_topic_refresh_schedule<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TopicRefreshSchedule>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TopicRefreshScheduleBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "IsEnabled" => {
                            builder = builder.set_is_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "BasedOnSpiceSchedule" => {
                            builder = builder.set_based_on_spice_schedule(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "StartingAt" => {
                            builder = builder.set_starting_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "Timezone" => {
                            builder = builder.set_timezone(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "RepeatAt" => {
                            builder = builder.set_repeat_at(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TopicScheduleType" => {
                            builder = builder.set_topic_schedule_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::TopicScheduleType::from(u.as_ref())))
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
            Ok(Some(crate::serde_util::topic_refresh_schedule_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
