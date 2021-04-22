use chrono::{NaiveTime, Utc, Weekday, Datelike};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;

use crate::models::database::user_account::UserAccount;
use crate::models::database::user_info::UserInfo;
use crate::models::api::treatment::TreatmentForm;
use crate::models::database::take_log::TakenTreatmentLog;
use crate::models::database::treatment::Treatment;

#[derive(Serialize, Deserialize)]
pub struct TodayTreatment {
    pub name: String,
    pub unit: String,
    pub dosage: String,
    pub concentration: String,
    pub time: NaiveTime,
    pub preference: Option<String>,
    pub color: String,
    pub taken: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodayTakeForm {
    pub treatment_name: String,
    pub time: NaiveTime,
    pub taken: bool,
}

impl TodayTreatment {
    fn weekday_to_string(d: Weekday) -> String {
        match d {
            Weekday::Mon => String::from("Monday"),
            Weekday::Tue => String::from("Tuesday"),
            Weekday::Wed => String::from("Wednesday"),
            Weekday::Thu => String::from("Thursday"),
            Weekday::Fri => String::from("Friday"),
            Weekday::Sat => String::from("Saturday"),
            Weekday::Sun => String::from("Sunday"),
        }
    }

    pub fn read(email: String, conn: &MysqlConnection) -> Result<Vec<TodayTreatment>, Error> {
        let mut today: Vec<TodayTreatment> = Vec::new();
        let treatments = TreatmentForm::read(email.clone(), conn)?;
        let account_id = UserAccount::read_by_email(email, conn)?.account_id.unwrap();
        let user_id = UserInfo::read_by_account_id(account_id, conn)?.user_id.unwrap();

        for treatment in treatments {
            for time in treatment.times {
                // if day is not today, skip
                let weekday: String = TodayTreatment::weekday_to_string(Utc::now().naive_utc().weekday());
                if !time.day.to_uppercase().eq("EVERYDAY") && !time.day.eq(&weekday) {
                    continue;
                }
                // get take log taken status
                let mut taken = false;
                let treatment_id = Treatment::read_by_user_id_and_treatment_name(
                    user_id,
                    treatment.name.clone(),
                    conn
                )?.treatment_id.unwrap();
                let treatment_log = TakenTreatmentLog::read_by_user_id_and_treatment_id_and_taken_time(
                    user_id,
                    treatment_id,
                    time.time,
                    conn
                )?;
                for l in treatment_log {
                    if l.timestamp.date() == Utc::now().naive_utc().date() {
                        if l.taken.to_uppercase().eq("YES") {
                            taken = true;
                        }
                        break;
                    }
                }

                let today_treatment = TodayTreatment {
                    name: treatment.name.clone(),
                    unit: treatment.unit.clone(),
                    dosage: treatment.dosage.clone(),
                    concentration: treatment.concentration.clone(),
                    time: time.time,
                    preference: time.preference,
                    color: treatment.color.clone(),
                    taken: taken,
                };
                today.push(today_treatment);
            }
        }
        Ok(today)
    }
}

impl TodayTakeForm {
    pub fn create (email: String, form: TodayTakeForm, conn: &MysqlConnection) -> Result<(), Error> {
        let account_id = UserAccount::read_by_email(email, conn)?.account_id.unwrap();
        let user_id = UserInfo::read_by_account_id(account_id, conn)?.user_id.unwrap();
        let treatment_id = Treatment::read_by_user_id_and_treatment_name(
            user_id,
            form.treatment_name,
            conn
        )?.treatment_id.unwrap();
        let taken = match form.taken {
            true => String::from("YES"),
            false => String::from("NO"),
        };

        // if log exists, delete and insert new one
        let mut log_id = -1;
        let treatment_log = TakenTreatmentLog::read_by_user_id_and_treatment_id_and_taken_time(
            user_id,
            treatment_id,
            form.time,
            conn
        )?;
        for l in treatment_log {
            if l.timestamp.date() == Utc::now().naive_utc().date() {
                log_id = l.taken_log_id.unwrap();
                break;
            }
        }
        TakenTreatmentLog::delete(log_id, conn);

        let take_log = TakenTreatmentLog {
            taken_log_id: None,
            user_id,
            treatment_id,
            timestamp: Utc::now().naive_utc(),
            taken_time: form.time,
            taken,
        };

        match TakenTreatmentLog::create(take_log, conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
        // unimplemented!()
    }
}