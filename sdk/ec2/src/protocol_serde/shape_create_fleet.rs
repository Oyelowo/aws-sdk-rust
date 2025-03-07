// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_fleet_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_fleet::CreateFleetOutput, crate::operation::create_fleet::CreateFleetError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_fleet::CreateFleetError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::create_fleet::CreateFleetError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_fleet_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_fleet::CreateFleetOutput, crate::operation::create_fleet::CreateFleetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_fleet::builders::CreateFleetOutputBuilder::default();
        output = crate::protocol_serde::shape_create_fleet::de_create_fleet(_response_body, output)
            .map_err(crate::operation::create_fleet::CreateFleetError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_fleet(
    inp: &[u8],
    mut builder: crate::operation::create_fleet::builders::CreateFleetOutputBuilder,
) -> std::result::Result<crate::operation::create_fleet::builders::CreateFleetOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateFleetResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateFleetResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("fleetId") /* FleetId com.amazonaws.ec2.synthetic#CreateFleetOutput$FleetId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_fleet_id(var_1);
            }
            ,
            s if s.matches("errorSet") /* Errors com.amazonaws.ec2.synthetic#CreateFleetOutput$Errors */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_create_fleet_errors_set::de_create_fleet_errors_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_errors(var_2);
            }
            ,
            s if s.matches("fleetInstanceSet") /* Instances com.amazonaws.ec2.synthetic#CreateFleetOutput$Instances */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_create_fleet_instances_set::de_create_fleet_instances_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instances(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
