use chrono::{NaiveTime, Utc, Weekday, Datelike, NaiveDate, Duration};
use diesel::mysql::MysqlConnection;
use diesel::result::Error;

use std::collections::HashMap;
use std::mem;

use crate::models::database::user_account::UserAccount;
use crate::models::database::user_info::UserInfo;
use crate::models::api::treatment::TreatmentForm;
use crate::models::database::take_log::TakenTreatmentLog;
use crate::models::database::treatment::Treatment;

#[derive(Serialize, Deserialize, Clone)]
pub struct CalendarTreatment {
    pub name: String,
    pub unit: String,
    pub dosage: String,
    pub concentration: String,
    pub time: NaiveTime,
    pub preference: Option<String>,
    pub color: String,
    pub taken: bool,
    pub weekday: String,
    pub date: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct TodayTakeForm {
    pub treatment_name: String,
    pub time: NaiveTime,
    pub taken: bool,
}

struct DateRange(NaiveDate, NaiveDate);

impl Iterator for DateRange {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DateRangeForm {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl CalendarTreatment {
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

    pub fn read_by_day(day: NaiveDate, email: String, conn: &MysqlConnection) -> Result<Vec<CalendarTreatment>, Error> {
        let mut today: Vec<CalendarTreatment> = Vec::new();
        let treatments = TreatmentForm::read(email.clone(), conn)?;
        let account_id = UserAccount::read_by_email(email, conn)?.account_id.unwrap();
        let user_id = UserInfo::read_by_account_id(account_id, conn)?.user_id.unwrap();

        for treatment in treatments {
            for time in treatment.times {
                // if day is not paramenter `day`, skip
                let weekday: String = CalendarTreatment::weekday_to_string(day.weekday());
                if !time.day.to_uppercase().eq("EVERYDAY") && !time.day.eq(&weekday) {
                    continue;
                }
                // get take log taken status
                let mut taken = false;
                let treatment_log = TakenTreatmentLog::read_by_user_id_and_treatment_id_and_taken_time(
                    user_id,
                    treatment.id.unwrap(),
                    time.time,
                    conn
                )?;
                for l in treatment_log {
                    if l.timestamp.date() == day {
                        if l.taken.to_uppercase().eq("YES") {
                            taken = true;
                        }
                        break;
                    }
                }

                let today_treatment = CalendarTreatment {
                    name: treatment.name.clone(),
                    unit: treatment.unit.clone(),
                    dosage: treatment.dosage.clone(),
                    concentration: treatment.concentration.clone(),
                    time: time.time,
                    preference: time.preference,
                    color: treatment.color.clone(),
                    taken: taken,
                    weekday: weekday,
                    date: day,
                };
                today.push(today_treatment);
            }
        }
        Ok(today)
    }

    pub fn read_between(
        start: NaiveDate,
        end: NaiveDate,
        email: String,
        conn: &MysqlConnection
    ) -> Result<Vec<Vec<CalendarTreatment>>, Error> {
        let mut calendar: Vec<Vec<CalendarTreatment>> = Vec::new();
        for date in DateRange(start, end) {
            calendar.push(CalendarTreatment::read_by_day(date, email.clone(), conn)?);
        }

        Ok(calendar)
    }

    pub fn read_thisweek(
        email: String,
        conn: &MysqlConnection
    ) -> Result<HashMap<String, Vec<CalendarTreatment>>, Error> {
        let today = Utc::now().naive_utc().date();
        // w:                           Mon	Tue	Wed	Thu	Fri	Sat	Sun
        // w.num_days_from_monday():	0	1	2	3	4	5	6
        let offset_from_monday= today.weekday().num_days_from_monday();
        let offset_to_sunday= 6 - offset_from_monday;
        let start = today - Duration::days(offset_from_monday as i64);
        let end = today + Duration::days(offset_to_sunday as i64);

        let mut week_calendar = HashMap::new();

        let week_treatments = CalendarTreatment::read_between(start, end, email, conn)?;
        let weekdays = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
        for idx in 0..7 {
            week_calendar.insert(String::from(weekdays[idx]), week_treatments[idx].clone());
        }

        Ok(week_calendar)
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
    }
}