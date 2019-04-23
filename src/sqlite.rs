use super::{row::*, DatabaseRead};
use rusqlite::{Connection, NO_PARAMS};

pub struct Sqlite {
    conn: Connection,
}

impl Sqlite {
    pub fn new() -> Self {
        Self {
            conn: Connection::open_in_memory().unwrap(),
        }
    }

    pub fn populate(&self) {
        self.conn
            .execute(
                "CREATE TABLE User (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
                NO_PARAMS,
            )
            .unwrap();

        self.conn
            .execute("INSERT INTO User (name) VALUES ('Bob')", NO_PARAMS)
            .unwrap();
    }
}

impl DatabaseRead for Sqlite {
    fn query(&mut self, q: &str, idents: &[TypeIdentifier]) -> Vec<PrismaRow> {
        let mut stmt = self.conn.prepare_cached(q).unwrap();

        stmt.query_map(NO_PARAMS, |row| row.to_prisma_row(idents))
            .unwrap()
            .map(|res| res.unwrap())
            .collect()
    }
}
