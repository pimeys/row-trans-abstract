use super::{row::*, Transaction, Transactional};
use postgres::{Client, NoTls, Transaction as PsqlTrans};

pub struct Psql {
    client: Client,
}

impl Psql {
    pub fn new() -> Self {
        Self {
            client: Client::connect("host=127.0.0.1 user=postgres password=prisma", NoTls).unwrap(),
        }
    }

    pub fn populate(&mut self) {
        self.client
            .simple_query("CREATE TABLE IF NOT EXISTS \"User\" (id SERIAL, name VARCHAR(255))")
            .unwrap();
        self.client
            .simple_query("INSERT INTO \"User\" (name) VALUES ('Bob')")
            .unwrap();
    }
}

impl<'a> Transaction for PsqlTrans<'a> {
    fn write(&mut self, q: &str) {
        self.simple_query(q).unwrap();
    }

    fn read(&mut self, q: &str, idents: &[TypeIdentifier]) -> Vec<PrismaRow> {
        self.query(q, &[])
            .unwrap()
            .into_iter()
            .map(|row| row.to_prisma_row(idents))
            .collect()
    }
}

impl Transactional for Psql {
    fn with_transaction<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Transaction) -> T,
    {
        let mut trans = self.client.transaction().unwrap();
        let res = f(&mut trans);
        trans.commit().unwrap();
        res
    }
}
