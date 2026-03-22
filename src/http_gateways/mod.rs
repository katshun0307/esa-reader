use crate::domains::{Post, PostNumber, Tag, User, UserId};
use chrono::DateTime;
use esa_api::apis::{
    configuration::Configuration,
    default_api::{
        self, V1TeamsTeamNamePostsGetParams, V1TeamsTeamNamePostsPostNumberStarDeleteParams,
        V1TeamsTeamNamePostsPostNumberStarPostParams,
        V1TeamsTeamNamePostsPostNumberWatchDeleteParams,
        V1TeamsTeamNamePostsPostNumberWatchPostParams,
    },
};

#[derive(Clone, Debug)]
pub struct EsaClient {
    team_name: String,
    conf: Configuration,
}

#[derive(Clone, Debug)]
pub struct PostListPage {
    pub posts: Vec<Post>,
    pub next_page: Option<i32>,
}

impl EsaClient {
    pub fn new(team_name: &str, api_token: &str) -> Self {
        let mut conf = Configuration::new();
        conf.api_key = Some(esa_api::apis::configuration::ApiKey {
            prefix: None,
            key: api_token.to_string(),
        });
        conf.bearer_access_token = Some(api_token.to_string());
        Self {
            conf,
            team_name: team_name.to_string(),
        }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait EsaClientHttpGateway: Send + Sync {
    async fn fetch_posts(&self, query: Option<String>, page: i32) -> anyhow::Result<PostListPage>;
    async fn fetch_post(&self, post_number: &PostNumber) -> Option<Post>;
    async fn fetch_post_content(&self, post_number: &PostNumber) -> anyhow::Result<String>;
    async fn watch_post(&self, post_number: &PostNumber) -> anyhow::Result<()>;
    async fn unwatch_post(&self, post_number: &PostNumber) -> anyhow::Result<()>;
    async fn star_post(&self, post_number: &PostNumber) -> anyhow::Result<()>;
    async fn unstar_post(&self, post_number: &PostNumber) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl EsaClientHttpGateway for EsaClient {
    async fn fetch_posts(&self, query: Option<String>, page: i32) -> anyhow::Result<PostListPage> {
        let params = V1TeamsTeamNamePostsGetParams {
            team_name: self.team_name.to_string(),
            q: query.or_else(|| Some("sort:updated".to_string())),
            include: None,
            sort: None,
            order: None,
            page: Some(page),
        };

        let response = default_api::v1_teams_team_name_posts_get(&self.conf, params).await?;
        let response_posts = response.posts.unwrap_or_default();
        let mut posts = vec![];
        for post in response_posts {
            match convert_post(post) {
                Ok(p) => posts.push(p),
                Err(e) => eprintln!("failed to convert post: {}", e),
            }
        }
        Ok(PostListPage {
            posts,
            next_page: response.next_page,
        })
    }

    async fn fetch_post(&self, post_number: &PostNumber) -> Option<Post> {
        let params = esa_api::apis::default_api::V1TeamsTeamNamePostsPostNumberGetParams {
            team_name: self.team_name.to_string(),
            post_number: post_number.to_i32(),
            include: None,
        };

        let response = esa_api::apis::default_api::v1_teams_team_name_posts_post_number_get(
            &self.conf, params,
        )
        .await
        .ok()?;
        convert_post(response).ok()
    }

    async fn fetch_post_content(&self, post_number: &PostNumber) -> anyhow::Result<String> {
        let params = esa_api::apis::default_api::V1TeamsTeamNamePostsPostNumberGetParams {
            team_name: self.team_name.to_string(),
            post_number: post_number.to_i32(),
            include: None,
        };

        let response = esa_api::apis::default_api::v1_teams_team_name_posts_post_number_get(
            &self.conf, params,
        )
        .await?;
        let content = response
            .body_md
            .ok_or_else(|| anyhow::anyhow!("missing body_md in post"))?;
        Ok(content)
    }

    async fn watch_post(&self, post_number: &PostNumber) -> anyhow::Result<()> {
        let params = V1TeamsTeamNamePostsPostNumberWatchPostParams {
            team_name: self.team_name.to_string(),
            post_number: post_number.to_i32(),
        };

        default_api::v1_teams_team_name_posts_post_number_watch_post(&self.conf, params).await?;
        Ok(())
    }

    async fn unwatch_post(&self, post_number: &PostNumber) -> anyhow::Result<()> {
        let params = V1TeamsTeamNamePostsPostNumberWatchDeleteParams {
            team_name: self.team_name.to_string(),
            post_number: post_number.to_i32(),
        };

        default_api::v1_teams_team_name_posts_post_number_watch_delete(&self.conf, params).await?;
        Ok(())
    }

    async fn star_post(&self, post_number: &PostNumber) -> anyhow::Result<()> {
        let params = V1TeamsTeamNamePostsPostNumberStarPostParams {
            team_name: self.team_name.to_string(),
            post_number: post_number.to_i32(),
            inline_object: None,
        };

        default_api::v1_teams_team_name_posts_post_number_star_post(&self.conf, params).await?;
        Ok(())
    }

    async fn unstar_post(&self, post_number: &PostNumber) -> anyhow::Result<()> {
        let params = V1TeamsTeamNamePostsPostNumberStarDeleteParams {
            team_name: self.team_name.to_string(),
            post_number: post_number.to_i32(),
        };

        default_api::v1_teams_team_name_posts_post_number_star_delete(&self.conf, params).await?;
        Ok(())
    }
}

pub(crate) fn convert_post(post: esa_api::models::Post) -> anyhow::Result<Post> {
    let esa_api::models::Post {
        number: Some(post_number),
        name,
        full_name,
        created_at: Some(created_at),
        updated_at: Some(updated_at),
        tags,
        stargazers_count,
        watchers_count,
        star,
        watch,
        created_by: Some(created_by),
        updated_by: Some(updated_by),
        url: Some(url),
        ..
    } = post
    else {
        return Err(anyhow::anyhow!("missing required fields in Post"));
    };

    let post_number = PostNumber::from(post_number);
    let name = name.unwrap_or_else(|| "(no title)".to_string());
    let full_name = full_name.unwrap_or_else(|| name.clone());
    let stars = stargazers_count.unwrap_or(0).max(0) as u32;
    let watches = watchers_count.unwrap_or(0).max(0) as u32;
    let starred = star.unwrap_or(false);
    let watched = watch.unwrap_or(false);
    let tags = tags
        .unwrap_or_default()
        .into_iter()
        .map(|label| Tag { label })
        .collect();
    let created_at = DateTime::parse_from_rfc3339(&created_at)?.to_utc();
    let updated_at = DateTime::parse_from_rfc3339(&updated_at)?.to_utc();
    let created_by = convert_user(*created_by);
    let updated_by = convert_user(*updated_by);
    let url = url.parse()?;

    Ok(Post {
        post_number,
        name,
        full_name,
        stars,
        starred,
        tags,
        watches,
        watched,
        created_at,
        updated_at,
        created_by,
        updated_by,
        url,
    })
}

pub(crate) fn convert_user(user_summary: esa_api::models::UserSummary) -> User {
    let esa_api::models::UserSummary {
        name, screen_name, ..
    } = user_summary;
    User {
        name: name.unwrap_or_else(|| "(no name)".to_string()),
        id: UserId(screen_name.unwrap_or_else(|| "(no id)".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use esa_api::models::{Post as ApiPost, UserSummary};
    use rstest::{fixture, rstest};

    // ── Fixtures ──────────────────────────────────────────────────────────────

    #[fixture]
    fn user_summary() -> Box<UserSummary> {
        Box::new(UserSummary {
            name: Some("Alice".to_string()),
            screen_name: Some("alice".to_string()),
            myself: Some(true),
            icon: None,
        })
    }

    /// A fully-populated API post that passes all required-field checks in
    /// `convert_post`. Individual tests can override fields using struct
    /// update syntax to exercise specific edge-cases.
    #[fixture]
    fn api_post(user_summary: Box<UserSummary>) -> ApiPost {
        ApiPost {
            number: Some(42),
            name: Some("My Test Post".to_string()),
            full_name: Some("Category/My Test Post".to_string()),
            created_at: Some("2024-01-01T00:00:00+00:00".to_string()),
            updated_at: Some("2024-01-02T12:30:00+00:00".to_string()),
            url: Some("https://docs.esa.io/posts/42".to_string()),
            tags: Some(vec!["rust".to_string(), "testing".to_string()]),
            stargazers_count: Some(5),
            watchers_count: Some(3),
            star: Some(true),
            watch: Some(false),
            created_by: Some(user_summary.clone()),
            updated_by: Some(user_summary),
            ..ApiPost::new()
        }
    }

    // ── convert_user ──────────────────────────────────────────────────────────

    #[rstest]
    fn test_convert_user_full(user_summary: Box<UserSummary>) {
        let user = convert_user(*user_summary);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.id.0, "alice");
    }

    #[rstest]
    #[case(None, None, "(no name)", "(no id)")]
    #[case(Some("Bob"), None, "Bob", "(no id)")]
    #[case(None, Some("bob"), "(no name)", "bob")]
    #[case(Some("Bob"), Some("bob"), "Bob", "bob")]
    fn test_convert_user_fallbacks(
        #[case] name: Option<&str>,
        #[case] screen_name: Option<&str>,
        #[case] expected_name: &str,
        #[case] expected_id: &str,
    ) {
        let summary = UserSummary {
            name: name.map(str::to_string),
            screen_name: screen_name.map(str::to_string),
            myself: None,
            icon: None,
        };
        let user = convert_user(summary);
        assert_eq!(user.name, expected_name);
        assert_eq!(user.id.0, expected_id);
    }

    // ── convert_post ──────────────────────────────────────────────────────────

    #[rstest]
    fn test_convert_post_success(api_post: ApiPost) {
        let post = convert_post(api_post).expect("conversion should succeed");

        assert_eq!(post.post_number.to_i32(), 42);
        assert_eq!(post.post_number.to_string(), "#42");
        assert_eq!(post.name, "My Test Post");
        assert_eq!(post.full_name, "Category/My Test Post");
        assert_eq!(post.stars, 5);
        assert_eq!(post.watches, 3);
        assert!(post.starred);
        assert!(!post.watched);
        assert_eq!(post.tags.len(), 2);
        assert_eq!(post.tags[0].label, "rust");
        assert_eq!(post.tags[1].label, "testing");
        assert_eq!(post.created_by.name, "Alice");
        assert_eq!(post.created_by.id.0, "alice");
        assert_eq!(post.url.as_str(), "https://docs.esa.io/posts/42");
    }

    /// Fields whose absence triggers the `else` branch in `convert_post`.
    #[rstest]
    #[case::missing_number({ let mut p = ApiPost::new(); p.number = None; p })]
    #[case::missing_created_at({ let mut p = ApiPost::new(); p.number = Some(1); p.created_at = None; p })]
    #[case::missing_url({
        let mut p = ApiPost::new();
        p.number = Some(1);
        p.created_at = Some("2024-01-01T00:00:00+00:00".to_string());
        p.updated_at = Some("2024-01-01T00:00:00+00:00".to_string());
        p.url = None;
        p
    })]
    fn test_convert_post_missing_required_field(#[case] incomplete_post: ApiPost) {
        assert!(convert_post(incomplete_post).is_err());
    }

    /// Optional fields should fall back to sensible defaults instead of
    /// causing a failure.
    #[rstest]
    fn test_convert_post_optional_fields_use_defaults(api_post: ApiPost) {
        let post_with_nones = ApiPost {
            name: None,
            full_name: None,
            stargazers_count: None,
            watchers_count: None,
            star: None,
            watch: None,
            tags: None,
            ..api_post
        };

        let post = convert_post(post_with_nones).expect("conversion should succeed");
        assert_eq!(post.name, "(no title)");
        assert_eq!(post.full_name, "(no title)", "full_name falls back to name");
        assert_eq!(post.stars, 0);
        assert_eq!(post.watches, 0);
        assert!(!post.starred);
        assert!(!post.watched);
        assert!(post.tags.is_empty());
    }

    /// Negative counts from the API should be clamped to zero.
    #[rstest]
    fn test_convert_post_negative_counts_are_clamped(api_post: ApiPost) {
        let post_neg = ApiPost {
            stargazers_count: Some(-3),
            watchers_count: Some(-1),
            ..api_post
        };
        let post = convert_post(post_neg).expect("conversion should succeed");
        assert_eq!(post.stars, 0);
        assert_eq!(post.watches, 0);
    }

    // ── MockEsaClientHttpGateway ───────────────────────────────────────────────

    /// Demonstrate injecting a mock gateway: callers that depend on the trait
    /// can be tested without a real network connection.
    #[tokio::test]
    async fn test_mock_gateway_returns_configured_posts() {
        let mut mock = MockEsaClientHttpGateway::new();

        mock.expect_fetch_posts()
            .withf(|query, page| query.is_none() && *page == 1)
            .once()
            .returning(|_, _| {
                Ok(PostListPage {
                    posts: vec![],
                    next_page: Some(2),
                })
            });

        let result = mock.fetch_posts(None, 1).await.unwrap();
        assert!(result.posts.is_empty());
        assert_eq!(result.next_page, Some(2));
    }

    #[tokio::test]
    async fn test_mock_gateway_propagates_errors() {
        let mut mock = MockEsaClientHttpGateway::new();

        mock.expect_fetch_posts()
            .returning(|_, _| Err(anyhow::anyhow!("network error")));

        let err = mock.fetch_posts(Some("q=test".to_string()), 1).await;
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().to_string(), "network error");
    }
}
