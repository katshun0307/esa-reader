use crate::domains::WorkspaceConfig;
use crate::http_gateways::EsaClient;
use crate::widgets::{self};
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
};
use std::io;
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::interval;
use std::process::Command;

pub struct App {
    exit: bool,
    post_list: widgets::PostList,
    post_content: widgets::PostContent,
}

impl App {
    pub fn new(conf: &WorkspaceConfig) -> Self {
        let api = Box::new(EsaClient::new(&conf.team_name(), &conf.token()));
        let post_views = conf.post_views.values().cloned().collect();
        Self {
            exit: false,
            post_list: widgets::PostList::new(api.clone(), post_views),
            post_content: widgets::PostContent::new(api),
        }
    }

    /// runs the application's main loop until the user quits
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.post_list.init().await;
        let mut events = EventStream::new();
        let mut tick = interval(Duration::from_millis(250));
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(&mut events, &mut tick).await?;
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

    async fn handle_events(
        &mut self,
        events: &mut EventStream,
        tick: &mut tokio::time::Interval,
    ) -> io::Result<()> {
        tokio::select! {
            maybe_event = events.next() => {
                if let Some(Ok(Event::Key(key_event))) = maybe_event {
                    if key_event.kind == KeyEventKind::Press {
                        self.handle_key_event(key_event).await;
                    }
                }
            }
            _ = tick.tick() => {}
        }
        Ok(())
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.post_list.handle_key(key_event).await;
        self.post_content.handle_key(key_event);
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Enter => {
                if let Some(selected_post) = self.post_list.selected_post() {
                    if let Err(e) = self.post_content.show_post(selected_post).await {
                        eprintln!("failed to show post: {}", e);
                    }
                }
            }
            KeyCode::Char('w') => self.post_list.watch_selected().await,
            KeyCode::Char('W') => self.post_list.unwatch_selected().await,
            KeyCode::Char('s') => self.post_list.star_selected().await,
            KeyCode::Char('S') => self.post_list.unstar_selected().await,
            KeyCode::Char('o') => self.open_selected_post_in_browser(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn open_selected_post_in_browser(&self) {
        let Some(selected_post) = self.post_list.selected_post() else {
            return;
        };
        let url = selected_post.url.as_str();
        #[cfg(target_os = "macos")]
        let result = Command::new("open").arg(url).status();
        #[cfg(target_os = "linux")]
        let result = Command::new("xdg-open").arg(url).status();
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        let result: Result<std::process::ExitStatus, std::io::Error> =
            Err(std::io::Error::new(std::io::ErrorKind::Other, "unsupported OS"));
        if let Err(e) = result {
            eprintln!("failed to open browser: {}", e);
        }
    }
}
