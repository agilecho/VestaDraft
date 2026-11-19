# VESTA Study 基线

<!-- P1-CURRENT-STATUS BEGIN -->
当前状态：03整卡设计施工完成，待用户手动转dsh整卡独审；rev48/D18-I20-C6。

2026-11-17 P1-03整卡设计施工完成：rev48/D18-I20-C6。85原路由、234参数和291组选定字段逐项冻结精确消费/来源/当前模型/单位精度/失败，1865组件覆盖290组，JP2一组保留真实无产物。完整源、损失同意、资源图、候选与提交后语义重开、D4-31/Saved及85键外既有IO设计齐备。最终完整自验243/0、exit0、输入稳定；旧失败及额外producer/上游资格边界保留。待用户手动转dsh整卡独审，尚无独立接受；rev45有界独审及旧4/81、01/02/05 partial、SG六/FS11/C6/P1两接受两未接受保持。04/06未启动。

[整卡设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/design.md) / [当前profile](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/contracts/format-profiles.json) / [后续消费](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/consumer-handoff.md)。
<!-- P1-CURRENT-STATUS END -->

S01 快照时间：2026-11-08T11:36:22.008671+08:00。执行计划：[S01](../superpowers/plans/2026-11-08-vesta-study.md)。

R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

## 目标与方法

- 原件目录：`/Users/agilecho/Documents/VESTA`。
- 主程序 SHA-256：`b4835068478ec1852f732c93ab25768f2b8af0166a92fc68dfd85704fe71c9e9`，与要求一致。
- 动态库 SHA-256：`c2c677facf6c67783cc7bbe64e92b75beb1a567fc373eacabee58b11fb81a77f`，与要求一致。
- Info.plist SHA-256：`9d3fe28038ab7ce82c2c97b8d2c7e301e0ad04bafe40d3b42e6a8c26d7a0d704`；LICENSE SHA-256：`c6690795ed2edf9f2c45d6a075d1964e242d5ef7ba1333891ece1da73b2bf1c1`，均与 S01 快照一致。
- 主程序/库均为 macOS x86_64/arm64 universal；不代表 Linux/x86 可用。
- Bundle ID：`org.jp-minerals.vesta`；包字段版本 `3.0`。S04 About 的 [原生 AX 摘要](../../output/vesta-study/20261108-113622/s04/about-read-summary.json)实读 **3.90.9a，64-bit Edition，Build date Aug 8 2026**。About 截图采集失败：[about.png 留档](../../output/vesta-study/20261108-113622/s04/about.png)实际仅 66×20，是共享附属窗口，不作为 About 外观或版本证据。
- Craft Anything 方法锁定提交为 `1f8508a104134c436cbba5ea2f675619ef47702f`，S01 时工作区干净；活跃上游后来已有推进，来源读取使用锁定 git 对象/快照，详见 METHOD，不能把此历史状态当当前 HEAD。
- Rust：`rustc 1.99.1 (58cf8f9c3 2026-10-13)`；Cargo：`cargo 1.99.1 (5f94df478 2026-08-27)`，实际命令退出 0。
- 当前 Craft 目录非 Git 仓库；本阶段未初始化仓库或修改上游。

## 证据与证明范围

本轮证据根：[20261108-113622](../../output/vesta-study/20261108-113622/)。

- [基线记录](../../output/vesta-study/20261108-113622/s01/baseline.json)：目标/资料摘要、方法提交、工具链。
- [摘要原始命令](../../output/vesta-study/20261108-113622/s01/target-shasum.json)、[架构识别](../../output/vesta-study/20261108-113622/s01/target-file.json)。
- [磁盘](../../output/vesta-study/20261108-113622/s01/disk.json)、[系统](../../output/vesta-study/20261108-113622/s01/os.json)、[内存](../../output/vesta-study/20261108-113622/s01/memory.json)。

S01.1–S01.3 已完成。CLI 构建/运行、目标 GUI/CLI、内部分析、模型调用和产品能力均未在 S01 验证。

## 基线资料修订

原始 [baseline.json](../../output/vesta-study/20261108-113622/s01/baseline.json)固定为 11:36 快照，SHA-256 为 `35eb5c3174e0d9f6e130f1f1504937914326fbcbd8fa6455a1d6de058f2dd0aa`，本次复审不修改。其 8 份引用资料已逐项重算，仅需求文档发生以下已授权的研究进展回写：

