mod log;
mod parser;
mod executer;

use std::io;
use log::Log;
use parser::CommandState;

use rustyline::{ error::ReadlineError, history::FileHistory, Editor, DefaultEditor };

pub struct Shell(Editor<(), FileHistory>);

impl Shell {
    pub fn new() -> io::Result<Self> {
        let mut shell = DefaultEditor::new().unwrap();

        if shell.load_history("history.txt").is_err() {
            Log::error_msg("failed to load history")?;
        }

        return Ok(Shell(shell));
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            match self.0.readline("$ ") {
                Ok(cmd) => {
                    self.0.add_history_entry(cmd.as_str()).unwrap();
                    let cmd_res = parser::parse_cmd(cmd);
                    if matches!(cmd_res, CommandState::Invalid) {
                        Log::error_msg("invalid shell command.")?;
                    } else if matches!(cmd_res, CommandState::Exit) {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    Log::error_msg("interrupted.")?;
                    break;
                }
                Err(ReadlineError::Eof) => {
                    Log::error_msg("end of file.")?;
                    break;
                }
                Err(_) => {
                    Log::error_msg("something went wrong.")?;
                    break;
                }
            }
        }

        Log::info_msg("exiting shell..")?;
        return Ok(());
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        self.0.save_history("history.txt").unwrap();
    }
}
