use crate::domains::WorkspaceConfig;
use crate::http_gateways::EsaClient;
use crate::widgets::{self};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
};
use std::io;

pub struct App {
    exit: bool,
    post_list: widgets::PostList,
    post_content: widgets::PostContent,
}

impl App {
    pub fn new(conf: &WorkspaceConfig) -> Self {
        let api = Box::new(EsaClient::new(&conf.team_name(), &conf.token()));
        Self {
            exit: false,
            post_list: widgets::PostList::new(api.clone()),
            post_content: widgets::PostContent::new(api),
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let frame_area = frame.area();
        let horizontal = Layout::horizontal([Constraint::Fill(1); 2]);
        let [left_area, right_area] = horizontal.areas(frame_area);
        frame.render_widget(&mut self.post_list, left_area);
        frame.render_widget(&mut self.post_content, right_area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.post_list.handle_key(key_event);
        self.post_content.handle_key(key_event);
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Enter => {
                if let Some(selected_post) = self.post_list.selected_post() {
                    self.post_content.show_post(selected_post);
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
