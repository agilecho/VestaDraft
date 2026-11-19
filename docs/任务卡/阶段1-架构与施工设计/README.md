# 阶段1 · 架构与施工设计（P1）

<!-- TASK-ALIGNMENT-20261118 BEGIN -->
**当前任务组织（2026-11-18）：P1设计阶段已收口（rev66独审通过，rev67接受登记；D35/I33-r1/IP6，wire/profile I33）。01—05设计需求由06当前后继完成，不再列剩余施工；旧版partial/待审不追认为旧卡独审通过。C4接受、C5条件/C6增量未审及03/04历史范围保持。P2—P5仅编卡，未施工。** 当前后续入口见[任务总览](/Users/agilecho/code/Draft/docs/任务卡/README.md)及[后续约定](/Users/agilecho/code/Draft/docs/任务卡/后续施工共同约定.md)。下方原卡面、复选框及各rev履历保留为其当时施工记录，不作为重新开工队列。
<!-- TASK-ALIGNMENT-20261118 END -->

<!-- P1-CURRENT-STATUS BEGIN -->
当前状态：rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。
<!-- P1-CURRENT-STATUS END -->

更新：2026-11-18。rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

## 给你看的

本阶段要把已经研究过的VESTA，设计成后续实现者能够独立接手的工程方案：复杂结构和科学计算怎样表达、文件怎样保真读写、保存和应用交互怎样衔接，以及如何分步实现和验证全部功能。完成设计仍不会产生可使用的编辑器。

技术路线和工具接线中已经接受的设计保留。原设计工作曾按六个可分别审查的结果组织，现已由06当前后继收口；六卡保留为已执行设计与历史履历。后续资产、平台和运行资格按P2—P5具体消费步骤核实，不作为重开P1的队列，也不随编卡自动获得运行权限。

**历史P1六卡发卡独立复审结论：可发卡。** 用户当时已带回dsh复审并授权回写，三处非阻塞文字建议已修正；该结论只针对原P1卡面的任务组织与交接，不外推为本次P2—P5审卡通过。原六卡已执行，当前设计接受和历史边界如下。

<a id="current-task-state"></a>
### 当前任务状态与接续（资料rev67，设计已收口）

rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。

原01–05卡的历史状态及复审范围继续保留；不自动开卡或启动产品。

### 历史快照：rev63任务接续

当前状态：P1-06同卡CLI续作身份契约设计交回，rev63/D32-I31-C6。NS/1补齐一次serve载入后的实际身份、消息/资源/重试、候选保存与宿主确认、关闭恢复；仅首链有界设计作者完成，待用户手动dsh。15单元/56步骤保留，SG六/FS11/AL02与AL04/资产及运行资格保持；P1两接受两未接受、整体partial。未进入产品实现或运行。

[06本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/RECEIPT.md) / [步骤与首链](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/native-session-design.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/DSH-REVIEW-HANDOFF.md)。

### 历史：04发卡时点（rev48）

04从rev48发卡时未施工，[原接手说明](../../../output/vesta-task-handoffs/20261117-222703-p1-04-issuance/HANDOFF.md)保持；用户随后明确“你新建会话执行04”，当前独立会话已执行。原只发卡声明不阻断后续授权。

### 历史：rev47部分设计交回时点

2026-11-15 P1-03同卡实质静态补证交回：追加rev47，D17/I19/C6，FC/1 evidence-r2。84/291字段组新增91处直接源组件绑定，另核20个冻结独立数值例，7类格式/单位精度规则；207组无新直接组件，已绑组也不等于完整语法。16路由typed参数选定域，m3d由direct-f32纠正为原UI直接32位整数，PRI仅flag0/little32；4旧空子集/81未准入不变。48个现有SG/FS属性给桥处置，64叶有限源独立分区9保留/55损失，两个共享bytes场分别枚举；完整Engine捕获/损失全域仍未证。原生相对资源引用与网格字节绑定，原恢复失败及hash布局/04承诺未证保持。8旧unknown分别列最小缺材，未新跑原件/产品/外算。rev45有界独审继续独立有效，rev46登记与本rev47新增量未独审。03整卡、81完整profile、291逐字段语义/精度、真实往返未接受；01/02/05原partial、SG六缺口/FS11、02rev43增量未审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受保持。04/06未启动，交回后停止。

