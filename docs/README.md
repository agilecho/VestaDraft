# Craft 软件方法与接入指南

**当前源码入口（2026-11-19）**：用户明确指定代码直接放在本项目 `/Users/agilecho/code/Draft`；应用在 [app/](../app/README.md)、引导工具在 `tools/ca-bootstrap/`，与文档使用同一 Git 仓库。`output/` 保留原证据。[开发与构建](开发与构建.md) / [用户目录裁定](任务卡/源码项目目录调整.md)。

<!-- P1-CURRENT-STATUS BEGIN -->
当前状态：rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。该接受仅属P1设计交付。用户随后已授权按独立任务卡继续施工与验证；当前实现状态见下方任务组织，P1接受范围不扩大。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。
<!-- P1-CURRENT-STATUS END -->

**当前任务组织（2026-11-19）**：P0研究与P1当前设计已收口；现行25卡v4/v3及共同约定v3已有dsh独立审卡通过，精确SHA接受见[登记](/Users/agilecho/code/Draft/docs/任务卡/后续任务卡规则对齐接受登记.md)。用户已授权Codex逐卡代建独立施工会话；P2-01已按用户要求在main完成独审整改，双目录各25/25自验通过，待用户手动dsh复审后继版本；目录一致性独审通过，P2-02已交partial、冷构建封顶待裁定，其余按依赖接续，四阶段整体UI E2E仍未执行。[任务总览](/Users/agilecho/code/Draft/docs/任务卡/README.md) / [执行预检](/Users/agilecho/code/Draft/output/vesta-task-handoffs/20261118-115219-rules-acceptance-execution-preflight/README.md)。原接受/封存及原累计预算保持，下方旧进度按历史时点理解。

**本机存储（2026-11-19更新）**：Craft Anything参考源码为`/Volumes/T9/Development/Sources/Artcraft/craft-anything`；用户明确指定本项目的源码和文档均在`/Users/agilecho/code/Draft`维护。`output/`及原封存继续保留，旧证据原路径不改，当前源码按[目录裁定](任务卡/源码项目目录调整.md)在本仓库维护。

本目录用于指导怎样研究一款软件、拆出核心能力、用 AI 辅助实现，再交付成类似 WordCraft 的产品：**可以在 Web 中使用，Agent 通过正式接口操作当前页面，用户实时看到效果，作品保存在个人空间，关闭后能重开继续编辑。**

它按“人可以学习全流程、AI 可以按需读取并实施新软件”的目标组织为工程教材和执行手册。七篇方法建立全貌，技术专题解释实现细节，贯穿实战把接口与步骤串起来，模板用于新项目落地。具体掌握程度由完整用户链验证。

2026-11-05 按这一目标重整。主线按工程方向组织；2026-10 的 ArtCraft 仓库分析保留原文，放到历史分析中。Craft Anything 的项目分析和 IntegrateCraft 接入评估作为研究工具参考保留。

这里的方法和评估资料不替代具体任务卡。当前 VESTA 范围、状态和接续点以专用要求、Study 计划及 HANDOFF 为准；角色与独立复审按根目录 [AGENTS.md](../AGENTS.md) 的最新用户裁定执行。

## 当前目标软件

[VESTA 全功能重写要求](VESTA全功能重写要求.md)记录 2026-11-08 用户确认的目标：严格按照 Craft Anything 的方法和步骤，1:1 重写本地 VESTA 的全部功能，并满足[接入技术要求](接入技术要求.md)。该项目的已确认范围以专用文档为准，阶段切片只安排施工顺序。

执行资料：[总体实施计划](superpowers/plans/2026-11-08-vesta-full-rewrite.md)、[Study 阶段执行计划](superpowers/plans/2026-11-08-vesta-study.md)、[逐步引用的资料索引](VESTA资料索引.md)。每步注明现有资料和前置产物，按计划顺序接续。

研究进度与下一动作见 [HANDOFF](VESTA研究/HANDOFF.md)；已取得的 [功能目录](VESTA研究/FUNCTIONS.md)、[格式研究](VESTA研究/FORMATS.md)、[样本身份](VESTA研究/SAMPLES.md)、[原生未知项](VESTA研究/UNKNOWNS.md) 均链接到本轮证据，未验证项保持明确。

历史R5状态（rev30时点，当前见文首）：2026-11-14 R5：P1.1继承接受、P1.3 C4设计级接受保持，2接受/2未接受；P1.2、P1.4和整体P1仍partial。D7/I9补31个有界领域操作的数据边界、逐单元交接、85格式入口的字段政策及15项具体补证阻塞；不是完整功能冻结或产品实现。C6保持R4原字节、其增量仍待独立复核。资料基线rev30；真实worker／Check／launcher未授权、未验证，不进入P2–P5、产品实现、部署或模型builder。

