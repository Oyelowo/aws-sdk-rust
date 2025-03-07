// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_safety_rule_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_safety_rule::DescribeSafetyRuleOutput,
    crate::operation::describe_safety_rule::DescribeSafetyRuleError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_safety_rule::DescribeSafetyRuleError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_safety_rule::DescribeSafetyRuleError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::operation::describe_safety_rule::DescribeSafetyRuleError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_safety_rule::DescribeSafetyRuleError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output).build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ValidationException" => crate::operation::describe_safety_rule::DescribeSafetyRuleError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_safety_rule::DescribeSafetyRuleError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output).build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_safety_rule::DescribeSafetyRuleError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_safety_rule_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_safety_rule::DescribeSafetyRuleOutput,
    crate::operation::describe_safety_rule::DescribeSafetyRuleError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_safety_rule::builders::DescribeSafetyRuleOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_safety_rule::de_describe_safety_rule(_response_body, output)
            .map_err(crate::operation::describe_safety_rule::DescribeSafetyRuleError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_safety_rule(
    value: &[u8],
    mut builder: crate::operation::describe_safety_rule::builders::DescribeSafetyRuleOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_safety_rule::builders::DescribeSafetyRuleOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "AssertionRule" => {
                    builder = builder.set_assertion_rule(crate::protocol_serde::shape_assertion_rule::de_assertion_rule(tokens)?);
                }
                "GatingRule" => {
                    builder = builder.set_gating_rule(crate::protocol_serde::shape_gating_rule::de_gating_rule(tokens)?);
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
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
