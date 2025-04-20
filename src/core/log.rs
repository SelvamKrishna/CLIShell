use std::io;

use crossterm::{ execute, style::{ Color, Print, ResetColor, SetForegroundColor } };

pub struct Log;

impl Log {
    pub fn error_msg(msg: &str) -> io::Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Red),
            Print("ERROR: "),
            SetForegroundColor(Color::White),
            Print(msg),
            Print("\n"),
            ResetColor
        )?;

        return Ok(());
    }

    pub fn info_msg(msg: &str) -> io::Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Blue),
            Print("INFO: "),
            SetForegroundColor(Color::White),
            Print(msg),
            Print("\n"),
            ResetColor
        )?;

        return Ok(());
    }

    pub fn help_msg(cmd: &str, msg: &str) -> io::Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(format!("{}:\t\t\t", cmd)),
            SetForegroundColor(Color::White),
            Print(msg),
            Print("\n"),
            ResetColor
        )?;

        return Ok(());
    }
}
