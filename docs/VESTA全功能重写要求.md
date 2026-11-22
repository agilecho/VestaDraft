# VESTA 全功能 1:1 重写要求

<!-- P1-CURRENT-STATUS BEGIN -->
当前状态：rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。
<!-- P1-CURRENT-STATUS END -->

R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

## 1. 已确认目标

用户原话：**“严格按照craft-anything的方法和步骤1比1重写VESTA的所有功能”。**

- 最终范围为目标 VESTA 的全部功能。此前“先选择一个科研或教学任务”的提问不再用于缩减范围。
- 严格采用锁定 Craft Anything 的研究、构建、检查和根据差异迭代的方法，保留研究证据和未解决问题。
- 最终产品同时满足《接入技术要求》：Web 手动编辑、Agent 正式命令操作当前页面、实时可见、个人空间保存/导出、关页续作和显式重开。
- **DEC-P1-04（2026-11-14 用户追加）**：最终软件结构须参考 `/Volumes/T9/Development/Sources/Artcraft/wordcraft`。后续架构与实施安排须说明参考结构如何用于本项目，具体入口和适配边界见下方“WordCraft 软件结构参考”。
- 分阶段只安排依赖与施工顺序；中间切片、演示、估算和占位均不替代全功能目标。功能困难或暂未完成时保留缺口，不静默删减。
- Codex 默认直接研究、编码、自验和维护文档；只有用户明确指定时才交 dsh 执行。产品里的 dsh 工作台与 Agent 不代表已指派 dsh 施工。

## 2. 研究基线

### VESTA 目标

- 目录：`/Users/agilecho/Documents/VESTA`。
- 主程序：`VESTA.app/Contents/MacOS/VESTA`，静态识别为 macOS x86_64/arm64 通用程序。
- 主程序 SHA-256：`b4835068478ec1852f732c93ab25768f2b8af0166a92fc68dfd85704fe71c9e9`。
- `libVESTA.dylib` SHA-256：`c2c677facf6c67783cc7bbe64e92b75beb1a567fc373eacabee58b11fb81a77f`。
- `Info.plist` 只有 `CFBundleVersion=3.0`；S04 运行 About 已核实 **3.90.9a / 64-bit Edition / Aug 8 2026**。证据见 [BASELINE](VESTA研究/BASELINE.md)，资源及外部程序身份仍须按步骤核实。
- 全部功能以这一本地目标实际能力为基线，官网手册和版本说明用于查漏与核实差异，不自行替换其他发布版本。
- 当前只有发布包，未发现源码。许可证说明源码不公开，发布文件再分发需书面许可；研究输入不自动纳入交付包，重写、依赖复用与再分发分别核实。

### Craft Anything 方法

- 目录：`/Volumes/T9/Development/Sources/Artcraft/craft-anything`。
- 提交：`1f8508a104134c436cbba5ea2f675619ef47702f`，工作区版本 `0.4.0`。
- 步骤与约束依据：`crates/ai/src/rebuild/{mod,study,brief}.rs`。
- 桌面应用与检查依据：`crates/ai/src/rebuild/kinds/{app,mechanics}.rs`、`crates/engine/src/workspace.rs`。
- 旧参考报告基线为 `078cf0c4842c`；实际步骤、工具与参数按本轮源码重新核实。

### WordCraft 软件结构参考

