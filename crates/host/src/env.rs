//! `.env` 加载（验证期密钥方案，见 ADR 0005）。
//!
//! 从当前目录向上查找**最近的** `.env`；已存在的环境变量不被覆盖。
//! 解析规则：忽略空行与 `#` 注释；`KEY=VALUE`（按第一个 `=` 分割）；
//! 值可被成对的单/双引号包围；`export ` 前缀会被剥除。
//!
//! 密钥持久化（keyring / 配置文件）留待后续迭代；本模块只是开发/验证期的便利。

use std::path::{Path, PathBuf};

/// 从当前目录向上查找最近的 `.env` 并加载（找不到则什么也不做）。
pub fn load_dotenv() {
    let Ok(dir) = std::env::current_dir() else {
        return;
    };
    let Some(path) = find_dotenv(&dir) else {
        return;
    };
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };

    for (key, value) in parse_dotenv(&content) {
        if std::env::var_os(&key).is_none() {
            std::env::set_var(key, value);
        }
    }
}

/// 向上查找最近的 `.env`（含起始目录自身）。
fn find_dotenv(start: &Path) -> Option<PathBuf> {
    let mut dir = Some(start);
    while let Some(current) = dir {
        let candidate = current.join(".env");
        if candidate.is_file() {
            return Some(candidate);
        }
        dir = current.parent();
    }
    None
}

/// 解析 `.env` 内容（纯函数，便于单测）。
fn parse_dotenv(content: &str) -> Vec<(String, String)> {
    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let line = line.strip_prefix("export ").unwrap_or(line);
            let (key, value) = line.split_once('=')?;
            let key = key.trim();
            if key.is_empty() {
                return None;
            }
            let value = value.trim();
            let value = value
                .strip_prefix('"')
                .and_then(|inner| inner.strip_suffix('"'))
                .or_else(|| {
                    value
                        .strip_prefix('\'')
                        .and_then(|inner| inner.strip_suffix('\''))
                })
                .unwrap_or(value);
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_lines() {
        assert_eq!(
            parse_dotenv("DEEPSEEK_API_KEY=sk-123\n"),
            vec![("DEEPSEEK_API_KEY".to_string(), "sk-123".to_string())]
        );
    }

    #[test]
    fn ignores_comments_blanks_and_invalid_lines() {
        let content = "\
# 注释
   
NO_EQUALS_HERE
=值没有键
KEY=value";
        assert_eq!(
            parse_dotenv(content),
            vec![("KEY".to_string(), "value".to_string())]
        );
    }

    #[test]
    fn strips_quotes_and_export_prefix() {
        let content = "\
export KEY_A=\"值 A\"
KEY_B='值 B'
KEY_C=原样=保留等号";
        assert_eq!(
            parse_dotenv(content),
            vec![
                ("KEY_A".to_string(), "值 A".to_string()),
                ("KEY_B".to_string(), "值 B".to_string()),
                ("KEY_C".to_string(), "原样=保留等号".to_string()),
            ]
        );
    }

    #[test]
    fn trims_spaces_around_key_and_value() {
        assert_eq!(
            parse_dotenv("  KEY  =  value  "),
            vec![("KEY".to_string(), "value".to_string())]
        );
    }
}
