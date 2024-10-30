use crate::editor::Position;
use crossterm::event::KeyCode;

pub struct KeyStroke;

impl KeyStroke {
    pub fn insert_char(buffer: &mut Vec<String>, cursor_position: &mut Position, input_char: char) {
        match buffer.get_mut(cursor_position.y) {
            Some(line) => {
                line.insert(cursor_position.x, input_char);
                cursor_position.x += 1;
            }
            None => {}
        }
    }

    pub fn new_line(buffer: &mut Vec<String>, cursor_position: &mut Position) {
        if cursor_position.y < buffer.len() {
            buffer.insert(cursor_position.y + 1, String::new());
            cursor_position.x = 0;
            cursor_position.y += 1;
        }
    }

    pub fn backspace(buffer: &mut Vec<String>, cursor_position: &mut Position) {
        if cursor_position.x > 0 {
            buffer[cursor_position.y].remove(cursor_position.x - 1);
            cursor_position.x -= 1;
        } else if cursor_position.y > 0 {
            cursor_position.y -= 1;
            cursor_position.x = buffer[cursor_position.y].len();
        }
    }

    pub fn move_cursor(
        buffer: &mut Vec<String>,
        cursor_position: &mut Position,
        direction: KeyCode,
    ) {
        match direction {
            KeyCode::Up => {
                if cursor_position.y > 0 {
                    cursor_position.y -= 1;
                    if cursor_position.x > buffer[cursor_position.y].len() {
                        cursor_position.x = buffer[cursor_position.y].len();
                    }
                }
            }
            KeyCode::Down => {
                if cursor_position.y < buffer.len() - 1 {
                    cursor_position.y += 1;
                    if cursor_position.x > buffer[cursor_position.y].len() {
                        cursor_position.x = buffer[cursor_position.y].len();
                    }
                }
            }
            KeyCode::Left if cursor_position.x > 0 => cursor_position.x -= 1,
            KeyCode::Right if cursor_position.x < buffer[cursor_position.y].len() => {
                cursor_position.x += 1
            }
            _ => (),
        }
    }
}
