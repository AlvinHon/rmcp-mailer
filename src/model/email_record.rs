use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Identifiable, Queryable},
};

use crate::database::schema::email_history;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, PartialEq, Eq)]
#[diesel(table_name = email_history)]
#[diesel(primary_key(id))]
pub struct EmailRecord {
    pub id: i32,
    pub recipient_id: i32,
    pub group_id: Option<i32>,
    pub subject: String,
    pub body: String,
    pub sent_at: NaiveDateTime,
}
