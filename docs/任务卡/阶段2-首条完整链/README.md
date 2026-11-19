# 阶段2 · 首条完整链

**源码位置（2026-11-19用户明确指定）**：代码直接存入本项目 `/Users/agilecho/code/Draft`，应用在 [app/](../../../app/README.md)、工具在 `tools/ca-bootstrap/`；已迁入应用147文件和工具来源868文件并逐字节核对。日常源码在本仓库维护；原 `output/` 树、包和账目保留。旧卡路径按[目录裁定](../源码项目目录调整.md)解释，正式 CA 工作区仍未创建，各卡接受/停报不变。

## 给你看的

第一条可编辑、保存、导出、关页CLI续改及全新页面重开的晶体工程链，并有真实CA检查。

当前：现行卡面已获dsh独立审卡通过。**P2-01已在main完成整改，双目录各25/25自验通过，待用户手动dsh复审；P2-02已交partial、冷构建封顶待用户裁定；整体UI E2E未执行、阶段未结束。** 用户已授权Codex逐卡代建独立施工会话并按真实依赖推进。P0/P1既有接受与封存、全部历史失败和累计资源保持；卡面接受不替代施工独审。

| 编号 | 任务名称 | 一句话说明 | 当前状态 | 卡面入口 |
|---|---|---|---|---|
| P2-01 | I33共享合同生成与可复验源码包 | 交出可离线独立复验的I33共享合同源码包，供后续实现接入。 | main整改提交0818939；双目录各25/25自验完成，待后继独审 | [卡面](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/01-I33共享合同生成与可复验源码包.md) |
| P2-02 | 原生工作区与研究输入迁入 | 由正式API创建、输入逐字节可核的原生工作区，并把已审源码包迁入真正的产品实现区。 | 源码/依赖缓存已封装；完整工具未编译通过，冷构建3轮用完、待同卡时长裁定 | [卡面](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/02-原生工作区与研究输入迁入.md) |
| P2-03 | 晶体命令内核与原生保存基础 | 拿到能真实创建晶胞、编辑位点、撤销并输出可独立读回工程字节的同内核原生程序。 | v3卡面独审可发放；未施工，待会话安排 | [卡面](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/03-晶体命令内核与原生保存基础.md) |
| P2-04 | 当前Web编辑与锁定双端交付 | 用户和正式页面工具能在同一个Web工程里连续修改晶胞，看到实时变化，并得到可重建的Web与Linux CLI制品。 | v3卡面独审可发放；未施工，待会话安排 | [卡面](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/04-当前Web编辑与锁定双端交付.md) |
| P2-05 | 个人空间保存重开与CLI续作 | 在隔离宿主中把实际晶体工程保存到本人空间，导出、关闭，再由CLI续改并在全新页面重开继续编辑。 | v3卡面独审可发放；未施工，待会话安排 | [卡面](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/05-个人空间保存重开与CLI续作.md) |
| P2-06 | 首链CA检查与故障恢复 | 让已经可用的首链在真正的Craft Anything worker中接受检查，保留差异、故意失败和恢复证据。 | v3卡面独审可发放；未施工，待会话安排 | [卡面](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/06-首链CA检查与故障恢复.md) |

## 给执行者看的

当前整改后继：main提交0818939，[同卡续办](P2-01-独审整改续办.md) / [施工回执](/Users/agilecho/code/Draft/output/vesta-p2-01/20261119-review-corrections/RECEIPT.md) / [手动dsh复审材料](/Users/agilecho/code/Draft/output/vesta-p2-01/20261119-review-corrections/DSH-REVIEW-HANDOFF.md)。后继源包151源文件+清单、双目录25/25、19生成项完整字节一致；原fetch/compile额度保持，未新增编译。下方首次独审回传和attempt3材料按历史时点保留。

2026-11-19已收到用户手动带回的dsh报告，目录一致性复核通过；P2-01技术结果通过但正式接受暂不通过。B2/B3已在临时副本复现，B1/B4/B5的事实与边界见[再复核记录](/Users/agilecho/code/Draft/output/vesta-p2-01/20261119-dsh-report-recheck/RECHECK.md)。下方attempt3原作者回执、首次转审材料和历史账目保留，不能再将其独审前状态当作当前待审。

P2-01最新[工程回执](/Users/agilecho/code/Draft/output/vesta-p2-01/20261118-132952-attempt3/RECEIPT.md)和[手动dsh材料](/Users/agilecho/code/Draft/output/vesta-p2-01/20261118-132952-attempt3/DSH-REVIEW-HANDOFF.md)已落盘。最终双目录合同各20/20、19生成输出完整集合及字节一致；没有新增获取或编译。累计受控446.291秒，保守已扣4103.174/7200秒；合同9轮累计317.432/1800秒。用户只取消该类硬轮数，获取3/3、编译5/5不增额；原失败与来源/下载限制保留。

