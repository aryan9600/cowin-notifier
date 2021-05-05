use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CentersResponse{
    pub centers: Vec<Center>
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
