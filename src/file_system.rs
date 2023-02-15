use std::fs;

pub fn list_files() -> Vec<String> {
    let mut file_names: Vec<String> = vec![];
    let files = fs::read_dir("./").unwrap();

    for file in files {
        match file {
            Ok(value) => file_names.push(value.path().display().to_string()),
            Err(_err) => panic!("Was not able to read file")
        }

    }

    return file_names;
}
