# VESTA 全功能重写总体实施计划

<!-- P1-CURRENT-STATUS BEGIN -->
当前状态：rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。
<!-- P1-CURRENT-STATUS END -->

> **For agentic workers:** 执行使用 `superpowers:executing-plans`，由 Codex 直接逐项实施；步骤用复选框记录。现有用户约定优先，本计划不自动委派子 Agent 或 dsh，不自动创建其他执行会话。

**Goal:** 严格按照锁定 Craft Anything 的方法和步骤，1:1 重写本地 VESTA 的全部功能，并通过现有平台的实时 Web、Agent、个人空间和续作验收。

**Architecture:** 先 Study 原件，形成完整功能、行为、格式和算法规格；按规格实现同一领域内核、Web 界面及正式命令。Build/Check 反复迭代，功能对照和平台用户链共同决定完成。本文固定总体依赖，具体代码路径、函数签名和技术栈由 Study 后的设计确定。

**Tech Stack:** 研究使用锁定 Craft Anything 0.4.0 的 CLI/MCP、证据/unknown/check 机制；其本机构建候选工具链为已安装的 Rust/Cargo 1.99.1。产品技术栈在 P1 明确，不把默认 Rust + egui 桌面栈当成已确认方案。

**Spec:** [VESTA 全功能重写要求](../../VESTA全功能重写要求.md)、[接入技术要求](../../接入技术要求.md)。

**资料总入口：** [VESTA 资料索引](../../VESTA资料索引.md)。每步的资料 ID 都可点击到准确路径/章节；“前置产物”由前一步实际产生。

R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

## 全局约束（Global Constraints）

- 最终范围为目标 VESTA 的全部功能；阶段划分只安排依赖和顺序。
- 参考主程序 SHA-256 为 `b4835068478ec1852f732c93ab25768f2b8af0166a92fc68dfd85704fe71c9e9`；S04 About 的 AX 文本已核为 3.90.9a / 64-bit / Aug 8 2026，不由官网版本替换；About 小图不能作版本证据。
- Craft Anything 方法基线为 `1f8508a104134c436cbba5ea2f675619ef47702f`；上游与原件作为研究输入保持只读。
- 人工与 Agent 修改同一 Web 工程；同一工程同一时刻由一个引擎修改，切视图只改布局。
- 保存、另存和导出写个人空间；真实写成才确认，保存期间的新编辑保留 dirty；单文件按既定 60 秒自动保存与关闭前写回契约执行。
- 浏览器不持久保存工程/恢复副本；工程身份、导入源、另存目标、导出副本分清。
- 正式操作使用软件命令/API/CLI/MCP；UI 自动化用于行为研究与验收。
- 模型、权限、配额、只读和安全路径复用平台；历史账号、Key、预算、资源和成功结论不继承到本项目。
- Draft当前为Git研究/设计仓库（HEAD 754d06d，有WIP），不是产品仓库。产品根固定为VestaCraft/app；父原生工作区由P2-02正式API创建，未创建前禁止手建；提交/发布沿具体授权。
- 不新建覆盖矩阵、颜色门或 mutations.tsv；维护功能规格及其实际结果/证据。未知、失败和未验证保持真实状态。

## 复核重点（Review Focus）

以下五类在 Study 中先建立可判定样本，之后由对应实现任务承接为自动化断言：

1. 非正交晶胞、周期边界及特殊位置：坐标换算、成键、去重和测量的科学语义正确。归 S04/S06，交 P3-结构几何。
2. 部分占位、各向异性/磁性及多相：导入、编辑、计算、导出与重开不静默丢字段。归 S05/S06，交 P3-格式与科学计算。
3. 大体数据、缺失/非法外部引用与资源失败：有界处理，失败保留原工程，可解释恢复。归 S05/S06，交 P3-体数据及 P4。
4. 保存期间继续编辑、外改、另存冲突及迟到回执：保存身份正确且不清掉新编辑。归 S05/S08，交 P2/P4。
5. 原件 CLI 依赖 GUI/外部程序、退出或超时不明确：核真实完成、错误与进程清理，不把无产物登记成功。归 S07，交 P3-CLI 及 P4。

## 计划分层和当前可执行范围

- P0 使用[Study 详细计划](2026-11-08-vesta-study.md)，逐步产出资料、证据和设计输入。
- P1–P5 是完整交付路线。进入某阶段前，将其细化为代码路径、真实接口/签名、样本、命令、失败处理及证据齐备的执行计划，引用已形成的 Study 规格。研究尚未确定的 API 不写成现成接口。
- 阶段产出须能独立检查；后续新发现功能补回唯一功能目录和依赖计划，最终范围保持全部。

## P0：Study 原件和全功能规格