[03本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/RECEIPT.md) / [实质设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/design.md) / [字段证据](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/field-evidence.json) / [剩余缺件](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/remaining-issues.json) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/DSH-REVIEW-HANDOFF.md)。

### 历史：rev46登记时点

2026-11-15 登记P1-03 rev45（D16/I18/C6）O1–O7有界证据/口径增量独审：partial-design-evidence-boundary-pass-with-0-blocking-findings，blocking=false，另附2项非阻断备注。追加rev46仅登记及文档/证据呈现说明，技术版本仍D16/I18/C6；rev45原封存及rev44接受范围分别保留。03卡接手段标明rev43→rev44历史并改称4个旧空schema子集；负控呈现新增targetedHit/allFailures/coCausedFailures，O3完整列2个失败。291仍路由级库存、81完整profile/逐字段语义精度/损失枚举/真实往返未接受；01/02/05原partial、SG六缺口/FS11、02rev43增量未审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受不变。本rev46登记说明仅Codex自验，未独审；04/06未启动，无产品/新运行授权，交回后停止。

[rev45独审登记](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-081622-p1-03-rev45-review-registration-notes/review-registration.json) / [登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-081622-p1-03-rev45-review-registration-notes/RECEIPT.md) / [两项说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-081622-p1-03-rev45-review-registration-notes/correction-notes.md)。

### 历史：rev45 O1–O7整改交回时（独审前）

2026-11-15 P1-03同卡O1–O7整改交回：追加rev45，D16/I18/C6，FC/1 evidence-r1。登记用户带回的rev44有界设计/来源/保全边界独审通过，原文6项与实列7项及411误写保留说明；当前整改增量仅Codex自验。291组明确为路由级字段清单，未建立字段到span/逐字段语义绑定；补3条读入路由的6份直接fixture及预期，I01改原生vesta；补RAW尺寸元数据来源，明确81候选shape及模板语义/精度未冻，损失枚举完整性未证，状态时间同步。原4子集/81未准入、SG六缺口/FS11项、01/02/05原partial、02rev43增量未独审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受保持。03仍partial，04/06未施工，无产品/新运行授权；交回后停止。

[03整改回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/RECEIPT.md) / [O1–O7处理](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/correction-notes.md) / [用户转交复审登记](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/review-registration.json) / [当前合同与边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/design.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/DSH-REVIEW-HANDOFF.md)。

### 历史：rev44首次FC/1交回

2026-11-15 P1-03格式合同设计交回：追加rev44，D15/I17/C6，FC/1；85原路由身份保持，57条旧未审及28条旧政策完成有界来源字段审计，交291组源字段、逐profile候选选项/失败政策、损失同意与资源身份及85条未来真实内容验收合同。原4首链子集/81未准入保持，完整语法/模型桥、精度、布局/方言/许可和04宿主承诺未闭合，03整卡partial，仅Codex自验待用户手动转dsh。01/02/05原partial、FS11项/SG六项缺口、02rev43增量未独审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受不变。04/06未施工，无产品/新运行授权；交回后停止。

[03回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/RECEIPT.md) / [格式合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/design.md) / [来源和原字节](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/source-audit.json) / [给04/06](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/consumer-handoff.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/DSH-REVIEW-HANDOFF.md)。

### 历史：rev43的02同卡整改

