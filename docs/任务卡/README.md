# 任务卡

更新：2026-11-19。需求以现行PRD/技术方案/VESTA需求与已接受设计为准；README维护真实任务状态，历史过程在各阶段HISTORY，临时变更在决策确认单。

**当前源码**：用户明确指定直接放在 `/Users/agilecho/code/Draft`，应用根为本仓库 [app/](../../app/README.md)、工具根为 `tools/ca-bootstrap/`，与文档同仓库管理。原封存和状态不变，旧卡源码路径按[目录裁定](源码项目目录调整.md)解释；本地源码存在不替代尚未创建的正式 CA 工作区。

## 给你看的

阶段0研究与阶段1设计已经按原范围收口。P1当前设计交付（P1.1—P1.4及整体）接受保持，C4设计/C5条件/C6增量未审及03/04历史范围不追认。原25张v2/v3卡面的独立接受按原SHA保留；本轮按用户指定v1.4规则形成**P2-01 v4、其余24卡v3、共同约定v3**，补齐原型、受影响测试和阶段整体界面E2E，现已核磁盘dsh报告通过、卡面可发放。用户已要求继续施工与验证；用户已明确授权Codex逐卡代建会话；P2-01已在main完成整改，双目录各25/25自验通过，待用户手动dsh复审后继包；P2-02已交partial、冷构建封顶待裁定；后续按真实依赖推进。

| 阶段 | 用户或工程结果 | 当前状态 |
|---|---|---|
| [阶段0](/Users/agilecho/code/Draft/docs/任务卡/阶段0-原件研究与设计输入/README.md) | 功能/行为/格式/领域/CLI研究交接 | 原研究范围完成，Study29/29，非产品全通过 |
| [阶段1](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/README.md) | 架构、接口、保存、应用、构建与实施计划 | rev66独审、rev67登记设计级接受，已收口 |
| [阶段2](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/README.md) | 第一条可编辑、保存、导出、关页CLI续改及全新页面重开的晶体工程链，并有真实CA检查。 | 6卡后继独审可发放；P2-01独审整改已提交main，双目录25/25自验通过、待后继独审；P2-02已交partial、冷构建封顶待裁定；其余未施工，整体UI E2E未执行，阶段未结束 |
| [阶段3](/Users/agilecho/code/Draft/docs/任务卡/阶段3-全功能实现/README.md) | 全部结构、几何、视图、体数据、科学、外算、应用与格式功能在同一版本真实贯通。 | 12卡后继独审可发放；未施工，整体UI E2E未执行，阶段未结束 |
| [阶段4](/Users/agilecho/code/Draft/docs/任务卡/阶段4-平台正式接入/README.md) | 在现有平台上正式加载、两布局连续使用、工具与本人空间保存、隔离治理和目标环境可用。 | 4卡后继独审可发放；未施工，整体UI E2E未执行，阶段未结束 |
| [阶段5](/Users/agilecho/code/Draft/docs/任务卡/阶段5-整链验收与交付/README.md) | 最终同版全功能与真实平台用户整链通过，交出可复现源码、制品和独立接手资料。 | 3卡后继独审可发放；未施工，整体UI E2E未执行，阶段未结束 |

仍为25张卡、15单元/56依赖步骤、31域/40操作/254功能/85路由，未扩大或缩减范围、不重算九摘要。P2-01已开始施工核实，正式VestaCraft根仍须保留不存在；计数和卡面接受均不证明实现完成。

## 给执行者看的

P2-02当前源码和离线依赖缓存已封装、锁定metadata通过；完整工具未编译通过。冷构建3轮已用完，正等用户对[本卡按原时长续办提案](/Users/agilecho/code/Draft/output/vesta-execution/20261118-150143-p2-02-cold-time-proposal/CONTINUATION-PROPOSAL.md)明确裁定；未答复前不新增编译，回执与保护已交回，当前没有施工进程。文件模块和驱动通过不能替代完整工具、正式prepare或P2接受。精确实况见阶段2入口。

当前P2-01[最终回执](/Users/agilecho/code/Draft/output/vesta-p2-01/20261118-132952-attempt3/RECEIPT.md)已核：两新目录禁网合同各20/20，完整依赖及19生成输出逐字节一致；原生/TS证据属于同一冻结包。现已收到用户手动带回的dsh复核：技术结果通过，正式接受暂不通过；Codex已在临时副本确认B2/B3门禁缺口，并对B1/B4/B5给出精确区分，见[再复核记录](/Users/agilecho/code/Draft/output/vesta-p2-01/20261119-dsh-report-recheck/RECHECK.md)。原[首次独审材料](/Users/agilecho/code/Draft/output/vesta-p2-01/20261118-132952-attempt3/DSH-REVIEW-HANDOFF.md)保留；本轮整改已在main提交，后继包双目录各25/25自验通过，待用户手动dsh复审；本卡未正式接受、阶段未结束。P2-02继续独立prepare工具源码与受影响验证；源码迁入和实际prepare前置保持。原两次partial、失败及全部资源消耗见阶段HISTORY，不改写旧结果。

当前[P2-01独立施工派发与用户授权](/Users/agilecho/code/Draft/output/vesta-execution/20261118-115438-p2-01-dispatch/DISPATCH.md)已核；threadId `01a1ee83-0ed5-7383-bba8-a64c75e21d9f`。主会话统筹，一卡一会话；dsh仍由用户手动转交。

最新状态：[规则后继接受登记](/Users/agilecho/code/Draft/docs/任务卡/后续任务卡规则对齐接受登记.md) / [执行预检与第一卡交接](/Users/agilecho/code/Draft/output/vesta-task-handoffs/20261118-115219-rules-acceptance-execution-preflight/README.md)。

