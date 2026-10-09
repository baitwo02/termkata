/**
 * 与 Rust `termkata-core::round` 的 serde 输出对齐。
 *
 * 格式由 core 的单测锁定（见 `crates/core/src/round.rs` 的 serde_format 测试）。
 */
export type RoundPhase = 'ready' | 'generating' | 'preparing' | 'speaking';

export interface RoundState {
  phase: RoundPhase;
  word: string | null;
  error: string | null;
}
