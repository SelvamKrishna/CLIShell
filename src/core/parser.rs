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
            _ => {
                return CommandState::Invalid;
            }
        }
    }

    return CommandState::Handled;
}
