use super::Writer;
use prettytable;
use std::fs::File;

pub struct Txt;

impl Writer for Txt {
    fn write(&self, output: &str, columns_name: &[String], rows_data: &[Vec<String>]) {
        let mut table = prettytable::Table::new();

        let mut row = prettytable::Row::empty();
        for name in columns_name {
            row.add_cell(prettytable::Cell::new(&name));
        }
        table.add_row(row);

        for data in rows_data {
            let mut row = prettytable::Row::empty();
            for column in data {
                row.add_cell(prettytable::Cell::new(&column));
            }
            table.add_row(row);
        }

        let mut file = File::create(output).unwrap();
        table.print(&mut file).expect("write txt file error!");
    }
}
