use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    #[serde(default = "Todo::default_id")]
    id: String,

    title: String,
    content: String,

    #[serde(default = "Todo::default_completed")]
    completed: bool,

    #[serde(default = "Todo::default_time")]
    created_at: DateTime<Utc>,
    #[serde(default = "Todo::default_time")]
    updated_at: DateTime<Utc>,
}

impl Todo {
    fn default_completed () -> bool {
        false
    }

    fn default_id () -> String {
        Uuid::new_v4().to_string()
    }

    fn default_time () -> DateTime<Utc> {
        Utc::now()
    }

    #[allow(unused)]
    pub fn new (
        title: String,
        content: String
    ) -> Self {
        Self {
            id: Self::default_id(),
            title,
            content,
            completed: false,
            created_at: Utc::now(),
            updated_at: Utc::now()
        }
    }

    fn set_new_updated_time (&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn complete (&mut self) {
        self.completed = true;
        self.set_new_updated_time();
    }

    pub fn uncomplete (&mut self) {
        self.completed = false;
        self.set_new_updated_time();
    }

    pub fn get_id (&self) -> &str {
        &self.id
    }

    pub fn get_title (&self) -> &str {
        &self.title
    }

    #[allow(unused)]
    pub fn get_content (&self) -> &str {
        &self.content
    }

    pub fn update_title (&mut self, title: String) {
        self.title = title;
        self.set_new_updated_time();
    }

    pub fn update_content (&mut self, content: String) {
        self.content = content;
        self.set_new_updated_time();
    }
}