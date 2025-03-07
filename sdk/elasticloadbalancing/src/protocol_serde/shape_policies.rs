// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_policies(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::Policies, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Policies::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AppCookieStickinessPolicies") /* AppCookieStickinessPolicies com.amazonaws.elasticloadbalancing#Policies$AppCookieStickinessPolicies */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_app_cookie_stickiness_policies::de_app_cookie_stickiness_policies(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_app_cookie_stickiness_policies(var_1);
            }
            ,
            s if s.matches("LBCookieStickinessPolicies") /* LBCookieStickinessPolicies com.amazonaws.elasticloadbalancing#Policies$LBCookieStickinessPolicies */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_lb_cookie_stickiness_policies::de_lb_cookie_stickiness_policies(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_lb_cookie_stickiness_policies(var_2);
            }
            ,
            s if s.matches("OtherPolicies") /* OtherPolicies com.amazonaws.elasticloadbalancing#Policies$OtherPolicies */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_policy_names::de_policy_names(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_other_policies(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
