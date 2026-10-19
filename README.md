# termkata（抽讲）

> 抽一个术语，限时讲清楚。

本地优先、跨平台的个人训练工具。练习循环：**抽词 → 限时准备 → 脱稿口播 → 回看录像 → 按结构评分卡自评 → 沉淀记录**。

- 当前状态：桌面壳（Tauri + Svelte）已接入，抽取/口播流程可点通；词汇来自 `FakeApi`，真实 LLM API 待接入。
- 项目约定（目标、路线图、开发范式、Agent 契约）：见 [`AGENTS.md`](./AGENTS.md)。

## 开发环境

由仓库内的 Nix flake 管理（Rust / rust-analyzer / Node / pnpm / jj / WebKitGTK）：

```bash
nix develop
```

前置：已安装 [Nix](https://nixos.org/download/)（flakes 已启用）。`flake.lock` 已提交，环境可复现。

## 桌面 UI

```bash
pnpm install
pnpm tauri dev      # 开发运行（首次编译 Rust，较慢）
pnpm lint           # svelte-check 类型检查
pnpm build          # 前端构建
```

## CLI（终端宿主）

```bash
cargo run -p termkata-cli
# draw 抽取 → speak 开始口播 → end 结束 → q 退出
```

## 常用命令

```bash
cargo test --workspace   # 全部单测（core / host / wordgen / cli）
cargo check --workspace  # 全部 Rust crate 编译检查
```

版本控制使用 [Jujutsu](https://www.jj-vcs.dev/)（colocated，Git 后端）；常用命令见 `AGENTS.md` §9。

## 许可证

[GPL-3.0](./LICENSE)
