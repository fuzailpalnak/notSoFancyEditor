use crate::editor::Position;

use std::io::Stdout;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;

pub struct Termx {}

impl Termx {
    pub fn setup(mut stdout: &Stdout) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        return Ok(());
    }

    pub fn cleanup(mut stdout: &Stdout) -> Result<(), std::io::Error> {
        stdout.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        return Ok(());
    }

    pub fn render(
        buffer: &[String],
        cursor_position: &Position,
        stdout: &mut Stdout,
    ) -> Result<(), std::io::Error> {
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

        for (i, line) in buffer.iter().enumerate() {
            let position_y = (i + 1) as u16;
            println!("{}", line);
            execute!(stdout, MoveTo(0, position_y))?;
        }

        execute!(
            stdout,
            MoveTo(cursor_position.x as u16, cursor_position.y as u16)
        )?;

        let (terminal_width, _) = size()?;
        let instructions = "Use <^> arrows to navigate, [Ctrl+X] to exit, [Ctrl+S] to save";
        let instruction_length = instructions.len() as u16;
        let center_position = (terminal_width - instruction_length) / 2;

        let highlighted_instructions = format!("\x1b[1;32m{}\x1b[0m", instructions);

        execute!(stdout, MoveTo(center_position, (buffer.len()) as u16))?;
        println!("{}", highlighted_instructions);

        execute!(
            stdout,
            MoveTo(cursor_position.x as u16, cursor_position.y as u16)
        )?;

        Ok(())
    }
}
