use clap::Parser;
use std::path::PathBuf;

mod directory;
mod python_module;
mod util;

use python_module::{find_python_modules, look_for_circular_imports};

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

    let modules = find_python_modules(&input_path, project_prefix);

    let circular_imports = look_for_circular_imports(modules);

    if !circular_imports.is_empty() {
        println!("\u{274C} Circular imports were found: \n");
        for pair in circular_imports {
            println!("'{}' and '{}' import each other.", pair.0, pair.1);
        }
    } else {
        println!("\u{2705} No circular imports were found.")
    }

    Ok(())
}
