use clap::Parser;
use std::path::PathBuf;

mod directory;
mod python_module;

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

    let modules = python_module::find_python_modules(input_path);
    println!("{:?}", modules);

    return Ok(());
}
