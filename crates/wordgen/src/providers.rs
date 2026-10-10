//! 厂商（provider）定义：端点是哪、密钥从哪来、默认模型是什么。
//!
//! 厂商与协议**分离**：厂商是数据（本文件），协议是代码（如 [`crate::openai_chat`]）。
//! 新增厂商 = 新增一条定义（必要时再加一份协议实现），不改生成逻辑与状态机。

/// 一个 LLM 厂商的定义。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderSpec {
    /// 稳定标识（未来持久化 / 配置引用用）。
    pub id: &'static str,
    /// 界面显示名。
    pub display_name: &'static str,
    /// API 根地址（具体路径由协议实现拼接）。
    pub base_url: &'static str,
    /// API key 所在的环境变量名。
    pub api_key_env: &'static str,
    /// 默认模型。
    pub default_model: &'static str,
}

/// DeepSeek（协议：OpenAI 兼容的 chat completions）。
///
/// 模型名经官方 `/models` 确认（2026-10）：`deepseek-flash`（默认）与 `deepseek-v4-pro`；
/// 旧的 `deepseek-chat` 已不在列表中。
pub const DEEPSEEK: ProviderSpec = ProviderSpec {
    id: "deepseek",
    display_name: "DeepSeek",
    base_url: "https://api.deepseek.com",
    api_key_env: "DEEPSEEK_API_KEY",
    default_model: "deepseek-flash",
};

#[cfg(test)]
mod tests {
    use super::*;

    /// 定义里的对外契约：环境变量名与模型名不应被无意改动。
    #[test]
    fn deepseek_contract() {
        assert_eq!(DEEPSEEK.api_key_env, "DEEPSEEK_API_KEY");
        assert_eq!(DEEPSEEK.default_model, "deepseek-flash");
    }
}
