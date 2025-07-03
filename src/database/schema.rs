diesel::table! {
    recipients {
        id -> Integer,
        name -> Text,
        email -> Text,
        status -> Text,
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

diesel::table! {
    event_attendees {
        id -> Integer,
        event_id -> Integer,
        recipient_id -> Integer,
        invitation_type -> Text,
    }
}

diesel::joinable!(group_recipients -> groups (group_id));
diesel::joinable!(group_recipients -> recipients (recipient_id));
diesel::joinable!(email_history_recipients -> recipients (recipient_id));
diesel::joinable!(email_history_recipients -> email_history (email_history_id));
diesel::joinable!(event_attendees -> events (event_id));
diesel::joinable!(event_attendees -> recipients (recipient_id));

diesel::allow_tables_to_appear_in_same_query!(
    recipients,
    groups,
    group_recipients,
    email_history,
    email_history_recipients,
    templates,
    events,
    event_attendees,
);

pub(crate) fn create_all_tables_sqls() -> Vec<&'static str> {
    vec![
        "PRAGMA foreign_keys = ON;",
        "CREATE TABLE IF NOT EXISTS recipients (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                status TEXT NOT NULL CHECK (status IN ('Active', 'Inactive'))
            );",
        "CREATE TABLE IF NOT EXISTS groups (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL UNIQUE
            );",
        "CREATE TABLE IF NOT EXISTS group_recipients (
                group_id INTEGER, 
                recipient_id INTEGER, 
                PRIMARY KEY (group_id, recipient_id), 
                FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
                FOREIGN KEY (recipient_id) REFERENCES recipients(id) ON DELETE CASCADE
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
        "CREATE TABLE IF NOT EXISTS event_attendees (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
                event_id INTEGER NOT NULL, 
                recipient_id INTEGER NOT NULL, 
                invitation_type TEXT NOT NULL CHECK (invitation_type IN ('Required', 'Optional')),
                FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
                FOREIGN KEY (recipient_id) REFERENCES recipients(id) ON DELETE CASCADE
            );",
    ]
}
