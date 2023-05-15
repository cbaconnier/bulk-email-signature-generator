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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use maplit::hashmap;

    fn compare_hash_maps(map1: &HashMap<String, String>, map2: &HashMap<String, String>) -> bool {
        if map1.len() != map2.len() {
            return false;
        }

        for (key, value) in map1.iter() {
            if let Some(map2_value) = map2.get(key) {
                if value != map2_value {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    #[test]
    fn test_csv_reader_new() {
        // Create a temporary CSV file with headers and records
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bulk-email-signature-generator-test.csv");
        let mut writer = csv::Writer::from_path(file_path.clone()).unwrap();
        writer.write_record(&["file_name", "name"]).unwrap();
        writer.write_record(&["file1.html", "John Doe"]).unwrap();
        writer.write_record(&["file2.html", "Jane Doe"]).unwrap();
        writer.flush().unwrap();

        // Test that CsvReader can read the CSV file and return the expected headers and records
        let reader = CsvReader::new(file_path.to_str().unwrap().to_owned());
        assert_eq!(reader.headers.len(), 2);
        assert_eq!(reader.records.len(), 2);
        assert_eq!(reader.records()[0]["file_name"], "file1.html");
        assert_eq!(reader.records()[1]["name"], "Jane Doe");
    }

    #[test]
    fn test_csv_reader_records() {
        // Create a CSV reader with headers and records
        let headers = StringRecord::from(vec!["file_name", "name"]);
        let record1 = StringRecord::from(vec!["file1.html", "John Doe"]);
        let record2 = StringRecord::from(vec!["file2.html", "Jane Doe"]);
        let reader = CsvReader {
            headers,
            records: vec![record1, record2],
        };

        // Test that CsvReader can return the expected records as HashMaps
        let expected_records = vec![
            hashmap!["file_name".to_string() => "file1.html".to_string(), "name".to_string() => "John Doe".to_string()],
            hashmap!["file_name".to_string() => "file2.html".to_string(), "name".to_string() => "Jane Doe".to_string()],
        ];

        assert_eq!(compare_hash_maps(&reader.records()[0], &expected_records[0]), true);
        assert_eq!(compare_hash_maps(&reader.records()[1], &expected_records[1]), true);
    }


    #[test]
    #[should_panic(expected = "CSV does not contain header file_name")]
    fn test_csv_reader_invalid_header() {
        // Create a CSV reader with headers and records, but with an invalid header name
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bulk-email-signature-generator-test.csv");
        let mut writer = csv::Writer::from_path(file_path.clone()).unwrap();
        writer.write_record(&["invalid_header", "name"]).unwrap();
        writer.write_record(&["file1.html", "John Doe"]).unwrap();
        writer.write_record(&["file2.html", "Jane Doe"]).unwrap();
        writer.flush().unwrap();

        CsvReader::new(file_path.to_str().unwrap().to_owned());
    }
}
