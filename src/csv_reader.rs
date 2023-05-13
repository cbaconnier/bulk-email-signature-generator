use std::collections::HashMap;
use csv::StringRecord;

pub struct CsvReader {
    headers: StringRecord,
    records: Vec<StringRecord>,
}

impl CsvReader {
    pub fn new(path: String) -> Self {
        let mut rdr = csv::Reader::from_path(path).unwrap();

        let headers = rdr.headers().unwrap().clone();
        let mut records = Vec::new();

        for result in rdr.records() {
            let record = result.unwrap();
            records.push(record);
        }

        Self::assert_header(headers.clone(), "file_name");

        CsvReader {
            headers,
            records,
        }
    }

    fn assert_header(headers: StringRecord, header: &str) {
        if !headers.iter().any(|h| h == header) {
            panic!("CSV does not contain header {}", header);
        }
    }

    pub fn headers(&self) -> &StringRecord {
        &self.headers
    }

    pub fn records(&self) -> Vec<HashMap<String, String>> {
        self.records.iter().map(|record| {
            let mut hashmap = HashMap::new();

            for (index, field) in record.iter().enumerate() {
                let header = self.headers.get(index).unwrap();
                hashmap.insert(header.to_owned(), field.to_owned());
            }

            hashmap
        }).collect()
    }
}
