use std::collections::HashSet;

pub struct Selection {
    pub cursor: usize,
    pub selected: HashSet<usize>,
    pub length: usize,
}

impl Selection {
    pub fn move_up(&mut self) {
        if self.cursor == 0 {
            self.cursor = self.length - 1;
        } else {
            self.cursor -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor == self.length - 1 {
            self.cursor = 0;
        } else {
            self.cursor += 1;
        }
    }

    pub fn toggle_selected(&mut self) {
        if self.selected.contains(&self.cursor) {
            self.selected.remove(&self.cursor);
        } else {
            self.selected.insert(self.cursor);
        }
    }
}
