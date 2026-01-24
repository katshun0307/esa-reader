mod fixtures;
pub use fixtures::*;

use chrono::{DateTime, Utc};
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
pub struct Post {
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
