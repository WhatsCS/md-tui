use crate::app::{App, InputMode};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, Paragraph, Tabs};
use tui::Frame;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_search_tab(f, app, chunks[1]),
        1 => draw_follows_tab(f, app, chunks[1]),
        2 => draw_series_tab(f, app, chunks[1]),
        3 => draw_chapter_tab(f, app, chunks[1]),
        _ => {}
    };
fn draw_search_bar<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // Draw the input box
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let search = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Green),
        })
        .block(Block::default().borders(Borders::ALL).title("Search Manga"));
    f.render_widget(search, chunks[0]);
    match app.input_mode {
        // if normal hide cursor (does it by default)
        InputMode::Normal => {}
        InputMode::Editing => {
            // we are editing so time to show the cursor
            f.set_cursor(
                // we need to manually manage cursor position lmao
                chunks[0].x + app.input.len() as u16 + 1,
                chunks[0].y + 1,
            )
        }
    }
}
}

fn draw_search_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_search_bar(f, app, chunks[0]);
}

fn draw_follows_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
}

fn draw_series_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
}

fn draw_chapter_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
}