| 资料 | S01 旧 SHA-256 | rev2登记的SHA-256 | 时间与原因 |
|---|---|---|---|
| `docs/VESTA全功能重写要求.md` | `8742f8106e117c2bbbfc4950799d0ec24b2caaba75acb5262ffb50e7411c3bde` | `5bd841496b4230fe028bed13db74b3457b11d82c002453963c4c85299955c3fc` | 文件 mtime 实读 2026-11-08 12:51:42.036067 +08:00；按用户授权回写 S04 版本事实、S04/S05 研究状态与覆盖数量。本节为复审时补登记，mtime 不冒充当时的修订日志。 |

其余 7 份资料和 4 个目标身份文件摘要与原快照一致。新建 [baseline-rev2.json](../../output/vesta-study/20261108-113622/s01/baseline-rev2.json)记录逐项新旧摘要、登记时间、版本 `resolved_by/resolved_at` 及图片证据用途，并归档当前需求正文。旧需求正文当时未单独归档，无法仅凭旧摘要还原逐行差异；不宣称已保存旧版全文。

原快照的 `release_version: null` 与 pending 状态只表示 S01 当时尚未查明。版本问题在原生 journal 的 `2026-11-08T03:52:02Z` 事件中解决（北京时间 11:52:02，秒级记录），依据是 AX evidence `ev_dd698613d65c7e33e47c41743b5368b7a5a24cef16e6844bd73d924efed7231e`、unknown `unk_42e315593eabb9ce` revision 2；机器读取后续状态应同时读取 rev2，不能把旧 pending 当当前结论。[复审记录](../../output/vesta-study/20261108-113622/reviews/20261108-130129-s01/)保留 journal 摘录、重跑命令、独立核对结果和修改前文档。

S01 未建立整个原件目录的逐成员清单；当前身份结论限于上述四个文件及已记录元数据。`madel`、`STRUCTURE_TIDY` 等外部程序的调用、架构和依赖继续留 S07，不将目录清单缺失扩大为 S01 已核四文件漂移。

## S04 运行补充

真实 Craft Anything `native-window-capture` 成功保存 1720×1456 的启动截图和 344 节点 UI 树，未截断。输出区显示 OpenGL 2.1 Metal - 91.7、Apple M5、最大视口 16384×16384、深度缓冲 32 bit；仅证明本机启动情况。[截图](../../output/vesta-study/20261108-113622/s04/startup.png)、[回执摘要](../../output/vesta-study/20261108-113622/s04/startup-capture-summary.json)。

原生版本证据来自上述 AX 文本；unknown `unk_42e315593eabb9ce` 已据此更新为 resolved，原生 verifier 的 valid=true 核实记录及证据引用完整性，不证明 About 图片可用。`startup.png` 经本次查看确为启动主窗口，仅用于启动画面。初始启动使用 VESTA_PREF 隔离；随后 CUA 获取已关闭 app 时自动启动了一次默认偏好实例，只查看 About 并退出，未声称该次偏好已隔离。后续交互实例由受控进程启动并设置 VESTA_PREF，再连接 CUA；不把默认偏好实例的设置状态当基准。

## 历史：S02资料状态纠错的第三版登记

当时 S02 独立复核后，需求文档明确 S02.3 仍为部分完成；全功能/Web目标未变。[baseline-rev3.json](../../output/vesta-study/20261108-113622/s01/baseline-rev3.json)以 previous_revision 摘要链到 rev2，登记该次需求旧/新摘要、mtime/登记时间、原因及[正文](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/requirements-at-rev3.md)。对应 before/diff 可复查；rev3 的 current 只表示当时状态，不是现在 S02 未完成的结论。

## 历史：S02验收状态对齐的第四版登记（2026-11-10）

用户要求将已复核的 S02 验收状态同步至各文档，再由新会话执行下一任务卡。需求、Study/总体计划、资料索引、AI入口、来源页和 HANDOFF 已区分当前与历史状态；下一卡为 S03 证据核验、必要补证及 dsh 独立复审，Study 仍 11 完成/18 未完成。

新增 [baseline-rev4.json](../../output/vesta-study/20261108-113622/s01/baseline-rev4.json)按 predecessor 摘要链接 rev3，保存这次需求正文、准确摘要及登记时间；baseline/rev2/rev3 和旧报告不改字节。当前引用资料身份应消费 rev4。原件/锁定 CLI、原生版本依据、254 功能及 78 样本未由本次文档对齐增加或重建；S02 来源接受不提升为完整格式、科学数值或产品通过。[状态对齐记录](../../output/vesta-study/20261108-113622/reviews/20261110-052358-s02-status-alignment/)保留修改前副本和校验。

