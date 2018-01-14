use api::UserApi;
use db::{PostgresSource, PostgresHelperImpl, ConnectionSource};
use iron::request::Request;
use std::clone::Clone;
use serde::Deserialize;
use router::Router;
use bodyparser;
use iron::prelude::*;

struct IronApi {
    user_api: UserApi<PostgresHelperImpl>
}

impl IronApi {
    pub fn new(conn_source: PostgresSource) -> Self {
        let helper = PostgresHelperImpl::new(conn_source); 
        IronApi {
            user_api: UserApi::new(helper)
        }
    }

    fn init_user_api(router: &mut Router) {
        unimplemented!()
    }
}


fn get_api_obj<T: Clone + 'static>(request: &mut Request) -> Result<T, ()> 
where for<'a> T: Deserialize<'a> {
        
    let result: Result<T, ()> = match request.get::<bodyparser::Struct<T>>() {
        Ok(Some(body_obj)) => Ok(body_obj.clone()),
        _ => Err(())
    };

    result
}
