#![feature(custom_attribute)]
#![feature(try_from)]
#![feature(extern_prelude)]

extern crate bcrypt;
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
extern crate toml;

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
extern crate serde_urlencoded;
extern crate threadpool;
extern crate url;
extern crate web3;
extern crate lettre;

mod api;
mod cors_middleware;
mod db;
mod domain;
mod jobs;
mod payment;
mod repository;
mod services;
mod tests;
mod config;
mod email;

use api::ApiError;
use bcrypt::{hash, verify, DEFAULT_COST};
use cors_middleware::CorsMiddleware;
use db::{PostgresHelper, PostgresHelperImpl};
use domain::{Order, Session, User};
use iron::headers::AccessControlAllowOrigin;
use iron::prelude::*;
use iron::status;
use iron::{AfterMiddleware, Iron, IronResult, Request, Response};
use jobs::JobLoop;
use persistent::Read;
use postgres::{Connection, TlsMode};
use std::collections::HashSet;
use std::error::Error;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use time::PreciseTime;
use config::*;

const CONFIG_PATH: &'static str = "./config.secret.toml";

fn main() {
    env_logger::init().expect("Could not start logger");
    let config = config::ServerConfig::from_file(CONFIG_PATH)
        .expect("could not open server config file"); 
    let tx = start_job_loop(&config);
    let chain = build_chain(&config, tx);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}

fn start_job_loop(config: &ServerConfig) -> Sender<jobs::JobRequest> {
    let (tx, rx) = channel();
    let mut job_loop = JobLoop::new(&config.get_connection_string(), &config.get_web3_address(), rx);
    thread::spawn(move || {
        job_loop.run();
    });
    tx
}


fn build_chain(config: &config::ServerConfig, sender: Sender<jobs::JobRequest>) -> iron::Chain {
    let (eloop, transport) = web3::transports::ipc::Ipc::new(config.get_web3_address()).unwrap();
    let web3 = web3::Web3::new(transport);
    let api_handler = api::ApiHandler::new(config, web3, sender);
    let mut chain = iron::Chain::new(api_handler);
    let middleware = CorsMiddleware {};
    chain.link_around(middleware);
    chain
}