## 历史：S03复核后的第五版登记（2026-11-10）

新增 [baseline-rev5.json](../../output/vesta-study/20261108-113622/reviews/20261110-053708-s03-tooling-audit/../../s01/baseline-rev5.json)按摘要链继承 rev4，归档当前需求正文及本次状态变更。rev4 的主状态已记 S02 完成，但继承字段 `study_status.S02.3=partial-unchecked` 是残留；仅在 rev5 纠正为 `completed-after-source-review`，旧 rev4 字节不改。S03 四项经 Codex 核验、dsh 独立复审通过，Study 保持 11 完成/18 未完成。

本次只补只读 CLI/Check help，不重建、不运行 VESTA 或 Check/builder，不改变原生记录。源码引用身份按锁定 git 对象/快照核实，活跃上游路径仍仅定位；[证据矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-053708-s03-tooling-audit/S03-MATRIX.md)、[dsh回执](../../output/vesta-study/20261108-113622/reviews/20261110-053708-s03-tooling-audit/dsh-review-retry.stdout)及[文档修订](../../output/vesta-study/20261108-113622/reviews/20261110-053708-s03-tooling-audit/document-revisions.json)分开记录验收。

## 历史：S04部分交付后的第六版登记（2026-11-10）

[baseline-rev6.json](../../output/vesta-study/20261108-113622/reviews/20261110-055459-s04-entry-interaction-audit/../../s01/baseline-rev6.json)按摘要链继承rev5，归档实际需求正文和本轮子项状态。S04.2 保持未勾选；S04.3/S04.4 已完成初步行为规格/遗漏登记；本轮 dsh 独立复审 partial、无硬阻断，S04 整卡未完成。13完成/16未完成；不把S04整卡、Study、格式/科学或产品记通过。旧baseline/rev2–5、原版本依据、锁定CLI及目标身份不改。新增10真实原生后当前1650为磁盘派生索引，不冒称raw；原R132的1288+352保留。

## 历史：S04触发补证后的第七版登记（2026-11-10）

[baseline-rev7.json](../../output/vesta-study/20261108-113622/reviews/20261110-063443-s04-trigger-completion/../../s01/baseline-rev7.json)按实际摘要继承rev6，归档本轮需求正文；原baseline/rev2–6及旧需求归档不改。本轮仅接续S04触发补证：新增90条原生至1740磁盘派生索引；254稳定ID/78样本，90条入口级待核。S04.2仍partial-unchecked，S04.3/.4既有接受保持；13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 90新增含2function-search、6function-study、37UI、45walkthrough；实际失败/截断/退出后重开和颜色副作用保留。[矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-063443-s04-trigger-completion/S04-MATRIX.md)及[复审](../../output/vesta-study/20261108-113622/reviews/20261110-063443-s04-trigger-completion/review-result.json)分开证据与裁定。

## 历史：Objects/焦点续证后的第八版登记（2026-11-10）

[baseline-rev8.json](../../output/vesta-study/20261108-113622/reviews/20261110-072741-s04-objects-focus/../../s01/baseline-rev8.json)按实际SHA继承rev7并归档需求正文。旧baseline/rev2–7和所有旧归档字节不改。本轮继续S04 Objects/焦点：新增45条原生至1785磁盘派生索引，254稳定ID/78样本/12头保持，87条入口级待核。S04.2保持partial-unchecked，S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 新45含3function-search/9function-study/22UI/11walkthrough，CUA和文件差异/静态机制分别登记，颜色关闭RGB副作用、误Space与184秒生命周期偏差保留。[矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-072741-s04-objects-focus/S04-MATRIX.md)与[实际复审](../../output/vesta-study/20261108-113622/reviews/20261110-072741-s04-objects-focus/review-result.json)分开证据和裁定。

## 历史：alpha事件路由后的第九版登记（2026-11-10）

