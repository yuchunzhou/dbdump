pub mod mysql;
pub mod postgresql;
pub mod sqlite;
mod writer;

pub trait Dumper {
    fn dump(&self, options: &crate::args::Options);
}
