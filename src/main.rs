use clap::Parser;
use std::path::PathBuf;

mod parser;

#[derive(Parser, Debug)]
#[command(name = "oxydoc")]
#[command(version = "0.1.0")]
#[command(about = "Convert OpenAPI Specification to PDF", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    #[arg(short, long, default_value = "a4")]
    page_size: String,

    #[arg(long)]
    include_schemas: bool,

    #[arg(long)]
    include_examples: bool,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    println!("Input : {:?}", args.input);
    println!("Output : {:?}", args.output);
    println!("Page Size : {}", args.page_size);
    println!("Include Schemas : {}", args.include_schemas);
    println!("Include Examples : {}", args.include_examples);
    println!("Verbose : {}", args.verbose);

    Ok(())
}
