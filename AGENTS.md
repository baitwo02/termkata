# AGENTS.md — termkata 工作宪法

> 本文件是本仓库的最高约定，面向所有在此工作的 AI agent 与人类协作者。
> 修改需用户明确批准；与本文冲突的建议、习惯、旧文档，一律以本文为准。

---

## 1. 项目概述

**termkata（抽讲）**：本地优先、跨平台的桌面训练工具。

它支撑的练习循环：

> **抽一个术语 → 限时准备（收集资料 + 关键词提纲）→ 脱稿口播 → 回看录音/视频 → 按结构评分卡自评 → 沉淀记录**

- **唯一用户与维护者**：仓库作者本人（下称"用户"）。后续可能开源（GPL-3.0），但开源是结果，不是设计驱动力。
- **当前阶段**：v0 之前，尚无应用代码。策略是"**系统先行**"——先产出可用系统，用户再开始练习；此后系统随真实摩擦持续小步升级。
- **从零开始**：不迁移原型期资产（旧 `words.json`、`templates/`、`draw.mjs` 已删除）；一切以本文件定义的规格为准。

---

## 2. 目标与非目标

### 目标

- 尽快到"可用"：**抽卡 + 计时器**完成即可开始练习。
- 词库可持续演进（远期：结合 GitHub 热门项目更新）。
- 数据全部本地、文件化、可 git 版本化、无锁定。

### 非目标（明确不做）

- 多用户、账号、后端服务、云同步。
- 移动端 App（Web 版可选，见 §6 架构约束）。
- app 内录制音视频——用户用 OBS / 手机录制，app 负责**导入或引用**。
- 视频剪辑、发布平台集成、社交/涨粉。
- 打卡、连续天数、强制周计划等"管住用户"的功能：工具给能力，纪律靠自觉。

---

## 3. 核心设计原则

1. **功能解耦**：抽卡 / 计时 / 媒体 / 评分 四功能彼此独立可用；记录与评分**可选**关联某次抽卡，不强制。
2. **词库与抽卡是核心资产**：词库结构、更新管线、抽词算法的质量要求最高。
3. **本地优先**：workspace 内的文件是唯一事实来源。
4. **纯 Web UI + 平台适配层**：UI 不直接调用 Tauri API，平台能力经 `lib/platform` 接口注入，为未来 Web 版留门。
5. **最小可验收**：一次迭代只交付一个最小可验收功能（§7）。
6. **简单优先**：能不加依赖就不加；能删的代码就删。
7. **中文界面**，深浅色跟随系统。

---

## 4. 功能路线图

顺序由用户确定：抽卡 → 计时器 → 媒体 → 评分卡；各功能解耦。

| 版本 | 功能 | 内容 |
|---|---|---|
| v0.1 | **抽卡**（核心） | 随机展示 + 重抽 → 过滤（领域/难度/反直觉）→ 不重复历史 |
| v0.2 | 考试计时器 | 20 min 收集 + 5 min 口播协议；支持自定义阶段 |
| v0.3 | 媒体关联 | 导入（复制进 workspace）+ 引用（路径/URL）；app 内可查看/打开 |
| v0.4 | 评分卡 | 8 分制结构自评 + 备注，可关联抽词/媒体 |
| 待定 | **词库更新**（核心） | 结合 GitHub 热门项目，机制未定（§10）；数据结构先按"可插拔词源"设计 |
| 远期 | 智能建议 | 基于音视频/记录的分析建议（形式与成本待定） |

> 评分卡定义（四项各 0-2 分，满分 8）：30 秒内点明「它解决什么矛盾」/ 主要矛盾出现在前 1/3 / 支撑点 2-3 个且不重叠 / 给出代价或边界。

**v0.1 细分迭代：**

- `v0.1.0`：打开 app，随机展示一个词及基础元信息，可重抽。
- `v0.1.1`：按领域 / 难度 / 反直觉过滤。
- `v0.1.2`：抽题历史与不重复（抽完自动重置）。

每个迭代的实际验收清单，在该迭代开始时由 agent 提出、用户确认（§7）。

---

## 5. 数据模型（workspace）

用户数据**不放在仓库内**。默认 workspace 为 `~/Documents/termkata/`（可配置）：

