use crossterm::ExecutableCommand;
use std::io::{self, Write};
use std::io::{stdout, Stdout};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

enum UseEditor {
    Continue, // Represents a success, storing a value of type `T`
    Quit,
}

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

    fn quit(&self, key_event: &KeyEvent) -> UseEditor {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => UseEditor::Quit,
            _ => UseEditor::Continue,
        }
    }

    fn read(&self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => {
                print!("{}", c); // Print the typed character
                io::stdout().flush().unwrap();
            }
            KeyCode::Enter => {
                // Handle Enter key, move to the next line
                println!();
                print!(">> ");
                io::stdout().flush().unwrap();
            }
            KeyCode::Backspace => {
                // Handle Backspace key, move cursor back and clear the character
                print!("\x08 \x08"); // "\x08" is the backspace escape code
                io::stdout().flush().unwrap();
            }
            _ => {}
        }
    }

    fn seup_terminal(&self, mut stdout: &Stdout) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(Hide)?;
        return Ok(());
    }

    fn cleanup_terminal(&self, mut stdout: &Stdout) -> Result<(), std::io::Error> {
        stdout.execute(Show)?;
        stdout.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        return Ok(());
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        let mut stdout: Stdout = stdout();

        self.seup_terminal(&mut stdout)?;
        println!("Not So Fancy Editor (Press Ctrl + X to quit)");
        print!(">> ");

        loop {
            let event = event::read()?;

            if let Event::Key(key_event) = event {
                match self.quit(&key_event) {
                    UseEditor::Quit => {
                        return self.cleanup_terminal(&stdout);
                    }

                    UseEditor::Continue => match key_event.kind {
                        KeyEventKind::Press => self.read(&key_event),
                        _ => (),
                    },
                }
            }
        }
    }
}
