mod generator;
mod parser;

use std::fmt::Display;

pub use generator::{PdfGenerationOptions, generate_pdf};
pub use parser::parse_openapi_spec;

/// Supported PDF page sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
#[non_exhaustive]
pub enum PageSize {
    /// A4 page size (210mm x 297mm)
    #[default]
    A4,

    /// US Letter page size (8.5in x 11in)
    Letter,
}

impl Display for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A4 => write!(f, "a4"),
            Self::Letter => write!(f, "letter"),
        }
    }
}
