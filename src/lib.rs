#![allow(proc_macro_derive_resolution_fallback)]

extern crate iron;
extern crate router;
#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate serde_json;
pub mod dao;
pub mod handler;
use self::dao::Dao;
use iron::prelude::*;

use handler::WidgetHandler;

use iron::method::Get;
use router::Router;

pub struct Server<D: 'static + Dao> {
    dao: &'static D,
}

impl<D: 'static + Dao> Server<D> {
    pub fn new(dao: &'static D) -> Self {
        Server { dao }
    }

    pub fn run(&self) {
        let handler = WidgetHandler::new(self.dao);
        let mut router = Router::new();
        router.route(Get, "/widget/*", handler, "get_widget_info");
        Iron::new(router).http("localhost:8080").unwrap();
    }
}
