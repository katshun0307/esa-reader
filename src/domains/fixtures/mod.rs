use chrono::{TimeZone, Utc};
use std::sync::LazyLock;
use url::Url;

use crate::domains::*;

pub static POSTS: LazyLock<Vec<Post>> = LazyLock::new(|| {
    vec![
        Post {
            name: "hi!".to_string(),
            full_name: "日報/2015/05/09/hi! #api #dev".to_string(),
            stars: 12,
            tags: vec![
                Tag {
                    label: "api".to_string(),
                },
                Tag {
                    label: "dev".to_string(),
                },
            ],
            watches: 3,
            created_at: Utc.with_ymd_and_hms(2015, 5, 9, 2, 54, 50).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2015, 5, 9, 2, 54, 51).unwrap(),
            created_by: User {
                name: "Atsuo Fukaya".to_string(),
                id: UserId("fukayatsu".to_string()),
            },
            updated_by: User {
                name: "Atsuo Fukaya".to_string(),
                id: UserId("fukayatsu".to_string()),
            },
            url: Url::parse("https://docs.esa.io/posts/1").unwrap(),
        },
        Post {
            name: "TUI初期構成".to_string(),
            full_name: "開発/2024/09/01/TUI初期構成 #rust #tui".to_string(),
            stars: 7,
            tags: vec![
                Tag {
                    label: "rust".to_string(),
                },
                Tag {
                    label: "tui".to_string(),
                },
            ],
            watches: 2,
            created_at: Utc.with_ymd_and_hms(2024, 9, 1, 9, 15, 10).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2024, 9, 1, 10, 2, 4).unwrap(),
            created_by: User {
                name: "Shun Katsuda".to_string(),
                id: UserId("katshun0307".to_string()),
            },
            updated_by: User {
                name: "Shun Katsuda".to_string(),
                id: UserId("katshun0307".to_string()),
            },
            url: Url::parse("https://docs.esa.io/posts/42").unwrap(),
        },
        Post {
            name: "検索メモ".to_string(),
            full_name: "メモ/2025/01/12/検索メモ #ux #idea".to_string(),
            stars: 4,
            tags: vec![
                Tag {
                    label: "ux".to_string(),
                },
                Tag {
                    label: "idea".to_string(),
                },
            ],
            watches: 1,
            created_at: Utc.with_ymd_and_hms(2025, 1, 12, 14, 8, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2025, 1, 12, 18, 30, 22).unwrap(),
            created_by: User {
                name: "Yui Takahashi".to_string(),
                id: UserId("yui".to_string()),
            },
            updated_by: User {
                name: "Yui Takahashi".to_string(),
                id: UserId("yui".to_string()),
            },
            url: Url::parse("https://docs.esa.io/posts/108").unwrap(),
        },
    ]
});
