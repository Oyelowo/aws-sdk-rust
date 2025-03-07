// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_optimization_job_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_optimization_job::CreateOptimizationJobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.optimization_job_name {
        object.key("OptimizationJobName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.role_arn {
        object.key("RoleArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.model_source {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ModelSource").start_object();
        crate::protocol_serde::shape_optimization_job_model_source::ser_optimization_job_model_source(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.deployment_instance_type {
        object.key("DeploymentInstanceType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.optimization_environment {
        #[allow(unused_mut)]
        let mut object_7 = object.key("OptimizationEnvironment").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.optimization_configs {
        let mut array_11 = object.key("OptimizationConfigs").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_optimization_config::ser_optimization_config(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.output_config {
        #[allow(unused_mut)]
        let mut object_15 = object.key("OutputConfig").start_object();
        crate::protocol_serde::shape_optimization_job_output_config::ser_optimization_job_output_config(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.stopping_condition {
        #[allow(unused_mut)]
        let mut object_17 = object.key("StoppingCondition").start_object();
        crate::protocol_serde::shape_stopping_condition::ser_stopping_condition(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.tags {
        let mut array_19 = object.key("Tags").start_array();
        for item_20 in var_18 {
            {
                #[allow(unused_mut)]
                let mut object_21 = array_19.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_21, item_20)?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    if let Some(var_22) = &input.vpc_config {
        #[allow(unused_mut)]
        let mut object_23 = object.key("VpcConfig").start_object();
        crate::protocol_serde::shape_optimization_vpc_config::ser_optimization_vpc_config(&mut object_23, var_22)?;
        object_23.finish();
    }
    Ok(())
}
