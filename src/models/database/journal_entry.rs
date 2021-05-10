use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
use chrono::NaiveDateTime;

use crate::schema::journal_entries;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "journal_entries"]
pub struct JournalEntry {
    pub entry_id: Option<i32>,
    pub user_id: i32,
    pub timestamp: NaiveDateTime,
    pub feeling_id: i32,
    pub details: String,
}

impl JournalEntry {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<JournalEntry>, Error> {
        journal_entries::table.load::<JournalEntry>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<JournalEntry, Error> {
        journal_entries::table.find(id).first(conn)
    }

    pub fn read_by_user_id(id: i32, conn: &MysqlConnection) -> Result<Vec<JournalEntry>, Error> {
        journal_entries::table.filter(journal_entries::user_id.eq(id)).load::<JournalEntry>(conn)
    }

    pub fn create(t: JournalEntry, conn: &MysqlConnection) -> Result<JournalEntry, Error> {
        let new_treatment = JournalEntry {
            ..t
        };

        let ops = diesel::insert_into(journal_entries::table)
            .values(&new_treatment)
            .execute(conn);

        match ops {
            Ok(_) => journal_entries::table.order(journal_entries::entry_id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    // pub fn update(id: i32, t: JournalEntry, conn: &MysqlConnection) -> Result<JournalEntry, Error> {
    //     let new_treatment = JournalEntry {
    //         ..t
    //     };

    //     let ops = diesel::update(journal_entries::table.find(id))
    //         .set(&new_treatment)
    //         .execute(conn);

    //     match ops {
    //         Ok(_) => journal_entries::table.find(id).first(conn),
    //         Err(e) => Err(e),
    //     }
    // }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(journal_entries::table.find(id)).execute(conn).is_ok()
    }
}