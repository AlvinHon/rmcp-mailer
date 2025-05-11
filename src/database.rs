use diesel::prelude::*;

pub(crate) mod schema {

    diesel::table! {
        recipients {
            id -> Integer,
            email -> Text,
        }
    }

    diesel::table! {
        groups {
            id -> Integer,
            name -> Text,
        }
    }

    diesel::table! {
        group_recipients (group_id, recipient_id) {
            group_id -> Integer,
            recipient_id -> Integer,
        }
    }

    diesel::joinable!(group_recipients -> groups (group_id));
    diesel::joinable!(group_recipients -> recipients (recipient_id));

    diesel::allow_tables_to_appear_in_same_query!(recipients, groups, group_recipients,);

    pub(crate) fn create_all_tables_sqls() -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS recipients (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                email TEXT NOT NULL UNIQUE
            );",
            "CREATE TABLE IF NOT EXISTS groups (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL UNIQUE
            );",
            "CREATE TABLE IF NOT EXISTS group_recipients (
                group_id INTEGER, 
                recipient_id INTEGER, 
                PRIMARY KEY (group_id, recipient_id), 
                FOREIGN KEY (group_id) REFERENCES groups(id), 
                FOREIGN KEY (recipient_id) REFERENCES recipients(id)
            );",
        ]
    }
}

use crate::{
    error::MailerError,
    model::{group::Group, recipient::Recipient, recipient_group::RecipientGroup},
};

pub struct Database {
    connection: diesel::SqliteConnection,
}

impl Database {
    pub fn new(db_path: &str) -> Self {
        let mut connection =
            SqliteConnection::establish(db_path).expect("Error connecting to database");

        for table_sql in schema::create_all_tables_sqls() {
            diesel::sql_query(table_sql)
                .execute(&mut connection)
                .expect("Error creating tables");
        }
        Self { connection }
    }

    pub fn list_recipients(&mut self) -> Result<Vec<Recipient>, MailerError> {
        use schema::recipients::dsl::*;

        recipients
            .load::<Recipient>(&mut self.connection)
            .map_err(MailerError::from)
    }

    pub fn list_groups(&mut self) -> Result<Vec<Group>, MailerError> {
        use schema::groups::dsl::*;

        groups
            .load::<Group>(&mut self.connection)
            .map_err(MailerError::from)
    }

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

#[test]
fn test_database() {
    const DB_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test.db");
    _ = std::fs::remove_file(DB_PATH);

    let mut db = Database::new(DB_PATH);

    // Test that the database is empty
    assert!(db.list_recipients().unwrap().is_empty());
    assert!(db.list_groups().unwrap().is_empty());

    // Test for adding recipient
    let nr = db.new_recipient("me@domain.com".to_string()).unwrap();
    assert!(!db.list_recipients().unwrap().is_empty());

    // Test for adding group
    let ng = db.new_group("test".to_string()).unwrap();
    assert!(!db.list_groups().unwrap().is_empty());

    // Test for adding recipient in group
    assert!(db.list_recipients_in_group(ng.id).unwrap().is_empty());
    db.add_recipient_to_group(ng.id, nr.id)
        .expect("Failed to add recipient to group");
    let res = db.list_recipients_in_group(ng.id).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(
        res[0],
        Recipient {
            id: nr.id,
            email: nr.email.clone(),
        }
    );

    // Test for updating recipient
    let updated_recipient = db
        .update_recipient(nr.id, "me2@domain.com".to_string())
        .expect("Failed to update recipient");
    assert_eq!(updated_recipient.email, "me2@domain.com");
    assert_eq!(updated_recipient.id, nr.id);

    // Test for removing recipient
    let removed_recipient = db
        .remove_recipient(nr.id)
        .expect("Failed to remove recipient");
    assert_eq!(removed_recipient.id, nr.id);
    assert_eq!(removed_recipient.email, "me2@domain.com");
    assert!(db.list_recipients().unwrap().is_empty());
    assert!(db.list_recipients_in_group(ng.id).unwrap().is_empty());

    // Test for updating group
    let updated_group = db
        .update_group(ng.id, "test2".to_string())
        .expect("Failed to update group");
    assert_eq!(updated_group.name, "test2");
    assert_eq!(updated_group.id, ng.id);

    // Test for removing group
    let removed_group = db.remove_group(ng.id).expect("Failed to remove group");
    assert_eq!(removed_group.id, ng.id);
    assert_eq!(removed_group.name, "test2");
    assert!(db.list_groups().unwrap().is_empty());

    // Test for removing recipient from group
    // 1. add new recipient and new group
    // 2. add recipient to group
    // 3. remove recipient from group
    let nr2 = db.new_recipient("someone@domain.com".to_string()).unwrap();
    let ng2 = db.new_group("test3".to_string()).unwrap();
    db.add_recipient_to_group(ng2.id, nr2.id)
        .expect("Failed to add recipient to group");
    let res = db.list_recipients_in_group(ng2.id).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(
        res[0],
        Recipient {
            id: nr2.id,
            email: nr2.email.clone(),
        }
    );
    db.remove_recipient_from_group(ng2.id, nr2.id)
        .expect("Failed to remove recipient from group");
    let res = db.list_recipients_in_group(ng2.id).unwrap();
    assert_eq!(res.len(), 0);

    drop(db);
    std::fs::remove_file(DB_PATH).expect("Failed to remove test.db");
}
