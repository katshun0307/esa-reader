mod app;
mod domains;
mod find_config;
mod http_gateways;
mod widgets;

#[cfg(test)]
extern crate rstest;

#[cfg(test)]
extern crate insta;

use app::App;
use crossterm::{
    execute,
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use find_config::find_config_path;
use ratatui::{DefaultTerminal, Terminal, backend::CrosstermBackend};
use std::io;

use crate::domains::{Config, Theme};

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = get_config().unwrap();
    let (workspace_name, workspace) = config.current_workspace();
    let theme_config = config.get_theme(&workspace_name);
    let theme = Theme::from_config(&theme_config);
    theme.apply_to_md_tui();
    let mut terminal = init_terminal()?;
    let res = App::new(&workspace, theme).run(&mut terminal).await;
    let restore_res = restore_terminal(&mut terminal);
    if let Err(err) = res {
        if let Err(restore_err) = restore_res {
            eprintln!("failed to restore terminal: {}", restore_err);
        }
        return Err(err);
    }
    restore_res?;
    Ok(())
}

fn get_config() -> anyhow::Result<Config> {
    let res = find_config_path("esa-reader", "config.toml")?;
    if let Some(p) = &res.existing {
        println!("Using config file at: {}", p.display());
    } else {
        println!("not found, recommended: {}", res.recommended.display());
        find_config::ensure_parent_dir(&res.recommended)?;
    }
    if let Some(config_path) = res.existing {
        let config_str = std::fs::read_to_string(config_path)?;
        let config: domains::Config = toml::from_str(&config_str)?;
        Ok(config)
    } else {
        anyhow::bail!("config file not found");
    }
}

fn init_terminal() -> io::Result<DefaultTerminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut DefaultTerminal) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        PopKeyboardEnhancementFlags,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}
