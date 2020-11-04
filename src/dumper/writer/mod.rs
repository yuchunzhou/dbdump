pub mod csv;
pub mod txt;
pub mod xls;

pub trait Writer {
    fn write(&self, output: &str, columns_name: &[String], rows_data: &[Vec<String>]);
}
