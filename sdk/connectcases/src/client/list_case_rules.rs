// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCaseRules`](crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_id(impl Into<String>)`](crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder::domain_id) / [`set_domain_id(Option<String>)`](crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder::set_domain_id):<br>required: **true**<br><p>Unique identifier of a Cases domain.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return per page.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p><br>
    /// - On success, responds with [`ListCaseRulesOutput`](crate::operation::list_case_rules::ListCaseRulesOutput) with field(s):
    ///   - [`case_rules(Vec::<CaseRuleSummary>)`](crate::operation::list_case_rules::ListCaseRulesOutput::case_rules): <p>A list of field summary objects.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_case_rules::ListCaseRulesOutput::next_token): <p>The token for the next set of results. This is null if there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListCaseRulesError>`](crate::operation::list_case_rules::ListCaseRulesError)
    pub fn list_case_rules(&self) -> crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder {
        crate::operation::list_case_rules::builders::ListCaseRulesFluentBuilder::new(self.handle.clone())
    }
}