## 按目的阅读

| 要回答的问题 | 文档 | 读后应得到什么 |
|---|---|---|
| AI 接到一个新软件目标后怎样开始、读取和执行 | [AI 执行入口](AI入口.md) | 按阶段选教材、产出方案、实施与交接 |
| 从空仓库到可交付软件怎样走完整遍 | [贯穿实战](实战/从空仓库到Agent实时Web软件.md) | 一致的模型、命令、人机操作、保存和验收实例 |
| 学会了哪些技能，如何检验有没有遗漏 | [技能覆盖与练习](技能覆盖与练习.md) | 通用/领域技能的产出和练习通过条件 |
| 一款软件做到什么才算 Craft 成功 | [目标与端到端路线](方法/01-目标与端到端路线.md) | 用户结果、边界、交付链与分阶段关口 |
| 怎样研究目标软件，决定先做哪些功能 | [软件研究与功能边界](方法/02-软件研究与功能边界.md) | 证据、核心用户链、领域差异和可实现范围 |
| 内核、命令、UI、CLI、MCP 怎么分层 | [命令内核与多端架构](方法/03-命令内核与多端架构.md) | 同一业务逻辑、命令契约和状态归属 |
| Agent 怎么改到用户眼前的页面 | [Web 化与实时交互](方法/04-Web化与实时交互.md) | 页面接口、命令往返、就绪与渲染确认 |
| 怎么让 Agent 会开发、会使用、会自纠 | [Agent 工具与开发协作](方法/05-Agent工具与开发协作.md) | 软件使用工具、开发资料和工作交接方式 |
| 工程怎么保存、冲突怎么处理、如何重开 | [工程文件与保存恢复](方法/06-工程文件与保存恢复.md) | 文件身份、宿主确认、多文件工程和异常恢复 |
| 用什么证据证明做成了，怎样交付 | [测试验收与构建交付](方法/07-测试验收与构建交付.md) | 分层验证、真实用户链、锁定构建和运行指纹 |
| 怎么接进 IntegrateCraft，哪些现成实现能复用 | [IntegrateCraft 接入与复用边界](接入/01-IntegrateCraft接入与复用边界.md) | 当前接缝、最小增量、平台治理和接入顺序 |
| 开始研究下一款软件时写哪些材料 | [单款软件 Craft 方案模板](模板/单款软件Craft方案模板.md) | 可复制的研究与设计模板，填写后再交发卡方 |
| Craft Anything 能做什么、成熟到什么程度 | [项目分析](参考/CraftAnything-项目分析.md) | 引擎、证据、重建能力和验证边界 |
| Craft Anything 怎么辅助本项目 | [IntegrateCraft 接入评估](参考/CraftAnything-IntegrateCraft接入评估.md) | 分析 MCP 的优先路径、重建限制与补齐项 |
| 哪些结论是事实，哪些是建议，旧资料哪里过时 | [依据与差异登记](参考/依据与差异登记.md) | 源码依据、决策入口和历史纠偏 |

## 技术方向教材

每篇给出输入与交付、实现步骤、关键取舍、常见错误和验证练习。已有软件按真实接口复用，新软件按这些方向设计。

| 方向 | 专题 |
|---|---|
| 数据结构、单位、事务和撤销 | [领域模型与编辑算法](专题/01-领域模型与编辑算法.md) |
| 布局、交互、输入法、键盘和无障碍 | [前端交互与可访问性](专题/02-前端交互与可访问性.md) |
| Canvas/SVG/GPU、字体、资源、Worker 和性能 | [渲染图形与性能](专题/03-渲染图形与性能.md) |
| 导入导出、codec、兼容、容器和迁移 | [文件格式与兼容工程](专题/04-文件格式与兼容工程.md) |
| 原生依赖、WASM、页面 API 与浏览器桥 | [原生内核到 Web](专题/05-原生内核到Web与WASM.md) |
| 模型、异步任务、费用、停止和恢复 | [模型接入与长任务](专题/06-模型接入与长任务系统.md) |
| 样本、真实 E2E、视觉检查和定位 | [自动化测试与故障诊断](专题/07-自动化测试与故障诊断.md) |
| 锁版本、制品、部署、升级与回滚 | [构建发布与升级维护](专题/08-构建发布与升级维护.md) |
| 排版、公式依赖、幻灯片和办公格式 | [文档表格与演示](专题/09-文档表格与演示软件.md) |
| 场景、几何、约束、拓扑、节点和资产 | [图形 CAD 三维与材质](专题/10-图形CAD三维与材质软件.md) |
| 时间基、信号/帧、编解码和合成 | [音频视频与时间线](专题/11-音频视频与时间线软件.md) |
| 结构化数据、语言服务、构建运行与预览 | [数据工具与编程工作台](专题/12-数据工具与编程工作台.md) |
| 权限、插件、执行隔离和多人协作边界 | [安全扩展与协作边界](专题/13-安全扩展与协作边界.md) |
| 目标观察、证据、行为规格和重建对照 | [软件研究与重建验证](专题/14-软件研究与重建验证.md) |

