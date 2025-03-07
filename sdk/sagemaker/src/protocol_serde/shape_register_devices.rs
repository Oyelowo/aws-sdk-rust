// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_register_devices_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::register_devices::RegisterDevicesOutput, crate::operation::register_devices::RegisterDevicesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::register_devices::RegisterDevicesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::register_devices::RegisterDevicesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceLimitExceeded" => crate::operation::register_devices::RegisterDevicesError::ResourceLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceLimitExceededBuilder::default();
                output = crate::protocol_serde::shape_resource_limit_exceeded::de_resource_limit_exceeded_json_err(_response_body, output)
                    .map_err(crate::operation::register_devices::RegisterDevicesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::register_devices::RegisterDevicesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_register_devices_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::register_devices::RegisterDevicesOutput, crate::operation::register_devices::RegisterDevicesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::register_devices::builders::RegisterDevicesOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_register_devices_input(
    input: &crate::operation::register_devices::RegisterDevicesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_register_devices_input::ser_register_devices_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
