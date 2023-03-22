use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use crate::directory;

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
impl PartialEq for PythonModule {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        } else {
            for import in &self.imports {
                if !other.imports.contains(&import) {
                    return false;
                }
            }
        }

        true
    }
}

/// Graph representation of import structure within Python project
#[derive(Debug)]
pub struct ImportGraph {
    /// List of modules within the Python project.
    pub modules: Vec<PythonModule>,
}

impl ImportGraph {
    fn new() -> Self {
        Self {
            modules: Vec::new(),
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
            let new_mod_id = n_current + i;
            new_module.update_id(new_mod_id);
            self.modules.push(new_module);
        }
    }

    /// Perform Depth-First Search (DFS), starting from a given root node
    fn dfs_recursion(
        &self,
        root_id: usize,
        dfs_stack: &mut Vec<usize>,
        visited_ids: &mut Vec<usize>,
        import_cycles: &mut Vec<ImportCycle>,
    ) {
        // At the start of each visit, we push the ID of the visited
        // module to the DFS stack to mark which elements are part of
        // the current import chain
        dfs_stack.push(root_id);

        for import_name in &self.modules[root_id].imports {
            // Check if imported module is part of the project graph and---if so--- returns its ID
            if let Some(import_id) = self.get_module_id_from_name(import_name) {
                // If the imported module has not been visited yet, it will be visited now.
                if !visited_ids.contains(&import_id) {
                    visited_ids.push(import_id);
                    self.dfs_recursion(import_id, dfs_stack, visited_ids, import_cycles);
                } else {
                    // Otherwise, i.e., if the imported module has been visited already,
                    // we check if the visit has happened as part of the current DFS stack
                    if let Some(dfs_stack_id) = dfs_stack.iter().position(|v_id| *v_id == import_id)
                    {
                        // If that is the case, a circular import must have happened, and we push the
                        // import chain to the list holding all such chains
                        let mut import_cycle = ImportCycle::new();
                        for mod_id in dfs_stack[dfs_stack_id..].iter() {
                            import_cycle.add_module(self.modules[*mod_id].name.clone());
                        }
                        import_cycle.add_module(self.modules[dfs_stack[dfs_stack_id]].name.clone());
                        import_cycles.push(import_cycle);
                    }
                }
            }
        }
        // At the end of each visit, we pop the ID of the stack to indicate
        // that this module is not part of currently investigated import chain
        dfs_stack.pop();
    }

    /// Find circular imports by applying DFS to the module import tree
    pub fn find_circular_imports(&self) -> Vec<ImportCycle> {
        let mut dfs_stack = Vec::new();
        let mut visited_ids = Vec::new();
        let mut import_cycles = Vec::new();

        for mod_id in 0..self.modules.len() {
            if !visited_ids.contains(&mod_id) {
                self.dfs_recursion(mod_id, &mut dfs_stack, &mut visited_ids, &mut import_cycles);
            }
        }

        import_cycles
    }

    fn get_module_id_from_name(&self, target_module_name: &str) -> Option<usize> {
        self.modules
            .iter()
            .map(|module| module.name.clone())
            .position(|mod_name| mod_name == target_module_name)
    }
}

impl PartialEq for ImportGraph {
    fn eq(&self, other: &ImportGraph) -> bool {
        if self.modules.len() != other.modules.len() {
            return false;
        }

        for module in self.modules.iter() {
            if !other.modules.contains(module) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
pub struct ImportCycle {
    pub modules: Vec<String>,
}
impl ImportCycle {
    fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    fn add_module(&mut self, module: String) {
        self.modules.push(module);
    }
}

impl PartialEq for ImportCycle {
    fn eq(&self, other: &ImportCycle) -> bool {
        if self.modules.len() != other.modules.len() {
            return false;
        }

        for cycle in &self.modules {
            if !other.modules.contains(&cycle) {
                return false;
            }
        }

        true
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
    let mut import_graph = ImportGraph::new();

    for dir_entry in fs::read_dir(local_path).unwrap() {
        let sub_path = dir_entry.unwrap().path();

        if sub_path.is_dir() && directory::path_is_not_hidden(&sub_path) {
            // check if directory is a Python module
            if directory::init_file_exists(&sub_path) {
                import_graph.extend(build_import_tree(&sub_path, prefix_for_strip));
            }
        } else if sub_path.is_file() && directory::is_python_file(&sub_path) {
            let imports = find_imports_in_py(&sub_path);

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
fn find_imports_in_py(file_path: &Path) -> Vec<String> {
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

pub fn print_import_cycles(import_cycles: Vec<ImportCycle>) {
    if import_cycles.is_empty() {
        println!("\u{2705} No circular imports were found.")
    } else {
        println!("\u{274C} Circular imports were found: \n");
        for cycle in import_cycles {
            let line = cycle.modules.join(" -> ");
            println!("{line}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_imports_in_py() {
        let file_path = PathBuf::from("test_files/package/module.py");
        let imports = find_imports_in_py(&file_path);
        assert_eq!(
            imports,
            vec!["os", "sys", "numpy", "pandas", "matplotlib.pyplot"]
        );
    }

    #[test]
    fn test_build_import_tree() {
        let local_path = PathBuf::from("test_files");
        let import_graph = build_import_tree(&local_path, "test_files");
        let expected_import_graph = ImportGraph {
            modules: vec![
                PythonModule {
                    self_id: 0,
                    name: "package.module".to_owned(),
                    imports: vec![
                        "os".to_owned(),
                        "sys".to_owned(),
                        "numpy".to_owned(),
                        "pandas".to_owned(),
                        "matplotlib.pyplot".to_owned(),
                    ],
                },
                PythonModule {
                    self_id: 1,
                    name: "package.submodule.test1".to_owned(),
                    imports: vec!["os".to_owned(), "sys".to_owned()],
                },
                PythonModule {
                    self_id: 2,
                    name: "package.submodule.subsubmodule.test2".to_owned(),
                    imports: vec!["os".to_owned(), "sys".to_owned()],
                },
            ],
        };
        assert_eq!(import_graph, expected_import_graph);
    }

    #[test]
    fn test_find_circular_imports() {
        let local_path = PathBuf::from("test_files");
        let import_graph = build_import_tree(&local_path, "test_files");
        let import_cycles = import_graph.find_circular_imports();
        let test_cycle = ImportCycle {
            modules: vec![
                "package.module".to_owned(),
                "package.submodule.test1".to_owned(),
                "package.submodule.subsubmodule.test2".to_owned(),
                "package.module".to_owned(),
            ],
        };
        let expected_import_cycles = vec![test_cycle];

        assert_eq!(import_cycles, expected_import_cycles);
    }
}
