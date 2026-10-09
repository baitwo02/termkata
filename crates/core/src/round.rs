//! 一轮练习状态机（纯逻辑）。
//!
//! 阶段（依次推进，非法事件一律忽略）：
//!
//! | 阶段       | 含义                             |
//! |------------|----------------------------------|
//! | `Ready`      | 待抽取                           |
//! | `Generating` | 生成中（等待 LLM 返回词汇）      |
//! | `Preparing`  | 准备阶段（收集资料 + 关键词提纲）|
//! | `Speaking`   | 口播阶段                         |
//!
//! 状态转移表：
//!
//! ```text
//! Ready      + RequestWord      -> Generating（清空 error）
//! Generating + WordReceived     -> Preparing（词就位）
//! Generating + RequestFailed    -> Ready（记录错误）
//! Preparing  + StartSpeaking    -> Speaking
//! Speaking   + EndSpeaking      -> Ready（清空词与错误）
//! 其他组合                       -> 状态不变
//! ```
//!
//! 不变量：`word` 非空 ⇒ `phase` ∈ {`Preparing`, `Speaking`}；
//! `error` 仅在 `Ready` 阶段有意义。

/// 一轮练习所处的阶段。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Phase {
    #[default]
    Ready,
    Generating,
    Preparing,
    Speaking,
}

/// 一轮练习的状态快照（不可变值类型）。
///
/// 字段私有：外部只能通过 `transition` 产生新状态、通过访问器读取，
/// 无法自行拼装出非法状态（如 `Preparing` 却没有词）。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RoundState {
    phase: Phase,
    word: Option<String>,
    error: Option<String>,
}

impl RoundState {
    /// 构造初始状态：就绪、无词、无错误。
    pub fn new() -> Self {
        Self::default()
    }

    /// 当前阶段。
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// 当前词汇（若有）。
    pub fn word(&self) -> Option<&str> {
        self.word.as_deref()
    }

    /// 最近一次抽取失败信息（仅在 `Ready` 阶段有意义）。
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

/// 状态机事件（外部输入的命令或事实）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoundEvent {
    /// 请求生成词汇（宿主据此发起 LLM 调用）。
    RequestWord,
    /// 词汇已生成。
    WordReceived(String),
    /// 生成失败。
    RequestFailed(String),
    /// 开始口播。
    StartSpeaking,
    /// 结束口播。
    EndSpeaking,
}

/// 纯转移函数：非法事件返回原状态。
pub fn transition(state: RoundState, event: RoundEvent) -> RoundState {
    match (state.phase, event) {
        (Phase::Ready, RoundEvent::RequestWord) => RoundState {
            phase: Phase::Generating,
            error: None,
            ..state
        },
        (Phase::Generating, RoundEvent::WordReceived(word)) => RoundState {
            phase: Phase::Preparing,
            word: Some(word),
            error: None,
        },
        (Phase::Generating, RoundEvent::RequestFailed(message)) => RoundState {
            phase: Phase::Ready,
            word: None,
            error: Some(message),
        },
        (Phase::Preparing, RoundEvent::StartSpeaking) => RoundState {
            phase: Phase::Speaking,
            ..state
        },
        (Phase::Speaking, RoundEvent::EndSpeaking) => RoundState::new(),
        _ => state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 从初始状态开始，依次应用事件，得到最终状态。
    fn apply_all(events: impl IntoIterator<Item = RoundEvent>) -> RoundState {
        events.into_iter().fold(RoundState::new(), transition)
    }
    #[test]
    fn starts_ready() {
        assert_eq!(
            RoundState::new(),
            RoundState {
                phase: Phase::Ready,
                word: None,
                error: None,
            }
        );
    }

    #[test]
    fn request_word_moves_to_generating() {
        let state = apply_all([RoundEvent::RequestWord]);
        assert_eq!(state.phase, Phase::Generating);
    }

    #[test]
    fn word_received_moves_to_preparing_with_word() {
        let state = apply_all([
            RoundEvent::RequestWord,
            RoundEvent::WordReceived("SSA（静态单赋值）".into()),
        ]);
        assert_eq!(state.phase, Phase::Preparing);
        assert_eq!(state.word.as_deref(), Some("SSA（静态单赋值）"));
        assert_eq!(state.error, None);
    }

    #[test]
    fn request_failed_returns_to_ready_with_error() {
        let state = apply_all([
            RoundEvent::RequestWord,
            RoundEvent::RequestFailed("API 超时".into()),
        ]);
        assert_eq!(
            state,
            RoundState {
                phase: Phase::Ready,
                word: None,
                error: Some("API 超时".into()),
            }
        );
    }

    #[test]
    fn start_speaking_moves_to_speaking_keeping_word() {
        let state = apply_all([
            RoundEvent::RequestWord,
            RoundEvent::WordReceived("逃逸分析".into()),
            RoundEvent::StartSpeaking,
        ]);
        assert_eq!(state.phase, Phase::Speaking);
        assert_eq!(state.word.as_deref(), Some("逃逸分析"));
    }

    #[test]
    fn end_speaking_returns_to_ready_and_clears() {
        let state = apply_all([
            RoundEvent::RequestWord,
            RoundEvent::WordReceived("写时复制".into()),
            RoundEvent::StartSpeaking,
            RoundEvent::EndSpeaking,
        ]);
        assert_eq!(state, RoundState::new());
    }

    #[test]
    fn request_word_after_failure_clears_error() {
        let state = apply_all([
            RoundEvent::RequestWord,
            RoundEvent::RequestFailed("API 超时".into()),
            RoundEvent::RequestWord,
        ]);
        assert_eq!(state.phase, Phase::Generating);
        assert_eq!(state.error, None);
    }

    /// 序列化格式是宿主与前端（TS 类型）之间的约定，用测试锁定。
    #[cfg(feature = "serde")]
    mod serde_format {
        use super::*;

        #[test]
        fn ready_state_serializes_to_expected_json() {
            let json = serde_json::to_string(&RoundState::new()).unwrap();
            assert_eq!(json, r#"{"phase":"ready","word":null,"error":null}"#);
        }

        #[test]
        fn preparing_state_serializes_to_expected_json() {
            let state = apply_all([
                RoundEvent::RequestWord,
                RoundEvent::WordReceived("SSA".into()),
            ]);
            let json = serde_json::to_string(&state).unwrap();
            assert_eq!(json, r#"{"phase":"preparing","word":"SSA","error":null}"#);
        }
    }
}
