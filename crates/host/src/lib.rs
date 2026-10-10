//! termkata 宿主编排：把状态机、词汇生成器与"当前状态"连接起来。
//!
//! 本 crate 不含任何 UI / 打印逻辑：CLI、Tauri（及未来的其他宿主）复用它，
//! 自行决定如何渲染与交互。

pub mod app;
pub mod env;
pub mod exec;

pub use app::{App, TransitionStatus};
