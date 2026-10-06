# termkata（抽讲）

> 抽一个术语，限时讲清楚。

本地优先、跨平台的个人训练工具。练习循环：**抽词 → 限时准备 → 脱稿口播 → 回看录像 → 按结构评分卡自评 → 沉淀记录**。

- 当前状态：v0.1.0 开发前，仅有项目脚手架。
- 项目约定（目标、路线图、开发范式、Agent 契约）：见 [`AGENTS.md`](./AGENTS.md)。

## 环境要求

- Node.js ≥ 24 + pnpm（`corepack enable`）
- Rust stable（[rustup](https://rustup.rs)）
- Linux（Fedora）：

```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel libappindicator-gtk3-devel librsvg2-devel libxdo-devel
sudo dnf group install "c-development"
```

## 常用命令

```bash
pnpm install
pnpm tauri dev      # 开发运行（首次编译 Rust，较慢）
pnpm lint           # svelte-check 类型检查
pnpm build          # 前端构建
pnpm tauri build    # 打包安装包
```

版本控制使用 [Jujutsu](https://www.jj-vcs.dev/)（colocated，Git 后端）；常用命令见 `AGENTS.md` §9。

## 许可证

[GPL-3.0](./LICENSE)
