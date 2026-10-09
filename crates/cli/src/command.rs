//! 输入解析：一行文本 → 命令（纯逻辑，可单测）。

/// 用户命令。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Draw,
    Speak,
    End,
    Help,
    Quit,
}

/// 一行输入的解析结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    /// 空行（忽略）
    Empty,
    /// 已知命令
    Command(Command),
    /// 无法识别的命令（保留去空白后的原文，用于提示）
    Unknown(String),
}

/// 解析一行输入（`q` 与 `quit` 等价；未知命令保留原文）。
pub fn parse(line: &str) -> Input {
    match line.trim() {
        "" => Input::Empty,
        "draw" => Input::Command(Command::Draw),
        "speak" => Input::Command(Command::Speak),
        "end" => Input::Command(Command::End),
        "help" => Input::Command(Command::Help),
        "q" | "quit" => Input::Command(Command::Quit),
        other => Input::Unknown(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_commands_and_aliases() {
        assert_eq!(parse("draw"), Input::Command(Command::Draw));
        assert_eq!(parse("  speak  "), Input::Command(Command::Speak));
        assert_eq!(parse("end"), Input::Command(Command::End));
        assert_eq!(parse("help"), Input::Command(Command::Help));
        assert_eq!(parse("q"), Input::Command(Command::Quit));
        assert_eq!(parse("quit"), Input::Command(Command::Quit));
    }

    #[test]
    fn treats_empty_and_unknown_input() {
        assert_eq!(parse(""), Input::Empty);
        assert_eq!(parse("   "), Input::Empty);
        assert_eq!(parse(" nope "), Input::Unknown("nope".to_string()));
    }
}
