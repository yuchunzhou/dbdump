use super::writer;
use super::writer::Writer;
use super::Dumper;
use crate::args::Options;
use mysql;
use mysql::prelude::*;

pub struct Mysql;

impl Dumper for Mysql {
    fn dump(&self, options: &Options) {
        let opts = mysql::OptsBuilder::new()
            .ip_or_hostname(Some(&options.host))
            .tcp_port(options.port)
            .user(Some(&options.username))
            .pass(Some(&options.password))
            .db_name(Some(&options.database));
        let mut conn = match mysql::Conn::new(opts) {
            Ok(conn) => conn,
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        };

        let mut result = conn
        .query_iter(format!(
            "select column_name from information_schema.columns where table_name = '{}' and table_schema = '{}';",
            options.table, &options.database
        ))
        .unwrap();

        let mut columns_name: Vec<String> = Vec::new();
        let result_set = result.next_set().unwrap().unwrap();
        for row in result_set {
            let name: Vec<u8> = row.unwrap().get(0).unwrap();
            columns_name.push(String::from_utf8(name).unwrap());
        }

        let mut sql = "select ".to_owned();
        for name in columns_name.iter() {
            sql.push_str(&name);
            sql.push(',');
        }
        sql.pop();

        drop(result);
        let mut result = conn
            .query_iter(format!(
                "{} from {} limit {}",
                sql, options.table, options.rows
            ))
            .unwrap();

        let mut rows_data: Vec<Vec<String>> = Vec::new();
        let columns_cnt = columns_name.len();
        let result_set = result.next_set().unwrap().unwrap();
        for row in result_set {
            let r = row.unwrap();
            let mut row_data = Vec::new();

            for i in 0..columns_cnt {
                let data: Vec<u8> = r.get(i).unwrap();
                row_data.push(String::from_utf8(data).unwrap());
            }
            rows_data.push(row_data);
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
