extern crate bcrypt;
extern crate iron;
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
mod init_api;

use iron::prelude::*;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use persistent::Read;
use router::Router;
use bcrypt::{DEFAULT_COST, hash, verify};
use postgres::{Connection, TlsMode};
use domain::{User, Order, Session};
use api::ApiError;
use db::{PostgresHelperImpl, PostgresHelper, UserRepository};
use std::error::Error;
use time::PreciseTime;

fn main() {
    const MAX_BODY_LENGTH: usize = 1024 * 512;
    let router = Router::new();
    Iron::new(router).http("localhost:3000").unwrap();
    //let mut chain = Chain::new(log_body);
    //chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    //Iron::new(chain).http("localhost:3000").unwrap();
    unimplemented!()
}
