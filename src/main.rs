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

pub trait DatabaseRead {
    fn query(&mut self, q: &str, idents: &[TypeIdentifier]) -> Vec<PrismaRow>;
}

fn evaluate_sqlite() {
    let mut sqlite = Sqlite::new();
    sqlite.populate();

    let sqlite_users = sqlite.query(
        "SELECT * FROM User",
        &[TypeIdentifier::Integer, TypeIdentifier::String],
    );

    println!("Hello from SQLite, {:?}!", sqlite_users[0]);
}

fn evaluate_psql() {
    let mut psql = Psql::new();
    psql.populate();

    let psql_users = psql.query(
        "SELECT * FROM \"User\"",
        &[TypeIdentifier::Integer, TypeIdentifier::String],
    );

    println!("Hello from PostgreSQL, {:?}!", psql_users[0]);
}

fn main() {
    evaluate_sqlite();
    evaluate_psql();
}
