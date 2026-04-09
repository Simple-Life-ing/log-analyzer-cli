use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct AnalyzerCLI {
    /// Path to the log file to analyze
    #[arg(short, long)]
    pub filepath: String,

    #[arg(short, long)]
    pub layer: String,

    #[arg(short, long)]
    pub keyword: String,
}

impl AnalyzerCLI {
    pub fn parse_args() -> Self {
        AnalyzerCLI::parse()
    }
}