P2-02实际源码及765个锁定registry节点的离线缓存已封装，`metadata --offline --locked`禁网通过且锁不变；早期版本文件安全模块8/8、同版控制器六项加一项定向断言通过。完整工具尚未编译成功，公开API临时链、正式创建及源码迁入仍未执行。三轮冷构建已用完：文件模块编译、主盘满中止、采样器异常后由主会话终止残留自有组；原失败与监督缺口保留。构建缓存已逐件核对迁至T9，修复后的驱动及[同卡时长提案](/Users/agilecho/code/Draft/output/vesta-execution/20261118-150143-p2-02-cold-time-proposal/CONTINUATION-PROPOSAL.md)可核，用户尚未裁定，禁止新编译。本轮回执和保护核对已交回，施工会话已idle、已知自有进程组全部退出。全卡冻结已扣8092.783秒、剩余6307.217秒；纯等待不新增机器耗时。正式根仍不存在；源码迁入等待P2-01施工独审，实际prepare保留本卡具体准入。

最新[卡面接受登记](/Users/agilecho/code/Draft/docs/任务卡/后续任务卡规则对齐接受登记.md)及[第一卡施工交接](/Users/agilecho/code/Draft/output/vesta-task-handoffs/20261118-115219-rules-acceptance-execution-preflight/P2-01-EXECUTION-HANDOFF.md)已落盘；不以原卡头待审时态否定精确SHA的已审结论。

先读[共同约定](/Users/agilecho/code/Draft/docs/任务卡/后续施工共同约定.md)、[原型与验收依据](/Users/agilecho/code/Draft/docs/任务卡/原型与验收依据.md)、本卡/最新续办、[HISTORY](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/HISTORY.md)与[决策确认单](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/决策确认单.md)，按真实产物依赖进入；编号不是强制串行，也不授权自动并行。用户已授权Codex逐卡代建独立施工会话并按真实依赖推进；同卡续办沿原会话。dsh由用户手动发起，Codex不代联系；旧预算/权限/费用/次数/停报不重置。

### 阶段整体界面E2E与结束判定

- 收尾负责人：[P2-06](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/06-首链CA检查与故障恢复.md) §4.1。角色/数据/原型、版本/环境、界面完整操作、独立断言、异常恢复和计划入口均在卡内；普通卡只按实际新增/修改/共用影响选择测试。
- 当前状态：**未执行／未通过／阶段未结束**。最终集成版本：尚无；实际UI证据：尚无；运行前锁`<RUN>/inputs/stage-e2e.json`，结果与原始证据在该卡`<RUN>/evidence/stage-e2e/`，交回后填本README具体绝对入口和最终指纹。
- 只有本阶段全部承诺结果和最终集成版本的完整鼠标/键盘界面E2E均通过、取得适用的独审结果，才登记结束。单卡测试、截图、直接API或CA Ready不代替。失败/未跑保持未结束；修复后重跑最终整体UI，未受影响单测不默认全量。
- P2-01只交暂存源码包；P2-02独占正式API创建根；P2-04补完首次U00.lock-inputs；P2-06精确授权未齐前连非CA构建/测试都禁止。

来源：[原接受登记](/Users/agilecho/code/Draft/docs/任务卡/后续任务卡接受登记.md) / [本轮手动dsh材料](/Users/agilecho/code/Draft/output/vesta-task-handoffs/20261118-114548-p2-p5-management-scaffolds/DSH-REVIEW-HANDOFF.md) / [任务总览](/Users/agilecho/code/Draft/docs/任务卡/README.md)。当前命令和测试文件都是未来计划，不是本轮已执行或现存产品脚本。

## 底部简史

P2-01独审整改已在main提交，双目录各25/25自验通过、待后继独审；目录一致性独审通过；P2-02已交partial、等待同卡资源裁定；其余卡未施工。详细过程与原失败保留在HISTORY。

| 日期 | 卡号 | 执行及接受结果摘要 | HISTORY详细记录入口 |
|---|---|---|---|
| 2026-11-18 | P2-01 | attempt3双目录合同各20/20及完整字节复验通过，同版原生/TS通过；自验完成待施工独审，未接受 | [HISTORY](HISTORY.md) |
| 2026-11-18 | P2-02 | 源码/缓存已封装，metadata通过；完整工具未编译成功，冷构建3/3、待同卡裁定；未接受 | [HISTORY](HISTORY.md) |
