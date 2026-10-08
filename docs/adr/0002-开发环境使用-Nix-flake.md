# ADR 0002：开发环境使用 Nix flake

- 状态：已接受
- 日期：2026-10-07
- 决策者：用户 + AI agent

## 背景

termkata 的构建依赖横跨三层：

1. 语言运行时与工具链（Node.js、Rust）；
2. 语言包依赖（npm / cargo）；
3. **操作系统原生库**（Linux 下 Tauri 需要 WebKitGTK / GTK / libsoup 等）。

rustup、pnpm、cargo 只能覆盖前两层；第三层通常要求开发者按发行版手动安装（sudo dnf / apt / pacman…），环境不可复现，换机器成本高。

## 决定

开发环境统一由仓库内的 `flake.nix` 管理：

- 提供 Node 24、pnpm、Rust（rust-overlay）、jj、WebKitGTK 及全部系统原生库；
- **唯一受支持的开发路径是 `nix develop`**；README / AGENTS.md 不维护按发行版安装的说明；
- `flake.lock` 提交入库，保证环境可复现。

## 理由与替代方案

- rustup + 发行版包管理器：能工作，但不可复现、需要 sudo、跨发行版差异大 → 弃。
- 纯 rustup + corepack + 手动系统依赖：无法解决第三层 → 弃。
- Docker / Devcontainer：同样可复现，但图形应用（Tauri 窗口）开发体验差 → 弃。

## 后果

- 开发者需要安装 Nix（flakes 已启用）；Nix 支持 Linux 与 macOS，Windows 开发需经由 WSL。
- CI 仍使用 GitHub 托管 runner（Ubuntu）+ apt 安装依赖，与开发环境相互独立。
- 升级环境：`nix flake update`。
