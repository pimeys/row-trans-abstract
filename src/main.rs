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

    sqlite.with_transaction(|trans| {
        trans.write("CREATE TABLE User (id INTEGER PRIMARY KEY, name TEXT NOT NULL)");
        trans.write("INSERT INTO User (name) VALUES ('Bob')");

        let sqlite_users = trans.read(
            "SELECT * FROM User",
            &[TypeIdentifier::Integer, TypeIdentifier::String],
        );

        let count = trans.read("SELECT COUNT(id) FROM \"User\"", &[TypeIdentifier::Integer]);

        println!(
            "Hello from SQLite, {:?}! Count is: {:?}",
            sqlite_users[0], count[0]
        );
    })
}

fn evaluate_psql() {
    let mut psql = Psql::new();

    psql.with_transaction(|trans| {
        trans.write("CREATE TABLE IF NOT EXISTS \"User\" (id SERIAL, name VARCHAR(255))");
        trans.write("INSERT INTO \"User\" (name) VALUES ('Bob')");

        let psql_users = trans.read(
            "SELECT * FROM \"User\"",
            &[TypeIdentifier::Integer, TypeIdentifier::String],
        );

        let count = trans.read("SELECT COUNT(id) FROM \"User\"", &[TypeIdentifier::Integer]);

        println!(
            "Hello from PostgreSQL, {:?}! Count is: {:?}",
            psql_users[0], count[0]
        );
    })
}

fn main() {
    evaluate_sqlite();
    evaluate_psql();
}
