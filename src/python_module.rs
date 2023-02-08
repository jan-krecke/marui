use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct PythonModule {
    pub name: String,
    pub imports: Vec<String>,
}

pub fn find_python_modules(local_path: PathBuf) -> Vec<PythonModule> {
    let local_path_string = local_path.clone().to_str().unwrap().to_owned();
    let mut modules: Vec<PythonModule> = Vec::new();
    for dir_entry in fs::read_dir(local_path).unwrap() {
        let sub_path = dir_entry.unwrap().path();
        if sub_path.is_dir()
            && !sub_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(".")
        {
            if crate::directory::init_file_exists(&sub_path) {
                let modules_to_add = find_python_modules(sub_path);
                modules.extend(modules_to_add);
            }
        } else if sub_path.is_file() && crate::directory::is_python_file(&sub_path) {
            let imports = find_imports(&sub_path);
            modules.push(PythonModule {
                name: sub_path
                    .strip_prefix(&local_path_string)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                imports: imports,
            })
        }
    }

    modules
}

fn find_imports(file_path: &Path) -> Vec<String> {
    // Find import in a Python file
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
