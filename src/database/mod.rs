pub(crate) mod group;
pub(crate) mod recipient;
pub(crate) mod recipient_group;
pub(crate) mod schema;
pub(crate) mod template;

use std::fmt::Debug;

use diesel::prelude::*;

use crate::config::DatabaseConfig;

pub struct Database {
    connection: diesel::SqliteConnection,
}

impl Database {
    pub fn new(config: DatabaseConfig) -> Self {
        let mut connection =
            SqliteConnection::establish(&config.db_path).expect("Error connecting to database");

        for table_sql in schema::create_all_tables_sqls() {
            diesel::sql_query(table_sql)
                .execute(&mut connection)
                .expect("Error creating tables");
        }
        Self { connection }
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("connection", &"SqliteConnection")
            .finish()
    }
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::recipient::Recipient;

    #[test]
    fn test_database() {
        const DB_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test.db");
        _ = std::fs::remove_file(DB_PATH);

        let mut db = Database::new(DatabaseConfig {
            db_path: DB_PATH.to_string(),
        });

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
}
