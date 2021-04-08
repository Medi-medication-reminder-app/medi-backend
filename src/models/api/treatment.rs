use chrono::NaiveTime;

#[derive(Serialize, Deserialize)]
pub struct TakeTimeForm {
    pub time: NaiveTime,
    pub frequency: i32,
    pub day: String,
    pub preference: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TreatmentForm {
    pub name: String,
    pub unit: String,
    pub dosage: String,
    pub concentration: String,
    pub color: String,
    pub times: Vec<TakeTimeForm>,
}

// TODO: 
// TODO: get user_id from jwt->account_id->user_id
