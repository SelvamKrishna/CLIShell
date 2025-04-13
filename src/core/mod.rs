mod log;

use std::io;
use log::Log;

use rustyline::{ error::ReadlineError, history::FileHistory, Editor, DefaultEditor };

pub struct Shell {
    shell: Editor<(), FileHistory>,
}

impl Shell {
    pub fn new() -> io::Result<Self> {
        let mut shell = DefaultEditor::new().unwrap();

        if shell.load_history("history.txt").is_err() {
            Log::error_msg("failed to load history")?;
        }

        return Ok(Shell { shell });
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            match self.shell.readline("$ ") {
                Ok(cmd) => {
                    self.shell.add_history_entry(cmd.as_str()).unwrap();
                }
                Err(ReadlineError::Interrupted) => {
                    Log::info_msg("exiting shell..")?;
                    break;
                }
                Err(ReadlineError::Eof) => {
                    Log::info_msg("End of file..")?;
                    break;
                }
                Err(_) => {
                    Log::error_msg("something went wrong..")?;
                    break;
                }
            }
        }
        return Ok(());
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        self.shell.save_history("history.txt").unwrap();
    }
}
