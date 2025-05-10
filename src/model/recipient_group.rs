use diesel::{
    Selectable,
    prelude::{Associations, Identifiable, Insertable, Queryable},
};

use crate::{
    database::schema::group_recipients,
    model::{group::Group, recipient::Recipient},
};

#[derive(
    Debug, Clone, Queryable, Insertable, Selectable, Identifiable, Associations, PartialEq, Eq,
)]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Recipient))]
#[diesel(table_name = group_recipients)]
#[diesel(primary_key(group_id, recipient_id))]
pub struct RecipientGroup {
    pub group_id: i32,
    pub recipient_id: i32,
}