[baseline-rev9.json](../../output/vesta-study/20261108-113622/reviews/20261110-075808-s04-alpha-dispatch/../../s01/baseline-rev9.json)按SHA继承rev8并归档需求全文；旧baseline/rev2–8不改。本轮继续S04 alpha事件路由：新增9条原生至1794磁盘派生索引，254稳定ID/78样本/12头/87入口级待核保持。S04.2保持partial-unchecked；S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 新9含1search/5study/3UI；单击对照85.992秒正常0，未显示alpha只作为boundednegative。静态vtable/非空事件链接不替代runtime值。[矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-075808-s04-alpha-dispatch/S04-MATRIX.md)、[复审](../../output/vesta-study/20261108-113622/reviews/20261110-075808-s04-alpha-dispatch/review-result.json)。

## 历史：alpha运行时后的第十版登记（2026-11-10）

[baseline-rev10.json](../../output/vesta-study/20261108-113622/s01/baseline-rev10.json)按实际SHA继承rev9，归档最终需求全文；旧原始baseline/rev2–9及全部旧封存不改。本轮S04 alpha入口已有原件阳性：同PID激活后28×128浮窗与AXSlider192；诊断副本实测点击门槛/显示链。新增32原生至1826派生磁盘索引，254ID/78样本/12头/87entry-only保持。dsh复核pass、无硬阻断；S04.2按入口/条件登记范围接受，S04整卡收尾，Study14完成/15未完成。未进入S05或产品实现。 原件/CLI/旧1794/12头/78样本身份未变。原件AX192与诊断门槛分离，popup图失败、首wrapper130/监督断开/缺poll、原件SIGTERM-15均保持。[实际复核](../../output/vesta-study/20261108-113622/reviews/20261110-081848-s04-alpha-runtime/review-result.json)。

## 历史：S04状态对齐后的第十一版登记（2026-11-10）

[baseline-rev11.json](../../output/vesta-study/20261108-113622/s01/baseline-rev11.json) 按实际摘要继承 rev10，保存当前需求正文；登记 S04 最终裁定及用户授权下一会话执行 S05。原 baseline/rev2–10 和 S04 封存包保持字节，旧 partial 记录仅作历史。S01–S04 已按各卡范围完成，S02/S03/S04 的 dsh 独立复审均通过；Study 14 项完成、15 项未完成。S04 接受的是入口与条件登记、初步行为规格和遗漏登记，完整行为、格式与科学验证仍继续。下一会话执行 **S05：研究全部格式、工程和保存/导出**，由 Codex 直接执行、dsh 独立复审；完成本卡后停下。完整 Study 及产品实现未完成。

当前 254 个稳定功能 ID、78 个稳定样本、87 条入口级待核；最新原生记录为 1826 条磁盘派生索引，不是全量原始导出。历史 R132 的完整核验派生快照为 1640=raw 1288+按 ID 补读 352，单独保留；本次状态对齐没有新增目标证据。 本次只对齐文档/机器状态，不重新裁定科学结果；上游更新分析保留，方法/CLI 继续锁定。[对齐证据](../../output/vesta-study/20261108-113622/reviews/20261110-085045-s04-status-alignment/)。

## S05 本轮部分交付的第十二版登记（2026-11-10）

[baseline-rev12.json](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/../../s01/baseline-rev12.json)按实际SHA继承rev11，归档[需求正文](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/requirements-at-rev12.md)及其旧/新SHA。本轮新增34原生来源记录、VS79和3个open问题；dsh接受本轮包及S05.4，S05.1–.3和整卡未完成，15完成/14未完成。原baseline/rev2–11、S04封存/旧状态对齐、原件与锁定CLI/方法均保持；source_binding=locked-git-object仍按锁定对象校验，不与活动工作树误比。

## S05完成的第十三版登记（2026-11-10）

[baseline-rev13.json](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/../../s01/baseline-rev13.json)继承未改rev12，并归档[需求快照](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/requirements-at-rev13.md)。实际dsh通过S05.1–.4研究卡；101样本/2015派生原生/23问题头，所有真实失败与完整codec/版本未知保持。Study18/11，停止S05，无产品施工。

## 历史：S05状态对齐后的第十四版登记（2026-11-10）

