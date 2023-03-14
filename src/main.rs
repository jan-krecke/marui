use clap::Parser;
use std::path::PathBuf;

mod directory;
mod python_module;
mod util;

use python_module::{build_import_tree, print_import_cycles};

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
    if !directory::pyproject_exists(&input_path) {
        return Err(From::from("Target directory is not a Python project."));
    }

    let project_prefix = input_path.to_str().unwrap();

    let module_graph = build_import_tree(&input_path, project_prefix);

    let circular_imports = module_graph.find_circular_imports();

    print_import_cycles(circular_imports);

    Ok(())
}
