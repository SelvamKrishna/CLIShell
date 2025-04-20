use std::io;

use crossterm::{ execute, style::{ Color, Print, ResetColor, SetForegroundColor } };

pub struct Log;

impl Log {
    pub fn line() -> io::Result<()> {
        execute!(io::stdout(), Print("\n"))?;
        Ok(())
    }

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

        Ok(())
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

        Ok(())
    }

    pub fn help_msg(cmd: &str, msg: &str) -> io::Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(format!("{}\t\t\t", cmd)),
            SetForegroundColor(Color::White),
            Print(msg),
            Print("\n"),
            ResetColor
        )?;

        Ok(())
    }

    pub fn help_with_args_msg(cmd: &str, args: &str, msg: &str) -> io::Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(cmd.to_string()),
            SetForegroundColor(Color::Cyan),
            Print(format!(" {}\t\t\t", args)),
            SetForegroundColor(Color::White),
            Print(msg),
            Print("\n"),
            ResetColor
        )?;

        Ok(())
    }

    pub fn custom_msg(msg: &str, color: Color) -> io::Result<()> {
        execute!(io::stdout(), SetForegroundColor(color), Print(msg), Print("\n"), ResetColor)?;
        Ok(())
    }
}
