// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCustomEndpoint`](crate::operation::get_custom_endpoint::builders::GetCustomEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_custom_endpoint::builders::GetCustomEndpointFluentBuilder::send) it.
    /// - On success, responds with [`GetCustomEndpointOutput`](crate::operation::get_custom_endpoint::GetCustomEndpointOutput) with field(s):
    ///   - [`endpoint_address(String)`](crate::operation::get_custom_endpoint::GetCustomEndpointOutput::endpoint_address): <p>The IoT managed integrations dedicated, custom endpoint for the device to route traffic through.</p>
    /// - On failure, responds with [`SdkError<GetCustomEndpointError>`](crate::operation::get_custom_endpoint::GetCustomEndpointError)
    pub fn get_custom_endpoint(&self) -> crate::operation::get_custom_endpoint::builders::GetCustomEndpointFluentBuilder {
        crate::operation::get_custom_endpoint::builders::GetCustomEndpointFluentBuilder::new(self.handle.clone())
    }
}
