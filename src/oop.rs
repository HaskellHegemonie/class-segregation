use std::{fs::File, io::Write};

pub struct Class {
    pub name: String,
    pub html_body: String,
}
impl Class {
    pub fn new() -> Class {
        Class {
            name: String::new(),
            html_body: String::new(),
        }
    }
    pub fn generate_class_file(&self, head_html: &str, foot_html: &str, output_dir: &str) {
        let mut class_file = File::create(format!("{}{}.html", output_dir, &self.name)).expect(
            format!(
                "Couldn't create file `{}`",
                format!("{}{}.html", output_dir, &self.name)
            )
            .as_str(),
        );
        class_file
            .write(format!("{}{}{}", head_html, &self.html_body, foot_html).as_bytes())
            .unwrap();
    }
}
