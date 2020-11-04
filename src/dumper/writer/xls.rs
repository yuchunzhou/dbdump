use super::Writer;
use simple_excel_writer as excel;

pub struct Xls;

impl Writer for Xls {
    fn write(&self, output: &str, columns_name: &[String], rows_data: &[Vec<String>]) {
        let mut wb = excel::Workbook::create(output);
        let mut sheet = wb.create_sheet("");

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let mut row = excel::Row::new();
            for name in columns_name {
                row.add_cell(name.as_str());
            }
            sheet_writer.append_row(row).unwrap();

            for data in rows_data {
                let mut row = excel::Row::new();
                for column in data {
                    match column.parse::<f64>() {
                        Ok(value) => row.add_cell(value),
                        Err(_) => row.add_cell(column.as_str()),
                    }
                }
                sheet_writer.append_row(row).unwrap();
            }
            Ok(())
        })
        .expect("write excel file error!");
        wb.close().expect("close excel file error!");
    }
}
