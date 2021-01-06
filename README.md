# dbdump
A database dump tool

### How to use it?
```bash
$ dbdump -h
Database dump tool 1.0

yuchunzhou <chunzhou.yu@qq.com>
dump the database data to an ordinary file

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
    -t, --table <table>          which database table to be dumped
    -d, --database <database>    which database contains the dumped table
    -f, --format <format>        destination file format [default: csv]  [possible values: csv, xls, txt]
    -n, --rows <rows>            how many rows to be dumped [default: 10]
    -o, --output <output>        output file name
```
