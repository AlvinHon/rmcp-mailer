use crate::{error::MailerError, model::template::Template};
use diesel::prelude::*;

use super::{Database, schema};

impl Database {
    pub fn list_templates(&mut self) -> Result<Vec<Template>, MailerError> {
        use schema::templates::dsl::*;

        templates
            .load::<Template>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn find_template_by_name(
        &mut self,
        template_name: String,
    ) -> Result<Template, MailerError> {
        use schema::templates::dsl::*;

        templates
            .filter(name.eq(template_name))
            .first::<Template>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn new_template(
        &mut self,
        name: String,
        format_string: String,
    ) -> Result<Template, MailerError> {
        diesel::insert_into(schema::templates::table)
            .values((
                schema::templates::name.eq(name),
                schema::templates::format_string.eq(format_string),
            ))
            .returning(Template::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn update_template(
        &mut self,
        template_id: i32,
        new_name: String,
        new_format_string: String,
    ) -> Result<Template, MailerError> {
        use schema::templates::dsl::*;

        diesel::update(templates.filter(id.eq(template_id)))
            .set((name.eq(new_name), format_string.eq(new_format_string)))
            .returning(Template::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn remove_template(&mut self, template_id: i32) -> Result<Template, MailerError> {
        use schema::templates::dsl::*;

        diesel::delete(templates.filter(id.eq(template_id)))
            .returning(Template::as_returning())
            .get_result(&mut self.connection)
            .map_err(MailerError::from)
    }
}
