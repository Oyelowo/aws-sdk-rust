// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_vpc_encryption_control(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::VpcEncryptionControl, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VpcEncryptionControl::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2#VpcEncryptionControl$VpcId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_1);
            }
            ,
            s if s.matches("vpcEncryptionControlId") /* VpcEncryptionControlId com.amazonaws.ec2#VpcEncryptionControl$VpcEncryptionControlId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_encryption_control_id(var_2);
            }
            ,
            s if s.matches("mode") /* Mode com.amazonaws.ec2#VpcEncryptionControl$Mode */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::VpcEncryptionControlMode, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VpcEncryptionControlMode::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_mode(var_3);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#VpcEncryptionControl$State */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::VpcEncryptionControlState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VpcEncryptionControlState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_4);
            }
            ,
            s if s.matches("stateMessage") /* StateMessage com.amazonaws.ec2#VpcEncryptionControl$StateMessage */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_state_message(var_5);
            }
            ,
            s if s.matches("resourceExclusions") /* ResourceExclusions com.amazonaws.ec2#VpcEncryptionControl$ResourceExclusions */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_vpc_encryption_control_exclusions::de_vpc_encryption_control_exclusions(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_resource_exclusions(var_6);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#VpcEncryptionControl$Tags */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
