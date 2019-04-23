use super::{row::*, Transaction, Transactional};
use postgres::{
    types::{FromSql, Type},
    Client, NoTls, Row as PsqlRow, Transaction as PsqlTrans,
};
use std::error::Error;

pub struct Psql {
    client: Client,
}

impl Psql {
    pub fn new() -> Self {
        Self {
            client: Client::connect("host=127.0.0.1 user=postgres password=prisma", NoTls).unwrap(),
        }
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

impl<'a> FromSql<'a> for PrismaValue {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let res = match ty {
            &Type::INT8 => PrismaValue::Integer(i64::from_sql(ty, raw).unwrap()),
            &Type::INT4 => PrismaValue::Integer(i32::from_sql(ty, raw).unwrap() as i64),
            &Type::INT2 => PrismaValue::Integer(i16::from_sql(ty, raw).unwrap() as i64),
            &Type::VARCHAR => PrismaValue::String(String::from_sql(ty, raw).unwrap()),
            &Type::TEXT => PrismaValue::String(String::from_sql(ty, raw).unwrap()),
            ty => panic!("Type {:?} not supported!", ty),
        };

        Ok(res)
    }

    fn accepts(ty: &Type) -> bool {
        ty == &Type::INT8
            || ty == &Type::INT4
            || ty == &Type::INT2
            || ty == &Type::VARCHAR
            || ty == &Type::TEXT
    }
}

impl ToPrismaRow for PsqlRow {
    fn to_prisma_row<'b, T>(&'b self, idents: T) -> PrismaRow
    where
        T: IntoIterator<Item = &'b TypeIdentifier>,
    {
        let mut row = PrismaRow::default();

        for (i, _) in idents.into_iter().enumerate() {
            row.values.push(self.try_get(i).unwrap());
        }

        row
    }
}
