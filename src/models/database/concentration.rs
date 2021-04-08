
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::concentrations;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "concentrations"]
pub struct Concentration {
    pub concentration_id: i32,
    pub concentration_amount: String,
}

impl Concentration {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<Concentration>, Error> {
        concentrations::table.load::<Concentration>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<Concentration, Error> {
        concentrations::table.find(id).first(conn)
    }
}