新项目还可复制 [文档骨架](模板/新项目文档骨架.md) 并填写 [命令与页面契约](模板/命令与页面接口契约.md)。这些资料将方法转成实际工程输入，不要求把整个目录一次塞进模型上下文。

## 贯穿所有方向的交付链

```mermaid
flowchart LR
    A[研究目标与核心用户链] --> B[领域内核与正式命令]
    B --> C[Web 编辑器与页面接口]
    C --> D[Agent 操作当前页面]
    D --> E[用户看到真实变化]
    E --> F[个人空间保存与重开]
    F --> G[内容与画面独立验收]
```

功能广度按这条完整链逐步扩展。第一次就选一条有业务价值的用户链贯穿；不要先堆几百个命令，再把页面接口、文件保存和内容验收留到最后。

## 如何使用旧资料

旧索引曾记载 `历史分析/2026-10-ArtCraft/README.md`，但本次核验在当前工作区未找到该目录，暂不把它作为可访问资料入口。旧文中的版本、软件数量、页面接口缺口、通道设想和验收状态只代表当时事实；当前差异以现存的[依据与差异登记](参考/依据与差异登记.md)为入口。

例如，旧文说只有 EffectCraft、FilmCraft 有页面 API；当前 WordCraft 等已具备平台接缝。旧文设想用新事件通道，当前生产代码已使用认证 Fetch 路由与页面队列往返。实际接入沿用现有实现，见 [差异登记](参考/依据与差异登记.md)。旧文中的 09 篇在整理开始时就不存在，本次未补造。

## 本轮范围

本轮只整理和撰写文档，保留已有施工代码、分支、运行资源和历史证据。新文中的接口形状、推进阶段和优先级属于方法建议；未核实的能力单列为前置研究，不据此启动构建、部署或真实模型调用。

VESTA最新S04接续（063443包）：本轮仅接续S04触发补证：新增90条原生至1740磁盘派生索引；254稳定ID/78样本，90条入口级待核。S04.2仍partial-unchecked，S04.3/.4既有接受保持；13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 入口见[HANDOFF](VESTA研究/HANDOFF.md)。

VESTA最新Objects/焦点续证：本轮继续S04 Objects/焦点：新增45条原生至1785磁盘派生索引，254稳定ID/78样本/12头保持，87条入口级待核。S04.2保持partial-unchecked，S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 [接续](VESTA研究/HANDOFF.md)。

VESTA最新alpha路由：本轮继续S04 alpha事件路由：新增9条原生至1794磁盘派生索引，254稳定ID/78样本/12头/87入口级待核保持。S04.2保持partial-unchecked；S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 [接续](VESTA研究/HANDOFF.md)。

VESTA最新alpha运行时：本轮S04 alpha入口已有原件阳性：同PID激活后28×128浮窗与AXSlider192；诊断副本实测点击门槛/显示链。新增32原生至1826派生磁盘索引，254ID/78样本/12头/87entry-only保持。dsh复核pass、无硬阻断；S04.2按入口/条件登记范围接受，S04整卡收尾，Study14完成/15未完成。未进入S05或产品实现。 [接续](VESTA研究/HANDOFF.md)。

VESTA本轮Study收束报告：[STUDY-REPORT](VESTA研究/STUDY-REPORT.md)，当前停在S08研究交付，不自动施工P1。


历史R5主体设计续办：D7/I9/C6，资料rev30；31个有界操作、85格式路由/6类字段政策/28入口绑定、15项具体资格阻塞。P1.2/P1.4/整体partial，P1.3仅C4设计接受，C6增量待审；不进入P2–P5、产品实现或运行验证。证据与手动复审入口：[R5](../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/README.md)。


