use chrono::NaiveTime;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;

use crate::models::database::{
    take_time::TakeTime, 
    treatment::Treatment, 
    time_preference::TimePreference, 
    user_account::UserAccount, 
    user_info::UserInfo,
    unit::Unit,
    dosage::Dosage,
    concentration::Concentration
};

#[derive(Serialize, Deserialize)]
pub struct TakeTimeForm {
    pub time: NaiveTime,
    pub day: String,
    pub preference: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TreatmentForm {
    pub name: String,
    pub unit: String,
    pub dosage: String,
    pub concentration: String,
    pub frequency: i32,
    pub color: String,
    pub times: Vec<TakeTimeForm>,
}

impl TreatmentForm {
    pub fn read(email: String, conn: &MysqlConnection) -> Result<Vec<TreatmentForm>, Error> {
        let account_id = UserAccount::read_by_email(email, conn)?.account_id.unwrap();
        let user_id = UserInfo::read_by_account_id(account_id, conn)?.user_id.unwrap();
        let db_treatments = Treatment::read_by_user_id(user_id, conn)?;

        let mut form_treatments: Vec<TreatmentForm> = Vec::new();

        for t in db_treatments {
            let times = TakeTimeForm::read(t.treatment_id.unwrap(), conn)?;
            let form_elem = TreatmentForm {
                name: t.name,
                unit: Unit::read_by_id(t.unit_id, conn)?.unit_name,
                dosage: Dosage::read_by_id(t.dosage_id, conn)?.dosage_type,
                concentration: Concentration::read_by_id(t.concentration_id, conn)?.concentration_amount,
                frequency: t.frequency,
                color: t.color,
                times: times,
            };
            form_treatments.push(form_elem);

        }

        Ok(form_treatments)
    }

    pub fn create(email: String, form: TreatmentForm, conn: &MysqlConnection) -> Result<(), Error> {
        // get user_id
        // build Treatment
        // create Treatment
        // for each time
        // build TimeForm
        // create TimeForm
        // end for
        let account_id = UserAccount::read_by_email(email, conn)?.account_id.unwrap();
        let user_id = UserInfo::read_by_account_id(account_id, conn)?.user_id.unwrap();

        let treatment = Treatment {
            treatment_id: None,
            user_id: user_id,
            name: form.name,
            unit_id: Unit::read_by_value(form.unit, conn)?.unit_id,
            dosage_id: Dosage::read_by_value(form.dosage, conn)?.dosage_id,
            concentration_id: Concentration::read_by_value(form.concentration, conn)?.concentration_id,
            frequency: form.frequency,
            color: form.color,
        };

        let new_treatment = Treatment::create(treatment, conn)?;

        for t in form.times {
            let time = TakeTimeForm {
                time: t.time,
                day: t.day,
                preference: match t.preference {
                    Some(p) => Some(TimePreference::read_by_value(p, conn)?.preference_type),
                    None => None,
                }
            };
            TakeTimeForm::create(time, new_treatment.treatment_id.unwrap(), conn)?;
        }

        Ok(())
    }
}

impl TakeTimeForm {
    pub fn read(treatment_id: i32, conn: &MysqlConnection) -> Result<Vec<TakeTimeForm>, Error> {
        let db_times = TakeTime::read_by_treatment_id(treatment_id, conn)?;

        let mut form_times: Vec<TakeTimeForm> = Vec::new();

        for t in db_times {
            let pref = match t.preference_id {
                Some(id) => {
                    match TimePreference::read_by_id(id, conn) {
                        Ok(p) => Some(p.preference_type),
                        Err(e) => return Err(e),
                    }
                },
                None => None,
            };
            form_times.push(TakeTimeForm {
                time: t.time,
                day: t.day,
                preference: match pref {
                    Some(p) => Some(p.clone()),
                    None => None,
                },
            })
        }
        Ok(form_times)
    }

    pub fn create(form: TakeTimeForm, treatment_id: i32, conn: &MysqlConnection) -> Result<TakeTime, Error> {
        
        let pref_id = match form.preference {
            Some(p) => Some(TimePreference::read_by_value(p, conn)?.preference_id),
            None => None,
        };
        TakeTime::create(
            TakeTime {
                take_time_id: None,
                treatment_id: treatment_id,
                time: form.time,
                day: form.day,
                preference_id: pref_id,
            }, 
            conn
        )
    }
}