```
<workspace>/
├── words.json      # 用户词库（初始来源待定，见 §10）
├── history.json    # 抽词历史
├── sessions/       # 每次训练一份记录（Markdown）
└── media/          # 导入的音视频文件（引用式媒体只存外部路径）
```

`words.json` 结构（v1，向后兼容；扩展字段需用户批准）：

```json
{
  "version": 1,
  "fields": { "compilers": "编译原理", "os": "操作系统", "...": "..." },
  "terms": [
    {
      "id": "ssa",
      "term": "SSA（静态单赋值）",
      "alias": "Static Single Assignment",
      "field": "compilers",
      "difficulty": 3,
      "math": 3,
      "counterintuitive": true,
      "hook": "钩子种子",
      "stake": "为什么重要",
      "source": "https://...（可选，新增字段）"
    }
  ]
}
```

**抽词规则**：

- 过滤：`field` / `difficulty` 范围 / `math` 上限 / `counterintuitive`。
- 不重复：按 `id` 记入 `history.json`；范围内的词抽完后自动重置并提示。
- 随机源、时间、文件路径必须可注入，保证测试确定性。

`sessions/` 与 `media/` 的具体格式在 v0.3 / v0.4 开始时定稿。原则：Markdown 主文件 + 资源路径，人可读、git 友好。

---

## 6. 技术架构

| 项 | 决策 |
|---|---|
| 桌面壳 | Tauri 2 |
| 前端 | Svelte 5（SvelteKit SPA） + TypeScript + Vite |
| 包管理 | pnpm |
| 版本控制 | Jujutsu (jj) + colocated Git 后端（GitHub 托管与 CI 不受影响） |
| 开发环境 | Nix flake（`nix develop`：Node 24 / pnpm / rust-bin / jj / WebKitGTK） |
| 测试 | Vitest（core 纯逻辑）；UI 用手动走查清单 |
| 分发 | GitHub Actions 矩阵构建 Linux / Windows / macOS；打 tag 触发 Release |
| 许可证 | **GPL-3.0**；依赖必须 GPL 兼容 |

**目标目录结构：**

```
termkata/
├── AGENTS.md
├── README.md
├── LICENSE
├── package.json
├── src/                    # 前端
│   ├── lib/core/           # 纯 TS 领域逻辑：抽词、词库解析/合并、计时、评分
│   ├── lib/platform/       # 平台适配层：接口定义 + tauri 实现（未来 + web 实现）
│   ├── lib/ui/             # Svelte 组件
│   └── main.ts
├── src-tauri/              # Rust 壳
└── docs/
    ├── adr/                # 架构决策记录
    └── backlog.md          # 各迭代暂不处理的想法/问题
```

**跨平台规则：**

- 优先使用 Tauri 的跨平台抽象；平台差异（默认 workspace 路径、用系统程序打开文件等）集中在 `src-tauri` 或 platform 层，用 `#[cfg(...)]` 或运行时判断。
- `#[cfg]` 不得散落到业务逻辑。
- CI 必须覆盖三平台构建。

---

## 7. 开发范式：MVP 迭代协议

**一次迭代 = 一个最小可验收功能。** 不允许夹带其他改动。

每个迭代严格按以下流程：

1. **提案（agent）**——动代码前向用户复述：
   - 本轮唯一功能是什么；
   - 验收标准（可手动执行的分步清单）；
   - 明确不做的事。
   - 用户确认后才开工。
2. **实现**——core 纯逻辑先行并配 Vitest 单测；UI 保持薄。
3. **自检**——`pnpm lint`、`pnpm test`、`pnpm build`、`cargo check` 全部通过。
4. **交付说明（agent）**——改了什么 / 手动验收步骤 / 已知缺口 / 建议的下一步。
5. **验收（用户）**——按清单手动走查；通过后打 tag（SemVer，0.x）。
6. **记录**——更新 `CHANGELOG.md`（Keep a Changelog 风格，中文）；关键决策写入 `docs/adr/NNNN-标题.md`。

### Definition of Done（每个功能必须全部满足）

- [ ] 验收清单手动走查通过
- [ ] 新增 core 逻辑有单测且全绿
- [ ] lint / typecheck / build 通过
- [ ] 无未批准的新依赖
- [ ] 数据格式变更经过批准且附迁移说明
- [ ] CHANGELOG 已更新
- [ ] （练习开始后追加）该功能被真实使用 ≥ 1 次

