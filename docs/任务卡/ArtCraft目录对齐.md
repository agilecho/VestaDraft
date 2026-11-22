# ArtCraft 目录对齐施工卡

## 给你看的

把 VestaDraft 的实际源码、构建和交付入口搬到 ArtCraft 自研应用共用的目录。验收时，从仓库根能找到 Rust workspace，在 `apps/` 找到 CLI 和 Web 入口，在 `crates/` 找到内核，在 `xtask/` 找到可运行的共用工程任务。目录迁移同时解决工具、生成脚本和交付命令的路径引用。

用户已确定采用 ArtCraft 的共同目录，并在收到本卡后答复“批准”。当前代码仍位于 `app/`，实际迁移及运行验证待独立施工会话接手。现行 React/Three.js 和同一 Rust 内核继续适用。P2-03 的 C8 准入和手动 dsh 复审仍按原卡办理。

## 给执行者看的

### 依据和基线

- 目录要求见 [VESTA 重写要求](../VESTA全功能重写要求.md)，一并读取 [共同约定](后续施工共同约定.md)、[当前阶段状态](阶段2-首条完整链/README.md)、[P1 原目录设计](../superpowers/specs/2026-11-10-vesta-p1-design.md#23-产品位置及目录职责) 和 [CA 构建选择与启动设计](../superpowers/specs/2026-11-10-vesta-ca-wiring.md)。平台边界参考实测 checkout 的 [PRD §10.10](/Users/agilecho/code/DraftDuet/docs/PRD.md:971) 和 [技术方案 §9.6](/Users/agilecho/code/DraftDuet/docs/技术方案.md:851)。
- 本卡准备时 `main` 为 `a4a418695af4c47c16a7b8e3b7da4d2105875aee`，C8 候选源码提交为 `d82fd97`。开工记录实际 HEAD、工作树、锁文件、工具链和当前 P2-03 准入状态，核对差异后接手。P2-03 尚缺 7 项真实验收资格，C8、G29、G30 的已批 3000 秒全部留归原卡。P2-01 源包、P2-02 工具树及 `output/` 下原始证据继续按封存位置读取。
- 结构取自 `/Volumes/T9/Development/Sources/Artcraft` 下七个项目的 **提交版本**。每个提交均有根 `Cargo.toml`、`Cargo.lock`、`apps/`、`crates/`、`xtask/`、`assets/`、`packaging/`、`docs/`，Cargo workspace 均组织为 `crates/*`、`apps/*`、`xtask`。桌面、CLI、Web 放在 `apps/`，各自按需要增加入口。WordCraft 工作树有本地改动，清单以 Git 对象为准。

| ArtCraft 参考 | 实测提交 |
|---|---|
| WordCraft | `685869239f31c890c6365f264c14b29ef48fcb7f` |
| PDFCraft | `4f23ee1c1c2afc6611ca3a71b38162f6259629fb` |
| VectorCraft | `063f55080244009e0e4bf28c24068479089481ac` |
| EffectCraft | `86a4828cb6e98495366307dcea323ae21ee55a15` |
| CADCraft | `ef6df4dffa7823a620260e16f609403f77ebed55` |
| SolidCraft | `82f65a2158b118c9afcee1f7a36514137e1bfc10` |
| PhotoCraft | `877d58844029967b80e36a964902d309de1be158` |

### 文件落位

| 现在 | 迁移后 |
|---|---|
| `app/kernel/Cargo.toml`、`app/kernel/Cargo.lock`、`app/rust-toolchain.toml` | 仓库根的 `Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml` |
| `app/kernel/crates/{bindings,codecs,commands,model,source-derive}/` | `crates/{bindings,codecs,commands,model,source-derive}/` |
| `app/kernel/crates/cli/` | `apps/vestacraft-cli/`，保留 Cargo 包名和可执行名 `vestacraft-cli` |
| `app/web/` | `apps/vestacraft-web/`，迁移现有 TypeScript 合同及其消费路径，页面实现仍由对应功能卡负责 |
| `app/contracts/`、现行 JS 合同工具、npm 配置及测试 | 根 `contracts/`、`scripts/`、`package.json`、对应锁和 `tests/`，共享工具继续服务 Rust 与 Web 两端 |
| `app/tools/generate-source7.py` 及现有合同生成、校验入口 | 脚本统一放 `scripts/`，由 `xtask/` 串接可执行的生成与校验命令 |
| 运行资源、制品打包、应用构建文档 | 分别归 `assets/`、`packaging/` 与 `docs/`。按实际文件和交付用途落位 |

`tools/ca-bootstrap/` 继续是独立的锁定工具工作区。`app/source-package-manifest.json` 及对应 P2-01 源包是冻结身份。现行源码从新入口记录新路径与同内容摘要，旧包按原相对路径复验。现有 crate 名称、依赖版本、I33/NS2 和工程文件格式继续沿用。

### 实施与验收

1. 在独立施工会话和隔离工作区保存基线树、锁、生成物与证据摘要。先完成实际文件迁移，再更新 Cargo 本地依赖、`include_*` 与运行时查找、生成器、npm/TS 配置、测试入口、构建与打包调用，以及 `.gitattributes` 的行尾保护。逐路径迁移，保留可审阅的移动差异。用新布局提交记录迁移前后的路径对照。
2. 从根执行 `cargo metadata --offline --locked --no-deps --format-version 1`，核实产品 workspace、成员和实际入口。核对 Cargo 锁的外部依赖身份，若 `xtask` 新增本地条目，单独说明。执行迁移后的 `node scripts/test-contracts.mjs`、`python3 -B scripts/generate-source7.py --check` 和离线 TS 类型检查，比较新旧生成物与原始合同的实际字节。生成模块因搬迁需要更新 import 时，逐项记录旧、新指向及其目标身份，并比较去掉这些定位差异后的内容。对活动构建、测试、交付路径扫描残留的旧 `app/kernel`、`app/web` 等引用，按用途逐项核对历史引用。
3. 根 `Cargo.toml` 会影响原 CA 自动选择。已审设计记录的 `build_check` 先识别 `Cargo.toml`，再识别 `package.json`。须复核锁定 CA 版本的路径选择，明确开发仓库和 CA 安装目录的入口，给出从新布局产出同版 Rust/WASM 与 Web 制品的明确接线和核对方法。优先用可显式选择的构建入口，或在 `packaging/` 定义适配的安装产物。从适配副本核对真实入口，正式 CA 安装、worker 和 Check 继续归属原卡。Web 资格须由实际 Web 制品证明。
4. Rust 编译或实际 CLI 往返须绑定明确的适用版本和获准运行额度。C8 候选达到原准入后，由原卡按新物理路径复核输入并执行 C8→G29→G30。若本卡先交回布局结果，列明静态与实跑各自的证据范围。旧 C7 二进制继续作为该版本的历史证据。
5. 交付移动差异、路径和摘要对照、活动命令入口、原始验证日志及待人工转交的 dsh 材料。源码路径、本机引用和同源证据相互对应后更新阶段入口。只在已核的目录、构建和交付范围内登记结果。

现行停报条件沿用原卡。迁移中发现合同字节改变、外部锁图变化、CA 只走 Rust 而漏掉 Web、或需要编译尚未准入的 C8 时，停止对应步骤，保留差异和已完成项。复审由用户手动发起。

### 施工检查表

> 由独立施工会话按顺序执行，可用 superpowers:executing-plans 配合项目的一卡一会话约定。以本卡和 [重写要求](../VESTA全功能重写要求.md) 为施工依据。隔离工作区从实际 Git 基线开始，开工时读取本卡和本轮已更新的文档。

**文件迁移与工作区**

- [ ] 以开工时的 Git HEAD 生成旧路径、Git blob、SHA-256、字节数清单。基线至少包括产品源码、Cargo 与 npm 锁、I33 合同、九份 CPJ、TS/RS 生成物与 P2-01 原包入口。在 RUN 保存 `git status --porcelain=v1` 和基线树。留存原源码包与 747 项 C8 pin 的旧路径对应。
- [ ] 按上表做物理迁移。沿用原 Cargo 解析器、工具链和依赖版本。根 workspace 收录实际 Rust 应用与库和 `xtask`，用明确的成员清单纳入 CLI，在 Web Rust 入口创建前保留 Node 目录。调整 `.gitattributes` 与 `.gitignore` 后用干净检出比较受保护文件的字节和 Git blob。
- [ ] 从根运行 `cargo metadata --offline --locked --no-deps --format-version 1`。预期能定位五个现有产品库、CLI 和工程任务 crate，CLI 原名与目标名一致，CA 引导工具仍由自己的 workspace 管理。独立比较两份 Cargo 锁的外部包、来源、校验和及依赖边。源包或证据出现未解释的字节变化，停对应步骤。

**生成与合同验收**

- [ ] 更新 `scripts/generate-contracts.mjs` 的 Rust/TS 输出路径、TypeScript 引用及 `tsconfig.contracts.json` 的输入。更新 `scripts/generate-source7.py` 对 `crates/`、`contracts/` 及输出模块的定位。修订 Rust 相对包含路径，优先逐项核 `crates/model/src/source7/mod.rs`、`crates/bindings/tests/generated_contracts.rs`、`crates/codecs/tests/retained_formats.rs` 所读目标。已有测试中的硬编码路径随消费者调整。
- [ ] 迁移 `scripts/verify-source-package.mjs` 对现行路径的生成物分类与目标目录排除。封存版 P2-01 包和其 manifest 在原包中复验。新路径的自验只对显式选定的产品文件树运行 `--seal`，在临时目录核实 `tests/contracts/verification-gates.test.mjs` 的源清单拒绝用例仍覆盖工具链绑定。避免从仓库根递归复制 `.git/`、`output/` 等与源码包无关的目录。
- [ ] `xtask` 先提供 `contracts-check`，调用现有 Node 合同入口和 Source7 生成检查，并传递错误退出码。只有当前运行范围允许时才构建执行该入口。现有的直接校验命令也应能从根运行：

  ```sh
  python3 -B scripts/generate-source7.py --check
  node scripts/test-contracts.mjs
  node node_modules/typescript/bin/tsc -p tsconfig.contracts.json --noEmit
  ```

  以锁定工具与私有离线依赖执行。预期 Source7 检查退出 0，Node 按 `tests/contracts/suite.json` 完整运行当版用例，零失败、零跳过、零取消，TS 无编译诊断。独立核对九份 CPJ 与旧值一致、生成模块实际引用可达。缺锁定输入时记录缺件与已通过项。

**打包与交回**

- [ ] 以 Craft Anything 提交 `1f8508a104134c436cbba5ea2f675619ef47702f` 的 `crates/ai/src/rebuild/check.rs` 的 `build_check` 为选择依据，核对适配目录。普通分支在检测到 Cargo.toml 时实际执行 `cargo build --quiet`，否则在检测到 package.json 时执行 `npm run build`。沿用原 Node 构建入口时，给 CA 单独准备适配的安装目录。将实际 CA 入口和产物约定放在 `packaging/`。完整 Web 应用尚待功能卡时，保留选择矩阵、入口设计与真实证据缺口。在后续适用授权下，从安装副本核对同版 CLI、WASM 与前端文件的实际清单。
- [ ] 更新根项目入口、现行构建文档和活跃任务所用路径，冻结材料引用照原路径解释。保存文件移动差异、路径及摘要核对、锁和生成验证原始输出、CA 选路结论、未测项及手动 dsh 复审材料。单独记账，本卡运行与 P2-03 C8/G29/G30 分别登记。

### 施工登记

迁移结果在隔离分支 `codex/artcraft-directory-alignment`。上面的原施工检查表保留发卡时的判据。文件迁移、依赖图、生成物、源码封存复验及 CA 适配副本选路已自验，待用户手动转交 dsh。Rust 产品构建、CLI 往返、真实双端制品和正式 CA 运行继续按各卡准入办理。

**从新布局续办。** 原任务卡及其封存以旧物理路径记录；在已迁移的 checkout 用下表定位实际文件。P2-03 的旧执行矩阵和 747 项 pin 继续作为迁移前证据。C8 准入时从实际 checkout 读取路径与 SHA 对照、为原卡另录新输入清单、更新原 RUN 派生驱动和监管的工作目录、源路径与二进制位置。隔离 checkout 中需要的 `output/` 夹具按各自固定摘要提供，原证据留在主工作区。

| 历史物理入口 | 现行入口 |
|---|---|
| `APP=<repo>/app` | `APP=<repo>`（开发与合同验证） |
| `kernel/Cargo.toml`、`kernel/Cargo.lock` | `Cargo.toml`、`Cargo.lock` |
| `kernel/crates/cli/`、`kernel/crates/<产品库>/` | `apps/vestacraft-cli/`、`crates/<产品库>/` |
| `web/src/`、`tools/generate-source7.py` | `apps/vestacraft-web/src/`、`scripts/generate-source7.py` |
| `BUILDING.md`、`licenses/` | `docs/开发与构建.md`、`assets/licenses/` |
| CA 工作区 `app/` 的源码与构建 | `app/source/` 为完整开发源树，`app/package.json` 为独立 npm 构建适配入口 |

P2-03 仍缺 7 项真实资格，保留 C8、G29、G30 的原 3000 秒和全部原停点。C8 工程入口从仓库根运行，产品锁保持原外部依赖。P2-04 消费 [CA 适配入口](../../packaging/ca/README.md) 及其制品约定。

- [施工回执](../../output/vesta-artcraft-layout/20261122-directory-alignment/RECEIPT.md)
- [旧新路径、Git blob 与 SHA-256 对照](../../output/vesta-artcraft-layout/20261122-directory-alignment/path-and-sha256.json)
- [C8 747 项对照](../../output/vesta-artcraft-layout/20261122-directory-alignment/c8-747-pin-crosswalk.json)
- [手动独审材料](../../output/vesta-artcraft-layout/20261122-directory-alignment/DSH-REVIEW-HANDOFF.md)
