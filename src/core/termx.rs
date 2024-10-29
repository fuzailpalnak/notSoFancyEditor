use std::io::Stdout;
use std::io::{self, Write};

use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;

pub enum UseEditor {
    Continue,
    Quit,
}

pub struct Termx {}
pub struct InputHandler {}
pub struct KeyStroke {}

impl Termx {
    pub fn setup(mut stdout: &Stdout) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        return Ok(());
    }

    pub fn cleanup(mut stdout: &Stdout) -> Result<(), std::io::Error> {
        stdout.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        return Ok(());
    }
}

impl InputHandler {
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

    pub fn read(
        key_event: &KeyEvent,
        buffer: &mut Vec<String>,
        cursor_x: &mut usize,
        cursor_y: &mut usize,
    ) {
        match key_event.code {
            KeyCode::Char(c) => KeyStroke::data(buffer, cursor_x, cursor_y, c),
            KeyCode::Enter => KeyStroke::enter(buffer, cursor_x, cursor_y),
            KeyCode::Backspace => KeyStroke::back_space(buffer, cursor_x, cursor_y),
            _ => {}
        }
    }

    pub fn render(
        buffer: &[String],
        cursor_x: &usize,
        cursor_y: &usize,
    ) -> Result<(), std::io::Error> {
        let mut stdout = io::stdout();
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

        // println!(
        //     "Not So Fancy Editor - Use arrow keys to navigate, 'Ctrl+S' to save, 'Ctrl+X' to quit."
        // );

        for (i, line) in buffer.iter().enumerate() {
            let position_y = (i + 1) as u16;
            println!("{}", line);
            execute!(stdout, MoveTo(0, position_y))?;
        }

        execute!(stdout, MoveTo(*cursor_x as u16, *cursor_y as u16))?;

        Ok(())
    }
}

impl KeyStroke {
    pub fn data(
        buffer: &mut Vec<String>,
        cursor_x: &mut usize,
        cursor_y: &mut usize,
        input_char: char,
    ) {
        match buffer.get_mut(*cursor_y) {
            Some(line) => {
                line.insert(*cursor_x, input_char);
                *cursor_x += 1;
            }
            None => {}
        }
    }

    pub fn enter(buffer: &mut Vec<String>, cursor_x: &mut usize, cursor_y: &mut usize) {
        match buffer.get_mut(*cursor_y) {
            Some(_line) => {
                buffer.push("\n".to_string());
                buffer.push(String::new());

                *cursor_x = 0;
                *cursor_y += 1;
            }
            None => {}
        }
    }

    pub fn back_space(buffer: &mut Vec<String>, cursor_x: &mut usize, cursor_y: &mut usize) {
        match buffer.get_mut(*cursor_y) {
            Some(_) => {
                if *cursor_x > 0 {
                    buffer[*cursor_y].remove(*cursor_x - 1);
                    *cursor_x -= 1;
                } else if *cursor_y > 0 {
                    let previous_line_length = buffer[*cursor_y - 1].len();
                    let current_line = buffer.remove(*cursor_y);
                    buffer[*cursor_y - 1].push_str(&current_line);
                    *cursor_x = previous_line_length;
                    *cursor_y -= 1;
                }
            }

            None => {}
        }
    }
}
