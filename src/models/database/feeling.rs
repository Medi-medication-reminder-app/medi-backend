use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::feelings;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "feelings"]
pub struct Feeling {
    pub feeling_id: i32,
    pub feeling_name: String,
}

impl Feeling {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<Feeling>, Error> {
        feelings::table.load::<Feeling>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<Feeling, Error> {
        feelings::table.find(id).first(conn)
    }

    pub fn read_by_value(value: String, conn: &MysqlConnection) -> Result<Feeling, Error> {
        feelings::table.filter(feelings::feeling_name.eq(value)).first(conn)
    }
}