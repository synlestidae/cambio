#![feature(custom_attribute)]

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
extern crate try_from_row; 

#[macro_use]
extern crate serde_derive;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate persistent;
extern crate iron_cors;

extern crate openssl;
extern crate crypto;
extern crate rand;
extern crate base64;
extern crate hex;
extern crate web3;
extern crate rlp;
extern crate secp256k1;
extern crate byteorder;

mod db;
mod domain;
mod tests;
mod api;
mod services;
mod repository;
mod repositories;

use iron::prelude::*;
use iron::{Iron, Request, Response, IronResult, AfterMiddleware};
use iron::status;
use iron::headers::AccessControlAllowOrigin;
use persistent::Read;
use router::Router;
use bcrypt::{DEFAULT_COST, hash, verify};
use postgres::{Connection, TlsMode};
use domain::{User, Order, Session};
use api::ApiError;
use db::{PostgresHelperImpl, PostgresHelper};
use std::error::Error;
use time::PreciseTime;
use iron_cors::CorsMiddleware;

fn main() {
    use web3::futures::Future;
    /*env_logger::init().expect("Could not start logger");
    let middleware = CorsMiddleware::with_allow_any();
    let helper =
        PostgresHelperImpl::new_from_conn_str("postgres://mate@localhost:5432/cambio_test");
    let mut router = Router::new();
    let mut api_init = api::TotalApiInit::new(helper);
    api_init.init_api(&mut router);
    let mut chain = iron::Chain::new(router);
    chain.link_around(middleware);
    debug!("Booting up HTTP server");
    Iron::new(chain).http("localhost:3000").unwrap();*/

    let (_eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(http);
    let mut address_bytes: [u8; 20] = [0; 20];
    let hex_addr = "A990F82d33Fd19C3872dc12c588A66224b9330A6";
    for (i, b) in hex::decode(hex_addr).unwrap().into_iter().enumerate() {
        address_bytes[i] = b;
    }
    let balance = web3.eth().balance(web3::types::H160(address_bytes), None).wait().unwrap();
}
