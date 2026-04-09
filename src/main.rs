use crate::error::AnalyzerError;
use crate::parser::AnalyzerCLI;

mod error;
mod parser;
fn main() {
    let args = parser::AnalyzerCLI::parse_args();

    if let Err(e) = run_from_args(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_from_args(args: &AnalyzerCLI) -> Result<(), AnalyzerError> {
    println!(
        "Analyzing log file: {}\nLayer: {}\nKeyword: {}",
        args.filepath, args.layer, args.keyword
    );
    Ok(())
}
