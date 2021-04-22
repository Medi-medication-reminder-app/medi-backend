use chrono::{NaiveTime, NaiveDateTime};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::taken_treatment_log;


#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "taken_treatment_log"]
pub struct TakenTreatmentLog {
    pub taken_log_id: Option<i32>,
    pub user_id: i32,
    pub treatment_id: i32,
    pub timestamp: NaiveDateTime,
    pub taken_time: NaiveTime,
    pub taken: String,
}

impl TakenTreatmentLog {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<TakenTreatmentLog>, Error> {
        taken_treatment_log::table.load::<TakenTreatmentLog>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<TakenTreatmentLog, Error> {
        taken_treatment_log::table.find(id).first(conn)
    }

    pub fn read_by_user_id(id: i32, conn: &MysqlConnection) -> Result<Vec<TakenTreatmentLog>, Error> {
        taken_treatment_log::table.filter(taken_treatment_log::user_id.eq(id)).load::<TakenTreatmentLog>(conn)
    }

    pub fn read_by_user_id_and_treatment_id(
        id: i32, t_id: i32,
        conn: &MysqlConnection
    ) -> Result<Vec<TakenTreatmentLog>, Error> {
        taken_treatment_log::table
        .filter(taken_treatment_log::user_id.eq(id))
        .filter(taken_treatment_log::treatment_id.eq(t_id))
        .load::<TakenTreatmentLog>(conn)
    }

    pub fn read_by_user_id_and_treatment_id_and_taken_time(
        id: i32, t_id: i32, time: NaiveTime,
        conn: &MysqlConnection
    ) -> Result<Vec<TakenTreatmentLog>, Error> {
        taken_treatment_log::table
        .filter(taken_treatment_log::user_id.eq(id))
        .filter(taken_treatment_log::treatment_id.eq(t_id))
        .filter(taken_treatment_log::taken_time.eq(time))
        .load::<TakenTreatmentLog>(conn)
    }

    pub fn create(t: TakenTreatmentLog, conn: &MysqlConnection) -> Result<TakenTreatmentLog, Error> {
        let new_treatment = TakenTreatmentLog {
            ..t
        };

        let ops = diesel::insert_into(taken_treatment_log::table)
            .values(&new_treatment)
            .execute(conn);

        match ops {
            Ok(_) => taken_treatment_log::table.order(taken_treatment_log::treatment_id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn update(id: i32, t: TakenTreatmentLog, conn: &MysqlConnection) -> Result<TakenTreatmentLog, Error> {
        let new_treatment = TakenTreatmentLog {
            ..t
        };

        let ops = diesel::update(taken_treatment_log::table.find(id))
            .set(&new_treatment)
            .execute(conn);

        match ops {
            Ok(_) => taken_treatment_log::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(taken_treatment_log::table.find(id)).execute(conn).is_ok()
    }
}