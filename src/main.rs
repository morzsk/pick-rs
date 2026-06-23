use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

struct Selection {
    cursor: usize,
    selected: HashSet<usize>,
    length: usize,
}

impl Selection {
    fn move_up(&mut self) {
        if self.cursor == 0 {
            self.cursor = self.length - 1;
        } else {
            self.cursor -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.cursor == self.length - 1 {
            self.cursor = 0;
        } else {
            self.cursor += 1;
        }
    }

    fn toggle_selected(&mut self) {
        if self.selected.contains(&self.cursor) {
            self.selected.remove(&self.cursor);
        } else {
            self.selected.insert(self.cursor);
        }
    }
}

fn format_entry(entry: &str, is_cursor: bool, is_selected: bool) -> String {
    let prefix = if is_cursor { ">" } else { " " };
    let marker = if is_selected { "[x]" } else { " - " };
    format!("{} {} {}\r\n", prefix, marker, entry)
}

fn main() {
    let entries: Vec<String> = std::env::args().skip(1).collect();

    let mut selection = Selection {
        cursor: 0,
        selected: HashSet::new(),
        length: entries.len(),
    };

    let tty = OpenOptions::new()
        .write(true)
        .open("/dev/tty")
        .expect("failed to open /dev/tty");
    let mut tty = BufWriter::new(tty);

    enable_raw_mode().expect("failed to enable raw mode");

    let (col, row) = cursor::position().expect("failed to get cursor position");
    queue!(tty, cursor::Hide).expect("failed to hide cursor");
    // todo: handle case where cursor is moved

    loop {
        queue!(tty, MoveTo(col, row), Clear(ClearType::FromCursorDown))
            .expect("failed to clear and move cursor");

        for (i, entry) in entries.iter().enumerate() {
            let is_cursor = i == selection.cursor;
            let is_selected = selection.selected.contains(&i);
            queue!(tty, Print(format_entry(entry, is_cursor, is_selected)))
                .expect("failed to print entry");
        }

        tty.flush().expect("failed to flush tty");

        if let Event::Key(key) = event::read().expect("failed to read event") {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => selection.move_up(),
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => selection.move_down(),
                KeyCode::Char(' ') => selection.toggle_selected(),
                _ => {}
            }
        }
    }

    queue!(tty, cursor::Show).expect("failed to show cursor");
    tty.flush().expect("failed to flush tty");
    disable_raw_mode().expect("failed to disable raw mode");

    for i in selection.selected {
        println!("{}", entries[i]);
    }
}
