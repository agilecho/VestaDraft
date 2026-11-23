# ArtCraft 工程检查与 CI 任务卡

2026-11-22 · 用户已批准执行 · 待独立施工会话接手

## 给你看的

把目前靠人工分开运行的工程检查收成一个入口。开发者在本地和 GitHub CI 运行同一套检查，能看到哪一项通过、哪一项失败、哪一项缺输入。改动引入错误的模块依赖、破坏已锁定的合同、漏跑测试或使共享内核无法通过 WASM 类型检查时，CI 应明确指出原因。

这是一张独立工程卡。P2-03 继续负责原生内核、保存与重开，以及 C8、G29、G30 的真实验收。P2-04 继续负责网页编辑、Worker、完整双端制品及同版验证。工程卡设置的日常检查提供额外证据，两张原卡的功能、判据、准入和额度按原卡办理。

## 给执行者看的

### 1. 权威依据、参考和实际基线

- 权威顺序见 [后续施工共同约定](后续施工共同约定.md#1-独立接手与权威材料)。落实 [VESTA 重写要求的 ArtCraft 目录决定](../VESTA全功能重写要求.md#artcraft-产品目录一致性2026-11-22-用户确认) 与 [制作方法](../VESTA全功能重写要求.md#3-必须遵循的方法和步骤)。DraftDuet 提交 `2495d4666603838c438b8a3074b60de9f5d3b25e` 的 [PRD §10.10](/Users/agilecho/code/DraftDuet/docs/PRD.md:971) 与 [技术方案 §9.6](/Users/agilecho/code/DraftDuet/docs/技术方案.md:851) 确定平台实证边界。页面内实际编辑、实时画面与保存结果沿原卡取证。
- 读取 [目录施工登记](ArtCraft目录对齐.md#施工登记)、[阶段 2 当前状态](阶段2-首条完整链/README.md#p2-03-当前状态)、[P2-03](阶段2-首条完整链/03-晶体命令内核与原生保存基础.md) 和 [P2-04](阶段2-首条完整链/04-当前Web编辑与锁定双端交付.md)。发卡时本地 `main` 与当前目录对齐工作树均为 `29df44fa4ffa981c6c52b8a674c9ea07eea70a6b`。已批准卡面位于发卡工作树。施工会话从接手时的实际基线开独立工作树，先读取本卡与最新阶段状态。
- 现有根 workspace 有五个产品库、原生 CLI 和 `xtask`，Web 目录有三份生成合同。根目录尚无 `.github/workflows/`，`xtask` 只有 `contracts-check`。该入口的直接子命令已有独立验证，合入时完整 Node 合同检查 25/25 通过。已验证的 TS 类型检查目前也走独立命令。记录当前 HEAD、index、工作区和锁再开工。 [现行构建说明](../开发与构建.md) · [合同原始日志](/Users/agilecho/code/VestaDraft/output/vesta-artcraft-layout/20261122-main-ff/contracts-final.stdout)
- 根 Cargo.lock 的基线 SHA-256 为 `2bbd8b7b4a8df85f3457a00184188a9677e713db03eb3bc73105b23083c1d9fc`，npm 锁为 `48990eb785197a35f367e38382a4a8577aad11582d170774ffdd70fa5627e719`。目录迁移时的离线解析已覆盖 macOS arm64，全平台图曾因缺 `android_system_properties v0.1.6` 失败。CI runner 须先核实际 host/target 工具、闭合缓存及锁，旧本机证据按其平台使用。

| 参照提交 | 必读提交对象 | 本卡采用的共同做法 |
|---|---|---|
| WordCraft `685869239f31c890c6365f264c14b29ef48fcb7f` | `AGENTS.md`、`docs/architecture.md`、`xtask/src/main.rs`、`.github/workflows/ci.yml` | 根 `cargo xtask ci` 集中调度 fmt、Clippy、测试、分层和 WASM，CI 调公共 workflow |
| Craft Anything `44c89badd69c76eca7fd79315263a9ae80b5770f` | `AGENTS.md`、`xtask/src/main.rs`、`.github/workflows/ci.yml` | 同一个入口和共享 workflow，分层检查按实际依赖图，WASM 选目标包 |
| 两者的公共 CI | `storytold/craft-common-actions/.github/workflows/gate.yml` | 调用层保持轻薄。公开接口提供根目录命令、runner、额外 setup 与缓存输入。施工固定实际可复核的提交并核对 setup 的行为。 [共享 workflow 接口](https://github.com/storytold/craft-common-actions/blob/main/.github/workflows/gate.yml) |

参考源位于 `/Volumes/T9/Development/Sources/Artcraft`。开工保存实际读取对象的 commit/blob、工作树差异和选择理由。Craft Anything 的公共 CI 参考提交与 VestaDraft 已锁定的 CA 安装、worker、Check 版本分别记录。

### 2. 工程结果与原卡边界

交付从仓库根可运行的 `cargo xtask ci`，以及调用这一入口的根 GitHub workflow。覆盖仓库当前适用的工程验证，普通 CI 的通过边界以明确的包、目标、测试清单和输入为准。

| 检查 | 本卡验收目标 |
|---|---|
| 分层 `cargo xtask layers` | 读实际 Cargo 元数据，覆盖直接与传递的本地依赖、目标条件和依赖种类。规则覆盖生成绑定与派生宏、领域模型、引擎命令、格式库、CLI 和工具。低层引入应用或 UI 依赖、新成员未归层时失败。现有 codecs 对 commands 的测试依赖按 dev 单列核实，不能凭这一项豁免生产边。 |
| 格式、lint 和 Rust 测试 | 参考 `cargo fmt --all -- --check`、`cargo clippy --offline --locked -p <实际包> --all-targets -- -D warnings` 和 `cargo test --offline --locked -p <实际包>`。开工根据已获准候选与可用夹具确定包和目标，保留实际清单并执行非空用例。生成的合同与 Source7 文件按原生成器检查精确字节。发现既有格式或 lint 问题时，记录受影响目标和原始诊断。完整关口保留真实失败，产品整改按其所属卡处理。 |
| 合同与类型 | 复用 `contracts-check`、完整 Node 合同套件、Source7 `--check` 和锁定 TS `--noEmit`。核九份 CPJ、I33/NS2 合同、生成文件与两份锁的前后字节。完整交付的 NS2 关口继续在各卡指定版本和入口执行。合同生成发生在隔离检出，核对生成结果与执行前的 Git 内容。 |
| WASM `cargo xtask wasm` | 对适合浏览器共享的现有 Rust 库运行 `cargo check --offline --locked --target wasm32-unknown-unknown`，至少锁定绑定、模型及命令内核的适用包与 feature。检查预设的必需包实际进入目标解析。先核对应目标与 host 构建输入。CLI 专属包、正式 Worker、wasm-bindgen、Vite、真实 Web/WASM 制品和浏览器往返继续按原卡完成。 |
| CI 接线 | 根 `.github/workflows/ci.yml` 用只读 GitHub 权限，优先复用固定提交的公共 workflow，从根调用同一 `xtask` 入口。锁定 Node/npm/TS、Python、Rust、WASM 目标和辅助工具。首次选一个确有闭合输入的 runner。普通 PR 与 main push 的启用以首次真实复跑及触发授权为准，阶段性验证明确标记范围。 |

P2-03 的 C8/G29/G30 及 7 项真实资格仍按原卡办理，已批 3,000 秒不计入本卡。P2-04 的 U00 双端构建、浏览器和 Linux CLI、同版制品清单及页面验证由原卡负责。正式 CA 安装、worker/Check、发布、部署和分支保护设置继续沿各自授权。保留产品源码、外部依赖身份、合同和封存样本。两份参考各自的字体、i18n、parity、插件和 release 专项不列入本次共同关口。

### 3. 实施顺序、启动条件与验收

1. **核实际可运行范围。** 记录应用和库的正常、构建、测试依赖，锁定 crate/target/feature、具名测试、编译期和运行期夹具。本机 `output/` 存在 GSAS 的编译期 `include_bytes!` 以及 Source7、Field 等运行期夹具。禁止把整个 `output/` 当 CI 输入。逐项核来源、SHA 与适用授权，使用可交付的已审输入。区分仅在执行时读取的夹具和编译期 `include_bytes!`，后者即使过滤测试名也可能被编译。未齐的命令记为阻塞并停在该项，保留可独立验证的分层与合同结果。
2. **实现并验证入口。** 主要施工在 `xtask/`、`.github/workflows/ci.yml`、必要的 CI 准备脚本及 [开发与构建说明](../开发与构建.md)。继续使用根 alias 及产品锁。尽量复用现有脚本与测试，不生成第二套业务断言。若格式检查影响冻结生成物、Rust 目标包含尚未准入的 C8，或启用 WASM 容易触及 P2-04 接线，先停止该项并交出实际差异和剩余工作。
3. **实跑正例与反例。** 同一候选用本地与 CI 的实际入口运行，核命令、测试计数、检查范围、结论和退出码。在自有临时副本分别加入一条违规依赖和一项可观测的检查失败，验证分层门禁与 CI 子命令失败会传到最终结论。另验证缺工具、缺夹具或无测试选中会明确报错或阻塞。阶段性运行有自己的结论，全量关口只有要求的检查实跑通过才记通过。
4. **收证和交回。** 保存 before/after/diff、版本及 lock SHA、Cargo 依赖图、测试清单、真实 argv/cwd、stdout/stderr/exit、CI workflow SHA、runner/运行 ID、耗时和失败原件。记录各项是否执行及证明范围。交 [共同约定](后续施工共同约定.md#5-每卡必交验收与证据) 所需双段回执及 `DSH-REVIEW-HANDOFF.md`，由用户手动转交。先交回本卡，不顺带启动原阶段卡。

### 4. 已批准的执行上限与停报

用户回复“批准执行”。按卡面单独记账，本地验证最多 2 组，各 ≤600 秒，远端 CI 最多 2 次，各 ≤1,200 秒，合计 ≤3,600 秒。同一时刻只运行一组，Cargo jobs ≤2。首次远端验证采用用户同意的验证分支或草稿 PR，并先核实该触发方式可用、runner 及费用。依赖获取、工具安装及 CI 对外动作由开工授权列明，测试阶段使用已核的锁定输入。现有工件可复用须记录精确版本与允许范围。任一额度到顶、锁或合同身份漂移、夹具不可交付、前卡未准入或真实 CI 输入未闭合时，停对应步骤，保留失败日志和能独立验收的部分。本卡已批准，施工与计量尚未开始。施工者须先接收独立会话派发并核对实际基线。
