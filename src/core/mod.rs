mod executer;
mod log;
mod parser;

use log::Log;
use parser::CommandState;
use std::{env, io, path::PathBuf};

use rustyline::{DefaultEditor, Editor, error::ReadlineError, history::FileHistory};

pub struct Shell {
    shell: Editor<(), FileHistory>,

    dir: PathBuf,
}

impl Shell {
    pub fn new() -> io::Result<Self> {
        let mut shell: Editor<(), FileHistory> = DefaultEditor::new().unwrap();

        if shell.load_history("history.txt").is_err() {
            Log::error_msg("failed to load history")?;
        }

        let dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(_) => {
                Log::error_msg("failed to allocate current directory.")?;
                return Err(io::Error::other("failed to get current directory"));
            }
        };

        Ok(Shell { shell, dir })
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            let line = match self
                .shell
                .readline(&format!("$ {} > ", self.dir.to_str().unwrap()))
            {
                Ok(cmd) => cmd,
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
            };

            self.shell.add_history_entry(line.as_str()).unwrap();
            match parser::parse_cmd(line, &mut self.dir) {
                CommandState::Invalid => {
                    Log::error_msg("invalid command - type 'help' for list of possible commands")?;
                }
                CommandState::Exit => break,
                _ => {}
            }
        }

        Log::info_msg("exiting shell..")?;
        Ok(())
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        self.shell.save_history("history.txt").unwrap();
    }
}
