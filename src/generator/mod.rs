use std::path::Path;

use openapiv3::OpenAPI;
use anyhow::{Result};

pub mod format;
pub mod world;

use crate::PageSize;

#[derive(Debug, Clone, Copy)]
pub struct PdfGenerationOptions {
    pub page_size: PageSize,
    pub include_schemas: bool,
    pub include_examples: bool,
}

pub fn generate_pdf(spec: &OpenAPI, output: &Path, options: PdfGenerationOptions) -> Result<()> {
    Ok(())
}
