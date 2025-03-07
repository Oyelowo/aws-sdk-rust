// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn de_dashboard_invalid_input_error_xml_err(
    inp: &[u8],
    mut builder: crate::types::error::builders::DashboardInvalidInputErrorBuilder,
) -> std::result::Result<crate::types::error::builders::DashboardInvalidInputErrorBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    if inp.is_empty() {
        return Ok(builder);
    }
    let mut document = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut error_decoder = crate::rest_xml_wrapped_errors::error_scope(&mut document)?;
    while let Some(mut tag) = error_decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("dashboardValidationMessages") /* dashboardValidationMessages com.amazonaws.cloudwatch#DashboardInvalidInputError$dashboardValidationMessages */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_dashboard_validation_messages::de_dashboard_validation_messages(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_dashboard_validation_messages(var_1);
            }
            ,
            s if s.matches("message") /* message com.amazonaws.cloudwatch#DashboardInvalidInputError$message */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_message(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
