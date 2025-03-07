// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_weekly_auto_scaling_schedule(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::WeeklyAutoScalingSchedule,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.monday {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Monday").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.tuesday {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Tuesday").start_object();
        for (key_7, value_8) in var_5 {
            {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    if let Some(var_9) = &input.wednesday {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Wednesday").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.thursday {
        #[allow(unused_mut)]
        let mut object_14 = object.key("Thursday").start_object();
        for (key_15, value_16) in var_13 {
            {
                object_14.key(key_15.as_str()).string(value_16.as_str());
            }
        }
        object_14.finish();
    }
    if let Some(var_17) = &input.friday {
        #[allow(unused_mut)]
        let mut object_18 = object.key("Friday").start_object();
        for (key_19, value_20) in var_17 {
            {
                object_18.key(key_19.as_str()).string(value_20.as_str());
            }
        }
        object_18.finish();
    }
    if let Some(var_21) = &input.saturday {
        #[allow(unused_mut)]
        let mut object_22 = object.key("Saturday").start_object();
        for (key_23, value_24) in var_21 {
            {
                object_22.key(key_23.as_str()).string(value_24.as_str());
            }
        }
        object_22.finish();
    }
    if let Some(var_25) = &input.sunday {
        #[allow(unused_mut)]
        let mut object_26 = object.key("Sunday").start_object();
        for (key_27, value_28) in var_25 {
            {
                object_26.key(key_27.as_str()).string(value_28.as_str());
            }
        }
        object_26.finish();
    }
    Ok(())
}

pub(crate) fn de_weekly_auto_scaling_schedule<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::WeeklyAutoScalingSchedule>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::WeeklyAutoScalingScheduleBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Monday" => {
                            builder = builder.set_monday(crate::protocol_serde::shape_daily_auto_scaling_schedule::de_daily_auto_scaling_schedule(
                                tokens,
                            )?);
                        }
                        "Tuesday" => {
                            builder = builder.set_tuesday(crate::protocol_serde::shape_daily_auto_scaling_schedule::de_daily_auto_scaling_schedule(
                                tokens,
                            )?);
                        }
                        "Wednesday" => {
                            builder = builder.set_wednesday(
                                crate::protocol_serde::shape_daily_auto_scaling_schedule::de_daily_auto_scaling_schedule(tokens)?,
                            );
                        }
                        "Thursday" => {
                            builder = builder.set_thursday(crate::protocol_serde::shape_daily_auto_scaling_schedule::de_daily_auto_scaling_schedule(
                                tokens,
                            )?);
                        }
                        "Friday" => {
                            builder = builder.set_friday(crate::protocol_serde::shape_daily_auto_scaling_schedule::de_daily_auto_scaling_schedule(
                                tokens,
                            )?);
                        }
                        "Saturday" => {
                            builder = builder.set_saturday(crate::protocol_serde::shape_daily_auto_scaling_schedule::de_daily_auto_scaling_schedule(
                                tokens,
                            )?);
                        }
                        "Sunday" => {
                            builder = builder.set_sunday(crate::protocol_serde::shape_daily_auto_scaling_schedule::de_daily_auto_scaling_schedule(
                                tokens,
                            )?);
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
