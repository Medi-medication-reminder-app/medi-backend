use chrono::NaiveTime;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::take_times;


#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "take_times"]
pub struct TakeTime {
    pub take_time_id: Option<i32>,
    pub treatment_id: i32,
    pub time: NaiveTime,
    pub day: String,
    pub preference_id: Option<i32>,
}

impl TakeTime {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<TakeTime>, Error> {
        take_times::table.load::<TakeTime>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<TakeTime, Error> {
        take_times::table.find(id).first(conn)
    }

    pub fn read_by_treatment_id(id: i32, conn: &MysqlConnection) -> Result<Vec<TakeTime>, Error> {
        take_times::table.filter(take_times::treatment_id.eq(id)).load::<TakeTime>(conn)
    }

    pub fn create(time: TakeTime, conn: &MysqlConnection) -> Result<TakeTime, Error> {
        let new_time = TakeTime {
            ..time
        };

        let ops = diesel::insert_into(take_times::table)
            .values(&new_time)
            .execute(conn);

        match ops {
            Ok(_) => take_times::table.order(take_times::take_time_id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn update(id: i32, user: TakeTime, conn: &MysqlConnection) -> Result<TakeTime, Error> {
        let new_user = TakeTime {
            ..user
        };

        let ops = diesel::update(take_times::table.find(id))
            .set(&new_user)
            .execute(conn);

        match ops {
            Ok(_) => take_times::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(take_times::table.find(id)).execute(conn).is_ok()
    }
}