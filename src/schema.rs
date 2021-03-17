table! {
    caretakers (caretaker_id) {
        phone_number -> Integer,
        user_id -> Integer,
        caretaker_id -> Integer,
        name -> Varchar,
    }
}

table! {
    concentrations (concentration_id) {
        concentration_id -> Tinyint,
        concentration_amount -> Varchar,
    }
}

table! {
    conditions (condition_id) {
        condition_id -> Integer,
        user_id -> Integer,
        condition_name -> Varchar,
        condition_details -> Varchar,
    }
}

table! {
    dosages (dosage_id) {
        dosage_id -> Integer,
        dosage_type -> Varchar,
    }
}

table! {
    feelings (feeling_id) {
        feeling_id -> Integer,
        feeling_name -> Varchar,
    }
}

table! {
    journal_entries (entry_id) {
        entry_id -> Integer,
        user_id -> Integer,
        timestamp -> Timestamp,
        feeling_id -> Integer,
        details -> Varchar,
    }
}

table! {
    take_times (take_time_id) {
        take_time_id -> Integer,
        treatment_id -> Integer,
        time -> Time,
        frequency -> Integer,
        day -> Varchar,
        preference_id -> Integer,
    }
}

table! {
    taken_treatment_log (taken_log_id) {
        taken_log_id -> Integer,
        user_id -> Integer,
        treatment_id -> Integer,
        timestamp -> Timestamp,
        taken_time -> Time,
        taken -> Varchar,
    }
}

table! {
    time_preferences (preference_id) {
        preference_id -> Integer,
        preference_type -> Varchar,
    }
}

table! {
    treatments (treatment_id) {
        treatment_id -> Integer,
        user_id -> Integer,
        name -> Varchar,
        unit_id -> Integer,
        dosage_id -> Integer,
        concentration_id -> Tinyint,
        color -> Varchar,
    }
}

table! {
    units (unit_id) {
        unit_id -> Integer,
        unit_name -> Varchar,
    }
}

table! {
    user_accounts (account_id) {
        account_id -> Integer,
        email -> Varchar,
        password -> Varchar,
        create_date -> Date,
        last_login -> Timestamp,
    }
}

table! {
    user_info (user_id) {
        user_id -> Integer,
        account_id -> Integer,
        name -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
        birthday -> Nullable<Date>,
    }
}

joinable!(caretakers -> user_info (user_id));
joinable!(conditions -> user_info (user_id));
joinable!(journal_entries -> feelings (feeling_id));
joinable!(journal_entries -> user_info (user_id));
joinable!(take_times -> time_preferences (preference_id));
joinable!(take_times -> treatments (treatment_id));
joinable!(taken_treatment_log -> treatments (treatment_id));
joinable!(taken_treatment_log -> user_info (user_id));
joinable!(treatments -> concentrations (concentration_id));
joinable!(treatments -> dosages (dosage_id));
joinable!(treatments -> units (unit_id));
joinable!(treatments -> user_info (user_id));
joinable!(user_info -> user_accounts (account_id));

allow_tables_to_appear_in_same_query!(
    caretakers,
    concentrations,
    conditions,
    dosages,
    feelings,
    journal_entries,
    take_times,
    taken_treatment_log,
    time_preferences,
    treatments,
    units,
    user_accounts,
    user_info,
);
