//! 渲染：状态 → 终端文本（纯函数，可单测）。
//!
//! 对应 MVU 中的 View：界面文案只出现在宿主层，core 保持无文案。

use termkata_core::round::{Phase, RoundState};

/// 抽取进行中的提示（阻塞式 CLI 的进度反馈）。
pub const GENERATING_HINT: &str = "生成中…";

/// 当前状态 → 一行文本。
pub fn state_text(state: &RoundState) -> String {
    match state.phase() {
        Phase::Ready => match state.error() {
            Some(error) => format!("[就绪] 上次抽取失败：{error}"),
            None => "[就绪] 等待抽取".to_string(),
        },
        Phase::Generating => "[生成中] 正在请求词汇…".to_string(),
        Phase::Preparing => format!("[准备] 当前词汇：{}", state.word_text()),
        Phase::Speaking => format!("[口播] 当前词汇：{}", state.word_text()),
    }
}

/// 被拒绝的操作提示。
pub fn rejected_text(phase: Phase) -> String {
    format!("当前阶段不能执行该操作（{}）", phase_text(phase))
}

/// 帮助文本。
pub fn help_text() -> &'static str {
    "命令：\n  draw   抽取一个词汇（FakeApi 轮换生成）\n  speak  准备结束，开始口播\n  end    结束口播，回到就绪\n  help   显示本帮助\n  q      退出"
}

/// 显示层扩展：核心值 → 界面文案。
trait RoundDisplay {
    /// 当前词汇的显示文本（无词时给出占位符）。
    fn word_text(&self) -> &str;
}

impl RoundDisplay for RoundState {
    fn word_text(&self) -> &str {
        self.word().unwrap_or("(无)")
    }
}

fn phase_text(phase: Phase) -> &'static str {
    match phase {
        Phase::Ready => "就绪",
        Phase::Generating => "生成中",
        Phase::Preparing => "准备",
        Phase::Speaking => "口播",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use termkata_core::round::{transition, RoundEvent};

    fn preparing(word: &str) -> RoundState {
        let state = transition(RoundState::new(), RoundEvent::RequestWord);
        transition(state, RoundEvent::WordReceived(word.to_string()))
    }

    fn failed(message: &str) -> RoundState {
        let state = transition(RoundState::new(), RoundEvent::RequestWord);
        transition(state, RoundEvent::RequestFailed(message.to_string()))
    }

    #[test]
    fn renders_ready_and_error_states() {
        assert_eq!(state_text(&RoundState::new()), "[就绪] 等待抽取");
        assert_eq!(
            state_text(&failed("API 超时")),
            "[就绪] 上次抽取失败：API 超时"
        );
    }

    #[test]
    fn renders_word_states() {
        assert_eq!(state_text(&preparing("SSA")), "[准备] 当前词汇：SSA");

        let speaking = transition(preparing("SSA"), RoundEvent::StartSpeaking);
        assert_eq!(state_text(&speaking), "[口播] 当前词汇：SSA");
    }

    #[test]
    fn rejected_text_includes_phase() {
        assert_eq!(
            rejected_text(Phase::Preparing),
            "当前阶段不能执行该操作（准备）"
        );
    }
}
