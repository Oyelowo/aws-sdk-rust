// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_plugin_visual_table_query_sort(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PluginVisualTableQuerySort,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.row_sort {
        let mut array_2 = object.key("RowSort").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_field_sort_options::ser_field_sort_options(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.items_limit_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("ItemsLimitConfiguration").start_object();
        crate::protocol_serde::shape_plugin_visual_items_limit_configuration::ser_plugin_visual_items_limit_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_plugin_visual_table_query_sort<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::PluginVisualTableQuerySort>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::PluginVisualTableQuerySortBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "RowSort" => {
                            builder = builder.set_row_sort(crate::protocol_serde::shape_row_sort_list::de_row_sort_list(tokens)?);
                        }
                        "ItemsLimitConfiguration" => {
                            builder = builder.set_items_limit_configuration(
                                crate::protocol_serde::shape_plugin_visual_items_limit_configuration::de_plugin_visual_items_limit_configuration(
                                    tokens,
                                )?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
