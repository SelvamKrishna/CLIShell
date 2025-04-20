use std::{ fs, io::stdout };
use super::{ log::Log, parser::CommandState };

use crossterm::{ cursor::MoveTo, execute, style::Color, terminal::{ Clear, ClearType } };

pub struct Execute;

impl Execute {
    pub fn help_cmd() -> CommandState {
        Log::line().unwrap();
        Log::help_msg("help", "Show this help message").unwrap();
        Log::help_msg("exit", "exits the shell").unwrap();
        Log::help_msg("clear", "clears the screen").unwrap();
        Log::help_msg("ls", "list contents of current directory").unwrap();
        Log::help_with_args_msg("ls", "path", "list contents of path directory").unwrap();
        Log::line().unwrap();

        Log::info_msg("More commands yet unimplemented").unwrap();
        return CommandState::Handled;
    }

    pub fn clear_cmd() -> CommandState {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap_or_else(|e| {
            Log::error_msg(&format!("Failed to clear screen: {}", e)).unwrap();
        });
        return CommandState::Handled;
    }

    pub fn ls_path_cmd(path: String) -> CommandState {
        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(e) => {
                Log::error_msg(&format!("Failed to read directory: {}", e)).unwrap();
                return CommandState::Handled;
            }
        };

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name = entry.file_name().to_string_lossy().into_owned();

                if path.is_dir() {
                    Log::custom_msg(&format!("{}/", file_name), Color::DarkMagenta).unwrap();
                } else {
                    Log::custom_msg(&format!("{}", file_name), Color::Cyan).unwrap();
                }
            }
        }

        return CommandState::Handled;
    }

    #[allow(unused)]
    pub fn cd_cmd(path: String) -> CommandState {
        return CommandState::Handled;
    }

    #[allow(unused)]
    pub fn cat_cmd(path: String) -> CommandState {
        return CommandState::Handled;
    }

    #[allow(unused)]
    pub fn echo_cmd(msg: String) -> CommandState {
        return CommandState::Handled;
    }
}
