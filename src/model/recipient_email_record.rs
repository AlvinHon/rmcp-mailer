use diesel::{
    Selectable,
    prelude::{Associations, Identifiable, Insertable, Queryable},
};

use crate::{
    database::schema::email_history_recipients,
    model::{email_record::EmailRecord, recipient::Recipient},
};

#[derive(
    Debug, Clone, Queryable, Insertable, Selectable, Identifiable, Associations, PartialEq, Eq,
)]
#[diesel(belongs_to(EmailRecord, foreign_key = email_history_id))]
#[diesel(belongs_to(Recipient))]
#[diesel(table_name = email_history_recipients)]
#[diesel(primary_key(email_history_id, recipient_id))]
pub struct RecipientEmailRecord {
    pub email_history_id: i32,
    pub recipient_id: i32,
}
