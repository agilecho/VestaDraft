# P2-04 · 当前Web编辑与锁定双端交付

日期：2026-11-18。卡面v3，按规则v1.4补齐的文档后继；待用户手动dsh复审，未发放施工、未执行。原v2的卡面接受按原SHA保留，不移用于本后继。

## 给你看的

用户和正式页面工具能在同一个Web工程里连续修改晶胞，看到实时变化，并得到可重建的Web与Linux CLI制品。

本卡覆盖：单Worker/页面API、首链基础三维视图、真实构建/preview、浏览器和Linux资格；源码齐全后才启动首次完整build。

不登记个人空间保存、关页续作、CA Check或平台正式接入；不自动部署。 依赖未就绪时可先做不依赖它的已授权部分，完整结果未成立则交partial。

## 给执行者看的

### 1. 接手、权威依据与实际基线

必须完整读[后续施工共同约定](/Users/agilecho/code/Draft/docs/任务卡/后续施工共同约定.md)、本卡、[阶段入口](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/README.md)及最新续办。共同约定是本卡的一部分，含绝对根、权威顺序、输入SHA、资源/停报/回执规则；先核真实状态，再动依赖步骤。

需求依据：总体计划P2.2及P1.4交付底座；SW-2/4/5/9；技术方案§2/9.6/13.3；BT/AB2/NS2/RENDER2。

实际基线：P1资料rev67、已审技术rev66/D35/I33-r1/IP6，wire/profile I33。Draft HEAD为754d06d9d5d1dcafd30ba951105c16b73644abe9且有继承WIP；正式VestaCraft根当前不存在，产品源码/产品HEAD/锁/构建/runtime均未产生。未来接手必须读取前卡真实产物而非沿用此开工前快照。Q/input-manifest.json固定本次所读原字节，未来核漂移。

| 当前精确设计入口 | 本卡消费方式 |
|---|---|
| T66/implementation-readiness.json `/units/0`（U00） | contractIds：`BT, AB, CA, AB2, COMPOSITION2, PLANNED_FILES, CURRENT_ORACLES, UNIT_RESULTS, NS2, PROFILE2, RESOURCES2, GENERATION`；按该单元contractBindings的path/SHA/version读实际文件 |
| T66/implementation-readiness.json `/units/1`（U01） | contractIds：`BOUNDED3, SG, FS, SG2, FS2, COMPOSITION2, HISTORY2, PLANNED_FILES, CURRENT_ORACLES, UNIT_RESULTS, NS2, PROFILE2, RESOURCES2, GENERATION`；按该单元contractBindings的path/SHA/version读实际文件 |
| T66/implementation-readiness.json `/units/6`（U06） | contractIds：`BOUNDED3, SG, AB, AL-base, AL4-history-evidence.contract.json, AL4-manipulation-history.schema.json, SG2, APP5, HISTORY2, AB2, COMPOSITION2, RENDER2, PLANNED_FILES, CURRENT_ORACLES, UNIT_RESULTS`；按该单元contractBindings的path/SHA/version读实际文件 |
| T66/implementation-readiness.json `/units/14`（U14） | contractIds：`BT, CA, EX, HS4, AB, AB2, EX3, COMPOSITION2, HISTORY2, PLANNED_FILES, CURRENT_ORACLES, UNIT_RESULTS, NS2, PROFILE2, RESOURCES2, GENERATION`；按该单元contractBindings的path/SHA/version读实际文件 |
| T66/domain-consumption.json、current-oracle-bindings.json、unit-result-assertions.json | 按所列单元的实际domainBindings/publicBoundaries匹配现有group，独立预期依当前绑定；应用/交付断言按unit-result-assertions的unit匹配，不虚构域名表；predecessor只作历史 |
| T66/planned-files.json、format-implementation-bindings.json | canonical文件和85路由当前handler；两条native路由只绑native_vesta_v2.rs |

#### 原型与流程依据

采用[原型与验收依据](/Users/agilecho/code/Draft/docs/任务卡/原型与验收依据.md)的 **MAIN/EDIT/HOST**（版本、页面/流程、原始证据及SHA入口见该文）。本卡采用：Web主视图/新建晶胞和位点/工具反馈；采用宿主页面连续性约束，P2只在隔离宿主，不验正式平台两布局。 对应PRD/技术方案仍按本节既有引用执行。原v2/v3接受仅属原卡面，当前状态见阶段README与HISTORY；不得用旧回执替代原型或需求。

### 2. 唯一结果、真实依赖与步骤

