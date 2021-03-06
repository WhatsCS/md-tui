mod app;
mod ui;
mod utils;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::backend::CrosstermBackend;
#[allow(unused_imports)]
use tui::layout::{Constraint, Direction, Layout};
#[allow(unused_imports)]
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

enum Event<I> {
    Input(I),
}

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

fn main() -> Result<(), Box<dyn Error>> {
    setup_logging().expect("Setting up logging failed");

    // build terminal thing
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    info!("Created Terminal!");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(250);
    thread::spawn(move || {
        info!("Spawning event thread");
        let last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
        }
    });

    let mut md_app = app::App::new("MangaDex TUI");

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui::draw(f, &mut md_app))?;
        match rx.recv()? {
            Event::Input(event) => {
                if md_app.input_mode == app::InputMode::Normal {
                    match event.code {
                        KeyCode::Char('q') => {
                            if event.modifiers == KeyModifiers::CONTROL {
                                disable_raw_mode()?;
                                md_app.quit();
                                execute!(
                                    terminal.backend_mut(),
                                    DisableMouseCapture,
                                    LeaveAlternateScreen,
                                )?;
                                terminal.show_cursor()?;
                                break;
                            }
                        }
                        KeyCode::Char('a') => md_app.prev_tab(),
                        KeyCode::Char('d') => md_app.next_tab(),
                        KeyCode::Char('i') => md_app.input_mode = app::InputMode::Editing,
                        _ => {}
                    }
                } else {
                    match event.code {
                        KeyCode::Char(c) => {
                            md_app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            md_app.input.pop();
                        }
                        KeyCode::Esc => {
                            md_app.input_mode = app::InputMode::Normal;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        if md_app.quit {
            break;
        }
    }
    Ok(())
}