2026-11-15 P1-02同卡O1–O4整改交回：追加rev43，D14/I16/C6，FS/1 evidence-r1；补35个真实投影合同有限例、澄清Patterson平方单位、固定Xcode3.9.6和11个旧WordCraft来源快照并披露2处漂移、将生命周期4个事件例移出准入证明。177定义/协议字节、11项本卡缺口及SG六缺口不改。dsh旧结论只覆盖rev42 partial设计/来源/保全边界，本轮整改增量仅Codex自验、待用户手动转dsh。01/05原partial与接受范围保持；C4接受/C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。03/04/06未施工，无产品或真实运行授权；交回后停止。

[本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/RECEIPT.md) / [整改说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/correction-notes.md) / [复现与漂移](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/reproduction-notes.md) / [下游边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/consumer-handoff.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/DSH-REVIEW-HANDOFF.md)。

### 历史：rev42首次交回

2026-11-14 P1-02设计交回：追加rev42，D13/I15/C6；提出FS/1场与科学接口设计，14组按实际来源和独立预期逐项划界，保留完整生成器、参数/许可、网格/拓扑/投影/盆地等11项具名缺口，02整卡partial，待用户手动转dsh。01/05原partial及真实独审范围不变，SG-01…SG-06开放；P1.1与P1.3 C4设计接受保持，C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。03/04/06未施工；无产品、新运行或新实验，交回后停止。

[02回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-221623-p1-02-field-science/RECEIPT.md) / [下游消费边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-221623-p1-02-field-science/consumer-handoff.md) / [具名缺口](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-221623-p1-02-field-science/remaining-issues.json) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-221623-p1-02-field-science/DSH-REVIEW-HANDOFF.md)。以下rev41段为原登记/授权时点，不回滚旧封存。

#### 历史：rev41登记与02开工授权

以下为rev41登记时点状态；本次02设计开工授权见本节末尾，原登记包的停卡范围不扩作新授权的阻断。

2026-11-14 P1-05独审登记及非阻断勘误：追加rev41，技术版本D12/I14/C6不变；dsh确认被审rev40 AB/1、EX/1、BT/1的partial设计/来源/保全边界准确、无阻断。本轮以独立后继修O1–O4，补未施工卡名单和fontTools复现环境说明，仅Codex自验、说明增量未独审；原施工包、dsh报告及rev40封存不改。05/01继续partial，SG-01…SG-06开放，C4接受/C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。02/03/04/06未施工；不启动下一卡、产品或真实运行，交回后停止。

rev41登记时点的用户授权为独审登记与O1–O4勘误；[登记对象](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/review-registration.json)绑定真实[dsh复审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-213000-p1-05-assets-build-dsh-review/review-result.json)及rev40原稿/SHA。[当前消费说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/consumer-handoff.md)给AB/EX定位/粒度后继，原BT/1和科学缺口照原字节消费；新说明仅Codex自验。

[rev41](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/s01/baseline-rev41.json)承接rev40；[机器状态](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/current-task-status.json)active_card=null、next_card=P1-01仍为同卡未完归属。05状态partial-dsh-design-boundary-reviewed；精确未施工卡为02/03/04/06，不将05混入not-started。原dsh的downstream02to06字段留原文，规范解释另列。

复现：作者rev40入口本次回写前exit0/3969项/0失败；dsh历史178项/0失败保留。当前默认Python3.9.6缺fontTools，复跑173项/172通过/1环境失败，字体另用标准库只读复算通过；原dsh解释器/库版本未记载，不补造。详见[复现说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/reproduction-notes.md)。

[本次回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/RECEIPT.md)；原[rev40回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/RECEIPT.md)和全部旧封存保持，仅历史入口不作当前文档SHA验收依据。

