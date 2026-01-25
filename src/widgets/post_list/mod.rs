use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, palette::tailwind::SLATE},
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Tabs, Widget,
    },
};

use crate::{
    domains::{Post, PostViewConfig},
    http_gateways::EsaClientHttpGateway,
};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct PostList {
    pub posts: Vec<Post>,
    pub state: ListState,
    post_views: Vec<PostViewConfig>,
    selected_view: usize,
    pub api: Box<dyn EsaClientHttpGateway>,
}

impl PostList {
    pub fn new(api: Box<dyn EsaClientHttpGateway>, post_views: Vec<PostViewConfig>) -> Self {
        let mut s = Self {
            posts: vec![],
            state: ListState::default(),
            post_views,
            selected_view: 0,
            api,
        };
        s.init();
        s
    }
}

impl PostList {
    fn init(&mut self) {
        match self.fetch_posts() {
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

    fn fetch_posts(&self) -> anyhow::Result<Vec<Post>> {
        let runtime = tokio::runtime::Runtime::new()?;
        let query = self
            .post_views
            .get(self.selected_view)
            .and_then(|view| view.query.clone());
        let posts = runtime.block_on(async { self.api.fetch_posts(query).await })?;
        Ok(posts)
    }

    fn refresh_posts(&mut self) {
        match self.fetch_posts() {
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

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.state.select_previous(),
            KeyCode::Char('h') | KeyCode::Left => self.select_prev_view(),
            KeyCode::Char('l') | KeyCode::Right => self.select_next_view(),
            _ => {}
        }
    }

    pub fn watch_selected(&mut self) {
        let Some(post_number) = self.selected_post_number() else {
            return;
        };
        let runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(e) => {
                eprintln!("failed to create tokio runtime: {}", e);
                return;
            }
        };
        if let Err(e) = runtime.block_on(async { self.api.watch_post(&post_number).await }) {
            eprintln!("failed to watch post: {}", e);
            return;
        }
        self.refresh_posts_keep_selection();
    }

    pub fn unwatch_selected(&mut self) {
        let Some(post_number) = self.selected_post_number() else {
            return;
        };
        let runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(e) => {
                eprintln!("failed to create tokio runtime: {}", e);
                return;
            }
        };
        if let Err(e) = runtime.block_on(async { self.api.unwatch_post(&post_number).await }) {
            eprintln!("failed to unwatch post: {}", e);
            return;
        }
        self.refresh_posts_keep_selection();
    }

    pub fn star_selected(&mut self) {
        let Some(post_number) = self.selected_post_number() else {
            return;
        };
        let runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(e) => {
                eprintln!("failed to create tokio runtime: {}", e);
                return;
            }
        };
        if let Err(e) = runtime.block_on(async { self.api.star_post(&post_number).await }) {
            eprintln!("failed to star post: {}", e);
            return;
        }
        self.refresh_posts_keep_selection();
    }

    pub fn unstar_selected(&mut self) {
        let Some(post_number) = self.selected_post_number() else {
            return;
        };
        let runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,
            Err(e) => {
                eprintln!("failed to create tokio runtime: {}", e);
                return;
            }
        };
        if let Err(e) = runtime.block_on(async { self.api.unstar_post(&post_number).await }) {
            eprintln!("failed to unstar post: {}", e);
            return;
        }
        self.refresh_posts_keep_selection();
    }

    pub fn selected_post(&self) -> Option<&Post> {
        if let Some(selected) = self.state.selected() {
            self.posts.get(selected)
        } else {
            None
        }
    }

    fn select_prev_view(&mut self) {
        if self.post_views.is_empty() {
            return;
        }
        if self.selected_view == 0 {
            self.selected_view = self.post_views.len().saturating_sub(1);
        } else {
            self.selected_view = self.selected_view.saturating_sub(1);
        }
        self.refresh_posts();
    }

    fn select_next_view(&mut self) {
        if self.post_views.is_empty() {
            return;
        }
        self.selected_view = (self.selected_view + 1) % self.post_views.len();
        self.refresh_posts();
    }

    fn selected_post_number(&self) -> Option<crate::domains::PostNumber> {
        let selected = self.state.selected()?;
        let post = self.posts.get(selected)?;
        Some(crate::domains::PostNumber::from(post.post_number.to_i32()))
    }

    fn refresh_posts_keep_selection(&mut self) {
        let selected = self.state.selected();
        match self.fetch_posts() {
            Ok(posts) => {
                self.posts = posts;
                if self.posts.is_empty() {
                    self.state.select(None);
                } else {
                    let idx = selected.unwrap_or(0).min(self.posts.len().saturating_sub(1));
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
            .block(Block::default().title("Post Views").borders(Borders::ALL))
            .select(self.selected_view)
            .highlight_style(SELECTED_STYLE);
        Widget::render(tabs, area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title("Posts").borders(Borders::ALL);

        let items: Vec<ListItem> = self
            .posts
            .iter()
            .map(|post| {
                let content = format!("{} (⭐ {})", post.name, post.stars);
                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
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
