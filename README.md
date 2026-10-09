# termkata（抽讲）

> 抽一个术语，限时讲清楚。

本地优先、跨平台的个人训练工具。练习循环：**抽词 → 限时准备 → 脱稿口播 → 回看录像 → 按结构评分卡自评 → 沉淀记录**。

- 当前状态：核心（`crates/core`）、词汇生成端口（`crates/wordgen`）、CLI 宿主（`crates/cli`）已可用；UI 尚未开始，Tauri / Svelte 壳将在 UI 迭代时引入。
- 项目约定（目标、路线图、开发范式、Agent 契约）：见 [`AGENTS.md`](./AGENTS.md)。

## 开发环境

由仓库内的 Nix flake 管理（Rust / jj）：

```bash
nix develop
```

前置：已安装 [Nix](https://nixos.org/download/)（flakes 已启用）。`flake.lock` 已提交，环境可复现。

## 试用（CLI）

```bash
cargo run -p termkata-cli
# draw 抽取 → speak 开始口播 → end 结束 → q 退出
```

## 常用命令

```bash
cargo run -p termkata-cli    # 运行 CLI（终端练习管道）
cargo test --workspace       # 全部单测
cargo check --workspace      # 编译检查
```

版本控制使用 [Jujutsu](https://www.jj-vcs.dev/)（colocated，Git 后端）；常用命令见 `AGENTS.md` §9。

## 许可证

[GPL-3.0](./LICENSE)
