# 更新日志

本项目遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 与 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 变更

- 词汇来源改为 LLM API 按需生成（BYOK），固定词库方案废弃（ADR 0003）

### 新增

- 项目脚手架：Tauri 2 + SvelteKit（SPA 模式）+ TypeScript + Vite
- 开发环境：Nix flake（Node 24 / pnpm / Rust / jj / WebKitGTK），替代手动安装系统依赖
- 版本控制：Jujutsu（colocated Git 后端）
- 许可证：GPL-3.0-only
- CI/CD：GitHub Actions 三平台检查；tag 触发安装包构建
