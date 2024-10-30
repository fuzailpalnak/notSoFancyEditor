#[path = "termx.rs"]
mod termx;

#[path = "key_stroke.rs"]
mod key_stroke;

use std::io::{stdout, Stdout};

use crossterm::event::{self, Event, KeyEventKind};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum UseEditor {
    Continue,
    Quit,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Editor {
    buffer: Vec<String>,
    cursor_position: Position,
    stdout: Stdout,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            buffer: vec![String::new()],
            cursor_position: Position { x: 0, y: 0 },
            stdout: stdout(),
        }
    }

    pub fn register_event(
        key_event: &KeyEvent,
        buffer: &mut Vec<String>,
        cursor_position: &mut Position,
    ) {
        match key_event.code {
            KeyCode::Char(c) => key_stroke::KeyStroke::read(buffer, cursor_position, c),
            KeyCode::Enter => key_stroke::KeyStroke::move_to_newline(buffer, cursor_position),
            KeyCode::Backspace => key_stroke::KeyStroke::back_space(buffer, cursor_position),
            KeyCode::Up => key_stroke::KeyStroke::move_up(buffer, cursor_position),
            KeyCode::Down => key_stroke::KeyStroke::move_down(buffer, cursor_position),
            KeyCode::Left => key_stroke::KeyStroke::move_left(buffer, cursor_position),
            KeyCode::Right => key_stroke::KeyStroke::move_right(buffer, cursor_position),
            _ => {}
        }
    }

    pub fn quit(key_event: &KeyEvent) -> UseEditor {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => UseEditor::Quit,
            _ => UseEditor::Continue,
        }
    }

    pub fn run(&mut self) {
        let result = self.repl();
        match result {
            Err(err) => panic!("{err:#?}"),
            _ => println!("Goodbye."),
        }
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        termx::Termx::setup(&mut self.stdout)?;
        termx::Termx::render(&self.buffer, &self.cursor_position, &mut self.stdout)?;

        loop {
            let event = event::read()?;

            if let Event::Key(key_event) = event {
                match Editor::quit(&key_event) {
                    UseEditor::Quit => {
                        return termx::Termx::cleanup(&self.stdout);
                    }

                    UseEditor::Continue => match key_event.kind {
                        KeyEventKind::Press => {
                            Editor::register_event(
                                &key_event,
                                &mut self.buffer,
                                &mut self.cursor_position,
                            );
                            termx::Termx::render(
                                &self.buffer,
                                &self.cursor_position,
                                &mut self.stdout,
                            )?;
                        }
                        _ => (),
                    },
                }
            }
        }
    }
}
