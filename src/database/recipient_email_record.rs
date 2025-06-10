use crate::error::MailerError;
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn add_recipient_email_record(
        &mut self,
        add_email_history_id: i32,
        add_recipient_id: i32,
    ) -> Result<(), MailerError> {
        use schema::email_history_recipients::dsl::*;

        diesel::insert_into(schema::email_history_recipients::table)
            .values((
                email_history_id.eq(add_email_history_id),
                recipient_id.eq(add_recipient_id),
            ))
            .execute(&mut self.connection)
            .map_err(MailerError::from)?;

        Ok(())
    }
}