唯一交付结果为本卡首段所述完整用户/工程结果；不因脚本成功或部分源码完成自动登记整单元。

| 依赖产物/条件 | 何时需要、缺失如何处理 |
|---|---|
| P2-03模型/内核/codec/NS2源码及生成锁 | build的真实输入；不得用空crate补洞 |
| P2-02正式工作区、工具/字体/浏览器/Linux输入 | 逐目标核实际包、SDK、镜像及receipt；Mac成功不能替代Linux |
| C6浏览器输入/launcher增量审核 | 采用C6相关能力前核用户手动复审结果 |

承接IP6步骤及原始依赖（源码、资格、完整结果分别验；不是按卡号串行）：

| 步骤 | dependsOn原值 |
|---|---|
| `U06.basic-scene` | `U01.model`、`U01.engine` |
| `U01.page` | `U01.engine`、`U06.basic-scene` |
| `U14.preview` | `U01.page`、`U00.scaffold` |
| `U00.lock-inputs`（本卡补完首次完整build的剩余输入） | `U00.scaffold` |
| `U00.build` | `U00.lock-inputs`、`U01.page`、`U02.save-state`、`U14.cli`、`U14.preview` |
| `U14.web-qualification` | `U00.build` |
| `U14.linux-qualification` | `U00.build` |
| `U01.current-page` | `U00.build`、`U14.web-qualification` |
| `U00.delivery` | `U00.build`、`U14.web-qualification`、`U14.linux-qualification` |

### 3. 文件、实施顺序与交付物


主要文件：`web/src/kernel/{worker,client,generated}.ts`、`web/src/worker/protocol-v2.ts`、`web/src/editor/{App,cell-editor,site-editor,interaction}.tsx`、`web/src/render/{renderer,packets,materials,labels,environment,camera}.ts`、`web/src/host/page-api.ts`、`scripts/{build,preview}.mjs`、`kernel/crates/preview/src/main.rs`和`Build/vestacraft-preview`。保留根只有package.json，Cargo workspace位于kernel。

**输入闭合由本卡负责：**P2-01仅交合同生成器的实际消费者图，不足以单独放行本卡完整`U00.build`。本卡在已有源码齐备后补完`U00.scaffold`的构建接缝，并完成本次build所消费的`U00.lock-inputs`剩余部分：实际Web/Worker/native/WASM/CLI/preview的package/Cargo图与制品、Node/npm/Rust/wasm-bindgen等工具来源、字体/资产、浏览器/SDK/目标镜像及LICENSE/NOTICE。计划文件包括`package.json`、`package-lock.json`、`rust-toolchain.toml`、`kernel/{Cargo.toml,Cargo.lock}`、`scripts/{build,test}.mjs`、`licenses/{dependencies,assets}.json`和`BUILDING.md`。沿精确候选版本重锁、核闭合传递图和缓存，再进行offline/locked build；缺任一实际消费输入则停其构建，不把P2-01局部通过冒称完整输入已齐。

输入完成回执必须绑定本次实际源码图、目标、锁/工具/资产/许可清单及SHA；未来P3/P4新增消费者或目标时，由引入它的卡重锁并重验受影响输入，不追认本轮已经覆盖未来未出现的依赖。

本卡同时计划新建并维护以下**APP内真实测试**，不放RUN空驱动：`tests/delivery/build-manifest.test.mjs`、`tests/browser/first-chain.spec.ts`、`tests/browser/first-chain-environment.spec.ts`。浏览器测试标题分别含`first-chain`/`first-chain-environment`并实际驱动本次构建；npm脚本须选择真实测试，零选中失败。`tests/contracts/{profile-digests,native-resource-types-v2,native-session-v2}.test.mjs`复用P2-01/03真实文件并补本卡消费者断言；不复制宽松替身。原始运行日志、环境、制品和独立比较结果留RUN。

页面API/Worker消费P2-01同源生成物，不能在TS另写科学逻辑。此卡只开启首链已实现能力，完整相机/样式/交互由P3-04交付。build需要P2-03完整源码与本卡页面/preview源码均在位；先编码再build，不把U00.build当这些源码的先决条件。

BrowserLaunchPlan1、source registry、buildRoot及安装receipt须由真实外置任务输入锁定；严格按C6§7.4，不自报信任摘要。Linux测试只能用该卡实际批准镜像/runner；没有环境则交未验证项，源码和Mac验证可继续。