[baseline-rev14.json](../../output/vesta-study/20261108-113622/s01/baseline-rev14.json)按实际摘要继承 rev13，归档当前需求全文并登记 S06 新会话授权。旧 baseline/rev2–13、S05 原 dsh 审查和1880文件封存包保持字节；S05 的历史 partial 与完成后停止记录不被改写。S01–S05 已按各卡范围完成，S05 经 dsh 独立复审通过；Study 18 项完成、11 项未完成。S05 接受逐格式研究及真实读写/编辑/输出、失败与未知记录，不代表全部格式兼容。用户已授权下一会话执行 **S06：研究领域规则、科学算法与数值预期**；Codex 直接执行，dsh 独立复审，本卡完成后停下。完整 Study 和产品实现未完成。 当前 254 个稳定功能 ID、101 个稳定样本、87 条入口级待核；2015 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。23 个原生问题头为 10 resolved/3 investigating/10 open；本次状态对齐不新增目标证据。 [本次对齐证据](../../output/vesta-study/20261108-113622/reviews/20261110-135039-s05-status-alignment/)。

## 历史：S06验收后的第十五版登记（2026-11-10）

[baseline-rev15.json](../../output/vesta-study/20261108-113622/s01/baseline-rev15.json)按实际SHA继承rev14，归档当前需求正文，保留旧baseline/rev2–14和历史封存字节。S01–S06 已按各卡研究范围完成，S06 经 dsh 独立复审通过；Study 22 项完成、7 项未完成。S06 接受逐领域已证规则或明确未知、独立预期、真实失败和边界，不代表全部科学算法或格式兼容通过。本轮仅完成 S06 并停止，未进入 S07、P1 或产品实现。完整 Study 和产品实现未完成。 当前 254 个稳定功能 ID、102 个稳定样本、84 条入口级待核；2036 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open；S06 新增 21 条真实原生记录和一个科学规则问题头，来源 rev10/数据库 rev4保持。 核密度/模型Patterson只验DC与不变量，有界资源只验选定45084值；没有全谱/容量/全部科学通过。实际[dsh复审](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/dsh-review-resume.stdout)及[本卡证据](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/)。

## 历史：S06状态对齐后的第十六版登记（2026-11-10）

[baseline-rev16.json](../../output/vesta-study/20261108-113622/s01/baseline-rev16.json)按实际摘要继承 rev15，归档当前需求全文并登记 S07 新会话授权。旧 baseline/rev2–15、S06 原审查和374文件封存保持；科学未知、失败及原会话停卡记录不改。S01–S06 已按各卡研究范围完成，S06 经 dsh 独立复审通过；Study 22 项完成、7 项未完成。S06 接受逐领域已证规则或明确未知、独立预期及真实失败，不代表全部科学算法或格式兼容通过。用户已授权下一会话执行 **S07：核实原件 CLI、外部程序与运行边界**；Codex 直接执行，dsh 独立复审，本卡交付后停下。完整 Study 和产品实现未完成。 当前 254 个稳定功能 ID、102 个稳定样本、84 条入口级待核；2036 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open；来源 revision10、数据库 revision4 保持。本次状态对齐不新增目标证据。 [本次对齐证据](../../output/vesta-study/20261108-113622/reviews/20261110-145914-s06-status-alignment/)。

## S07 执行修订 rev17

本会话授权下已形成S07三项具体研究合同，尚待Codex自验及真实dsh独立复审，仍22完成/7未完成；不预勾项。需求正文已归档，rev17按SHA继承rev16，锁定对象/目标/CLI身份不变。新增1条TIDY原生观察、VS0103/0104，平台/外部执行/再分发资格未知保留。见[基线](../../output/vesta-study/20261108-113622/s01/baseline-rev17.json)。

## S07 收尾修订 rev18

S07三项及整卡取得本机dsh实际pass后同步Study25完成/4未完成；仅研究验收，原科学/格式/平台/依赖未知保留。rev18归档本次需求正文、按SHA继承不可回改rev17；原目标/锁定CLI/方法对象不变。当前会话完成S07即停，不授权S08/P1或产品实现。见[基线](../../output/vesta-study/20261108-113622/s01/baseline-rev18.json)。

## 历史：S07状态对齐后的第十九版登记（2026-11-10）

