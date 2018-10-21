#[macro_use]
extern crate lazy_static;
extern crate di_example_iron;
use di_example_iron::dao::DaoImpl;
use di_example_iron::Server;

lazy_static! {
    static ref DAO: DaoImpl = DaoImpl::new().unwrap();
}

fn main() {
    let server = Server::<DaoImpl>::new(&DAO);
    server.run();
}
