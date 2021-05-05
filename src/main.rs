use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use chrono::{Local, Duration};
use tokio::sync::mpsc;
use tokio::time::sleep;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CentersResponse{
    centers: Vec<Center>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Center {
    pub center_id: i64,
    pub name: String,
    pub address: String,
    pub state_name: String,
    pub district_name: String,
    pub block_name: String,
    pub pincode: i64,
    pub lat: i64,
    pub long: i64,
    pub from: String,
    pub to: String,
    pub fee_type: String,
    pub sessions: Vec<Session>,
    pub vaccine_fees: Option<Vec<VaccineFee>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub date: String,
    pub available_capacity: i64,
    pub min_age_limit: i64,
    pub vaccine: String,
    pub slots: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaccineFee {
    pub vaccine: String,
    pub fee: String,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatesResponse {
    pub states: Vec<State>,
    pub ttl: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub state_id: i64,
    pub state_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistrictsResponse {
    pub districts: Vec<District>,
    pub ttl: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct District {
    pub district_id: i64,
    pub district_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationMessage {
    pub center_name: String,
    pub address: String,
    pub date: String
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let opt = Opt::from_args();
    let district_arg = opt.district;
    let age = opt.age;
    let state_arg = opt.state;
    let states_response = reqwest::get("https://cdn-api.co-vin.in/api/v2/admin/location/states")
        .await?
        .json::<StatesResponse>()
        .await?;
    
    let mut state_id = None;
    for state in &states_response.states {
        if state.state_name == state_arg {
            state_id = Some(state.state_id);
        }
    }

    if let Some(state_id) = state_id {
        println!("{}", &state_id);
        let disticts_response = reqwest::get(format!("https://cdn-api.co-vin.in/api/v2/admin/location/districts/{}", state_id))
            .await?
            .json::<DistrictsResponse>()
            .await?;

        let mut district_id = None;
        for district in &disticts_response.districts {
            if district.district_name == district_arg {
                district_id = Some(district.district_id);
            }
        }

        if let Some(district_id) = district_id {
            println!("{}", &district_id);

            let (tx, mut rx) = mpsc::channel(16);            
            let tx2 = tx.clone();
            let tx3 = tx.clone();
            let tx4 = tx.clone();
            let age2 = age.clone();
            let age3 = age.clone();
            let age4 = age.clone();
            let district_id2 = district_id.clone();
            let district_id3 = district_id.clone();
            let district_id4 = district_id.clone();

            tokio::spawn(async move {
                loop {
                    let local = Local::now();
                    let date = local.format("%d-%m-%Y");
                    let resp = reqwest::get(format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/calendarByDistrict?district_id={}&date={}", district_id, date))
                        .await
                        .unwrap()
                        .json::<CentersResponse>()
                        .await
                        .unwrap();
                    for center in &resp.centers {
                        for session in &center.sessions  {
                            if session.available_capacity > 0 {
                                if age == 18 && session.min_age_limit == 18 {
                                    tx.send(NotificationMessage {
                                        center_name: center.name.clone(),
                                        address: center.address.clone(),
                                        date: session.date.clone()
                                    }).await.ok();
                                } else if age == 45 && session.min_age_limit == 45 {
                                    tx.send(NotificationMessage {
                                        center_name: center.name.clone(),
                                        address: center.address.clone(),
                                        date: session.date.clone()
                                    }).await.ok();
                                }  
                            }
                        }
                    }
                    sleep(tokio::time::Duration::from_secs(30)).await;
                }
            });

            tokio::spawn(async move {
                loop {
                    let local = Local::now() + Duration::days(7);
                    let date = local.format("%d-%m-%Y");
                    let resp = reqwest::get(format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/calendarByDistrict?district_id={}&date={}", district_id2, date))
                        .await
                        .unwrap()
                        .json::<CentersResponse>()
                        .await
                        .unwrap();
                    for center in &resp.centers {
                        for session in &center.sessions  {
                            if session.available_capacity > 0 {
                                if age2 == 18 && session.vaccine == "COVAXIN" {
                                    tx2.send(NotificationMessage {
                                        center_name: center.name.clone(),
                                        address: center.address.clone(),
                                        date: session.date.clone()
                                    }).await.ok();
                                } else if age2 == 45 && session.min_age_limit == 45 {
                                    tx2.send(NotificationMessage {
                                        center_name: center.name.clone(),
                                        address: center.address.clone(),
                                        date: session.date.clone()
                                    }).await.ok();
                                }  
                            }
                        }
                    }
                    sleep(tokio::time::Duration::from_secs(30)).await;
                }
            });

            tokio::spawn(async move {
                loop {
                    let local = Local::now() + Duration::days(14);
                    let date = local.format("%d-%m-%Y");
                    let resp = reqwest::get(format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/calendarByDistrict?district_id={}&date={}", district_id3, date))
                        .await
                        .unwrap()
                        .json::<CentersResponse>()
                        .await
                        .unwrap();
                    for center in &resp.centers {
                        for session in &center.sessions  {
                            if session.available_capacity > 0 {
                                if age3 == 18 && session.vaccine == "COVAXIN" {
                                    tx3.send(NotificationMessage {
                                        center_name: center.name.clone(),
                                        address: center.address.clone(),
                                        date: session.date.clone()
                                    }).await.ok();
                                } else if age3 == 45 && session.min_age_limit == 45 {
                                    tx3.send(NotificationMessage {
                                        center_name: center.name.clone(),
                                        address: center.address.clone(),
                                        date: session.date.clone()
                                    }).await.ok();
                                }  
                            }
                        }
                    }
                    sleep(tokio::time::Duration::from_secs(30)).await;
                }
            });

            tokio::spawn(async move {
                loop {
                    let local = Local::now() + Duration::days(21);
                    let date = local.format("%d-%m-%Y");
                    let resp = reqwest::get(format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/calendarByDistrict?district_id={}&date={}", district_id4, date))
                        .await
                        .unwrap()
                        .json::<CentersResponse>()
                        .await
                        .unwrap();
                    for center in &resp.centers {
                        for session in &center.sessions  {
                            if session.available_capacity > 0 {
                                if age4 == 18 && session.vaccine == "COVAXIN" {
                                    tx4.send(NotificationMessage {
                                        center_name: center.name.clone(),
                                        address: center.address.clone(),
                                        date: session.date.clone()
                                    }).await.ok();
                                } else if age4 == 45 && session.min_age_limit == 45 {
                                    tx4.send(NotificationMessage {
                                        center_name: center.name.clone(),
                                        address: center.address.clone(),
                                        date: session.date.clone()
                                    }).await.ok();
                                }  
                            }
                        }
                    }
                    sleep(tokio::time::Duration::from_secs(30)).await;
                }
            });

            while let Some(message) = rx.recv().await {
                use notify_rust::Notification;

                println!("something");
                Notification::new()
                    .summary(format!("{}", message.center_name).as_str())
                    .body(format!("{} - {}", message.date, message.address).as_str())
                    .appname("cowin-notifier")
                    .icon("Toastify")
                    .show().unwrap();
            }
            Ok(())
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}
