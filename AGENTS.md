# AGENTS

本仓库是开源心理测试 **mind-pulse** 的服务端：用 Rust 编写的只读量表内容 API，外加一个简单的匿名统计模块。

## 1. 项目速览

- 技术栈：Rust 2024（rust-version 1.99）、Salvo 1、sqlx + SQLite、tokio。
- 产物：单文件 **server** 可执行程序，默认监听 **127.0.0.1:4819**。
- 量表数据全部硬编码在 **src/scale/data/**，不存数据库（原因见 README「常见问题」）。
- 统计模块 **src/statistics/** 只记录匿名计数，不存可识别个人信息。

主要接口：

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | /version | 版本号 |
| GET | /list | 量表概览列表 |
| GET | /scales | 量表概览列表（兼容） |
| GET | /scales/{id} | 单个量表的完整数据 |
| GET | /statistics | 记录一次匿名统计 |
| GET | /get_statistics | 查询统计 |

## 2. 目录与职责

    src/main.rs          启动入口
    src/server.rs        Salvo 服务装配
    src/config.rs        端口与 MIND_PULSE_DB_PATH
    src/db.rs            SQLite 连接池
    src/error.rs         统一错误与状态码
    src/routes/          HTTP 路由
    src/statistics/      匿名统计
    src/scale/           量表领域
      definition.rs      Scale / Tag
      question.rs        Question / QuestionOption
      interpretation.rs  解释区间与状态
      summary.rs         列表页概览
      data/              各量表数据（一份量表一个模块）

## 3. 本地开发与验证

运行：

    cargo run
    cargo run -- 9999
    MIND_PULSE_DB_PATH=./data/mind_pulse.db cargo run

数据库目录必须已存在，程序不会自动创建目录。

任何 Rust 代码的变动都必须经过下面的检查才能提交：

    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings
    cargo test

如果改动涉及 API 字段（新增、删除、重命名），还必须同步前端仓库 **/home/thepoy/confidant/www**，并运行：

    bun run check:types
    bun run test

## 4. 领域规则（最重要）

### 4.1 后端只提供数据，不承载计算规则

后端只下发题目、选项、分值、常模、解释文本与区间；**怎么算**由前端负责。已移除的规则型字段：

- 所有量表的 **formula_mode**（SDS/SAS 的 ×1.25 换算）
- NEO 的 **scoring_rule**（T 分分档）
- 16PF 的 **second_personality_factor[].expression**（X1–X4 公式）
- SCL-90 的 **interpretation.positive**（160 / 43 / ≥2 阈值）
- Y-BOCS 的 **interpretation[].range.any**（0–20 分量表分档）

新增量表时不要把这些规则再加回后端；确有必要时先与维护者讨论。前端承接位置见前端仓库 **docs/scale-rules-migration.md**。

### 4.2 解释区间

- 统一使用**闭区间** [start, end]。
- 相邻档位必须连续、不重叠：next.start = prev.end + 1。
- 最高档必须覆盖该量表的满分，不得留空洞。
- 修改区间必须同步测试（参考各量表文件里的 interpretation_ranges_are_contiguous_closed_intervals）。

### 4.3 量表数据约定

- 一份量表一个模块，在 **src/scale/data/mod.rs** 注册；模块名用全称，缩写用官方命名。
- 数据使用编译期常量（&'static [T]），不要改成运行时 JSON 反序列化。
- **QuestionOption.point** 是唯一计分依据；不要在前端重复硬编码分值。
- 16PF 聪慧性（B 因子）为 0/1 计分，其余因子 0/1/2；改动计分键前先核对题面，并跑 **per_factor_max_matches_published_norms**。
- 16PF 存在两套中文译本，题号与题面不同；不要只看题号就改计分键。
- BDI-II 为 21 项、0–3 分、总分 0–63；第 16、18 题各有 7 个选项（0/1a/1b/2a/2b/3a/3b），分值 0/1/1/2/2/3/3。
- **tags** 用于列表页小标签（info/normal/warning/error），长文案放 **warning**。例如 BDI-II 的 tags.warning = ["13+"]，warning 里写完整版权与年龄说明。
- 自杀/危机条目由前端 **criticalItems** 按题号触发；调整题号必须同步前端。

