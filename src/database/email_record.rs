use crate::{
    error::{MailerError, new_rmcp_error},
    model::email_record::EmailRecord,
};
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn list_email_records_by_criteria(
        &mut self,
        start_end_time: Option<(chrono::NaiveDateTime, chrono::NaiveDateTime)>,
        by_recipient_id: Option<i32>,
    ) -> Result<Vec<EmailRecord>, MailerError> {
        use schema::email_history::dsl::*;
        use schema::email_history_recipients::dsl::*;

        if start_end_time.is_none() && by_recipient_id.is_none() {
            return Err(new_rmcp_error(
                "At least one filter must be provided for listing email records.",
            ));
        }

        let mut query = email_history_recipients
            .inner_join(schema::email_history::table)
            .into_boxed();

        if let Some((start, end)) = start_end_time {
            query = query.filter(sent_at.ge(start).and(sent_at.le(end)));
        }

        if let Some(by_recipient_id) = by_recipient_id {
            query = query.filter(recipient_id.eq(by_recipient_id));
        }
        query
            .select(EmailRecord::as_select())
            .load::<EmailRecord>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn add_email_record(
        &mut self,
        new_subject: String,
        new_body: String,
    ) -> Result<EmailRecord, MailerError> {
        use schema::email_history::dsl::*;
        diesel::insert_into(schema::email_history::table)
            .values((
                subject.eq(new_subject),
                body.eq(new_body),
                sent_at.eq(diesel::dsl::now),
            ))
            .returning(EmailRecord::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }
}
