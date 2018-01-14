use api::UserApi;
use db::{PostgresSource, PostgresHelperImpl, ConnectionSource};
use iron::request::Request;
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


fn get_api_obj<'de, T: Deserialize<'de>>(request: &Request) -> Result<T, ()> 
    where for<'a> T: Deserialize<'a> {
    request.get::<bodyparser::Struct<T>>().map_err(|x| ())
}
