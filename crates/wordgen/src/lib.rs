//! 词汇生成端口与实现。
//!
//! 词汇生成是**外部子系统**：宿主（Tauri 壳 / CLI）持有具体实现，
//! 拿到结果后转换为核心事件（`WordReceived` / `RequestFailed`）。
//! 本 crate 不依赖 UI，也不感知状态机。
//!
//! - [`WordSource`]：端口（trait）
//! - [`fake::FakeApi`]：开发用假实现（固定词表轮换；真实 LLM API 后续接入）

use std::future::Future;

pub mod fake;

/// 词汇来源端口：成功返回词汇，失败返回错误信息。
///
/// 返回类型显式标注 `Send`（而非直接使用 `async fn`），
/// 便于宿主把 future 交给多线程运行时。
pub trait WordSource {
    fn request_word(&mut self) -> impl Future<Output = Result<String, String>> + Send;
}