本次后续授权：用户要求对齐任务卡状态并直接新开独立施工会话，下一张执行[02](02-体数据与科学计算接口.md)，从rev41/D12/I14/C6接手且仍只做设计。先整理现有Field/网格/算子字段、原来源与独立预期，消费01的SG/1-r2和05当前消费说明；许可、完整参数、科学语义或独立预期未齐时，只停相关冻结步骤。01/05继续partial，不把代表资产身份当完整科学资格。开工前02/03/04/06未施工，02的新会话负责核实际状态并登记开工；机器next_card=P1-01是本次授权前的同卡未完归属，不作02缺少授权的理由。本次只对齐导航，不改rev41、技术文书或旧封存；[导航与接手回执](../../../output/vesta-task-handoffs/20261114-220822-p1-02/RECEIPT.md)记录新导航身份。

## 原P1四项的真实状态

| 原项 | 当前结果 | 接受范围 |
|---|---|---|
| P1.1 路线与目录职责 | 完成、设计接受 | 既有接受保持；Rust/WASM单Worker、Web、同内核CLI |
| P1.2 领域与程序接口 | 完成、设计接受 | rev66 I33-r1/NS2/SG2/FS2及当前完整合同，旧版本不追认 |
| P1.3 CA执行与检查接线 | 完成、设计接受 | C4接受保持；C5条件/C6增量未审，运行待后续有界实施 |
| P1.4 完整实施计划 | 完成、设计接受 | rev66 IP6的15结果单元/56产物步骤/31域/254功能/85路由 |
| 整体P1 | 设计阶段收口 | rev67登记用户确认，实际产品/运行未开始 |

接受来源为[用户已确认的接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json)；四项及整体全true。01—05旧交付不再作为阶段1当前待施工；本次不新增独审或技术版本，不重置预算/费用/次数/停报。

## 给执行者看的

### 发卡复审与本次文字勘误

[dsh原复审报告](../../../output/vesta-study/20261108-113622/reviews/20261114-161000-p1-task-card-issuance-dsh-review/REVIEW.md)及[原机器结论](../../../output/vesta-study/20261108-113622/reviews/20261114-161000-p1-task-card-issuance-dsh-review/review-result.json)记`issuanceAccepted=true`、可发卡，附ISS-N1–N3三条非阻塞建议。原结论、原发卡清单和被审字节身份保留，不追改原件。

按用户本次指令：01卡明确新契约相对本卡证据目录，03卡明确37输出的22/1/14构成及模块计划表G4的codec指代；上游G4详细节编号留待后续任务。本次另记[勘误回执与前后摘要](../../../output/vesta-study/20261108-113622/reviews/20261114-161846-p1-card-notes-writeback/README.md)。原独审针对原被审版本，本次勘误稿另经Codex自验，不将新SHA冒称已独审；原发卡验证器和独立检查器的旧计数/交付摘要仍属于原稿，不为适配勘误覆盖旧报告。

发卡及文字勘误时点的历史状态：当时P1仍2项接受/2项未接受，R7与C6待独审，六张卡均未施工，未改变机器状态或rev32。此后R7–R9及01的F1–F6已有真实独审交回；当前状态见本页“当前任务状态与接续”及下方队列。

### 历史交回、整改与保留的复审范围

| 既有交付 | 当前判断 | 下一动作 |
|---|---|---|
| R7：D9/I11、rev32，原卡身份/投影勘误 | [dsh R7独立复审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-160100-p1-design-r7-dsh-review/REVIEW.md)确认实质整改，仍有R7-R1–R3文书/交接残留；旧包及rev32保持封存 | 已在R8实质修正；R8复审另有文书/状态残留，由原卡R9接续，R6-R5证明边界保留 |
| R8：D9/I11文书勘误、rev33 | [dsh R8](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163303-p1-design-r8-dsh-review/REVIEW.md)确认原三项实质整改，但另有R8-R1–R3文书/状态残留；旧包及rev33封存 | 原卡R9整改，不把局部通过升级为整体接受 |
| R9：D9/I11、rev34被审快照 | [dsh R9](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-170522-p1-design-r9-dsh-review/REVIEW.md)确认R8-R1–R3闭合；M1–M8非阻塞，整体P1不升级 | [rev35收口回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-171658-p1-r9-closure-rev35/RECEIPT.md)；下一步按01卡新会话接手，不再把R9标待独审 |
| C6相对已接受C4的增量 | C6字节仍为`a688ef07…`，不因此算已审 | 沿原C6待审范围手动复审；保留C4接受。若发现问题，按原卡整改，不把工具运行强加为设计接受条件 |

