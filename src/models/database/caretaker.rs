
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::caretakers;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "caretakers"]
pub struct Caretaker {
    pub caretaker_id: Option<i32>,
    pub user_id: Option<i32>,
    pub name: String,
    pub phone_number: String,
}

impl Caretaker {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<Caretaker>, Error> {
        caretakers::table.load::<Caretaker>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<Caretaker, Error> {
        caretakers::table.find(id).first(conn)
    }

    pub fn create(user: Caretaker, conn: &MysqlConnection) -> Result<Caretaker, Error> {
        let new_user = Caretaker {
            ..user
        };

        let ops = diesel::insert_into(caretakers::table)
            .values(&new_user)
            .execute(conn);

        match ops {
            Ok(_) => caretakers::table.order(caretakers::caretaker_id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn update(id: i32, user: Caretaker, conn: &MysqlConnection) -> Result<Caretaker, Error> {
        let new_user = Caretaker {
            ..user
        };

        let ops = diesel::update(caretakers::table.find(id))
            .set(&new_user)
            .execute(conn);

        match ops {
            Ok(_) => caretakers::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(caretakers::table.find(id)).execute(conn).is_ok()
    }
}