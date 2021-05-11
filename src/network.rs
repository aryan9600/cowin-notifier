use anyhow::Result;

use crate::{center::CentersResponse, district::DistrictsResponse, state::StatesResponse};

pub async fn get_centers(
    district_id: i64,
    date: String,
) -> Result<CentersResponse, reqwest::Error> {
    let client = prepare_client()?;
    let resp = client.get(format!("https://cdn-api.co-vin.in/api/v2/appointment/sessions/calendarByDistrict?district_id={}&date={}", district_id, date)).send()
        .await?
        .json::<CentersResponse>()
        .await?;
    Ok(resp)
}

pub async fn get_states(client: &reqwest::Client) -> Result<StatesResponse, reqwest::Error> {
    client
        .get("https://cdn-api.co-vin.in/api/v2/admin/location/states")
        .send()
        .await?
        .json::<StatesResponse>()
        .await
}

pub async fn get_districts(
    client: &reqwest::Client,
    state_id: i64,
) -> Result<DistrictsResponse, reqwest::Error> {
    client
        .get(format!(
            "https://cdn-api.co-vin.in/api/v2/admin/location/districts/{}",
            state_id
        ))
        .send()
        .await?
        .json::<DistrictsResponse>()
        .await
}

pub fn prepare_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:59.0) Gecko/20100101 Firefox/59.0")
        .build()
}
