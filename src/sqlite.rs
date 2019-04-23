use super::{row::*, Transaction, Transactional};
use rusqlite::{Connection, Row as RusqliteRow, Transaction as RusqliteTrans, NO_PARAMS};

pub struct Sqlite {
    conn: Connection,
}

impl Sqlite {
    pub fn new() -> Self {
        Self {
            conn: Connection::open_in_memory().unwrap(),
        }
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

impl<'a, 'stmt> ToPrismaRow for RusqliteRow<'a, 'stmt> {
    fn to_prisma_row<'b, T>(&'b self, idents: T) -> PrismaRow
    where
        T: IntoIterator<Item = &'b TypeIdentifier>,
    {
        let mut row = PrismaRow::default();

        for (i, typid) in idents.into_iter().enumerate() {
            match typid {
                TypeIdentifier::String => row.values.push(
                    self.get_checked(i)
                        .map(|val| PrismaValue::String(val))
                        .unwrap(),
                ),
                TypeIdentifier::Integer => row.values.push(
                    self.get_checked(i)
                        .map(|val| PrismaValue::Integer(val))
                        .unwrap(),
                ),
            }
        }

        row
    }
}
