#[path = "termx.rs"]
mod termx;

use std::io::{stdout, Stdout};

use crossterm::event::{self, Event, KeyEventKind};

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

    pub fn run(&mut self) {
        let result = self.repl();
        match result {
            Err(err) => panic!("{err:#?}"),
            _ => println!("Goodbye."),
        }
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        termx::Termx::setup(&mut self.stdout)?;

        loop {
            let event = event::read()?;

            if let Event::Key(key_event) = event {
                match termx::InputHandler::quit(&key_event) {
                    termx::UseEditor::Quit => {
                        return termx::Termx::cleanup(&self.stdout);
                    }

                    termx::UseEditor::Continue => match key_event.kind {
                        KeyEventKind::Press => {
                            termx::InputHandler::register_event(
                                &key_event,
                                &mut self.buffer,
                                &mut self.cursor_position,
                            );
                            termx::InputHandler::render(
                                &self.buffer,
                                &self.cursor_position.x,
                                &self.cursor_position.y,
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
