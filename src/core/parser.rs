use std::path::PathBuf;

use super::executer::Execute;

pub enum CommandState {
    Handled,
    Invalid,
    Exit,
}

pub fn parse_cmd(cmd: String, dir: &mut PathBuf) -> CommandState {
    let tokens: Vec<&str> = cmd.split_ascii_whitespace().collect();

    if tokens.is_empty() {
        return CommandState::Handled;
    }

    match tokens.as_slice() {
        ["help"] => Execute::help_cmd(),

        ["exit"] => CommandState::Exit,
        ["clear"] => Execute::clear_cmd(),

        ["ls"] => Execute::ls_path_cmd(".".to_string()),
        ["ls", path] => Execute::ls_path_cmd(path.to_string()),

        ["cd", path] => Execute::cd_cmd(path.to_string(), dir),
        ["cat", path] => Execute::cat_cmd(path.to_string()),
        ["echo", msg] => Execute::echo_cmd(msg.to_string()),

        _ => CommandState::Invalid,
    }
}
