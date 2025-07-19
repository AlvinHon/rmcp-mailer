use crate::{error::MailerError, model::event_attendee::EventAttendee};
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn list_event_attendees(
        &mut self,
        by_event_id: i32,
    ) -> Result<Vec<EventAttendee>, MailerError> {
        use schema::event_attendees::dsl::*;

        event_attendees
            .filter(event_id.eq(by_event_id))
            .select(EventAttendee::as_select())
            .load::<EventAttendee>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn add_event_attendee(
        &mut self,
        new_event_id: i32,
        new_recipient_id: i32,
    ) -> Result<EventAttendee, MailerError> {
        use schema::event_attendees::dsl::*;

        diesel::insert_into(event_attendees)
            .values((event_id.eq(new_event_id), recipient_id.eq(new_recipient_id)))
            .returning(EventAttendee::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }
}
