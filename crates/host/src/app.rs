//! 宿主编排：把命令与事件接到状态机与词汇生成器上（本层不打印任何东西）。
//!
//! 对应 MVU 中的 Update：`命令 + 当前状态 → 新状态`。

use termkata_core::round::{transition, Phase, RoundEvent, RoundState};
use termkata_wordgen::WordSource;

/// 一次事件派发的结果：转移是否生效。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionResult {
    /// 转移已生效（状态已更新）
    Applied,
    /// 事件被忽略（当前阶段不允许该操作）
    Rejected(Phase),
}

/// 宿主编排：持有权威状态与词汇来源。
pub struct App<S: WordSource> {
    state: RoundState,
    source: S,
}

impl<S: WordSource> App<S> {
    pub fn new(source: S) -> Self {
        Self {
            state: RoundState::new(),
            source,
        }
    }

    /// 当前状态（只读）。
    pub fn state(&self) -> &RoundState {
        &self.state
    }

    /// 发起抽取：就绪 → 生成中。返回转移结果。
    pub fn begin_draw(&mut self) -> TransitionResult {
        self.dispatch(RoundEvent::RequestWord)
    }

    /// 等待生成结果并应用（在 `begin_draw` 返回 `Applied` 后调用）。
    pub async fn finish_draw(&mut self) {
        let event = match self.source.request_word().await {
            Ok(word) => RoundEvent::WordReceived(word),
            Err(message) => RoundEvent::RequestFailed(message),
        };
        let _ = self.dispatch(event);
    }

    /// 开始口播：准备 → 口播。
    pub fn start_speaking(&mut self) -> TransitionResult {
        self.dispatch(RoundEvent::StartSpeaking)
    }

    /// 结束口播：口播 → 就绪。
    pub fn end_speaking(&mut self) -> TransitionResult {
        self.dispatch(RoundEvent::EndSpeaking)
    }

    /// 把事件派发给状态机；状态未变化即视为被拒绝。
    fn dispatch(&mut self, event: RoundEvent) -> TransitionResult {
        let before = self.state.phase();
        self.state = transition(self.state.clone(), event);
        if self.state.phase() == before {
            TransitionResult::Rejected(before)
        } else {
            TransitionResult::Applied
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use termkata_wordgen::fake::FakeApi;

    use crate::exec::block_on;

    fn app() -> App<FakeApi> {
        App::new(FakeApi::new())
    }

    #[test]
    fn draw_from_ready_lands_on_first_word() {
        let mut app = app();
        assert_eq!(app.begin_draw(), TransitionResult::Applied);
        block_on(app.finish_draw());
        assert_eq!(app.state().phase(), Phase::Preparing);
        assert_eq!(app.state().word(), Some("SSA（静态单赋值）"));
    }

    #[test]
    fn speaking_before_preparing_is_rejected() {
        let mut app = app();
        assert_eq!(app.start_speaking(), TransitionResult::Rejected(Phase::Ready));
    }

    #[test]
    fn full_round_then_second_draw_rotates_words() {
        let mut app = app();

        assert_eq!(app.begin_draw(), TransitionResult::Applied);
        block_on(app.finish_draw());
        assert_eq!(app.start_speaking(), TransitionResult::Applied);
        assert_eq!(app.end_speaking(), TransitionResult::Applied);
        assert_eq!(app.state().phase(), Phase::Ready);

        assert_eq!(app.begin_draw(), TransitionResult::Applied);
        block_on(app.finish_draw());
        assert_eq!(app.state().word(), Some("逃逸分析"));
    }
}
