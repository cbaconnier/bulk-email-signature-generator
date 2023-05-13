use std::collections::HashMap;
use tinytemplate::TinyTemplate;

use serde::{Deserialize, Serialize};

pub struct HtmlGenerator {
    records: Vec<HashMap<String, String>>,
    template: String,
    output: String,
}

#[derive(Serialize)]
struct ContactSignatureContext {
    contact: HashMap<String, String>,
}

impl HtmlGenerator {
    pub fn new(path: String, records: Vec<HashMap<String, String>>, output: String) -> Self {
        let template = std::fs::read_to_string(path).unwrap();
        HtmlGenerator {
            records,
            template,
            output,
        }
    }

    pub fn format_html(
        &self,
        contact: HashMap<String, String>
    ) -> String {
        let mut template = TinyTemplate::new();
        template.add_template("signature", &self.template).unwrap();

        let context = ContactSignatureContext {
            contact: contact.clone(),
        };

        template.render("signature", &context).unwrap()
    }

    pub fn generate(&self) {
        for record in &self.records {
            let record_html = self.format_html(record.clone());

            let mut slug = record.get("file_name").unwrap().to_owned();
            if !slug.ends_with(".html") {
                slug.push_str(".html");
            }

            let file_path = format!("{}/{}", self.output, slug);

            std::fs::create_dir_all(&self.output).unwrap();

            std::fs::write(&file_path, &record_html).unwrap();
        }
    }
}
