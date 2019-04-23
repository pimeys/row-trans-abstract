#[derive(Debug, Clone, Copy)]
pub enum TypeIdentifier {
    Integer,
    String,
}

#[derive(Debug, Clone)]
pub enum PrismaValue {
    Integer(i64),
    String(String),
}

impl From<String> for PrismaValue {
    fn from(s: String) -> Self {
        PrismaValue::String(s)
    }
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
