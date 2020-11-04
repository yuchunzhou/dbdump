mod args;
mod dumper;

use clap::{load_yaml, App};
use dumper::Dumper;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let app = App::from(yaml);
    let options = match args::parse_args(app) {
        Some(options) => options,
        None => return,
    };

    match options.kind.as_str() {
        "sqlite" => {
            use dumper::sqlite;
            let sqlite_dumper = sqlite::Sqlite;
            sqlite_dumper.dump(&options);
        }
        "mysql" => {
            use dumper::mysql;
            let mysql_dumper = mysql::Mysql;
            mysql_dumper.dump(&options);
        }
        "postgresql" => {
            use dumper::postgresql;
            let postgresql_dumper = postgresql::Postgresql;
            postgresql_dumper.dump(&options);
        }
        _ => (),
    }
}
