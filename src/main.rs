use anyhow::Result;
use chrono::{Duration, Local};
use log;
use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
};
use structopt::StructOpt;
use tokio::sync::mpsc;
use tokio::time::sleep;

mod center;
mod district;
mod notification;
mod state;
use center::CentersResponse;
use notification::NotificationMessage;

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
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:59.0) Gecko/20100101 Firefox/59.0")
        .build()?;
    let states_response = client
        .get("https://cdn-api.co-vin.in/api/v2/admin/location/states")
        .send()
        .await?
        .json::<state::StatesResponse>()
        .await?;
    let mut state_id = None;
    for state in &states_response.states {
        if state.state_name == state_arg {
            state_id = Some(state.state_id);
        }
    }

    if let Some(state_id) = state_id {
        println!("State ID: {}", &state_id);
        let disticts_response = client
            .get(format!(
                "https://cdn-api.co-vin.in/api/v2/admin/location/districts/{}",
                state_id
            ))
            .send()
            .await?
            .json::<district::DistrictsResponse>()
            .await?;

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
                            let notifications = get_notifications(resp, age).ok();
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
                    .summary(format!("{}", message.center_name).as_str())
                    .body(format!("{} - {}", message.date, message.address).as_str())
                    .appname("cowin-notifier")
                    .icon("Toastify")
                    .show()?;
            }
            Ok(())
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

async fn get_centers(district_id: i64, date: String) -> Result<CentersResponse, reqwest::Error> {
    let thread_client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:59.0) Gecko/20100101 Firefox/59.0")
        .build()?;
    let resp = thread_client.get(format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/calendarByDistrict?district_id={}&date={}", district_id, date)).send()
        .await?
        .json::<CentersResponse>()
        .await?;
    Ok(resp)
}

fn get_notifications(resp: CentersResponse, age: usize) -> Result<Vec<NotificationMessage>> {
    let mut notifications = vec![];

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .append(true)
        .open("cowin-notifications-logs.json")?;
    let read_buffer = BufReader::new(&file);
    let prev_notifs: Vec<NotificationMessage> = serde_json::from_reader(read_buffer)?;

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
                    notifications.push(NotificationMessage {
                        center_name: center.name.clone(),
                        address: center.address.clone(),
                        date: session.date.clone(),
                    })
                } else if age == 45 && session.min_age_limit == 45 {
                    notifications.push(NotificationMessage {
                        center_name: center.name.clone(),
                        address: center.address.clone(),
                        date: session.date.clone(),
                    })
                }
            }
        }
    }
    let mut write_buffer = BufWriter::new(file);
    for notification in &notifications {
        serde_json::to_writer(&mut write_buffer, notification)?;
    }
    Ok(notifications)
}
