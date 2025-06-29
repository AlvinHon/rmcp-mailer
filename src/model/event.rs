use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};

use crate::database::schema::events;

#[derive(Debug, Clone, Queryable, Insertable, Selectable, Identifiable, PartialEq, Eq)]
#[diesel(table_name = events)]
#[diesel(primary_key(id))]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub is_all_day: bool,
}
