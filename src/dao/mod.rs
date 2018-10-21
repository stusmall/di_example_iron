pub mod model;
mod schema;

use dao::model::Widget;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use failure::Error;

pub trait Dao: 'static + Send + Sync {
    fn get_widget(&self, _: i32) -> Result<Option<Widget>, Error>;
}

pub struct DaoImpl {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DaoImpl {
    pub fn new() -> Result<Self, Error> {
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = Pool::builder().build(manager)?;
        Ok(DaoImpl { pool })
    }
}

impl Dao for DaoImpl {
    fn get_widget(&self, incoming_id: i32) -> Result<Option<Widget>, Error> {
        let connection = self.pool.get()?;
        use self::schema::widget::dsl::*;
        let mut x = widget
            .filter(id.eq(incoming_id))
            .limit(1)
            .load::<Widget>(&connection)?;
        Ok(x.pop())
    }
}
