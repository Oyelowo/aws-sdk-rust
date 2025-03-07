// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_agent_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_agent::CreateAgentInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.activation_key {
        object.key("ActivationKey").string(var_1.as_str());
    }
    if let Some(var_2) = &input.agent_name {
        object.key("AgentName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tags {
        let mut array_4 = object.key("Tags").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_tag_list_entry::ser_tag_list_entry(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.vpc_endpoint_id {
        object.key("VpcEndpointId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.subnet_arns {
        let mut array_9 = object.key("SubnetArns").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.security_group_arns {
        let mut array_12 = object.key("SecurityGroupArns").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    Ok(())
}