已有待审不等于重新开卡。上游实际复审若改变具体合同，仅影响消费该合同的步骤；不把历史待审状态设为所有旧证据阅读的总前置。新卡后续交接时须携带最新复审结果，不能以本页覆盖后到的真实结果。

### 六张设计卡的收口归属

| 设计卡 | 当前任务状态 | 历史接受边界与后继 |
|---|---|---|
| [P1-01](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/01-结构群与几何接口.md) | 设计需求已由06后继闭合，退出待施工队列 | rev38 F1—F6修补已审；SG完整设计由06 SG2后继接受 |
| [P1-02](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/02-体数据与科学计算接口.md) | 设计需求已由06后继闭合，退出待施工队列 | rev42有界/43未审原范围保留；FS完整设计由06 FS2后继接受 |
| [P1-03](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/03-格式字段与读写合同.md) | 设计需求已由06后继闭合，退出待施工队列 | rev48整卡未追认独审；当前85路由/格式与接口组合在06后继接受 |
| [P1-04](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/04-保存宿主与应用交互.md) | 设计需求已由06后继闭合，退出待施工队列 | rev57 F1—F6已审/58原范围保留；当前HS/APP/HISTORY后继在06接受 |
| [P1-05](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/05-构建资源与外算合同.md) | 设计需求已由06后继闭合，退出待施工队列 | rev40有界已审/41说明范围保持；AB2/EX3/构建输入设计由06后继接受 |
| [P1-06](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/06-全功能实施计划与P1收口.md) | 设计接受，已收口 | rev66独审通过，rev67登记P1.2/P1.4及整体设计接受 |

后续实施进入P2—P5独立卡，不沿旧01—05重新开工。真实构建/宿主/数值/外算/CA资格按后续消费者步骤验证，不反向变成本次设计接受前置。

### 依赖验收与阶段完成

- 下游消费的是具体合同及被审摘要，不是“上游卡号已做过”。上游partial时，已接受且不受缺口影响的部分可继续；受影响步骤停止并列出所缺字段、预期或许可。
- 01–05各自可交有边界的设计部分成果；卡的自验、dsh接受、原P1单项接受分别记录。关键公共接口仍未闭合时，P1.2或P1.4继续partial，不用拒绝占位类型代替完成。
- P1整体接受仍按[总体计划P1通过条件](../../superpowers/plans/2026-11-08-vesta-full-rewrite.md)：可独立接手、相邻关键接口明确、完整施工与验证合同可执行。P1.3按设计级验收，真实worker/Check/launcher另行授权。
- 验收设计中是否明确未来真实写入、独立读取、往返内容和失败恢复的判据；设计例和检查数不能证明产品已做到。

### 接手入口与维护

先读[共同执行约定](00-共同执行约定.md)、本卡及[决策确认单](决策确认单.md)，再读对应技术材料。原设计文档仍是技术权威，本页负责当前任务组织；[HISTORY](HISTORY.md)保留接受及复审沿革，原始报告保持原字节。

旧[整张续办卡](../../superpowers/plans/2026-11-14-vesta-p1-design-continuation.md)保留原文，其中“未施工”和rev25是发卡时点快照，不能作为当前状态。新卡接管剩余工作的任务组织，R1–R7仍属于原卡历史，同卡整改按原归属续办。

每次实际交回更新本页对应行，附施工回执、dsh结果及被审SHA；历史过程放HISTORY，裁定放决策确认单，详细输出留output。发卡不改旧机器接受状态或研究基线；本次发卡核验见[证据入口](../../../output/vesta-study/20261108-113622/reviews/20261114-154911-p1-task-card-issuance/README.md)。

