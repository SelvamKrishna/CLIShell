use super::executer::Execute;

pub enum CommandState {
    Null,
    Handled,
    Invalid,
    Exit,
}

pub fn parse_cmd(cmd: String) -> CommandState {
    let tokens: Vec<&str> = cmd.split_ascii_whitespace().map(str::trim).collect();

    if tokens.is_empty() {
        return CommandState::Null;
    }

    return match tokens.as_slice() {
        ["help"] => Execute::help_cmd(),

        ["exit"] => CommandState::Exit,
        ["quit"] => CommandState::Exit,

        ["clear"] => Execute::clear_cmd(),
        ["cls"] => Execute::clear_cmd(),

        ["ls"] => Execute::ls_path_cmd(".".to_string()),
        ["dir"] => Execute::ls_path_cmd(".".to_string()),
        ["ls", path] => Execute::ls_path_cmd(path.to_string()),
        ["dir", path] => Execute::ls_path_cmd(path.to_string()),

        ["cd", path] => Execute::cd_cmd(path.to_string()),
        ["cat", path] => Execute::cat_cmd(path.to_string()),
        ["echo", msg] => Execute::echo_cmd(msg.to_string()),
        _ => CommandState::Invalid,
    };
}
