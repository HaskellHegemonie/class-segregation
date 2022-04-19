use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn read_file(path: &str) -> String {
    let file =
        File::open(&path).expect(format!("No such file with path `{}` found", &path).as_str());
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    let file_input = convert_to_utf8_string(&buffer);
    file_input
}
fn convert_to_utf8_string(vec_of_bytes: &Vec<u8>) -> String {
    // why can't it just be in utf8?
    vec_of_bytes.into_iter().map(|&item| item as char).collect()
}
