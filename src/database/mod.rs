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
    use crate::{error::MailerError, model::recipient::Recipient};

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

        test_script_for_recipient(&mut db).expect("Failed to run test_script_for_recipient");
        test_script_for_group(&mut db).expect("Failed to run test_script_for_group");
        test_script_for_recipient_group(&mut db)
            .expect("Failed to run test_script_for_recipient_group");
        test_script_for_template(&mut db).expect("Failed to run test_script_for_template");

        drop(db);
        std::fs::remove_file(DB_PATH).expect("Failed to remove test.db");
    }

    fn test_script_for_recipient(db: &mut Database) -> Result<(), MailerError> {
        let nr = db.new_recipient("me".to_string(), "me@domain.com".to_string())?;
        assert!(!db.list_recipients()?.is_empty());

        // Test for updating recipient
        let updated_recipient =
            db.update_recipient(nr.id, "me2".to_string(), "me2@domain.com".to_string())?;
        assert_eq!(updated_recipient.name, "me2");
        assert_eq!(updated_recipient.email, "me2@domain.com");
        assert_eq!(updated_recipient.id, nr.id);

        // Test for removing recipient
        let removed_recipient = db.remove_recipient(nr.id)?;
        assert_eq!(removed_recipient.name, "me2");
        assert_eq!(removed_recipient.id, nr.id);
        assert_eq!(removed_recipient.email, "me2@domain.com");
        assert!(db.list_recipients()?.is_empty());

        Ok(())
    }

    fn test_script_for_group(db: &mut Database) -> Result<(), MailerError> {
        let ng = db.new_group("test".to_string())?;
        assert!(!db.list_groups()?.is_empty());

        // Test for updating group
        let updated_group = db.update_group(ng.id, "test2".to_string())?;
        assert_eq!(updated_group.name, "test2");
        assert_eq!(updated_group.id, ng.id);

        // Test for removing group
        let removed_group = db.remove_group(ng.id)?;
        assert_eq!(removed_group.id, ng.id);
        assert_eq!(removed_group.name, "test2");
        assert!(db.list_groups()?.is_empty());

        Ok(())
    }

    /// Test for removing recipient from group
    /// 1. add new recipient and new group
    /// 2. add recipient to group
    /// 3. remove recipient from group
    fn test_script_for_recipient_group(db: &mut Database) -> Result<(), MailerError> {
        let nr2 = db.new_recipient("Some One".to_string(), "someone@domain.com".to_string())?;
        let ng2 = db.new_group("test3".to_string())?;
        db.add_recipient_to_group(ng2.id, nr2.id)?;
        let res = db.list_recipients_in_group(ng2.id)?;
        assert_eq!(res.len(), 1);
        assert_eq!(
            res[0],
            Recipient {
                id: nr2.id,
                name: nr2.name.clone(),
                email: nr2.email.clone(),
            }
        );
        db.remove_recipient_from_group(ng2.id, nr2.id)?;
        let res = db.list_recipients_in_group(ng2.id)?;
        assert_eq!(res.len(), 0);
        Ok(())
    }

    fn test_script_for_template(db: &mut Database) -> Result<(), MailerError> {
        let nt = db.new_template("test".to_string(), "template {name}".to_string())?;
        assert!(!db.list_templates()?.is_empty());

        // Test for updating template
        let updated_template = db.update_template(
            nt.id,
            "test2".to_string(),
            "template {name} {version}".to_string(),
        )?;
        assert_eq!(updated_template.name, "test2");
        assert_eq!(updated_template.id, nt.id);

        // Test for removing template
        let removed_template = db.remove_template(nt.id)?;
        assert_eq!(removed_template.id, nt.id);
        assert_eq!(removed_template.name, "test2");
        assert!(db.list_templates()?.is_empty());

        Ok(())
    }
}
