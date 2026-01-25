use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style, palette::tailwind::SLATE},
    widgets::{HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::{domains::Post, http_gateways::EsaClientHttpGateway};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct PostList {
    pub posts: Vec<Post>,
    pub state: ListState,
    pub api: Box<dyn EsaClientHttpGateway>,
}

impl PostList {
    pub fn new(api: Box<dyn EsaClientHttpGateway>) -> Self {
        let mut s = Self {
            posts: vec![],
            state: ListState::default(),
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
        let posts = runtime.block_on(async { self.api.fetch_posts().await })?;
        Ok(posts)
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.state.select_previous(),
            _ => {}
        }
    }

    fn render_list(&mut self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let block = ratatui::widgets::Block::default()
            .title("Post List")
            .borders(ratatui::widgets::Borders::ALL);

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
        self.render_list(area, buf);
    }
}
