use std::collections::HashMap;

use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};

use crate::database::schema::templates;

#[derive(Debug, Clone, Queryable, Insertable, Selectable, Identifiable, PartialEq, Eq)]
#[diesel(table_name = templates)]
#[diesel(primary_key(id))]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub format_string: String,
}

impl Template {
    /// Apply the template to the given data.
    pub fn format(&self, data: HashMap<String, String>) -> Result<String, String> {
        // convert HashMap<String, String> to HashMap<&str, String>
        let data = data
            .iter()
            .map(|(k, v)| (k.as_str(), v))
            .collect::<HashMap<_, _>>();

        new_string_template::template::Template::new(self.format_string.clone())
            .render(&data)
            .map_err(|e| format!("Failed to render template: {}", e))
    }
}
