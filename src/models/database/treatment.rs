use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::treatments;


#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "treatments"]
pub struct Treatment {
    pub treatment_id: Option<i32>,
    pub user_id: i32,
    pub name: String,
    pub unit_id: i32,
    pub dosage_id: i32,
    pub concentration_id: i32,
    pub frequency: i32,
    pub color: String,
}

impl Treatment {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<Treatment>, Error> {
        treatments::table.load::<Treatment>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<Treatment, Error> {
        treatments::table.find(id).first(conn)
    }

    pub fn read_by_user_id(id: i32, conn: &MysqlConnection) -> Result<Vec<Treatment>, Error> {
        treatments::table.filter(treatments::user_id.eq(id)).load::<Treatment>(conn)
    }

    pub fn read_by_user_id_and_treatment_id(
        user_id: i32, treatment_id: i32,
        conn: &MysqlConnection
    ) -> Result<Treatment, Error> {
        treatments::table
        .filter(treatments::user_id.eq(user_id))
        .filter(treatments::treatment_id.eq(treatment_id))
        .first(conn)
    }

    pub fn read_by_user_id_and_treatment_name(
        id: i32, name: String,
        conn: &MysqlConnection
    ) -> Result<Treatment, Error> {
        treatments::table
        .filter(treatments::user_id.eq(id))
        .filter(treatments::name.eq(name))
        .first(conn)
    }

    pub fn create(t: Treatment, conn: &MysqlConnection) -> Result<Treatment, Error> {
        let new_treatment = Treatment {
            ..t
        };

        let ops = diesel::insert_into(treatments::table)
            .values(&new_treatment)
            .execute(conn);

        match ops {
            Ok(_) => treatments::table.order(treatments::treatment_id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn update(id: i32, t: Treatment, conn: &MysqlConnection) -> Result<Treatment, Error> {
        let new_treatment = Treatment {
            ..t
        };

        let ops = diesel::update(treatments::table.find(id))
            .set(&new_treatment)
            .execute(conn);

        match ops {
            Ok(_) => treatments::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(treatments::table.find(id)).execute(conn).is_ok()
    }
}