### 版本控制约定（Jujutsu / Git 后端）

- 本仓库使用 **Jujutsu（jj）**，colocated 模式（`.jj` 与 `.git` 并存）；GitHub 托管、Actions、PR 均照旧。
- trunk-based：`main` 书签（bookmark）为主线，小步 change，不搞长期分支。
- Conventional Commits 作为变更描述格式：`feat(draw): 完成随机抽卡界面`——类型与 scope 用英文，描述用中文。
- 每个迭代结束打 tag：`jj tag set v0.1.0` + `jj git push --tag`。
- 不提交 workspace 用户数据。

### 测试策略

- 单测覆盖：抽词（过滤/不重复/重置/随机注入）、词库解析与合并、计时状态机、评分计算。
- 不写 UI 端到端自动化（成本 > 收益）；UI 用固定验收清单。
- "词库更新"功能立项时，必须先用单测锁定去重与格式规则。

### CI/CD

- push / PR：lint + typecheck + test + 前端 build + `cargo check`（三平台）。
- tag：用 `tauri-apps/tauri-action` 产出三平台安装包并创建 draft release。
- 暂不做代码签名。

---

## 8. Agent 工作契约（硬性规则）

1. **不确定就问**，禁止猜测需求后开写。
2. **一次只做一件事**：不夹带重构、美化、依赖升级。
3. **不擅自改数据格式、不加依赖**；要改先提案，说明理由与替代方案。
4. **发现问题不顺手修**：记入 `docs/backlog.md` 或在交付说明中指出。
5. **验收标准先行**：没有经用户确认的验收清单，不进入实现。
6. **交付三件套**：改了什么 / 怎么手动验收 / 已知缺口。
7. **保持本文件最新**：架构、流程、数据格式的决策变更，先更新 AGENTS.md 再继续。
8. **测试确定性**：随机、时间、文件路径可注入，测试可复现。
9. **UI 文案中文**，代码标识符英文。
10. **范围守门**：用户提出新想法时，先判断属于当前迭代还是 backlog，不自动扩权。
11. **版本控制纪律**：只用 `jj`（或 colocated 下的 `git`）操作仓库，禁止手改 `.git`/`.jj` 内部文件；出错先 `jj undo`，不要手删。

---

## 9. 常用命令

> 脚手架落地后此节更新为实际命令。

```bash
pnpm install
pnpm tauri dev      # 开发运行
pnpm test           # Vitest
pnpm lint           # lint + svelte-check
pnpm tauri build    # 本地打包
```

```bash
# 版本控制（Jujutsu）
jj status / jj diff            # 当前改动
jj describe -m "..."           # 修改当前 change 描述
jj new                         # 开始下一个 change
jj log                         # 历史
jj undo / jj op log            # 撤销 / 操作日志
jj bookmark set main           # 推进 main 书签
jj git push --bookmark main    # 推送
```

环境依赖：推荐 `nix develop`（flake 已提交，含 Node 24 / pnpm / Rust / jj / WebKitGTK）；不使用 Nix 时按各自平台安装 rustup 与系统依赖（见 README / Tauri 官方文档）。

---

## 10. 已知待定项（不阻塞启动）

| 项 | 现状 | 决策时机 |
|---|---|---|
| 初始词库来源 | 已定：全新手写 12 词种子（含领域/难度等元信息），随 v0.1.0 落地 | 已解决 |
| 词库更新机制 | 倾向：抓取 GitHub Trending / awesome 列表 → 生成候选 → 人工审核入库；数据结构先按可插拔词源设计 | v0.1 之后单独提案 |
| 媒体细节 | 支持导入复制 + 外部引用两种；播放与格式细节未定 | v0.3 开始时 |
| 智能建议 | 形式（本地模型 / API）、成本、隐私未定 | 评分卡稳定后 |
| Web 版 | 架构预留 platform 适配层 | 有需要时 |
| 国际化 | 目前仅中文 | 有需要时 |

---

## 11. 当前状态

- 脚手架已落地：Tauri 2 + SvelteKit（SPA）+ TypeScript + jj（colocated）+ GPL-3.0 + 三平台 CI；初始提交位于 `main` 书签，工作区已开始新的 change。
- 下一步：确认 v0.1.0 迭代提案（抽卡：随机展示 + 重抽）→ 实现 → 手动验收。
