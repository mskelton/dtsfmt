use super::*;
use std::fs;
use std::path::Path;

pub fn get_specs_in_dir(path: &Path) -> Vec<Spec> {
    let spec_files = get_files_in_dir_recursive(path);

    spec_files.iter().flat_map(|text| parse_specs(text.clone())).collect()
}

pub fn get_files_in_dir_recursive(path: &Path) -> Vec<String> {
    return read_dir_recursively(path);

    fn read_dir_recursively(dir_path: &Path) -> Vec<String> {
        let mut result = Vec::new();

        for entry in dir_path.read_dir().expect("read dir failed").flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                let text = fs::read_to_string(&entry_path).unwrap();
                result.push(text);
            } else {
                result.extend(read_dir_recursively(&entry_path));
            }
        }

        result
    }
}