阶段3—5尚未施工；阶段2从P2-01开始按实际施工追加HISTORY，实际确认后填写决策；未发生的记录继续留空。编卡/审卡记录留在原交接包，不作为阶段执行历史。当前管理文档修正见[回执](/Users/agilecho/code/Draft/output/vesta-task-handoffs/20261118-114548-p2-p5-management-scaffolds/RECEIPT.md)，手动复审使用[最新材料入口](/Users/agilecho/code/Draft/output/vesta-task-handoffs/20261118-114548-p2-p5-management-scaffolds/DSH-REVIEW-HANDOFF.md)。

读取顺序：[共同约定](/Users/agilecho/code/Draft/docs/任务卡/后续施工共同约定.md) → 相应阶段README/HISTORY/决策确认单 → 本卡/最新续办 → 所引PRD/技术方案和[原型与验收依据](/Users/agilecho/code/Draft/docs/任务卡/原型与验收依据.md) → 当前实际输入。[原卡面接受登记](/Users/agilecho/code/Draft/docs/任务卡/后续任务卡接受登记.md)只接受原字节；[本轮后继精确SHA](/Users/agilecho/code/Draft/output/vesta-task-handoffs/20261118-110937-p2-p5-rules-alignment/card-issuance.json)已有精确SHA对应的dsh审卡通过结论，见[新接受登记](/Users/agilecho/code/Draft/docs/任务卡/后续任务卡规则对齐接受登记.md)。[给dsh的材料](/Users/agilecho/code/Draft/output/vesta-task-handoffs/20261118-114548-p2-p5-management-scaffolds/DSH-REVIEW-HANDOFF.md)由用户手动转交，不自动派发/建会话。

普通卡及小修只测新增/修改/共用影响范围；指定集中回归留在关口。阶段2的P2-06、阶段3的P3-12、阶段4的P4-04、阶段5的P5-02负责最终整体界面E2E，P5-03核最终交付同版。未运行或失败则阶段未结束，最终UI证据写阶段README，全部过程写HISTORY。所有运行/预算/费用条件沿原卡，不赠送新额度；P2-06精确授权未齐前禁止任何机器执行。

当前P1接受[true,true,true,true,true]、资料rev67/技术rev66 D35/I33-r1/IP6及wire I33保持。[机器状态](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/current-task-status.json)仅作当前组织镜像，保留原接受/历史字段，不代替需求或增加长期功能台账。每新卡须独立施工会话，同卡续办可沿原会话；用户已明确授权Codex逐卡代建独立施工会话并按依赖推进。原预算/费用/审核分工不变。

## 历史任务组织记录（以下保留当时状态）


下方各rev“当前/partial/未启动/待审”均属当时交回记录；现行状态见上方，不重开已接受设计、不改原封存。

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


## 历史：P1-05设计交回（rev40）

2026-11-14 P1-05设计交回：追加rev40，D12/I14/C6；交AB/1资产输入、EX/1外算、BT/1构建/目标与未来验证合同，仅Codex自验、待用户手动转dsh。原表身份及本地Noto字体输入可供01/02/03/06按边界消费；许可、选表科学语义、同位素及完整锁/目标环境仍有具名缺件，05整卡partial。01 F1–F6在rev38被审通过、整卡及SG-01…SG-06保持partial/open。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受。02/03/04/06未施工；产品和运行验证未授权，交回后停止。

[施工回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/RECEIPT.md) / [手动dsh](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/DSH-REVIEW-HANDOFF.md)。


## P1-05当前：独审登记（rev41）

2026-11-14 P1-05独审登记及非阻断勘误：追加rev41，技术版本D12/I14/C6不变；dsh确认被审rev40 AB/1、EX/1、BT/1的partial设计/来源/保全边界准确、无阻断。本轮以独立后继修O1–O4，补未施工卡名单和fontTools复现环境说明，仅Codex自验、说明增量未独审；原施工包、dsh报告及rev40封存不改。05/01继续partial，SG-01…SG-06开放，C4接受/C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。02/03/04/06未施工；不启动下一卡、产品或真实运行，交回后停止。

[本次回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/RECEIPT.md) / [真实dsh](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-213000-p1-05-assets-build-dsh-review/review-result.json) / [当前消费说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/consumer-handoff.md)。


rev51补件入口：[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/RECEIPT.md) / [完整设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/design.md) / [最少补件](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/external-inputs.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/DSH-REVIEW-HANDOFF.md)。 原四缺口均有新增具体设计；HS-02实际能力仍共享HS-01，未宣布整卡完成。


rev52当前入口：[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/RECEIPT.md) / [设计与来源](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/design.md) / [最小原件取证方案](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/DSH-REVIEW-HANDOFF.md)。 宿主来源已自行补，原件方案仅待明确范围许可；不启动06。


rev53当前入口：[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/RECEIPT.md) / [设计与静态事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/design.md) / [最小剩余范围](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/DSH-REVIEW-HANDOFF.md)。 首批6静态成功，条件7未在关闭窗口内执行，动态仍待另行授权；不开06。


rev54当前入口：[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/design.md) / [最小剩余](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/DSH-REVIEW-HANDOFF.md)。 两轮14段已各1次，原件转换/历史分组仍精确保留，动态未授权；不开06。


rev55当前入口：[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/design.md) / [最小剩余输入](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/DSH-REVIEW-HANDOFF.md)。 16体3槽已按三轮授权各1次，完整UI域/持续动画分组仍partial，不跟其他函数/不开06。


rev56当前入口：[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/design.md) / [最小剩余证据](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/DSH-REVIEW-HANDOFF.md)。 20体3槽2常量已各1次，实际资格/动画分组仍partial，不泛化追函数/不开06。

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
