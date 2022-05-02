use regress::Regex;
use std::env::args;
use tokio_stream::StreamExt;
pub mod content_of_file;
pub mod oop;
#[tokio::main]
async fn main() {
    let path = args().nth(1);
    if let None = path {
        print!("Usage: command [FILE] [OUTPUT]\n{}", "Reads in a file, splits the classes up and writes single utf8 encoded html files with the information for the class only.\nBy default [OUTPUT] will be your current directory");
        return;
    }
    let path = path.unwrap();
    let mut output_dir = args().nth(2).unwrap_or(String::new());
    if &output_dir.as_bytes()[output_dir.len() - 1] != &b'/'
        && &output_dir.as_bytes()[output_dir.len() - 1] != &b'\\'
    {
        if cfg!(windows) {
            output_dir += r"\";
        } else {
            output_dir += "/";
        }
    }
    let file_content = content_of_file::read_file(&path).await;
    // Regex to find everything on top of all classes
    let head_html_regex =
        Regex::with_flags(r#".+?(?=<td class="list inline_header" colspan="5")"#, "s").unwrap();

    // Regex to find the html of the classes
    let class_regex = Regex::with_flags(
        r#"(<td class="list inline_header" colspan="5")(?:.*?(?=\1|<\/table>))"#,
        "gs",
    )
    .unwrap();

    // create the string with the head_html_regex
    let mut head_html =
        file_content[head_html_regex.find(&file_content).unwrap().range()].to_string();
    // the foot html which should be the same every time (send feet)
    let foot_html = r#"</table><p><font size="3"
        face="Arial">Stpl_21_22_finis</font></center><p><center><font face="Arial" size="2"><a
        href="http://www.untis.at" target="_blank" >Untis Stundenplan
        Software</a></font></center></body></html>"#;
    // Don't reload
    head_html = head_html.replace(
        r#"<meta http-equiv="refresh" content="4; URL=subst_001.htm">"#,
        "",
    );
    // Use utf8 instead of some java instant legacy encoding
    head_html = head_html.replace(r"iso-8859-1", "utf8");

    // Get the html of all classes
    let mut stream = tokio_stream::iter(class_regex.find_iter(&file_content));
    while let Some(regex_match) = stream.next().await{
        let mut current_class = oop::Class::new();
        current_class.html_body = file_content[regex_match.range()].to_string();
        current_class.name = current_class
            .html_body
            .split_once(">")
            .unwrap()
            .1
            .to_string();
        current_class.name = current_class.name[0..3].trim().to_string();
        current_class.generate_class_file(&head_html, foot_html, &output_dir);
    }
}
