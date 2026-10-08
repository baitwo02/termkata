# 更新日志

本项目遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 与 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 新增

- 一轮练习状态机核心（`crates/core`，Rust）：4 个阶段 / 事件 / `transition` 纯转移函数；7 个功能用例（cargo test）
- 开发环境：Nix flake（Rust / jj）
- 版本控制：Jujutsu（colocated Git 后端）
- 许可证：GPL-3.0-only
- CI：三平台 `cargo test` / `cargo check`

### 变更

- 词汇来源改为 LLM API 按需生成（BYOK），固定词库方案废弃（ADR 0003）
- 核心实现语言改为 Rust（ADR 0004）
- 仓库当前只保留 core 与文档；Tauri / Svelte 壳将在 UI 迭代时引入
