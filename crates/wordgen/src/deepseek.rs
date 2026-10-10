//! DeepSeek 词汇来源：组合厂商定义（[`crate::providers::DEEPSEEK`]）、
//! 协议实现（[`crate::openai_chat::OpenAiChat`]）与生成层（提示词 + 解析）。
//!
//! 对状态机而言它只是一个 [`WordSource`]；厂商与协议细节到此为止。

use std::future::Future;

use crate::openai_chat::OpenAiChat;
use crate::providers::DEEPSEEK;
use crate::WordSource;

/// 出题提示词（首版固定；兴趣领域作为上下文留到 v0.1.1）。
const PROMPT: &str = "\
你是一个计算机术语出题人。请随机给出一个计算机科学领域有一定深度的术语，\
涵盖但不限于：编译原理、操作系统、分布式系统、数据库、算法、计算机体系结构。\
要求：只输出术语本身（可含英文缩写与中文全称），不要解释、不要编号、不要引号。";

/// DeepSeek 词汇来源。
#[derive(Debug, Clone)]
pub struct DeepSeekSource {
    chat: OpenAiChat,
}

impl DeepSeekSource {
    /// 用 API key 构造（端点与默认模型取自厂商定义）。
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            chat: OpenAiChat::new(DEEPSEEK.base_url, api_key, DEEPSEEK.default_model),
        }
    }
}

impl WordSource for DeepSeekSource {
    fn request_word(&mut self) -> impl Future<Output = Result<String, String>> + Send {
        let chat = self.chat.clone();
        async move {
            let raw = chat.complete(PROMPT).await?;
            parse_word(&raw)
        }
    }
}

/// 模型输出 → 词汇：去空白、剥掉成对的引号/反引号；为空则报错。
fn parse_word(raw: &str) -> Result<String, String> {
    let cleaned = raw
        .trim()
        .trim_matches(|c| {
            matches!(
                c,
                '"' | '\'' | '`' | '“' | '”' | '‘' | '’' | '「' | '」' | '『' | '』'
            )
        })
        .trim();

    if cleaned.is_empty() {
        Err("模型返回了空词汇".to_string())
    } else {
        Ok(cleaned.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_word() {
        assert_eq!(parse_word("SSA（静态单赋值）").unwrap(), "SSA（静态单赋值）");
    }

    #[test]
    fn strips_whitespace_and_quotes() {
        assert_eq!(parse_word("  \"逃逸分析\"\n").unwrap(), "逃逸分析");
        assert_eq!(parse_word("`写时复制（CoW）`").unwrap(), "写时复制（CoW）");
        assert_eq!(parse_word("「KV 缓存」").unwrap(), "KV 缓存");
    }

    #[test]
    fn rejects_empty_output() {
        assert!(parse_word("").is_err());
        assert!(parse_word("   \n\t").is_err());
        assert!(parse_word("\"\"").is_err());
    }
}
