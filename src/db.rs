use anyhow::Result;
use rusqlite::params;
use rusqlite::Connection;

use crate::notification::NotificationMessage;

pub fn db_conn(path: String) -> Result<Connection> {
    let db = Connection::open(&path)?;
    Ok(db)
}

pub fn setup_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notification (
            id          INTEGER PRIMARY KEY,
            center_name TEXT NOT NULL,
            address     TEXT NOT NULL,
            date        TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn fetch_notifications_from_db(conn: &Connection) -> Result<Vec<NotificationMessage>> {
    let mut stmt = conn.prepare("SELECT center_name, address, date FROM notification")?;
    let prev_notifs_iter = stmt.query_map([], |row| {
        Ok(NotificationMessage {
            center_name: row.get(0)?,
            address: row.get(1)?,
            date: row.get(2)?,
        })
    })?;

    let mut prev_notifs = vec![];
    for notif in prev_notifs_iter {
        if let Ok(notif) = notif {
            prev_notifs.push(notif);
        }
    }
    Ok(prev_notifs)
}

pub fn insert_notificatons_to_db(
    conn: &Connection,
    notifications: &[NotificationMessage],
) -> Result<()> {
    for notification in notifications {
        conn.execute(
            "INSERT INTO notification (center_name, address, date) values (?1, ?2, ?3)",
            params![
                notification.center_name,
                notification.address,
                notification.date
            ],
        )?;
    }
    Ok(())
}
