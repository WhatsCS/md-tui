mod app;
mod utils;
use log::{debug, error, info, trace, warn};
use std::{io, sync::mpsc};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

fn setup_logging() -> Result<(), fern::InitError> {
    // setup logging
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}::{}::{} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("md-tui.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    setup_logging().expect("Setting up logging failed");
    // build terminal thing
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    info!("Created Terminal!");

    let (tx, rx) = mpsc::channel();

    let mut md_app = app::App::new("MangaDex TUI");

    terminal.clear()?;

    loop {
        terminal.draw(|f| md_app.draw(f))?;
    }
    Ok(())
}
