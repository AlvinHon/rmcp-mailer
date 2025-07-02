use diesel::{
    Selectable,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::{Identifiable, Insertable, Queryable},
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
};

use crate::database::schema::recipients;

#[derive(Debug, Clone, Queryable, Insertable, Selectable, Identifiable, PartialEq, Eq)]
#[diesel(table_name = recipients)]
#[diesel(primary_key(id))]
pub struct Recipient {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub status: RecipientStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum RecipientStatus {
    Active,
    Inactive,
}

impl ToSql<Text, Sqlite> for RecipientStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let status_str = match self {
            RecipientStatus::Active => "Active",
            RecipientStatus::Inactive => "Inactive",
        };
        out.set_value(status_str);
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for RecipientStatus {
    fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(t.as_str().try_into()?)
    }
}

impl TryFrom<&str> for RecipientStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Active" => Ok(RecipientStatus::Active),
            "Inactive" => Ok(RecipientStatus::Inactive),
            _ => Err(format!("Invalid recipient status: {}", value)),
        }
    }
}
