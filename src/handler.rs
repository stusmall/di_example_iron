use dao::Dao;
use iron::prelude::*;
use iron::status;
use iron::Handler;
use serde_json;
use std::str::FromStr;
pub struct WidgetHandler<D: Dao> {
    dao: &'static D,
}

impl<D: Dao> WidgetHandler<D> {
    pub fn new(dao: &'static D) -> Self {
        WidgetHandler { dao }
    }
}

impl<D: Dao> Handler for WidgetHandler<D> {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let id_s = request.url.path().pop().unwrap_or("");
        let id: i32 = match FromStr::from_str(id_s) {
            Ok(x) => x,
            Err(_) => return Ok(Response::with((status::BadRequest, "Bad ID"))),
        };
        match self.dao.get_widget(id) {
            Ok(Some(widget)) => Ok(Response::with((
                status::Ok,
                serde_json::to_string(&widget).unwrap(),
            ))),
            Ok(None) => Ok(Response::with((
                status::BadRequest,
                "No value found with ID",
            ))),
            Err(_) => Ok(Response::with((
                status::InternalServerError,
                "Database Failure",
            ))),
        }
    }
}
