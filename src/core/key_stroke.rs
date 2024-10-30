use crate::editor::Position;

pub struct KeyStroke {}

impl KeyStroke {
    pub fn read(buffer: &mut Vec<String>, cursor_position: &mut Position, input_char: char) {
        match buffer.get_mut(cursor_position.y) {
            Some(line) => {
                line.insert(cursor_position.x, input_char);
                cursor_position.x += 1;
            }
            None => {}
        }
    }

    pub fn move_to_newline(buffer: &mut Vec<String>, cursor_position: &mut Position) {
        match buffer.get_mut(cursor_position.y) {
            Some(_) => {
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

    pub fn move_up(buffer: &mut Vec<String>, cursor_position: &mut Position) {
        match buffer.get_mut(cursor_position.y) {
            Some(_) => {
                if cursor_position.y > 0 {
                    cursor_position.y -= 1;

                    if cursor_position.x > buffer[cursor_position.y].len() {
                        cursor_position.x = buffer[cursor_position.y].len();
                    }
                }
            }
            None => {}
        }
    }

    pub fn move_down(buffer: &mut Vec<String>, cursor_position: &mut Position) {
        match buffer.get_mut(cursor_position.y) {
            Some(_) => {
                if cursor_position.y < buffer.len() - 1 {
                    cursor_position.y += 1;

                    if cursor_position.x > buffer[cursor_position.y].len() {
                        cursor_position.x = buffer[cursor_position.y].len();
                    }
                }
            }
            None => {}
        }
    }

    pub fn move_left(buffer: &mut Vec<String>, cursor_position: &mut Position) {
        match buffer.get_mut(cursor_position.y) {
            Some(_) => {
                if cursor_position.x > 0 {
                    cursor_position.x -= 1;
                } else if cursor_position.y > 0 {
                    cursor_position.y -= 1;
                    cursor_position.x = buffer[cursor_position.y].len();
                }
            }
            None => {}
        }
    }

    pub fn move_right(buffer: &mut Vec<String>, cursor_position: &mut Position) {
        match buffer.get_mut(cursor_position.y) {
            Some(_) => {
                if cursor_position.x < buffer[cursor_position.y].len() {
                    cursor_position.x += 1;
                } else if cursor_position.y < buffer.len() - 1 {
                    cursor_position.y += 1;
                    cursor_position.x = 0;
                }
            }
            None => {}
        }
    }
}
