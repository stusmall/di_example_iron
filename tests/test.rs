extern crate di_example_iron;
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate iron;
extern crate iron_test;
extern crate serde_json;

use di_example_iron::dao::model::Widget;
use di_example_iron::dao::Dao;
use failure::err_msg;
use failure::Error;
use iron::status::Status;
use iron::Headers;
use iron_test::request;

use di_example_iron::handler::WidgetHandler;
use iron_test::response::extract_body_to_string;

#[test]
fn test_get_widget() {
    struct BadDBDao {};
    impl Dao for BadDBDao {
        fn get_widget(&self, _: i32) -> Result<Option<Widget>, Error> {
            Ok(Some(Widget::default()))
        }
    }

    lazy_static! {
        static ref DAO: BadDBDao = BadDBDao {};
    }

    let handler = WidgetHandler::<BadDBDao>::new(&DAO);

    let response =
        request::get("http://localhost:3000/widget/1", Headers::new(), &handler).unwrap();
    assert_eq!(response.status, Some(Status::Ok));
}

#[test]
fn test_bad_db() {
    struct BadDBDao {};
    impl Dao for BadDBDao {
        fn get_widget(&self, _: i32) -> Result<Option<Widget>, Error> {
            Err(err_msg("db is down!"))
        }
    }

    lazy_static! {
        static ref DAO: BadDBDao = BadDBDao {};
    }

    let handler = WidgetHandler::<BadDBDao>::new(&DAO);

    let response =
        request::get("http://localhost:3000/widget/1", Headers::new(), &handler).unwrap();
    let body = extract_body_to_string(response);
    assert_eq!(body, "Database Failure");
}
