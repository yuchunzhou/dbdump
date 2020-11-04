use super::writer;
use super::writer::Writer;
use super::Dumper;
use crate::args::Options;
use sqlite;

pub struct Sqlite;

impl Dumper for Sqlite {
    fn dump(&self, options: &Options) {
        let conn = sqlite::open(&options.database).unwrap();
        let mut statement = conn
            .prepare(format!(
                "select * from PRAGMA_TABLE_INFO('{}');",
                options.table
            ))
            .unwrap();

        let mut columns_name: Vec<String> = Vec::new();
        loop {
            match statement.next() {
                Ok(sqlite::State::Row) => {
                    columns_name.push(statement.read::<String>(1).unwrap());
                }
                Ok(sqlite::State::Done) => break,
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            }
        }

        let mut sql = "select ".to_owned();
        for name in columns_name.iter() {
            sql.push_str(&name);
            sql.push(',');
        }
        sql.pop();

        let mut statement = conn
            .prepare(format!(
                "{} from {} limit {};",
                sql, options.table, options.rows
            ))
            .unwrap();

        let mut rows_data: Vec<Vec<String>> = Vec::new();
        loop {
            match statement.next() {
                Ok(sqlite::State::Row) => {
                    let mut one_row = Vec::new();
                    for i in 0..statement.count() {
                        one_row.push(statement.read::<String>(i).unwrap());
                    }
                    rows_data.push(one_row);
                }
                Ok(sqlite::State::Done) => break,
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            }
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
