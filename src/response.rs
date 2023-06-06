mod error;
mod good;
mod list;

use error::Error;
use good::Good;
use list::List;

use crate::model::Todo;

pub struct Response;

// maybe it is possible to doing by a marcos
impl Response {
    pub fn bad (message: &str) -> Error {
        Error::new(message.to_string())
    }

    pub fn good (data: Todo) -> Good {
        Good::new(data)
    }

    pub fn list (data: Vec<Todo>) -> List {
        List::new(data)
    }
}