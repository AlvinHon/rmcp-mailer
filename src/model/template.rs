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
