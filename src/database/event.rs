use crate::{error::MailerError, model::event::Event};
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn list_events(
        &mut self,
        from_time: chrono::NaiveDateTime,
        to_time: Option<chrono::NaiveDateTime>,
    ) -> Result<Vec<Event>, MailerError> {
        use schema::events::dsl::*;

        let mut query = events.filter(start_time.ge(from_time)).into_boxed();

        if let Some(to_time) = to_time {
            query = query.filter(start_time.le(to_time));
        }

        query
            .select(Event::as_select())
            .load::<Event>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn add_event(
        &mut self,
        new_title: String,
        new_description: Option<String>,
        new_start_time: chrono::NaiveDateTime,
        new_end_time: Option<chrono::NaiveDateTime>,
        new_is_all_day: bool,
    ) -> Result<Event, MailerError> {
        use schema::events::dsl::*;

        diesel::insert_into(events)
            .values((
                title.eq(new_title),
                description.eq(new_description),
                start_time.eq(new_start_time),
                end_time.eq(new_end_time),
                is_all_day.eq(new_is_all_day),
            ))
            .returning(Event::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn remove_event(&mut self, event_id: i32) -> Result<usize, MailerError> {
        use schema::events::dsl::*;

        diesel::delete(events.filter(id.eq(event_id)))
            .execute(&mut self.connection)
            .map_err(MailerError::from)
    }
}