[baseline-rev19.json](../../output/vesta-study/20261108-113622/s01/baseline-rev19.json)按实际摘要继承 rev18，归档当前需求全文并登记 S08 明确授权。旧 baseline/rev2–18、S07 原审查及284文件封存保持；原停卡记录、失败和未验证边界不改。S01–S07 已按各卡研究范围完成，S07 经本机 dsh 实际只读独立复审通过；Study 25 项完成、4 项未完成（S08四项）。用户已明确授权下一会话执行 **S08：收束 Study，交付可直接设计的材料**；Codex 直接执行，dsh 独立复审，交付后停下。已知失败和未验证边界保留，不代表全功能行为、科学、格式、平台或外部程序全部通过；完整 Study 及产品实现尚未完成，不自动进入 P1/产品实现。 当前 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open；来源 revision10、数据库 revision4 保持。本次对齐没有新增目标证据。 [本次对齐证据](../../output/vesta-study/20261108-113622/reviews/20261110-155139-s07-status-alignment/)。

## S08研究收束执行修订rev20（待独立验收）

[baseline-rev20.json](../../output/vesta-study/20261108-113622/s01/baseline-rev20.json)按摘要继承rev19，归档S08执行中的需求正文；只修改研究状态/导航，不改变目标、锁定方法或全部功能目标。报告见[STUDY-REPORT](STUDY-REPORT.md)，本轮无目标新运行/无新稳定样本/无原生头更新；2037派生记录、24头10/3/11、254ID/104样本/84entry-only保持。先待dsh实际裁定，再更新S08/P0状态；产品未开始。

## S08实际验收修订rev21

[baseline-rev21.json](../../output/vesta-study/20261108-113622/s01/baseline-rev21.json)按SHA继承不可回改rev20，归档实际dsh-pass后的需求正文；rev20保留执行中授权/待验登记，rev21明确给当前状态。S08.1–.4、Study研究交付与P0.2/P0.3按原标准完成，3项非阻断文档发现由Codex澄清并做最终自验；原审查字节保持，未重跑dsh。目标/方法/CLI/原生24头/2037派生记录/254ID/104样本/84entry-only均未改变，不声称科学/格式/平台或产品全过；交付后停止。见[报告](STUDY-REPORT.md)与[本轮证据](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/)。

## 当前：S08状态与P1授权对齐后的第二十二版登记（2026-11-10）

[baseline-rev22.json](../../output/vesta-study/20261108-113622/s01/baseline-rev22.json)按实际摘要继承 rev21，归档当前需求全文并登记 P1 设计授权。旧 baseline/rev2–21、原dsh审查和S08的601文件封存保持；Study通过仍是原研究交付判据，不升级为全行为/科学/格式/平台/产品通过。S01–S08 已按原研究交付判据完成，S08 经 dsh 实际独立复审通过；Study 29/29，P0.1–P0.3 按研究范围完成。用户已授权下一会话执行 **P1 设计阶段：架构、接口和施工安排（P1.1–P1.4）**，Codex 直接执行，dsh 独立复审。设计需处理 DPRE-01–08 中相关前置缺口；P1 四项尚未完成，交付后停下，不自动进入 P2–P5 或产品实现。Study 完成不代表全部行为、科学、格式、平台、模型或外部程序通过。 当前 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open，即14个未解决；来源 revision10、数据库 revision4 保持。本次状态对齐没有新增目标证据或产品代码。 [本次对齐证据](../../output/vesta-study/20261108-113622/reviews/20261110-164640-s08-status-alignment/)。

## P1设计稿与两项用户决定的rev23登记

[baseline-rev23.json](../../output/vesta-study/20261108-113622/s01/baseline-rev23.json)按rev22摘要追加，归档需求全文及两项实际用户裁定。新R/reviews/20261110-p1-design有预编辑副本、来源版本/依赖获取、最小本机WASM/Worker/WebGL2实验；没有目标原件/原生记录/产品builder。旧基线与S08封存不改；P1验收以实际后续dsh结论和最终状态为准。

## P1部分交付的rev24登记

[baseline-rev24.json](../../output/vesta-study/20261108-113622/s01/baseline-rev24.json)按SHA继承rev23并归档需求全文。真实dsh裁定仅P1.1完成，P1.2–.4未完成；Study29/29保持。源/工具/原件与原生记录没有因P1改动；两项用户决定沿用。R/reviews/20261110-p1-design保留所有审查输入/输出、实际失败/退出、修订与最终自验；未创建产品/worker、未调用模型builder或原生Check。

## 文书/绑定收尾与T9路径迁移的rev25登记

