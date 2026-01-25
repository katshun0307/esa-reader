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
use find_config::find_config_path;
use std::io;

use crate::domains::Config;

fn main() -> io::Result<()> {
    let config = get_config().unwrap();
    ratatui::run(|terminal| App::new(&config.current_workspace().1).run(terminal))
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
