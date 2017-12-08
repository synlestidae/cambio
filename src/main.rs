extern crate bcrypt;
extern crate iron;
extern crate postgres;
extern crate bodyparser;
extern crate serde;
extern crate serde_json;
extern crate router;
#[macro_use] extern crate serde_derive;

mod db;
mod domain;

use iron::prelude::*;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};
use bcrypt::{DEFAULT_COST, hash, verify};
use postgres::{Connection, TlsMode};
use domain::{User, Order};

fn make_order(req: &mut Request) -> IronResult<Response> { 
    let order = req.get::<bodyparser::Struct<Order>>().unwrap().unwrap();
    let conn = Connection::connect("postgres://mate@localhost:5432/coin_boi", TlsMode::None).unwrap();

    let buy_asset_id: u32;
    let sell_asset_id: u32;

    let asset_query = conn.execute("SELECT get_asset_id($1, $2) UNION get_asset_id($3, $4)", &[
        &order.sell_asset_type.to_string(), 
        &order.sell_asset_denom.to_string(), 
        &order.buy_asset_type.to_string(), 
        &order.buy_asset_denom.to_string()
    ]).unwrap();

    unimplemented!();
}

fn register_user(req: &mut Request) -> IronResult<Response> {  
    let struct_body = req.get::<bodyparser::Struct<User>>();
    let user_obj = match struct_body {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(Response::with((iron::status::BadRequest, "No data supplied"))),
        Err(err) => return Ok(Response::with((iron::status::BadRequest, format!("Cannot parse your request: {:?}", err))))
    };

    // TODO use the db helper

    /*let email_address = user_obj.email_address;
    if user_obj.password.is_some() {
        let password_hash = hash(&user_obj.password.unwrap(), DEFAULT_COST).unwrap();
        let conn = Connection::connect("postgres://mate@localhost:5432/coin_boi", TlsMode::None).unwrap();
        
        conn.execute("SELECT register_user($1, $2);", 
            &[&email_address, &password_hash]).unwrap();

        return Ok(Response::with(status::Ok));
    }*/

    Ok(Response::with(status::BadRequest))
}


fn log_in_user(req: &mut Request) -> IronResult<Response> {  
    let struct_body = req.get::<bodyparser::Struct<User>>();
    let user_obj = match struct_body {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(Response::with((iron::status::BadRequest, "No data supplied"))),
        Err(err) => return Ok(Response::with((iron::status::BadRequest, format!("Cannot parse your request: {:?}", err))))
    };

    let user_obj_password = match user_obj.password {
        Some(ref password) => password.clone(),
        None => return Ok(Response::with((status::Unauthorized, "Password is a required field")))
    }

    // let email_address = user_obj.email_address;
    let conn = Connection::connect("postgres://mate@localhost:5432/coin_boi", TlsMode::None).unwrap();
    let db = PostgresHelperImpl::new(conn);

    let query_result = db.query("SELECT email_address, password_hash FROM users WHERE email_address = $1;", 
        &[&email_address]);

    match query_result {
        Ok(matching_users) => {
            let user_match = matching_users.pop();
            match user_match {
                Some(user) => {
                    if (user.hash_matches_password(user_obj_password)) {
                        // Create a new session for the user
                    } else {
                        // Stop immediately and return an unauthorised response
                    }
                },
                None => unimplemented!(),
            }
        },
        _ => Ok(Response::with((status::Unauthorized, "User not found")))
    }

    //println!("Selecting {}", email_address);

    // TODO use the DB helper

    /*for row in conn.query("SELECT email_address, password_hash FROM users WHERE email_address = $1;", 
        &[&email_address]).unwrap().iter() {
        
        let db_email_address:String  = row.get(0);
        let db_password_hash:String = row.get(1);

        println!("Verifying {:?} with {}", user_obj.password, db_password_hash);

        if db_email_address == email_address && verify(&user_obj.password, &db_password_hash).is_ok() {
            for row in conn.query("SELECT activate_user_session($1);", &[&email_address]).unwrap().iter() {
                println!("Session boiz: {:?}", row);//, other);
                println!("Session toix: {:?}", row.columns());//, other);
                let session_token: String = row.get("user_login");
                return Ok(Response::with((status::Ok, 
                    format!("{{\"email_address\": \"{}\", \"session_token\": \"{}\"}}", 
                        email_address, 
                        session_token))));
            }
        }
    }*/
    Ok(Response::with((status::Unauthorized, "User not found")))
}

fn main() {
    const MAX_BODY_LENGTH:usize = 1024 * 512; //max 512 Kb
    let mut router = Router::new();
    router.post("/user/register", register_user, "register_user");
    router.post("/user/login", log_in_user, "login_user");
    Iron::new(router).http("localhost:3000").unwrap();
}