实施顺序：核授权/输入/真实依赖→固定独立expected并留必要RED→实现本卡业务及贯通适配→运行下表必要检查和失败反例→真实内容/生命周期验证→保存源码/锁/制品/runtime及原始日志→更新状态并交手动dsh材料。只有卡面所列消费者输入缺失才停依赖部分；不擅自改需求、黄金值或老封存。

### 4. 关键验收断言与原始证据

| 验收项 | 必须证明的结果 |
|---|---|
| 同一当前页 | 工具a=5→用户a=6→工具读回6→undo/redo，页面/Worker/Engine身份一致；无需刷新/重开才可见 |
| 完成时刻 | Committed、Rendered与Loaded绑定同一doc/epoch/revision；坏字体/丢GPU不报Rendered，也不撤销已提交状态 |
| 完整构建 | native→WASM→匹配wasm-bindgen→TS→Vite→manifest真实执行；manifest绑定源码、锁、字体、生成物、Web与CLI字节 |
| 完整输入 | 本次build消费的package/Cargo/工具/字体/目标/许可清单实际闭合；缺工具或任一包/资产即失败，P2-01局部锁不作为整图通过 |
| 目标资格 | 真实浏览器及linux-x86_64二进制载入同一source/profile指纹；本机ARM不冒充x86/现场 |
| launcher | 正常可见窗口和明确--screenshot模式；截图等真实Loaded/Rendered/font ready，90秒失败清理自有进程 |
| 可重建 | 干净runner离线按锁重建并运行；故意破坏WASM/CLI/字体/manifest必须在正确阶段失败 |

每项同时保留真实输入/操作/输出与独立预期来源、完整内容比较、原始失败及版本身份；公共写入链按共同约定核实际写/读/销毁/重开/续改。只验类型、计数、文件存在、HTTP成功或自读自写不能完成本卡。

### 5. 必要命令、前提与失败处理

#### 受影响测试选择

新建editor/Worker/scene与build-manifest、first-chain及first-chain-environment；首次完整交付才跑NS2与目标构建，小修按editor→Worker→render调用链选用例/grep。

先按共同约定§5.1落`<RUN>/inputs/affected-tests.json`。下列为首次新增结果的候选命令；卡内小修使用实际测试定义的用例过滤、明确文件/目标和非零选中清单，记录预期与未受影响项不跑的理由。正例须证明本卡真实结果，反例须拒绝且不污染当前工程；失败/超时/0选中均不算通过。纯文档只做相关静态核对。本卡不是阶段收尾时，其单卡界面测试不替代阶段整体UI E2E。



下面全部是**未来待实现/待运行**命令，当前未执行。npm/cargo命令cwd=APP；RUN driver从卡面实际受管根调用，其源/输入/工具版本须锁定。先有真实源码/脚本/锁/包/目标资格再运行，不创建占位成功出口。命令内的占位必须在开工输入落成实际绝对路径。

```text
npm ci --offline --ignore-scripts
npm run build
node --test tests/delivery/build-manifest.test.mjs
npm run test:delivery:ns2
npm run test:browser -- --grep first-chain-environment
npm run test:browser -- --grep first-chain
npm run test:delivery -- --target linux-x86_64
```

测试名/grep须绑定真实测试定义且非零选中；没有选中不能算通过。顶层失败状态原样传播。编译/单元测试只证明对应层；真实用户链必须另留当前runtime、画面/状态、持久内容和新实例证据。若需新增源/依赖/环境，仅按本卡未来授权及共同约定拟定上限取得；不静默换版本或联网补件。

### 6. 授权、停报与回执

本轮仅修订卡面和管理文档。未来明确执行本卡后，允许范围限本卡文件/结果/必要检查；资源上限和每次计量执行共同约定§6，旧同卡累计/更严限额优先。新增原件/外算/provider/CA/模型/目标付费资源必须有具体授权输入；缺者停对应分支，保留未执行，不把其它项目权限或旧卡余额继承成新额度。

输入/类型关键冲突、原生根冲突、未知写者、环境或剩余额度缺失、超时/越额时停相关步骤并交最小复现；保留已完成及可继续项。不得自动启动下一卡或清别人的WIP。回执按“给你看的／给执行者看的”交真实结果、版本、完整证据/失败/限制/未验/设计偏差及DSH-REVIEW-HANDOFF；只由用户手动转交dsh，Codex不联系或启动复审会话。

不从发卡推导stage/commit/merge/push/发布/部署权；已存在的具体授权按其范围执行，不重复询问。独审通过仅接受本卡已证明结果，阶段/整产品接受须满足其余真实判据。
