use rusqlite::Connection;
const DB_URL: &str = "sqlite://sqlite.db";
fn init_database() -> Connection {
    let conn = Connection::open("my.db").unwrap();

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        (),
    )
    .expect("TODO: panic message");
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        (),
    )
    .expect("TODO: panic message");
    conn
}

#[derive(Debug)]
pub struct DataConnection {
    connection_pool: Connection,
}

impl DataConnection {
    pub fn new() -> DataConnection {
        DataConnection {
            connection_pool: init_database(),
        }
    }
}
