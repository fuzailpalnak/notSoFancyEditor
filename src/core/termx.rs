use std::io::Stdout;
use std::io::{self, Write};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;

pub enum UseEditor {
    Continue,
    Quit,
}

pub struct Termx {}
pub struct Editor {}

impl Termx {
    pub fn setup(mut stdout: &Stdout) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(Hide)?;
        return Ok(());
    }

    pub fn cleanup(mut stdout: &Stdout) -> Result<(), std::io::Error> {
        stdout.execute(Show)?;
        stdout.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        return Ok(());
    }
}

impl Editor {
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

    pub fn read(key_event: &KeyEvent) {
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
}
