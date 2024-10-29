#[path = "termx.rs"]
mod termx;

use std::io::{stdout, Stdout};

use crossterm::event::{self, Event, KeyEventKind};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&mut self) {
        let result = self.repl();
        match result {
            Err(err) => panic!("{err:#?}"),
            _ => println!("Goodbye."),
        }
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        let mut buffer: Vec<String> = vec![String::new()];
        let (mut cursor_x, mut cursor_y) = (0, 0);

        let mut stdout: Stdout = stdout();

        termx::Termx::setup(&mut stdout)?;

        loop {
            let event = event::read()?;

            if let Event::Key(key_event) = event {
                match termx::InputHandler::quit(&key_event) {
                    termx::UseEditor::Quit => {
                        return termx::Termx::cleanup(&stdout);
                    }

                    termx::UseEditor::Continue => match key_event.kind {
                        KeyEventKind::Press => {
                            termx::InputHandler::read(
                                &key_event,
                                &mut buffer,
                                &mut cursor_x,
                                &mut cursor_y,
                            );
                            termx::InputHandler::render(&buffer, &cursor_x, &cursor_y)?;
                        }
                        _ => (),
                    },
                }
            }
        }
    }
}