历史R6同卡整改：2026-11-14 R6：D8/I10按dsh R5复审整改D1–D14及σ公式，Codex自验待用户手动转dsh。P1.1继承接受、P1.3 C4设计级接受保持，C6原字节及未审增量身份不变；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料rev31；不进入P2–P5、产品实现、部署或运行验证，真实worker／Check／launcher未授权、未验证。 施工回执与手动复审入口：[R6](../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/README.md)。


R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。 [回执/复审入口](../output/vesta-study/20261108-113622/reviews/20261114-161832-p1-design-r8-consistency/README.md)。


## 当前续办：P1-01 F1–F6独审结论登记（rev39）

2026-11-14 P1-01同卡独审结论登记：追加资料rev39，技术版本仍D11/I13/C6；dsh已独立复审通过rev38的F1–F6证据/边界修补，最新独审指针指向该被审包及真实JSON/SHA。本次rev39仅回写结论并Codex自验，O1登记为非阻断报告导航说明，不改旧封存或AGENTS。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受，SG-01…SG-06六项科学缺口不变。02–06未启动，产品和运行验证未授权。

最新[dsh独审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/REVIEW.md) / [真实JSON](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/review-result.json) / [本次登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/RECEIPT.md) / [O1非阻断说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/nonblocking-observations.json)。

O1：旧oracle-bindings.currentVerificationEvidence定位04-full正常模式546项报告；封存只读final为570项，两者均exit0/0失败。final入口为[/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json)。本次只登记该导航边界，不改旧字段、旧报告或封存。独审被审版本是rev38，rev39仅为结论回写且尚无新的独立复审。

## 04复核整改后继（rev57 / D27-I29-C6）

> 历史交付时状态；随后R57已独审通过F1–F6技术增量，登记及当前R1–R3修补以文末rev58为准。历史“待dsh”不表示本次技术增量仍未审；R3正文已按后继补清。

04按dsh复核完成F1–F6设计整改，rev57/D27-I29-C6；rev56静态证据独审通过，整改增量仅作者自验、待用户手动dsh。当前notebook全部已开文档的原生作用范围、逐Doc配置摘要、汇总优先级和事件处理器副作用已澄清；完整偏好输入资格与实际动画撤销重做分组仍缺，04整卡partial、未接受。

F1：原生身份链为canvas+0x568→WinManager*、manager+8→vxNotebook*、notebook+0x3c0/+0x3c8→本notebook全部已开文档向量；不推及其它notebook/窗口或账号。raw manipulation为1或>=4时，有效Doc当前Scene清选择返回>=1才逐Doc请求PushHistory(2)。适配器单Doc/明确授权集合是对原生范围的有意收窄。rev56“ownerDoc/ownerDocument”表述已由本段修订，冻结旧包和复审原件不改。

F2：`selectionProfileDigest`必须放在每个`capturedDocuments`条目，允许不同文档使用不同profile；请求级同名字段禁止。逐Doc权限/资格由服务器预检，不接收客户端permission/qualified声明。documentEnvelope、schema、闭合值、真实构造请求与有限例一致。[后继合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/contracts/history-evidence.contract.json) / [schema](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/contracts/manipulation-history.schema.json)。

F3：集合结果按顺序计算：任一unknown→unresolved；无unknown且failed+committed→partial；有failed但无committed（含failed+unchanged）→failed；全unchanged含空集合→unchanged；其余至少有committed、仅committed/unchanged→complete。逐Doc失败/unknown保留已提交结果，不宣称跨Doc原子性或自动整批回滚/重放。

F4：Mode0主选择0写raw0、隐藏次选择、停运行timer并Enable(1)；非0显示次选择并写raw=secondary+1，raw2停运行timer，其它已资格raw仅未运行时start(pref+3a0,false)，此分支不调用Enable。Modes1同样写raw=secondary+1，沿同timer规则且Enable(raw!=3)，raw1/raw2启用、raw3禁用；Modes2写axis selector并重置向量(1,0,0)。事件处理器副作用与适配器停止/屏障/不自动恢复策略分开；named映射仍为跨源推断，未测实际节拍或历史分组。

F5：[数值子集schema](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/contracts/preferences-input.schema.json)只覆盖animationStepDeg、animationIntervalMs、historyLimit、rasterTessellationFactor、isosurfaceN五项。其余textFont、三artifact、openTarget、startupBondSearch、hardwareAntialias七项各自沿AL3/05字体资产、宿主和能力资格，去向见[偏好合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/contracts/preferences-input.contract.json)；12字段矩阵不等于全字段wire准入或全原生资格。

F6：仅解码既存rev55 SetValue指令bytes：ADRP@0x100016e00+ADD@0x100016e04→0x1004d4f28（%.*f），ADRP@0x100016e28+ADD@0x100016e2c→0x1004d4f1c（%g），连同BL到Printf重载核对。候选double、typed存储、显示与实际生效读回保持分离；locale/FPCR/完整词法溢出和原件实际格式结果未验证。

