use std::fs::File;
use std::io::Read;

pub fn get_file_content_as_str(path: &str) -> Option<String> {
    let file = File::open(path);
    if file.is_err() { return None; }
    let mut file = file.unwrap();

    let mut content = String::new();
    let read_result = file.read_to_string(&mut content);
    if read_result.is_err() { return None; }
    Some(content)
}