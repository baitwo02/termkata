# 更新日志

本项目遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 与 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 新增

- 一轮练习状态机核心（`crates/core`，Rust）：4 个阶段 / 事件 / `transition` 纯转移函数；7 个功能用例（cargo test）
- 词汇生成适配器（`crates/wordgen`，Rust）：`WordSource` 端口 + `FakeApi`（固定词表轮换）；3 个单测
- CLI 宿主（`crates/cli`）：分层（command / render）驱动完整管道（draw / speak / end），当前使用 FakeApi；10 个测试（单元 + 端到端）
- 共享宿主编排（`crates/host`）：`App`（begin_draw / finish_draw / start_speaking / end_speaking）与极简执行器，CLI 与 Tauri 共用
- core 可选 `serde` feature：`Phase` / `RoundState` 序列化；wire 格式由测试锁定
- 桌面 UI（Tauri 2 + Svelte 5）：IPC 命令、平台适配层、`$state` 镜像、按阶段渲染（中文、跟随系统深浅色）
- 开发环境：Nix flake（Rust / rust-analyzer / jj）
- 版本控制：Jujutsu（colocated Git 后端）
- 许可证：GPL-3.0-only
- CI：三平台 `cargo test` / `cargo check`

### 变更

- 词汇来源改为 LLM API 按需生成（BYOK），固定词库方案废弃（ADR 0003）
- 核心实现语言改为 Rust（ADR 0004）
- 仓库当前只保留 core 与文档；Tauri / Svelte 壳将在 UI 迭代时引入
