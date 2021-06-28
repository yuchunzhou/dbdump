# dbdump
A database dump tool

### How to use
```bash
$ dbdump -h
Database dump tool 0.2.0

yuchunzhou <chunzhou.yu@qq.com>
Dump database data to an ordinary file

USAGE:
    dbdump [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -H, --host <host>            database server address [default: 127.0.0.1]
    -P, --port <port>            database server port [default: 3306]
    -k, --kind <kind>            database kind [default: mysql]  [possible values: sqlite, mysql, postgresql]
    -u, --user <username>        database username
    -p, --password <password>    database password
    -d, --database <database>    which database contains the dumped table
    -t, --table <table>          the target table
    -n, --rows <rows>            how many rows to be dumped [default: 10]
    -f, --format <format>        output file format [default: csv]  [possible values: csv, xls, txt]
    -o, --output <output>        output file name
```
