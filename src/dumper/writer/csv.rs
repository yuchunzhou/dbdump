use super::Writer;
use csv;

pub struct Csv;

impl Writer for Csv {
    fn write(&self, output: &str, columns_name: &[String], rows_data: &[Vec<String>]) {
        let mut writer = csv::Writer::from_path(output).unwrap();
        writer
            .write_record(columns_name)
            .expect("write csv file error!");
        for data in rows_data {
            writer
                .write_record(data.as_slice())
                .expect("write csv file error!");
        }
    }
}
