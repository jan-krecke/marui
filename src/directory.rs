use std::fs;
use std::path::PathBuf;

pub fn init_file_exists(path: &PathBuf) -> bool {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_file()
            && entry_path.file_name().unwrap().to_str().unwrap() == "__init__.py"
        {
            return true;
        }
    }
    false
}

pub fn pyproject_exists(path: &PathBuf) -> bool {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_file()
            && entry_path.file_name().unwrap().to_str().unwrap() == "pyproject.toml"
        {
            return true;
        }
    }
    false
}

pub fn is_python_file(path: &PathBuf) -> bool {
    if !path.ends_with("__init__.py") {
        match path.extension() {
            Some(ext) => ext == "py",
            None => false,
        }
    } else {
        false
    }
}

pub fn path_is_not_hidden(path: &PathBuf) -> bool {
    !path.file_name().unwrap().to_str().unwrap().starts_with('.')
}
