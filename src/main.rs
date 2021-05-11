use anyhow::Result;
use chrono::{Duration, Local};
use std::fs::OpenOptions;
use structopt::StructOpt;
use tokio::sync::mpsc;
use tokio::time::sleep;

mod center;
mod db;
mod district;
mod network;
mod notification;
mod state;

use center::CentersResponse;
use network::{get_centers, get_districts, get_states, prepare_client};
use notification::NotificationMessage;

use crate::db::{db_conn, fetch_notifications_from_db, insert_notificatons_to_db, setup_table};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(help = "Enter your state")]
    state: String,

    #[structopt(help = "Which district do you want to the available slots in?")]
    district: String,

    #[structopt(help = "Which age group do you belong in? (Enter 18 for 18+ or 45 for 45+)")]
    age: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    let district_arg = opt.district;
    let age = opt.age;
    let state_arg = opt.state;
    let client = prepare_client()?;
    let states_response = get_states(&client).await?;
    let mut state_id = None;
    for state in &states_response.states {
        if state.state_name == state_arg {
            state_id = Some(state.state_id);
        }
    }

    if let Some(state_id) = state_id {
        println!("State ID: {}", &state_id);
        let disticts_response = get_districts(&client, state_id).await?;
        let mut district_id = None;
        for district in &disticts_response.districts {
            if district.district_name == district_arg {
                district_id = Some(district.district_id);
            }
        }

        if let Some(district_id) = district_id {
            println!("District ID: {}", &district_id);

            let (tx, mut rx) = mpsc::channel(128);
            let tx2 = tx.clone();
            let tx3 = tx.clone();
            let tx4 = tx.clone();
            let txes = vec![tx, tx2, tx3, tx4];

            OpenOptions::new()
                .create(true)
                .write(true)
                .open("cowin_notifications.db3")?;

            let conn = db_conn(String::from("./cowin_notifications.db3"))?;
            setup_table(&conn)?;

            let error_msg =
                "An error occured. Please mail this error message to contact@ieeevit.org, 
                or open an issue at https://github.com/aryan9600/cowin-notifier/issues/new.";

            let mut local = Local::now();
            for tx in txes {
                tokio::spawn(async move {
                    loop {
                        let date = local.format("%d-%m-%Y").to_string();
                        let resp = get_centers(district_id, date).await.ok();
                        if let Some(resp) = resp {
                            println!("{:?}", resp);
                            let notifications = get_notifications(resp, age, "./cowin_notifications.db3".to_string()).ok();
                            if let Some(notifications) = notifications {
                                for notif in notifications {
                                    match tx.send(notif).await {
                                        Ok(()) => {}
                                        Err(e) => log::error!("{} \n {}", error_msg, e),
                                    }
                                }
                            }
                        }
                        sleep(tokio::time::Duration::from_secs(30)).await;
                    }
                });
                local = local + Duration::days(7);
            }

            while let Some(message) = rx.recv().await {
                use notify_rust::Notification;

                Notification::new()
                    .summary(message.center_name.as_str())
                    .body(format!("{} - {}", message.date, message.address).as_str())
                    .appname("cowin-notifier")
                    .icon("Toastify")
                    .show()?;
            }
        } 
    }
    Ok(())
}

fn get_notifications(resp: CentersResponse, age: usize, db_path: String) -> Result<Vec<NotificationMessage>> {
    let mut notifications = vec![];
    let conn = db_conn(db_path)?;
    let prev_notifs = fetch_notifications_from_db(&conn)?;

    for center in &resp.centers {
        for session in &center.sessions {
            if session.available_capacity > 0 {
                let notification = NotificationMessage {
                    center_name: center.name.clone(),
                    address: center.address.clone(),
                    date: session.date.clone(),
                };
                if prev_notifs.contains(&notification) {
                    continue;
                }
                if age == 18 && session.min_age_limit == 18 {
                    notifications.push(notification);
                } else if age == 45 && session.min_age_limit == 45 {
                    notifications.push(notification);
                } else {
                }
            }
        }
    }
    insert_notificatons_to_db(&conn, &notifications)?;
    conn.close().ok();
    Ok(notifications)
}

#[cfg(test)]
mod tests {
    use std::{fs::{File, remove_file}, io::BufReader};

    use super::*;

    fn setup_test(db_path: String) -> Result<()> {
        let file = File::open("./test_data/db_notifs.json")?;
        let reader = BufReader::new(file);

        let db_notifs: Vec<NotificationMessage> = serde_json::from_reader(reader)?;
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(db_path.as_str())?;
        let conn = db_conn(db_path)?;
        setup_table(&conn)?;
        insert_notificatons_to_db(&conn, &db_notifs)?;
        conn.close().ok();
        Ok(())
    }

    #[test]
    fn test_get_notifications() -> Result<()>{
        let db_path = "./test_db_notifs.db3";
        setup_test(db_path.to_string())?;
        let file = File::open("./test_data/center_response.json")?;
        let reader = BufReader::new(file);

        let centers: CentersResponse = serde_json::from_reader(reader)?;
        let notifications = get_notifications(centers, 45, db_path.to_string())?;
        assert_eq!(notifications.len(), 2);
        let mut center_names = vec![];
        for notif in &notifications {
            center_names.push(notif.center_name.clone());
        } 
        center_names.sort();

        assert_eq!(center_names, vec!["Center 1".to_string(), "Center 4".to_string()]);

        remove_file(db_path)?;
        Ok(())
    }
}
