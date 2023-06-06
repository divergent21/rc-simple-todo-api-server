use serde::Serialize;
use crate::model::Todo;

#[derive(Serialize)]
pub struct Good {
    status: u16,
    data: Todo
}

impl Good {
    pub fn new (data: Todo) -> Self {
        Self {
            status: 200,
            data
        }
    }
}