本轮新增原件读取/绑定/GUI/动态/网络/额外模型均0，未运行会读原件的旧复审检查器。原复审含当前原件离线otool/nm与旧字节比较，不能笼统记为“未新读原件静态”；未target绑定/GUI/运行是另一边界。只追加补注，不改R或裁定权限违规，不重置累计预算/费用/次数/停报。原8问题ID、AL-02/AL-04、03rev48待整卡独审及旧47子项、01/02/05、SG六/FS11、C4接受/C6未审、P1两接受两未接受、06未启动均保持。未实现产品、未迁移177定义/productprotocol3、未自动转审。

[04整改回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/RECEIPT.md) / [整改设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/design.md) / [最小剩余输入](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/collection-plan.md) / [手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/DSH-REVIEW-HANDOFF.md) / [复审边界补注](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/boundary-errata.md)。

## 04独审登记与R1–R3修补（rev58 / D28-I30-C6）

04本轮登记rev57 F1–F6技术整改独审通过（53/53），并完成R1–R3设计修补；当前rev58/D28-I30-C6，本轮增量仅作者自验、待用户手动dsh。04整卡仍partial、不接受，AL-02/AL-04未闭合，C6未审，未实现产品或启动运行/06。

登记范围仅为[rev57 dsh报告](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-053126-p1-04-rev57-dsh-review/REVIEW.md)的F1–F6技术整改增量：53/53，182例零失配、85种聚合组合一致，作者及复审411/0、七负控检出；原文correctionVerdict仍为pass-with-minor-doc-residual。未扩大为04整卡、整个接口、C6、其它P1或产品/运行接受。本轮R1–R3尚待用户手动dsh。原8问题ID、AL-02/AL-04、03rev48另案待整卡独审/旧47子项、01/02/05 partial、SG6/FS11、C4接受/C6未审、P1两接受两未接受、06未启动保持。

R1：当前摘要/导航统一到rev58/D28-I30-C6；原生范围为当前notebook全部已开文档，单Doc/明确授权集合是适配器有意收窄。STATE的P1.scope_clarification、P1.items.P1.2/P1.4及P1.bookkeeping_round已具名修正；历史rev56/57记录保留并标明由本节覆盖。新校验只放行这些精确字段及04当前指针/登记，不豁免其它P1字段、其它卡或user_decisions。

R2：新检查器按sorted(others)输出七项非数值排除检查，另核其它输出循环的顺序。相同冻结输入、final阶段及同一解释器用两个不同PYTHONHASHSEED实跑，逐字节stdout SHA/检查顺序/内容对照见[确定性记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-054254-rev57-review-registration/determinism.json)。不修改旧checker/stdout/seal。

F4：Mode0主选择0写raw0、隐藏次选择、停运行timer并Enable(1)；非0显示次选择并写raw=secondary+1，raw2停运行timer，其它已资格raw仅未运行时start(pref+3a0,false)，此分支不调用Enable。Modes1同样写raw=secondary+1，沿同timer规则且Enable(raw!=3)，raw1/raw2启用、raw3禁用；Modes2写axis selector并重置向量(1,0,0)。事件处理器副作用与适配器停止/屏障/不自动恢复策略分开；named映射仍为跨源推断，未测实际节拍或历史分组。

R3补注：rev57模型native_mode0已写secondary='show'，但rev57合同primaryNonzero并无显式secondary/show字段，R57称“合同与模型均覆盖”不够精确。本后继合同显式增加secondary='show'，正文同步Mode0非0显示及Modes1 secondary+1，并核模型/合同一致；只消费已保存证据，不新增原生行为或取证。R57原报告不改。

rev57 sealed189产物、18作者run及复审另加的runs/20261118-053226-180186-final一并保全；该复审run仍非原作者seal成员。rev56及两份dsh目录/旧失败/基线原字节保留；本轮新增原件读取、target绑定、GUI、动态、网络、额外模型均0，不运行会访问原件的旧R56/E56检查器，不重置预算/费用/累计次数/停报。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-054254-rev57-review-registration/RECEIPT.md) / [登记与R1–R3设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-054254-rev57-review-registration/design.md) / [手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-054254-rev57-review-registration/DSH-REVIEW-HANDOFF.md)。


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

权威顺序仍是现行需求→技术方案→界面→任务卡。原唯一 functions.json 不改，本轮的派生覆盖检查仅为06自验。

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
