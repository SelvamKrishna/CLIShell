use super::{log::Log, parser::CommandState};
use std::{
    env,
    fs::{self, read_to_string},
    io::stdout,
    path::PathBuf,
};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::Color,
    terminal::{Clear, ClearType},
};

pub struct Execute;

impl Execute {
    pub fn help_cmd() -> CommandState {
        Log::line().unwrap();
        Log::help_msg("help", "show this help message").unwrap();
        Log::help_msg("exit", "exits the shell").unwrap();
        Log::help_msg("clear", "clears the screen").unwrap();
        Log::help_msg("ls", "list contents of current directory").unwrap();
        Log::help_args_msg("ls", "path", "list contents of path directory").unwrap();
        Log::help_args_msg("cd", "path", "change working directory to path").unwrap();
        Log::help_args_msg("cat", "path", "prints file contents to console").unwrap();
        Log::help_args_msg("echo", "text", "prints text to console").unwrap();
        Log::line().unwrap();

        Log::info_msg("more commands yet unimplemented").unwrap();
        CommandState::Handled
    }

    pub fn clear_cmd() -> CommandState {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap_or_else(|e| {
            Log::error_msg(&format!("failed to clear screen: {}", e)).unwrap();
        });

        CommandState::Handled
    }

    pub fn ls_path_cmd(path: String) -> CommandState {
        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(e) => {
                Log::error_msg(&format!("failed to read directory: {}", e)).unwrap();
                return CommandState::Handled;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().into_owned();

            if path.is_dir() {
                Log::custom_msg(&format!("{}/", file_name), Color::DarkCyan).unwrap();
            } else {
                Log::custom_msg(&file_name.to_string(), Color::Cyan).unwrap();
            }
        }

        CommandState::Handled
    }

    pub fn cd_cmd(path: String, dir: &mut PathBuf) -> CommandState {
        if let Err(e) = env::set_current_dir(&path) {
            Log::error_msg(&format!("failed to change directory: {}", e)).unwrap();
        }

        match env::current_dir() {
            Ok(new_dir) => *dir = new_dir,
            Err(e) => {
                Log::error_msg(&format!("failed to get new directory: {}", e)).unwrap();
                return CommandState::Invalid;
            }
        };

        CommandState::Handled
    }

    #[allow(unused)]
    pub fn cat_cmd(path: String) -> CommandState {
        let content = match read_to_string(path) {
            Ok(text) => text,
            Err(_) => return CommandState::Invalid,
        };

        if let Err(e) = Log::plain_msg(&content) {
            Log::error_msg(&format!("cat failed: {}", e)).unwrap();
            return CommandState::Invalid;
        }

        CommandState::Handled
    }

    #[allow(unused)]
    pub fn echo_cmd(msg: String) -> CommandState {
        if let Err(e) = Log::plain_msg(&msg) {
            Log::error_msg(&format!("echo failed: {}", e)).unwrap();
            return CommandState::Invalid;
        }

        CommandState::Handled
    }
}
