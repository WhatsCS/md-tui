use tui::layout::Constraint;

pub struct TabList<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabList<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabList<'a> {
        TabList { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
    pub fn add(&mut self, new_tab: &'a str) {
        self.titles.push(new_tab);
    }
    pub fn remove(&mut self, old_tab: &'a str) {
        if self.index < 2 {
            return;
        }
        self.titles.pop();
    }
}
