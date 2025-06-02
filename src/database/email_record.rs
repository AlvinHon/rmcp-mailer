use crate::{error::MailerError, model::email_record::EmailRecord};
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn list_email_records_by_time(
        &mut self,
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
    ) -> Result<Vec<EmailRecord>, MailerError> {
        use schema::email_history::dsl::*;
        email_history
            .filter(sent_at.ge(start_time).and(sent_at.le(end_time)))
            .load::<EmailRecord>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn add_email_record(
        &mut self,
        new_recipient_id: i32,
        new_group_id: Option<i32>,
        new_subject: String,
        new_body: String,
    ) -> Result<(), MailerError> {
        use schema::email_history::dsl::*;
        diesel::insert_into(schema::email_history::table)
            .values((
                recipient_id.eq(new_recipient_id),
                group_id.eq(new_group_id),
                subject.eq(new_subject),
                body.eq(new_body),
                sent_at.eq(diesel::dsl::now),
            ))
            .execute(&mut self.connection)
            .map_err(MailerError::from)?;
        Ok(())
    }
}
