// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_kubernetes_network_config_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::KubernetesNetworkConfigRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.service_ipv4_cidr {
        object.key("serviceIpv4Cidr").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ip_family {
        object.key("ipFamily").string(var_2.as_str());
    }
    if let Some(var_3) = &input.elastic_load_balancing {
        #[allow(unused_mut)]
        let mut object_4 = object.key("elasticLoadBalancing").start_object();
        crate::protocol_serde::shape_elastic_load_balancing::ser_elastic_load_balancing(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