### 历史：原卡接续R8（rev33时点）

R8时点历史摘要：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

R8时点只续办R7-R1–R3，六张剩余卡均未开工；该轮消费[R8合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-161832-p1-design-r8-consistency/contracts)和最新真实dsh结果。新卡内R7/rev32路径保留为发卡快照，不能回退当时rev33；当前接续见R9。完整新卡施工仍由用户另开独立会话。

### 历史：R9施工与待审（rev34时点，现已闭合）

2026-11-14 R9：仅文书与状态勘误，整改R8-R1–R3并补漏检；D9/I11/C6技术版本、schema、字段身份、shape数学及R6-R5证明边界不变。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev34；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

当前接手材料为[R9](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163802-p1-design-r9-doc-state/README.md)和真实[dsh R8](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163303-p1-design-r8-dsh-review/review-result.json)；六张剩余卡仍为已发卡、未施工，不因原卡文书整改开工。新卡中的R7/rev32路径仅属发卡快照，执行时核当前rev34及实际被审SHA。

### 历史接手：P1-01（rev35时点；当前见本页最新增量）

2026-11-14 R9闭合回写：dsh已确认R8-R1–R3在rev34被审范围闭合，M1–M8登记为非阻塞续办；本轮追加rev35，仅回写结论及接手入口，D9/I11/C6技术版本、契约、数学和R6-R5证明边界不变。P1.1继承接受，P1.3 C4设计级接受保持、C6增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。下一卡P1-01由用户新开施工会话，当前六卡均未施工；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

[01卡](01-结构群与几何接口.md)与[新会话接手说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-171658-p1-r9-closure-rev35/01-HANDOFF.md)是下一入口；[非阻塞续办](非阻塞续办.md)仅在相关内容触及时处理，不额外设前置。R9独审闭合与原卡收口不等于P1主体完成，01–06六行状态均继续为已发卡、未施工。


## 历史：P1-01首次施工增量（rev36，2026-11-14）

2026-11-14 P1-01设计交回：追加rev36，D10/I12增量提出SG/1结构、群与几何公共设计及来源/独立演算；完整14组接口仍有6项精确缺口，本卡partial、待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受。R9闭合仅属被审rev34，历史补档保留；02–06未施工。不进入产品实现或运行验证，真实worker／Check／launcher另行授权。

[施工回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/RECEIPT.md) / [SG/1设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/design.md) / [手动转dsh](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/DSH-REVIEW-HANDOFF.md)。已完成输入保全、选定字段/生命周期设计和可由现有证据决定的独立演算；完整冻结仍partial，详见remaining-issues。历史补档及原接受保持；不把历史卡改待办、不启动02。


### 历史：P1-01后继 rev37

2026-11-14 P1-01设计交回：当前rev37、D10/I12/C6；在rev36部分设计上合入并发用户裁定DEC-P1-04，已按WC01–WC03说明本卡WordCraft分层/依赖适配。SG/1完整14组仍有6项精确缺口，本卡partial、待用户手动转dsh。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受。rev35/36、R1–R9及历史补档保留；02–06未施工，产品/worker/Check/launcher运行未授权未验证。

已消费DEC-P1-04，详见[本卡结构适配](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/wordcraft-structure-adaptation.md)。旧rev36交回摘要保留作本轮中间时点，不改旧baseline；共同约定/决策确认单中的并发用户裁定保留原字节。


## 历史续办：P1-01 F1–F6整改后继（rev38）

2026-11-14 P1-01同卡F1–F6整改：追加rev38，D11/I13/C6；另建SG/1-r2证据后继，修复数学引用、明确角色权威及四条旧表冲突、登记保留AGENTS后继、补13条MODEL/VECTS入口，并补schema范围/语义拒绝例及归档基路径。rev37旧SG/1已获dsh“主体准确、partial、轻微证据整改”结论；本次新增量仅Codex自验、待用户手动转dsh。完整六项科学缺口不变，P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial、2接受/2未接受。02–06未启动，产品和真实运行未授权未验证。

