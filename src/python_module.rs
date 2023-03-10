use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use crate::util;

/// Representation of a Python module
#[derive(Debug)]
pub struct PythonModule {
    /// Name of the module
    pub name: String,
    /// List of imported modules by this module
    pub imports: Vec<String>,
}

/// Find all modules in a Python project
///
/// Calls itself recursively on any submodules in the project.
///
/// # Arguments
/// * `local_path` - Path to Python project
/// * `prefix_for_strip` - project prefix to strip from fully qualified project path
pub fn find_python_modules(local_path: &PathBuf, prefix_for_strip: &str) -> Vec<PythonModule> {
    // let local_path_string = local_path.clone().to_str().unwrap().to_owned();
    let mut modules: Vec<PythonModule> = Vec::new();

    for dir_entry in fs::read_dir(local_path).unwrap() {
        let sub_path = dir_entry.unwrap().path();

        if sub_path.is_dir() && crate::directory::path_is_not_hidden(&sub_path) {
            // check if directory is a Python module
            if crate::directory::init_file_exists(&sub_path) {
                modules.extend(find_python_modules(&sub_path, prefix_for_strip));
            }
        } else if sub_path.is_file() && crate::directory::is_python_file(&sub_path) {
            let imports = find_imports(&sub_path);

            modules.push(PythonModule {
                name: sub_path
                    .strip_prefix(prefix_for_strip)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('/', ".")
                    .replace(".py", ""),
                imports,
            })
        }
    }

    modules
}

/// Find imports in a Python file
///
/// Each string in the returned vector corresponds to one
/// import line.
///
/// # Arguments
/// * `file_path` - Path to Python file
fn find_imports(file_path: &Path) -> Vec<String> {
    let mut imports = Vec::new();
    if let Ok(file) = fs::File::open(file_path) {
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("import") || line.starts_with("from") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                imports.push(parts[1].to_owned());
            }
        }
    }
    imports
}

/// Detect circular imports in a series of Python modules
///
/// # Arguments
/// * `modules` - a list of Python modules extracted from a Python project
pub fn look_for_circular_imports(modules: Vec<PythonModule>) -> Vec<util::UnorderedPair<String>> {
    let mut circular_import_pairs = Vec::new();
    for module in &modules {
        for import in module.imports.clone() {
            if let Some(desired_module) = modules.iter().find(|module| module.name == import) {
                if desired_module.imports.contains(&module.name) {
                    let pair =
                        util::UnorderedPair(module.name.clone(), desired_module.name.clone());
                    if !circular_import_pairs.contains(&pair) {
                        circular_import_pairs.push(pair);
                    }
                }
            }
        }
    }
    circular_import_pairs
}
