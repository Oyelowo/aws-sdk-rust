// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_predictor_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::describe_predictor::DescribePredictorOutput, crate::operation::describe_predictor::DescribePredictorError>
{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_predictor::DescribePredictorError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_predictor::DescribePredictorError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInputException" => crate::operation::describe_predictor::DescribePredictorError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_predictor::DescribePredictorError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::describe_predictor::DescribePredictorError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_predictor::DescribePredictorError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_predictor::DescribePredictorError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_predictor_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::describe_predictor::DescribePredictorOutput, crate::operation::describe_predictor::DescribePredictorError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_predictor::builders::DescribePredictorOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_predictor::de_describe_predictor(_response_body, output)
            .map_err(crate::operation::describe_predictor::DescribePredictorError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_describe_predictor_input(
    input: &crate::operation::describe_predictor::DescribePredictorInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_predictor_input::ser_describe_predictor_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_describe_predictor(
    value: &[u8],
    mut builder: crate::operation::describe_predictor::builders::DescribePredictorOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_predictor::builders::DescribePredictorOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "PredictorArn" => {
                    builder = builder.set_predictor_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "PredictorName" => {
                    builder = builder.set_predictor_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "AlgorithmArn" => {
                    builder = builder.set_algorithm_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "AutoMLAlgorithmArns" => {
                    builder = builder.set_auto_ml_algorithm_arns(crate::protocol_serde::shape_arn_list::de_arn_list(tokens)?);
                }
                "ForecastHorizon" => {
                    builder = builder.set_forecast_horizon(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i32::try_from)
                            .transpose()?,
                    );
                }
                "ForecastTypes" => {
                    builder = builder.set_forecast_types(crate::protocol_serde::shape_forecast_types::de_forecast_types(tokens)?);
                }
                "PerformAutoML" => {
                    builder = builder.set_perform_auto_ml(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                }
                "AutoMLOverrideStrategy" => {
                    builder = builder.set_auto_ml_override_strategy(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::AutoMlOverrideStrategy::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "PerformHPO" => {
                    builder = builder.set_perform_hpo(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                }
                "TrainingParameters" => {
                    builder = builder.set_training_parameters(crate::protocol_serde::shape_training_parameters::de_training_parameters(tokens)?);
                }
                "EvaluationParameters" => {
                    builder =
                        builder.set_evaluation_parameters(crate::protocol_serde::shape_evaluation_parameters::de_evaluation_parameters(tokens)?);
                }
                "HPOConfig" => {
                    builder = builder.set_hpo_config(
                        crate::protocol_serde::shape_hyper_parameter_tuning_job_config::de_hyper_parameter_tuning_job_config(tokens)?,
                    );
                }
                "InputDataConfig" => {
                    builder = builder.set_input_data_config(crate::protocol_serde::shape_input_data_config::de_input_data_config(tokens)?);
                }
                "FeaturizationConfig" => {
                    builder = builder.set_featurization_config(crate::protocol_serde::shape_featurization_config::de_featurization_config(tokens)?);
                }
                "EncryptionConfig" => {
                    builder = builder.set_encryption_config(crate::protocol_serde::shape_encryption_config::de_encryption_config(tokens)?);
                }
                "PredictorExecutionDetails" => {
                    builder = builder.set_predictor_execution_details(
                        crate::protocol_serde::shape_predictor_execution_details::de_predictor_execution_details(tokens)?,
                    );
                }
                "EstimatedTimeRemainingInMinutes" => {
                    builder = builder.set_estimated_time_remaining_in_minutes(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i64::try_from)
                            .transpose()?,
                    );
                }
                "IsAutoPredictor" => {
                    builder = builder.set_is_auto_predictor(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                }
                "DatasetImportJobArns" => {
                    builder = builder.set_dataset_import_job_arns(crate::protocol_serde::shape_arn_list::de_arn_list(tokens)?);
                }
                "Status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Message" => {
                    builder = builder.set_message(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "CreationTime" => {
                    builder = builder.set_creation_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "LastModificationTime" => {
                    builder = builder.set_last_modification_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "OptimizationMetric" => {
                    builder = builder.set_optimization_metric(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::OptimizationMetric::from(u.as_ref())))
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
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
