use crate::{error::MailerError, model::group::Group};
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn list_groups(&mut self) -> Result<Vec<Group>, MailerError> {
        use schema::groups::dsl::*;

        groups
            .load::<Group>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn find_group_by_name(&mut self, group_name: String) -> Result<Group, MailerError> {
        use schema::groups::dsl::*;

        groups
            .filter(name.eq(group_name))
            .first::<Group>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn new_group(&mut self, group_name: String) -> Result<Group, MailerError> {
        use schema::groups::dsl::*;
        diesel::insert_into(groups)
            .values(name.eq(group_name))
            .returning(Group::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn update_group(&mut self, group_id: i32, new_name: String) -> Result<Group, MailerError> {
        use schema::groups::dsl::*;

        diesel::update(groups.filter(id.eq(group_id)))
            .set(name.eq(new_name))
            .returning(Group::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn remove_group(&mut self, group_id: i32) -> Result<Group, MailerError> {
        use schema::groups::dsl::*;

        diesel::delete(groups.filter(id.eq(group_id)))
            .returning(Group::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }
}
