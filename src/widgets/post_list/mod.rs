use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Tabs, Widget,
    },
};

use crate::{
    domains::{Post, PostViewConfig, Theme},
    http_gateways::EsaClientHttpGateway,
};

pub struct PostList {
    pub posts: Vec<Post>,
    pub state: ListState,
    post_views: Vec<PostViewConfig>,
    selected_view: usize,
    pub api: Box<dyn EsaClientHttpGateway>,
    theme: Theme,
}

impl PostList {
    pub fn new(
        api: Box<dyn EsaClientHttpGateway>,
        post_views: Vec<PostViewConfig>,
        theme: Theme,
    ) -> Self {
        Self {
            posts: vec![],
            state: ListState::default(),
            post_views,
            selected_view: 0,
            api,
            theme,
        }
    }
}

impl PostList {
    pub async fn init(&mut self) {
        match self.fetch_posts().await {
            Ok(posts) => {
                self.posts = posts;
                if !self.posts.is_empty() {
                    self.state.select(Some(0));
                }
            }
            Err(e) => {
                eprintln!("failed to fetch posts: {}", e);
            }
        }
    }

    async fn fetch_posts(&self) -> anyhow::Result<Vec<Post>> {
        let query = self
            .post_views
            .get(self.selected_view)
            .and_then(|view| view.query.clone());
        let posts = self.api.fetch_posts(query).await?;
        Ok(posts)
    }

    async fn refresh_posts(&mut self) {
        match self.fetch_posts().await {
            Ok(posts) => {
                self.posts = posts;
                if self.posts.is_empty() {
                    self.state.select(None);
                } else {
                    self.state.select(Some(0));
                }
            }
            Err(e) => {
                eprintln!("failed to fetch posts: {}", e);
            }
        }
    }

    pub async fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.state.select_previous(),
            KeyCode::Char('h') | KeyCode::Left => self.select_prev_view().await,
            KeyCode::Char('l') | KeyCode::Right => self.select_next_view().await,
            _ => {}
        }
    }

    pub async fn watch_selected(&mut self) {
        let Some(post_number) = self.selected_post_number() else {
            return;
        };
        if let Err(e) = self.api.watch_post(&post_number).await {
            eprintln!("failed to watch post: {}", e);
            return;
        }
        self.refresh_posts_keep_selection().await;
    }

    pub async fn unwatch_selected(&mut self) {
        let Some(post_number) = self.selected_post_number() else {
            return;
        };
        if let Err(e) = self.api.unwatch_post(&post_number).await {
            eprintln!("failed to unwatch post: {}", e);
            return;
        }
        self.refresh_posts_keep_selection().await;
    }

    pub async fn star_selected(&mut self) {
        let Some(post_number) = self.selected_post_number() else {
            return;
        };
        if let Err(e) = self.api.star_post(&post_number).await {
            eprintln!("failed to star post: {}", e);
            return;
        }
        self.refresh_posts_keep_selection().await;
    }

    pub async fn unstar_selected(&mut self) {
        let Some(post_number) = self.selected_post_number() else {
            return;
        };
        if let Err(e) = self.api.unstar_post(&post_number).await {
            eprintln!("failed to unstar post: {}", e);
            return;
        }
        self.refresh_posts_keep_selection().await;
    }

    pub fn selected_post(&self) -> Option<&Post> {
        if let Some(selected) = self.state.selected() {
            self.posts.get(selected)
        } else {
            None
        }
    }

    async fn select_prev_view(&mut self) {
        if self.post_views.is_empty() {
            return;
        }
        if self.selected_view == 0 {
            self.selected_view = self.post_views.len().saturating_sub(1);
        } else {
            self.selected_view = self.selected_view.saturating_sub(1);
        }
        self.refresh_posts().await;
    }

    async fn select_next_view(&mut self) {
        if self.post_views.is_empty() {
            return;
        }
        self.selected_view = (self.selected_view + 1) % self.post_views.len();
        self.refresh_posts().await;
    }

    fn selected_post_number(&self) -> Option<crate::domains::PostNumber> {
        let selected = self.state.selected()?;
        let post = self.posts.get(selected)?;
        Some(crate::domains::PostNumber::from(post.post_number.to_i32()))
    }

    async fn refresh_posts_keep_selection(&mut self) {
        let selected = self.state.selected();
        match self.fetch_posts().await {
            Ok(posts) => {
                self.posts = posts;
                if self.posts.is_empty() {
                    self.state.select(None);
                } else {
                    let idx = selected
                        .unwrap_or(0)
                        .min(self.posts.len().saturating_sub(1));
                    self.state.select(Some(idx));
                }
            }
            Err(e) => {
                eprintln!("failed to fetch posts: {}", e);
            }
        }
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles: Vec<Line> = if self.post_views.is_empty() {
            vec![Line::from("Posts")]
        } else {
            self.post_views
                .iter()
                .map(|view| Line::from(view.title.clone()))
                .collect()
        };
        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .title("Post Views")
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(self.theme.muted))
                    .title_style(Style::new().fg(self.theme.primary)),
            )
            .select(self.selected_view)
            .style(Style::new().fg(self.theme.primary))
            .highlight_style(
                Style::new()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD),
            );
        Widget::render(tabs, area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Posts")
            .borders(Borders::ALL)
            .border_style(Style::new().fg(self.theme.muted))
            .title_style(Style::new().fg(self.theme.primary));

        let items: Vec<ListItem> = self
            .posts
            .iter()
            .map(|post| {
                let tags = post
                    .tags
                    .iter()
                    .map(|tag| tag.label.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                let header = if tags.is_empty() {
                    format!("{} {}", post.post_number, post.full_name)
                } else {
                    format!("{} {} {}", post.post_number, post.full_name, tags)
                };
                let updated_at = post.updated_at.format("%Y-%m-%d %H:%M").to_string();
                let meta = format!("\u{f007} @{}  {}", post.updated_by.id.0, updated_at);
                let stats = format!("\u{f41e} {} \u{f441} {}", post.stars, post.watches);
                ListItem::new(vec![
                    Line::from(Span::styled(header, Style::new().fg(self.theme.primary))),
                    Line::from(Span::styled(meta, Style::new().fg(self.theme.muted))),
                    Line::from(Span::styled(stats, Style::new().fg(self.theme.muted))),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::new()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl Widget for &mut PostList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
        let [tabs_area, list_area] = layout.areas(area);
        self.render_tabs(tabs_area, buf);
        self.render_list(list_area, buf);
    }
}