### 4.4 新增量表清单

1. 在 **src/scale/data/** 下新建模块，定义 **Scale**（id、名称、分类、instruction、questions、interpretation、introduction、references、warning、tags）。
2. 在 **src/scale/data/mod.rs** 添加 **mod** 与 **pub use**。
3. 在 **src/scale/catalog.rs** 的 **register_scales!** 中注册；id 必须唯一。
4. 题目、选项与分值；解释区间用闭区间；需要标准分的量表补常模。
5. 至少补一条区间连续性测试。
6. 如前端需要新路由/新结果视图，同步前端仓库。

### 4.5 API 契约与前端同步

- 新增、删除、重命名字段都属于契约变更，必须同步前端类型与调用方。
- 错误统一用 **crate::error::Result**；新增领域错误加到 **Error** 并映射合适的状态码。
- 删除字段前先确认前端已不再读取（前端仓库 grep + 类型检查）。
- 前端有独立的仓库与提交历史，不要在本仓库提交前端代码。

## 5. 编码规范

- 所有代码、模块严格遵循 SRP；按职责拆分，不要堆在一个文件里。
- 注释解释「为什么」，不复述「是什么」。
- 不要用 unwrap() 处理用户输入、环境变量或 IO；测试里可以。
- 不保留未使用代码；确需保留的死代码要写明原因。
- 不修改与本次变更无关的文件；格式化只应影响本次改动，避免顺带改动大量存量代码。

## 6. 测试

- cargo test 必须全绿。
- 修 bug 先补回归测试。
- 新增或修改量表时至少覆盖：
  - 解释区间连续、闭区间、无空洞；
  - 各因子原始分上限与常模表一致（16PF 的 per_factor_max_matches_published_norms 是范例）。

## 7. CI 与发布

- **pr-check.yml**：PR 到 main、push 到 main 时跑 fmt / clippy / test；dev 的改动由对应 PR 的 pull_request 事件覆盖，不要再把 dev 加进 push，否则同一提交会跑两次。
- **publish.yml**：手动触发，选择 bump_type；UPX 压缩可选，默认关闭。
- **automerge-label.yml**：给 PR 打 **automerge** 标签，检查通过后由脚本直接合并；改动 .github/workflows/** 的 PR 需要配置 **AUTOMERGE_TOKEN**。
- **nightly.yml**、**warm-cache.yml**、**cleanup-old-caches.yml**：夜间构建、缓存预热与清理。
- Rust 安装与缓存统一走 **.github/actions/rust-setup**。
- 修改 workflow 后至少本地校验 YAML，改 .cjs 后跑 node --check。

## 8. 分支与提交

- 单一功能的变动必须基于 main 创建分支；一个分支可以包含多个子功能提交。
- 按功能拆分提交：一个提交只承载一项功能性变更；跨区域改动拆成多个提交，并说明每部分承担的具体工作。
- 每次提交后都要询问维护者是推送还是保留在本地，并等待答复；推送、强制推送、更新 PR 都需要明确指示。
- 修改提交或变基前先运行 git log --oneline '@{u}..HEAD'；若目标提交已到远程，追加新提交而不是重写。
- 未经明确指示，绝不重写已推送的提交。

消息格式：

    <type>(<scope>): <subject>

类型：feat、fix、perf、refactor、chore、docs、style、test、ci。中文、祈使语气、小写、句末不加句号；一行主题，必要时加简短正文；不要工具或 Co-authored-by 署名行。

示例：

    fix(bdi): tags.warning 增加 "13+" 年龄标签
    refactor(scale): 移除后端计算规则，只保留基础数据
    ci(pr-check): 避免 push 与 pull_request 重复触发

## 9. 安全与数据

- 不要提交 *.sqlite、日志、token；.gitignore 已覆盖常见项。
- 密钥只放 GitHub Secrets；AUTOMERGE_TOKEN 不写进仓库。
- 统计接口只记录匿名计数，不要新增可识别个人信息。

## 10. 文档

- 行为、API、量表规则变更时，同步更新 README 或 docs/。
- 跨仓库的规则说明写在前端仓库 docs/（如 docs/scale-rules-migration.md）。