[当前施工回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/RECEIPT.md) / [逐项修补](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/correction-notes.md) / [手动转dsh](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/DSH-REVIEW-HANDOFF.md)。同卡授权内续办，不重新发卡、不启动02；原六项科学冻结问题继续保留。旧独审只覆盖rev37前稿，新修补稿待用户手动复审。


## 历史：P1-01 F1–F6独审结论登记（rev39）

2026-11-14 P1-01同卡独审结论登记：追加资料rev39，技术版本仍D11/I13/C6；dsh已独立复审通过rev38的F1–F6证据/边界修补，最新独审指针指向该被审包及真实JSON/SHA。本次rev39仅回写结论并Codex自验，O1登记为非阻断报告导航说明，不改旧封存或AGENTS。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受，SG-01…SG-06六项科学缺口不变。02–06未启动，产品和运行验证未授权。

最新[dsh独审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/REVIEW.md) / [真实JSON](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/review-result.json) / [本次登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/RECEIPT.md) / [O1非阻断说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/nonblocking-observations.json)。

O1：旧oracle-bindings.currentVerificationEvidence定位04-full正常模式546项报告；封存只读final为570项，两者均exit0/0失败。final入口为[/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json)。本次只登记该导航边界，不改旧字段、旧报告或封存。独审被审版本是rev38，rev39仅为结论回写且尚无新的独立复审。


## P1-02实际设计交回（rev42）

2026-11-14 P1-02设计交回：追加rev42，D13/I15/C6；提出FS/1场与科学接口设计，14组按实际来源和独立预期逐项划界，保留完整生成器、参数/许可、网格/拓扑/投影/盆地等11项具名缺口，02整卡partial，待用户手动转dsh。01/05原partial及真实独审范围不变，SG-01…SG-06开放；P1.1与P1.3 C4设计接受保持，C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。03/04/06未施工；无产品、新运行或新实验，交回后停止。

02执行了开工核对、来源/字段审计、设计合同与独立数学/拒绝自验；11项缺口阻止完整冻结。03/04/06按实际相邻字段消费，不把本次自验当独审或授权自动开卡。机器active_card=null、next_card=P1-02为本卡待手动复审归属，01/05已接受partial边界不撤销。


## 历史接续：03同卡O1–O7整改交回时（rev45，独审前）

2026-11-15 P1-03同卡O1–O7整改交回：追加rev45，D16/I18/C6，FC/1 evidence-r1。登记用户带回的rev44有界设计/来源/保全边界独审通过，原文6项与实列7项及411误写保留说明；当前整改增量仅Codex自验。291组明确为路由级字段清单，未建立字段到span/逐字段语义绑定；补3条读入路由的6份直接fixture及预期，I01改原生vesta；补RAW尺寸元数据来源，明确81候选shape及模板语义/精度未冻，损失枚举完整性未证，状态时间同步。原4子集/81未准入、SG六缺口/FS11项、01/02/05原partial、02rev43增量未独审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受保持。03仍partial，04/06未施工，无产品/新运行授权；交回后停止。

[03整改回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/RECEIPT.md) / [O1–O7处理](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/correction-notes.md) / [用户转交复审登记](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/review-registration.json) / [当前合同与边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/design.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/DSH-REVIEW-HANDOFF.md)。

原rev44通过仅路由级有界设计/来源/保全；本次O1按复审允许的收窄措辞处理，字段span缺口仍在，O4/O5完整参数/语义/精度不因勘误关闭。用户要求继续，同卡办理；不开04/06，不扩大产品/目标/新实验。原文与规范登记的6/7差异、411误写及完整性未证均保留，旧封存/基线不改。


## 历史接续：rev45独审登记与两项说明（rev46）

