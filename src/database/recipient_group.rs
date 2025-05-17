use crate::{
    error::MailerError,
    model::{group::Group, recipient::Recipient, recipient_group::RecipientGroup},
};
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn list_recipients_in_group(
        &mut self,
        by_group_id: i32,
    ) -> Result<Vec<Recipient>, MailerError> {
        let res_groups = schema::groups::table
            .filter(schema::groups::id.eq(by_group_id))
            .load::<Group>(&mut self.connection)
            .map_err(MailerError::from)?;
        RecipientGroup::belonging_to(&res_groups)
            .inner_join(schema::recipients::table)
            .select(Recipient::as_select())
            .load::<Recipient>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn find_recipients_by_group_id(
        &mut self,
        group_id: i32,
    ) -> Result<Vec<Recipient>, MailerError> {
        schema::group_recipients::table
            .filter(schema::group_recipients::group_id.eq(group_id))
            .inner_join(schema::recipients::table)
            .select(Recipient::as_select())
            .load::<Recipient>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn add_recipient_to_group(
        &mut self,
        group_id: i32,
        recipient_id: i32,
    ) -> Result<(), MailerError> {
        diesel::insert_into(schema::group_recipients::table)
            .values((
                schema::group_recipients::group_id.eq(group_id),
                schema::group_recipients::recipient_id.eq(recipient_id),
            ))
            .execute(&mut self.connection)
            .map_err(MailerError::from)?;
        Ok(())
    }

    pub fn remove_recipient_from_group(
        &mut self,
        group_id: i32,
        recipient_id: i32,
    ) -> Result<(), MailerError> {
        diesel::delete(schema::group_recipients::table)
            .filter(
                schema::group_recipients::group_id
                    .eq(group_id)
                    .and(schema::group_recipients::recipient_id.eq(recipient_id)),
            )
            .execute(&mut self.connection)
            .map_err(MailerError::from)?;
        Ok(())
    }
}
