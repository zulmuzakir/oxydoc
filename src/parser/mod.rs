use std::{fs, path::Path};

use anyhow::{Context, Result};
use openapiv3::OpenAPI;

pub fn parse_openapi_spec(path: &Path) -> Result<OpenAPI> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    Ok(OpenAPI::default())
}
