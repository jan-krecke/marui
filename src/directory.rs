use std::fs;
use std::path::Path;

/// Check if a directory contains a `__init__.py` file
///
/// This function is used to determine whether a given directory
/// is a Python module or not.
///
/// # Arguments
/// * `path` - Path to the directory in question
pub fn init_file_exists(path: &Path) -> bool {
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

/// Check if a directory contains a `pyproject.toml` file
///
/// This function is useful to determine whether a given directory
/// is a Python project or not
///
/// # Arguments
/// * `path` - Path to the directory in question
pub fn pyproject_exists(path: &Path) -> bool {
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

/// Check if a given file is a Python file or not.
///
/// `__init__.py` is not considered a Python file, although this
/// may be changed in the future.
///
/// # Arguments
/// * `path` - Path to the file in question.
pub fn is_python_file(path: &Path) -> bool {
    if !path.ends_with("__init__.py") {
        match path.extension() {
            Some(ext) => ext == "py",
            None => false,
        }
    } else {
        false
    }
}

/// Check if a given directory or file is not hidden
///
/// Hidden files will not be considered by marui.
///
/// * `path` - Path to the given file or directory.
pub fn path_is_not_hidden(path: &Path) -> bool {
    !path.file_name().unwrap().to_str().unwrap().starts_with('.')
}
