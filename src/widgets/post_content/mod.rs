use crate::domains::{POSTS, Post};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use md_tui::{
    nodes::{root::Component, textcomponent::TextComponent},
    parser::parse_markdown,
};
use ratatui::{layout::Rect, prelude::Widget};

#[derive(Clone, Debug)]
pub struct PostContent {
    pub post: Post,
    pub markdown_content: String,
    pub scroll: u16,
}

impl Default for PostContent {
    fn default() -> Self {
        Self {
            post: POSTS[0].clone(),
            markdown_content: include_str!("../../domains/fixtures/sample_markdown.md").to_string(),
            scroll: 0,
        }
    }
}

impl PostContent {
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.scroll = self.scroll.saturating_add(1),
            KeyCode::Char('k') | KeyCode::Up => self.scroll = self.scroll.saturating_sub(1),
            _ => {}
        }
    }

    fn render_paragraph(&self, inner_area: Rect, buf: &mut ratatui::buffer::Buffer, scroll: u16) {
        let local_area = Rect::new(0, 0, inner_area.width, inner_area.height);
        let mut inner_buf = ratatui::buffer::Buffer::empty(local_area);
        let component_root = parse_markdown(None, &self.markdown_content, local_area.width);
        let mut text_components: Vec<TextComponent> = component_root
            .children()
            .into_iter()
            .filter_map(|c| match c {
                Component::TextComponent(t) => Some(t.clone()),
                _ => None,
            })
            .collect();
        let mut y_offset = 0;
        for mut comp in text_components.drain(..) {
            let height = comp.height() as u16;
            comp.set_y_offset(y_offset);
            comp.set_scroll_offset(scroll);
            comp.render(local_area, &mut inner_buf);
            y_offset = y_offset.saturating_add(height);
        }
        for y in 0..inner_area.height {
            for x in 0..inner_area.width {
                if let Some(cell) = inner_buf.cell((x, y)).cloned() {
                    if let Some(dst) = buf.cell_mut((inner_area.x + x, inner_area.y + y)) {
                        *dst = cell;
                    }
                }
            }
        }
    }
}

impl Widget for &mut PostContent {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let block = ratatui::widgets::Block::default()
            .title("Post Content")
            .borders(ratatui::widgets::Borders::ALL);
        let inner_area = block.inner(area);
        block.render(area, buf);

        let max_scroll = {
            let component_root = parse_markdown(None, &self.markdown_content, inner_area.width);
            let total_height: u16 = component_root
                .children()
                .into_iter()
                .filter_map(|c| match c {
                    Component::TextComponent(t) => Some(t.height() as u16),
                    _ => None,
                })
                .sum();
            total_height.saturating_sub(inner_area.height)
        };
        self.scroll = self.scroll.min(max_scroll);
        self.render_paragraph(inner_area, buf, self.scroll);
    }
}
