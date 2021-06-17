use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use chrono::{Utc, NaiveDateTime};

use crate::models::database::{
    user_account::UserAccount,
    user_info::UserInfo,
    feeling::Feeling,
    journal_entry::JournalEntry,
};

#[derive(Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct JournalForm {
    pub feeling: String,
    pub details: String,
    pub timestamp: Option<NaiveDateTime>,
}

impl JournalForm {
    pub fn read(email: String, conn: &MysqlConnection) -> Result<Vec<JournalForm>, Error> {
        let account_id = UserAccount::read_by_email(email, conn)?.account_id.unwrap();
        let user_id = UserInfo::read_by_account_id(account_id, conn)?.user_id.unwrap();
        let entries = JournalEntry::read_by_user_id(user_id, conn)?;

        let mut form_entries: Vec<JournalForm> = Vec::new();

        for e in entries {
            let feeling = Feeling::read_by_id(e.feeling_id, conn)?;
            let form_elem = JournalForm {
                feeling: feeling.feeling_name,
                details: e.details,
                timestamp: Some(e.timestamp),
            };
            form_entries.push(form_elem);
        }

        form_entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(form_entries)
    }

    pub fn create(email: String, form: JournalForm, conn: &MysqlConnection) -> Result<(), Error> {
        let account_id = UserAccount::read_by_email(email, conn)?.account_id.unwrap();
        let user_id = UserInfo::read_by_account_id(account_id, conn)?.user_id.unwrap();

        let db_journal_entry = JournalEntry {
            entry_id: None,
            user_id: user_id,
            timestamp: Utc::now().naive_utc(),
            feeling_id: Feeling::read_by_value(form.feeling, conn)?.feeling_id,
            details: form.details,
        };

        JournalEntry::create(db_journal_entry, conn)?;


        Ok(())
    }
}