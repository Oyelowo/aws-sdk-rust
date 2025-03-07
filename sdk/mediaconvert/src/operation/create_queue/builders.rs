// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_queue::_create_queue_output::CreateQueueOutputBuilder;

pub use crate::operation::create_queue::_create_queue_input::CreateQueueInputBuilder;

impl crate::operation::create_queue::builders::CreateQueueInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_queue::CreateQueueOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_queue::CreateQueueError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_queue();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateQueue`.
///
/// Create a new transcoding queue. For information about queues, see Working With Queues in the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateQueueFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_queue::builders::CreateQueueInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_queue::CreateQueueOutput,
        crate::operation::create_queue::CreateQueueError,
    > for CreateQueueFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_queue::CreateQueueOutput,
            crate::operation::create_queue::CreateQueueError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateQueueFluentBuilder {
    /// Creates a new `CreateQueueFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateQueue as a reference.
    pub fn as_input(&self) -> &crate::operation::create_queue::builders::CreateQueueInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_queue::CreateQueueOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_queue::CreateQueueError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_queue::CreateQueue::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_queue::CreateQueue::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_queue::CreateQueueOutput,
        crate::operation::create_queue::CreateQueueError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// Specify the maximum number of jobs your queue can process concurrently. For on-demand queues, the value you enter is constrained by your service quotas for Maximum concurrent jobs, per on-demand queue and Maximum concurrent jobs, per account. For reserved queues, specify the number of jobs you can process concurrently in your reservation plan instead.
    pub fn concurrent_jobs(mut self, input: i32) -> Self {
        self.inner = self.inner.concurrent_jobs(input);
        self
    }
    /// Specify the maximum number of jobs your queue can process concurrently. For on-demand queues, the value you enter is constrained by your service quotas for Maximum concurrent jobs, per on-demand queue and Maximum concurrent jobs, per account. For reserved queues, specify the number of jobs you can process concurrently in your reservation plan instead.
    pub fn set_concurrent_jobs(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_concurrent_jobs(input);
        self
    }
    /// Specify the maximum number of jobs your queue can process concurrently. For on-demand queues, the value you enter is constrained by your service quotas for Maximum concurrent jobs, per on-demand queue and Maximum concurrent jobs, per account. For reserved queues, specify the number of jobs you can process concurrently in your reservation plan instead.
    pub fn get_concurrent_jobs(&self) -> &::std::option::Option<i32> {
        self.inner.get_concurrent_jobs()
    }
    /// Optional. A description of the queue that you are creating.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// Optional. A description of the queue that you are creating.
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Optional. A description of the queue that you are creating.
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// The name of the queue that you are creating.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// The name of the queue that you are creating.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// The name of the queue that you are creating.
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand.
    pub fn pricing_plan(mut self, input: crate::types::PricingPlan) -> Self {
        self.inner = self.inner.pricing_plan(input);
        self
    }
    /// Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand.
    pub fn set_pricing_plan(mut self, input: ::std::option::Option<crate::types::PricingPlan>) -> Self {
        self.inner = self.inner.set_pricing_plan(input);
        self
    }
    /// Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand.
    pub fn get_pricing_plan(&self) -> &::std::option::Option<crate::types::PricingPlan> {
        self.inner.get_pricing_plan()
    }
    /// Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.
    pub fn reservation_plan_settings(mut self, input: crate::types::ReservationPlanSettings) -> Self {
        self.inner = self.inner.reservation_plan_settings(input);
        self
    }
    /// Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.
    pub fn set_reservation_plan_settings(mut self, input: ::std::option::Option<crate::types::ReservationPlanSettings>) -> Self {
        self.inner = self.inner.set_reservation_plan_settings(input);
        self
    }
    /// Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.
    pub fn get_reservation_plan_settings(&self) -> &::std::option::Option<crate::types::ReservationPlanSettings> {
        self.inner.get_reservation_plan_settings()
    }
    /// Initial state of the queue. If you create a paused queue, then jobs in that queue won't begin.
    pub fn status(mut self, input: crate::types::QueueStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// Initial state of the queue. If you create a paused queue, then jobs in that queue won't begin.
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::QueueStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// Initial state of the queue. If you create a paused queue, then jobs in that queue won't begin.
    pub fn get_status(&self) -> &::std::option::Option<crate::types::QueueStatus> {
        self.inner.get_status()
    }
    ///
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
