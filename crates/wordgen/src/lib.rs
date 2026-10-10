//! 词汇生成端口与实现。
//!
//! 词汇生成是**外部子系统**：宿主（Tauri 壳 / CLI）持有具体实现，
//! 拿到结果后转换为核心事件（`WordReceived` / `RequestFailed`）。
//! 本 crate 不依赖 UI，也不感知状态机。
//!
//! 分层（厂商与协议分离，见 ADR 0005）：
//!
//! - [`WordSource`]：端口（trait），宿主与状态机只认识它
//! - [`providers`]：厂商定义（数据：端点 / 密钥环境变量 / 默认模型）
//! - [`openai_chat`]：协议实现（OpenAI 兼容 chat completions 的传输）
//! - [`deepseek`]：生成层（提示词 + 输出解析），组合厂商与协议
//! - [`fake::FakeApi`]：开发用假实现（固定词表轮换）
//! - [`any::AnySource`]：宿主装配用的来源枚举（真实 / 假 / 未配置）

use std::future::Future;

pub mod any;
pub mod deepseek;
pub mod fake;
pub mod openai_chat;
pub mod providers;

/// 词汇来源端口：成功返回词汇，失败返回错误信息。
///
/// 返回类型显式标注 `Send`（而非直接使用 `async fn`），
/// 便于宿主把 future 交给多线程运行时。
pub trait WordSource {
    fn request_word(&mut self) -> impl Future<Output = Result<String, String>> + Send;
}
