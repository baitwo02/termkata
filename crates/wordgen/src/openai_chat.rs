//! OpenAI 兼容协议（chat completions）的传输实现。
//!
//! 只负责"发请求、取回文本"：不关心提示词与输出解析（那是生成层的事）。
//! 凡使用该协议的厂商（DeepSeek、OpenAI、Ollama、vLLM…）都可复用本实现。

use std::time::Duration;

use serde_json::json;

/// 一次 chat 请求所需的三要素：端点、鉴权、模型。
#[derive(Debug, Clone)]
pub struct OpenAiChat {
    base_url: String,
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAiChat {
    /// 构造客户端（30 秒超时；rustls，无系统 OpenSSL 依赖）。
    pub fn new(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            model: model.into(),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("构建 HTTP 客户端失败"),
        }
    }

    /// 发送一条 user 消息，返回模型输出的纯文本。
    pub async fn complete(&self, prompt: &str) -> Result<String, String> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let body = json!({
            "model": self.model,
            "messages": [{ "role": "user", "content": prompt }],
            "stream": false,
        });

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|error| format!("请求 {} 失败：{}", self.base_url, error_chain(&error)))?;

        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|error| format!("读取响应失败：{}", error_chain(&error)))?;

        if !status.is_success() {
            return Err(format!(
                "API 返回 {}：{}",
                status.as_u16(),
                truncate(&text, 200)
            ));
        }

        let value: serde_json::Value =
            serde_json::from_str(&text).map_err(|error| format!("响应不是合法 JSON：{error}"))?;
        value["choices"][0]["message"]["content"]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| {
                format!(
                    "响应缺少 choices[0].message.content：{}",
                    truncate(&text, 200)
                )
            })
    }
}

/// 展开错误原因链：reqwest 顶层消息常常只有 "error sending request"，真正原因在 source 里。
fn error_chain(error: &dyn std::error::Error) -> String {
    let mut parts = vec![error.to_string()];
    let mut source = error.source();
    while let Some(cause) = source {
        let text = cause.to_string();
        if parts.last().map(String::as_str) != Some(text.as_str()) {
            parts.push(text);
        }
        source = cause.source();
    }
    parts.join("：")
}

/// 截断过长文本（错误信息里避免刷屏）。
fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut out: String = text.chars().take(max_chars).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_error_source_chain() {
        #[derive(Debug)]
        struct Inner;
        impl std::fmt::Display for Inner {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "内层原因")
            }
        }
        impl std::error::Error for Inner {}

        #[derive(Debug)]
        struct Outer(Inner);
        impl std::fmt::Display for Outer {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "顶层消息")
            }
        }
        impl std::error::Error for Outer {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                Some(&self.0)
            }
        }

        assert_eq!(error_chain(&Outer(Inner)), "顶层消息：内层原因");
    }

    #[test]
    fn truncates_only_when_needed() {
        assert_eq!(truncate("abc", 5), "abc");
        assert_eq!(truncate("abcde", 5), "abcde");
        assert_eq!(truncate("abcdef", 5), "abcde…");
    }

    #[test]
    fn truncates_on_char_boundaries() {
        assert_eq!(truncate("中文词汇", 2), "中文…");
    }
}
