use std::{fs, path::Path};

use anyhow::{Context, Result};
use openapiv3::OpenAPI;

pub fn parse_openapi_spec(path: &Path) -> Result<OpenAPI> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();

    let openapi: OpenAPI = match extension.to_lowercase().as_str() {
        "json" => serde_json::from_str(&content).with_context(|| "Failed to parse JSON")?,
        "yaml" | "yml" => serde_yaml::from_str(&content).with_context(|| "Failed to parse YAML")?,
        _ => {
            serde_json::from_str(&content)
                .or_else(|_| serde_yaml::from_str(&content))
                .with_context(|| "Failed to parse file (not valid JSON or YAML)")?
        }
    };

    Ok(openapi)
}
