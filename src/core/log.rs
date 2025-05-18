use std::io;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

macro_rules! log_msg {
    ($color:expr, $label:expr, $msg:expr) => {
        execute!(
            io::stdout(),
            SetForegroundColor($color),
            Print($label),
            SetForegroundColor(Color::White),
            Print($msg),
            Print("\n"),
            ResetColor
        )
    };
}

pub struct Log;

impl Log {
    #[allow(dead_code)]
    pub fn line() -> io::Result<()> {
        execute!(io::stdout(), Print("\n"))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn error_msg(msg: &str) -> io::Result<()> {
        log_msg!(Color::Red, "ERROR: ", msg)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn info_msg(msg: &str) -> io::Result<()> {
        log_msg!(Color::Blue, "INFO: ", msg)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn custom_msg(msg: &str, color: Color) -> io::Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(color),
            Print(msg),
            Print("\n"),
            ResetColor
        )?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn plain_msg(msg: &str) -> io::Result<()> {
        Log::custom_msg(msg, Color::White)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn todo_msg(cmd: &str) -> io::Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(cmd),
            SetForegroundColor(Color::White),
            Print("\t\t\twork in progress"),
            Print("\n"),
            ResetColor
        )?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn help_msg(cmd: &str, msg: &str) -> io::Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(format!("{:<24}", cmd)),
            SetForegroundColor(Color::White),
            Print(msg),
            Print("\n"),
            ResetColor
        )?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn help_args_msg(cmd: &str, args: &str, msg: &str) -> io::Result<()> {
        let cmd_args_len = cmd.len() + 1 + args.len();
        let pad_len = 24usize.saturating_sub(cmd_args_len);
        let padding = " ".repeat(pad_len);

        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(cmd),
            Print(" "),
            SetForegroundColor(Color::Cyan),
            Print(args),
            Print(padding),
            SetForegroundColor(Color::White),
            Print(msg),
            Print("\n"),
            ResetColor
        )?;

        Ok(())
    }
}
