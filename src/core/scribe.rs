#[path = "termx.rs"]
mod termx;

use std::io::{stdout, Stdout};

use crossterm::event::{self, Event, KeyEventKind};

pub struct NotSoFancy {}

impl NotSoFancy {
    pub fn default() -> Self {
        NotSoFancy {}
    }

    pub fn run(&mut self) {
        let result = self.repl();
        match result {
            Err(err) => panic!("{err:#?}"),
            _ => println!("Goodbye."),
        }
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        let mut stdout: Stdout = stdout();

        termx::Termx::setup(&mut stdout)?;
        println!("Not So Fancy Editor (Press Ctrl + X to quit)");
        print!(">> ");

        loop {
            let event = event::read()?;

            if let Event::Key(key_event) = event {
                match termx::Editor::quit(&key_event) {
                    termx::UseEditor::Quit => {
                        return termx::Termx::cleanup(&stdout);
                    }

                    termx::UseEditor::Continue => match key_event.kind {
                        KeyEventKind::Press => termx::Editor::read(&key_event),
                        _ => (),
                    },
                }
            }
        }
    }
}
