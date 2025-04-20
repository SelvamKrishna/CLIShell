pub struct Execute;

use super::{ log::Log, parser::CommandState };

impl Execute {
    pub fn help_cmd() -> CommandState {
        Log::help_msg("help", "Show this help message").unwrap();
        Log::help_msg("exit", "exits the shell").unwrap();

        Log::info_msg("More commands yet unimplemented").unwrap();
        return CommandState::Handled;
    }
}
