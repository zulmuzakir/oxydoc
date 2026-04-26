use anyhow::Ok;
use clap::Parser;
use std::{default, fmt::Display, path::PathBuf};
use tracing_subscriber::{EnvFilter, fmt};

mod parser;
use parser::parse_openapi_spec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
#[non_exhaustive]
pub enum PageSize {
    // a4 size (210 x 297mm)
    #[default]
    A4,
    // letter size (8.5 x 11 in)
    Letter,
}

impl Display for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageSize::A4 => write!(f, "a4"),
            PageSize::Letter => write!(f, "letter"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "oxydoc", version = clap::crate_version!())]
#[command(about = "Convert OpenAPI/Swagger specifications to PDF", long_about = None)]
struct Cli {
    // Input OpenAPI File (JSON or YAML)
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    // Output PDF file
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    // page size (a4 or letter)
    #[arg(short, long, default_value = "a4")]
    page_size: PageSize,

    // include schema definitions in the output
    #[arg(long)]
    include_schemas: bool,

    // include request/response examples
    #[arg(long)]
    include_examples: bool,

    // verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args: Cli = Cli::parse();

    // initialize tracing subscriber
    let filter: EnvFilter = if args.verbose {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::new("info")
    };
    fmt().with_env_filter(filter).init();

    tracing::info!("Parsing OpenAPI spec: {:?}", args.input);
    let spec = parse_openapi_spec(&args.input)?;

    tracing::info!("Successfully parsed OpenAPI spec!");
    tracing::info!("Title: {}", spec.info.title);
    tracing::info!("Version: {}", spec.info.version);

    let path_count = spec.paths.paths.len();
    tracing::info!("Number of endpoints: {}", path_count);

    tracing::info!("\nOutput would be saved to: {:?}", args.output);
    tracing::info!("(PDF generation coming in phase 4)");

    Ok(())
}
