use crate::{error::MailerError, model::recipient::Recipient};
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn list_recipients(&mut self) -> Result<Vec<Recipient>, MailerError> {
        use schema::recipients::dsl::*;

        recipients
            .load::<Recipient>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn find_recipient_by_email(&mut self, email_str: String) -> Result<Recipient, MailerError> {
        use schema::recipients::dsl::*;

        recipients
            .filter(email.eq(email_str))
            .first::<Recipient>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn new_recipient(&mut self, email_str: String) -> Result<Recipient, MailerError> {
        use schema::recipients::dsl::*;

        diesel::insert_into(recipients)
            .values(email.eq(email_str))
            .returning(Recipient::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn update_recipient(
        &mut self,
        recipient_id: i32,
        new_email: String,
    ) -> Result<Recipient, MailerError> {
        use schema::recipients::dsl::*;

        diesel::update(recipients.filter(id.eq(recipient_id)))
            .set(email.eq(new_email))
            .returning(Recipient::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn remove_recipient(&mut self, recipient_id: i32) -> Result<Recipient, MailerError> {
        use schema::recipients::dsl::*;

        diesel::delete(recipients.filter(id.eq(recipient_id)))
            .returning(Recipient::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }
}
