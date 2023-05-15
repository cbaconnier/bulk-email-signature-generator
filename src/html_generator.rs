use std::collections::HashMap;
use tinytemplate::TinyTemplate;
use serde::{Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use maplit::hashmap;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_html_generator() {

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bulk-email-signature-generator-template.html");
        let output = dir.path().join("output_testing");

        let html = r#"
        <div>
            {{- if contact.name -}}
            <p>{contact.name}</p>
            {{- endif -}}
            {{- if contact.email -}}
            <p>{contact.email}</p>
            {{- endif -}}
            {{- if contact.phone -}}
            <p>{contact.phone}</p>
            {{- endif -}}
        </div>
        "#.trim();

        let mut writer = File::create(&file_path).unwrap();
        writer.write_all(html.as_bytes()).unwrap();
        writer.flush().unwrap();

        let path: String = file_path.display().to_string();
        let records: Vec<HashMap<String, String>> = vec![
            hashmap! {
                "file_name".to_string() => "contact1".to_string(),
                "name".to_string() => "John Smith".to_string(),
                "email".to_string() => "john@example.com".to_string(),
                "phone".to_string() => "+1 123-456-7890".to_string(),
            },
            hashmap! {
                "file_name".to_string() => "contact2".to_string(),
                "name".to_string() => "Jane Doe".to_string(),
                "email".to_string() => "jane@example.com".to_string(),
                "phone".to_string() => "+1 234-567-8901".to_string(),
            },
        ];

        let html_generator = HtmlGenerator::new(path.clone(), records.clone(), output.display().to_string());

        let mut expected_file_paths = vec![];
        let mut expected_file_contents = vec![];

        // Test format_html returns the expected HTML
        for record in &records {
            let mut slug = record.get("file_name").unwrap().to_owned();
            if !slug.ends_with(".html") {
                slug.push_str(".html");
            }

            let file_path = format!("{}/{}", output.display().to_string(), slug);
            expected_file_paths.push(file_path.clone());

            let expected_html = format!(
                "<div><p>{}</p><p>{}</p><p>{}</p></div>",
                record.get("name").unwrap(),
                record.get("email").unwrap(),
                record.get("phone").unwrap(),
            );
            expected_file_contents.push(expected_html.clone());

            let contact = record.clone();
            let actual_html = html_generator.format_html(contact);
            assert_eq!(actual_html, expected_html);
        }

        html_generator.generate();

        // Verify the expected files were created with the expected HTML
        for (i, file_path) in expected_file_paths.iter().enumerate() {
            let expected_content = &expected_file_contents[i];
            let actual_content = std::fs::read_to_string(file_path).unwrap();
            assert_eq!(actual_content, *expected_content);
        }
    }
}
