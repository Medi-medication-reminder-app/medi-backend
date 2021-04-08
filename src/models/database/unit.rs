use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::units;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "units"]
pub struct Unit {
    pub unit_id: i32,
    pub unit_name: String,
}

impl Unit {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<Unit>, Error> {
        units::table.load::<Unit>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<Unit, Error> {
        units::table.find(id).first(conn)
    }
}