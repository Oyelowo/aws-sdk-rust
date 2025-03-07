// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_connection::_create_connection_output::CreateConnectionOutputBuilder;

pub use crate::operation::create_connection::_create_connection_input::CreateConnectionInputBuilder;

impl crate::operation::create_connection::builders::CreateConnectionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_connection::CreateConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_connection::CreateConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_connection();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateConnection`.
///
/// <p>Creates a new connection. In Amazon DataZone, a connection enables you to connect your resources (domains, projects, and environments) to external resources and services.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_connection::builders::CreateConnectionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_connection::CreateConnectionOutput,
        crate::operation::create_connection::CreateConnectionError,
    > for CreateConnectionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_connection::CreateConnectionOutput,
            crate::operation::create_connection::CreateConnectionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateConnectionFluentBuilder {
    /// Creates a new `CreateConnectionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateConnection as a reference.
    pub fn as_input(&self) -> &crate::operation::create_connection::builders::CreateConnectionInputBuilder {
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
        crate::operation::create_connection::CreateConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_connection::CreateConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_connection::CreateConnection::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_connection::CreateConnection::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_connection::CreateConnectionOutput,
        crate::operation::create_connection::CreateConnectionError,
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
    /// <p>The location where the connection is created.</p>
    pub fn aws_location(mut self, input: crate::types::AwsLocation) -> Self {
        self.inner = self.inner.aws_location(input);
        self
    }
    /// <p>The location where the connection is created.</p>
    pub fn set_aws_location(mut self, input: ::std::option::Option<crate::types::AwsLocation>) -> Self {
        self.inner = self.inner.set_aws_location(input);
        self
    }
    /// <p>The location where the connection is created.</p>
    pub fn get_aws_location(&self) -> &::std::option::Option<crate::types::AwsLocation> {
        self.inner.get_aws_location()
    }
    /// <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that is provided to ensure the idempotency of the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>A connection description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A connection description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A connection description.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The ID of the domain where the connection is created.</p>
    pub fn domain_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_identifier(input.into());
        self
    }
    /// <p>The ID of the domain where the connection is created.</p>
    pub fn set_domain_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_identifier(input);
        self
    }
    /// <p>The ID of the domain where the connection is created.</p>
    pub fn get_domain_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_identifier()
    }
    /// <p>The ID of the environment where the connection is created.</p>
    pub fn environment_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_identifier(input.into());
        self
    }
    /// <p>The ID of the environment where the connection is created.</p>
    pub fn set_environment_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_identifier(input);
        self
    }
    /// <p>The ID of the environment where the connection is created.</p>
    pub fn get_environment_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_identifier()
    }
    /// <p>The connection name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The connection name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The connection name.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The connection props.</p>
    pub fn props(mut self, input: crate::types::ConnectionPropertiesInput) -> Self {
        self.inner = self.inner.props(input);
        self
    }
    /// <p>The connection props.</p>
    pub fn set_props(mut self, input: ::std::option::Option<crate::types::ConnectionPropertiesInput>) -> Self {
        self.inner = self.inner.set_props(input);
        self
    }
    /// <p>The connection props.</p>
    pub fn get_props(&self) -> &::std::option::Option<crate::types::ConnectionPropertiesInput> {
        self.inner.get_props()
    }
}
