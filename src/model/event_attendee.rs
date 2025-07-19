use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};

use crate::database::schema::event_attendees;

#[derive(Debug, Clone, Queryable, Insertable, Selectable, Identifiable, PartialEq, Eq)]
#[diesel(table_name = event_attendees)]
#[diesel(primary_key(id))]
pub struct EventAttendee {
    pub id: i32,
    pub event_id: i32,
    pub recipient_id: i32,
}
