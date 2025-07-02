use crate::{
    error::MailerError,
    model::recipient::{Recipient, RecipientStatus},
};
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn list_recipients(&mut self) -> Result<Vec<Recipient>, MailerError> {
        use schema::recipients::dsl::*;

        recipients
            .filter(status.eq(RecipientStatus::Active))
            .load::<Recipient>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn find_recipient_by_email(&mut self, email_str: String) -> Result<Recipient, MailerError> {
        use schema::recipients::dsl::*;

        recipients
            .filter(email.eq(email_str).and(status.eq(RecipientStatus::Active)))
            .first::<Recipient>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn new_recipient(
        &mut self,
        name_str: String,
        email_str: String,
    ) -> Result<Recipient, MailerError> {
        use schema::recipients::dsl::*;

        diesel::insert_into(recipients)
            .values((
                name.eq(name_str),
                email.eq(email_str),
                status.eq(RecipientStatus::Active),
            ))
            .returning(Recipient::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn update_recipient(
        &mut self,
        recipient_id: i32,
        new_name: String,
        new_email: String,
    ) -> Result<Recipient, MailerError> {
        use schema::recipients::dsl::*;

        diesel::update(recipients.filter(id.eq(recipient_id)))
            .set((name.eq(new_name), email.eq(new_email)))
            .returning(Recipient::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn remove_recipient(&mut self, recipient_id: i32) -> Result<Recipient, MailerError> {
        use schema::recipients::dsl::*;

        // inactive the recipient instead of deleting
        diesel::update(recipients.filter(id.eq(recipient_id)))
            .set(status.eq(RecipientStatus::Inactive))
            .returning(Recipient::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }
}
