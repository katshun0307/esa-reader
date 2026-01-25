use crate::domains::{Post, PostNumber, Tag, User, UserId};
use chrono::DateTime;
use esa_api::apis::{
    configuration::Configuration,
    default_api::{self, V1TeamsTeamNamePostsGetParams},
};

#[derive(Clone, Debug)]
pub struct EsaClient {
    team_name: String,
    conf: Configuration,
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

#[async_trait::async_trait]
pub trait EsaClientHttpGateway: Send + Sync {
    async fn fetch_posts(&self) -> anyhow::Result<Vec<Post>>;
    async fn fetch_post_content(&self, post_number: &PostNumber) -> anyhow::Result<String>;
}

#[async_trait::async_trait]
impl EsaClientHttpGateway for EsaClient {
    async fn fetch_posts(&self) -> anyhow::Result<Vec<Post>> {
        let params = V1TeamsTeamNamePostsGetParams {
            team_name: self.team_name.to_string(),
            q: Some("sort:updated".to_string()),
            include: None,
            sort: None,
            order: None,
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
        Ok(posts)
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
}

fn convert_post(post: esa_api::models::Post) -> anyhow::Result<Post> {
    let esa_api::models::Post {
        number: Some(post_number),
        name,
        full_name,
        created_at: Some(created_at),
        updated_at: Some(updated_at),
        tags,
        stargazers_count,
        watchers_count,
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
        tags,
        watches,
        created_at,
        updated_at,
        created_by,
        updated_by,
        url,
    })
}

fn convert_user(user_summary: esa_api::models::UserSummary) -> User {
    let esa_api::models::UserSummary {
        name, screen_name, ..
    } = user_summary;
    User {
        name: name.unwrap_or_else(|| "(no name)".to_string()),
        id: UserId(screen_name.unwrap_or_else(|| "(no id)".to_string())),
    }
}
