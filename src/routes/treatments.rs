use rocket_contrib::json::{Json, JsonError};

use crate::utils::response::*;
use crate::DbConn;
use crate::utils::jwt::{apikey::ApiKey, claim::get_claim};
use crate::models::api::treatment::*;
use crate::models::database::concentration::Concentration;
use crate::models::database::dosage::Dosage;
use crate::models::database::unit::Unit;

#[get("/")]
fn read(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = TreatmentForm::read(claim.usr, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[post("/single", data = "<data>")]
fn read_by_name(data: Result<Json<TreatmentNameForm>, JsonError>, conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let new = TreatmentNameForm {
                ..d.into_inner()
            };
            let result = TreatmentNameForm::read_by_name(
                claim.usr,
                new,
                &conn
            );
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[post("/", data = "<data>")]
fn create(
    data: Result<Json<TreatmentForm>, JsonError>,
    conn: DbConn,
    key: ApiKey
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let new = TreatmentForm {
                ..d.into_inner()
            };
            let result = TreatmentForm::create(
                claim.usr,
                new,
                &conn
            );
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[put("/", data = "<data>")]
fn update(
    data: Result<Json<TreatmentForm>, JsonError>,
    conn: DbConn,
    key: ApiKey
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let new = TreatmentForm {
                ..d.into_inner()
            };
            let result = TreatmentForm::update(
                claim.usr,
                new,
                &conn
            );
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[delete("/", data = "<data>")]
fn delete(
    data: Result<Json<TreatmentNameForm>, JsonError>,
    conn: DbConn,
    key: ApiKey
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let new = TreatmentNameForm {
                ..d.into_inner()
            };
            let result = TreatmentNameForm::delete(
                claim.usr,
                new,
                &conn
            );
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[get("/concentration")]
fn read_concentrations(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let _claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = Concentration::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r.into_iter().map(|c| c.concentration_amount).collect::<Vec<String>>()))),
        Err(e) => Err(db_error(e)),
    }
}

#[get("/dosage")]
fn read_dosages(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let _claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = Dosage::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r.into_iter().map(|d| d.dosage_type).collect::<Vec<String>>()))),
        Err(e) => Err(db_error(e)),
    }
}

#[get("/unit")]
fn read_units(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let _claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = Unit::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r.into_iter().map(|u| u.unit_name).collect::<Vec<String>>()))),
        Err(e) => Err(db_error(e)),
    }
}

#[get("/info")]
fn read_info(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let _claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = ConcentrationDosageUnit::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        read,
        read_by_name,
        create,
        update,
        delete,
        read_concentrations,
        read_dosages,
        read_units,
        read_info
        ]
}