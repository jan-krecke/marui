use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use crate::directory;
use crate::util;

/// Representation of a Python module
#[derive(Debug, Clone)]
pub struct PythonModule {
    /// Name of the module
    pub name: String,
    /// List of imported modules by this module
    pub imports: Vec<String>,
    /// ID of node within `ImportGraph`
    self_id: usize,
}
impl PythonModule {
    fn new(name: String, imports: Vec<String>, self_id: usize) -> Self {
        Self {
            name,
            imports,
            self_id,
        }
    }

    pub fn update_id(&mut self, new_id: usize) {
        self.self_id = new_id;
    }
}

/// Graph representation of import structure within Python project
#[derive(Debug)]
pub struct ImportGraph {
    /// List of modules within the Python project.
    pub modules: Vec<PythonModule>,
    /// Index of current module.
    current_module: Option<usize>,
}

impl ImportGraph {
    fn new() -> Self {
        Self {
            modules: Vec::new(),
            current_module: None,
        }
    }

    /// Add new module to the graph
    ///
    /// # Arguments
    /// * `name` - Name of the new module
    /// * `imports` - Modules imported by the new module
    fn add_module(&mut self, name: String, imports: Vec<String>) {
        let mod_id = self.modules.len();
        let pmodule = PythonModule::new(name, imports, mod_id);
        self.modules.push(pmodule);
    }

    /// Extend the graph structure with another one
    ///
    /// # Arguments
    /// * `other` - another instance of `ImportGraph` which shall be added to the current one
    fn extend(&mut self, other: ImportGraph) {
        let n_current = self.modules.len();

        for (i, module) in other.modules.iter().enumerate() {
            let mut new_module = module.clone();
            new_module.update_id(n_current + i);
            self.modules.push(new_module);
        }
    }

    /// Perform Depth-First Search (DFS), starting from a given root node
    fn dfs_recursion(&self, root_id: usize, visited_ids: &mut HashSet<usize>) -> Vec<usize> {
        let mut dfs_stack = vec![root_id];

        for import_name in &self.modules[root_id].imports {
            if let Some(import_id) = self.get_module_id(&import_name) {
                if !visited_ids.contains(&import_id) {
                    visited_ids.insert(import_id);
                    dfs_stack.extend(self.dfs_recursion(import_id, visited_ids));
                }
            }
        }
        dfs_stack
    }

    fn get_module_id(&self, target_module_name: &str) -> Option<usize> {
        self.modules
            .iter()
            .map(|module| module.name.clone())
            .position(|mod_name| mod_name == target_module_name)
    }
}

/// Find all modules in a Python project
///
/// Calls itself recursively on any submodules in the project.
///
/// # Arguments
/// * `local_path` - Path to Python project
/// * `prefix_for_strip` - project prefix to strip from fully qualified project path
pub fn build_import_tree(local_path: &PathBuf, prefix_for_strip: &str) -> ImportGraph {
    // let local_path_string = local_path.clone().to_str().unwrap().to_owned();
    let mut import_graph = ImportGraph::new();

    for dir_entry in fs::read_dir(local_path).unwrap() {
        let sub_path = dir_entry.unwrap().path();

        if sub_path.is_dir() && crate::directory::path_is_not_hidden(&sub_path) {
            // check if directory is a Python module
            if crate::directory::init_file_exists(&sub_path) {
                import_graph.extend(build_import_tree(&sub_path, prefix_for_strip));
            }
        } else if sub_path.is_file() && crate::directory::is_python_file(&sub_path) {
            let imports = find_imports(&sub_path);

            import_graph.add_module(
                directory::convert_path_to_module_id(&sub_path, prefix_for_strip),
                imports,
            )
        }
    }

    import_graph
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
