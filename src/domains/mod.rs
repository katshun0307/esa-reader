mod config;

use chrono::{DateTime, Utc};
pub use config::*;
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
    pub tags: Vec<Tag>,
    pub watches: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: User,
    pub updated_by: User,
    pub url: Url,
}
