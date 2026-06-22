use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::collections::HashSet;
use std::io::stdout;

struct Selection {
    cursor: usize,
    selected: HashSet<usize>,
}

impl Selection {
    fn move_up(&mut self, len: usize) {
        if self.cursor == 0 {
            self.cursor = len - 1;
        } else {
            self.cursor -= 1;
        }
    }

    fn move_down(&mut self, len: usize) {
        if self.cursor == len - 1 {
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
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dir = std::path::PathBuf::from(&args[0]);

    let entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("failed to read dir")
        .map(|e| {
            e.expect("failed to read entry")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .collect();

    let mut selection = Selection {
        cursor: 0,
        selected: HashSet::new(),
    };

    enable_raw_mode().expect("failed to enable raw mode");

    let (col, row) = cursor::position().expect("failed to get cursor position");
    // todo: handle case where cursor is moved
    // todo: use tty (I think) for cleaner piping

    loop {
        execute!(stdout(), MoveTo(col, row), Clear(ClearType::FromCursorDown))
            .expect("failed to clear and move cursor");

        for (i, entry) in entries.iter().enumerate() {
            let is_cursor = i == selection.cursor;
            let is_selected = selection.selected.contains(&i);
            execute!(stdout(), Print(format_entry(entry, is_cursor, is_selected)))
                .expect("failed to print entry");
        }

        if let Event::Key(key) = event::read().expect("failed to read event") {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                    selection.move_up(entries.len())
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                    selection.move_down(entries.len())
                }
                KeyCode::Char(' ') => selection.toggle_selected(),
                _ => {}
            }
        }
    }

    disable_raw_mode().expect("failed to disable raw mode");

    for i in selection.selected {
        println!("{}", entries[i]);
    }
}
