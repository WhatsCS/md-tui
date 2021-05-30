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

    pub fn next_tab(&mut self) {
        self.tabs.next();
    }

    pub fn prev_tab(&mut self) {
        self.tabs.prev();
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn add_tab_series(&mut self) {
        // TODO: get series name and use that instead
        self.tabs.add("Series");
    }

    pub fn add_tab_chapter(&mut self) {
        // TODO: get chapter name and use that
        self.tabs.add("Chapter");
    }

    pub fn remove_tab(&mut self) {
        self.tabs.remove()
    }
}
