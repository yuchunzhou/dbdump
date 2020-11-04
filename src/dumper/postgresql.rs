use super::writer;
use super::writer::Writer;
use super::Dumper;
use crate::args::Options;
use elephantry;

pub struct Postgresql;

impl Dumper for Postgresql {
    fn dump(&self, options: &Options) {
        let client = elephantry::Pool::from_config(&elephantry::Config {
            host: Some(options.host.clone()),
            user: Some(options.username.clone()),
            dbname: Some(options.database.clone()),
            port: Some(format!("{}", options.port)),
            password: Some(options.password.clone()),
        })
        .unwrap();

        let result = client
            .execute(&format!(
                "select column_name from information_schema.columns where table_name = '{}'",
                options.table
            ))
            .unwrap();

        let mut columns_name: Vec<String> = Vec::new();
        for row in &result {
            let name: String = row.nth(0);
            columns_name.push(name);
        }

        let mut rows_data: Vec<Vec<String>> = Vec::new();
        let mut sql = "select ".to_owned();
        for name in columns_name.iter() {
            sql.push_str(&name);
            sql.push(',');
        }
        sql.pop();

        let result = client
            .execute(&format!(
                "{} from {} limit {}",
                sql, options.table, options.rows
            ))
            .unwrap();

        for row in &result {
            let mut one_row: Vec<String> = Vec::new();
            for i in 0..row.len() {
                let data: String = row.nth(i as usize);
                one_row.push(data);
            }
            rows_data.push(one_row);
        }

        match options.format.as_str() {
            "csv" => {
                let csv_writer = writer::csv::Csv;
                csv_writer.write(&options.output, &columns_name, &rows_data);
            }
            "xls" => {
                let xls_writer = writer::xls::Xls;
                xls_writer.write(&options.output, &columns_name, &rows_data);
            }
            "txt" => {
                let txt_writer = writer::txt::Txt;
                txt_writer.write(&options.output, &columns_name, &rows_data);
            }
            _ => (),
        }
    }
}
