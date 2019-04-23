use super::{row::*, Transaction, Transactional};
use rusqlite::{Connection, Transaction as RusqliteTrans, NO_PARAMS};

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

impl<'conn> Transaction for RusqliteTrans<'conn> {
    fn write(&mut self, q: &str) {
        self.execute(q, NO_PARAMS).unwrap();
    }

    fn read(&mut self, q: &str, idents: &[TypeIdentifier]) -> Vec<PrismaRow> {
        let mut stmt = self.prepare_cached(q).unwrap();

        stmt.query_map(NO_PARAMS, |row| row.to_prisma_row(idents))
            .unwrap()
            .map(|res| res.unwrap())
            .collect()
    }
}

impl Transactional for Sqlite {
    fn with_transaction<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Transaction) -> T,
    {
        let mut trans = self.conn.transaction().unwrap();
        let res = f(&mut trans);
        trans.commit().unwrap();
        res
    }
}
