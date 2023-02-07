use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Default, Debug)]
#[clap(author = "Jan Krecke", version)]
/// Find circular imports in a Python project
struct Arguments {
    #[clap(forbid_empty_values = true)]
    /// Path to Python project
    input_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    let input_path = args.input_path;

    return Ok(());
}
