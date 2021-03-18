#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;

mod utils;
mod models;
mod routes;
mod schema;
mod cors;

#[database("mediusers")]
pub struct DbConn(diesel::MysqlConnection);

// this is for utils/ "research"
// https://github.com/kayrules/rust-rocket-mysql-starter/blob/master/src/controllers/user.rs

// this if for everything
// https://cprimozic.net/blog/rust-rocket-cloud-run/

fn main() {
    // println!("Hello, world!");
    rocket::ignite()
        .mount("/users", routes::users::routes())
        .register(utils::catcher::catchers())
        .attach(DbConn::fairing())
        .attach(cors::CorsFairing)
        .launch();
}
