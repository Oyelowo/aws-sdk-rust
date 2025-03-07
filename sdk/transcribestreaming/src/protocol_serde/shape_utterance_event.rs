// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_utterance_event_payload(
    input: &[u8],
) -> ::std::result::Result<crate::types::UtteranceEvent, ::aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(input)).peekable();
    let tokens = &mut tokens_owned;
    let result = crate::protocol_serde::shape_utterance_event::de_utterance_event(tokens)?
        .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    result
}

pub(crate) fn de_utterance_event<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::UtteranceEvent>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::UtteranceEventBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "UtteranceId" => {
                            builder = builder.set_utterance_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "IsPartial" => {
                            builder = builder.set_is_partial(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "ParticipantRole" => {
                            builder = builder.set_participant_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ParticipantRole::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "BeginOffsetMillis" => {
                            builder = builder.set_begin_offset_millis(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "EndOffsetMillis" => {
                            builder = builder.set_end_offset_millis(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "Transcript" => {
                            builder = builder.set_transcript(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Items" => {
                            builder = builder.set_items(crate::protocol_serde::shape_call_analytics_item_list::de_call_analytics_item_list(
                                tokens,
                            )?);
                        }
                        "Entities" => {
                            builder = builder.set_entities(crate::protocol_serde::shape_call_analytics_entity_list::de_call_analytics_entity_list(
                                tokens,
                            )?);
                        }
                        "Sentiment" => {
                            builder = builder.set_sentiment(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Sentiment::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "IssuesDetected" => {
                            builder = builder.set_issues_detected(crate::protocol_serde::shape_issues_detected::de_issues_detected(tokens)?);
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
