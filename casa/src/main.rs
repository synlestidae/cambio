#![feature(custom_attribute)]
#![feature(try_from)]

extern crate bcrypt;
extern crate bodyparser;
extern crate checkmail;
extern crate chrono;
extern crate env_logger;
extern crate hyper;
extern crate iron;
extern crate iron_test;
#[macro_use]
extern crate log;
extern crate params;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;

#[macro_use]
extern crate try_from_row;
#[macro_use]
extern crate sql_id;
#[macro_use]
extern crate serde_derive;

extern crate iron_cors;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate base64;
extern crate byteorder;
extern crate crypto;
extern crate hex;
extern crate openssl;
extern crate rand;
extern crate rlp;
extern crate secp256k1;
extern crate web3;
extern crate threadpool;
extern crate serde_xml_rs;

mod api;
mod cors_middleware;
mod db;
mod domain;
mod jobs;
mod query;
mod repositories;
mod repository;
mod services;
mod tests;
mod payment;

use api::ApiError;
use bcrypt::{hash, verify, DEFAULT_COST};
use cors_middleware::CorsMiddleware;
use db::{PostgresHelper, PostgresHelperImpl};
use domain::{Order, Session, User};
use iron::headers::AccessControlAllowOrigin;
use iron::prelude::*;
use iron::status;
use iron::{AfterMiddleware, Iron, IronResult, Request, Response};
use persistent::Read;
use postgres::{Connection, TlsMode};
use std::error::Error;
use time::PreciseTime;
use std::sync::mpsc::channel;
use std::collections::HashSet;
use jobs::JobLoop;
use std::thread;

fn main() {
    const WEB3_ADDRESS: &'static str = "../eth_test/data/geth.ipc";
    let conn_str = "postgres://mate@localhost:5432/cambio_test";
    env_logger::init().expect("Could not start logger");
    let middleware = CorsMiddleware {};
    let (tx, rx) = channel();
    let mut job_loop = JobLoop::new(conn_str, WEB3_ADDRESS, rx);
    thread::spawn(move || {
        job_loop.run();
    });
    let api_handler = api::ApiHandler::new(conn_str, WEB3_ADDRESS, tx);
    let mut chain = iron::Chain::new(api_handler);
    chain.link_around(middleware);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}