2026-11-15 登记P1-03 rev45（D16/I18/C6）O1–O7有界证据/口径增量独审：partial-design-evidence-boundary-pass-with-0-blocking-findings，blocking=false，另附2项非阻断备注。追加rev46仅登记及文档/证据呈现说明，技术版本仍D16/I18/C6；rev45原封存及rev44接受范围分别保留。03卡接手段标明rev43→rev44历史并改称4个旧空schema子集；负控呈现新增targetedHit/allFailures/coCausedFailures，O3完整列2个失败。291仍路由级库存、81完整profile/逐字段语义精度/损失枚举/真实往返未接受；01/02/05原partial、SG六缺口/FS11、02rev43增量未审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受不变。本rev46登记说明仅Codex自验，未独审；04/06未启动，无产品/新运行授权，交回后停止。

[rev45独审登记](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-081622-p1-03-rev45-review-registration-notes/review-registration.json) / [本次回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-081622-p1-03-rev45-review-registration-notes/RECEIPT.md) / [两处备注说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-081622-p1-03-rev45-review-registration-notes/correction-notes.md) / [当前被审合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/design.md) / [可手动转交材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-081622-p1-03-rev45-review-registration-notes/DSH-REVIEW-HANDOFF.md)。


## 历史接续：03实质静态补证（rev47）

2026-11-15 P1-03同卡实质静态补证交回：追加rev47，D17/I19/C6，FC/1 evidence-r2。84/291字段组新增91处直接源组件绑定，另核20个冻结独立数值例，7类格式/单位精度规则；207组无新直接组件，已绑组也不等于完整语法。16路由typed参数选定域，m3d由direct-f32纠正为原UI直接32位整数，PRI仅flag0/little32；4旧空子集/81未准入不变。48个现有SG/FS属性给桥处置，64叶有限源独立分区9保留/55损失，两个共享bytes场分别枚举；完整Engine捕获/损失全域仍未证。原生相对资源引用与网格字节绑定，原恢复失败及hash布局/04承诺未证保持。8旧unknown分别列最小缺材，未新跑原件/产品/外算。rev45有界独审继续独立有效，rev46登记与本rev47新增量未独审。03整卡、81完整profile、291逐字段语义/精度、真实往返未接受；01/02/05原partial、SG六缺口/FS11、02rev43增量未审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受保持。04/06未启动，交回后停止。

[03本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/RECEIPT.md) / [实质设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/design.md) / [字段证据](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/field-evidence.json) / [剩余缺件](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/remaining-issues.json) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-084010-p1-03-substantive-design/DSH-REVIEW-HANDOFF.md)。


## 2026-11-17 P1-03最终设计自验

当前状态：03整卡设计施工完成，待用户手动转dsh整卡独审；rev48/D18-I20-C6。

2026-11-17 P1-03整卡设计施工完成：rev48/D18-I20-C6。85原路由、234参数和291组选定字段逐项冻结精确消费/来源/当前模型/单位精度/失败，1865组件覆盖290组，JP2一组保留真实无产物。完整源、损失同意、资源图、候选与提交后语义重开、D4-31/Saved及85键外既有IO设计齐备。最终完整自验243/0、exit0、输入稳定；旧失败及额外producer/上游资格边界保留。待用户手动转dsh整卡独审，尚无独立接受；rev45有界独审及旧4/81、01/02/05 partial、SG六/FS11/C6/P1两接受两未接受保持。04/06未启动。

[整卡设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/design.md) / [当前profile](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/contracts/format-profiles.json) / [后续消费](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/consumer-handoff.md)。

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

P1.1路线设计与P1.3 C4设计接受保持；P1.2公共接口、P1.4完整实施安排已作者完成，等待本次整体独审。下一动作仅为用户转交本次最终包并带回复审结果；不自动启动产品施工、新卡、P2–P5或新的复审会话。旧01–05的 partial/独审记录仍按各实际被审范围保留。

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
