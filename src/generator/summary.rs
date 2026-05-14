use openapiv3::{MediaType, Operation, Parameter, ReferenceOr, Schema, SchemaKind, Type};

use crate::generator::format::truncate;

pub fn parameter_summary(parameter: &ReferenceOr<Parameter>) -> String {
    match parameter {
        ReferenceOr::Reference { reference } => format!("Reference parameter: {reference}"),
        ReferenceOr::Item(parameter) => {
            let location = match parameter {
                Parameter::Query { .. } => "query",
                Parameter::Header { .. } => "header",
                Parameter::Path { .. } => "path",
                Parameter::Cookie { .. } => "cookie",
            };

            let data = parameter.parameter_data_ref();
            let mut summary = format!(
                "{} ({location}{})",
                data.name,
                if data.required { ", required" } else { "" }
            );

            if let Some(description) = data.description.as_deref() {
                summary.push_str(": ");
                summary.push_str(description.trim());
            }

            summary
        }
    }
}

pub fn media_type_summary(media_type: &MediaType) -> String {
    match &media_type.schema {
        Some(ReferenceOr::Reference { reference }) => format!("schema {reference}"),
        Some(ReferenceOr::Item(schema)) => format!("schema {}", schema_kind_summary(schema)),
        None => "no schema".to_owned(),
    }
}

pub fn media_type_examples(media_type: &MediaType) -> Vec<String> {
    let mut examples = Vec::new();

    if let Some(example) = &media_type.example {
        examples.push(truncate(&json_value_summary(example), 240));
    }

    for (name, example) in &media_type.examples {
        let summary = match example {
            ReferenceOr::Reference { reference } => format!("{name}: reference {reference}"),
            ReferenceOr::Item(example) => {
                let rendered = example
                    .value
                    .as_ref()
                    .map_or_else(|| "no inline value".to_owned(), json_value_summary);
                format!("{name}: {}", truncate(&rendered, 240))
            }
        };
        examples.push(summary);
    }

    examples
}

pub fn schema_kind_summary(schema: &Schema) -> String {
    match &schema.schema_kind {
        SchemaKind::Type(Type::String(_)) => "string".to_owned(),
        SchemaKind::Type(Type::Number(_)) => "number".to_owned(),
        SchemaKind::Type(Type::Integer(_)) => "integer".to_owned(),
        SchemaKind::Type(Type::Boolean(_)) => "boolean".to_owned(),
        SchemaKind::Type(Type::Array(_)) => "array".to_owned(),
        SchemaKind::Type(Type::Object(object)) => format!(
            "object with {} properties{}",
            object.properties.len(),
            if object.required.is_empty() {
                String::new()
            } else {
                format!(", {} required", object.required.len())
            }
        ),
        SchemaKind::OneOf { one_of } => format!("oneOf ({})", one_of.len()),
        SchemaKind::AllOf { all_of } => format!("allOf ({})", all_of.len()),
        SchemaKind::AnyOf { any_of } => format!("anyOf ({})", any_of.len()),
        SchemaKind::Not { .. } => "not".to_owned(),
        SchemaKind::Any(_) => "any".to_owned(),
    }
}

pub fn primary_tag(operation: &Operation) -> &str {
    operation.tags.first().map_or("General", String::as_str)
}

pub fn json_value_summary(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(text) => text.clone(),
        _ => serde_json::to_string(value).unwrap_or_else(|_| "<invalid json>".to_owned()),
    }
}
