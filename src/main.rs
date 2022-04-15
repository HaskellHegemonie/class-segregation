use std::{
    fs::File,
    io::{BufReader, Read},
};
pub mod regexx;
use format as f;
use print as p;
use regress::Regex;
fn main() {
    // regexx::run();
    let filter_classes = Regex::with_flags(
        r#".+?(?=<td class="list inline_header" colspan="5"|<\/table><p>)"#,
        "gs",
    )
    .unwrap();
    let file = File::open("../stundenplan.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer).unwrap();

    // Read.
    let file_input = convert_to_string(&buffer);
    let mut prerequisite = String::new();
    for (enumerator, class) in filter_classes.find_iter(&file_input).enumerate() {
        if enumerator == 0 {
            prerequisite = file_input[class.range()].to_string();
            continue;
        }
        let current_class_html = &file_input[class.range()].to_string();
        let current_class = current_class_html.split_once(">").unwrap().1.to_string()
            + &current_class_html.split_once("<").unwrap().0.to_string();
    }
}
fn convert_to_string(vec_of_bytes: &Vec<u8>) -> String {
    vec_of_bytes.into_iter().map(|&item| item as char).collect()
}
// let regex = Regex::with_flags(
//     r#"(<td class="list inline_header" colspan="5")(?:.+?(?=\1|<\/table>))"#,
//     "gs",
// )
// .unwrap();
