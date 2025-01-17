// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteIntegration`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`integration_arn(impl Into<String>)`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::integration_arn) / [`set_integration_arn(Option<String>)`](crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::set_integration_arn):<br>required: **true**<br><p>The unique identifier of the integration to delete.</p><br>
    /// - On success, responds with [`DeleteIntegrationOutput`](crate::operation::delete_integration::DeleteIntegrationOutput) with field(s):
    ///   - [`integration_arn(Option<String>)`](crate::operation::delete_integration::DeleteIntegrationOutput::integration_arn): <p>The Amazon Resource Name (ARN) of the integration.</p>
    ///   - [`integration_name(Option<String>)`](crate::operation::delete_integration::DeleteIntegrationOutput::integration_name): <p>The name of the integration.</p>
    ///   - [`source_arn(Option<String>)`](crate::operation::delete_integration::DeleteIntegrationOutput::source_arn): <p>The Amazon Resource Name (ARN) of the database used as the source for replication.</p>
    ///   - [`target_arn(Option<String>)`](crate::operation::delete_integration::DeleteIntegrationOutput::target_arn): <p>The Amazon Resource Name (ARN) of the Amazon Redshift data warehouse to use as the target for replication.</p>
    ///   - [`status(Option<ZeroEtlIntegrationStatus>)`](crate::operation::delete_integration::DeleteIntegrationOutput::status): <p>The current status of the integration.</p>
    ///   - [`errors(Option<Vec::<IntegrationError>>)`](crate::operation::delete_integration::DeleteIntegrationOutput::errors): <p>Any errors associated with the integration.</p>
    ///   - [`create_time(Option<DateTime>)`](crate::operation::delete_integration::DeleteIntegrationOutput::create_time): <p>The time (UTC) when the integration was created.</p>
    ///   - [`description(Option<String>)`](crate::operation::delete_integration::DeleteIntegrationOutput::description): <p>The description of the integration.</p>
    ///   - [`kms_key_id(Option<String>)`](crate::operation::delete_integration::DeleteIntegrationOutput::kms_key_id): <p>The Key Management Service (KMS) key identifier for the key used to encrypt the integration.</p>
    ///   - [`additional_encryption_context(Option<HashMap::<String, String>>)`](crate::operation::delete_integration::DeleteIntegrationOutput::additional_encryption_context): <p>The encryption context for the integration. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    ///   - [`tags(Option<Vec::<Tag>>)`](crate::operation::delete_integration::DeleteIntegrationOutput::tags): <p>The list of tags associated with the integration.</p>
    /// - On failure, responds with [`SdkError<DeleteIntegrationError>`](crate::operation::delete_integration::DeleteIntegrationError)
    pub fn delete_integration(&self) -> crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder {
        crate::operation::delete_integration::builders::DeleteIntegrationFluentBuilder::new(self.handle.clone())
    }
}
