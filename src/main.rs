mod psql;
mod row;
mod sqlite;

use psql::*;
use row::*;
use sqlite::*;

#[derive(Debug, Clone)]
struct User {
    fields: Vec<TypeIdentifier>,
}

pub trait Transaction {
    fn write(&mut self, q: &str);
    fn read(&mut self, q: &str, idents: &[TypeIdentifier]) -> Vec<PrismaRow>;
}

pub trait Transactional {
    fn with_transaction<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Transaction) -> T;
}

fn evaluate_sqlite() {
    let mut sqlite = Sqlite::new();
    sqlite.populate();

    sqlite.with_transaction(|trans| {
        let sqlite_users = trans.read(
            "SELECT * FROM User",
            &[TypeIdentifier::Integer, TypeIdentifier::String],
        );

        println!("Hello from SQLite, {:?}!", sqlite_users[0]);
    })
}

fn evaluate_psql() {
    let mut psql = Psql::new();
    psql.populate();

    psql.with_transaction(|trans| {
        let psql_users = trans.read(
            "SELECT * FROM \"User\"",
            &[TypeIdentifier::Integer, TypeIdentifier::String],
        );

        println!("Hello from PostgreSQL, {:?}!", psql_users[0]);
    })
}

fn main() {
    evaluate_sqlite();
    evaluate_psql();
}
