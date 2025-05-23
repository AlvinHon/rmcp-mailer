use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};

use crate::database::schema::recipients;

#[derive(Debug, Clone, Queryable, Insertable, Selectable, Identifiable, PartialEq, Eq)]
#[diesel(table_name = recipients)]
#[diesel(primary_key(id))]
pub struct Recipient {
    pub id: i32,
    pub name: String,
    pub email: String,
}