**执行入口：** [S01–S08](2026-11-08-vesta-study.md)。

- [x] **P0.1 固定目标和研究方法基线。** 资料：[D01](../../VESTA资料索引.md#d01) §2、[CA01](../../VESTA资料索引.md#ca01)、[V01](../../VESTA资料索引.md#v01)。产物 [BASELINE](../../VESTA研究/BASELINE.md)、S01 独立复审及已登记 Deep/CLI 身份可查；仅此基线子项完成，不代表 P0/完整 Study 完成。
- [x] **P0.2 完成功能、行为、格式、算法和 CLI 研究。** 资料：[CA02–CA06](../../VESTA资料索引.md#ca02)、[V03](../../VESTA资料索引.md#v03)、Study 各步指定章节。产出唯一功能目录、行为规格、样本及 unknown 记录。
- [x] **P0.3 收束 Study 并形成设计输入。** 资料：[D12](../../VESTA资料索引.md#d12)、[D02](../../VESTA资料索引.md#d02)、S01–S07 的实际产物。产出 `STUDY-REPORT.md` 和 `HANDOFF.md`，列影响设计的已知缺口及补证动作。

**通过条件：** 所有已发现入口都能追溯到功能条目；目标版本/证据范围清楚；关键未知有明确归属，不以手册目录冒充实测。影响所选架构/模块的未知先补证，独立研究可继续。

## P1：架构、接口和施工安排

**本次新会话授权（2026-11-10）：** 用户已明确要求执行 P1 设计阶段。按 P1.1–P1.4 及引用读取 Study 交付和 DPRE-01–08，处理影响设计冻结的相关前置、比较并确定实现路线/目录职责、领域及命令契约、CA worker/Check 接线与恢复；形成可评审设计后再写精确模块执行计划与测试契约。Codex 直接执行，dsh 独立复审。该次授权时四项未勾选；当前接受状态以下列复选框及本文件头部为准；本阶段交付后停下，不自动进入 P2–P5、产品编码/部署或模型 builder。

**前置：** P0 的功能目录、各域规格、样本/unknown、工具能力和报告；这些是本阶段必须直接阅读的输入。

- [x] **P1.1 确定实现路线、产品仓库和目录职责。** 资料：[D05](../../VESTA资料索引.md#d05)、[D07](../../VESTA资料索引.md#d07)、[CA07](../../VESTA资料索引.md#ca07)、`STUDY-REPORT.md`。比较符合全功能/Web 要求的实际可行路线，写成设计与选择依据。
- [x] **P1.2 固定领域与程序接口。** 资料：[D06](../../VESTA资料索引.md#d06)、`FUNCTIONS.md`、`FORMATS.md`、`ALGORITHMS.md`、`CLI.md`。确定对象/修订、单位、数值精度、撤销/事务、命令 schema、错误、保存和渲染完成时刻。
- [x] **P1.3 固定 Craft Anything 的执行与检查接线（设计级接受）。** 资料：[CA04](../../VESTA资料索引.md#ca04)、[CA08](../../VESTA资料索引.md#ca08)、`METHOD.md`。说明原生工作区和 Codex builder 的职责、真实 build worker/inbox、Check 调用时机与恢复；`rebuild_check` 不能当独立离线检查器使用。
- [x] **P1.4 形成可执行的模块计划与构建/测试契约。** 资料：[D11](../../VESTA资料索引.md#d11)、[D12](../../VESTA资料索引.md#d12)、已形成设计。列精确代码文件/接口、实际测试命令、资源边界、依赖及失败后的可继续项；每个任务携带对应功能 ID、规格和资料。

**P1.3口径裁定（2026-11-14，DEC-P1-03）：** 本项按设计级验收；真实new_project/copy/worker/Check/launcher、故意失败和恢复另行授权作有界实施验证，不能把未运行写成已通过，也不在本P1设计卡启动。用户已带回dsh对C4的设计级接受并明确授权回写，本项复选框记设计完成；C5增量另待复核，不代表真实运行通过。

**通过条件：** 设计与实施计划能由新会话独立接手；产品目录和技术栈已确定；所选库/算法能力有证据；不存在一边施工一边猜相邻接口的关键缺口。设计审核与后续实施按届时用户指令和适用流程进行。

## P2：第一条领域—页面—文件完整链

**前置：** P1 设计及精确执行计划；第一条链按依赖选择，剩余功能仍保留在 P3。

- [ ] **P2.1 建立可验证的工程模型、命令和 codec。** 资料：[D05](../../VESTA资料索引.md#d05)、[D06](../../VESTA资料索引.md#d06)、P1 接口、首条链对应行为/样本。新增逻辑先有可失败的业务断言，再实现并核通过。
- [ ] **P2.2 让 Web 人工操作和正式工具到达同一状态。** 资料：[D07](../../VESTA资料索引.md#d07)、[D09](../../VESTA资料索引.md#d09)、P1 页面契约；核工具修改、画面更新、手改后工具读回。
- [ ] **P2.3 完成工程保存、导出和新实例重开。** 资料：[D08](../../VESTA资料索引.md#d08)、[D02](../../VESTA资料索引.md#d02) §5、Study 对应文件/行为规格；核真实字节、内容、资源和保存状态，不由测试脚本补写最终产物。
- [ ] **P2.4 运行该链的 Craft Anything 检查及领域对照并迭代。** 资料：[CA08](../../VESTA资料索引.md#ca08)、[CA09](../../VESTA资料索引.md#ca09)、P1 检查接线和对应参考证据；保存差异、修正和未测项。

**通过条件：** 一条真实可编辑、可保存与重开的用户链成立；P2-06在最终集成版本完成首链整体界面E2E（新建/编辑/保存/导出/关页CLI/新页续改及失败恢复），连同P2.4真实CA检查均有证据才结束阶段2；只证明本链，不登记全部功能完成。

## P3：依赖顺序实现全部功能

以下各组都必须从 `FUNCTIONS.md` 取对应全部功能 ID，并阅读相关行为规格、格式/算法/CLI 条目及原始证据。每组的实际执行计划须包含新增逻辑断言、实现、真实对照、必要回归和交接，不能只写“继续完善”。

- [ ] **P3.1 结构、对称、晶胞和几何。** 资料：[VM06](../../VESTA资料索引.md#vm06)–[VM11](../../VESTA资料索引.md#vm11)、[D05](../../VESTA资料索引.md#d05)、S04/S06 规格；含多相、变换、成键、多面体、附加对象和测量。
- [ ] **P3.2 全部已发现输入、工程和输出格式。** 资料：[VM18](../../VESTA资料索引.md#vm18)、[D08](../../VESTA资料索引.md#d08)、S05 格式逐项依据/样本；核语义往返和外部读取，不以扩展名清单算支持。
- [ ] **P3.3 全部视图、样式、对象属性和交互。** 资料：[VM04](../../VESTA资料索引.md#vm04)、[VM05](../../VESTA资料索引.md#vm05)、[VM10](../../VESTA资料索引.md#vm10)–[VM13](../../VESTA资料索引.md#vm13)、[VMA](../../VESTA资料索引.md#vma)、[D09](../../VESTA资料索引.md#d09)、S04 行为证据。
- [ ] **P3.4 体数据、晶形和二维显示。** 资料：[VM05](../../VESTA资料索引.md#vm05)、[VM06](../../VESTA资料索引.md#vm06)、[VM14](../../VESTA资料索引.md#vm14) §14.12–14.15、[VM15](../../VESTA资料索引.md#vm15)、S05/S06 规格；核数据、切片/等值面和输出。
- [ ] **P3.5 全部科学计算和外部程序相关功能。** 资料：[VM14](../../VESTA资料索引.md#vm14) §14.1–14.15、[VM02](../../VESTA资料索引.md#vm02) §2.5、[CA06](../../VESTA资料索引.md#ca06)、S06/S07 算法来源及独立预期；保留所有计算目标。
- [ ] **P3.6 CLI、偏好、动画、历史和其他新增发现项。** 资料：[VM16](../../VESTA资料索引.md#vm16)、[VM17](../../VESTA资料索引.md#vm17)、S04/S07；核功能参数、错误、持久化、取消、资源和完成信号。
- [ ] **P3.7 收束每组差异并重新核全功能目录。** 资料：[CA02](../../VESTA资料索引.md#ca02)、[CA08](../../VESTA资料索引.md#ca08)、[CA09](../../VESTA资料索引.md#ca09)、该组实际 checks/unknown 和对照产物；发现遗漏补回目录及计划。

**顺序说明：** 上述分类不是互不依赖的并行许可；基础模型/codec、几何、显示、体数据/计算和接口依赖由 P1 写成明确顺序。同一功能有多个入口时保持同一规格和实现。

**P3通过条件：** 全部本阶段功能有完整语义与独立对照，P3-12在最终集成版本通过跨全部功能流程的整体界面E2E及必要失败恢复；未跑/失败保持阶段未结束，不能用源码或单卡测试拼通过。

## P4：现有平台正式接入

**前置：** 本款已有可用正式接口、锁定制品、文件协议；接入影响使用者范围及实际代码基线在本阶段锁定。

- [ ] **P4.1 注册软件、加载和两布局连续使用。** 资料：[D02](../../VESTA资料索引.md#d02) §2/4、[P01](../../VESTA资料索引.md#p01)、[P02](../../VESTA资料索引.md#p02)、本款页面契约；当前SW-3按D4-33/34两布局及真实loading，共享布局依赖R4-15交付；旧三模式为历史，明确沿用和最小增量。
- [ ] **P4.2 接当前页面工具与关页后 CLI/MCP。** 资料：[P03](../../VESTA资料索引.md#p03)、[P05](../../VESTA资料索引.md#p05)、[D06](../../VESTA资料索引.md#d06)、本款 CLI/页面规格；核同一工程、串行、真实交接和重开。
- [ ] **P4.3 接个人空间、安全路径与保存确认。** 资料：[P04](../../VESTA资料索引.md#p04)、[D02](../../VESTA资料索引.md#d02) §5–6、[D08](../../VESTA资料索引.md#d08)、本款保存协议；核外改、配额、只读、迟到回执和浏览器存储。
- [ ] **P4.4 核中文、模型/权限治理、版本与目标环境。** 资料：[D02](../../VESTA资料索引.md#d02) §6–9、[D10](../../VESTA资料索引.md#d10)、[P06](../../VESTA资料索引.md#p06) 和实际构建清单；零模型链先行，真实模型按实际授权，Mac/容器/x86 分开。

**P4通过条件：** 正式平台接入、两布局真实交付/连续性、工具/本人空间/治理与目标环境成立，P4-04在最终集成实际平台通过整体界面E2E及拒绝恢复；真实模型条件分支未验不写整阶段结束。

## P5：全功能与平台整链验收、交付

- [ ] **P5.1 逐功能核完整目录、原件对照与关键失败。** 资料：唯一 `FUNCTIONS.md`、全部域规格及本轮产物、[D11](../../VESTA资料索引.md#d11)、[CA08](../../VESTA资料索引.md#ca08)。科学内容与数值使用独立预期。
- [ ] **P5.2 跑真实平台用户链和既定最终关口。** 资料：[D02](../../VESTA资料索引.md#d02) §10、开工时现行平台执行者手册/测试方案及本款实际E2E；把真实关口定义/命令/输入/SHA锁到当卡`RUN/inputs/final-gates.json`。本索引[P07](../../VESTA资料索引.md#p07)只作历史测试样例，不能冒充现行最终关口；与平台阶段0原型卡P07分清。覆盖工具→当前页面→手改→保存→导出→关页续作→新页面重开。
- [ ] **P5.3 交付可复现源码、制品、资料和接手入口。** 资料：[D02](../../VESTA资料索引.md#d02) §11、[D11](../../VESTA资料索引.md#d11)、[D12](../../VESTA资料索引.md#d12)、真实 runtime/制品清单；保留原始失败、未测边界、独立复审与整合记录。

**最终判据：** 全部实际功能有实现和真实对照证据，影响一致性的未知已解决；平台完整链及规定目标环境验证成立。P5-02须在最终集成实际平台通过整体界面E2E，P5-03核最终交付同版或按最终候选重跑，才可结束阶段5。任何剩余缺口如实保留，不由 `Ready`、截图相似或中间阶段通过替代。

## 当前任务卡接续（2026-11-18）

**当前任务组织（2026-11-18）**：P0研究与P1当前设计已收口；现行25卡v4/v3及共同约定v3已有dsh独立审卡通过，精确SHA接受见[登记](/Users/agilecho/code/Draft/docs/任务卡/后续任务卡规则对齐接受登记.md)。用户已授权Codex逐卡代建独立施工会话；P2-01已启动并核开工输入，其余按依赖接续，四阶段整体UI E2E仍未执行。[任务总览](/Users/agilecho/code/Draft/docs/任务卡/README.md) / [执行预检](/Users/agilecho/code/Draft/output/vesta-task-handoffs/20261118-115219-rules-acceptance-execution-preflight/README.md)。原接受/封存及原累计预算保持，下方旧进度按历史时点理解。

本轮用户要求已回写P2—P5通过条件、各阶段三份管理文档和卡面；集中回归只在指定关口，普通卡/小修按实际新增/修改/共用影响选择测试。原型按现有原件行为与平台现行两布局修订，不把R4-15排期称为交付。阶段通过复选框保持未勾；P0/P1设计级收口不倒追产品界面E2E。

## 历史编制与接续（以下保持各时点记录）



本轮只编写计划、索引并核对路径/源码/官方章节链接。所有施工复选框初始未完成。下一次执行从 Study 的 S01 起，先读 D01、D02、本计划和 S01；已产生的研究结果从 `docs/VESTA研究/HANDOFF.md` 接续，不重复收集已回答的证据。

VESTA最新S04接续（063443包）：本轮仅接续S04触发补证：新增90条原生至1740磁盘派生索引；254稳定ID/78样本，90条入口级待核。S04.2仍partial-unchecked，S04.3/.4既有接受保持；13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 入口见[HANDOFF](../../VESTA研究/HANDOFF.md)。

VESTA最新Objects/焦点续证：本轮继续S04 Objects/焦点：新增45条原生至1785磁盘派生索引，254稳定ID/78样本/12头保持，87条入口级待核。S04.2保持partial-unchecked，S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 [接续](../../VESTA研究/HANDOFF.md)。

VESTA最新alpha路由：本轮继续S04 alpha事件路由：新增9条原生至1794磁盘派生索引，254稳定ID/78样本/12头/87入口级待核保持。S04.2保持partial-unchecked；S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 [接续](../../VESTA研究/HANDOFF.md)。

VESTA最新alpha运行时：本轮S04 alpha入口已有原件阳性：同PID激活后28×128浮窗与AXSlider192；诊断副本实测点击门槛/显示链。新增32原生至1826派生磁盘索引，254ID/78样本/12头/87entry-only保持。dsh复核pass、无硬阻断；S04.2按入口/条件登记范围接受，S04整卡收尾，Study14完成/15未完成。未进入S05或产品实现。 [接续](../../VESTA研究/HANDOFF.md)。

## 历史：P1本轮实际验收（2026-11-10）

[dsh正式复审](../../../output/vesta-study/20261108-113622/reviews/20261110-p1-design/dsh-review-final.md)及[同session差异复核](../../../output/vesta-study/20261108-113622/reviews/20261110-p1-design/dsh-review-delta-final.md)均为partial，只有P1.1可勾；整体P1未通过。首轮stream中断exit1没有裁定，不计通过。P1.1仅接受Rust原创内核/WASM单Worker+TS/React/Three.js、同内核CLI和未来workspace/app职责，不表示完整Web/科学/性能/Linuxx86运行成功。

[架构](../specs/2026-11-10-vesta-p1-design.md)、[接口](../specs/2026-11-10-vesta-interfaces.md)、[CA接线](../specs/2026-11-10-vesta-ca-wiring.md)、[模块计划](2026-11-10-vesta-modules.md)已交付；Task1–8是基础计划草案，G1–G6为受依赖约束的后续设计安排，不能当全功能精确计划。

剩余最小设计范围：冻结会改变公共payload的群/多相/科学和codec边界；补足原生workspace prepare/Study迁入/目标kind/launcher/Check的精确桥接设计；再把高级模块转成文件/函数/独立断言/命令/预算完整的实施卡。真正产品worker/builder/Check联调须后续明确授权，不把本阶段禁止运行的动作强加成P1当次操作。P1.3当时未完成的依据还包括prepare工具构建/迁入绑定及launch恢复设计未定，不仅是未运行worker。

接口I3获差异复核的主修正接受；其两处残余由Codex再修为I4，I4已于2026-11-11由dsh独立复审，R3-1/R3-2实质解决。P1.2/.3/.4和整体均不因此改通过。Study及旧封存保持；文书/绑定收尾见rev25，不实施P2。

## 历史：2026年11月14日 P1续办交付（R1）

2026-11-14 P1设计续办部分交付：D3／I5／C3及31组领域、codec/G6计划细化已形成，待用户手动转交dsh独立复审。P1.1已接受保持；P1.2／P1.4仍有具名公共接口及高级任务缺口，P1.3设计稿待审；原整体partial和1项接受/3项未接受不变。Study29/29保持，未进入产品实现、部署、worker、模型builder或原生Check。当前资料基线rev26，证据为20261114-100546-p1-design-continuation。

原P1四项通过条件保持。C3是可单独评审的精确桥接设计，不表示工具/worker已实施；I5和模块细化仍明确保留C-SCI、C-CODEC、C-HOST、资产及数据库的具体缺口。待审入口为R1/DSH-REVIEW-HANDOFF.md；本卡不进入P2，不因自验绿改变接受复选框。


## 2026年11月14日 R2复审接受回写与R3整改

2026-11-14：用户带回dsh R2复审并明确授权回写，P1.1继承接受，P1.3按DEC-P1-03设计级接受；真实worker／Check／launcher验证另行授权、仍未验证。P1.2、P1.4及整体P1保持partial，2项接受/2项未接受。N1–N7及C4旧口径的R3整改（D5/I7/C5）待用户手动转交dsh复核增量，不把当前新稿说成已审。当前资料基线rev28。仍不进入P2–P5、产品实现、部署或模型builder。Study29/29和原科学/格式/宿主缺口保持。原R1/rev26/C3段只记录当时交付；当前依据为R3接受记录、施工回执及增量待审材料。R2原C4设计接受继续保留，不能用后续运行资格未满足重新阻塞该设计项。


## R4当前接续（2026-11-14）

2026-11-14 R5：P1.1继承接受、P1.3 C4设计级接受保持，2接受/2未接受；P1.2、P1.4和整体P1仍partial。D7/I9补31个有界领域操作的数据边界、逐单元交接、85格式入口的字段政策及15项具体补证阻塞；不是完整功能冻结或产品实现。C6保持R4原字节、其增量仍待独立复核。资料基线rev30；真实worker／Check／launcher未授权、未验证，不进入P2–P5、产品实现、部署或模型builder。接受复选框继续仅P1.1/P1.3；C5有条件审查和当前C6整改不混写，dsh真实结果入口取代作者转录，用户接受决定独立保留。


历史R5主体设计续办：D7/I9/C6，资料rev30；31个有界操作、85格式路由/6类字段政策/28入口绑定、15项具体资格阻塞。P1.2/P1.4/整体partial，P1.3仅C4设计接受，C6增量待审；不进入P2–P5、产品实现或运行验证。证据与手动复审入口：[R5](../../../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/README.md)。


历史R6同卡整改：2026-11-14 R6：D8/I10按dsh R5复审整改D1–D14及σ公式，Codex自验待用户手动转dsh。P1.1继承接受、P1.3 C4设计级接受保持，C6原字节及未审增量身份不变；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料rev31；不进入P2–P5、产品实现、部署或运行验证，真实worker／Check／launcher未授权、未验证。 施工回执与手动复审入口：[R6](../../../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/README.md)。


R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。 [回执/复审入口](../../../output/vesta-study/20261108-113622/reviews/20261114-152228-p1-design-r7-errata/README.md)。


## 当前续办：P1-01 F1–F6独审结论登记（rev39）

2026-11-14 P1-01同卡独审结论登记：追加资料rev39，技术版本仍D11/I13/C6；dsh已独立复审通过rev38的F1–F6证据/边界修补，最新独审指针指向该被审包及真实JSON/SHA。本次rev39仅回写结论并Codex自验，O1登记为非阻断报告导航说明，不改旧封存或AGENTS。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受，SG-01…SG-06六项科学缺口不变。02–06未启动，产品和运行验证未授权。

最新[dsh独审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/REVIEW.md) / [真实JSON](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/review-result.json) / [本次登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/RECEIPT.md) / [O1非阻断说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/nonblocking-observations.json)。

O1：旧oracle-bindings.currentVerificationEvidence定位04-full正常模式546项报告；封存只读final为570项，两者均exit0/0失败。final入口为[/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json)。本次只登记该导航边界，不改旧字段、旧报告或封存。独审被审版本是rev38，rev39仅为结论回写且尚无新的独立复审。

## 当前06实施设计交回（rev60）

当前状态：P1-06已完成已有输入下的实施计划设计并partial交回，待用户手动转dsh；rev60/D29-I30-C6。15单元具名依赖与真实内容验收已安排，SG六/FS11/资产外算/AL02和AL04仍限制完整冻结。03rev48整卡独审待发起；04rev57 F1–F6已审、rev58 R1–R3未审且整卡partial。P1.1/C4接受保持，C5条件/C6未审，P1.2/P1.4/整体partial；未启动产品或运行。

[06实施与接受论证](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/acceptance-arguments.md) / [施工回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/RECEIPT.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/DSH-REVIEW-HANDOFF.md)。

P1原四项复选框维持两接受两未接受；本轮仅更新计划与论证，P2–P5未启动。

## P1-06历史步骤准入续办（rev61；当前见rev62）

当前状态：P1-06同卡步骤准入设计交回，rev61/D30-I30-C6，待用户手动dsh。已区分源码、构建、运行资格与完整结果，首个Web保存导出重开续改方案明确；CLI同生命周期身份绑定缺口单独停受影响路径。SG六/FS11/AL02与AL04等原缺口保持，P1两接受两未接受、整体partial；未实现产品或执行运行。

[06本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-063853-step-admission/RECEIPT.md) / [步骤与首链](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-063853-step-admission/step-admission.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-063853-step-admission/DSH-REVIEW-HANDOFF.md)。

本条只登记作者设计交回，不登记独审或运行通过。


## 历史同卡CLI契约续办（rev63 / IP3；当前见rev64）

P1-06同卡CLI续作身份契约设计交回，rev63/D32-I31-C6。NS/1补齐一次serve载入后的实际身份、消息/资源/重试、候选保存与宿主确认、关闭恢复；仅首链有界设计作者完成，待用户手动dsh。15单元/56步骤保留，SG六/FS11/AL02与AL04/资产及运行资格保持；P1两接受两未接受、整体partial。未进入产品实现或运行。

[06本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/RECEIPT.md) / [CLI契约设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/native-session-design.md) / [手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/DSH-REVIEW-HANDOFF.md)。

设计与验收依据沿原06需求材料；[NS/1机器合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/contracts/native-session.contract.json)、[封闭schema](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/contracts/native-session.schema.json)、[生产消费和权限](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/producer-consumer.json)、[设计身份链](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/identity-trace.json)、[当前步骤图](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/step-plan.json)。P106-CLI-BINDING仅在NS/1首链范围作者设计补齐、未获独审或运行接受。I31明确取代I30跨进程inspect→run传runtime ID的错误续作配方，commands/inspect可保留诊断身份；完整254/31领域/85格式需求与SG/FS/AL/资产缺口不缩减。

历史rev60/61与原封存保持；文中旧“CLI合同未冻结”是当时缺口，不是当前NS/1状态。未来实现须迁移native/core/save/host严格profile生成及现有私有宿主driver，再单独获产品/运行授权、分配具体根/环境/预算/次数；本轮没有操作VESTA/产品/worker/Check/launcher/模型，没有自动发起dsh或新卡。累计费用/次数/停报继承原06，完整账本未核不记零总额。


### rev63上位输入对齐

封存前只读核验发现平台PRD/技术方案/阶段4决策新增D4-33；已核三份全文差异均仅插入新条目。平台软件后续目标由旧三模式改两布局，布局切换保留同一页面、工程、任务和草稿；本轮不重做UI，CLI仍仅真实受控关页后启动。旧三模式表述作为历史背景，不作为现行平台布局要求。新条目不改变本NS/1身份/宿主保存合同或开放产品/运行。首轮3项SHA失败与中间baseline62及其字节快照保留，当前追加rev63/D32-I31-C6对齐输入，NS/1 schema及核心类型未改。

[实际差异与输入SHA](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/input-drift.json)。

## 历史 P1-06 全功能设计交付（rev64 / D33-I32-C6 / IP4，存在复审漏项）

P1-06 全功能作者设计已完成，rev64 / D33-I32-C6 / IP4，整体待用户手动 dsh 独立复审。科学六域、FS十一项、偏好与动画历史、资产及四个外算流程已有具体类型、来源规则与完整输出先验；同生命周期 CLI 由保留的 NS/1 明确扩展为 NS/2。15 实施单元、56 依赖步骤覆盖原 254 功能和 85 格式路由。当前没有待补且会改变公共契约的来源/类型字段；未来产品与原件运行资格另列。P1 接受仍为两项接受、两项未接受，整体未接受；没有产品实现或运行。

[06 最终回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/RECEIPT.md) / [统一接口与 CLI](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/composition-design.md) / [完整实施计划](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/implementation-readiness.json) / [一次性手动 dsh 待审包](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/DSH-REVIEW-HANDOFF.md)。本节为当前同卡后继；下文/前文各 rev60–63 及更早的“当前、partial、尚缺”描述均保留为当时记录，原字节证据与旧 baseline 不改。新合同是明确的设计后继，不追认旧卡、旧 177 定义、bounded protocol3、NS/1 或 C6 已支持新字段。

用户 DEC-P1-05 已确认“先不管许可的事，先完成功能”：许可不再阻塞本轮功能设计；保留资产来源与条款身份，不推定已取得授权。当前只做设计，继续原卡累计预算、权限、费用、次数及停报；完整旧账本未核，不记零累计。原件执行、产品运行、worker、Check、launcher、模型调用均未发生。dsh 由用户手动发起，本轮不代发或建立复审会话。

P1四项论证现见 [acceptance-arguments](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/acceptance-arguments.md)。P1.2/P1.4已具备作者完整设计与施工计划；接受位仍为 false，等待真实dsh结果。未来工程 `/Users/agilecho/code/VestaCraft/app` 未创建；本轮不执行产品命令。

## 历史 P1-06 同卡复审整改（rev65 / D34-I33-C6 / IP5，仍有两项残留）

P1-06 rev64复审问题已同轮完成作者设计整改，当前 rev65 / D34-I33-C6 / IP5，待用户一次性手动dsh复审。NS2计划、九摘要派生、typed资源、31域预期、15单元证据与canonical文件已统一；rev64原封存保持。P1接受仍为两项接受、两项未接受、整体未接受。只做设计，真实运行/目标资格是后续施工产物，不作为本轮设计接受前置。

[06整改回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-124023-rev64-review-corrections/RECEIPT.md) / [I33/IP5整改设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-124023-rev64-review-corrections/DESIGN.md) / [当前实施计划](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-124023-rev64-review-corrections/implementation-readiness.json) / [一次性手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-124023-rev64-review-corrections/DSH-REVIEW-HANDOFF.md)。旧rev64“全部作者设计完成”的表述是当时交付结论；收到复审后已确认计划与接口接线漏项，本节替代其当前判断，保留原证据和原文作为历史。

当前精确输入见本轮 `contracts/profile-derivation.contract.json`（9个输入选择/CPJ1/SHA及固定向量）、`contracts/resource-bindings.contract.json`（6类资源及11完成类型）、`current-oracle-bindings.json`、`unit-result-assertions.json`、`planned-files.json`。NS2仍17外层操作/14顺序＋3带外、11内核命令，当前40科学/表面操作保持；新代际必须同步Rust/Worker/editor/CLI/HostAgentMCP/HostSave/CodecRestore。旧NS1仅明确的前代参考，不允许自动降级。

IP5保留15结果单元和56按产物依赖步骤，U00生成器及delivery CLI分支必须执行NS2/资源/摘要契约测试；U02完整终端不再因“缺NS1设计”受阻。31域的旧 `missingExpected/blockedBy` 仅在显式predecessor中，当前预期按完整字段/数组/资源绑定；shape-only示例不代替科学数值预期。15单元入口均指本轮文件并锁SHA，当前原生codec唯一计划文件为 `kernel/crates/codecs/src/formats/native_vesta_v2.rs`，高维模型在 `kernel/crates/model/src/native_dimension.rs`。所有产品文件/命令均待创建/待执行。

接受沿DEC-P1-03设计级口径：现有P1.1/C4接受保持，C5条件/C6未审及03/04旧整卡边界不追认。实际锁/构建/OS/ABI/GPU/IME/宿主持久化/外算/数值误差资格留给获授权的实施步骤；不能改为本轮设计验收必须先运行。DEC-P1-05许可后置保持，既有预算/累计费用/次数/停报不重置。只由用户转交dsh，本轮无产品/原件/worker/Check/launcher/模型、安装、部署、stage/commit/push。

## 历史：P1-06 两项残留同卡整改（rev66 / D35-I33-r1-C6 / IP6）

P1-06 rev65两项残留已同轮作者整改，当前 rev66 / D35-I33-r1-C6 / IP6，线协议摘要代际保持I33。两条原生路由handler统一v2，payloadEnvelope及代际来源已入唯一合同，生成/校验实际读取声明并新增反例；九摘要全部重算，仅保存profile摘要变化，另八项字节不变。rev64/rev65原封存保持。仅设计，接受仍[true,false,true,false,false]，待一次性手动dsh。

[06本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/RECEIPT.md) / [两项整改设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/DESIGN.md) / [摘要变更对照](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/profile-digest-delta.json) / [一次性手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/DSH-REVIEW-HANDOFF.md)。

当前原生输入VM18-I01与输出OBS-O01的 `handlerPlanned` 均为 `kernel/crates/codecs/src/formats/native_vesta_v2.rs`；85路由绑定表是本轮后继，底层codec资料仍按SHA引用，不声称整套源文件复制入包。全85路由当前handler、两处部分漏改、其它路由误指旧名均有回归检查。

派生合同明确三字段信封、四字段输入记录、选择/投影/逻辑根替换顺序和唯一 `interfaceGeneration` 来源。文档修订I33-r1与wire/profile代际I33分开；所有9项按合同重算。单补信封不改变任何原向量；本轮两条handler改变仅saveBundleProfileDigest。若以后真正改代际，全部九项需重算/协商，不能沿用本轮“只变一项”结论。缺声明/代际冲突/记录缺项或多项/错序拒绝，禁止从脚本常量或样例反推。

本次只关闭F5-r/F3-r及对应回归检查，原科学模型/源/NS2消息/NS1/C6不改；15单元/56步骤/31域/40操作/254主责/85路由保持。原03/04及C5/C6被审范围不追认。DEC-P1-03仍按设计级复审，不新增真实运行前置；许可后置和原预算/累计费用次数/停报继承，不归零。产品/原件/worker/Check/launcher/外算/模型未运行，未安装部署或提交推送；仅由用户手动转dsh。

## rev67：rev66独审通过与P1设计阶段收口登记

rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。

这是接受状态后继，不改rev64／rev65／rev66封存。上文带历史版本号的partial、待审和未完成段落保留其当时范围；现行接受状态以文首登记和本节为准。未来实际构建、宿主及数值资格仍按获授权步骤验证，不反向变成本次设计接受前置。原累计预算／费用／次数和停报规则继承，不归零。