- 用户原话：“最终的软件结构需要参考/Volumes/T9/Development/Sources/Artcraft/wordcraft”。该目录是新增的产品结构参考，与 Craft Anything 的研究/重写方法分工并列。
- 优先读取该仓库的 `docs/architecture.md`（Layers、Data flow、Testing and gates）、根 `Cargo.toml` 和 `README.md` 的 Architecture 节。参考重点为基础类型、领域模型、计算/布局、格式、渲染、统一引擎/命令、Agent 适配、界面与应用入口的职责分离和依赖方向，以及 `crates/`、`apps/`、`xtask/` 的工程组织。
- 后续设计需给出上述职责与 VESTA 科学内核、Web、CLI、Agent 及平台宿主的对应关系和差异理由。现有 Rust/WASM 单 Worker＋TS/React/Three.js、同内核 CLI 的路线级接受继续保留；本要求没有指定改用 WordCraft 的 egui 界面栈或照搬文稿业务模块。若结构适配确需改变已接受路线，先交具体影响及待裁定问题。
- 本次只登记新增结构约束和只读来源；结构对照尚待后续设计，未修改被审设计快照、接受状态或产品代码。来源版本、文件摘要和本机入口见[资料索引的 WordCraft 结构参考](VESTA资料索引.md#wordcraft-structure)。

### ArtCraft 产品目录一致性（2026-11-22 用户确认）

VestaDraft 的产品目录须与 ArtCraft 自研应用的共同结构保持一致。仓库根管理产品 Cargo workspace，`apps/` 放应用入口，`crates/` 放产品库，`xtask/` 放工程任务，`assets/`、`packaging/`、`docs/` 分别放资源、打包和文档。按职责迁移已有源码，新增应用入口和交付材料按相应目录落位。七个本地参考项目及路径映射见 [ArtCraft 目录对齐施工卡](任务卡/ArtCraft目录对齐.md)。

现行 Rust/WASM 单 Worker、TS/React/Three.js 和同内核 CLI 路线继续适用。源码与文档继续用 VestaDraft 的同一 Git 仓库，`output/` 继续保存历史证据，正式 CA 工作区及安装按相应任务的现行准入办理。迁移完成前，旧路径仍用于定位现行源码及封存输入。

## 3. 必须遵循的方法和步骤

用户阶段为 **Study → Build → Check → Ready**；检查发现差异时反复修改、构建与检查。

### Study：研究并形成行为规格

1. 先读证据总览和摘要，再用真实工具回答明确问题；复用已有证据，不重复已回答的探针。
2. 从菜单、对话框、快捷键、鼠标操作、文件格式、CLI、计算和导出入口逐项发现功能，形成可追溯的完整功能目录。
3. 逐项记录输入、前置状态、默认值、操作、状态变化、数值/单位、画面、保存结果、错误及恢复，准备有区分度的样本。
4. 按其 unknown 机制登记未回答问题、重要程度和补证方法；解决时关联证据。未知不能默认排除在实现范围外。
5. 明确研究模式：Clean room 不读取原程序内部代码和文件；Deep 可作内部分析，要求先研究相关功能的反编译结果，再记录常量、公式、规则和函数位置，自行实现。模式与证据类型如实记录，不把伪代码称为原源码。
6. 已有准备工作读取了许可证、包元数据和文件身份，不包装为纯 Clean room 全程证据。本轮已选择 Deep、运行真实 Craft Anything triage 并完成原生记录往返；S04 开始原件观察，尚未启动 rebuild 或产品实现。

### Build：按证据原创实现

1. 按依赖维护简短、可跟踪的施工步骤，功能目录保留全部最终范围。
2. 进入 Craft Anything 原生 rebuild 工作区时，遵守其 `app/` 为实现区、其他目录为只读研究材料的规则；当前需求文档不冒充已生成的调查项目或 `craft.json`。
3. 领域模型、科学算法、格式读写和命令由同一业务实现负责，Web 人工操作与 Agent 操作共享它。
4. 按证据重建布局、交互和行为；Deep 模式按其契约维护 `MECHANICS.md`/`mechanics.json`，记录计算依据，不粘贴反编译代码。
5. 不以固定样本输出、截图或占位代替真实计算和可编辑功能。

### Check 与迭代：同样输入对照原件

1. 按其 `rebuild_check` 流程检查产物，消费具体差异、claims、unknowns 和结果；未真正调用时不记为已执行。
2. 用同样输入、操作与选项比较领域状态、科学数值、交互、文件和画面；独立预期来自参考观察、可核公式或独立解析。
3. 把差异反馈到下一轮修改，重新构建与检查，保留每轮证据及剩余问题。
4. 当前桌面 app 默认检查包括构建、适用的运行对照、截图和 Deep 模式 mechanics 检查。常量在源码中出现只证明这一检查的范围，不代替科学算法正确性与全功能验收。
5. 完成前复核功能目录和未解决问题，逐项记通过、失败、未验证与差异。

## 4. 本项目对默认行为的约束

以下遵循 `brief.rs` 中用户要求优先、不得静默删减的规则：

- `AppKind` 默认目标是桌面应用的核心功能，默认栈为 Rust + egui；本项目已要求全部功能和 Web/Agent 接入，不能由默认值缩减范围或改成仅桌面交付。具体栈在研究后确定。
- `RunOptions::default` 的 `max_rounds=6`、`good_enough=90.0`，以及 CLI `rebuild --rounds` 的默认 2，都不是用户确定的本项目资源预算或最终完成标准；入口不同须核实际配置。
- 当前实现达到迭代上限后，可将仍可使用的产物记为 `Ready` 并保留不足。因此 `Ready` 必须结合真实检查解读，不能单独证明本项目全功能重写完成。
- 数值公差、图像容差和兼容差异按领域及目标行为明确记录，不以统一相似度代替功能一致性。

## 5. 全功能发现入口

以下用于后续 Study 查漏，尚不是已穷尽或已验收的功能清单：

- 文件与工作区：新建、打开、导入、多结构/数据集、标签页、保存、另存、关闭、重开、工程引用和资源路径。
- 结构编辑：晶胞、晶格、原子位点、元素、坐标、占位、位移参数、对称性及相关操作。
- 晶格与几何：晶胞变换、超胞/子胞、周期边界、成键、距离、角度、平面、配位多面体和几何输出。
- 三维显示：模型、对象属性、标签、选择、视图、相机、缩放、旋转、绘制边界、颜色、透明度及撤销/重做。
- 体数据：导入、网格、等值面、切片、颜色映射、多数据集运算和结构叠加。
- 晶体形貌及本地版本提供的其他结构类别。
- 科学计算：结构因子、密度及相关变换、几何和统计计算，逐项按实际功能核实。
- 二维数据显示、图像编辑/属性、线剖面、等高线和相关输出。
- 全部实际支持的输入、工程、输出格式及选项，区分读取、显示、编辑、保留和导出语义。
- 栅格、矢量及几何资产导出，包含尺寸、分辨率、透明背景、视图和内容选项。
- CLI、快捷键、偏好、自动处理和外部程序联动；依赖 MADEL、STRUCTURE TIDY、RIETAN-FP 等的功能须查明，不能因依赖外部程序而直接排除。

## 6. 平台要求与完成判据

同时逐项满足[接入技术要求](接入技术要求.md)，特别是当前页面实时操作、三种视图连续使用、真实保存确认、个人空间、权限配额、无浏览器作品持久副本、关页续作及新页面重开。

保存/导出和视图行为按平台明确契约适配；与桌面原件存在差异时在功能规格中登记，不通过复刻桌面行为绕过平台要求。源码、bundle、Web/CLI 制品和运行版本须一致，Mac、容器及目标 x86 的证据分别登记。

最终完成要求是：完整功能目录中的功能均有实现与真实对照证据，影响功能一致性的未知项已解决，平台完整用户链成立。阶段成果、可运行状态或 `Ready` 标签不替代最终验收。

## 7. 当前状态

- 已确认：全部功能 1:1 重写、严格采用 Craft Anything 方法、同时满足平台接入要求。
- 已核实：本轮方法源码、VESTA 包元数据、主要二进制身份及本地许可文本。
- S01–S08按原研究判据完成，Study29/29与P0研究交付保持。rev66设计独审通过、rev67接受登记后，P1.2/P1.4设计级接受，P1四项及整体设计阶段收口；P1.1和P1.3 C4既有接受保持，C5条件/C6未审及03/04历史范围不追认。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 详见 [Study 计划](superpowers/plans/2026-11-08-vesta-study.md)、[总体计划 P1](superpowers/plans/2026-11-08-vesta-full-rewrite.md)及 [HANDOFF](VESTA研究/HANDOFF.md)。
- 当前 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open，即14个未解决；来源 revision10、数据库 revision4 保持。本次状态对齐没有新增目标证据或产品代码。 历史 R132 的1640条核验派生快照（raw1288+按ID补读352）单独保留。
- S02.3 曾因五个入口不足撤销勾选；R134 补齐 22 项指定来源范围（19 条本地原字节身份、3 条无 raw 摘要的网页正文），经 dsh 及收尾验收后恢复完成。本会话再次 [dsh 复核通过](../output/vesta-study/20261108-113622/reviews/20261110-051231-s02-delivery-audit/dsh-review.stdout)，没有硬阻断；阅读及目标绑定边界见 [SOURCE_NOTES](VESTA研究/SOURCE_NOTES.md)。
- S08具体交付及实际dsh裁定见 [STUDY-REPORT](VESTA研究/STUDY-REPORT.md)，已按原研究判据验收。已登记的行为/科学/格式/外部程序及目标环境前置补证保留；技术栈与实现计划留后续设计，本卡不自动接续。

## 8. 计划与资料入口

- [总体实施计划](superpowers/plans/2026-11-08-vesta-full-rewrite.md)：完整阶段、依赖、产出和最终验收要求。
- [Study 阶段执行计划](superpowers/plans/2026-11-08-vesta-study.md)：S01–S08 的具体动作、前置产物、逐步引用、检查和失败处理。
- [资料索引](VESTA资料索引.md)：固定资料 ID、实际本机源码入口、官方手册章节、平台复用样例及已知旧口径。

2026-11-08 用户要求每一步列明引用资料，并已要求从 S01 执行 Study。状态以 Study 计划与 HANDOFF 为准；产品施工未开始。

历史S04触发补证状态：本轮仅接续S04触发补证：新增90条原生至1740磁盘派生索引；254稳定ID/78样本，90条入口级待核。S04.2仍partial-unchecked，S04.3/.4既有接受保持；13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 旧手册修饰键/最佳平面启用条件已按实测登记差异；未答入口与失败不删，仍保持全功能最终范围。

历史Objects/焦点续证：本轮继续S04 Objects/焦点：新增45条原生至1785磁盘派生索引，254稳定ID/78样本/12头保持，87条入口级待核。S04.2保持partial-unchecked，S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 颜色关闭实际应用RGB、F2/Control-click及窗口/列表焦点条件已回写，剩余alpha pointer入口不删。

历史alpha路由续证：本轮继续S04 alpha事件路由：新增9条原生至1794磁盘派生索引，254稳定ID/78样本/12头/87入口级待核保持。S04.2保持partial-unchecked；S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 新假设未成功均保留，未据静态链接删除未知入口。

S04 alpha运行时接续：本轮S04 alpha入口已有原件阳性：同PID激活后28×128浮窗与AXSlider192；诊断副本实测点击门槛/显示链。新增32原生至1826派生磁盘索引，254ID/78样本/12头/87entry-only保持。dsh复核pass、无硬阻断；S04.2按入口/条件登记范围接受，S04整卡收尾，Study14完成/15未完成。未进入S05或产品实现。 失败/副本与原件边界均保留。

## 9. P1本会话用户裁定（2026-11-10）

- **DEC-P1-01**：默认复现已证原件行为，明确提示科学/数据风险；修正行为须显式选择。只适用于有实际证据的分支；未知不补造，科学/数据失败不洗成正确。D02要求失败保留当前作品、宿主隔离和真实重开，因此原件崩溃不得连带宿主，产品保存不得故意丢其资源；差异必须显式登记。
- **DEC-P1-02**：默认工程保留原生 `.vesta` 加资源文件，设计完整提交与冲突恢复；没有批准改为默认单文件 `.vcraft`。
- 产品路线/目录与协议为Codex设计选择，不冒称用户逐项确认。[P1架构](superpowers/specs/2026-11-10-vesta-p1-design.md)、[接口](superpowers/specs/2026-11-10-vesta-interfaces.md)、[CA接线](superpowers/specs/2026-11-10-vesta-ca-wiring.md)、[模块计划](superpowers/plans/2026-11-10-vesta-modules.md)已经形成设计稿，整体经dsh裁定仍partial；P1.1继承和P1.3设计级接受。科学/codec公共payload及完整高级计划继续阻止P1.2/P1.4和整体接受；CA真实运行单列另授权，不自动实施。

## 历史：10. P1实际部分交付（2026-11-10）

P1经真实dsh独立复审为partial：P1.1完成，P1.2–P1.4与整体P1未完成。Rust/WASM单Worker＋TS/React/Three.js、同内核原生CLI及未来VestaCraft/app目录是已接受的路线级设计；尚无产品目录/代码或实际worker。两项用户裁定§9保持。

[设计](superpowers/specs/2026-11-10-vesta-p1-design.md)、[接口](superpowers/specs/2026-11-10-vesta-interfaces.md)、[CA接线](superpowers/specs/2026-11-10-vesta-ca-wiring.md)、[模块计划](superpowers/plans/2026-11-10-vesta-modules.md)和[实际复审](../output/vesta-study/20261108-113622/reviews/20261110-p1-design/review-result-effective.json)为当时接续。科学/codec公共payload、CA桥接设计、全模块精确计划当时继续留P1；不以未定接口凑完整计划，不自动进入产品施工/部署/模型builder。

## 11. P1.3验收口径裁定（2026-11-14）

**DEC-P1-03，用户原话：“按设计级验收；真实 worker／Check／launcher 验证另行授权”。** P1.3本阶段验收的是锁定真实系统的可独立接手设计：精确输入身份/摘要、公开API/构建合同、迁入、kind、launcher、队列/Check时机及恢复；不能要求本卡越权运行以满足这项设计条件。真实new_project/copy/worker/Check/launcher、故意失败与恢复另列后续有界实施验证，需单独授权，当前仍未运行。该裁定不改变P1.2/P1.4或整体P1的partial，不授予P2–P5、产品、部署或模型builder权限；整改设计的独立接受仍由dsh判断。


## 12. P1.3设计级接受的实际回写（2026-11-14）

用户已带回dsh的R2独立复审，复核确认C4的设计闭合，并明确指令“把P1.3设计级接受回写权威文档和机器状态”。因此P1.3设计项记完成/接受，与P1.1共2项接受；P1.2/P1.4和整体仍partial。依据及被审C4 SHA在[R3接受记录](../output/vesta-study/20261108-113622/reviews/20261114-120337-p1-design-r3/P1.3-design-acceptance.json)，独立复审原文按字节归档，不修改其结尾措辞；本段按DEC-P1-03与用户本次指令澄清设计完成和运行验证的区别。

本轮D5/I7/C5的N1–N7/C4口径整改是待dsh增量复核的新稿，不能声称已独立接受。真实prepare/new_project/copy/worker/Check/launcher、故意失败与恢复仍未验证、未授权，另行发起有界实施验证才能运行；不因P1.3设计接受进入P2–P5、产品实现、部署或模型builder。


## 13. R4复审残留与证据保真（2026-11-14）

P1.3 C4设计级接受仍由用户授权记录承载；真实dsh R3机器结果单独作为review_result，其中C5为有条件通过。R4不改原文或把用户授权后的true写成审稿原始字段；当前D6/I8/C6待复核。N3独立输入/root/批准表/安装receipt合同补齐、生产批准表为空，D5§9历史状态与当前接受分开。P1.2/P1.4和整体partial，真实运行仍未授权/未验证。


历史R5主体设计续办：D7/I9/C6，资料rev30；31个有界操作、85格式路由/6类字段政策/28入口绑定、15项具体资格阻塞。P1.2/P1.4/整体partial，P1.3仅C4设计接受，C6增量待审；不进入P2–P5、产品实现或运行验证。证据与手动复审入口：[R5](../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/README.md)。


历史R6同卡整改：2026-11-14 R6：D8/I10按dsh R5复审整改D1–D14及σ公式，Codex自验待用户手动转dsh。P1.1继承接受、P1.3 C4设计级接受保持，C6原字节及未审增量身份不变；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料rev31；不进入P2–P5、产品实现、部署或运行验证，真实worker／Check／launcher未授权、未验证。 施工回执与手动复审入口：[R6](../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/README.md)。


R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。 [回执/复审入口](../output/vesta-study/20261108-113622/reviews/20261114-161832-p1-design-r8-consistency/README.md)。


## 当前续办：P1-01 F1–F6独审结论登记（rev39）

2026-11-14 P1-01同卡独审结论登记：追加资料rev39，技术版本仍D11/I13/C6；dsh已独立复审通过rev38的F1–F6证据/边界修补，最新独审指针指向该被审包及真实JSON/SHA。本次rev39仅回写结论并Codex自验，O1登记为非阻断报告导航说明，不改旧封存或AGENTS。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受，SG-01…SG-06六项科学缺口不变。02–06未启动，产品和运行验证未授权。

最新[dsh独审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/REVIEW.md) / [真实JSON](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/review-result.json) / [本次登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/RECEIPT.md) / [O1非阻断说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/nonblocking-observations.json)。

O1：旧oracle-bindings.currentVerificationEvidence定位04-full正常模式546项报告；封存只读final为570项，两者均exit0/0失败。final入口为[/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json)。本次只登记该导航边界，不改旧字段、旧报告或封存。独审被审版本是rev38，rev39仅为结论回写且尚无新的独立复审。


## 2026-11-18补充：许可事项与本轮设计完成条件

用户在P1-06整体设计收口时明确“先不管许可的事，先完成功能”（[DEC-P1-05](任务卡/阶段1-架构与施工设计/决策确认单.md#dec-p1-05--许可后置先完成本轮功能设计2026-11-18)）。当前功能设计不以取得参数表/原件发布文件的再分发许可为前置；原始数据/程序的来源、版本、SHA与许可现状继续如实登记，许可事项后续发布前处理。完整功能、已证原件默认行为、科学/格式未知的真实证明要求保持。本轮仍只设计，整体作者设计完成后统一整理提审材料，不自动进入实现或运行。

## 历史 P1-06 全功能设计交付（rev64 / D33-I32-C6 / IP4，存在复审漏项）

P1-06 全功能作者设计已完成，rev64 / D33-I32-C6 / IP4，整体待用户手动 dsh 独立复审。科学六域、FS十一项、偏好与动画历史、资产及四个外算流程已有具体类型、来源规则与完整输出先验；同生命周期 CLI 由保留的 NS/1 明确扩展为 NS/2。15 实施单元、56 依赖步骤覆盖原 254 功能和 85 格式路由。当前没有待补且会改变公共契约的来源/类型字段；未来产品与原件运行资格另列。P1 接受仍为两项接受、两项未接受，整体未接受；没有产品实现或运行。

[06 最终回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/RECEIPT.md) / [统一接口与 CLI](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/composition-design.md) / [完整实施计划](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/implementation-readiness.json) / [一次性手动 dsh 待审包](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/DSH-REVIEW-HANDOFF.md)。本节为当前同卡后继；下文/前文各 rev60–63 及更早的“当前、partial、尚缺”描述均保留为当时记录，原字节证据与旧 baseline 不改。新合同是明确的设计后继，不追认旧卡、旧 177 定义、bounded protocol3、NS/1 或 C6 已支持新字段。

用户 DEC-P1-05 已确认“先不管许可的事，先完成功能”：许可不再阻塞本轮功能设计；保留资产来源与条款身份，不推定已取得授权。当前只做设计，继续原卡累计预算、权限、费用、次数及停报；完整旧账本未核，不记零累计。原件执行、产品运行、worker、Check、launcher、模型调用均未发生。dsh 由用户手动发起，本轮不代发或建立复审会话。

全功能1:1及原科学/格式范围不缩减。遵守 DEC-P1-01：默认原件行为与明确风险，修正模式须显式选择；原件无操作/不安全分支必须有具名结果，不能虚构原件能力。保存默认仍为原生 .vesta＋资源。现行平台 D4-33 为两种软件布局，切换保留同页/Engine/工程/任务/草稿，真实受控关页后才转 CLI；旧三模式文字为历史。科学规则、接口、工程路径是 Codex 设计选择，不伪称新增用户裁定。

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
