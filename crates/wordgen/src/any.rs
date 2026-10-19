//! 宿主装配用的词汇来源：真实 API / 假实现 / 未配置。
//!
//! 用 enum 而非 trait object：`WordSource::request_word` 返回 `impl Future`，
//! 该 trait 不是对象安全（不能 `Box<dyn WordSource>`）。

use std::future::Future;

use crate::deepseek::DeepSeekSource;
use crate::fake::FakeApi;
use crate::providers::DEEPSEEK;
use crate::WordSource;

/// 宿主可装配的词汇来源。
#[derive(Debug, Clone)]
pub enum AnySource {
    /// 真实 LLM API。
    DeepSeek(DeepSeekSource),
    /// 开发用假实现（固定词表轮换）。
    Fake(FakeApi),
    /// 未配置：抽取时返回清晰错误，**不静默回退**假实现。
    Unconfigured { reason: String },
}

impl AnySource {
    /// 从环境变量（`.env` 应已由宿主加载）构造 DeepSeek 来源。
    pub fn deepseek_from_env() -> Result<Self, String> {
        Self::deepseek_from_key(std::env::var(DEEPSEEK.api_key_env).ok())
    }

    /// 未配置占位：抽取时返回 `reason`。
    pub fn unconfigured(reason: impl Into<String>) -> Self {
        Self::Unconfigured {
            reason: reason.into(),
        }
    }

    /// 开发用假实现。
    pub fn fake() -> Self {
        Self::Fake(FakeApi::new())
    }

    /// 由可选的 key 值构造（拆出来以便单测，避免改进程环境变量）。
    fn deepseek_from_key(key: Option<String>) -> Result<Self, String> {
        match key.map(|value| value.trim().to_string()) {
            Some(key) if !key.is_empty() => Ok(Self::DeepSeek(DeepSeekSource::new(key))),
            _ => Err(format!(
                "未配置 {}：请写入 .env 或设置环境变量",
                DEEPSEEK.api_key_env
            )),
        }
    }
}

impl WordSource for AnySource {
    fn request_word(&mut self) -> impl Future<Output = Result<String, String>> + Send {
        async move {
            match self {
                Self::DeepSeek(source) => source.request_word().await,
                Self::Fake(fake) => fake.request_word().await,
                Self::Unconfigured { reason } => Err(reason.clone()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use super::*;

    /// 测试用极简 block_on：假实现/未配置分支的 future 立即就绪。
    fn block_on<F: Future>(fut: F) -> F::Output {
        let mut fut = pin!(fut);
        let mut cx = Context::from_waker(Waker::noop());
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("future 应当立即就绪"),
        }
    }

    #[test]
    fn fake_variant_delegates_to_fake_api() {
        let mut source = AnySource::fake();
        assert_eq!(
            block_on(source.request_word()).unwrap(),
            "SSA（静态单赋值）"
        );
    }

    #[test]
    fn unconfigured_reports_reason() {
        let mut source = AnySource::unconfigured("未配置测试 key");
        assert_eq!(
            block_on(source.request_word()),
            Err("未配置测试 key".to_string())
        );
    }

    #[test]
    fn missing_key_reports_env_var_name() {
        let error = AnySource::deepseek_from_key(None).unwrap_err();
        assert!(
            error.contains("DEEPSEEK_API_KEY"),
            "错误应包含环境变量名：{error}"
        );
        let error = AnySource::deepseek_from_key(Some("   ".to_string())).unwrap_err();
        assert!(error.contains("DEEPSEEK_API_KEY"));
    }

    #[test]
    fn present_key_builds_deepseek_source() {
        assert!(matches!(
            AnySource::deepseek_from_key(Some("sk-test".to_string())),
            Ok(AnySource::DeepSeek(_))
        ));
    }
}
