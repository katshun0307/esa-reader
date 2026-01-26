mod config;
mod theme;

use chrono::{DateTime, Utc};
pub use config::*;
pub use theme::Theme;
use core::fmt;
use url::Url;

#[derive(Clone, Debug)]
pub struct UserId(pub String);

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub id: UserId,
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct PostNumber(i32);

impl From<i32> for PostNumber {
    fn from(value: i32) -> Self {
        PostNumber(value)
    }
}

impl PostNumber {
    pub fn to_i32(&self) -> i32 {
        self.0
    }
}

impl fmt::Display for PostNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Post {
    pub post_number: PostNumber,
    pub name: String,
    pub full_name: String,
    pub stars: u32,
    pub starred: bool,
    pub tags: Vec<Tag>,
    pub watches: u32,
    pub watched: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: User,
    pub updated_by: User,
    pub url: Url,
}
