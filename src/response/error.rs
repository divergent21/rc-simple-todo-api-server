use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    status: u16,
    message: String
}

impl Error {
    pub fn new (message: String) -> Self {
        Self {
            status: 500,
            message
        }
    }
}