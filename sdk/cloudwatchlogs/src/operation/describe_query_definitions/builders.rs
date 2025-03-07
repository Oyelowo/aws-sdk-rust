// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_query_definitions::_describe_query_definitions_output::DescribeQueryDefinitionsOutputBuilder;

pub use crate::operation::describe_query_definitions::_describe_query_definitions_input::DescribeQueryDefinitionsInputBuilder;

impl crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_query_definitions::DescribeQueryDefinitionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_query_definitions::DescribeQueryDefinitionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_query_definitions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeQueryDefinitions`.
///
/// <p>This operation returns a paginated list of your saved CloudWatch Logs Insights query definitions. You can retrieve query definitions from the current account or from a source account that is linked to the current account.</p>
/// <p>You can use the <code>queryDefinitionNamePrefix</code> parameter to limit the results to only the query definitions that have names that start with a certain string.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeQueryDefinitionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_query_definitions::DescribeQueryDefinitionsOutput,
        crate::operation::describe_query_definitions::DescribeQueryDefinitionsError,
    > for DescribeQueryDefinitionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_query_definitions::DescribeQueryDefinitionsOutput,
            crate::operation::describe_query_definitions::DescribeQueryDefinitionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeQueryDefinitionsFluentBuilder {
    /// Creates a new `DescribeQueryDefinitionsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeQueryDefinitions as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsInputBuilder {
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
        crate::operation::describe_query_definitions::DescribeQueryDefinitionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_query_definitions::DescribeQueryDefinitionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_query_definitions::DescribeQueryDefinitions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_query_definitions::DescribeQueryDefinitions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_query_definitions::DescribeQueryDefinitionsOutput,
        crate::operation::describe_query_definitions::DescribeQueryDefinitionsError,
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
    /// <p>The query language used for this query. For more information about the query languages that CloudWatch Logs supports, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_AnalyzeLogData_Languages.html">Supported query languages</a>.</p>
    pub fn query_language(mut self, input: crate::types::QueryLanguage) -> Self {
        self.inner = self.inner.query_language(input);
        self
    }
    /// <p>The query language used for this query. For more information about the query languages that CloudWatch Logs supports, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_AnalyzeLogData_Languages.html">Supported query languages</a>.</p>
    pub fn set_query_language(mut self, input: ::std::option::Option<crate::types::QueryLanguage>) -> Self {
        self.inner = self.inner.set_query_language(input);
        self
    }
    /// <p>The query language used for this query. For more information about the query languages that CloudWatch Logs supports, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_AnalyzeLogData_Languages.html">Supported query languages</a>.</p>
    pub fn get_query_language(&self) -> &::std::option::Option<crate::types::QueryLanguage> {
        self.inner.get_query_language()
    }
    /// <p>Use this parameter to filter your results to only the query definitions that have names that start with the prefix you specify.</p>
    pub fn query_definition_name_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_definition_name_prefix(input.into());
        self
    }
    /// <p>Use this parameter to filter your results to only the query definitions that have names that start with the prefix you specify.</p>
    pub fn set_query_definition_name_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_definition_name_prefix(input);
        self
    }
    /// <p>Use this parameter to filter your results to only the query definitions that have names that start with the prefix you specify.</p>
    pub fn get_query_definition_name_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_definition_name_prefix()
    }
    /// <p>Limits the number of returned query definitions to the specified number.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Limits the number of returned query definitions to the specified number.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Limits the number of returned query definitions to the specified number.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
