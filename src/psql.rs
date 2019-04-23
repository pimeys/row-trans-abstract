use super::{row::*, DatabaseRead};
use postgres::{Client, NoTls};

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

impl DatabaseRead for Psql {
    fn query(&mut self, q: &str, idents: &[TypeIdentifier]) -> Vec<PrismaRow> {
        self.client
            .query(q, &[])
            .unwrap()
            .into_iter()
            .map(|row| row.to_prisma_row(idents))
            .collect()
    }
}
