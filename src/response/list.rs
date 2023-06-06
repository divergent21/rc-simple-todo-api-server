use serde::Serialize;
use crate::model::Todo;

#[derive(Serialize)]
pub struct List {
    status: u16,
    data: Vec<Todo>,
    count: usize
}

impl List {
    pub fn new (data: Vec<Todo>) -> Self {
        Self {
            status: 200,
            count: data.len(),
            data,
        }
    }
}