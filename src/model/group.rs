use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};

use crate::database::schema::groups;

#[derive(Debug, Clone, Queryable, Insertable, Selectable, Identifiable, PartialEq, Eq)]
#[diesel(table_name = groups)]
#[diesel(primary_key(id))]
pub struct Group {
    pub id: i32,
    pub name: String,
}
