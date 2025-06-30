use diesel::{
    Selectable,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::{Identifiable, Insertable, Queryable},
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
};

use crate::database::schema::event_attendees;

#[derive(Debug, Clone, Queryable, Insertable, Selectable, Identifiable, PartialEq, Eq)]
#[diesel(table_name = event_attendees)]
#[diesel(primary_key(id))]
pub struct EventAttendee {
    pub id: i32,
    pub event_id: i32,
    pub recipient_id: i32,
    pub acceptance_status: AcceptanceStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum AcceptanceStatus {
    Accepted,
    Declined,
    Tentative,
}

impl ToSql<Text, Sqlite> for AcceptanceStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let status_str = match self {
            AcceptanceStatus::Accepted => "Accepted",
            AcceptanceStatus::Declined => "Declined",
            AcceptanceStatus::Tentative => "Tentative",
        };
        out.set_value(status_str);
        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for AcceptanceStatus {
    fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(t.as_str().try_into()?)
    }
}

impl TryFrom<&str> for AcceptanceStatus {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(AcceptanceStatus::Accepted),
            "Declined" => Ok(AcceptanceStatus::Declined),
            "Tentative" => Ok(AcceptanceStatus::Tentative),
            _ => Err(format!("Invalid acceptance status: {}", value)),
        }
    }
}
