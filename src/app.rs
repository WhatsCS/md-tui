use crate::utils::TabList;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub struct App<'a> {
    pub title: &'a str,
    pub quit: bool,
    pub tabs: TabList<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        // do init stuff
        App {
            title,
            quit: false,
            tabs: TabList::new(vec!["Search", "Follows"]),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(f.size());
        let titles = self
            .tabs
            .titles
            .iter()
            .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title(self.title))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(self.tabs.index);
        f.render_widget(tabs, chunks[0]);
    }

    pub fn next_tab(&mut self) {
        self.tabs.next();
    }

    pub fn prev_tab(&mut self) {
        self.tabs.prev();
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }
}
