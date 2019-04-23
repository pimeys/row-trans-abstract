use postgres::row::Row as PsqlRow;
use rusqlite::Row as RusqliteRow;

#[derive(Debug, Clone, Copy)]
pub enum TypeIdentifier {
    Integer,
    String,
}

#[derive(Debug, Clone)]
pub enum PrismaValue {
    Integer(i32),
    String(String),
}

#[derive(Debug, Clone, Default)]
pub struct PrismaRow {
    pub values: Vec<PrismaValue>,
}

pub trait ToPrismaRow {
    fn to_prisma_row<'b, T>(&'b self, idents: T) -> PrismaRow
    where
        T: IntoIterator<Item = &'b TypeIdentifier>;
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

impl ToPrismaRow for PsqlRow {
    fn to_prisma_row<'b, T>(&'b self, idents: T) -> PrismaRow
    where
        T: IntoIterator<Item = &'b TypeIdentifier>,
    {
        let mut row = PrismaRow::default();

        for (i, typid) in idents.into_iter().enumerate() {
            match typid {
                TypeIdentifier::String => row
                    .values
                    .push(self.try_get(i).map(|val| PrismaValue::String(val)).unwrap()),
                TypeIdentifier::Integer => row.values.push(
                    self.try_get(i)
                        .map(|val| PrismaValue::Integer(val))
                        .unwrap(),
                ),
            }
        }

        row
    }
}
