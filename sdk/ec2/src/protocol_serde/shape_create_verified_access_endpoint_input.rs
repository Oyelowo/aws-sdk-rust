// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_verified_access_endpoint_input_input_input(
    input: &crate::operation::create_verified_access_endpoint::CreateVerifiedAccessEndpointInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateVerifiedAccessEndpoint", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("VerifiedAccessGroupId");
    if let Some(var_2) = &input.verified_access_group_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("EndpointType");
    if let Some(var_4) = &input.endpoint_type {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AttachmentType");
    if let Some(var_6) = &input.attachment_type {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DomainCertificateArn");
    if let Some(var_8) = &input.domain_certificate_arn {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("ApplicationDomain");
    if let Some(var_10) = &input.application_domain {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("EndpointDomainPrefix");
    if let Some(var_12) = &input.endpoint_domain_prefix {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("SecurityGroupId");
    if let Some(var_14) = &input.security_group_ids {
        if !var_14.is_empty() {
            let mut list_16 = scope_13.start_list(true, Some("item"));
            for item_15 in var_14 {
                #[allow(unused_mut)]
                let mut entry_17 = list_16.entry();
                entry_17.string(item_15);
            }
            list_16.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("LoadBalancerOptions");
    if let Some(var_19) = &input.load_balancer_options {
        crate::protocol_serde::shape_create_verified_access_endpoint_load_balancer_options::ser_create_verified_access_endpoint_load_balancer_options(scope_18, var_19)?;
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("NetworkInterfaceOptions");
    if let Some(var_21) = &input.network_interface_options {
        crate::protocol_serde::shape_create_verified_access_endpoint_eni_options::ser_create_verified_access_endpoint_eni_options(scope_20, var_21)?;
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("Description");
    if let Some(var_23) = &input.description {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("PolicyDocument");
    if let Some(var_25) = &input.policy_document {
        scope_24.string(var_25);
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("TagSpecification");
    if let Some(var_27) = &input.tag_specifications {
        if !var_27.is_empty() {
            let mut list_29 = scope_26.start_list(true, Some("item"));
            for item_28 in var_27 {
                #[allow(unused_mut)]
                let mut entry_30 = list_29.entry();
                crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_30, item_28)?;
            }
            list_29.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("ClientToken");
    if let Some(var_32) = &input.client_token {
        scope_31.string(var_32);
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("DryRun");
    if let Some(var_34) = &input.dry_run {
        scope_33.boolean(*var_34);
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("SseSpecification");
    if let Some(var_36) = &input.sse_specification {
        crate::protocol_serde::shape_verified_access_sse_specification_request::ser_verified_access_sse_specification_request(scope_35, var_36)?;
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("RdsOptions");
    if let Some(var_38) = &input.rds_options {
        crate::protocol_serde::shape_create_verified_access_endpoint_rds_options::ser_create_verified_access_endpoint_rds_options(scope_37, var_38)?;
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("CidrOptions");
    if let Some(var_40) = &input.cidr_options {
        crate::protocol_serde::shape_create_verified_access_endpoint_cidr_options::ser_create_verified_access_endpoint_cidr_options(
            scope_39, var_40,
        )?;
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