[baseline-rev25.json](../../output/vesta-study/20261108-113622/s01/baseline-rev25.json)按SHA继承rev24并归档现行需求全文。2026-11-11的变更只有两类：①外置卷由Samsung_T5迁移为T9，5份文档做路径更新（需求正文/资料索引/README/研究计划/更新影响分析），无科学/格式/接口语义变化；②I4两处类型邻接残余由dsh独立复审为实质解决，P1仍partial，P1.2–.4未完成。本轮未创建产品目录/代码/worker，未跑model builder或原生Check。rev22登记段恢复为当时原文，历史不改写；rev24及更早封存保持原字节。

## P1设计续办待审的rev26登记

[baseline-rev26.json](../../output/vesta-study/20261108-113622/s01/baseline-rev26.json)按SHA继承不可回改rev25，归档现行需求正文及本轮D3/I5/C3/模块计划；绑定Draft当前定位，历史绝对路径仍保持于旧记录。仅设计/导航/状态变化，无原件/调查/样本/unknown更新。P1.1保持已接受；其余三项待审或有未冻结接口，不能预勾。dsh由用户手动转交后独立复审；本轮未调用。


## P1复审后同卡设计整改的第二十七版登记（2026-11-14）

[baseline-rev27.json](../../output/vesta-study/20261108-113622/s01/baseline-rev27.json)按实际SHA继承rev26，归档需求正文与用户DEC-P1-03。D4/I6/C4补规则/格式注册表、类型名、Revision边界、初始保存状态和PNG帧合同，精确定义目标可执行文件与正式调查规范清单摘要；锁定公开源码增补ai/lib.rs，未建工具。31单元补已证子域签名和原输入/清理/续作，85个格式入口逐项分派；F1原七门与F13完整高级合同仍未闭合，不提升整体或P1.2/P1.4。

P1.3由用户裁定设计级验收，当前作者稿待dsh独立复核，真实worker/Check/launcher另行授权。仅P1.1勾选保持。R1全部148文件和dsh原复审、baseline/rev2–26、S08封存及正式调查未修改；R2的before/after/diffs、分项自验和手动待审入口见[整改包](../../output/vesta-study/20261108-113622/reviews/20261114-105518-p1-design-corrections/)。这次未增加原件证据、问题头、样本或模型调用；不进入P2–P5、产品施工或部署。


## P1.3设计接受回写与R3整改的第二十八版登记（2026-11-14）

[baseline-rev28.json](../../output/vesta-study/20261108-113622/s01/baseline-rev28.json)以实际SHA继承rev27，保存本轮需求正文及D5/I7/C5/模块计划身份。用户带回dsh R2结论并明确授权登记C4设计级接受；仅P1.1/P1.3两项接受，P1.2/P1.4和整体partial，真实运行未验证/未授权。

N1–N7及C4旧口径在新R3中整改；当前增量仍待用户转交dsh，不把原C4已审身份扩写成C5已审。R2原包/封存、基线至rev27、正式调查和样本保持；R3 before/after/diffs、原复审入库来源、接受记录及分项自验可独立复查。没有创建产品或运行任何worker/Check/launcher/外算/模型，费用预算延续不重置。


## 第二十九版：R3复审残留与证据保真整改（2026-11-14）

rev29以实际SHA继承rev28，新增需求归档。review_result回指真实dsh R3 JSON，忠实摘要保留原文false，用户授权后的true保留在独立接受记录。C6定义独立pin/root/registry/receipt、补反例；D6§9标历史，N1/N2/N5字段口径补齐。旧R1/R2/R3和原复审/基线至rev28不改，当前增量待审，不自行宣告C6通过。


历史R5主体设计续办：D7/I9/C6，资料rev30；31个有界操作、85格式路由/6类字段政策/28入口绑定、15项具体资格阻塞。P1.2/P1.4/整体partial，P1.3仅C4设计接受，C6增量待审；不进入P2–P5、产品实现或运行验证。证据与手动复审入口：[R5](../../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/README.md)。


历史R6同卡整改：2026-11-14 R6：D8/I10按dsh R5复审整改D1–D14及σ公式，Codex自验待用户手动转dsh。P1.1继承接受、P1.3 C4设计级接受保持，C6原字节及未审增量身份不变；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料rev31；不进入P2–P5、产品实现、部署或运行验证，真实worker／Check／launcher未授权、未验证。 施工回执与手动复审入口：[R6](../../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/README.md)。


R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。 [回执/复审入口](../../output/vesta-study/20261108-113622/reviews/20261114-161832-p1-design-r8-consistency/README.md)。

## 历史登记：资料基线rev33（原卡R8交接勘误）

R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

