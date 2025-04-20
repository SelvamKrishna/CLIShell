use super::executer::Execute;

pub enum CommandState {
    Null,
    Handled,
    Invalid,
    Exit,
}

pub fn parse_cmd(cmd: String) -> CommandState {
    let tokens: Vec<&str> = cmd
        .split_ascii_whitespace()
        .map(|c| c.trim())
        .collect();

    if tokens.len() == 0 {
        return CommandState::Null;
    }

    if tokens.len() == 1 {
        match tokens[0] {
            "help" => {
                return Execute::help_cmd();
            }
            "exit" => {
                return CommandState::Exit;
            }
            "clear" => {
                return Execute::clear_cmd();
            }
            "ls" => {
                return Execute::ls_cmd();
            }
            _ => {
                return CommandState::Invalid;
            }
        }
    }

    if tokens.len() == 2 {
        match tokens[0] {
            "cd" => {
                return Execute::cd_cmd(tokens[1].to_string());
            }
            "echo" => {
                return Execute::echo_cmd(tokens[1..].join(" "));
            }
            "cat" => {
                return Execute::cat_cmd(tokens[1].to_string());
            }
            "ls" => {
                return Execute::ls_path_cmd(tokens[1].to_string());
            }
            _ => {
                return CommandState::Invalid;
            }
        }
    }

    return CommandState::Handled;
}
