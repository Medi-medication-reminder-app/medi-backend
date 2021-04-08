use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::time_preferences;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "time_preferences"]
pub struct TimePreference {
    pub preference_id: i32,
    pub preference_type: String,
}

impl TimePreference {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<TimePreference>, Error> {
        time_preferences::table.load::<TimePreference>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<TimePreference, Error> {
        time_preferences::table.find(id).first(conn)
    }
}