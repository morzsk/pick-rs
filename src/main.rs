use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::collections::HashSet;
use std::io::stdout;

struct State {
    entries: Vec<String>,
    cursor: usize,
    selected: HashSet<usize>,
}

impl State {
    fn move_up(&mut self) {
        if self.cursor == 0 {
            self.cursor = self.entries.len() - 1;
        } else {
            self.cursor -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.cursor == self.entries.len() - 1 {
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

    let mut state = State {
        entries,
        cursor: 0,
        selected: HashSet::new(),
    };

    enable_raw_mode().expect("failed to enable raw mode");

    loop {
        execute!(stdout(), Clear(ClearType::FromCursorDown), MoveTo(0, 1)).unwrap();

        for (i, entry) in state.entries.iter().enumerate() {
            let is_cursor = i == state.cursor;
            let is_selected = state.selected.contains(&i);
            let prefix = if is_cursor { ">" } else { " " };
            let marker = if is_selected { "[x]" } else { " - " };
            execute!(
                stdout(),
                Print(format!("{} {} {}\r\n", prefix, marker, entry))
            )
            .unwrap();
        }

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => state.move_up(),
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => state.move_down(),
                KeyCode::Char(' ') => state.toggle_selected(),
                _ => {}
            }
        }
    }

    disable_raw_mode().expect("failed to disable raw mode");

    for i in state.selected {
        println!("{}", state.entries[i]);
    }
}
