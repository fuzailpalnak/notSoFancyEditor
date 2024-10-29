use crate::editor::Position;

use std::io::Stdout;

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
pub struct Action {}

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

    pub fn register_event(
        key_event: &KeyEvent,
        buffer: &mut Vec<String>,
        cursor_position: &mut Position,
    ) {
        match key_event.code {
            KeyCode::Char(c) => Action::read(buffer, cursor_position, c),
            KeyCode::Enter => Action::enter(buffer, cursor_position),
            KeyCode::Backspace => Action::back_space(buffer, cursor_position),
            _ => {}
        }
    }

    pub fn render(
        buffer: &[String],
        cursor_x: &usize,
        cursor_y: &usize,
        stdout: &mut Stdout,
    ) -> Result<(), std::io::Error> {
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

impl Action {
    pub fn read(buffer: &mut Vec<String>, cursor_position: &mut Position, input_char: char) {
        match buffer.get_mut(cursor_position.y) {
            Some(line) => {
                line.insert(cursor_position.x, input_char);
                cursor_position.x += 1;
            }
            None => {}
        }
    }

    pub fn enter(buffer: &mut Vec<String>, cursor_position: &mut Position) {
        match buffer.get_mut(cursor_position.y) {
            Some(_line) => {
                buffer.push("\n".to_string());
                buffer.push(String::new());

                cursor_position.x = 0;
                cursor_position.y += 1;
            }
            None => {}
        }
    }

    pub fn back_space(buffer: &mut Vec<String>, cursor_position: &mut Position) {
        match buffer.get_mut(cursor_position.y) {
            Some(_) => {
                if cursor_position.x > 0 {
                    buffer[cursor_position.y].remove(cursor_position.x - 1);
                    cursor_position.x -= 1;
                } else if cursor_position.y > 0 {
                    let previous_line_length = buffer[cursor_position.y - 1].len();
                    let current_line = buffer.remove(cursor_position.y);
                    buffer[cursor_position.y - 1].push_str(&current_line);
                    cursor_position.x = previous_line_length;
                    cursor_position.y -= 1;
                }
            }

            None => {}
        }
    }
}
