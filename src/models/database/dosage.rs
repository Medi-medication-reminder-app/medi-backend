use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::dosages;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "dosages"]
pub struct Dosage {
    pub dosage_id: i32,
    pub dosage_type: String,
}

impl Dosage {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<Dosage>, Error> {
        dosages::table.load::<Dosage>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<Dosage, Error> {
        dosages::table.find(id).first(conn)
    }

    pub fn read_by_value(value: String, conn: &MysqlConnection) -> Result<Dosage, Error> {
        dosages::table.filter(dosages::dosage_type.eq(value)).first(conn)
    }
}