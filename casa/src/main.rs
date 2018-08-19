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
extern crate toml;
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
extern crate lettre;
extern crate lettre_email;
extern crate openssl;
extern crate rand;
extern crate rlp;
extern crate secp256k1;
extern crate serde_urlencoded;
extern crate threadpool;
extern crate url;
extern crate web3;
extern crate pub_sub;

mod api;
mod event;
mod config;
mod cors_middleware;
mod db;
mod domain;
mod email;
mod payment;
mod repository;
mod services;
mod tests;
mod colectivo;
mod clerks;

use api::ApiError;
use bcrypt::{hash, verify, DEFAULT_COST};
use config::*;
use cors_middleware::CorsMiddleware;
use db::{PostgresHelper, PostgresHelperImpl};
use domain::{Order, Session, User};
use iron::headers::AccessControlAllowOrigin;
use iron::prelude::*;
use iron::status;
use iron::{AfterMiddleware, Iron, IronResult, Request, Response};
use persistent::Read;
use postgres::{Connection, TlsMode};
use std::collections::HashSet;
use std::error::Error;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use time::PreciseTime;

const CONFIG_PATH: &'static str = "./config.secret.toml";

fn main() {
    env_logger::init().expect("Could not start logger");
    let config =
        config::ServerConfig::from_file(CONFIG_PATH).expect("could not open server config file");
    let mut colectivo = colectivo::Colectivo::new();
    build_clerks(&mut colectivo, &config);
    let chain = build_chain(&config, colectivo);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}


fn build_chain(config: &config::ServerConfig, colectivo: colectivo::Colectivo) -> iron::Chain {
    debug!("Building chain");
    let api_handler = api::ApiHandler::new(config, colectivo);
    let mut chain = iron::Chain::new(api_handler);
    let middleware = CorsMiddleware {};
    chain.link_around(middleware);
    chain
}

fn build_clerks(colectivo: &mut colectivo::Colectivo, config: &ServerConfig) {
    use event::EventHandler;
    let mut email_clerk = clerks::EmailClerk::new(config.get_email_noreply_config());
    let email_bus = event::Bus::from_colectivo("registration", colectivo); 
    thread::spawn(move || {
        loop {
            let (e, t) = email_bus.recv().unwrap(); 
            info!("Received email message");
            email_clerk.handle(e, t);
        }
    });
}
