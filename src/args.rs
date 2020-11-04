use clap::App;

#[derive(Debug, Default)]
pub struct Options {
    pub host: String,
    pub port: u16,
    pub kind: String,
    pub username: String,
    pub password: String,
    pub database: String,
    pub table: String,
    pub format: String,
    pub rows: usize,
    pub output: String,
}

pub fn parse_args(app: App) -> Option<Options> {
    let mut options: Options = Default::default();
    let args = app.get_matches();

    if let Some(host) = args.value_of("host") {
        options.host = host.to_owned();
    }

    if let Some(port) = args.value_of("port") {
        match port.parse::<u16>() {
            Ok(port) => options.port = port,
            Err(_) => {
                println!("please input a valid database server port!");
                return None;
            }
        }
    }

    if let Some(kind) = args.value_of("kind") {
        options.kind = kind.to_owned();
    }

    if let Some(username) = args.value_of("username") {
        options.username = username.to_owned();
    }

    if let Some(password) = args.value_of("password") {
        options.password = password.to_owned();
    }

    match args.value_of("database") {
        Some(database) => options.database = database.to_owned(),
        None => {
            println!("database can't be empty!");
            return None;
        }
    }

    match args.value_of("table") {
        Some(table) => options.table = table.to_owned(),
        None => {
            println!("table can't be empty!");
            return None;
        }
    }

    if let Some(format) = args.value_of("format") {
        options.format = format.to_owned();
    }

    if let Some(rows) = args.value_of("rows") {
        match rows.parse::<usize>() {
            Ok(rows) => options.rows = rows,
            Err(_) => {
                println!("please input a valid rows count number!");
                return None;
            }
        }
    }

    match args.value_of("output") {
        Some(output) => {
            options.output = output.to_owned();
            if !output.ends_with(&options.format) {
                options.output.push('.');
                options.output.push_str(&options.format);
            }
        }
        None => {
            println!("please specify an output file name, eg: test.csv!");
            return None;
        }
    }

    Some(options)
}
