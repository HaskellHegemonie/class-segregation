use regress::Regex;
use std::{
    fs::File,
    io::Read,
    io::{BufReader, Write},
};
fn main() {
    let path_to_create = "../output/";
    let file = File::open("../subst_001.htm").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    let file_input = convert_to_utf8_string(&buffer);

    // Regex
    let head_html_regex =
        Regex::with_flags(r#".+?(?=<td class="list inline_header" colspan="5")"#, "s").unwrap();

    let class_regex = Regex::with_flags(
        r#"(<td class="list inline_header" colspan="5")(?:.*?(?=\1|<\/table>))"#,
        "gs",
    )
    .unwrap();

    let mut head_html = file_input[head_html_regex.find(&file_input).unwrap().range()].to_string();
    let foot_html = r#"</table><p><font size="3" face="Arial">Stpl_21_22_finis</font></center><p><center><font face="Arial" size="2"><a href="http://www.untis.at" target="_blank" >Untis Stundenplan Software</a></font></center></body></html>"#;
    // Don't reload
    head_html = head_html.replace(
        r#"<meta http-equiv="refresh" content="4; URL=subst_001.htm">"#,
        "",
    );
    // Use utf8 instead of some java instant legacy encoding
    head_html = head_html.replace(r"iso-8859-1", "utf8");

    // Get the html of all classes
    for regex_match in class_regex.find_iter(&file_input) {
        let class_html = &file_input[regex_match.range()].to_string();
        let mut class = class_html.split_once(">").unwrap().1.to_string();
        // 3 chars should be enough
        class = class[0..3].trim().to_string();
        //Combines the head html with the class-html
        let mut class_file = File::create(format!("{}{}.html", path_to_create, &class)).unwrap();
        class_file
            .write(format!("{}{}{}", head_html, class_html, foot_html).as_bytes())
            .unwrap();
    }
}
fn convert_to_utf8_string(vec_of_bytes: &Vec<u8>) -> String {
    // why can't it just be in utf8?
    vec_of_bytes.into_iter().map(|&item| item as char).collect()
}
