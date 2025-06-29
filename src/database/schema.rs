diesel::table! {
    recipients {
        id -> Integer,
        name -> Text,
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

diesel::table! {
    templates {
        id -> Integer,
        name -> Text,
        format_string -> Text,
    }
}

diesel::table! {
    email_history {
        id -> Integer,
        subject -> Text,
        body -> Text,
        sent_at -> Timestamp,
    }
}

diesel::table! {
    email_history_recipients (email_history_id, recipient_id) {
        email_history_id -> Integer,
        recipient_id -> Integer,
    }
}

diesel::table! {
    events {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        is_all_day -> Bool,
    }
}

diesel::joinable!(group_recipients -> groups (group_id));
diesel::joinable!(group_recipients -> recipients (recipient_id));
diesel::joinable!(email_history_recipients -> recipients (recipient_id));
diesel::joinable!(email_history_recipients -> email_history (email_history_id));

diesel::allow_tables_to_appear_in_same_query!(
    recipients,
    groups,
    group_recipients,
    email_history,
    email_history_recipients,
    templates
);

pub(crate) fn create_all_tables_sqls() -> Vec<&'static str> {
    vec![
        "CREATE TABLE IF NOT EXISTS recipients (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL,
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
        "CREATE TABLE IF NOT EXISTS templates (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL UNIQUE, 
                format_string TEXT NOT NULL
            );",
        "CREATE TABLE IF NOT EXISTS email_history (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                subject TEXT NOT NULL, 
                body TEXT NOT NULL, 
                sent_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
        "CREATE TABLE IF NOT EXISTS email_history_recipients (
                email_history_id INTEGER, 
                recipient_id INTEGER, 
                PRIMARY KEY (email_history_id, recipient_id), 
                FOREIGN KEY (email_history_id) REFERENCES email_history(id), 
                FOREIGN KEY (recipient_id) REFERENCES recipients(id)
            );",
        "CREATE TABLE IF NOT EXISTS events (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                title TEXT NOT NULL, 
                description TEXT, 
                start_time DATETIME NOT NULL, 
                end_time DATETIME, 
                is_all_day BOOLEAN NOT NULL DEFAULT 0
            );",
    ]
}
