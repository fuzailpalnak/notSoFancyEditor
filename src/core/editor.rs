#[path = "termx.rs"]
mod termx;

#[path = "key_stroke.rs"]
mod key_stroke;
use key_stroke::KeyStroke;

use chrono::{DateTime, Local};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::fs::File;
use std::io::{self, stdout, Stdout, Write};

/// Represents the outcome of handling an editor event.
pub enum EditorAction {
    Continue,
    Quit,
}

/// Position of the cursor in the editor buffer.
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Main text editor struct, containing buffer and cursor information.
pub struct Editor {
    buffer: Vec<String>,
    cursor_position: Position,
    stdout: Stdout,
    datetime: DateTime<Local>,
}

impl Editor {
    /// Creates a new editor instance with default settings.
    pub fn new() -> Self {
        Self {
            buffer: vec![String::new()],
            cursor_position: Position { x: 0, y: 0 },
            stdout: stdout(),
            datetime: Local::now(),
        }
    }

    /// Saves the editor's buffer to a file with a timestamped name.
    pub fn save(&self) -> io::Result<()> {
        let filename = format!("output_{}.txt", self.datetime.format("%Y-%m-%d_%H-%M-%S"));
        let mut file = File::create(&filename)?;
        for line in &self.buffer {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    /// Registers key events for editor functionality.
    pub fn handle_event(&mut self, key_event: &KeyEvent) -> EditorAction {
        match key_event.code {
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                match self.save() {
                    Ok(()) => EditorAction::Continue,
                    Err(_) => EditorAction::Quit,
                }
            }
            KeyCode::Char('x') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                EditorAction::Quit
            }
            KeyCode::Char(c) => {
                KeyStroke::insert_char(&mut self.buffer, &mut self.cursor_position, c);
                EditorAction::Continue
            }
            KeyCode::Enter => {
                KeyStroke::new_line(&mut self.buffer, &mut self.cursor_position);
                EditorAction::Continue
            }
            KeyCode::Backspace => {
                KeyStroke::backspace(&mut self.buffer, &mut self.cursor_position);
                EditorAction::Continue
            }
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                KeyStroke::move_cursor(&mut self.buffer, &mut self.cursor_position, key_event.code);
                EditorAction::Continue
            }
            _ => EditorAction::Continue,
        }
    }

    /// Main editor loop to read and process events.
    pub fn run(&mut self) {
        if let Err(err) = self.event_loop() {
            eprintln!("Error: {:?}", err);
        }
    }

    fn event_loop(&mut self) -> io::Result<()> {
        termx::Termx::setup(&mut self.stdout)?;
        termx::Termx::render(&self.buffer, &self.cursor_position, &mut self.stdout)?;

        loop {
            let event = event::read()?;
            if let Event::Key(key_event) = event {
                if let KeyEventKind::Press = key_event.kind {
                    match self.handle_event(&key_event) {
                        EditorAction::Continue => {
                            termx::Termx::render(
                                &self.buffer,
                                &self.cursor_position,
                                &mut self.stdout,
                            )?;
                        }
                        EditorAction::Quit => {
                            return termx::Termx::cleanup(&self.stdout);
                        }
                    }
                }
            }
        }
    }
}
