pub enum CommandType {
    Null,
    Handled,
    Invalid,
    Exit,
}

pub fn parse_cmd(cmd: String) -> CommandType {
    let tokens: Vec<&str> = cmd
        .split_ascii_whitespace()
        .map(|c| c.trim())
        .collect();

    if tokens.len() == 0 {
        return CommandType::Null;
    }

    if tokens.len() == 1 {
        match tokens[0] {
            "help" => todo!(),
            "exit" => {
                return CommandType::Exit;
            }
            _ => {
                return CommandType::Invalid;
            }
        }
    }

    return CommandType::Handled;
}