追加[baseline-rev33.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/s01/baseline-rev33.json)，前驱rev32 SHA为`48852f781e0269e1e0ebb0dd735a6d9c9f488bdd7ea44d18e62cfa9c65f4350c`，保持封存；[R8回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-161832-p1-design-r8-consistency/RECEIPT.md)。R7独立复审原件保留，R8仍待用户手动转dsh。

## 历史登记：资料基线rev34（R9独审对象）

rev34是R9独审时点的登记；[baseline-rev34.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/s01/baseline-rev34.json)与[需求归档](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163802-p1-design-r9-doc-state/requirements-at-rev34.md)绑定同一当前文件。rev32/33均按原SHA保留；R8独审及本轮自验不改变P1接受状态。

## 资料基线rev35：R9闭合结果回写

[baseline-rev35.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/s01/baseline-rev35.json)绑定当前结论与需求归档；前驱rev34 SHA为`96d7f8689e72eb6c2d145427e1a2ba545c5d385e4bf16eeac05a0eaddd728732`。rev32/33/34保留原字节，R9闭合仅限dsh原报告确认的三项，整体P1仍partial。[收口回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-171658-p1-r9-closure-rev35/RECEIPT.md)。


## P1-01设计后继登记 · rev36

2026-11-14 P1-01设计交回：追加rev36，D10/I12增量提出SG/1结构、群与几何公共设计及来源/独立演算；完整14组接口仍有6项精确缺口，本卡partial、待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受。R9闭合仅属被审rev34，历史补档保留；02–06未施工。不进入产品实现或运行验证，真实worker／Check／launcher另行授权。

[baseline-rev36.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/s01/baseline-rev36.json)链接rev35原SHA；本卡before/after/diff及字段/演算在[新证据包](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/README.md)。rev35及R1–R9、历史补档原件不改；新的导航摘要不冒称原独审已覆盖。


## P1-01并发裁定合入 · rev37

2026-11-14 P1-01设计交回：当前rev37、D10/I12/C6；在rev36部分设计上合入并发用户裁定DEC-P1-04，已按WC01–WC03说明本卡WordCraft分层/依赖适配。SG/1完整14组仍有6项精确缺口，本卡partial、待用户手动转dsh。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受。rev35/36、R1–R9及历史补档保留；02–06未施工，产品/worker/Check/launcher运行未授权未验证。

[baseline-rev37.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/s01/baseline-rev37.json)链接原rev36摘要，requirements-at-rev37.md封存新增用户要求及当前头。


## P1-01同卡F1–F6后继 · rev38

2026-11-14 P1-01同卡F1–F6整改：追加rev38，D11/I13/C6；另建SG/1-r2证据后继，修复数学引用、明确角色权威及四条旧表冲突、登记保留AGENTS后继、补13条MODEL/VECTS入口，并补schema范围/语义拒绝例及归档基路径。rev37旧SG/1已获dsh“主体准确、partial、轻微证据整改”结论；本次新增量仅Codex自验、待用户手动转dsh。完整六项科学缺口不变，P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial、2接受/2未接受。02–06未启动，产品和真实运行未授权未验证。

[baseline-rev38.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/s01/baseline-rev38.json)按SHA链接原rev37；AGENTS后继作为当前真实输入单列，原rev35–37/旧封存不改。新字节和新类型范围仅自验、待手动独审。


## 当前续办：P1-01 F1–F6独审结论登记（rev39）

2026-11-14 P1-01同卡独审结论登记：追加资料rev39，技术版本仍D11/I13/C6；dsh已独立复审通过rev38的F1–F6证据/边界修补，最新独审指针指向该被审包及真实JSON/SHA。本次rev39仅回写结论并Codex自验，O1登记为非阻断报告导航说明，不改旧封存或AGENTS。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受，SG-01…SG-06六项科学缺口不变。02–06未启动，产品和运行验证未授权。

最新[dsh独审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/REVIEW.md) / [真实JSON](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/review-result.json) / [本次登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/RECEIPT.md) / [O1非阻断说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/nonblocking-observations.json)。

O1：旧oracle-bindings.currentVerificationEvidence定位04-full正常模式546项报告；封存只读final为570项，两者均exit0/0失败。final入口为[/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json)。本次只登记该导航边界，不改旧字段、旧报告或封存。独审被审版本是rev38，rev39仅为结论回写且尚无新的独立复审。
