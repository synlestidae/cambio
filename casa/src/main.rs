extern crate bcrypt;
extern crate iron;
extern crate hyper;
#[macro_use]
extern crate postgres;
extern crate bodyparser;
extern crate serde;
extern crate serde_json;
extern crate router;
extern crate chrono;
extern crate time;
extern crate checkmail;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate postgres_derive;

#[macro_use]
extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate persistent;

mod db;
mod domain;
mod tests;
mod api;

use iron::prelude::*;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use persistent::Read;
use router::Router;
use api::ApiInit;
use bcrypt::{DEFAULT_COST, hash, verify};
use postgres::{Connection, TlsMode};
use domain::{User, Order, Session};
use api::ApiError;
use db::{PostgresHelperImpl, PostgresHelper, UserRepository};
use std::error::Error;
use time::PreciseTime;

fn main() {
    env_logger::init();
    debug!("Starting up");
    const MAX_BODY_LENGTH: usize = 1024 * 512;
    let mut helper =
        PostgresHelperImpl::new_from_conn_str("postgres://mate@localhost:5432/cambio_test");
    let mut router = Router::new();
    let mut api_init = api::TotalApiInit::new(helper);
    api_init.init_api(&mut router);
    debug!("Booting up HTTP server");
    Iron::new(router).http("localhost:3000").unwrap();
}
