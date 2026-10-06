# termkata（抽讲）

> 抽一个术语，限时讲清楚。

本地优先、跨平台的个人训练工具。练习循环：**抽词 → 限时准备 → 脱稿口播 → 回看录像 → 按结构评分卡自评 → 沉淀记录**。

- 当前状态：v0.1.0 开发前，仅有项目脚手架。
- 项目约定（目标、路线图、开发范式、Agent 契约）：见 [`AGENTS.md`](./AGENTS.md)。

## 开发环境

推荐使用 Nix flake（一次获得 Node 24 / pnpm / Rust / jj / WebKitGTK 等全部依赖）：

```bash
nix develop
```

`flake.lock` 已提交，环境可复现；`nix flake update` 升级。

<details>
<summary>不使用 Nix 时的准备（按操作系统）</summary>

- Node.js ≥ 24 + pnpm
- Rust stable（[rustup](https://rustup.rs)）
- 系统构建依赖（Linux 的 WebKitGTK 等，Windows / macOS 各有要求）：见 [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

</details>

## 常用命令

在 `nix develop` 内执行：

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
