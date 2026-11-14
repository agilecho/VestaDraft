# VESTA Study 接续

计划：`docs/superpowers/plans/2026-11-08-vesta-study.md`。
证据：`/Users/agilecho/code/Draft/output/vesta-study/20261108-113622`（历史 Craft 前缀按同后缀核SHA重定位）。
台账：本轮R7（20261114-152228-p1-design-r7-errata）/progress.md；R1–R6及原研究总账保留历史。

## 当前状态

2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

当前入口：[R4回执](../../output/vesta-study/20261108-113622/reviews/20261114-124229-p1-design-r4/RECEIPT.md)、[用户手动转交dsh材料](../../output/vesta-study/20261108-113622/reviews/20261114-124229-p1-design-r4/DSH-REVIEW-HANDOFF.md)。当前review_result直接指向[真实dsh R3结果](../../output/vesta-study/20261108-113622/reviews/20261114-122114-p1-design-r3-dsh-review/review-result.json)，用户授权后的P1.3设计接受仅在独立接受记录和当前状态，不能把作者转录当原始review_result。原R3的错误转录保留历史字节，R4忠实摘要保持原文字段false。

R4生产浏览器来源表为空，实际worker/Check/launcher仍未授权/未验证；C6新增合同只是设计。原254ID/104样本/2037证据/24问题10/3/11及完整F1/F13保持。下一动作仅用户手动转交增量材料，不自动联系dsh。

[Study 报告](STUDY-REPORT.md)、[S08实际dsh裁定](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/review-result.json)、[3项文档处理](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/review-finding-resolutions.json)、[601文件封存验收](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/seal-verification.json)。

[S07交付矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/S07-MATRIX.md)、[实际dsh裁定](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/review-result.json)、[复审后纯文档处理](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/review-finding-resolutions.json)、[最终自验](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/packet-verification-accepted.json)。封存报告位于同交付目录，交付后停止。

[S06交付矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/S06-MATRIX.md)、[实际dsh复审](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/review-result.json)、[轻微问题处理](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/review-minor-resolutions.json)、[封存验收](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/seal-verification.json)。

[S05矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/S05-MATRIX.md)、[输入](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-format-matrix.json)、[输出](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/output-format-matrix.json)、[新输入字段](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/new-input-analysis.json)、[编辑](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/gui-edit-analysis.json)、[新进程](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/fresh-analysis.json)、[独立PRI](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/independent-pri-analysis.json)、[独立mesh](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/independent-mesh-analysis.json)、[unknown](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/unknown-heads.json)、[自验](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/packet-verification-final.json)、[实际dsh](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/dsh-review-resume.stdout)、[裁定](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/review-result.json)。本轮有原件GUI/CUA，无产品实现。所有目标失败、setup/分析错误和有界终止保持。

## 执行裁定

2026-11-10 上游更新影响已单列[分析](../参考/CraftAnything-20261110更新影响分析.md)：活动 `67b9510` 不替换锁定方法/CLI，当前研究不因更新自动重做。继续用 METHOD 的原运行件；升级时另验 Mach-O 输出与 rebuild v4，当前 Study 状态不因本次分析变更。

当前目录非 Git 仓库，使用新证据目录及本文件接续，不运行 Git-only 工作区脚本、不初始化仓库。Study 属研究/文档，按真实观测与产物验收，尚无产品代码测试。原始日志及既有失败保持。

## 已执行与恢复

- S02 正文摘要：[SOURCE_NOTES.md](SOURCE_NOTES.md)与[来源续读](SOURCE_SUPPLEMENT.md)。XSF旧失败保留，续作已读官方HTTP正文；其余格式/论文/目标版本缺口仍明确。
- S03 模式：Deep；准确 CLI 身份、隔离配置、原生调查、恢复命令和 Check 前提见 [METHOD.md](METHOD.md)。工具配置 `output/vesta-study/20261108-113622/config`，正式调查 `vesta.cany`，`roundtrip.cany` 仅为持久化验证副本。
- 原件研究没有启动 rebuild/builder 或产品实现；独立复审已实际调用 dsh，不能把早期“模型调用 0”当当前复审调用统计。Ghidra 未安装；已有工具能力和限制见 METHOD。VESTA 的“Model Electron Densities”是科研计算，不是 AI 模型请求。
- 当前24个原生unknown：10 resolved、3 investigating、11 open，最新身份和待办见 [UNKNOWNS](UNKNOWNS.md)。问题数不是全部未研究功能的计数。
- S04 首个探针：`s04/startup-capture.json/.stdout/.stderr`，使用 `VESTA_PREF=…/s04/preferences`；预算 120 秒。每次探针结果先检查错误/限制/产物，不能只看外层脚本退出 0。

## 当前交互实例与已回答问题

首轮受控 VESTA PID 11307 已正常退出：`s04/interactive-session.json` 记录退出 0、未超时、1707.521 秒。该进程的偏好指定本轮 `s04/preferences`；目标身份复核限 S01 已记录的四个文件，不冒充整个发布包或默认偏好逐字节审计。CUA 首轮已完成 completion_summary；重新观察时，压缩上下文后先 `cua.rewriteDocumentation()`，再取最新状态，不用旧索引，不用 getApp 自动启动默认偏好实例。

- GUI 建立 C1/P1、4 Å 立方体，体积 64、默认边界显示 8 个角原子；2+2 失焦转 4.00000。
- Properties/Preview 临时关闭 compass 后 Cancel 会恢复；Boundary/Apply 后 Cancel 则保留已应用范围。边界 max 1→0.5，Output 显示原子数 8→1；Undo/Redo 重开参数为 1/0.5。
- +x 工具栏一次旋转，取向读回标准 Rx(45°)；工具栏展开实读步长 45°、10 px、10%。
- 选中原子 Delete 隐藏，Esc 恢复。三份保存工程为 `s04/temporary-C1.vesta`、`temporary-C1-hidden.vesta`、`temporary-C1-restored.vesta`。隐藏只改 DLATM；恢复文件与初始文件摘要相同，比较在 visibility-artifact-comparison.json。三个文件尚未完成独立实例重开验证，留 S05。
- 新版 Powder Diffraction 独立窗口的 Conditions/Reflections/Plot 已观察，条件和空反射表已留树；未 Calculate。
- MADEL 输入两个无 Cancel 按钮的提示：radius=1 Å、region=2 Å^-1；生成偏好 tmp 下 .pme，界面短暂禁用后恢复，未见计算输出。R12已定位原包MADEL的绝对GNU Fortran库缺失及GUI无结果静默分支；不能记零能量通过，S06/S07领域算法/数值及完整外部契约仍待查。
- 已用 0.5 Å 分辨率生成 Model Electron Densities，F000=6、Summary 为 8×8×8。保存 `temporary-C1-density.vesta` 时实际生成相邻 `temporary-C1-density.pgrid` 并相对引用；摘要见 generated-data-manifest.json。原生 capture 与 GUI 重开均已完成：工程没有恢复体数据，pgrid 单开正常；不要再次计算或重复这两条探针。
- 2D General/Contours、三种 Slice 模式已留原生树。有效 (001)、2 Å、[0,1]³ 包围盒切片显示 Z(min)=-1.211、Z(max)=0.16857；File/Export 2D Data 仍禁用，Raster 可用。双向量/投影只检查输入，未计算；2D 窗口关闭后退出主实例。

CA `runtime-ui-read` 默认截图选到 66×20 的共享附属窗口，因此后续 capture_ui.py 只取原生树；不把这些小图当布局证据。`startup.png` 为可用的启动主窗口图；`about.png` 实为 66×20，保留失败产物，不能证明 About 外观或版本。版本仅由 `about-read-summary.json` 的 AX 文本及对应原生 evidence/unknown 证明。UI 动作用 CUA；原生树保存用真实 CA CLI，不伪称 CUA 是 CA 回执。

## 第二轮实例及资源差异检查点

第二轮隔离实例 PID 16742 已正常退出：`s04/interactive-session-2.json` 记录退出 0、未超时、914.049 秒；原执行会话 6026 已结束。`s04/target-runtime-cwd.stdout` 核实 CWD 为 s04。CUA 第二轮已 completion_summary。退出后误取 AX 曾重新拉起两个默认偏好空白实例，随后均退出，最后 cua.getState 实读 isRunning=false；不能在退出后再向 app 读 AX。

Deep 已研究库中 `readVESTAfile`（arm64 0xa185c，19620 字节），完整回执 `s04/library-vesta-reader-study.stdout`，evidence `ev_d11994a88496f8c507dac1cf00b225d7467a10a52fbda8d74b6268a54572ea82`。输出存在 322 处未处理内联汇编等限制，不能由寄存器级伪码直接断定资源失败原因。另研究 `check_format`（0x8d83c，2388 字节），回执 `s05/format-recognizer-study.stdout`，evidence `ev_75a00d2a1975476931ae692351e7c3ee9e960be2addc3bc23aafedc531fd7955`；也有寄存器/分支/未处理指令限制，不当完整源码。

## S05 早期已回答探针（以下计数为当轮历史）

- [FORMATS](FORMATS.md)：实际结构导出 15 项、体数据导出 11 项，入口观察和真实产物分开；输入 47 类仍是旧手册范围，当前识别清单未核完。
- [SAMPLES](SAMPLES.md)：37 个稳定样本及 SHA-256。官方 ZIP 142 成员，本地清单已保存；不当可再分发产品资产。原始样本哈希在独立比较中复核未变化。
- `run_format_read_probes.py` 已完成 34 个每例 15 秒的原件探针，无超时；**不能重跑覆盖**。原件全部退出 255，详见 read-probe-summary.json。MXDORTO 明确 Failed to read，随后仍生成空工程。
- 非正交 P1 的晶胞、坐标、占位和 Biso 与独立预期相符，体积显示误差 3.09e-7 Å³ 在 6 位小数舍入界内；中文空格路径副本输出字节一致。证据 nonorthogonal-semantic-comparison.json；未实测 Uiso 转换或 GUI 编辑。
- 三份 C1 工程经 -nogui 读取再保存，结构/隐藏块保留但 SCENE 的 Rx(45°) 被重写为 I；不可记完整工程往返通过，GUI 读回矩阵仍待核。
- 密度工程：绝对引用、短名引用、官方 KOD 工程均无体数据，两个 pgrid 单开正常。当时未知到 revision 3；现已在READER_PATHS解释拼接失败并到revision4/resolved，不再重复路径变体。
- MXDORTO：全路径与原样本目录中的相对 FILE07.DAT 都失败，当时unknown revision 2；现已闭合大小写分支及固定宽度/越界机制，到revision4/resolved；只补空格的受控变体被原件保存为300位点，原样本失败保留。
- 截断pgrid及a=-2的CIF仍进入读取/保存；R13已解释短读复用标量/负长度词法转换与场景转发，unknown2 resolved。仍不当输入有效，其它约束/资源/错误状态未全验。
- 真实 CA 终端记录已跑 CIF、截断网格、负晶胞、MXDORTO 两种路径，退出/进程组/文件摘要有原生回执。普通直接原件日志与 CA evidence 明确分开。准确参数及单次 --allow-run 授权说明见 METHOD。

## P1复审后文书/绑定收尾（2026-11-11）

[dsh复审I4当前字节](../../output/vesta-study/20261108-113622/reviews/20261110-p1-design/dsh-review-I4-final.md)确认R3-1/R3-2实质解决，P1仍partial；产物 `R/dsh-review-I4-final.md` 与 `.json`，`review-result-effective.json` 已标 `independently_reviewed=true`，不改2026-11-10原裁定。封存后5份文档因外置卷Samsung_T5→T9做路径迁移（需求/索引/README/研究计划/更新影响分析，无科学语义）；rev25登记并归档现行需求正文，`BASELINE.md` 的rev22段恢复当时原文，研究计划头由rev22改为rev25。I4四处文字残留（§3表、§9.2签名、§8.2方法归属、文首范围）已清。本轮未创建产品/代码/worker，未跑model builder或原生Check；P1.2–.4未完成。

## 历史：2026-11-11的下一动作

**当时停止点：P1部分交付（2026-11-11）。** [dsh正式复审](../../output/vesta-study/20261108-113622/reviews/20261110-p1-design/dsh-review-final.md)和[同session差异复核](../../output/vesta-study/20261108-113622/reviews/20261110-p1-design/dsh-review-delta-final.md)均为partial，只有P1.1接受；P1.2–P1.4保持未完成。本会话未创建产品目录/代码/worker、不跑模型builder或Check，不自动进入P2–P5。

1. 先读[架构D2](../superpowers/specs/2026-11-10-vesta-p1-design.md)、[接口I4](../superpowers/specs/2026-11-10-vesta-interfaces.md)、[CA接线C2](../superpowers/specs/2026-11-10-vesta-ca-wiring.md)、[模块计划](../superpowers/plans/2026-11-10-vesta-modules.md)。dsh接受的是D2路线、I3/计划rev3主修正；I4两处追加补正已于2026-11-11经dsh复审，R3-1/R3-2实质解决（R/dsh-review-I4-final.md）。
2. 路线：Rust原创领域/codec/科学内核，WASM单Worker，TS/React/Three.js WebGL2，同内核原生CLI。未来workspace /Users/agilecho/code/VestaCraft、产品root app/，尚不存在。DEC-P1-01默认复现已证行为并提示风险、修正显式；DEC-P1-02默认原生.vesta＋资源，不重问。
3. 继续P1.2时按接口§7/G1–G4，先补影响公共payload的群/磁群/调制、多相R/t、ADP/键价/等值面/投影/Nyquist和全codec边界；已有31组nextProbe保留，不把所有unknown关闭当新门。基础身份/修订/事务/保存草案可独立设计，不自动施工。
4. P1.3仍缺prepare工具精确构建/输入、Study派生迁入绑定、实际target/request/stack→kind、launcher/worker/Check/恢复的完整桥接设计。原生真实联调需要后续产品/模型授权，当前不以空Check或builder补证。检查请求是checks/asks，不是离线检查器，同进程API自查拒绝要结束turn再Check。
5. P1.4在相应schema冻结后，把G1–G6写成精确文件/函数/独立断言/命令/资源任务。当前254ID均有唯一主责，仅为安排，不是全功能覆盖或可施工验收；16条原CLI的相关VF导航见计划文末，不假称原合同带VF字段。
6. R=E/reviews/20261110-p1-design有实际预编辑副本、CA锁定27份源码/平台18份快照、一手依赖获取、两个有界本机probe和全部dsh轮次。平台HEAD a62fb23b/72既有状态、18文件与S08相同；CA继续1f8508a1。probe不证明完整Web/科学/性能/x86。
7. Study29/29、254功能/104样本/84入口pending、2037派生非raw原生索引、24头10resolved/3investigating/11open保持；functions.json与旧封存原字节不改。资料基线rev25（T9路径迁移+I4复审）；机器状态/总体计划只勾P1.1。下一步仍是剩余P1设计，不是P2实施。


## 复审前检查点验收证据

[checkpoint-verification.json](../../output/vesta-study/20261108-113622/s05/checkpoint-verification.json)实际通过：254 条功能字段/ID/证据引用、37 个样本摘要、324 个本地链接目标、计划仅勾选真实完成的 11 步，仍有 18 步未勾选；原件 S01 四个身份文件摘要不变，锁定方法 HEAD/Cargo.lock/CLI 摘要不变、上游干净，未残留目标 VESTA 进程。此检查不代替未完成的领域研究或产品验收。

原生调查已导出 [checkpoint-evidence.json](../../output/vesta-study/20261108-113622/s05/checkpoint-evidence.json)：84 条 evidence、8 个 unknown。导入全新 `checkpoint-roundtrip.cany` 实际返回 84 imported、0 rejected、8 unknowns added，见 [回执](../../output/vesta-study/20261108-113622/s05/checkpoint-import.stdout)。该目录只用于验证持久化；继续工作仍使用正式 `vesta.cany`，不能在往返副本中更新权威状态。

## S01 独立复审修订

2026-11-08 对 dsh 反馈逐项复核，S01.1–.3 勾选保留。已纠正 About 小图的证据用途，补列 Info.plist/LICENSE 摘要，并在 [BASELINE](BASELINE.md) 登记需求文档旧/新摘要、文件修改时间和回写原因。原 11:36 baseline.json 和 About 图片均保留原字节；后续版本状态及资料修订从 [baseline-rev2.json](../../output/vesta-study/20261108-113622/s01/baseline-rev2.json)读取，未回改历史 pending 字段。

此前校验只核本地链接存在，未校验基线引用资料的摘要演进，也不证明链接图片的内容/用途。本次增加资料修订与原图尺寸/摘要校验；独立重跑命令、修改前文档和复核报告保存在 [复审记录目录](../../output/vesta-study/20261108-113622/reviews/20261108-130129-s01/)（最终结果文件 `verification.json`）。原 S05 checkpoint-verification.json 保持历史记录，不覆盖为本次结果；这是S01当时的接续状态；S02之后继续研究应消费BASELINE/rev3及下节修订。

## S02独立复核修订（2026-11-08）

S02.1保留[x]；S02.2在重新核正文/小节并补脚注后保留[x]；S02.3调整[ ]，未为保留旧勾选降低专业来源核验标准。当前已完成步骤为10项，未完成19项；旧S05/S01报告中的11项是历史状态，不回写旧报告。

本次[复核目录](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/)保留修改前文档、源码身份/摘录、新获取/目录/来源登记、diff与新验证。旧21个HTML摘要均可由本次官方响应复核，摘要对象明确为HTML；临时文本不是永久归档，旧记录也没有访问时间。专业文献135条只读书目条目，未宣称全文或算法已核；45个异站格式/程序外链及同站Dysnomia和脚注另登记。

目标3.90.9a属下载页开发分支；旧手册2022-08-10与changes到3.5.8不能覆盖新版全部功能。新增来源缺口 `unk_e1a118c694cc99ea` revision1/high/open，正式vesta.cany读回共9项；原8项记录与84条evidence未改变。本次未运行VESTA/GUI/科学探针/rebuild/builder或模型任务，未更改全局配置。

新增基线[baseline-rev3.json](../../output/vesta-study/20261108-113622/s01/baseline-rev3.json)登记需求文档本次状态纠错的摘要/正文版本，原baseline/rev2保持字节；最新检查结果见[本次检查目录](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/)中的 `verification-acceptance.json` 与 `s02-integrity-acceptance.json`，它只核资料身份、引用和状态一致性，人工正文结论见SOURCE_NOTES/复核结果。

## S02.3来源续读（2026-11-08）

已读范围和剩余清单见[SOURCE_SUPPLEMENT](SOURCE_SUPPLEMENT.md)，逐份记录见[25项阅读台账](../../output/vesta-study/20261108-113622/reviews/20261108-133154-s02-sources/source-reading-ledger.json)。4批35次获取分开保留HTTP/TLS结果、字节摘要及阅读裁定；XSF官方HTTP两次200，CIF续读、PDB指定章节、CUBE/CONFIG/VRML和RIETAN/MADEL/ORFFE等补证完成各自注明的范围。RIETAN旧域名已失效转无关站，现定位JP-Minerals官方恢复文档；官方VESTA下载PDF封面2019与2022HTML分开。没有把付费摘要、空白格式页或归档下载记为全文阅读。

原生 `unk_e1a118c694cc99ea` 真实更新至revision2/high/investigating，正式读回9项（1resolved/3investigating/5open）；84条evidence及原8个问题不变。更新接口不修改question，故旧问题的XSF失败表述为创建时快照，probes与阅读台账给当前状态。未触发GUI、原件/外部科学运行、反编译、builder或模型。S02.3仍[ ]，总10项完成/19项未完成；没有修改8项基线引用或四个目标身份，不新增baseline-rev4。

第一轮验证入口为[该轮证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-133154-s02-sources/)及其README；旧复核报告保持历史字节。后续进展见下节，目标行为仍由S04–S07单独取证。

## S02.3格式与科学来源续作（2026-11-08）

已补生产者CTfile2020、SHELX97/当前指令的指定章节、X-PLOR当前地图页；Chem3D读写代码固定OpenBabel2.3.1提交并核标签/提交字节一致。STL读作者书节和4张原图，原厂商规范未得；Baur1974经作者上传原论文web通道补选定正文，未声称21页全读/公式视觉通过。Hoppe核出版页采用的DOI及印刷/上线日期，正文仍受限；Robinson/Brown未读原文，Crossref429/403均保留。

15项续作台账与[本轮证据](../../output/vesta-study/20261108-113622/reviews/20261108-135713-s02-formats-science/)配套[SOURCE_FORMATS_SCIENCE](SOURCE_FORMATS_SCIENCE.md)及[47输入/23数据输出来源表](FORMAT_SOURCE_COVERAGE.md)。表是逐项来源图，不是格式通过数；VESTA原件仍为254功能/37样本的既有状态，没有新增运行或产品实现。来源问题真实更新为revision3/high/investigating，正式vesta.cany读回9问题，84条evidence/原8问题不变。S02.3继续[ ]、总10项已完成/19项未完成；需求/BASELINE/基线引用不变。

下一步按覆盖表补剩余生产者规范和字典/扩展，再核核心科学正文；已知不可读的出版正文保留状态，避免重复相同下载。当前验证以本轮README和验收报告为准；旧两轮证据保持原字节，临时原文不冒充永久完整归档。


生产者追加选读已补FEFF、FHI-aims、Elk、GSAS、WIEN2k、CASTEP、GAMESS/MacMolPlt，另有[8项台账](../../output/vesta-study/20261108-113622/reviews/20261108-135713-s02-formats-science/producer-reading-ledger.json)，与本轮前15项合计23项。各版次、实际章节及rho/charg_frm/GEOMETRY.OUT等剩余缺口已回写来源续读与覆盖表；来源问题继续revision3/high/investigating。

## S02.3体数据与科学来源接续（2026-11-08）

本轮9条来源续记含6处新增正文选读、2处摘要/定位及1处重复版本参照；完整范围见[SOURCE_VOLUME_SCIENCE](SOURCE_VOLUME_SCIENCE.md)和[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-143347-s02-volume-science/)。rho/charg_frm/Elk writer与Chantler1995/Tsirelson2002均有补读，目标支持、版次绑定、离散算法与其余正文仍未核。NIST目录仅给2000论文DOI，1996勘误未得；不把1995原表视为修正后全部表值。

来源问题接续revision4/high/investigating；9问题/84evidence及其余8问题不变。计划仍10项完成/19项未完成，S02.3不勾选；需求/BASELINE不变，不新增baseline-rev4。最新验收读本轮README，旧三轮证据保持历史字节。未运行原件、外部科学程序、模型或builder。

## S02.3字典与历史格式接续（2026-11-08）

本轮8条记录含6处正文选读、2处入口/摘要，读四字典指定数据项、USPEX输出选段和CMTX版本变更，见[SOURCE_DICTIONARIES_LEGACY](SOURCE_DICTIONARIES_LEGACY.md)及[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-145707-s02-dictionaries-legacy/)。core3.3.0晚于目标构建；磁性/调制字典导入依赖、dREL方法、全字典和目标解析均未整体验证。AMCSD未取得字段正文。

来源问题接续revision5/high/investigating；9问题/84evidence及其余8问题不变。仍10项完成/19项未完成，S02.3不勾选。需求/BASELINE不改、无baseline-rev4，旧四轮清单保持历史字节；当前验收读本轮README。不重复本轮选读或已知相同受限下载，后续补剩余来源/版本，再按既有S04–S07问题取证。

## S02.3历史生产者与来源绑定接续（2026-11-08）

本轮7条记录含5处正文、1条失效入口及1条上下文复核，见[SOURCE_LEGACY_PRODUCERS](SOURCE_LEGACY_PRODUCERS.md)与[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-152746-s02-legacy-producers/)。FDAT补CCDC指定规范，CSSR/CRYSTIN消费说明按来源效力标注，配套TXT验证码页不算成功取文。特别澄清：VM18 rho指wien2venus三维输出，LAPW5二维case.rho只能作不同生产者的同后缀参照，原脚本未得。

来源问题revision6/high/investigating；9问题/84evidence及其余8问题保持。S02.3仍[ ]、总10项完成/19项未完成；需求/BASELINE未改，无baseline-rev4。前五轮清单字节保全，最新验收见本轮README；未运行原件/外部科学程序/模型/builder。

## S02.3 MOLDA、C04D与XTL接续（2026-11-08）

本轮7条来源记录含4处新增正文、2处失效入口和1处VM18复核，范围见[SOURCE_REMAINING_FORMATS](SOURCE_REMAINING_FORMATS.md)与[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-155158-s02-remaining-formats/)。补MOLDA数据集作者格式、坂根2026-02-08手册C04D指定页、GULP5.1输出段及固定CP2K2025.1消费者源文件。ASSE/eb作者入口404；eb不能套LAPW5二维rho，SCA完整字段/原始Readme仍缺。首次CA读回因相对路径未启动，失败回执保留；绝对路径另存真实成功读回。

来源问题revision7/high/investigating，9问题/84evidence及其余8问题不变。S02.3仍[ ]、10项完成/19项未完成；六轮450个历史清单文件保全。需求/BASELINE未改，无baseline-rev4；原件、外部科学程序、模型和builder未运行。接续前先读本轮README，避免重读已读页或重复失效入口。

## S02.3 梯度、参数与MEM接续（2026-11-08）

见[SOURCE_SCIENCE_PARAMETERS](SOURCE_SCIENCE_PARAMETERS.md)与[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-160825-s02-science-parameters/)。Kirzhnits期刊PDF附翻译勘误已核指定范围，ERIS实际封面7月28日、兼容prf而非旧pri/den。MINCRYST旧路径失效后现站可读，仅跟一张Card1及初始信息，不运行Java。bvparm2020补参数loop库存1934条、463条B非0.37，选择逻辑仍须查目标。Brown/Waasmaier镜像超时不记正文。

来源问题revision8/high/investigating，9问题/84evidence及其余8问题不变；S02.3仍[ ]、10完成/19未完成。七轮525历史清单文件保全，需求/BASELINE不改、无rev4。后续优先原生Deep查read_density_data路径/返回与MXDORTO识别固定列，保留未得来源，不让受限资料的相同下载阻止其它已授权研究。

## S04/S05 读取路径机制接续（2026-11-08）

见[READER_PATHS](READER_PATHS.md)与[本轮证据](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/)。原库识别函数12个名字、43项结构跳表及有限分支已核；密度错误204字节路径由临时副本fopen轨迹与未改原库调用逐字交叉验证。密度问题revision4/resolved/valid=true只表示诊断闭合，原件往返仍失败。VS-0033同inode大小写两例都失败；小写副本在BuildCrystal2出现EXC_BAD_ACCESS，MXD问题revision3/investigating。

当前104evidence/9问题，2resolved/2investigating/5open；原84证据保留，新增20，来源问题8未变。254功能/37样本、10完成/19未完成仍不变。原包257个普通文件全部摘要未变；临时副本只改主程序签名，两架构TEXT区节与库完整字节均相同。副本、独立原库调用、原件CLI观察必须分开消费。研究调用器编译不是产品施工或模型builder；没有系统调试设置修改。后续继续MXD内容/建模、GUI取向/状态读回及剩余格式/科学/CLI契约，不以本轮检查作为Study完成。

## R10 MXDORTO失败机制接续

[本轮证据](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/)、[解析/索引交叉核对](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/mxd-cause-crosscheck.json)、[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/mxd-unknown-verify.stdout)。首3行逐字段原函数轨迹、300位点/900坐标的原件变体导出已交叉核。14条新增后共118条evidence；3 resolved/1 investigating/5 open，版本/密度/来源等其它8个问题不变。启动超时另由原件sample确认为AppKit恢复上次窗口的NSAlert，不能作为格式失败；后续启动已自行不再阻塞，未观察该状态如何改变，没有点击弹窗或修改全局恢复设置。下一步继续GUI取向读回等未完成项。

## R11 CLI完成状态机制

[CLI_COMPLETION](CLI_COMPLETION.md)给出parse→OnInit→CallOnInit→wxEntry的0/0/0/255链，原程序全局字节直接为255，构造体复制到返回字段。unknown_b84现revision2/resolved/native valid=true，130 evidence、4 resolved/1 investigating/4 open，其余8项不变。没有重跑旧有效/无效转换对，也没有产品实现。S07进入选定补证但未勾选；继续未完成格式/GUI取向/验证边界/MADEL及完整CLI矩阵。未归属VESTA实例继续保留，未操作。

## R12 MADEL外部启动阻塞

[MADEL_COMPLETION](MADEL_COMPLETION.md)记录原包x86_64程序身份、GNU Fortran绝对依赖、原件SIGABRT(-6)及完整dyld日志；使用既有PME逐字副本，未安装依赖或改原件。OnMADEL丢弃wxExecute返回、缺结果文件直接清理。历史GUI的子程序stderr未捕获，本轮不伪作历史直接日志。问题revision2/resolved/native valid=true；136 evidence、5 resolved/1 investigating/3 open，其余8项不变。继续输入校验边界、GUI取向、来源/领域与CLI未完成项。

## R13选定输入接受机制

[INPUT_VALIDATION](INPUT_VALIDATION.md)：512载荷读取仅2项成功/510短读复用最后标量，内部729含周期边界；负晶胞经数值转换、CIF场景、建模SetC2Ortho和保存均保留-2。unknown2 resolved/native valid=true，151 evidence、6 resolved/1 investigating/2 open，其余8项不变。广泛断点启动超时、19个回调KeyError版本及一次错误CLI路径读回均保留；最终轨迹干净。下一步继续数据库空白、精确截图窗口、GUI工程取向/dirty、完整格式和领域/CLI契约，原样本矩阵不重跑。

R13末次复核观察到上游活跃HEAD变为56aa31976b4ff0950e6b34ba020b0a568d2462fa，工作树干净；本轮未修改上游，变化来源未归属。锁定CLI摘要不变，方法提交仍1f8508a104134c436cbba5ea2f675619ef47702f。提取该提交的Cargo.lock/Cargo.toml、apple_runtime.rs、native_ui_helper.swift和CLI main.rs选定源码快照，五个当前活跃文件也与锁定提交相同；见output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/method-source-snapshot.json。保留最初Method HEAD changed失败报告，不回退上游、不新建worktree、不更新方法/目标基线。后续工具源码一律读该快照或git show锁定提交；未在快照中的新增源码也须按锁定提交读取。当前校验门禁核锁定对象/快照和CLI，另如实报告活跃HEAD，不再以未归属仓库HEAD必须不动为条件。

## R14窗口选择接口与GUI限制

[WINDOW_CAPTURE](WINDOW_CAPTURE.md)：锁定CLI已有window-id，默认同PID首个零层窗口不保证目标对话框，AX仍进程范围。错误ID负控制真实not_found/noimage，正确窗口正控制未通过，unknown2 investigating。唯一包标识副本被VESTA自身单实例IPC busy提示挡住，CUA无动作/截图超时；copy10.185秒后255退出，来源未定，研究路径无进程，CUA已summary。不要再次启动GUI碰撞、关闭未归属原实例或改IPC键；继续可独立进行的数据库/源码/领域研究。153 evidence、6resolved/2investigating/1open；19步骤仍未勾选。

## 独立复审职责裁定（2026-11-08）

用户最新AGENTS约定已替换此前同类约定，见根目录AGENTS.md：Codex继续直接执行研究、施工、自验、验收文档和执行证据；独立复审统一由dsh承担，不再使用Hermes复审口径。向dsh安排独立复审不等于授权dsh施工，也不授权开始产品实现。本轮各完整性脚本属于Codex自验，未冒充dsh独立复审。后续需要独立复审时沿用此裁定，不重复请示复审归属；历史复审和原证据保持。

当前AI入口及协作方法文档已同步dsh独立复审。S01锁定来源docs/接入技术要求.md保留历史字节；其中旧Hermes复审职责由根目录AGENTS.md的最新用户裁定覆盖，不再作为当前复审规则。未修改目标/资料基线，未产生rev4。

## R15数据库空白诊断与GUI恢复

[DATABASE](DATABASE.md)：用户确认81929为研究遗留实例可正常关闭，复查已退出，无需再问。原件空白GUI经Don’t Reopen后可用；HTTPS真实显示COD表单，HTTP仍空白。同TEXT副本两次直接观察NSURLErrorDomain/-1022，空白原因已解释；查询/提供者/导入未核，数据库unknown2 investigating。169 evidence、6resolved/3investigating/0open，其余8问题不变。原件-i GUI启动崩溃、nscurl错误及Quit后AX自动重启均保留，后者已菜单退出；今后Quit后仅清单/ps验证。GUI阻挡已解除，可继续受控原件GUI取向/dirty与数据库公共查询，不沿用R14“不要启动GUI”的临时阻挡措辞。显式窗口截图正控制仍未完成；Study仍10勾选/19未勾选，Codex自验，独立复审归dsh。

## R16原件GUI取向与文件状态

[GUI_LIFECYCLE](GUI_LIFECYCLE.md)：GUI读回/初始Save As完整保留Rx45且文件字节相同，nogui改I是独立路径。a4→5→6体积64→80→96；第二次关闭提示Cancel明确保留，第一次提示消失/保存动作未观察。外改工作文件a7/标题后GUI Save覆盖为内存a6/旧标题、未见冲突提示，同实例重开CUA一致；原生重开捕获超时，无Evidence。PID9454正常退出0/564.104s，未deadline。172 evidence、9问题头全不变；254功能/37样本/10勾选19未勾选保持。继续未完成格式/图像/No/Yes/并发/领域与CLI，不重复已答取向；显式窗口正控制仍待核。

## R17八相晶形与首个S06选定数值

[行为02](行为/02-结构与几何.md)/[ALGORITHMS](ALGORITHMS.md)：VS18八个shape-only相，结构Style禁用/晶形可用；相8参考6，Copy暂存9，Cancel重开9→8，原文件不保存。48面独立半空间/多边形面积按6有效数字全部逐字匹配，D/d_hkl=1.51034一致。精确十进制体积与显示差4.14e-5，binary32参数控制可重现显示，未直接追此样本内部算术，不作高精度原因结论。原件19297退出0/319.835秒无deadline，CUA已summary。178 evidence、9头不变；S06已选定启动但所有步骤未勾选，10完成/19未完成。继续磁性/准晶/配位等上下文、实际格式/导出、全领域与CLI；不重做48面核对。

## R18原子距离与成键边界

[ALGORITHMS](ALGORITHMS.md)/[行为02](行为/02-结构与几何.md)：VS11两点独立2 Å，GUI测量2.00000(0)；C/O四组提交区间1.9–2.1、1.9–2、1.9–1.99、2–2.1分别1/1/0/1键。Deep定位float界限先平方/升double、double平方距离包含上下界局部谓词；非运行调用链/全部搜索证明。首Apply0后Tab/重Apply1保留，不把未选行默认字段当实际规则。本轮21065在Quit提示后达到450秒期限exit-15，No动作未执行，不能记No或正常退出通过；样本未改，CUA已summary、无残留。188 evidence、9头不变；254功能/37样本/10勾选19未勾选保持。后续GUI需更早收尾；继续非正交/PBC/角度配位、磁性准晶、文件图像输出/数据库/全CLI等未完项。

## R19普通有名工程的No/Yes关闭

[GUI_LIFECYCLE](GUI_LIFECYCLE.md)：独立C1工作副本均a4→5/80，Quit后明确No/Yes。No27835正常0/97.767s，文件仍原字节a4；Yes27893正常0/101.596s，文件仅CELLP a改5。原生before/after与CUA按钮选择分开，未到期限，CUA已summary/无残留；源不改。结合R16 Cancel，三选项此处已核，不再重跑。R18的期限No失败保持历史；未命名/写失败/多文档/并发恢复仍待。192 evidence、9头不变；254功能/37样本、10勾选19未勾选。下一项图像导出/实际格式与未完领域、磁性准晶/PBC和CLI。

## R20 PNG图像与新实例读回

[IMAGE_EXPORT](IMAGE_EXPORT.md)：Raster10/Vector4实际菜单；PNG1×透明1112×940 RGBA/alpha0..255，PNG2×不透明2224×1880 RGB，原文件独立解码/逐图视觉查看。编码库规范版本、其它格式/全参数未核；Vector仅枚举取消。首次图像代表VS0038新增，原37身份/记录不改，Scale2不新分样本；254功能/38样本，196 evidence、9头不变。新进程28169读回R19 Yes产物a5/80，导出不改工程，正常0/466.398s无deadline，CUA已summary。PNG不是CA截图，显式窗口正控制仍待。下一项实际Vector/PDF/SVG及其它Raster/输入格式/领域/CLI，10完成19未完成。

## R21 Vector实物（不改变计划勾选）

四种Vector实际生成，PDF/SVG独立解析渲染，EPS/PS只结构核验；GL2PS1.3.8为元数据自报。向量蓝色Helvetica12标签/无PNG亮高光，内部原因与全画面等价待核，不能直接混同Raster EPS。原件28517正常0/337.508秒、工程不改、CUA已summary。198 evidence/9头不变；254功能/42样本，新VS0039—0042、原38保持。10完成19未完成，Study未完，Codex自验，独立复审归dsh尚未开始。继续其它Raster实际格式/参数、输入/科学/数据库/CLI等剩余范围。 [图像权威说明](IMAGE_EXPORT.md)。

## R22 Raster格式实物与异常（不改勾选）

九项RasterScale1及TIFF透明变体均成功提示，但JP2无文件、RGB严格解码失败、TIFF透明无效。BMP/PPM/TGA/TIFF解码像素相同，RAW/EPS固定流同RGB，JPEG有损。静态JPEG2000 writer是零返回stub，SGI短头/TIFF自定义RGB路径与实物一致，现场调用绑定仍investigating。原件28787正常0/258.724秒、工程不改、CUA已summary。207证据/10问题（6resolved4investigating），旧9头不变；254功能/50样本，旧42保持，新8个格式实物代表，缺失JP2不编ID。10完成19未完成，Study未完，Codex自验、独立复审归dsh尚未开始。 [图像说明](IMAGE_EXPORT.md)。

## R23 Raster三异常现场归因（不改计划勾选）

同TEXT/原库研究副本实测：JP2临时Save返回1→空转换返回0被忽略→删~→仍成功；RGB index7转自定义type4，坏文件与原件完全相同；TIFF生成实际alpha0..255却index9转RGB-only type6，实物仍与原件RGB TIFF相同。0回调错误/目标正常0，watch本轮未截断、原件不改。问题unk_ff152c8e4008160b rev3 resolved仅指原因查清，三项功能仍失败。208证据、10问题=7resolved3investigating，旧9头不变；254功能/50样本完全保持，副本产物不重复编样本ID。10完成19未完成、完整Study未完，Codex自验、dsh独立复审尚未开始。 [图像权威说明](IMAGE_EXPORT.md)。

## R24角与镜像二面角（不改勾选）

独立四原子XYZ预期键角90°、无符号二面角45°、CF=2√2Å；原件90.0000(0)、两镜像均45.0000(0)、2.82843(0)Å按显示精度符合。静态通用核心是归一化法向点积夹紧后acos，与无符号镜像结果一致；退化/PBC/SUE仍未核。Quit两文档依次保存提示/No，正常0/300.248秒，两源未改，dirty触发点未分离。220证据/10头不变，254功能/51样本，旧50保持、新VS0051，镜像变体不加ID。10完成19未完成，Study未完，Codex自验、dsh独立复审尚未开始。

## R25周期图像与边界（不改勾选）

正交P1双site主像距离9Å，选O(-1,0,0)像后1Å；测量使用所选图像，非自动全局最短像。默认显示4图像原子/2键，x上限1→2→1提交并读回，计数4/2→6/3→4/2。条件边端点枚举一致，但不宣称通用边界/三斜算法或化学成键。原件36492正常0/267.323秒、QuitNo/CIF未改、CUA已summary。227证据/10头不变，254功能/52样本，旧51保持、新VS0052；10完成19未完成，完整Study未完，Codex自验、dsh独立复审尚未开始。

## R26 VS0009非正交补测（不改勾选）

主像2.17945Å、(-1,0,0)像1.32288Å、两O像围C64.3066°与独立metric/点积显示精度相符。真实选中C/O规则是递归extraatoms；122/116→禁止越界10/4→小范围3/1，修改后规则/边界再次原生读回未完成。早期错误中心角79.1066保留，正确新组原子坐标另核。生命周期原37980期限-15，No未成功；AX重启空白38111已菜单关闭，无残留，源未改，不签正常关闭。232证据/10头不变，254功能/52样本身份摘要保持，仅VS9期望补证，其它51记录保持。10完成19未完成，完整Study未完，Codex自验、dsh独立复审未开始。

## R27反演一般/特殊位置（不改勾选）

独立P-1有理数去重C双像/O原点单重；原件C2i/O1a，闭胞10图像，xyz上限.99后3像，反演C坐标/操作实际读回符合。仅不对称显示checkbox1/0，CUA对象3→2→3，Output旧计数不刷新，不凭此宣称内部数组不变。组成CO/F00014仅独立条件预期，Summary未提供读回；Wyckoff函数定位不等完整算法。38361正常0/354.149秒、No/源未改，softstop360前关闭，无重启。241证据/10头不变，254功能/53样本，旧52保持、新VS0053；10完成19未完成，Study未完，Codex自验、dsh复审未开始。

## R28规则配位八面体（不改勾选）

初始Bonds规则空，明确Si/O1.9..2.1Å+Showpolyhedra1后7/6/1；全局Polyhedral/选择中心面，报告六Si-O2Å/平均2/体积10.6667Å³，与独立32/3按显示精度相符。distortion0/elongation1/ECN6仅规则基准，完整公式/非规则待；surfacearea未读回，Utilities/ORFFE未打开。体积/生成静态函数定位，浮点舍入指令存在不当完整算法/现场路径证明。原件38984正常0/309.52秒、softstop前No关闭、源未改。248证据/10头不变，254功能/54样本，旧53保持、新VS0054；10完成19未完成，Study未完、Codex自验、dsh复审未开始。

## R29畸变八面体公式控制（不改勾选）

VS0054轴向拉伸诊断变体四2Å/两2.5Å；独立先验V40/3、mean13/6、BaurD4/39、lambda1.023356、官方手册一次加权ECoN4.314817，实际13.3333/2.1667/.10256/1.0234/4.3148均按显示精度符合。预登记重复加权候选4.342384与实际区分。Robinson/Hoppe全文未得、角方差/表面积/退化/SUE/ORFFE/全公式覆盖仍待。首轮softstop超过/读取拒绝/AX重启空白已关均保留；重试39507正常0/132.653秒、softstop前QuitNo/源未改/无重启残留。252证据/10头不变，254功能/54样本身份保持，仅VS54补变体、其它53记录不改。10完成19未完成，Study未完，Codex自验、dsh复审未开始。详情见权威结构几何/ALGORITHMS及R29证据。

## R30原生主窗口截图正控制（不改勾选）

默认同PID截图仍选66×20附属窗口60844；只读SCK同PID元数据实际定位标题对应主窗口60826/860×728，原锁定CLI显式传ID后得同ID/尺寸PNG，解码原字节并实际视觉确认标题/结构/侧栏/Output。AX仍进程范围。截图unknown rev3 resolved，仅指归属方法/正控制完成；所有模态/缩放/渲染等价/其它功能不由此签通过。GUI39830正常0/147.53秒、softstop前、无保存提示观测、输入parent-watch SHA不改、Quit后无AX/无重启残留。新4至256证据，旧252保持；其余9头不改，8resolved2investigating。254功能/54样本记录全部保持，10完成19未完成、完整Study未完，Codex自验、dsh独立复审未开始。详情见WINDOW_CAPTURE及R30证据。

## R31测距对角SUE与显示（不改勾选）

独立带误差CIF先验d2Å、σsqrt(.000504)=.0224499443206Å。原件实际2.00(3)Å，C/Oσx.001/.002与σa.01正确读回；同TEXT/原库现场formatterσ.02245001638122369、returnprecision2/ESD3，与原件同显示。0.1σ有限扰动/晶胞临时binary32控制解释7.2e−8差，控制与现场仅差3.9e−16；FCVTPS显示向上取整解释括号3。原件214.409秒/副本98秒内均正常0/softstop前QuitNo、watch输入不改/无AX重启。新12至268证据，10头不变，8resolved2investigating；新VS0055、旧54记录保持，254功能/55样本。仅选定P1轴向对角SUE，全部阈值/角/特殊位置/协方差/完整SUE仍待；10完成19未完成、Study未完，Codex自验、dsh复审未开始。

## R32占位/反演密度DC及CUBE单位待办（不改勾选）

沿VS0053 C2*.5+O1*1，中性F00014；原件Resolution2Å输出14、6³。binary独立积分13.9999999786，正28.2841125208/负−14.2841125422分别符合原件两栏（不是复数实虚部），完整CUBE反演差0；102负体素/5峰不作化学峰归属。pgrid/ggrid均fType1，112代表/内部7³索引，本例与全CUBE.5E值相符但非完整压缩规范。CUBE坐标原子单位、标量仍内部密度值，直接按Cube轴积分94.476674≠14，单位合同不通过；新unk_3f5d08b5dc777b37 rev2 investigating/nativeverifyfalse只因未解决，writer/消费者待查。原件50858正常0/326.522秒/softstop前、无Quit提示观测/无AX重启，输入不改。新增5至273证据，旧10头不变，新问题后11头=8resolved3investigating；254功能/55样本身份保持，仅VS53补控制、其它54记录相同。10完成19未完成，Study未完，Codex自验、dsh复审未开始。

## R33 CUBE坐标/标量语义归因（不改勾选）

原库完整case：坐标用bohr换算常数.5291772086/1.88972613285，scalarfloat仅提升double按%12.5E输出；读入向量另换算、scalar%f直接存。原版读R32Cube并明确-saveggrid得到7³raw，全部343值等Cube parsedfloat+周期补边界，胞值精确符合binary32(6*3.149544*BohrA)。内部Å积分13.99999672，本程序往返不使Gaussian原子单位积分94.476674变正确。无现场writer断点/外部Gaussian执行，所有Cube变量仍待。新9至282证据，仅Cube问题rev3 resolved（原因查清、兼容仍未过），其它10头不改；11头9resolved2investigating。254功能/55样本记录保持、无新ID，10完成19未完成、Study未完，Codex自验、dsh复审未开始。

## R34非零结构因子与CuKα1预设精度（不改勾选）

既有VS55P1C/O，外部维护DABAX选定中性系数+独立复指数公式先验。原件dmin9/Ignoreanomalous1得到完整6±轴反射，real/imag/magnitude/d/同角I比均匹配，±100虚部变号；无直接phase列，phase仅分量推导。λ显示1.54059精确代入曾6个θ不符合，失败保留；同TEXT现场计算前d1/模型double1.5405929，独立Bragg得8.8356997149→原件8.83570，输入诊断解释，不改FhOracle。合计36字段按实际打印精度核对，全部scattering/ADP/中子/反常/Custom/误差传播仍未完。原件0/160.954秒、副本0/92.683秒/均softstop前，无Quit提示观测/退出后AX重启。新11至293证据，最终native-evidence-final.json，先前285中间导出不覆写；11头不变9resolved2investigating。254功能/55身份保持，仅VS55补控制、其它54记录不改；10完成19未完成，Study未完，Codex自验、dsh复审未开始。

## R35各向同性位移与占位（不改勾选）

P1既有VS55诊断衍生，独立公式/外部系数先验；B2/4与U=B/(8π²)的原件六行Export逐字相同。独立B0/占位.5/.75符合复数加权和，18反射108字段按实际打印半末位均符合。只指定导入值/计算贡献，不代表EditData提交/各向异性/完整ADP通过。B保存名拼接导致parent watch漏实际结果，真实原名保留、后续native分析取SHA；U/占位parent watch覆盖。三GUI正常0/75.362、72.107、89.558秒且均softstop前，退出无AX重启/残留。新10至303证据，11头保持9resolved2investigating；55身份保持只补VS55/其它54记录不改。254功能、10完成19未完成，Study继续，Codex自验、dsh复审未开始。算法正文见docs/VESTA研究/ALGORITHMS.md。

## R36各向异性位移张量（不改勾选）

VS55正交P1诊断衍生，C/O正定U张量含三种有符号交叉项；独立二次型/外部系数先验，原件18反射90字段按实际打印半末位符合，110与1-10模差可区分trace/3近似。EditData仅选行读回12U分量全部符合、Cancel不Apply。原件53508正常0/134.861秒、softstop前，输入/真实导出parent watch覆盖，退出后无AX重启/残留。静态core/helper已读，原常量2π²及混合精度记录，worker虚目标仍未绑定，不宣称全部ADP或核心通过。新8至311证据、11头保持9resolved2investigating；55身份只补VS55，其它54不改。254功能、10完成19未完成，Study继续，Codex自验、dsh复审未开始。

## R37中子C/O与负H散射（不改勾选）

独立NIST历史H/C/O bc与另一NIST表的两份预测均先于GUI登记；原件Neutron/手输λ2Tab/d9Tab，每例六反射，72字段符合历史行，H−3.739符号保留，2θ11.47834符合Bragg2Å。另一NIST表28字段超输出舍入，差异保留，不推出原包全表来源/版本或当前推荐物理参数通过。web选段可读、raw403分开。原件53840/53952正常0/117.822、105.512秒，parent watch覆盖真实源/导出，退出后不AX无残留。新7至318证据、11头保持9resolved2investigating；55身份只补VS55、其它54不改。254功能、10完成19未完成、Study继续，Codex自验、dsh复审未开始。

## R38单斜P21螺旋消光（不改勾选）

新VS0056独立单斜CIF/明确螺旋操作，先验12完整HKL含2禁戒、10允许、5允许orbit。原件五行M2覆盖全10允许点、±010省略，25先验字段与volume/2a符合；螺旋平移相位不混同。5I事后面积LP模型匹配，与25先验分开；CalcI分字段归一、ReflectionExporter再乘LP/M/scale的静态体与6g格式已读，完整分支未验。原件54201正常0/116.167秒，parent watch覆盖真实源/导出、输入不变，退出后不AX无残留。新9至327证据、11头保持；254功能/56样本，原55记录不变。10完成19未完成，Study继续，Codex自验、dsh复审未开始。

## R39非正交ADP与自动Ueq（不改勾选）

新VS0057单斜P1/Uani/未给Uiso，常规CIF Uij/倒易metric独立先验22HKL含I共132字段全部符合，Cartesian分量另模型42字段超舍入。原件自动Ueq.0207726/.0505589与metric公式符合，粗输出后追加诊断、在选中精确字段前登记，不混入原132先验。原件54624正常0/200.133秒，实际源/导出watch覆盖、无Quit提示观测/退出后AX重启或残留。新6至333证据，11头保持；新VS57、旧56记录不变，254功能/57样本，10完成19未完成，Study继续，Codex自验/dsh复审未开始。

## R40解析Fourier三模式（不改勾选）

新VS0058通用8列单100反射，Fc/Fo/Fo−Fc独立先验3+4i/4.8+6.4i/1.8+2.4i，F0008/res2。三个原件6³共648体素全匹配，最大误差2.51685e−8，积分均符合8；相位/体积归一/显式DC线性关系成立，原件正向Volumetric分支未运行。F000(+/−)零来自导入入口旧原子模型核贡献，非最终积分；初始化零仍未现场绑定。三次Quit明确No，正常0/275.970、171.913、155.396秒，均softstop前，无退出后AX重启/残留。新20至353证据、11头保持；新VS58/旧57记录不变，新增行为03实质规格。254功能/58样本、10完成19未完成，Study继续，Codex自验/dsh复审未开始。

## R41体数据Patterson（不改勾选）

复用VS58解析Fc网格作为输入，独立先验P=.064+.05cos2πx、积分64；原件FromVolumetricData明确选体数据层。216体素全符合，离散卷积最大差2.984e−8、积分64、偶对称残差0，导出保持6³且0原子记录。静态绑定det/N→r2c→复模方/虚部清零→c2r→1/det，未冒称完整核版本。原件55761正常0/392.461秒，softstop360超32.461秒的偏差明记，仍早于hard570，QuitNo保源、无重启/遗留。7新至360证据、11头不变；58样本不新增，前57记录保持、VS58仅追加派生取证。40历史包4738文件保持。254功能、10完成19未完成，Study继续，Codex自验/dsh复审未开始。

## R42原件体数据正向结构因子（不改勾选）

复用VS58解析Fc CUBE，真正运行StructureFactors/VolumetricData、dmin9；六轴反射/24预登记数值全符独立体元积分，±100=3.000001±3.999999i/|F|5，四轴零，M1。I/角度次级预测也符；R33既知floatcell/标量解释末位是另列postGUI诊断。静态det/N→r2c→packed系数取出/虚部符号与运行分开。原件56123正常0/91.373秒、softstop前、Quit无保存提示、输入保持，无重启/遗留。6新至366证据，11头不变；58样本不增、前57记录保持。41历史包4858文件保持，254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。

## R43非等轴四谐波三轴布局（不改勾选）

新VS59四唯一反射100/010/001/111，P1cell10/12/14、F0008/res2。实际6×6×8/288体素全符独立解析场，max4.84371e−8、积分8.00000045；六种坐标排列仅XYZidentity通过，五错误排列max差≥.0118022。相位/布局证据扩至非平凡三轴低频例，未冒称全FFT/Nyquist/空间群。原件56372正常0/153.565秒、softstop前，QuitNo保输入、无重启/遗留。5新至371证据、11头不变；59样本/旧58记录全保持，42历史包4975文件保持。254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。

## R44三轴正向/Patterson（不改勾选）

VS59三轴网格原件正向dmin6.5、28反射/112先验字段全符；Patterson288体素全符四余弦独立先验，解析max7.2345e−8/直接周期卷积max5.3846e−8、积分64.00000873。带符号输入的负Patterson值被先验解释，原件未截零。56723正常0/152.875秒、softstop前，QuitNo保输入，无重启/遗留。5新至376证据、11头不变；59样本不增、前58记录全保持。43历史包5090文件/原包257保持，254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。

## R45三轴线剖面（不改勾选）

VS59网格线段(0,.125,.25)->(1,.625,.75)、N13，26先验字段全符周期三线性/晶胞度量sqrt185A；max密度差5e−9、距离5.367e−6A，连续解析再合成模型差.001375。两端包含/x1周期corner成立；static8角权重/float采样/物理距离/%14.6E绑定，域外负坐标/非法N/写失败未验。首次只读取证缺路径段exit2保留，correctproject新回读0，GUI未重算。56998正常0/73.401秒、softstop前，Quit无保存提示、保源无重启/遗留。6新至382证据、11头不变；59样本不增/前58记录保持，44历史包5205文件保持。254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。

## R46线剖面负周期/反向/N边界（不改勾选）

VS59负x跨−1..0及反向各13点，52先验字段全符，max密度差5e−9/距离5.367e−6A。N1/N0均实际导出两个有限端点/两文件byteidentical；GUIhandler在接受filedialog后、调用底层前把N<=1改2，不能由底层N−1推GUI除零。边界探索与52数值先验分开；每次重开控制重置默认值。57369正常0/202.487秒、softstop前，Quit无提示/保源，无重启/遗留。10新至392证据、11头不变；59样本/前58记录保持，45历史包5325文件保持。254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。

## R47双向量二维切片（不改勾选）

VS59/grid6×6×8，001/V1[100]/V2[010]/center(.5,.5,.3125)/offset0，实际7×7闭合grid确认预列候选。49密度/98localXYÅ字段全符独立半z格平均/10×12平面，max密度5e−10/几何1e−6Å。UI范围拆两节点，早期单行匹配false原JSON保持、另逐节点修正true。平面实际z4.375Å而Distancefromorigin标签/输出0指center-relativeoffset，header不含centerprojection，不当可完整恢复。2Dexport静态+1计数/闭合循环/float转double/三列格式绑定；其它模式/轮廓颜色视觉仍待。57701正常0/150.244秒/softstop前，2Dclose再Quit无提示/保源，无重启/遗留。8新至400证据、11头不变；59样本/前58记录保持，46历史包5457文件保持。254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。

## R48周期投影近似合同（不改勾选）

VS59完整/半周期projection各7×7。GUI前五canonical累计/平均/积分候选两案全不符，原先验保持；full首值后、halfCalculate前另登记2xgridfrequency闭合均值×厚度假设，half49留出点max差4.44445e−8/full诊断5.64706e−8。静态CalcSlice→GenPeriodic及加倍frequency、floorRloop、length/(Rraw+1)权重、float累加相符；非整R待。与原三线性场精确厚度积分max差full.00365888/half.00320333375，scientificExactIntegralAccepted=false保持，不冒称数学全通过。export缺center/上界。58155正常0/217.670秒/softstop前、保源无重启/遗留。13新至413证据/11头不变；59样本/前58记录保持，47历史包5584文件保持。254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。

## R49恒定场非整数投影（不改勾选）

新VS60独立constantρ1、0atom/6×6×8/10×12×14Å。GUI前物理积分L与兼容L*(roundR+1)/(Rraw+1)分开登记：R4.8round5/L4.2实际49个4.344828（精确4.2，偏+3.448285714%），R5/L4.375实际49个4.375。98兼容标量/196坐标全部符先验，恒定场隔离循环取整/分母不取整偏差，scientificnoninteger/generalIntegralAccepted=false、aligned=true，不实现修复。58585正常0/158.282秒/softstop前、保源无重启/遗留。5新至418证据/11头不变；60样本/旧59记录全保持，48历史包5724文件保持。254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。

## R50包围盒二维导出状态（不改勾选）

复用VS59：001/.5d/7Å/xyz0..1，独立范围与两次bbox显示符5e−7先验；六位小数字面格式候选不符、格式规则未定。CUA观察fresh bbox→vector→bbox的Export2DData分别disabled→enabled(49行成功文件)→disabled，两个bbox文件均无、未强制禁用动作。原生AX不暴露enabled，证据类型分开。六共用字段跨模式按新语义重解释，OK前恢复预定值。58948正常0/242.779秒/softstop前，源/60样本保持，无遗留。7新至425证据/11头不变，49历史包5839文件保持；失败setup及错误CLI选项保留且不计Evidence。254功能、60样本、10完成19未完成，Study继续/Codex自验/dsh复审未开始。详见ALGORITHMS与体数据规格R50。

## R51二维控制、等高线及重开（不改勾选）

VS59平面：linear .02/−.01/.005/step2与log A.003/B4/Nmin−2/Nmax2/step2参数native读回，六个正确window-id原生PNG确认曲线/填色/网格/鸟瞰渲染响应。当前正负各major/other线宽有别于旧手册简述；static calc_contour_num揭示linear排除上端点、本例6，log指数界/step8幅度跨零16正负候选，未读取运行levelbuffer、不算数值全验。关闭重开同进程log与grid1/width1.5/fill0/bird1保留，Zscale字段100→1；底层几何/精确恢复未定，鸟瞰+45°有裁切。59383正常0/267.864秒，退出后AX误重启空白59599已正常关闭/pgrep无遗留，方法偏差明确保留，不称无重启。20新至445证据/11头不变，50历史包5969文件保持、60样本不变；254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。

## R52恒定密度转换单位及lned失败（不改勾选）

VS60constant1，两独立GUI分别Angstrom/Atomicunit并选择5输出。10文件各7×7×9/441；4410字段符独立GUI前TF/单位/零导数/lnraw1预期：gA.1191446、gau2.871234，v−2g/he−g，Lap~0/ln0。四类能量/led自动开新页；lned写完整却两次unsupported alert。GUI后预注册samebytes suffixcontrol：原lned CLI−11/SIGSEGV/0byteggrid，改名led后255/1916bytes有效441个zero网格，具体classifier/崩溃传播未绑。新增unk_41a422ce94027042 rev2 medium/investigating，12头9resolved3investigating、旧11保持。两GUI正常0/82.906与70.055秒/softstop前，Quit后只ps无AX重启/遗留。12新至457证据，前51包6154文件保持，60样本不变；254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。详见ALGORITHMS R52。

## R53lned分类及CLI NULL保存原因（不改勾选）

读取R52拥有PID60060的系统crash报告，main/libUUID核定锁定原件；实际原库check_format只读诊断：lned/LNED−2、led/ked/ped/ted/ned/xed45。静态三字节xed判定缺四字节lned，ReadFile负type回−2、cmdRead回−1且不追加对象，后续CLI Save不检objects仍export。报告PC export_data+284/lib0xb1b9c，x8=0/x21=0/FAR0绑定NULLScene数组；fopen(w)/close先于dispatch解释0byte产物。未重跑崩溃，未修复；首次PythonCDLL依赖路径失败保留、独立C诊断用byteidenticalPNG副本且原件未改。unk_41a422ce94027042 rev3/resolved/nativeverifyvalid，当前12头10resolved2investigating、其它11保持；7新至464证据，前52包6336文件/257原件保持、60样本不变。254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。详见ALGORITHMS R53。

## R54非恒定正密度导数（不改勾选）

新VS61为ρ1+.2cosX、6×6×8/10×12×14Å/0atom Cube。GUI前独立连续导数与普通中心差分两案皆不符全模型；首次计算前静态26邻点inverse-distance/6÷26候选L全符，Kx.32400554356 vs普通.36，Atomic/Å导数比a0²。原static候选中心梯度能量Atomic有超容差字段，保留失败；读输出后静态G²=Σw(oppositeρdiff_f32²)/8解释全部4410字段，无独立holdout、不算4410先验通过。lnrawρ两案通过，非正交/零负阈值仍待。原GUI60733/60813正常0/153.211与79.435秒/softstop前，Quit后只ps无遗留；OS崩溃恢复提示选不重开、lned已知unsupported保留、不重跑CLI崩溃。8新至472证据/12头不变；前53包6513文件保持，新VS61旧60记录不变。254功能、61样本、10完成19未完成，Study继续/Codex自验/dsh复审未开始。详见ALGORITHMS R54。

## R55三维非正交与阈值新输入检验（不改勾选）

首GUI前登记R54学得规则对新VS62非正交β100/三维耦合正场两单位和VS63signed/zero/近1e−6场。正场4410字段原tol全符；阈值315closedsites三能量精确0/126active、logzero63个−Infinity如先验。阈值189固定Abs失败保留，阈值计算前另登记的%14.6E打印精度准则全部符，6615适用准则通过不冒称原Abs全通过；连续物理精确导数仍不符。三GUI正常0/72.712/78.012/134.372秒，源保留/Quit后只ps无遗留；lned已知unsupported保留，不重跑CLI崩溃。10新至482证据/12头不变；前54包6661文件保持，新VS62/63旧61记录不变。254功能、63样本、10完成19未完成，Study继续/Codex自验/dsh复审未开始，详见ALGORITHMS R55。

## R56 标准化与 Niggli 选定合同（不改勾选）

新 VS64/65 独立 P1 正交/斜晶胞，先验度量/周期距离/带符号坐标候选符合选定报告。标准化更新当前晶胞与坐标、保留占位/U；Niggli 报告约化解但两案 SaveAs 保留原晶胞/原始超胞外坐标，初始“应用变换”假设拒绝。斜案状态预期在其GUI启动后、操作结果前登记；首分析周期搜索和时序范围错误保留，新scoped报告仅重算。真实外部协议是 filename .stin 参数和stdout重定向，不是stdin输入；原附带x86_64 TIDY实际可用，子退出码未单独捕获。三GUI正常退出，无目标遗留。13新至495证据/12头不变，旧63样本记录保留、新64/65；前55包6830文件及原257文件保持。254功能/65样本、10完成19未完成；完整Study继续，Codex自验/dsh复审未开始。详见ALGORITHMS R56。

## R57 正负峰及 Voronoi 菜单共用（不改勾选）

新VS66常量2整胞积分128；VS67 signed三轴场正/负积分±68、唯一正/负峰±6的位置与首GUI前独立预期相符。Peak/Voronoi菜单178/179确实绑定同一OnPkSearch（构造/全局事件表/Mach-O常量），初始按原子中心划分预期不适用。Max0负峰开得到两峰±60/体积各32；max1仍报检测2但分配只用首峰，整胞体积64/积分0，不仅打印限制。事后欧氏分配对照相符，严格等距20与原件boundary63不同，确切计数语义仍待核。两GUI正常退出，无遗留。30新至525证据/12头不变，旧65样本保持、新66/67；前56包7018文件及原257保持。254功能/67样本、10完成19未完成，Study继续/Codex自验/dsh复审未开始。详见ALGORITHMS R57。

## R58 Voronoi 计数/细分规则的新输入检验（不改勾选）

R57边界数现解释为中心与26个周期邻格点的单标签差异flag，不是几何等距数；原库常量表/循环/计数指令绑定。边界2³子位置各±1/4格距，保持源格点标量，近等距1e−6均分。新VS68为6×8×10/5×7×11Å耦合signed场，首GUI前预测flag342/480、严格等距0、积分−351.9832/+467.4832、体积各192.5，原件在预定tol内全部命中，无本轮拟合。旧63边界/20等距仍保留，现有选定路径原因解释；一般科学精度/收敛仍待核。GUI正常0/58.122秒，无遗留。4新至529证据/12头不变；旧67样本保持、新68；前57包7232文件及原257保持。254功能/68样本、10完成19未完成，Study继续/Codex自验/dsh复审未开始。详见ALGORITHMS R58。

## R59 逐多面体积分空处理（不改勾选）

锁定原件arm64菜单“Integrate Voxel Data in Each Polyhedron”ID185落入182..186事件范围，绑定OnMulti0x10009fedc，仅4bytes/ret。GUI前登记无计算预期；复用VS66含原子/体数据而0polyhedra，菜单enabled、点击后无对话框/Output追加，主窗口保持。无条件空处理结论来自静态分发，不以该场景冒称数值算法执行。AX树353→360的指针/窗口菜单缓存差异保留，不称全树相同。GUI正常0/48.459秒，无遗留。4新至533证据/12头不变；全部68样本记录保持；前58包7350及原257保持。254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。详见ALGORITHMS R59。

## R60 公共数据库查询到原生保存（不改勾选）

原件HTTPS COD公开ID1000000只查一次、返回1条，CIF链接实际进入原生新标签并保留基线标签；首导入后误关主窗口、未保存的失败及窗口树保留。新受控实例只补同URL原生导入/SaveAs，明确改Where到R60，35标签/元素对及146数值与另取公共CIF相符。源是首次导入后另GET12580bytes，未证明临时native流SHA、不称独立物理Oracle。主Quit可能留数据库窗；ps核拥有PID仍活再AX关闭剩窗，末次不AX，无遗留。两GUI正常0/220.658/162.645秒。DB头revision3仍investigating，提供者/错误/取消/dirty矩阵未完，其它11头不变。13新至546证据/12头10resolved2investigating，新VS69/旧68记录保持，前59包7468及原257保持。254功能/69样本、10完成19未完成，Study继续/Codex自验/dsh复审未开始。详见DATABASE R60。

## R61 数据库导航/404/dirty（不改勾选）

复用VS64，GUIa7→8/V120未保存；HTTPSsearch/tips Back/Forward/Reload路径及URL核对，失败999999999.cif实际404提示/无导入新标签；关数据库后a8Output全文保持，Quit保存提示No，源文件不变。Stop点击时上帧enabled但后态页面完成，实际传输取消仍未证明；Menu自动ID−1、选定23方法无独立提供者handler/点击无popup，仅限该路径，非全局缺席证明。GUI正常0/328.770秒、末次No后无AX/无遗留。DB头rev4仍investigating，其它11头保持。19新至565证据，全部69样本保持、前60包7636及原257保持。254功能、10完成19未完成，Study继续/Codex自验/dsh复审未开始。详见DATABASE R61。

## R62 晶胞变换/超胞（不改勾选）

独立新VS70/P1/4×5×6Å/C-O不同occ/U，首GUI前固定三案。P2a两方案均V240/4展开位置；AddSymOP存2位点/2操作/TRANM1，NewSites存4位点/1操作/TRANM0且标签C1/C1/O1/O1保持。剪切a,a+b,c与原点.1/.2/.3得b√41、γ51.340191、V120、C(.025,0,0)/O(.325,.2,.2)，111数值先验全符，occ/U保持。F00020/20/10仅由保存组成推导、未调用原件计算。内层OK后仍有外层modelessEditData未提交，重新聚焦原有编辑器/外层OK才更新场景；证据保留。三GUI正常0/259.468/182.020/105.459秒，无遗留。18新至583证据/12头不变，前61包7809及原257保持，新VS70旧69保持。254功能/70样本、10完成19未完成，完整变换/Study继续，Codex自验/dsh复审未开始。详见ALGORITHMS R62。

## R63 变换模式与拒绝分支（不改勾选）

沿用VS70/70记录全不变，首GUI前固定P[a,a+b,c]/p(.4,.3,.6)两案：NormalizeOff保存负分数C(-.175,-.1,-.3)/O(.125,.1,-.1)、cell4/√41/6、γ51.340191、V120；KeepParametersUnchanged保留原cell/坐标，矩阵P/p仍保存。34先验数值字段全符、occ/U不变。P1恒等对称不能代表一般SG。det0实际拒绝并回编辑器，det−1实际左右手警告/No返回；未接受负det。防护定位7个0字段抛错发生在setValue前，原始记录保留。两GUI正常0/146.559/113.463秒，无遗留。10新至593Evidence/12头不变，前62包7997及原257保持。254功能/70样本、10完成19未完成；完整变换/Study继续，Codex自验/dsh复审未开始。详见ALGORITHMS R63。

## R64 各向异性换基（不改勾选）

复用VS57/70记录全不变，独立β′=QβQᵀ经倒易长度转换U′，右手[b,a,-c]与非正交[a,a+b,c]+原点两案：58字段按保存精度政策符合、Ueq不变量保存.020773/.050559。原1e−6U分量tol有3剪切项失败，原行保留；第一案发现固定5小数后、第二GUI前登记5e−6半末位补充政策，不冒称原定58全过。内层后外层确认/SaveAs路径守卫，两GUI正常0/97.809/81.206秒无遗留。10原生结果1复用/9新至602Evidence，旧593/12头保持，前63包8143及原257保持。254功能/70样本、10完成19未完成，完整变换/Study继续；Codex自验/dsh复审未开始。详见ALGORITHMS R64。

## R65 CLI几何/去重契约（不改勾选）

总计划仍有166入口待行为；推进S07，首CLI前固定17案：三轴旋转与非交换顺序左乘、scale2及2×.5、去重0/.0004Å阈值3/3/2，339先验字段符合。合并后C坐标.125033/occ.5只是结果后观测，未预设平均公式。空列表五案与基线同terminal/保存字节，-scale_width_to8在nogui亦无变化，不当正列表/GUI宽度通过。17target正常255/无超时信号、无GUI/遗留。新增CLI.md权威文档；旧70保持/新VS71，27新至629Evidence/12头保持，前64包8289及原257保持。254功能/71样本、10完成19未完成；完整CLI/Study继续，Codex自验/dsh复审未开始。详见CLI.md/ALGORITHMS R65。

## R66 CLI正几何/别名（不改勾选）

复用VS54首GUI前独立SiO6预期；GUI显式Si/O1.9..2.1Å存7/6/1工程、正常0/102.784秒。后续8CLI正常255，无界面重建7/6/1：bond12双向2Å、angle15组(12×90/3×180)、polyV10.6667/D0/λ1/ECN6，组合参数bond,angle,poly均给完整报告，76先验科学字段符合。-o与-save及8工程逐字节相同，cell/site仍无额外报告。CalcF仅静态识别5参数/CalcI/ExportData，未执行。16新至645证据/12头保持；前65包8531/原257、旧71样本保持，254功能/71样本、32权威文件、10完成19未完成。完整CLI/Study继续，Codex自验/dsh复审未开始。详见CLI.md/ALGORITHMS R66。

## R67 CLI显式calcf（不改勾选）

复用VS55/旧71记录保持，5案首CLI前固定外部DABAX/NIST历史系数/参数/完整argv。source0中性X-rayλ1/2、source1neutronλ2各6HKL，108字段在打印半末位内全部符合；λ不改F而2θ5.73197/11.47834，中文空格out成功。anomalous1改变F/Friedel幅度，仅观察未数值验收。resolution10.1空HKL无反射文件/Failedtowrite日志，但后续-save仍有效/target仍255，不能当计算成功。默认.7/.709317/0/1/stem.txt静态推导未运行。5target正常255/noGUI/无遗留，6新至651Evidence/12头保持，前66包8710/原257保持。254功能/71样本、32权威、10完成19未完成；完整CLI/Study继续，Codex自验/dsh复审未开始。详见CLI.md/ALGORITHMS R67。

## R68 默认calcf与输出错误（不改勾选）

原stem+.txt假设失败：默认CIF实际写cwd/cif.txt；原两默认共用s04导致后full覆盖首limited，首表无原SHA、默认路径未watch，原计划/日志保留，full副本/原哈希和偏差明确。观察后先登记、再独立cwd两后续并watch全目录：默认组合source0/λ.709317/anomalous1/res.7(或9)输出与显式逐字节同，6/12196HKL；四表24404行/73212d-twoθ-M字段独立符合，不作反常F准确性。有效输入缺失out父目录报失败/无表但后续Save有效/target255。7target均正常255/noGUI/无遗留；8新至659证据/12头、旧71样本保持，前67包8846/原257保持。254功能/71样本、32权威、10完成19未完成；完整CLI/Study继续，Codex自验/dsh复审未开始。详见CLI.md R68。

## R69 保存选项/F居中（不改勾选）

新VS72/旧71保持；默认Direct/Cartesian常规Na4Cl4/V64，pcell/reduced各1/V15.9999991792、最近NaCl1.9999999658Å符合先验；CLI约化TIDY报告与实际写出，不混同R56GUI报告后未应用。原闭盒27/14Na13Cl假设失败：默认成键0..3.39412补边界外30Cl，实际57/14Na43Cl，全部43Cl匹配结果后邻居并集。观察后先登记8Å变体再target：27/0bond/as_displayed14+13/V512符合后续先验。默认.ins实为SHELX，format=rietan另有模板RIETAN字段；SOF10.02083/外部精修精度未验。9target正常255/noGUI/无遗留；17新至676Evidence/12头保持，前68包8999/原257保持。254功能/72样本、32权威、10完成19未完成；完整CLI/Study继续，Codex自验/dsh复审未开始。

R69验收补记：原九个target的监控均完整；初次分析辅助进程watch_depth=2在空preferences/style与tmp目录处置file_list_cut_short=true，专项验收如实失败。原报告/执行未覆盖；独立递归20个实际文件、全部与原snapshot哈希一致，再以显式文件/深度6/上限1000做native完整监控审计，child0/cutfalse/23节点。科学比较和原件均未重跑。规范native-evidence-final.json为676证据/17新；原675导出仅中间态。使用新的scoped主/专项验收与实际运行，保留初失败。

## R70 GUI命令交接与图像（不改勾选）

长VESTA_PREF服务名148字节失败/忙提示，SDK sun_path104与OnInit_prefdir构造支持路径超限；观察后先登记短共享pref30字节，对照socket/listener及11发送进程正常0。-open中文文件7/210→外部副本修改后-reopen8/240→-close指定文件保留first4/5/6，并-save确认；Rx30矩阵、实读Width8Å、PNG1112×940RGB与2倍透明2224×1880RGBA/alpha0..255符合。初始SVG矢量假设失败，PDF/SVG补测也失败，handler走ExportRaster；相对PNG实际以结构文件目录解析，原发送cwd预测失败保留。发送退出时PNG可仍0字节，后续父快照+解码才证实完成。两个超时发送、一个诊断−15和长host软时限超4.394秒如实保留；三host正常0无遗留。38新/714Evidence，12头10resolved2investigating保持；254功能/72样本、32权威、10完成19未完成。完整CLI/IPC/Study继续，Codex自验，dsh复审未开始。

## R71 内部CLI分派（不改勾选）

25案复用VS54/R66正成键工程，每案新cwd/独立pref/20秒。首19别名预期15通过4失败：15clear/add/del/import实际分派cmdSave，产生未改工程；set_bond/isosurf/slice/vector被首字符s分派跳过，仅后续-save生效，原4失败保留。add_bond为8bytes mov0/ret，实测不改7/6/1；clear_bond无参亦未清键。remove实为C文件操作，仅删除本轮新建载荷；缺失/无参案继续-save且最终255。40工程文件逐字节等同源5144bytes；无GUI/无遗留、25正常255。28新/742Evidence、12头保持、72样本不变；254功能/32权威、10完成19未完成。Powder/v2site/外部联动及完整Study继续，Codex自验，dsh复审未开始。

## R72 v2site与powder边界（不改勾选）

VS53工作GUI创建仅单C箭头；(1,2,3)输入在Modulus3.74提交后成为(.99956,1.99911,2.99867)，初始精确保留假设失败。-v2site只把VECTR分配1/1→1/0改为C1位点，组件/其它工程字节保持，一次两次SHA一致；GUI重开C1位点1、个体列空，实际两平行箭头，原“个体列应显示继承1”预测失败。VS55普通CIF的-powder在空DataSet首指针处SIGSEGV11/PID2124/lib0x1f8fc/x9=0，未进入计算；系统ips/UUID/指令绑定，原生code1/exited不足以称正常退出。4正常255/1崩溃/第6案未跑，停止/三原失败保留、不重复崩溃；2GUI正常0无遗留。22新/764Evidence、12头保持/72样本不变；254功能/32权威、10完成19未完成。粉末科学/axialpolar/完整Study继续，Codex自验、dsh复审未开始。

## R73 内置粉末正向（不改勾选）

VS70/4×5×6Å，首GUI前固定λ1/range9..10/step.01独立±001、d6、2θ9.5603836944。新偏好实际进入内置DlgPowder，创建DataSet/ExpCond/ObsData后Calculate成功，反射2行d6/2θ9.56038符合；不同于R72裸CLI空数据集崩溃，也不同于旧VM14外部RIETAN。页面Export按钮两种点击无效，File菜单反射导出成功；FileExportData写GENERAL$/100三列.int，面板误标ExportGraph。曲线max9.55偏Bragg−.0103837，原最近网格tol.0051失败，单bin左移只是结果后模型，需新step留出。显式calcf与GUI反射20token一致，非独立反常精度。GUI正常0/384.496秒，软360超24.496秒如实保留/硬570未触发/无遗留。12新/776Evidence、12头保持/72样本不变；254功能/32权威、10完成19未完成。完整粉末/Study继续，Codex自验，dsh复审未开始。

## R74 粉末网格留出（不改勾选）

运行前固定step.02/range9..10与step.01/range9.005..10.005：第一案一格左移预测9.54，实际max9.56，模型被反例否定；第二案预测9.555实际9.555，不能挽救第一案。floor−1预测9.54/9.545两案均失败。50/100行半开网格与原反射表均保持/逐字节一致，独立d6/2θ9.5603836944成立；原因未解，不能把采样最大值偏差直接定为bug。TCH/FindIndex原生路径已读但无实际分支绑定/完整独立峰形oracle。两GUI正常0/176.577、202.842秒<soft240，无遗留；B总时长超180交互目标22.842秒，不声称全交互按时。8新/784Evidence、12头保持/72样本不变，254功能/32权威、10完成19未完成。完整粉末/Study继续，Codex自验，dsh复审未开始。

## R75 粉末峰形因果与λ2留出（不改勾选）

工厂链绑定默认TCH_Ida，rd=.1、zmin=−rd²cot(2θ)/2；论文Howard卷积独立Simpson积分解释旧三谱偏峰，固定一格索引模型继续被否定。H在本版实际用作gamma，通常FWHM换算备选误差约27.5–29.1/100，不能混称标准FWHM等价。新λ2/19..20/step.01在target前登记max19.18/内部tol.01；实际max19.18、内部最大差.0016398951通过，反射±001/d6/19.18814符合独立Bragg。首点原版0/独立1.9423983科学失败，左循环排除index0与之相符；整谱/完整粉末不通过。GUI正常0/123.251秒无遗留；一次误相对project工程保留并绝对路径重取/ID去重，thin库误读与AX差分校验提前失败均保留。24新/808Evidence、12头保持、72样本不变、254功能/32权威、10完成19未完成。详见ALGORITHMS R75，Codex自验，dsh复审未开始。

## R76 双波长条件积分（不改勾选）

新GUI前登记λ2/2.1Å、权重1/.5、range19..21/step.01，独立±001/d6各2条、Bragg19.18813645/20.15731622、分量采样峰19.18/20.15。实际4反射/200曲线行及双峰19.18/100、20.15/44.54929符合；以导出|F|²为条件振幅的独立积分最大内部差.001645282164<预定.01，忽略权重备选误差44.45316180被否定。这是条件形状/权重检查，非绝对F科学独立证明。首点原版0/条件积分1.96803397724科学失败保持，整谱/完整粉末待。GUI正常0/94.347秒，无遗留。3新/811Evidence、12头/72样本保持，254功能/32权威、10完成19未完成；详见ALGORITHMS R76，Codex自验，dsh复审未开始。

## R77 峰位修正原预测失败（不改勾选）

λ7.713451316238471/80°附近、CTHM.8/PCOR.999、Z=.05/Ds=.04/Ts=−.03预登记反射80.05109755/峰80.05，实际80.06830/80.07，内部差11.83108048>预定.01，三原预测失败。重读指令发现Codex把分母360误读180；本版实际Tcorr=T+Z+Ds cos(T/4)+Ts sin(T/2)。该半角映射对同一案的事后解释差.00569045、完整精度λ更符合打印角度；尚未新输入留出验证，不能回填原预测通过。首点科学失败保持。GUI正常0/141.195秒，无遗留。3新/814Evidence，12头/72样本保持、254功能/32权威/10完成19未完成；详见ALGORITHMS R77，Codex自验，dsh复审未开始。

## R78 半角位移新输入验证（不改勾选）

GUI前固定λ7.7、Z−.02/Ds−.06/Ts.08、CTHM.6/PCOR.997。指令半角映射预报反射79.80735918/峰79.81，实际79.80736/79.81；内部独立积分差.005668704972<预定.01。旧错角/默认LP/未修正宽度三备选误差25.51844/.743217/.0213175均被否定。R77原失败原样保留，仅本次新输入映射通过；一般物理公式正确性/绝对反常F/完整粉末不冒称。首点原版0/积分3.07070227科学失败保持。GUI正常0/124.951秒无遗留。3新/817Evidence、12头/72样本保持、254功能/32权威/10完成19未完成；详见ALGORITHMS R78，Codex自验，dsh复审未开始。

## R79 ORFFE结果读取与链接（不改勾选）

实际入口为.ffe查看器，所读直接链未启动外部ORFFE。人工7位点fixture故填9.8765±.0123/123.456±.0045（模型2Å/90°）。首显式identity/码1案读后空表，原成功预测失败；零额外identity/码0联合诊断变体成功，表值9.8765/123.4560目视确认，选行原生详情9.877(13)/123.456(5)和对应原子。主图高亮未目视。SaveAs仅加IMPORT_ORFFE相对链接，尚未重开。GUI正常0/136.708、144.014秒无遗留。新增VS73人工.ffe，旧72保持；13新/830Evidence、12头保持、254功能/73样本/32权威/10完成19未完成。完整外部协方差计算/ORFFE/Study不通过，详见ALGORITHMS R79，Codex自验，dsh复审未开始。

## R80 ORFFE重开/高亮与坏读（不改勾选）

新实例相对IMPORT_ORFFE侧文件自动恢复，无手动Read；原生详情9.877(13)/Si1/O1一致。指定拥有PID91883/主窗口70518两张860×728原图目视确认选择行在O1/Si1产生黄圈，模型区黄色像素0→298，非全渲染验收。只删O6条目7→6后表空、无错误提示，顶部旧详情仍留；失败后原生UI读被180截止阻止，没有回执。坏读SaveAs清IMPORT_ORFFE，逐字节回到原R66无链接5144bytes；原输入/有效侧文件保持。GUI正常0/227.666<soft240，但超180目标47.666且清理晚，allTimingTargetsMet=false。6新/836Evidence、12头/73样本保持，254功能/32权威/10完成19未完成；详见ALGORITHMS R80，Codex自验，dsh复审未开始，完整ORFFE/Study待。


## R81 磁矩反演与时间反演（2026-11-09）

新增VS0074/75：10Å立方，Fe1(.1,.2,.3)，磁矩(1,2,3)μB；两个完整操作只在反演的时间符号±1上不同。首CLI前登记 [独立期望](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-before-targets.json)。一手Shinohara2023 §2.4 Eq7给m′=εdet(W)AW A⁻¹m；本案A=10I、W=−I，故普通反演保持磁矩、带时间反演则变号；magCIF字典只选读坐标/单位和xyz时间符号，全文阅读未完成。

原版两个-nogui导入/Save正常255，完整watch；实际MAGNETIC_CRYSTAL、SYMOP时间列±1、单site分配均保留。VECTR保存Cartesian(1,2,3)，末列type0/Axial（R83修正原坐标模式解释），原先预测晶格(.1,.2,.3)失败并留原计划；没有把表示差异当科学失败。原件自行识别2.4/P−1和2.6/P−1′，这些编号是目标观测，非本轮独立群号验算。[17项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-crosscheck.json)。

两幅归属窗口原生PNG（872×558）已逐一目视：反演行−x/−y/−z、T+1显示mx/my/mz；T−1显示−mx/−my/−mz，符合独立规则。[普通反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/inversion-equivalent-ui.png) / [时间反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/anti-repeat-equivalent-ui.png)。wx表行不暴露于AX，此为原图视觉读数；没有提取实际展开原子上的数值磁矩，也未校准3D箭头方向/长度。polar、非正交、特殊位置、磁超空间与New-dialog编辑仍待，不把VF0091/VF0177或完整磁性科学标作完成。

首截图native调用尚未完成时Codex提前Close/Quit，实际screenshot_window_not_found/单AXApplication节点；失败与 [原因记录](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/native-capture-failure.json) 保留。随后两个新归属进程分别33.382/30.084秒正常0退出，成功取证；首进程75.711秒也正常0，全部<180且无残留。CIF reader0xa671c已有静态证据ID去重，不重复增加记录；新增8条至844，12问题头/旧836证据/原件257/旧73样本不变，稳定样本增至75。80个历史manifest/11112文件完整。研究仍10完成19未完成、Codex自验，dsh独立复审未开始，无产品实现。


## R82 降对称后的第二位点矢量缺失（2026-11-09）

复用VS0075磁群2.6/P−1′，首GUI前登记 [坐标与两磁矩保留预期](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/axial-before-target.json)；VM06 §6.2.5/6.2.7与VM09 §9.1.1本轮只重读相关段落，不新增全文阅读声明。Remove symmetry→OK→新文件SaveAs实际得到磁群1.1/P1、两个独立Fe位点(.1,.2,.3)/(.9,.8,.7)，坐标展开符合，但VECTR与输入逐字相同，仅矢量1分配site1；第二个预期(−1,−2,−3)没有分配，**磁矩保留预期失败**。缺失矢量不是测得零磁矩。[11项证据核对及原科学失败](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/p1-crosscheck.json)。

结果后登记新进程重开对照：同一归属窗口两幅860×728原生PNG经目视，前2箭头、后1箭头，第二原子保持可见而箭头消失；选定ROI红像素840→0。[降对称前](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/before-main-ui.png) / [重开后](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/after-main-ui.png)。Vectors原图1034×333显示第1位点Vector1、第2位点空；wx行不在AX，由目视记录。[位点分配](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/after-vector-ui.png)。这是所测输入的持久化丢失行为，非完整3D渲染校准/一般P1转换验证。

原生OnRemoveSymmetry0x10003b0b4/184bytes在0x10003b0f0调用lib convertToP1 0x69c28/1748bytes；读取直接克隆XSite/TransformCoord路径，未隔离完整矢量丢失根因，不凭未见直接调用断言所有间接行为。两个归属GUI9852/10318正常0，51.365/102.995秒，均<180且无遗留。首分析脚本把真实9token STRUC行错设>9而返回空站点，[失败报告](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/p1-crosscheck-initial.json)和初版脚本保留，改>=9后的新报告通过证据检查，原磁矩科学失败仍保留。一次CUA状态保护阻止旧动作，刷新AX后继续，不推断人为操作。

新增8至852Evidence，旧844/12问题头/75稳定样本/原件257与81历史manifest的11277文件不变。VF0095因新发现纳入规格，原VF0091/0177范围沿用；polar、非正交与实际展开数值磁矩仍待，未因坐标通过而关闭完整磁性/Study。254功能/32权威、10完成19未完成，Codex自验，dsh独立复审/S08未开始，无产品实现。


## R83 Axial/Polar 空间反演与文件类型标志（2026-11-09）

新增VS0076：立方10Å、Fe1(.1,.2,.3)、磁群反演T+1，磁矩改为单位(1,0,0)。首CLI/GUI前分别登记 [独立空间奇偶性](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/polar-before-targets.json) 与 [固定显示Scale3](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/polar-gui-before-target.json)；用模长1避免旧3.74提交舍入，Scale3使箭头露出Fe半径，两个状态比例相同。VM09 §9.1.1给Axial=det(M)Mm、Polar=Mp，反演期望分别(+a,+a)/(+a,−a)。

原版CLI正常255；GUI原生AX先Axial后Polar，Cartesian字段始终1/0/0、Modulus1.00、Scale3。两个同PID12648/window71712的860×728原始PNG已目视，[Axial双+a](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/axial-unit-main-ui.png) / [Polar第二原子−a](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/polar-unit-main-ui.png)，第一箭头保持+a；可见a轴朝屏幕右。结果后选定箭头端点ROI最大红列宽，第二左右(7,17)→(16,7)，第一(6,17)保持，支持视觉方向，不当作事前独立像素oracle或任意3D数值校准。[12项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/polar-crosscheck.json)。

**修正R81解释**：新SaveAs文件 [仅两处diff](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/saved-project.diff) 为VECTR末列0→1、VECTS1→3，其它字节相同；末列是Axial0/Polar1类型标志，不能再称coordinate-mode0。原生save_data0xacf04/10176bytes在0xaea90读对象flags+0x8c，0xaea94提取bit2写入该列。当前权威文档/功能字段已纠正；R81冻存原始文件和报告中的storedCoordinateMode字段按历史错误解释保留，由 [解释更正记录](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/interpretation-correction.json) 明确废止其语义。构造函数参数3不再单独用作Axial类型oracle，原先晶格组件保存预期失败仍保留。

GUI正常0/148.640秒<180，无残留，4次nativeUI读取均在截止前且逐一等待完成。两次CUA状态保护阻止旧动作，刷新AX后重试，不推断人为改动。新增7至859Evidence、12头/旧852/旧75样本/原件257与82历史manifest/11426文件不变，样本增至76。Polar时间反演无效的实际对照、非正交和完整矢量/磁性仍待；R82降对称丢失失败不变。254功能/32权威、10完成19未完成，Codex自验，dsh独立复审/S08未开始，无产品实现。


## R84 Polar 时间反演、视角与CLI比例（2026-11-09）

新增VS0077：由VS76单位磁矩CIF只把反演时间+1改−1，首target前登记 [独立Polar时间不变预期](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-before-targets.json)。原件CLI导入后，按R83证实的类型位/Scale格式制作新受控Polar1/Scale3副本；其与R83正时间Polar工程仅GROUP2.4→2.6及SYMOP反演时间列不同。GUI新进程回读实际Polar、Cartesian1/0/0、Modulus1.00、Scale3。

**两项原预期失败保留**：①CLI无界面回读/Save [只把VECTS3写回1](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/roundtrip-actual.diff)，其余字节与受控源相同、Polar类型保留；字节一致先验失败，不能推广为全部场景字段/一般读取丢失。首GUI准备断言在启动前停止，实际 [准备失败记录](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/roundtrip-preparation-failure.json) 保留，后续GUI明确选择未改动Scale3受控源。②首GUI原图a轴沿视线、b右c上，箭头正对视线，原注册ROI与参考差18590RGBbytes；不能由此读出方向或宣称时间反演失败。[初始原图](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-minus-main-ui.png)。

结果后另登记 [新进程显式Viewalongc](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/projection-before-target.json)，原ROI[305,80,859,550]没有移动。[沿c观察原图](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-minus-c-view-ui.png) 已目视：a右b上，两个箭头(+a,−a)，与T+1参考相同；该554×470模型ROI逐RGB字节相等，0差。独立物理预期来自VM09 §9.1.1，图像等值是按该不变量检验的原件间一致性，不是外部绝对渲染oracle。限定Polar时间不敏感有证，首失败不回写成通过。[12项核对与失败状态](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-crosscheck.json)。

GUI14360/14733正常0，75.529/37.637秒均<180无残留；一处CUA差分索引解析未找到Edit而中止，刷新完整AX后正常继续，未发额外编辑动作。所有native读取等待结束后才Cancel/Quit。新增5至864Evidence，旧859/12头/旧76样本/原件257与83历史manifest/11566文件保持，稳定样本增77。下一步可追CLI为何未保留VECTS及非正交/一般矢量；完整Study10完成19未完成，254功能/32权威，Codex自验，dsh独立复审/S08未开始，无产品实现。


## R85 CLI全局样式与完整证据快照（2026-11-09）

新target前登记VECTS7、.125及7→.125双文件append三案；原版均正常255/完整watch，输出全局VECTS1、相数1/1/2，每相矢量(1,0,0)/Polar1/site1保留。静态cmdRead临时Scene读取后仅转移Crystal列表及文件名，未转移Scene全局XStyle，解释所测比例重置；未读取临时内存或验证所有样式字段/格式。[15项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/style-crosscheck.json)。

锁定工具首次触发64MiB加载预算：原生export实际869/not_loaded2，磁盘871条均存在。两旧记录经原生按ID接口完整读取，与旧包/磁盘一致；[原始部分导出](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/native-evidence-final.json)保留，不改成全量；[完整核验派生快照871](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/verified-complete-evidence.json)明确为原生导出+原生ID解析的核验合并，[来源与身份](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/complete-evidence-provenance.json)、[预算重放](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/method-load-budget-audit.json)可审。不得将wrapper0或导出记录数当完整性；后续须核not_loaded并按需补齐。

新增7至871原生来源记录，旧864/12头/77样本/原件257与84历史manifest/11713文件不变；raw导出869，完整核验871，两者须区分。未改工具二进制/锁定提交/工程历史记录，无产品实现。254功能/32权威、10完成19未完成，Codex自验，dsh独立复审/S08待。一般样式、非正交矢量及完整Study继续。


## R86 GUI样式保留、Cancel崩溃与数值隔离未成（2026-11-09）

首target前 [登记GUI5/传入7对照](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/gui-style-before-target.json)：当前一相X矢量(1,0,0)/全局5，传入新相Y矢量(0,1,0)/全局7，经短共享VESTA_PREF发送-i/-save。客户端0/exited，另核父进程仍活及新5347byte文件稳定；保存两相标题current_style_five/incoming_style_seven、全局VECTS5，前后原生AX比例5、后相下拉表含两名。所测“仅追加相数据、保留当前全局样式”得到GUI反例控制，非总重置1/非取传入7。未发送-refresh，未验第二相主模型渲染。

**原两项预期失败**：保存第二相rawY为.10000，预期1.00000，原始分量一致性失败；尚未证明重开后的物理磁矩损坏。另，父进程18317在Vectors的Cancel恢复预览时SIGSEGV11/−11，146.850秒<180且无残留，但正常退出/正常cleanup失败。先前对话称“Quit时”已被系统栈修正，不能把工具序列中的Quit当已证触发动作。[11项证据核对及原失败](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/gui-style-crosscheck.json)。

具体上下文：先在1相时打开/Cancel Vectors，再CLI追加为2相，重开Vectors查看相列表后Cancel。[系统报告原档（历史文件名含quit）](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/VESTA-18317-quit-crash.ips) 与 [地址/UUID/寄存器绑定](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/vector-cancel-crash-binding.json)：main UUID cd806142…，PC0x1000214e8执行ldp x1,x2,[x20]，x20/FAR=0x300；调用栈OnCancel→SetPreviewMode(false)→VectorBase赋值。还原循环以现有Scene相数为界，却索引backupScene+0x4a0的相数组，无旧长度检查，再加箭头向量偏移0x300；空源指针与报告吻合。旧备份实际长度未读内存，1→2相造成旧备份不匹配是有据推断，未宣称全部根因/所有Cancel场景。

崩溃后另登记两个无界面数值隔离对照；第一个Y-only在18秒idle后被SIGTERM终止（native code1/reason idle_timeout、wrapper0），无命令输出/无文件。研究脚本错误地先读预期文件而报FileNotFoundError；raw失败/脚本保留，第二个X/Y对照未调用，不能形成数值结论。[停止记录](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/headless-control-stop-record.json)。下轮先用新有界拥有实例检查是否有启动恢复提示；超时原因尚未确认，不原样盲重跑，也不重放已知两相Cancel崩溃。

新增7至878原生来源记录；raw原生导出876/not_loaded2，[完整核验派生快照878](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/verified-complete-evidence.json) 保留旧871及12头、77样本、原件257、85历史manifest/11873文件。两个旧记录仍经原生ID解析补齐，无记录重写。254功能/32权威、10完成19未完成；Codex自验，dsh独立复审/S08待，无产品实现。


## R87 恢复提示、命令行对照与持久化矢量回查（2026-11-09）

R87先登记新进程计划，实际原生AX捕获macOS“unexpectedly quit while reopening windows”提示及Reopen/Don’t Reopen按钮；对先前拥有的研究窗口选择Don’t Reopen后，新单相主窗口正常加载并退出0。随后重新登记两组短偏好目录nogui对照：单Y、X再Y均原版正常255/exited，保存Y=1，分别1/2相，VECTS仍按R85命令行契约写1。此前PID25030的idle原因为何仍未唯一确定：当时没有UI证据，新的恢复选择和偏好目录也同时改变，不能把旧失败回写成通过。

两个全新GUI分别回读原incoming输入和R86已保存双相输出。原输入为Polar/Cartesian(0,1,0)/Modulus1.00/Scale7；输出第二相实际为Polar/Cartesian(0,.1,0)/Modulus.10/Scale5。因此原先保存Y身份失败已落实为持久化后GUI显示数值差异，不再只是一种尚未解释的序列化表示；仍未定位是导入、缓存场景还是保存阶段改变，不推广为一般磁矩科学损坏。原输入与回读工作副本哈希保持。两个回查均只Cancel内层Edit a Vector，外层Vectors用OK退出，Quit时对研究副本选No，不重放已知“缓存单相→IPC追加双相→外层Cancel”崩溃。外层OK即使无数值编辑也在这两次流程触发保存提示，此观测不外推到所有对话框。

三个拥有的GUI PID29107/29811/30583正常0，64.226/76.968/108.440秒均<180，无截止终止/残留。新增6个原生来源ID至884；raw原生导出882/not_loaded2，另两条经原生按ID接口补齐的完整884为明确标注派生快照。旧878记录、12问题头、旧77样本、原件257文件及86历史manifest/12021文件保持；新增稳定VS0078为独立规定的单相PolarY单位输入，样本增78。R86的raw失败、Cancel SIG11和旧headless idle记录不改。254功能/32权威文件、计划10完成19未完成；Codex自验，dsh独立复审/S08未开始，无产品实现。

[15项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-142400-s07-recovery-vector-controls/recovery-crosscheck.json) / [封存目录](../../output/vesta-study/20261108-113622/reviews/20261109-142400-s07-recovery-vector-controls/README.md)。


## R88 单相直接保存与取消对照（2026-11-09）

R88预先登记同一VS0078单相PolarY1/Scale7输入的两种GUI上下文：①不开Vectors直接IPC-save；②单相打开Vectors/Preview on/Scale7后直接外层Cancel，不改字段、不增加相，再IPC-save。有效目标PID37035/41032的客户端均正常0，父进程另核实际文件两次稳定；两份3836字节输出均与输入SHA逐字节相同，Y1/Scale7完整保留。有效GUI正常0，45.565/123.453秒<180，无残留。因此在此样本中，单相打开/取消矢量窗口及单独IPC-save不足以触发R86第二相Y.1；尚需没有预开Vectors的双相追加对照，未把GUI追加阶段或完整根因宣称已定位。

初次nativeUI参数错误加入不支持的max_depth，工具bad_input/1未形成Evidence；按已知schema改为accessibility/max_nodes，原失败保留。首次第二组PID39128退出255/51.067秒，原生UI只depth0根节点，未执行取消/保存，对照不可用。同期另见空白研究实例PID37165，已通过CUA正常Quit清理；其创建及启动退出的精确因果未唯一证明，不把它当科学结果或正常GUI验收。新目录/偏好/唯一输入名重新登记后，先原生确认41032与已载入窗口再CUA操作；退出后通过拥有进程生命周期核实，避免对已退出app再次调用可能重新启动它的入口。中途一次在native读取未结束时尝试JSON读取空stdout发生JSONDecodeError，未触发产品动作，后续等待真实完成再读，不能把文件存在当完成。

14项限定核对通过；新增6原生ID至890，其中包含保留的空根节点启动观测。raw批量888/not_loaded2，完整890为原生raw加2条原生ID补读的明确派生快照。旧884/12问题头/78样本/原件257文件及87历史manifest/12184文件保持。R86持久化Y身份失败、Cancel SIG11和R87实证数值差异不撤销。254功能/32权威文件、计划10完成19未完成；Codex自验，dsh独立复审/S08未开始，无产品实现。

[14项核对](../../output/vesta-study/20261108-113622/reviews/20261109-145500-s07-single-phase-save-controls/save-controls-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-145500-s07-single-phase-save-controls/README.md)。


## R89 无预开Vectors的GUI双相追加（2026-11-09）

R89在首target前登记全新GUI：直接载入current X单位/Scale5，只通过共享短VESTA_PREF的CLI客户端执行-i incoming Y单位/Scale7及-save新文件，整个过程没有打开Vectors窗口。原生AX先绑定PID41723与唯一输入窗口；客户端正常0，另核父进程输出5347字节的两次稳定读数，确有两个不同TITLE、全局VECTS5，但实际vectors仍为(1,0,0)/(0,.1,0)，独立预期第二相Y1失败。新输出与R86原输出逐字节相同，SHA0eea1bd2c41b423d19d2dca370e361477dff1f250fb9d36e16183980e0c46214。

因此R86曾预开/取消Vectors并不是此数值变化的必要条件；R88单相Cancel/save两份逐字节保留输入，R89没有预开Vectors仍复现双相Y.1。数值问题已与已知VectorsCancel恢复崩溃的触发条件区分。R89没有重新读取Edit Vector数值或验证主模型第二相渲染；R87对逐字节相同R86文件的独立GUI回查已实证CartesianY.1/modulus.10，不能把R89称为又一次实际GUI字段回读。导入与保存之间的精确变化阶段仍未定位，下一步可在只-i尚未-save时，以全新双相对话框回查第二相数值，保持先验Y1。

父GUI正常0，47.116秒<180，无期限强制终止/残留；CUA仅在原生读取全部结束后绑定已有窗口并Quit，未调用Vectors/Cancel，不重放缓存单相→追加双相→Cancel崩溃。12项限定核对通过，但原数值身份预测失败保留。新增3原生ID至893，raw891/not_loaded2、完整893为明确派生的原生raw+2条按ID补读；旧890/12问题头/78样本/原件257文件及88历史manifest/12348文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[12项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-150700-s07-bare-gui-append/bare-append-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-150700-s07-bare-gui-append/README.md)。


## R90 保存前的第二相矢量读取（2026-11-09）

R90在首target前登记GUI current X1/Scale5与IPC incoming Y1/Scale7，只发送-i，不发送-save。原生先绑定PID44794与唯一主窗口，导入客户端正常0/父进程仍活；首次Vectors窗口是在双相导入之后才创建，Phase2/incoming_style_seven实际选中，Edit a Vector为Polar、Cartesian(0,.1,0)、Modulus.10、全局Scale5。独立保留原Y1/modulus1预期，此预测失败。主日志有Reading incoming、没有Saved data，工作目录始终只有两份未改输入，没有保存输出；因此差异已在任何保存之前的第一次GUI读取中出现，保存不是此显示数值变化的必要条件。对话框读取本身是否会改变内部状态没有被直接内存检查排除，不能把该观测推广成精确导入指令根因。

补充原生AX观测：该活体第二相的晶格、沿晶轴、倒易与Cartesian四组字段均为(0,.1,0)，而R87原单相输入为晶格Y.1/晶轴Y1/倒易Y10/CartesianY1，保存输出新开时为晶格Y.01/晶轴Y.1/倒易Y1/CartesianY.1。坐标基底未初始化成为有依据的待验证假设，尚未直接读取矩阵内存、也未证明初始化遗漏。旧R85 CLI Read函数已见临时Scene读取后只搬Crystal指针/文件名，无显式CoreInit调用，但其它间接/调用上下文仍需核，不据此单独宣布完整因果。

结束时只Cancel内层Edit Vector、外层Vectors OK，Quit对拥有研究副本选No，未执行任何Save/未重放缓存单相→追加双相→外层Cancel崩溃。原版GUI正常0/160.717秒<180，无强制终止/残留。13项限定核对通过，原数值保留失败不改写成成功。新增3原生ID至896，raw892/not_loaded4、完整896为明确派生原生raw+4条原生按ID补读；旧893/12问题头/78样本/原件257及89历史manifest/12476文件保持。254功能/32权威、10完成19未完成，Codex自验，dsh/S08与完整Study待，无产品实现。

[13项核对](../../output/vesta-study/20261108-113622/reviews/20261109-151300-s07-before-save-vector/before-save-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-151300-s07-before-save-vector/README.md)。


## R91 构造矩阵与命令调度初始化路径（2026-11-09）

R91仅静态研究，未启动VESTA运行实例。28次原生函数/符号查询成功，17个新增ID、11个已有身份复用。Crystal构造lib0x35f68/472bytes在+150/+178/+1a0创建MatrixDD，并在36074/7c/84调用MatrixD::Initialize0x33b0；该函数_bzero后按stride+1写IEEE double1，故构造期矩阵为单位阵。MatrixDD数据指针+10与旧writer读取Crystal+160吻合。此为静态对象布局证据，不是已读取活体第二相矩阵。

无界面split_command_and_exec(lib0x1fed4/1336)在20264构造VESTA_cmd_Read地址1e45c，202a4比较函数指针，202bc优先执行并从待执行数组移除所有Read；随后20358调用Scene::Build(NULL,-1)，2037c再执行余下保存等命令。GUI事件经execVSTXfile(main100019cb0/568)，100019e68解析、100019e80直接调用处理函数，所见路径无等价Build前置。旧Read函数仅临时Scene读取后插入Crystal指针/复制文件名，无显式Scene::Build调用。正常GUI文件打开OnFileReadImpl(main1000a7680/2280)则在a77f0 ReadFile后有a7d6c Scene::Build。三条执行路径差异已绑定指令与调用点。

Scene::Build(lib71a64/2096)在71b34调用BuildOrientMatrix(712a0/1836)；后者713a0 UnitCell::SetC2Ortho，并715d8/71600等写Crystal+150基底，71880/718a8处理其余矩阵。此前文档泛称“CoreInit”不是本次发现的原生符号，当前应使用这些已绑定函数名。构造单位阵、GUI-i缺少同样的前置Build、R90活体各坐标均Y.1及无界面Y1之间形成具体机制假设；尚无运行时矩阵内存和新holdout，完整导入根因不宣称已验证。

GUI解析器的实际令牌是flush，映射VESTA_cmd_gui_Flush(main100018dc0/92)：虚表槽310/e8/230各调用一次，三个目标尚未解析，不能只由名称宣称能修复数值；不把不存在于该解析器的refresh令牌当已验证命令。下一组应事前登记无界面-save先于-i的参数顺序holdout，以及GUI-i/-save在-flush前后两个新输出，观察是否恢复Y1；原件不修复、旧失败保留。

15项静态限定核对通过。完整证据913，原生raw893/not_loaded20，20条均通过原生按ID接口取回、与旧896逐项一致，明确派生完整快照；不是按上一轮4条遗漏推定。旧896/12问题头/78样本/原件257和90历史manifest/12627文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完，无产品实现。

[15项静态核对](../../output/vesta-study/20261108-113622/reviews/20261109-152400-s07-import-initialization/initialization-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-152400-s07-import-initialization/README.md)。


## R92 读取优先顺序与flush恢复失败（2026-11-09）

R92在首target前登记两项新对照。无界面参数故意写为-nogui -save OUT -i X -i Y：原版正常255/exited，实际先形成两相，保存X1/Y1、全局VECTS1，整个输出与R87常规-i X -i Y -save的两相文件逐字节相同。事前R91读取优先→Build→其余命令的顺序预测通过，这不是按参数文本从左至右逐条保存的契约。

GUI新进程载入X1/Scale5，第一次IPC-i Y1/Scale7/-save before输出复现两相Y.1/global5。第二次明确发送-flush/-save after；两个客户端均0、父进程仍活，两个不同输出文件分别经两次稳定内容检查，after仍Y.1、与before及R86完整5347字节逐字节相同。因此事前“flush后恢复Y1”的预测失败，不能宣称flush修复初始化，也不把它回写成预期.1。未运行时追踪三个虚调用回调是否逐一执行，不由此推广-flush一般无效。主日志有两次Saved路径；没有Vectors窗口/字段修改，主模型第二相渲染未另验。坐标基底初始化的完整因果仍待，不由无界面顺序成功替代GUI恢复失败。

GUI父57445正常0/64.178秒<180，无期限强制终止/残留；CUA仅在原生读完后绑定实际主窗口并Quit，无已知Cancel崩溃重放。15项证据范围核对通过，原flush恢复预期及originalAllPredictionsPass均false。新增5原生ID至918；本轮实际raw896/not_loaded22，22条经原生按ID补读构成明确派生完整918，旧913/12问题头/78样本/原件257及91历史manifest/12872文件保持。254功能/32权威、计划10完成19未完成；Codex自验，dsh/S08与完整Study未完，无产品实现。下一步转入全功能覆盖缺口盘点并补未观察入口，数值失败/静态假设作为明确边界保留。

[15项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-153900-s07-read-order-flush/read-order-flush-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-153900-s07-read-order-flush/README.md)。


## R93 输出文本、清空与目录覆盖回写（2026-11-09）

R93转向全功能缺口：修改前254功能中162条canonical status仍为entry-observed/behavior-pending，不能据此断言162条从未运行。重新核VM04 §4.2.1/.3及VM18 §18.5.4，并Deep定位OnSaveLog(main100098a08/456)调用wxTextAreaBase::SaveFile@100098ba8；OnClearLog(main100099568/20)尾跳br x1未解析，静态不冒称该虚目标已知。符号查询42匹配只返回40，目标两项已找到，不称全目录穷尽。

事前登记当前Output标签测试：File/Save Output Text保存到中文空格路径，首次1062字节与保存前原生AX文本的UTF8逐字节相同；View/Clear Text Area后原生AX为空，再保存另一文件为0字节，首次文件哈希保持。输入模型源不变。首GUI58637正常0/209.600秒，超过180交互目标29.600秒、在240软截止前退出；末尾模型检查被deadline断言在生成参数/调用工具之前阻止，未执行、未伪装成功。另事前登记只含清空→模型保存的新短流程，GUI59217正常0/69.892秒；原生确认空Output，IPC客户端0与父进程稳定文件另核，模型3834字节与输入逐字节相同。没有重放已知Cancel崩溃或修改模型参数。

19项限定证据核对通过，首轮originalAllGUIUnder180=false及originalFirstModelCheckExecuted=false保持。VF0006只核Command-Q选定正常退出、VF0017只核Output非空/空导出、VF0036只核Output清空及另流程模型保存保持；其余Summary/Comment、取消/覆盖/写失败/Undo及一般退出分支待。三条catalog记录回写具体前提/动作/持久化/失败边界，其他251记录不变；入口级待核由162降159，不把这三条称全分支完成。稳定ID仍254、样本78。

新增8原生ID至926，本轮实际raw898/not_loaded28，28条经原生按ID补读构成明确派生完整926；旧918/12问题头/78样本/原件257及92历史manifest/13075文件保持。32权威、计划10完成19未完成；Codex自验，dsh/S08与完整Study未完，无产品实现。

[19项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-154800-s04-output-text/output-text-crosscheck.json) / [覆盖回写](../../output/vesta-study/20261108-113622/reviews/20261109-154800-s04-output-text/coverage-after.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-154800-s04-output-text/README.md)。


## R94 Summary与Comment保存/回读（2026-11-09）

R94补Summary/Comment及选中Comment时的文本导出。事前已登记已知立方10Å/90°/1000Å³晶胞与Unicode两行注释，允许只覆盖新研究工作副本，原始样本/前轮产物只读。Summary原生AX实际显示current_style_five、Magnetic Space Group P-1' (BNS#2.6)、三轴10.00000Å、三角90.0000°、V1000.0000Å³；核已知几何与输入标签显示，不额外认证BNS编号科学正确性。

Comment的保存前原生AX与登记文本逐字相同；GUI Command-S后，实际文件注释正文比原预期多space+LF，原exact预测失败。保存调用中的首个CUA getAXState在Command-S之前已显示尾空格，与更早原生捕获有差异，提交瞬间未再次原生捕获，来源未隔离；不能唯一归因写入器、人或输入工具。保存完成由文件mtime/inode变化与两次稳定读数判定，不以文本符合预期作为完成条件。格式3.5.4把正文写在 <!-- COMMENT -- 与 -- END COMMENT --> 标记间；3911字节工程去掉该完整注释块后，与原3834字节工程逐字节相同，模型/样式数据保持。

另在首进程退出后登记全新副本/偏好，按已观察文件正文建立结果后、目标前的回读一致性预期；新GUI精确回读实际正文含额外尾空白，原始独立文字预期仍失败，不重新命名成通过。Comment实际选中时执行File/Save Output Text，输出1064字节精确等于切换前Output AX的UTF8，而非Comment正文，限定导出来源已有实证。两个拥有GUI62748/63370正常0，172.006/137.457秒均<180，无强制终止/残留，未打开Vectors/重放旧Cancel崩溃。

19项限定证据核对通过，originalExactCommentRoundtripPassed/originalAllPredictionsPass为false、尾空格提交原因未定。四条catalog记录回写，其中Summary/Comment两条入口级状态得到限定研究，Save与SaveOutputText扩展；其余250条不变，入口级待核159→157，不称四项全分支完成。样本78不变，这些是研究执行工作副本而非新独立稳定样本。

新增12原生ID至938，本轮实际raw900/not_loaded38，38条原生按ID补读组成明确派生完整938；旧926/12问题头/78样本/原件257及93历史manifest/13315文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[19项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-160900-s04-summary-comment/summary-comment-crosscheck.json) / [提交状态差异](../../output/vesta-study/20261108-113622/reviews/20261109-160900-s04-summary-comment/comment-stage-difference.json) / [覆盖回写](../../output/vesta-study/20261108-113622/reviews/20261109-160900-s04-summary-comment/coverage-after.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-160900-s04-summary-comment/README.md)。


## R95 相机缩放、适配与投影（2026-11-09）

R95验证View的Parallel、Perspective、Zoom In、Zoom Out、Fit to Screen五个入口。目标运行前登记独立数值预测，并只把工作副本的SCENE平移改成(1.5,-2.5,0.75)、zoom改成1.25；原始前轮文件只读。Zoom In实际为(1.65,-2.75,0.75)/1.375，Zoom Out整文件精确恢复基线；默认有效倍率1.1得到运行验证，未声称已读原测试窗口隐藏的Step控件。Fit to Screen把三个位移清零、zoom恢复1，3834字节结果逐字节等于原始零平移/zoom1文件。旋转矩阵及所有非SCENE字节保持。

OnZoomIn/Out main 0x100099324/0x1000993d8在Scene+0x120、+0x108、+0x10c应用(100+Canvas[0x580])/100，Out取除法，Z位移不变。OnReplot 0x10009948c写zoom=1及三个位移=0，与实际Fit入口效果一致；静态未解决的尾虚调用不宣称已动态追踪。

Perspective仅把DISPF 37753794改为39850946（bit21），PROJT 0改成1、距离0.962不变；Parallel切回后整文件精确恢复。OnProjectionMode 0x1000992b4按eventID0x85设Scene+0x5b0 bit21并调用InitCanvas。所有模型/SCENE数据保持；不据此宣称透视渲染像素或任意相机条件正确。

七次IPC保存均分别核client正常0与父进程新文件两次稳定读数，不以预期字段正确作为完成条件；四次原生AX读取绑定拥有PID/独有标题。两个主GUI 64339/64626正常0，99.118/82.843秒均<180。首次Quit后附加getAX出现空白PID64524，已正常Quit请求并确认消失，未捕获其退出码；时间关系与退出后查询可重新启动应用一致，启动机制未独立追踪，不把它伪装成拥有主进程的正常退出证据。之后Quit不再调用死应用UI。

26项限定核对通过，原数值预测均通过。五条功能记录回写，其余249条不变，入口级待核157→152；自定义步长、极值、旋转相机、无模型、多相和像素渲染仍待核。样本78不变。新增21原生ID到959，实际raw914/not_loaded45，45条原生按ID补读组成明确派生完整959；旧938、12问题头、原件257及94历史manifest/13594文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[26项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-163500-s04-view-camera/view-camera-crosscheck.json) / [功能回写](../../output/vesta-study/20261108-113622/reviews/20261109-163500-s04-view-camera/coverage-after.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-163500-s04-view-camera/README.md)。


## R96 结构模型显示与样式（2026-11-09）

R96验证结构模型三个显示开关与五种绘制样式，目标前登记字段、复原及原生AX预期。原DISPF37753794/MODEL0,1,0。Show Models关闭实际DISPF37753792且MODEL0,0,0；点表面开启实际DISPF37819330且MODEL0,1,1；仅非对称单元开启DISPF37770178且MODEL0,1,0。原“显示开关只改DISPF、MODEL不变”在hidden/dot两阶段失败，原计划保留，不把观测后的双字段解释当事前通过。MODEL第二/第三字段分别与bit1/bit16一致，当前仅为观测一致性；旧R83保存函数在lib0xaf630调用XStyle::write_style0xb89ac，写入器本体尚未追踪，不称唯一原因已证明。

五种样式实际MODEL首字段为Ball-and-Stick0、Space-Filling1、Polyhedral2、Wireframe3、Stick4，原预测通过，DISPF与其余字节不变；恢复Ball-and-Stick整文件精确复原。菜单OnModelStyle main0x100099b30以eventID减0x8b写Scene+0x5b4，侧栏OnRadioModel0x1000a1354以event值写同字段。三个显示处理器写Scene+0x5b0 bit1/14/16，与实际文件及AX开关一致，未解决的控件虚调用与刷新不宣称动态追踪。

菜单与侧栏交替操作，11次原生AX绑定拥有PID/独有标题，确认对应开关和五选一结构样式；13次IPC保存各自核client正常0和父文件两次稳定观测。显示开关三次复原与样式最终复原均整文件等于输入。所有阶段除DISPF/MODEL两行外逐字节不变。样本只有两个Fe、零键/零多面体，不能用本轮样式选择证明渲染几何；未做新实例回读/多相/像素验收。

主GUI65157正常0/181.153秒，超过180目标1.153秒，全部原生研究捕获仍在180前、生命周期小于240；styles65515正常0/141.813秒。allPrimaryGUIUnder180与originalAllPredictionsPass为false，未改写通过。两次Quit后只做外部进程检查，无残留、未打开Vectors/追加相/重放Cancel崩溃。初版分析按标题合并三个面板的Wireframe控件，后者0覆盖结构模型1而误报；原生AX无enabled字段。已保留初版脚本/失败报告，修正为从Ball-and-stick起的连续五项且校验标题顺序，原生载荷不变、未重跑目标。

63项限定核对通过，原预测和耗时失败保留。八条记录回写，其中六条入口状态得到限定契约、两条旧契约扩展，其余246条不变，入口级待核152→146，旧非对称过滤及多面体公式证据保留。78样本不变。新增30原生ID到989，实际raw928/not_loaded61，61条原生按ID补读组成明确派生完整989；旧959/12问题头/原件257及95历史manifest/13942文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[63项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-170000-s04-model-display/model-display-crosscheck.json) / [功能回写](../../output/vesta-study/20261108-113622/reviews/20261109-170000-s04-model-display/coverage-after.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-170000-s04-model-display/README.md)。


## R97 显示状态写入与新实例回读（2026-11-09）

R97静态查XStyle::write_style lib0xb89ac/2588字节647指令：0xb89cc读XStyle+0x388打印DISPF；0xb89e8/0xb89ec从同字提取bit1/bit16，0xb89f0读+0x38c样式号，0xb8a08打印MODEL首字段/显示模型/点表面三列。复用旧R83 save_data原生证据，其0xaf628以Scene+0x228为XStyle、0xaf630调用该写入器；0x228+0x388=0x5b0、+0x38c=0x5b4，与R96处理器字段对齐。这静态解释了双编码，未声称动态调用追踪或冲突字段优先级已核。

分别用全新副本/短偏好目录/新GUI回读R96 hidden、dot、wire文件，目标前登记按已观察保存状态建立的回读一致性预期。原生AX确认隐藏模型、启用点表面、线框单选状态；三次IPC再次保存的3834字节文件分别精确等于原输入。三个拥有GUI67360/67419/68017正常0，31.583/28.920/34.409秒，全部<60，无残留；未做编辑/追加相/打开Vectors，Quit后不读死应用UI。此处是结果后、目标前的回读检验，不冒充独立科学预期；R96原only-DISPF预测及181.153秒超目标失败原样保留。

26项限定核对通过，3条既有功能契约扩展、其余251条保持，入口级待核仍146。字段冲突优先级、其它样式/显示位的新实例、复杂/多相及像素渲染仍待核。新增7原生ID到996，实际raw935/not_loaded61，按ID补读形成明确派生完整快照；旧989/12问题头/78样本/原件257及96历史manifest/14398文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[26项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-171600-s04-display-persistence/display-persistence-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-171600-s04-display-persistence/README.md)。


## R98 体数据显示与表面样式（2026-11-09）

R98直接读VS-0067有符号4×4×4 Cube的只读字节副本，验证切片、等值面及三种表面样式。目标前登记按初始AX值切换对应显示位、写入器重复字段及复原预期。切片关闭DISPF37753794→37753666、SECTS32,1→32,0；等值面关闭DISPF→37753730、SURFS0,1,1→0,0,1，复原各自整文件精确相同。原生AX确认1→0→1；关闭等值面时CUA可见表面样式单选禁用，恢复后可用。样式Smooth0/Wireframe1/Dot2只改SURFS首字段，恢复Smooth整文件精确相同。

处理器OnShowSection0x1000997ec/OnShowIsoSurf0x1000998a0写Scene+0x5b0 bit7/6；OnShowTexture0x1000999cc静态写bit8。OnSurfStyle0x100099bb4以eventID减0x93、OnRadioSurf0x1000a13ec以event值写Scene+0x356。复用R97 XStyle写入器：SURFS首字段从+0x12e读short，后两字段提取flags bit6/8；SECTS后字段提取bit7，Scene+0x228+0x12e=0x356。只观察指定显示状态，未做网格/切面/等值面渲染科学或像素验收。

Surface Coloring在实际两个单Cube上下文的菜单和侧栏均禁用，原生AX值0而文件bit8与SURFS第三字段仍为1，原“所有基线控件值均等保存位”的预测失败。保留潜在场景标志与禁用控件投影的差异，不改称通过。原生AX未给enabled元数据，禁用状态来源明确为CUA工具观察转录。按事前分支未强制点击，未启动另一个coloring进程，无其客户端参数/输出/偏好目录；不声称已验证着色功能或已隔离启用条件。

9次IPC各自核client正常0与父新文件两次稳定读数，8次原生AX绑定拥有PID/独有Cube标题；除DISPF/SURFS/SECTS三行外全部工程字节保持，源Cube原样。两个GUI69947/76652正常0，132.357/108.454秒均<180，无残留、未开Vectors/追加相/重放Cancel。58项限定核对通过，originalAllPredictionsPass及SurfaceColoringExecuted为false。六条记录修改、其余248条保持，其中五条入口状态得到限定契约；VF0047仍entry-only，入口级待核146→141。78稳定样本不变。

新增22原生ID至1018，实际raw953/not_loaded65，按ID补读形成明确派生完整快照；旧996/12问题头/78样本/原件257及97历史manifest/14736文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[58项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-172600-s04-volume-display/volume-display-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-172600-s04-volume-display/README.md)。


## R99 晶形显示与样式（2026-11-09）

R99用官方Adularia-single-crystal六晶面工程的只读字节副本核Show Shapes与三种晶形样式。源SHA ebb658d176db93ffbfce169e8ff6bd3deea7e8b4896447aeceabd89640130c59，旧3.0.0/3807字节，当前另存基线3.5.4/3752字节，DISPF5058/FORMS0,1；不要求旧新格式整文件相等，也未认证完整格式转换或晶面几何/面积。源工程保持，旧新差异文件保留，稳定样本目录78不变，本轮为官方示例的研究工作副本。

第一次CUA菜单Show Shapes点击后，原生AX仍1，hidden命名的3752字节文件与基线精确相同，原隐藏预测失败。后在首次结果之后、第二次操作之前登记重试：第二次菜单点击原生AX变0、DISPF5058→4546/FORMS0,1→0,0；再侧栏恢复1，文件整字节恢复基线。初始菜单checked状态和真实回调未被捕获，不能唯一归因应用同步、旧格式或输入工具。保留first-menu failure与结果后重试的边界，不用重试覆盖原失败。

侧栏Custom color1、菜单晶形Wireframe2、侧栏Unicolor0均有原生三选一状态，只有FORMS首字段改变，恢复单色整字节等于基线。处理器OnShowShapes main0x100099a70写Scene+0x5b0 bit9，OnShapeStyle0x100099c1c/OnRadioShape0x1000a1468以eventID−0x98/事件值写Scene+0x3c0。复用R97写入器FORMS从XStyle+0x198读样式并提取flags bit9；Scene+0x228+0x198=0x3c0。所有阶段除DISPF/FORMS两行外保持基线全字节，不据此接受几何、配色像素、多孪晶或完整渲染契约。

8次IPC分别核client正常0和父文件两次稳定读数，8次原生AX绑定拥有PID/独有标题。主GUI88180/93853正常0，154.241/98.468秒均<180，无残留、未开Vectors/追加相/重放Cancel，Quit后不查询死应用。56项限定核对通过，originalFirstMenuHiddenPredictionPass/originalAllPredictionsPass为false，registeredRetryPredictionPass及样式预测true，首次无效原因未隔离。

四条入口记录改成限定契约，其中ShowShapes明确partial/failure；其余250条保持，入口级待核141→137，不称四功能完整成功。新增19原生ID到1037，实际raw972/not_loaded65，按ID补读组成明确派生完整1037；旧1018/12问题头/78样本/原件257及98历史manifest/15161文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[56项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-174100-s04-shape-display/shape-display-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-174100-s04-shape-display/README.md)。


## R100 晶形菜单对照与内部状态门控（2026-11-09）

R100在两个独立新实例中使用相同当前3.5.4文件（SHA0fedd1bdef2efef374149c334a7842e22e83cfeeeb6b0a9050e49ef5a42aaa0d、3752字节、DISPF5058/FORMS0,1），事前登记新的行为假设。未调节fresh实例首次菜单ShowShapes后原生checkbox仍1，保存全文件等于基线，复现R99的无变化现象，说明旧3.0格式不是该选定现象的必要条件。另一conditioned实例先侧栏off/on，再首次菜单点击，原生状态0，文件仅DISPF4546/FORMS0,0；侧栏off文件与after_menu精确相同、on和baseline精确同源。六次IPC各自核client正常0及父文件两次稳定观测；六次原生AX绑定拥有PID/独有标题。两GUI4180/4648正常0，46.419/89.879秒均<180，无残留。

静态VestaFrame::SetControlState main0x1000a8848完整91指令体显式取显示bit1/7/6/8并对结构/体数据菜单调用虚槽，直接体没有晶形菜单字段+0x4c0或ID0x97的同类更新；随后调用ChangeShapeStyle、SidePanel::SetControlState。后者0x10008d21c完整107指令体更新侧栏，未列具名Menu callee；OnMenuOpen0x1000a0154为651指令体。结合对照支持“初始菜单/侧栏同步不一致”的解释，但菜单内部checked值与真实回调未捕获，虚调用/其它路径未闭合，不能宣称全程序无同步或唯一根因已证明。本轮事前假设通过，不把R99原始关闭预期失败改成通过，亦不再循环重放该现象。

同一控制状态代码补SurfaceColoring静态条件：Frame把Scene+0x130内部状态与+0x5b0显示标志传侧栏；菜单着色可用性参数由显示bit6与内部状态bit2共同决定。侧栏先检查内部参数bit2，成立才把显示bit8送到着色控件+0x340的虚槽0x720；bit2不成立跳过赋值，不能说一定重置0。该控件虚槽0x1e0参数为(stateArg&0x6)==0x6。方法名未动态解析，以控制对象绑定/观察解释值与可用性；内部位的用户语义未建立、未读取运行时内存、未执行着色正向分支。R98禁用UI0而潜在位1的原失败保持，不等同于已验证启用条件的完整用户输入契约。

33项限定核对通过。仅VF0047静态条件与VF0051已有部分契约扩展，其余252条保持，入口级待核仍137，着色仍entry-only，稳定样本78不变。新增15原生ID至1052，实际raw987/not_loaded65，原生按ID补读组成明确派生完整1052；旧1037/12问题头/原件257及99历史manifest/15575文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[33项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-175600-s04-shape-menu-sync/shape-menu-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-175600-s04-shape-menu-sync/README.md)。


## R101 常规属性与关闭阶段工作副本异常（2026-11-09）

R101先追原生DlgGeneralPage与DlgAllProp，再只读观察属性面板，事前登记三个独立功能实例。UCOLP第一值为线型Solid0/Dotted1/Dashed2、第二值为晶胞DoNotShow0/Single1/All2。分别写Scene+0x22a/+0x228；原生XStyle写入器0xb8f20/24依此顺序输出。COMPS为枚举而非两个独立位：0隐藏罗盘、1罗盘带标签、2罗盘不带标签。OnShowAxis100048b40/OnShowAxLabel100048c88与UpdateCanvas10004a33c写Scene+0x234，写入器从XStyle+0xc输出。原生AX初值、各组选中与Preview1均捕获。

属性对话框保持打开时，IPC可另存预览Scene的当前值，16个阶段输出逐字节符合事前UCOLP/COMPS字段预期，除指定字段外所有工程字节保持规范源。单晶胞恢复整文件相同；再次选择All后原生捕获，Cancel再保存精确恢复原3834字节。线型实线往返精确复原，再选虚线后OK提交style2；轴标签关闭后OK提交COMPS2。罗盘关闭时，轴标签控件值仍1但禁用，COMPS为0，开启后恢复；不把禁用控件勾选等同于当前可见状态。Preview-off、完整渲染/几何、其它页面及默认保存未验。

必须保留的失败：只读观察GUI14762正常0/94.013秒，超过90秒目标4.013；三个功能GUI15560/16295/21844正常0，152.371/179.579/166.828秒均<180，其中线型接近目标。线型退出实际出现保存提示，CUA选择No，输入工作副本保持；罗盘退出未捕获所属提示，预期提示查询与进程退出竞态后观察到空白实例23103，已正常Quit请求并确认消失，退出码未跟踪。更重要的是compass-model.vesta被未预期改成COMPS2（SHA52cfb649b8e6b8549c67efc2c7c3c4ede511d372d807c2b4b3a5e5a1183a015c），原“全部工作输入只读保持”失败。最后after_ok客户端原生before/after快照仍为d4c50885原哈希；其结束上界10:30:15.267588UTC，文件mtime10:30:27.975007，主进程正常退出10:30:28.102002，变化处于该区间，写入者/精确触发未隔离，不归咎某个组件。

规范源R93 clear-model-only.vesta哈希d4c508851f6fe92c09e2931e31079dff6d9468fe748d950250887dab16bc9f48及cell/lines输入均保持。异常compass工作副本、mtime、差异及原生旧快照保留，没有恢复或改写以制造只读通过；字段预期从事前规范源计算，不能从变化后的副本倒推。88项限定核对通过、originalSerializedFieldPredictionsPass=true，但allWorkingInputsPreserved/inspection90GoalPass/originalAllExpectationsPass=false。关闭协议后续改为先等拥有Popen，若仍活跃以显式PID原生捕获证明确有该工作文件的关闭提示，再用CUA处理；不因一次仍活跃读数就按bundle查询应用，避免退出竞态重开。下轮需登记并实现该清理专用守卫，以及Quit前后输入哈希取证。

十条功能记录回写，九条入口状态细化，General既有契约扩展，其余244条不变，入口级待核137→128；不称十功能完整成功。稳定样本78不变。新增48原生ID至1100，实际raw1017/not_loaded83，83条原生按ID补读组成明确派生完整1100；旧1052/12问题头/原件257及100历史manifest/15963文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[88项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-181000-s04-general-properties/general-properties-crosscheck.json) / [输入改写取证](../../output/vesta-study/20261108-113622/reviews/20261109-181000-s04-general-properties/working-input-change.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-181000-s04-general-properties/README.md)。


## R102 常规属性关闭与No保存控制（2026-11-09）

R102先静态追关闭链，再事前登记两个短偏好目录/独立研究副本，禁止退出后按bundle盲查。Properties/OK main0x1000206f0在0x10002072c调用Document::PushHistory(2)；lib0x20a2c在非空历史分支把Document+0x1f8所指对象+0x131写1。OnFileClose main0x1000a71e8从Document::GetCurrent取得Scene，检查Scene+0x131，提示调用位于0x1000a7374；消息返回8跳过保存、2走0x1000a7538调用OnFileSave，其它走0x13ed取消。Quit事件OnExit0x100098db8转wxWindowBase::Close(true)，OnDestroy0x1000a14e4遍历文档调用OnFileClose并在13ed停止。所读函数内未见无条件回写工程分支，但含未解析虚调用，不作全程序不存在自动保存的断言；数字返回值不是动态回调追踪。

两个新当前格式副本均来自R93规范3834字节源d4c508851f6fe92c09e2931e31079dff6d9468fe748d950250887dab16bc9f48，Properties Preview1，仅线型改Dashed或轴标签关闭后OK。分别IPC另存到新文件：虚线SHA02cbf587b308922e808910690916b4ee743d7e73475504c5e5647b5bfbdbd85b，COMPS2为52cfb649b8e6b8549c67efc2c7c3c4ede511d372d807c2b4b3a5e5a1183a015c；整文件精确符合目标前指定字段预期，源与输入工作副本仍原字节。Quit前、按原PID捕获提示前后、退出后均记录输入哈希/mtime；两个实际原生提示均含所属文件名及Yes/No/Cancel。只有原生守卫确认提示且所属PID仍活才查询CUA并明确点击No，输入完整哈希与mtime均未变。0.1秒轮询未观察到变化，不冒充系统调用/写入者追踪。

拥有GUI33485/34014正常0、112.338/104.157秒，均<180且无期限强停/残留；8次原生UI/2个IPC客户端分别核验。退出后再执行compass原PID守卫，确认lifetime已记，既不原生读取也不查询CUA，不会因研究查询主动重开空白实例。首线型CUA选择器错误地要求No后行尾，而实际有ID后缀，未点击即抛错；刷新结果明确按钮4后点击No成功，选择器失误保留。两流程没有新空实例。

这两个单次字段变更控制中没有观察到R101工作副本异常写入，并非完整重放R101全部切换序列，不能据此把R101原只读预期失败改成通过或归因某组件。R101异常COMPS2工作副本、mtime区间与全部原生旧快照保持，写入者和精确触发仍未隔离。58项限定核对通过；本轮所选No丢弃和守卫契约有证，Cancel/Yes/多文档/写失败/完整关闭协议未因此全部验收。

新增19原生ID至1119，实际raw1031/not_loaded88；88条原生ID补读组成明确派生完整1119。旧1100/12问题头/78稳定样本、原件257及101历史manifest/16588文件保持。六条旧记录扩展、其它248条不变，入口级待核128不变；254功能/32权威、10完成19未完成，Codex自验，dsh/S08与完整Study未完成，无产品实现。

[58项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-185900-s05-close-write-semantics/close-write-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-185900-s05-close-write-semantics/README.md)。


## R103 边框Cancel与默认属性选择性复制（2026-11-09）

R103事前登记同一新单相晶形副本及后续复用短偏好目录的新CIF进程；原规范源为R99 current3.5.4/3752字节/SHA0fedd1bdef2efef374149c334a7842e22e83cfeeeb6b0a9050e49ef5a42aaa0d，含六个晶面。General Show edges的Scene+0x3c2对应XStyle+0x19a，原生写入器0xb8b04打印FORMP首值。边框关/开输出仅该值0/1，开启精确恢复整源；各阶段完整文件符合目标前字段预期。

两个宽泛假设均被否定，保留原登记而不改成通过：关闭边框和轴标签后点击Save as Default，再Cancel，当前Scene保存为FORMP0/COMPS1，指南针恢复而边框仍关闭，所以“Cancel恢复全部改变字段”不成立。全局默认最终为FORMP1/COMPS2，指南针变更进入默认而边框未进入，所以“按钮保存所改全部General字段”亦不成立。这是所选字段与路径结果，不擅称所有Cancel/默认行为均坏。原生CopyPropertiesTo lib0xba0ec复制+c的COMPS，直接块0x148..0x197在FORMP的0x198..0x1a3之前结束，后续直接字段由0x1a4起；R101 CopyParam/Cancel与按钮都调用它。静态选择性复制与两种实际不对称吻合，未动态追踪回调。

按钮main0x100020750将Scene+0x228属性复制到_defParam，处理器无直接文件写入。原生GetPrefvar lib0x21198读取VESTA_PREF并赋_prefdir，存在变量时跳过HOME回退；FinalizeLibrary0x22890转save_default_pref0x2219c，在0x22810调用DefaultParam::save_style0xb9cc0，拼_prefdir与style/default.ini并调用既有XStyle写入器。运行明确设/tmp/v103d-1930，按钮前/后和Quit前的归属快照均无default.ini，正常退出才出现50724字节文件SHA080fcc15316eee9ccf493d8a9ad5637f4e52aad45cc2343a49bd33b7c8155ac2、FORMP1/COMPS2。随后新进程直接读既有VS0070的533字节CIF副本，原生General读到Showedges1/axislabels0，另存3975字节项目字段一致；第二次退出default.ini整字节哈希仍相同，mtime正常更新。外部用户偏好没有独立前后清点，不能把归属路径证据扩大为已核所有外部文件。

GUI44013/58052正常0，159.555/67.855秒均<180；六个IPC输出各独立核发送结果及父文件稳定，八次原生UI完整。两个Quit后归属Popen已结束，清理守卫跳过原生与CUA查询，未重开空白实例。两工作输入及规范源哈希/mtime保持，稳定样本78未增。61项限定检查通过、静态选定字段预测通过；broadCancelAllFieldsHypothesisPass/broadSaveDefaultAllFieldsHypothesisPass=false。不改R101副本异常写入的旧失败、也未隔离旧写入者。

新增25原生ID至1144；实际raw1022/not_loaded122，122个ID原生补读组成明确派生完整1144，旧1119及12问题头保持。原件257和102历史manifest/17117文件校验，六条功能扩展、其余248不变，入口级待核128→126；254功能/32权威、10完成19未完成。Codex自验，dsh独立复审/S08与完整Study待，无产品实现。

[61项限定检查与两个被否定假设](../../output/vesta-study/20261108-113622/reviews/20261109-191500-s04-general-defaults/general-defaults-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-191500-s04-general-defaults/README.md)。


## R104 Atoms字段与选择性Cancel（2026-11-09）

R104使用既有VS0057 P1非正交各向异性CIF（774字节，SHAdc52d2f42554ceb909a6d9ea129a3bfa54efa1ca708d4bb1c605b34f3d892840），两个U张量的Sylvester主子式均正，作为有用的正ADP输入，不等同椭球几何验收。先静态追Atoms控件，再登记GUI元变形规则：球体0/椭球1对应DISPF bit17和ATOMS第二值，主椭圆对应Scene+0x274/XStyle+0x4c、ATOMP第三值；隐藏非成键原子为bit11/ATOMP第六值，手性标记为bit23/LABEL第四值。其余所有工程字节不变作为目标前检验规则，基于首个稳定GUI基线而非独立科学渲染oracle。

原生Atoms UpdateCanvas10002e148返回35条指令、两处间接跳转未解，coverage complete1/1与未截断只表示该结果有一个函数，不证明所有分支覆盖；原始限制明确说可能缺代码。首次native objdump以地址范围调用，/usr/bin/objdump实际忽略范围，exit0却从100001630开始且共378601行，不能当范围取证成功。保留该记录，另用offset40000/limit12000原生分页，从其输出按地址派生182条连续指令，完整覆盖10002e148..420，补147条遗漏；派生文本不是原生记录原样。两个原生u8读取表1004d73aa/ab均66，事件2bc2/2bc3从基址10002e194加66×4均到10002e29c，0x10002e2b0 bfi写bit17，证明模式分支字段。R97写入器支持各重复编码，R103 CopyPropertiesTo mask0x02422800含11/13/17/22/25而不含23；静态映射不是动态执行追踪。

首个modes GUI53175未先确认监听即发送基线保存，客户端255/exited且无文件，wrapper0不代表成功；parent仍活，后续原生主窗证明模型已加载。controller还在client完成前试读缺失文件而报FileNotFoundError，脚本失误保留。该GUI未改属性，正常Quit0/86.503秒；发送时实际IPC监听未取证，原因不唯一归为启动竞态。另登记新modes_ready，先原生绑定主窗、lsof证实拥有PID的短socket，再发客户端0且稳定文件；后续短实例沿用这个守卫。

模式往返GUI72670初始两个模式单选框AX均0，工程ATOMS第二值0，异常初值保留，未唯一归因。手动选椭球后选中1且启用概率/主椭圆控件，principal开/关、球体恢复取得5个4180字节文件；最终整文件恢复基线SHAbc14120712f5edbfece81aca0689b2805fabfbf6cb9498c703513e3c99ff8577。该实例正常0/185.547秒，超过180目标5.547；未执行原计划改变状态后的Cancel/after_cancel，不从无编辑收尾Cancel倒推回滚通过。原toggles长实例未启动，之后重新事前登记三个最小上下文。

新mode_cancel88866将椭球+principal开启后Cancel，整文件恢复基线；hide_cancel95156开启Hide后Cancel亦完整恢复；mark_cancel96110开启Mark后Cancel，bit23/LABEL末值仍1，afterCancel与changed文件整字节相同（SHAf27eebd6581c0c781732de15623f3cb1c802b03037e1e4397b145ea191ff6eff），宽泛“Cancel恢复所有改动”在手性标记上不成立。三实例正常0/94.999、88.677、95.001秒，均<100；与选择性copy掩码一致。五个实例均正常退出，原输入及规范源哈希保持；所有退出后的原PID守卫跳过原生/CUA查询，无空白新实例。14个阶段输出精确符合元变形规则，11次原生UI完整，95项限定核对通过；初始保存失败/旧180目标失败/mark完整回滚假设失败保留。椭球/主椭圆几何、成键/孤立视觉对照及手性配对着色尚未验收。

本轮新增37原生ID至1181，raw1052/not_loaded129，129个ID原生补读组成明确派生完整1181，旧1144/12头保持。汇总脚本在完整快照/来源/加载审计均完成后，索引阶段把commands数组当dict而失败；原脚本/错误保留，仅补isinstance检查并独立重验union/旧记录，补完37-ID索引，不重导出或重跑原生。原件257、103历史manifest/17748文件及78稳定样本保持；7条功能扩展、其余247不变，入口级待核126→121。254功能/32权威、10完成19未完成，Codex自验，dsh/S08与完整Study待，无产品实现。

[95项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-193400-s04-atom-properties/atom-properties-crosscheck.json) / [分支覆盖限制与补证](../../output/vesta-study/20261108-113622/reviews/20261109-193400-s04-atom-properties/dispatch-coverage-followup.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-193400-s04-atom-properties/README.md)。


## R105 Bonds与SBOND耦合及不完整Cancel（2026-11-09）

R105以旧R66真实成键工程positive-project.vesta为规范源（5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，原有七原子/六条2Å键/一个SiO6多面体证据保持）。三个新GUI各先原生绑定PID/窗口/7atoms6bonds1polyhedra，再lsof证明所属短socket，然后发送基线IPC；三个基线均整字节等源。Properties/Bonds六个样式索引为Unicolor0/Bicolor1/Colorline2/Gradient3/Dotted4/Dashed5；原生六单选框的唯一选中均符合所测阶段，初始Bicolor1假设通过。

静态OnBondMode main0x10002ea34读组选中索引到dialog+0x41c，转event0x2c28/UpdateCanvas，在0x10002fd50调用Scene::UpdateBondsStyle。lib0x8134c写Scene+0x28c，并循环每相调用Crystal::UpdateBondsStyle0x6b534，后者对Crystal+0x238向量对象进行引用/复制并在0x6b57c写byte+0x40。原始计划曾把这些视为可能只供绘制的对象，实际文件证明它们还对应被序列化的SBOND规则，不能回写旧计划制造事前知道。复用R97 XStyle写入器从+0x64打印BONDS，Scene+0x228+0x64=0x28c；R103默认样式文件SBOND格式的第十值（零基token9，含行号）在0xb9ffc读byte+0x40。后者是style/default.ini写入器的静态布局证据，未冒充已追踪本轮工程保存调用。

初始圆柱组单色0实际同时改变BONDS0与唯一Si/O SBOND规则样式0，原“只改BONDS、其余工程字节保持”失败；手动改回Bicolor1精确恢复整源。再选Unicolor0、原生捕获后Cancel，保存结果BONDS回1，但SBOND仍0，原完整回滚预期失败，afterCancel SHA4dcfd80f917f53ed3de489aa138c628c2f98d1d4abb7b743592e93245a26108e，输入研究副本仍原哈希。与CopyParam仅恢复所选XStyle/其它向量、未见Crystal+238恢复的静态路径一致，但没有动态恢复调用/唯一原因追踪。

观察后、后续目标前另登记耦合字段预期：彩色/渐变组2/3，点/虚线组4/5，两级样式一起变化；Cancel后预测全局回1、SBOND保留最后3/5。原始单字段/完整回滚预期保持失败，不把后验规则追溯成通过。两后续组8个文件逐字节满足新先验，其余字节、结构/规则距离/半径/线宽/RGB等保持。三个afterCancel实际为(1,0)/(1,3)/(1,5)，后两文件SHA859e1e750407ae8162600903b6e86660aaeb7c653d9498e1321de74d7f35bb8e和65b731abfca2973635acb6a4c7c303687b6d7fdfde8cdfadc2b715b49d088f39。重新打开的优先级、实际颜色/圆柱/虚线像素和完全内存回滚未验，不能因BONDS1或文件存在宣称Cancel成功。

三拥有PID45983/54319/57263正常0，190.894/122.949/121.544秒，无期限强停。120目标全部未达，分别超70.894/2.949/1.544秒；圆柱亦超180目标10.894，其余两组<180，所有功能native发送/读取在180前、关闭均<240。退出后原PID守卫直接跳过原生/CUA查询，无空白新实例，规范源及三个输入哈希/mtime保持，六个归属偏好文件归档。首次后续计划准备脚本因原计划缺actions键抛KeyError，未写新计划或改目标，错误保留；修正的新计划明确写动作且先于后续GUI。原计划未补写actions。

72项限定核对通过、12阶段文件与11原生UI完整；originalOnlyBONDSPredictionsPass/originalFullProjectCancelRestorePass/originalAllExpectationsPass=false，followupCoupledPredictionsPass=true。新增31原生ID至1212，实际raw1074/not_loaded138，138个ID原生补读组成明确派生完整1212；旧1181/12头、原件257、104历史manifest/18538文件及78稳定样本保持。八条功能扩展/其余246不变，入口级待核121→115；254功能/32权威、10完成19未完成。Codex自验，dsh/S08与完整Study待，无产品实现。

[72项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-200500-s04-bond-properties/bond-properties-crosscheck.json) / [初始失败保留](../../output/vesta-study/20261108-113622/reviews/20261109-200500-s04-bond-properties/cylinders-observed-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-200500-s04-bond-properties/README.md)。


## R106 键规则冲突重开与手动同步（2026-11-09）

R106以R105冻结失败后像为源，分别新建全局BONDS1/规则SBOND0及1/3的工作副本，短独立偏好、原版进程，先原生PID/窗口/7atoms6bonds1polyhedra及lsof所属监听就绪，再执行初始IPC。三个实际baseline（梯度初次与重试、单色）均5144字节整源一致；重新打开未自动归一化被Cancel留下的规则，R105原始单字段/完整回滚预期仍false。

单色冲突副本原生Properties/Bonds六单选仅Bicolor1选中，与全局BONDS一致而非规则0；未编辑、直接Cancel后的新输出整字节保持(1,0)。梯度独立重试同样原生仅Bicolor1选中、规则仍3；再次点击已选Bicolor，after_same整字节仍(1,3)。先Unicolor0、再Bicolor1，每步CUA确认唯一选中，随即Cancel，新输出仅SBOND规则3→1，BONDS仍1，其余字节保持，整个5144字节精确等于R66规范源SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。原生读框与CUA动作记录分别标识，不冒充手动中间状态都有原生捕获。

复用R105完整130指令UpdateCanvas：0x10002fd40读Scene+0x28c、fd44读dialog+0x41c，fd48比较，fd4c相等跳至fdd0、绕过fd50的UpdateBondsStyle；后者经Scene8134c/Crystal6b534向规则byte+0x40写样式。此静态路径与同选项点击不归一化、异选项往返归一化的实际文件一致，但本轮没有事件/回调追踪；已选单选也可能根本不发事件，不能宣称动态唯一原因。界面选中值与文件值分歧不等于已验证实际绘图优先级；颜色、圆柱/虚线像素、多规则覆盖、几何科学验收仍未完成。新文件仅是研究行为控制，不修补/覆盖R105失败后像、原件或产品。

初次梯度PID66525完成baseline后遇上下文恢复，恢复时已过180功能窗及240关闭目标；仅读取既有主窗口后Quit，未打开Bonds或执行后续交互。实际正常0/251.587秒，120/180/240目标与完整动作执行皆false，两个计划输出不存在，失败单独留存。另立、先登记的新梯度重试PID76692正常0/82.804秒，单色PID72362正常0/58.586秒，均<120，未用新成功抹去原超时。所有实际功能native读取/IPC在各180窗前，退出后无应用重查/空白新实例；初次过清理读取截止后未补做原生读取。三工作输入哈希/mtime和每0.1秒抽样单一状态保持，三源为冻结旧证据，六个所属偏好常规文件已归档。未宣称捕捉每次系统调用或写入者。

54项限定核对通过、6个阶段文件/5个PID原生界面；新增11原生ID至1223，raw1082/not_loaded141，141个原生按ID补读组成显式派生完整快照1223，原raw不改。旧1212记录/12问题头、原件257、105历史manifest/19284文件及78稳定样本保持。扩展五条VF57/131/132/134/143，其余249不变；入口级待核115不变，254功能/32权威文档、计划10完成19未完成。Codex自验；dsh独立复审、S08与全Study仍未完成，未开展产品实现。

[54项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-203400-s04-bond-reopen/bond-reopen-crosscheck.json) / [初次超时留存](../../output/vesta-study/20261108-113622/reviews/20261109-203400-s04-bond-reopen/gradient-time-window-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-203400-s04-bond-reopen/README.md)。


## R107 多面体边线与线宽（2026-11-09）

R107继续Properties/Polyhedra，复用R66原版5144字节真实SiO6工程SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56；七原子/六条键/一个多面体为本轮原生就绪前提，未以其存在替代新科学几何验收。两份工作副本与短独立偏好，各先绑定原生PID/窗口/7/6/1及lsof所属监听，再发baseline IPC；两个基线整字节等源。两组动作、整文件变化预期、Cancel预测均先于GUI登记。

native函数目录DlgPolyPage共15项。OnChkShowLine main0x10005fb8c（272字节/68指令）读checkbox控件到dialog+0x464，在0x10005fc44存Scene+0x2a8、fc4c存线宽float+0x2ac；UpdateCanvas0x1000610f4（540字节/135指令）在611f0/611f8存相同字段。GetPolyParam0x100060e80（572字节/143指令）从Scene+2a8/+2ac回读到dialog+464/+474。三个已知范围均逐4字节连续包含尾部，保留builtin indirect_jumps_unresolved等局限；虚拟调用/尾跳并非已动态追踪。Scene内XStyle始于+0x228，故边线int+0x80、线宽float+0x84。复用R97 XStylewrite_style0xb89ac：b8c10读+80、b8bfc读+84，格式500e8c为“ %3d %i %6.3f %3i %3i %3i\n”，前两个实参为opacity与edge，第三float为width；POLYS的+7c为另一整体样式字段，不能混淆。复用R103 CopyPropertiesTo在ba18c/190复制+7c..83（包含edge+80），ba194/198复制width+84，为Cancel预期的静态依据。

边线组原生初始Show polyhedral edges=1/width1.0，关闭后原生edge0，CUA线宽与三个RGB输入均禁用；新文件仅POLYP第二项1→0，SHAcec92f4931944c92e055f8b0ee50b39022f352e6ef19391eb8ed55072b5dcb78，其余工程字节保持。手动开启后整源恢复，控件重新可编辑。再次关边线、原生确认edge0后Cancel，新输出整源恢复，不能把这个通过推广到R103边线形状、R104mark或R105键规则失败字段。

独立线宽组实际Line width输入1.0→2.0，Tab失焦后CUA和原生均显示2.0；新文件仅POLYP第三项1.000→2.000，SHA92b0f5529395072226bc9844593b7f0951afa60a389bd8349572f4a8ca7b70b6。Cancel后又精确等于整份5144字节规范源。两个真正改动状态的Cancel控制通过，DISPF/POLYS/SBOND及其余字节均不变。没有依靠关闭后AX重查启动空白实例，也未按disabled控件强行操作；实际屏幕边线/GL线宽像素、透明度颜色、不同中心原子、全状态回滚仍待。

拥有PID81001/88127均正常0，118.728/80.457秒，120与180目标均满足；所有功能nativeIPC/读取在180窗前，退出后原PID守卫直接跳过native/CUA查询。两个输入的哈希/mtime在0.1秒抽样和退出前后保持，规范源及四个所属偏好常规文件保持/归档。

49项限定核对通过，7个阶段文件/7次原生UI；新增18原生ID至1241，raw1100/not_loaded141，141次按ID原生补读组成显式派生完整快照，raw不改。旧1223记录/12问题头、原件257、106历史manifest/19945文件、78稳定样本保持。扩展VF58/137/143三条、其余251不变；入口级待核115→114，254功能/32权威、计划10完成19未完成。Codex自验；dsh独立复审/S08与完整Study未完成，无产品实现。

[49项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-205300-s04-polyhedra-properties/polyhedra-properties-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-205300-s04-polyhedra-properties/polyhedra-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-205300-s04-polyhedra-properties/README.md)。


## R108 等值面属性与资源重开失败（2026-11-09）

R108先研究Isosurfaces原生路径：DlgIsosurfPage目录16项，OnTextCtrl main0x10004c408范围1304字节/326指令，constructor0x10004c928为5260/1315，InitDialogParam4e2e0为88/22，ResetScene4e3b0为180/45，均逐4字节连续覆盖已知范围，保留间接调用等限制。constructor给+3a8的checkboxID2cee，对应前后绘制；OnTextCtrl在4c4c8清bit13、4c4cc用0x2000并在4c4dc写Scene+5b0（XStyle+388，DISPF）。反色checkbox+408/ID2cf6，constructor4d440按signed Scene+2f1的负号选中；handler4c610写ff（-1）、4c914写1。复用R97写入器b8cfc从XStyle+c9符号扩展，500ed7格式“%3i %12.5E %12.5E\n”写TEX3P首项；Scene+2f1=+228+c9。CopyPropertiesTo掩码含bit13，ba1bc/ba1c0半字复制+c8包含+c9，先登记对应Cancel预期。

首次采用R98冻结current3.5.4项目baseline.vesta（3539字节/SHA9f60f6acdb72c9641e0ddbee3942c870b585479bce52b50ecb7c7c45caf31838），同时复制其IMPORT_DENSITY引用的相邻visibility-volume.cube（1519字节/SHA04a77cbbe05ff93df31612dfc25d5a38c8f69bb4d1fd853277e63aa098b13f68）。原生PID/窗口及文件哈希正确，但实际等值面0polygons/0vertices，CUA体数据控件禁用；正体数据16/16的事前就绪预测失败。readiness断言返回1，未发任何baselineIPC，未开Properties或更改控件，原front三阶段输出不存在，原inverse未启动。PID93347正常0/29.244秒后退出，输入及资源不改。原项目重开失败保留，不能以有文件或相邻Cube就宣称加载成功，也未隔离读取原因或推广为所有VESTA项目失效。

另立且先登记两个直接Cube控制；两份同名visibility-volume.cube新副本/独立短偏好，原生各16等值面多边形/16顶点、0atoms0bonds0polyhedra及lsof所属监听就绪后才发送baseline。实际3539字节序列化基线均精确等于旧R98冻结项目，输入Cube与预期序列化VESTA的哈希分别核验。两组初始原生front0/inverse0/Preview1；前后绘制开启时唯一front1，文件只DISPF37753794→37761986（bit13）；反色组唯一inverse1，文件只TEX3P首项1→-1（固定三字符字段），两on文件SHA43456ba264d9f397dcd4651447f390a49a2228eb12615d4584888355bffe2a0b及7140454e644b52fe145d90270e674d9191d6bbb65053434e0d7f1b3c0bee30e5。两个真正改动状态Cancel后整份3539字节恢复旧基线，其余字节包括引用声明不变。

两个直接Cube拥有PID93593/96562正常0/86.752、96.687秒，三拥有实例皆<120/180，无强停/退出后应用查询/空白新实例，源/工作输入哈希mtime保持，六个所属偏好常规文件归档。一次只读旧UI缓存文件名FileNotFound留存，已枚举正确缓存替代，不冒充目标异常。前述成功只是直接Cube属性控制，不覆盖原项目加载失败；实际深度混合/颜色像素、Surface coloring启用、科学重建及动态回滚调用追踪仍未完成。

56项限定核对通过，6阶段文件/7次原生UI（含一次未就绪上下文）。原originalProjectResourceReadinessPredictionPass/originalAllPlannedStagesExecuted=false；后登记selectedCubeControlPredictionsPass与两次Cancel整基线相等true。新增18原生ID至1259，raw1118/not_loaded141，经141原生按ID补读得到明确派生完整快照，raw不改。旧1241记录/12问题头、原件257、107历史manifest/20608文件、78稳定样本保持。扩展VF17/59/138/139/143五条、其余249保持，入口级待核114→112；254功能/32权威，计划10完成19未完成。Codex自验；dsh/S08与完整Study待，无产品实现。

[56项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-210600-s04-isosurface-properties/isosurface-properties-crosscheck.json) / [原资源就绪失败](../../output/vesta-study/20261108-113622/reviews/20261109-210600-s04-isosurface-properties/original-resource-readiness-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-210600-s04-isosurface-properties/README.md)。


## R109 体数据重开路径与打开方式控制（2026-11-09）

R109不重做已有函数study，复用readVESTAfile0xa185c（旧R1/4905指令）、check_format0x8d83c、IO::toAbsolutePath0x287f0、read_density_data0xbb5b4与readCUBEfile0xbeb7c五份已存在原生记录。IMPORT_DENSITY分支实际先在a38f4做格式识别、a3904转绝对路径、a391c拼路径/倍率描述符，a3934调用密度读取器。原生u8地址4fa6b3=32，拼接分隔符为空格；不可凭猜测写成换行。已有路径函数对前导/提前返回，不是原生调用追踪。

现有read_density_data返回493条指令/size1972、body_ranges不连续，仍带一个未解析间接跳转；虽然函数级coverage显示1/1完整，不能据此称整个Cube分支已覆盖。check_format在8dc04/8dc08比较little-endian65627563（cube）、8dc18转8e090返回11。密度分发bbb40减10，bbb54基PC=bbb64，bbb58读u16；本轮原生读取50fd12=113，故Cube目标bbb64+113×4=bbd28。按事前上限12指令/遇首个bl停止的计划，五次原生instruction-show取得bbd28..bbd38，设置栈参数后调用readCUBEfilebeb7c；这五个地址确实不在旧493条body中、原生报告function=None。已补到首个调用，未声称完整read_density_data或动态调用/文件访问链都已恢复，原文件和工具未修补。

随后事前登记两个有区分度、只读的恢复预测。绝对引用副本只把R98项目IMPORT_DENSITY中的相对文件名替换为拥有目录中同哈希Cube的绝对路径，其余字节精确保持；原生确认absolute-density.vesta标题/4Å晶胞，实际仍0polygons/0vertices，正16/16恢复预测false。另开空白原版实例，通过真正GUIFileOpen选取与R98整字节相同的相对引用项目和相邻Cube；原生先证空白窗口VESTA，后证manual_open-density.vesta标题/晶胞，结果同为0/0，第二个正恢复预测false。与R108同Cube直接打开16/16的既有控制对照，仅在这些成对上下文中，相对路径写法与启动argv打开路线都不是观察失败的必要条件；唯一读取失败原因仍未隔离，不能推广为所有IMPORT_DENSITY或所有VESTA项目失效。未开Properties、未编辑/保存/发送IPC，更未覆盖冻结旧失败。

两个拥有PID1595/1715均正常0，25.277/170.985秒，后者120目标超50.985秒、两者均<180；所有功能native读取结束在180前，退出后原PID守卫跳过应用查询。手动打开时typeText和paste后的GoToPath实际值丢失下划线，Return仍停在路径sheet、未打开错误文件；最终setValue原样填入后核对正确路径，才点击Open并取得指定项目原生后像。输入问题及120目标失败留存，未声称其唯一原因。两输入哈希/mtime的0.1秒抽样保持，规范源及两资源哈希/mtime保持，四个拥有偏好常规文件归档。

34项限定核对通过，两个真实只读控制/3次原生UI；bothPositiveRestorePredictionsPass=false、all120GoalPass=false，bothRegisteredControlsExecuted/all180GoalPass=true。新增10原生ID至1269，实际raw1126/not_loaded143，143次原生按ID补读组成显式派生完整快照，raw不改。旧1259记录/12问题头、原件257、108历史manifest/21293文件及78稳定样本保持。只扩展VF17，其余253条保持；入口级待核112不变，254功能/32权威，计划10完成19未完成。Codex自验；dsh/S08及完整Study未完成，无产品实现。

[34项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-212400-s05-import-density-readback/import-density-crosscheck.json) / [事前控制](../../output/vesta-study/20261108-113622/reviews/20261109-212400-s05-import-density-readback/import-density-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-212400-s05-import-density-readback/README.md)。


## R110 截面标志与绝对值关联范围（2026-11-09）

R110以直接打开的VS67 signed4×4×4 Cube（1519字节/SHA04a77cbbe05ff93df31612dfc25d5a38c8f69bb4d1fd853277e63aa098b13f68）研究Sections三个控件，未用仍失败的VESTA资源重开作正上下文。三份同名visibility-volume.cube独立副本/短偏好，每组原生PID/标题/16等值面多边形16顶点及lsof监听就绪后才发baseline，三个3539字节序列化基线均精确等于冻结R98 SHA9f60f6acdb72c9641e0ddbee3942c870b585479bce52b50ecb7c7c45caf31838，Cube输入哈希与VESTA基线哈希分别核验。

DlgSectionPage目录19项。Absolute handler main79204在7922c写dialog+48a bit0，并先调用SetSatulationLabel7b7e0(arg1)；Recursive handler7924c在79260/64写bit1；DrawContour handler79270在79284/88写bit6。UpdateCanvas7baa4在7bbb8写Scene+354半字，Scene内XStyle+228，故字段+12c。参数读取器7b6fc也从Scene354回读dialog48a。复用R97 writer b8a34读取XStyle+12c、打印SECTS第一项，b8d40读取XStyle+b8/+bc、打印SECTP最小/最大值。7bba0/7bb80分别写Scene2e0min/2e4max，对应XStyle+b8/+bc。

Absolute不能按只改flag预期：652字节/163指令SetSatulationLabel在普通分支使用数据绝对最大值6及当前百分比0/100，预计把物理下限-6改成0、上限仍6；Scene+5f0==2有另一分支，其输入谓词未单独追踪，事前明确若实际不符保留失败，不后验扩大预期。本轮实际原生/GUI绝对值单选1、物理Max6/Min0，on文件同时SECTS32→33、SECTPmin-6→0，SHAf1cb9c8a97f05c8dcc021a7950963d674a95cb99ce7909f123abf4aaa1a9a74c，整份文件仅这两处变动，耦合先验通过。

独立Recursive和DrawContour组各只有该原生checkbox为1，其余两项0/Preview1/物理range[-6,6]。on文件只SECTS32→34或96，其余字节精确保持，SHA459ff76968d4813cdba5fc0743ec8de73083d919e5c996598d983efebc53db18与04223267a517009fe35bba26683dabf62cb5701151e63d75f9b3e00845a26344。三组均在真正改动状态Cancel，三个after_cancel整字节恢复3539字节冻结基线。静态CopyPropertiesTo ba1cc/1d0半字包含+12c，ba1d4/1d8从+b8复制16字节，包含min/max；与所测恢复相符，未声称动态调用追踪或所有Properties字段都能回滚。

七个函数已知范围连续：72/18、36/9、36/9、Update1060/265、Reader216/54、SetSatLabel652/163、getminmax56/14（字节/指令）；保留Update虚拟尾跳的间接告警，不推广为动态科学算法验收。三个拥有PID7830/8226/8479正常0、95.182/84.336/82.167秒，皆<120/180，所有native功能动作在180前；退出后守卫直接跳过应用查询，三输入哈希/mtime及0.1秒抽样保持，六个所属偏好文件归档。所测只含checkbox/UI数值、序列化及选定Cancel，不含截面像素、递归颜色周期、等高线数值/科学绝对值计算、DataType2谓词唯一原因或资源读回修复。R108/R109失败保持。

71项限定核对通过、9阶段文件/9原生UI，全部事前整文件预期及三次Cancel通过。新增26原生ID至1295；raw1140/not_loaded155，经155次原生按ID补读组成明确派生完整快照，raw保持原样。旧1269记录/12问题头、原件257、109历史manifest/21897文件、78稳定样本保持。扩展VF60/140/141/142/143五条、其余249保持；入口级待核112→109，254功能/32权威、计划10完成19未完成。Codex自验；dsh/S08与完整Study未完成，无产品实现。

[71项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-214400-s04-section-properties/section-properties-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-214400-s04-section-properties/section-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-214400-s04-section-properties/README.md)。


## R111 Properties Preview 隔离与应用（2026-11-09）

R111补足 Properties Preview-off 隔离与 Preview-on 应用的单相轴标签控制。先读已有 SetPreviewMode main100021264（308字节/77指令）、CopyParam 100021398（220/55）、OnPreview 1000204a0（520/130）及轴标签 handler 100048c88（176/44），逐4字节连续覆盖已知范围，不重复同一study。SetPreviewMode关闭分支在两个Scene引用相等时调用虚拟clone(mode2)，随后CopyParam把基线参数送回活跃Scene；开启分支CopyParam(编辑Scene,活跃Scene)后把编辑引用切回活跃Scene。CopyParam从Scene+228的XStyle边界调用CopyPropertiesTo，OnPreview把各页面Scene引用指向选定Scene。此为静态控制流解释，未捕获动态指针/调用追踪。

两份独立R66规范SiO6工程副本，均5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。各组先原生PID/标题/7atoms6bonds1polyhedron、所属短Unix监听和整文件baseline门，再开General：Show compass1、Show axis labels1、Show edges1、Preview1。关闭Preview后再取消轴标签，原生对话框为Preview0/labels0，但IPC保存的活跃Scene仍整份等基线、COMPS1；两组相同。第一组直接Cancel后亦整份基线。第二组重新开启Preview，原生Preview1/labels0，on_apply文件唯一COMPS1→2，5144字节/SHA73ae84b72c4ba06f2d86d2c8faf7edc35935a5edf7f51a76286383a0223a7f1b；真正已应用改动后Cancel整份恢复基线。七个新阶段输出全部符合事前整文件预期。未据此推广所有属性的隔离/回滚，R103边、R104标记、R105键规则的保留字段失败均保持。

拥有PID15614/15762正常0退出，66.527/81.938秒，均<120/180；无追加相、Vectors、SaveDefault、强停或退出后应用查询。两输入/规范源哈希mtime及0.1秒抽样保持，四个所属偏好文件归档。50项限定核对通过，7阶段文件/7次原生UI；不含轴标签像素、动态Scene指针追踪、所有Properties字段/预览/回滚或体数据资源修复验收。

新增14原生ID至1309，raw1149/not_loaded160，经160次原生按ID补读形成显式派生完整快照；raw不修改。旧1295记录/12问题头、原件257、110历史manifest/22656文件与78样本保持。仅扩展VF55/143，其余252保持；入口级待核109不变，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08及完整Study仍待，未进入产品实现。

[50项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-220943-s04-properties-preview/preview-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-220943-s04-properties-preview/preview-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-220943-s04-properties-preview/README.md)。


## R112 晶胞线宽与红通道（2026-11-09）

R112补General晶胞线宽与颜色字段。目录原生搜索返回34项（含GeneralPage2），本轮只研究GeneralPage三个尚无旧study的handler：OnKillFocus main100048ef8为44字节/11指令，OnColour100048b38及OnSpinCtrl100048b30各8/2；都把event+28的ID送入UpdateCanvas10004a33c。复用完整constructor、GetGeneralParam、UpdateCanvas、XStyle写入器和CopyPropertiesTo，未重复相同study。constructor49320绑定dialog+474浮点线宽/ID2b61；495b8绑定dialog+488 RGB数组、ID2b5c/2b5d、3组件。UpdateCanvas4a3cc/3d0写Scene22c线宽，4a3e0/3ec/3f8写Scene230/231/232三颜色字节；Scene的XStyle基址228，分别对应+4和+8..a。库b8f28..38从这些字段读取，500f49为UCOLP、500f50为固定宽度格式；CopyPropertiesTo ba114..130复制线宽及RGB。静态映射与运行序列化相互支持，无动态callback跟踪。

两份独立R66规范SiO6副本5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。原生各证自有PID/标题/7atoms6bonds1polyhedron、短监听及整文件baseline门，General初始线宽1、RGB0/0/0、Preview1。width组仅setValue线宽2后Tab失焦，原生首四文本字段2/0/0/0，保存只UCOLP第三项1.000→2.000（SHAac1f6767380611bbfc7f9d9bf0bac7fca72c646ef7ab74044f7e9b5dbbd4aa30）；red组仅红值64后Tab失焦，原生字段1/64/0/0，保存只UCOLP第四项0→64（SHA4dd3c6c855bc488a509a6cffc66bdcbf22c84e17c6390865b685973436c142e1）。每组均从真正改动状态Cancel，after_cancel整份恢复规范基线，六个阶段文件均符合事前整文件预期；没有强制点击禁用控件、SaveDefault或更改相。

拥有PID18574/19296正常0退出，用时74.861/61.892秒，均<120/180；所有原生功能阶段在180前，退出后死PID守卫跳过应用查询，未重启空白实例。规范源/两个输入哈希mtime及0.1秒抽样保持，四个所属偏好文件归档。56项限定核对通过、6阶段文件/6原生UI。本轮只测线宽2与红值64的UI/序列化/Cancel，不包含绿色蓝色非零、范围/非法输入、颜色选择器、绘制像素、动态callback或所有属性回滚；R103/104/105保留字段反例不变。

新增16原生ID至1325，raw1152/not_loaded173，经173次原生按ID补读形成显式派生完整快照，原raw不改。旧1309记录/12问题头、原件257、111历史manifest/23367文件和78样本保持。扩展VF55/143/152三条，其余251保持；入口级待核109→108，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08和完整Study仍待，未进入产品实现。

[56项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-221913-s04-cell-line-properties/cell-line-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-221913-s04-cell-line-properties/cell-line-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-221913-s04-cell-line-properties/README.md)。


## R113 原子标签类型与偏移（2026-11-09）

R113研究Atoms标签类型和z偏移。复用R104目录、GetAtomsParam、只有35条可达指令/140字节的内置UpdateCanvas记录，以及R104原生objdump分页offset40000/limit12000；新提取2e148..2e420共182条连续指令，保持原内置分支覆盖缺口和当年objdump忽略range失败，不重跑同一study或修补工具。新study为OnLabelType main10002b594（144字节/36指令）、constructor10002b660（8432/2108）、OnKillFocus10002b634（44/11），各已知范围连续，保留间接调用/尾跳限制。

OnLabelType在2b5c4按choice是否非零设置Scene+5b0的bit22。constructor2d094为2bd0/两选项，2d4fc把bit22作为当前selection；偏移2d180绑定dialog4a8、2d190为ID2bd1。GetAtomsParam从Scene280回读dialog4a8。2bd1-2bc0=17，对应原生u8地址1004d73b9=137，故dispatch base2e194+137×4=2e3b8；原页2e3b8..3c0将偏移写Scene280，即XStyle+58。原生writer b8f78取DISPF bit22、b8f7c读取XStyle54/58、b8f88取bit23，格式500f76为LABEL %i %5g %6.3f %i。CopyPropertiesTo ba154/158复制58，mask02422800含bit22而不含bit23；这解释本轮选定标签类型/偏移可恢复与R104 markbit23保留的差别，仍非动态调用跟踪。

两份独立R66规范SiO6工程副本，5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56；原生每组证自有PID/标题/7atoms6bonds1polyhedron、短监听及整文件baseline门。Atoms标签类型初始Names of sites、offset1、Preview1。type组实际展开两选项Names of elements/Names of sites，选择第0项Names of elements；原生on为元素名称/offset1，保存仅DISPF37753794→33559490（清bit22）及LABEL首项1→0，SHA899850b2c5b59fd6754ff24796bc80e02335b6a39f52464fe931175976571c18。offset组保持Names of sites，将z偏移setValue1.5后Tab失焦，原生offset1.5，文件唯一LABEL第三项1.000→1.500，SHA74975df24e7e1efcdebf78a0882d9cfa5dae4fbe6e31fd692cb178a74e57edbe。两组均从真正改动状态Cancel，输出整份恢复规范基线，六个阶段文件均符合事前整文件预期。

两组Atoms初始Show as balls/Show as displacement ellipsoids都为0，修改标签后亦为0；这个已知异常原样记录，不强制更改radio，也不把正7/6/1模型或标签控制通过当Atoms模式正确性验收。Hide nonbonding/Mark enantiomorphic为0不动，Preview1。拥有PID21226/21371正常0退出，74.568/67.464秒，均<120/180；所有原生功能动作在180前，退出后死PID守卫跳过查询，规范源/工作输入哈希mtime及0.1秒抽样保持，四个所属偏好文件归档。61项限定核对通过，6阶段文件/6原生UI。未验标签绘制像素、动态callback、偏移边界或所有LABEL/Properties回滚；R104 markbit23及其它保留字段失败不变。

新增16原生ID至1341，raw1140/not_loaded201，经201次原生按ID补读形成显式派生完整快照，原raw不改。旧1325记录/12问题头、原件257、112历史manifest/24120文件与78样本保持。扩展VF56/143/157三条，其余251保持；入口级待核108→107，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08及完整Study仍待，无产品实现。

[61项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-222917-s04-atom-label-properties/atom-label-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-222917-s04-atom-label-properties/atom-label-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-222917-s04-atom-label-properties/README.md)。


## R114 原子 stacks 与 slices（2026-11-09）

R114研究Atoms tessellation的stacks/slices，未研究材质或Bonds控件。复用R113完整constructor、R104 GetAtomsParam/内置不完整UpdateCanvas及原生完整182指令分页、R113失焦handler、XStyle writer和CopyPropertiesTo。新原生OnSpinCtrl main10002b62c仅8字节/2指令，把event+28的ID尾跳到UpdateCanvas2e148。constructor2bab0绑定dialog47c、2bac0为ID2bc4/stacks、2bac8为下限2；2bb14绑定dialog480、2bb24为ID2bc5/slices、2bb2c下限3，栈上界均10000。这些只说明静态构造器，不当上下限运行验收。

2bc4/2bc5相对分发表基ID2bc0的索引4/5；新原生u8地址1004d73ac/ad均为0，故跳转2e194。复用原页2e198/1a0将dialog47c写Scene268，2e1a4/1ac将dialog480写Scene26c；Scene的XStyle基址228，故对应40/44。writer b8b64读取40/44，500e3b为ATOMP、500e42格式为%3i %3i %3i %3i %4.1f %3i；CopyPropertiesTo ba144/148复制40..4f，包括这两个整数。保持原内置Update缺少分支和旧objdump忽略range失败，不重跑相同study、不修补工具或声称动态调用追踪。

两份独立R66规范SiO6工程，5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。原生各证自有PID/标题/7atoms6bonds1polyhedron、短所属监听及整个基线序列化门。Atoms初始stacks24/slices24/Preview1，两个样式radio均0；本轮保持异常原样，不强制修复也不据此验收模式。stacks组只setValue32后Tab，原生32/24，文件仅ATOMP首项24→32，SHA7a42d2ea6d435a8642c1246c0fe033cac3043bda7fbfe8eafb85b1dd12c38f27；slices独立组只第二字段32后Tab，原生24/32，文件仅ATOMP第二项24→32，SHA0bd926b86141fcd4c375f562589ed64464ceca19fdd33363c51f9888d87fd9f8。每组真正改动状态Cancel后，整份5144字节恢复规范基线；六个阶段文件均符合事前整文件预期。

拥有PID22918/23312正常0退出，72.749/65.879秒，均<120/180；所有原生功能动作在180前，无Vectors/追加相/SaveDefault/强停/退出后应用查询。规范源/工作输入哈希mtime及0.1秒抽样保持，四个所属偏好文件归档。55项限定核对通过、6阶段文件/6原生UI。只验所选32值UI/序列化/Cancel，未验实际球面网格/像素、材质、Bonds参数、范围/非法值、性能/科学算法或通用属性回滚；原R103/104/105保留字段反例保持。

新增15原生ID至1356，raw1151/not_loaded205，经205次原生按ID补读形成显式派生完整快照，原raw不改。旧1341记录/12问题头、原件257、113历史manifest/24954文件和78稳定样本保持。扩展VF56/143/153三条，其余251保持；入口级待核107→106，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08与完整Study未完成，产品实现未开始。

[55项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-223928-s04-atom-tessellation/atom-tess-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-223928-s04-atom-tessellation/atom-tess-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-223928-s04-atom-tessellation/README.md)。


## R115 原子材质与百分比换算（2026-11-09）

R115研究Atoms材质红通道与shininess百分比，保持单相、无Vectors/追加相/SaveDefault或实现。新原生vxMaterialTool::Create main1000155d8为796字节/199连续指令、OnSpinChanged1000159b8为52/13，目录返回2函数；先核1356旧记录未有Create study再调用，未重复相同研究。复用Atoms constructor/reader/失焦与spinhandler、不完整内置Update和原生182指令分页、XStyle writer/copy。

Atoms constructor2b898/89c把dialog49d RGBA和4a4 rawshininess交共享MaterialTool，事件ID2bc0/2bc1。Create1562c/630拼float32位模式3fa3d70a（约1.2799999713897705），15638将raw除以该常量、转double后加0.5、fcvtms取floor生成界面整数百分比。OnSpinChanged159bc读取spin缓存double，159c0转整数，159d4按同一float32常量相乘，159dc写回raw指针。事前Python按相同float32舍入计算40%，得到固定三位字符串51.200；不把百分比数值直接写进ATOMM。Create给spin静态范围1..100，未做范围/非法值运行验收。

新原生表字节1004d73a8/a9均78，2e194+78×4=2e2cc；原页2e2d8/2e2e4写Scene3cc/3d0，即XStyle1a4/1a8。writer b91c0..cc读RGBA、b91ec读shininess，ATOMM按所选原始格式输出；CopyPropertiesTo ba30c/310和ba33c/340分别复制1a4和1a8。此为静态控制/字段说明，无动态callback跟踪。

两份独立R66规范SiO6工程副本5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，原生各证自有PID/标题/7atoms6bonds1polyhedron、短监听及整文件baseline门。Atoms材质初始Specular204/204/204、shininess20%、Preview1。red组setValue红128后Tab，原生128/204/204/20，文件仅ATOMM红204→128（SHA bd8369495e389f620fb411d8939de972b76f700fef8d6dbbb716d14b1f2ce781）；shine组独立setValue40后Tab，原生204/204/204/40，文件唯一ATOMM原始shininess25.600→51.200（SHA1b61525a82ca81468f01d04e1d87dbb2e06064d79e1b3bcebc7778978ea76dc0）。两组真正改动状态Cancel后整份恢复规范基线，六个阶段文件符合事前整文件预期，float32换算先验通过。

拥有PID25034/25438正常0退出，71.869/70.911秒，均<120/180；所有原生功能动作在180前。两个初始模式radio均0仍原样保留，不强制改动或声称模式正确。规范源/工作输入哈希mtime及0.1秒抽样保持，四个所属偏好文件归档，无退出后应用查询。59项限定核对通过、6阶段文件/6原生UI。未验其它颜色/通道/页面、范围/非整数输入、实际材质光照/像素、科学数值或通用Properties回滚；R103/104/105保留字段反例不变。

新增17原生ID至1373，raw1168/not_loaded205，经205次原生按ID补读形成显式派生完整快照，raw保持原样。旧1356记录/12问题头、原件257、114历史manifest/25796文件与78样本保持。只扩展VF56/143/153三条，其余251保持；入口级待核106不变，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08及完整Study未完成，未进入产品实现。

[59项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-224835-s04-atom-material/atom-material-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-224835-s04-atom-material/atom-material-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-224835-s04-atom-material/README.md)。


R115文档入口修订：初验时续记已写R115、头部却仍R114，根因是复用更新器保留R113匹配起点，替换未命中。原更新器及初验报告保持在本轮；新增current-header检查先实际失败退出2，再更正头部并加入主/专项检查，fresh重新验收。原生实验/输入/1356旧记录及冻结失败不改，不把初验报告当修订后通过。详见该轮header-correction.json与pre-header-correction。


## R116 Bonds 材质与规则隔离（2026-11-09）

R116以两个独立R66规范SiO6副本研究Bonds材质，先读R105 GetBondParamFromS/完整UpdateCanvas、共享R115 MaterialTool换算以及原生writer/copy；新study为Bonds constructor main10002ea84（3500字节/875连续指令）和OnTextCtrl2ea7c/OnColour2ea6c/OnSpinCtrl2ea74（各8/2），未重复相同study。constructor2ecb0/4绑定dialog414 RGBA与418 rawshininess，2ecbc/2ecc0给事件ID2c24/25，调用共享155d8。UpdateCanvas2fc00按ID减2c24、比较2，2fc08转2fd08；2fd10/1c写Scene3d4/3d8，即XStyle1ac/1b0。2c28样式分支在2fd50调用SetBondStyle，半径/线宽/颜色有其它分支，不把Atoms父层绑定直接当Bonds证据。

writer b9224..30读XStyle1ac..1af RGBA、b9248读1b0，501021为BONDM；CopyPropertiesTo ba314/318及ba344/348复制两材质字段。共享float32常量3fa3d70a约1.2799999713897705，界面floor(float(raw/constant)+0.5)，反向integerpercent*constant。事前float32预期50%输出64.000，非将50直接当raw。静态控制链说明未声称动态callback或SBOND调用追踪。

两份5144字节规范源SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，各在原生自有PID/标题/7atoms6bonds1polyhedron及所属短监听后，通过整文件baseline门。Bonds初始specular255/255/255、shininess100%、Preview1，唯一选中Bicolor cylinder。red组setValue红128后Tab，原生128/255/255/100，文件仅BONDM红255→128（SHA24ae9079bfcdb02650a0a4888c4473920cd9e9bda343193fd53b0fe2c6ef19ed）。shine独立组setValue50后Tab，原生255/255/255/50，文件唯一BONDM raw128.000→64.000（SHA9cabd3113a123f112fdfe9616835f62be975c50489f61b59228c289557f9e637）。六个阶段文件符合事前整文件预期，完整SBOND块全部保持，样式也保持Bicolor；两个真正改动状态Cancel均恢复整份规范基线。此成功不覆盖R105样式改动后SBOND保留的Cancel失败。

拥有PID31735/32425正常0退出，98.987/76.028秒，均<120/180；所有原生功能动作在180前。规范源/输入哈希mtime与0.1秒抽样保持，四个所属偏好文件归档，无Vectors/追加相/SaveDefault/强停/退出后应用查询。65项限定核对通过、6阶段文件/6原生UI。只验两所选材质控制的UI/序列化/恢复和SBOND不变，不含实际光照像素、其它通道/范围/非整数输入、通用Bonds或Properties回滚。

新增16原生ID至1389，raw1182/not_loaded207，经207次原生按ID补读形成显式派生完整快照，raw保持。旧1373记录/12问题头、原件257、115历史manifest/26658文件与78样本保持。仅扩展VF57/143/153，其余251保持，入口级待核106不变，254功能/32权威、计划10完成19未完成。头部更新从before快照核对实际旧文本后生成，并以本轮round/provenance/coverage核对，迁移旧R115当前头部guard以避免下一轮叠加过期要求。Codex自验；dsh/S08及完整Study未完成，无产品实现。

[65项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-230440-s04-bond-material/bond-material-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-230440-s04-bond-material/bond-material-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-230440-s04-bond-material/README.md)。


## R117 Bonds stacks 与 slices（2026-11-09）

R117以独立R66规范SiO6副本研究Bonds stacks/slices，直接复用R116完整constructor和Text/Spin handler、R105 reader/完整UpdateCanvas、原生BONDP writer及CopyPropertiesTo，不重复同一函数study。constructor2eb50令x20=dialog40c，2eed8给stacks事件2c26，2eedc把该指针传入spin控件，静态min1/max10000；slices在2ef30绑定dialog410，2ef40事件2c27，min3/max10000。reader从Scene284/288回读40c/410；Update2fc0c将ID减2c26、与2比较，小于2时落入2fc1c，2fc28/34写Scene284/288，即XStyle5c/60。后续所选分支只重建显示列表，样式2c28有另外的SetBondStyle分支；这是静态控制说明，无动态callback或规则更新跟踪。

原生writer b8bbc读取XStyle5c/60，500e5d是BONDP，按其前两整数写出；CopyPropertiesTo ba15c/160复制5c..6b，涵盖两字段。两份5144字节规范源SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，各在原生自有PID/标题/7atoms6bonds1polyhedron、所属短监听后通过整文件baseline门。Bonds初始stacks1/slices16、Preview1、唯一Bicolor cylinder选中。

stacks组只setValue1→2后Tab失焦，原生2/16，文件仅BONDP第一项1→2（SHA304ae948b68203cf575595586fc8208e64b304a5323d72c2a37ef2b1e9400d54）；slices独立组16→24后Tab，原生1/24，文件仅第二项16→24（SHAee3694e7b8a60a4ba2c6c38208d7660f72955f6978d6f4c2522b5b881c8a6c24）。六份阶段文件符合事前整文件预期，完整SBOND块全部保持，Bicolor样式未变；两个真正改动状态Cancel后整份恢复规范基线。此成功不替代R105样式改动后规则保留的Cancel失败。

拥有PID46915/47294正常0退出，70.527/72.120秒，均<120/180，原生功能动作均在180前。规范源/两个输入哈希mtime和0.1秒抽样保持，四个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。63项限定核对通过、6阶段文件/6原生UI。仅验所选resolution值的UI/序列化/Cancel与SBOND不变，未验实际网格、像素、性能、边界/非法输入或通用回滚。静态构造器范围不当运行范围验收。

新增12原生ID（全为6UI+6IPC，没有新static study）至1401；raw1194/not_loaded207，经207次原生按ID补读形成显式派生完整快照，原raw保持。旧1389记录/12问题头、原件257、116历史manifest/27515文件与78样本保持。仅扩展VF57/143/153，其余251保持，入口级待核106不变，254功能/32权威、计划10完成19未完成。before快照匹配后更新当前头部，验证读取本轮实际round/provenance/coverage；原R115头部失败和修订包仍保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。

[63项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-231655-s04-bond-tessellation/bond-tess-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-231655-s04-bond-tessellation/bond-tess-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-231655-s04-bond-tessellation/README.md)。


## R118 Bonds 半径与线宽（2026-11-09）

R118研究Bonds圆柱半径及线宽。复用完整constructor、reader、UpdateCanvas、Text handler、XStyle writer/CopyProperties和已有完整save_data项目保存函数。新增Scene/Crystal两个半径与两个线宽helper的原生study：Scene81294写全局29c并遍历各相调用6b3ac，Scene812f0写298并调用6b470；Crystal COW路径分别在6b3f8/6b4bc写规则+34/+38。保留未解虚拟clone/destructor及cold边，不把内建callee落入下个函数的元数据当实际调用。VESTA名writer正则零结果仅是有限负搜索，实际save_data已有证据，未重复study。

constructor2f428绑定dialog424、2f438事件2c29；2f598绑定420、2f5a8事件2c2b。Update2fd6c/2fc98调用对应Scene半径/线宽import，比较同值时跳过更新。save_data ae7e4/e8读取当前相规则向量240/238，ae82c读取规则34/38，ae850依次传给4ff628完整SBOND格式，落在第11/12个token（零基10/11）；这与只含前10项的default-style SBOND writer不同。全局writer b8ba4读取XStyle70(linewidth)/74(radius)，写BONDP第三/四项；CopyProperties ba16c/170复制两全局字段。静态路径与观察相符，无动态callbacktrace、虚拟目标解析或Cancel唯一因果证明。

两份规范SiO6副本各5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，独立短偏好目录。原生自有PID/标题/7atoms6bonds1polyhedron/所属监听后，整文件baseline门通过；Bonds初始radius0.25/width2.0、Preview1及唯一Bicolor1。半径组只setValue0.5并Tab，BONDP半径及唯一SBOND半径同时0.250→0.500；线宽组只setValue3.0并Tab，BONDP线宽及SBOND线宽同时2.000→3.000，其他字节保持。两个真正改动状态Cancel都仅恢复全局BONDP，SBOND规则仍保留编辑值。六份阶段文件全部符合事前精确预期；两份after_cancel都不等于完整规范基线，不能表述为整体回滚成功。R105旧样式Cancel失败继续保留。

67项限定核对、6阶段文件/6原生UI通过。自有PID54746/56113均正常0退出，74.559/65.646秒，均<120；原生功能动作在180前。源/输入哈希及mtime、0.1秒抽样不变，四个所属偏好文件已归档，无Vectors/追加相/SaveDefault/强停/退出后应用查询。未验RGB、实际几何/像素/性能、多规则/多相、边界非法值或通用回滚。新增18原生ID（4helper study+2search+6UI+6IPC）至1419，raw1201/not_loaded218，经218次原生补读形成显式派生完整快照；1401旧记录、12问题头、原件257、117历史manifest/28357文件及78样本保持。

只扩展VF57/143/158，其余251记录保持；VF158为所选半径/线宽partial，RGB及更广语义待核，精确入口级待核106→105并不代表全部未完成功能。254功能/32权威文档、计划10完成19未完成。当前头部以before快照匹配并由实际provenance/coverage验证；R115旧头部失败/修订包保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。

[67项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-232635-s04-bond-radius-width/bond-size-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-232635-s04-bond-radius-width/bond-size-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-232635-s04-bond-radius-width/README.md)。


## R119 Bonds RGB三通道（2026-11-09）

R119独立验证Bonds的R/G/B三通道127→64。复用既有完整constructor、reader、UpdateCanvas、Text/Colour handler、XStyle writer/copy以及项目save_data；旧原生search已定位Scene81234及Crystal6b2e8，因此没有重复search。新增两个helper完整已知范围study：Scene96字节24指令，在81234/38把输入32位值写入全局2a0并在8126c遍历相调用Crystal；Crystal196字节49指令，COW路径6b330/334把输入32位写入规则3c。第四拷贝字节的含义、虚拟clone/destructor及cold目标仍未解决，不能扩张为alpha契约或动态callbacktrace。

constructor2f694绑定dialog428、2f69c/6a0注册2c2c/2d；Update2fcb8/fcc8/fcd8写Scene2a0/1/2，比较RGB后在2fcf4调用Scene import。同值分支不调用该helper；这是静态路径，不证明本次具体回调或Cancel唯一因果。项目writer ae838/3c/40读取规则3c/3d/3e，按ae848/4c参数排布写SBOND零基token12/13/14；全局BONDP writer b8bb0/b4/b8读取XStyle78/79/7a，CopyProperties ba174..188包含三全局字节。

三个独立规范SiO6输入各5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，一相一规则，原生自有PID/标题/7atoms6bonds1polyhedron/所属监听及整文件baseline门均先通过。每组只用setValue+Tab把一个通道127改64；原生UI保持另外两通道127及Bicolor1，保存文件仅相应BONDP和SBOND两处通道改变。三个真正改动状态Cancel均恢复全局RGB127/127/127，却保留规则的相应64；三份after_cancel整体基线相等均false，九份阶段文件符合事前整文件预期。R118半径/线宽和R105样式的旧保留规则反例保持。

91项限定核对、9阶段文件/9原生UI通过。PID59511/62448/62733正常0退出，72.252/69.671/69.951秒，均<120，原生功能动作均在180前。源及三个输入hash/mtime、0.1秒抽样保持，六个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。仅验三通道所选值与UI/持久化，颜色选择器、实际渲染、第四字节、多规则/多相、非法边界、通用回滚仍待核。

新增20原生ID（2helper study+9UI+9IPC），完整核验派生快照1439；原生raw1220/not_loaded219，按ID补读219次形成显式并集，raw和canonical均未重写。1419旧记录、12问题头、257原件、118历史manifest/29248文件及78样本保持。仅扩展VF57/143/158，其余251保持；VF158所选半径/线宽/RGB partial，精确入口级待核105不变，不能称所有功能已验。254功能/32权威，计划10完成19未完成；before匹配头部并按实际provenance/coverage复核，R115旧失败保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。

[91项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-234135-s04-bond-rgb/bond-rgb-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-234135-s04-bond-rgb/bond-rgb-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-234135-s04-bond-rgb/README.md)。


## R120 Polyhedra 边线RGB（2026-11-09→10）

R120独立验证Polyhedra边线R/G/B三通道180→64。先查1439个既有native记录，确认所选constructor/Colour/Spin/Killfocus四个函数尚无native_function_study，再补读完整已知范围3572/893、8/2、8/2、44/11（字节/指令）。复用R107完整UpdateCanvas540/135、reader572/143和已有XStyle writer/CopyProperties。constructor609d0绑定dialog470，609d8/9dc事件2c8e/2c90，609e0设3通道；Colour/Spin尾调用6110f4，Killfocus在5fd30调用同一Update。

Update61124将ID减2c8c后构造bit，61144的mask0x14选择2c8e/2c90，61150..170把dialog470/471/472写Scene2b0/2b1/2b2，即XStyle88/89/8a。reader61040/44从Scene2b0回读dialog470。writer b8c04/08/0c读XStyle88/89/8a，经格式500e8c写POLYP零基token3/4/5；CopyProperties ba19c..1b0复制三个全局字节。所选分支没有调用Bonds的规则更新helper；这是静态路径，不能声称动态callback、写入actor或唯一回滚因果已跟踪。既有virtualcall/tail及builtin警示保留。

三份规范一相SiO6输入各5144字节，SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。每组原生自有PID/标题/7atoms6bonds1polyhedron/所属短监听和整文件baseline门均在getApp前通过。Polyhedra初始Showedges1/Preview1/width1.0，边线180/180/180；每组仅setValue+Tab把一个边线通道180改64。原生控件与阶段文件匹配，仅POLYP相应通道变化，完整SITET及其他字节保持；三个真正改动状态Cancel均恢复整份5144字节规范基线。九份事前整文件预期通过，不替代R105/R118/R119 Bonds规则保留等旧失败。

82项限定核对、9阶段文件/9原生UI通过；PID67039/67297/67686正常0退出，76.781/76.980/75.141秒，均<120，原生功能动作在180前。源及三个输入hash/mtime、0.1秒抽样保持，六个所属偏好文件归档，无Vectors/追加相/SaveDefault/强停/退出后应用查询。面RGBA、透明度、颜色选择器、实际渲染、边界非法输入/多相或通用回滚仍待核。新增22原生ID（4函数study+9UI+9IPC），完整派生1461；raw1220/not_loaded241，241次原生按ID补读构成显式完整并集。1439旧记录/12问题头、257原件、119历史manifest/30182文件、78样本保持。

仅扩展VF58/143/159，其余251保持；VF159为所选边线RGB partial，并补链R107已验edge/width证据，opacity/faceRGBA仍待核。精确入口级待核105→104不等于总未完成功能减少到104。254功能/32权威，计划10完成19未完成。头部以before匹配并按实际provenance/coverage核验，R115旧失败保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。

[82项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-235637-s04-polyhedra-edge-rgb/poly-edge-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-235637-s04-polyhedra-edge-rgb/poly-edge-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-235637-s04-polyhedra-edge-rgb/README.md)。


## R121 Polyhedra 全局透明度与面alpha（2026-11-10）

R121分别研究Polyhedra全局opacity与所选Si面alpha204→128。四个新增native study为Scene80ef4（244/61）、Scene80df4（256/64）、Crystal6acd4（184/46）、Crystal6ac00（212/53），均已知范围完整连续；复用R120 constructor/handlers、R107完整Update/reader、原生project save_data、CopyProperties、CopyParam及refcountedvectorassignment21474。没有重复旧study/search。保留builtin已知函数边界外错误fallthrough callee条目、虚拟clone/destructor/cold目标未解，不把它们当实际执行。

原生全局路径：constructor60070 dialog468/60080事件2c8b，Update611a8调用Scene80ef4；80f0c写Scene2b3，80f40遍历Scene5c0元素alpha8f，80fbc调用Crystal6acd4，6ad18写全部Crystal1c8位点alphaef。所选面路径：constructor605c8 dialog46c/2c8f/91，Update先在6121c/230/244写所选元素RGB8c/d/e，alpha变化时6129c直接写8f，再经612b8调用Scene80df4及Crystal6ac00；后者以位点d4匹配元素，6ac50写位点ec..ef四字节。

原始事前整文件预期漏记ATOMT联动。冻结原计划、expected文件、分析器及poly-opacity-crosscheck.json：69项检查中4项false，6整文件预期仅3通过/3失败，实际重算exit1保留。3个失败整文件分别是global/on、alpha/on、alpha/after_cancel；另外bothCancelwholebaseline断言失败。全部baseline、源输入及native调用正确，并非运行调用失败。

观察后另建显式characterization文件及独立分析器/79项核对，不能把它称事前预测。Global204→128实际只改POLYP首项、全部7个SITET alpha和2个ATOMT alpha；Cancel整份5144规范基线精确恢复。独立Si facealpha204→128实际只改Si1 SITET alpha及Si ATOMT alpha，O两层alpha与全局POLYP204保持；Cancel后所有SITET恢复204，却留下ATOMT Si alpha128。选中Si的原生UI始终正确，global组编辑时界面global/facealpha128/128，alpha组为204/128。

原生项目writer同时区分两层：ae908..914读取Crystal1c8位点ec..ef写SITET token6..9；af448/44c遍历Scene5c0，af484..490读取元素8c..8f写ATOMT token6..9，格式4ff85b。CopyParam213c4复制Scene5c0、213f8复制各Crystal1c8，经21474拷贝指针并增加引用计数；全局XStyle8b由CopyPropertiesba1b4/1b8复制。所选Update在Scene COW helper前直接写元素，符合共享快照提前变化的候选解释；实际指针别名、动态callback及唯一Cancel因果仍未证明。未修改原版或修复失败输出，也不据全局成功推翻选中alpha反例。

79项观察后限定核对、6阶段文件/6原生UI完成；原始false报告及exit1通过sha/mtime冻结保护。自有PID71101/71647正常0退出，74.390/77.164秒，均<120；所有原生功能动作在180前。源及两个输入hash/mtime、0.1秒抽样保持，四个所属偏好文件归档，无Vectors/追加相/SaveDefault/强停/退出后应用查询。仅一相SiO6所选值；faceRGB、颜色选择器、实际透明度像素、边界非法输入、多相或通用回滚仍待核。

新增16原生ID（4helper study+6UI+6IPC），完整派生1477，raw1229/not_loaded248，248次原生按ID补读成显式并集。1461旧记录/12问题头、257原件、120历史manifest/31184文件、78样本保持；仅扩展VF58/143/159，其余251保持，精确入口级待核104不变，不等于所有未完成功能。254功能/32权威、计划10完成19未完成；before匹配当前头部/实际provenance与coverage复核，R115旧失败和所有旧反例保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。


[79项观察后限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-000803-s04-polyhedra-opacity/poly-opacity-scoped-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-000803-s04-polyhedra-opacity/poly-opacity-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-000803-s04-polyhedra-opacity/README.md)。


## R122 Polyhedra 面RGB与ATOMT残留（2026-11-10）

R122将R121所选面alpha反例的线索，转为操作前登记的三个独立面RGB检验。全部native core直接复用：R120 constructor/handlers、R107完整Update/reader、R121 Scene/Crystal颜色helper、项目save_data、CopyProperties/CopyParam/refcountedvectorassignment；15份复用源逐份hash/extent核对，没有新增或重复static study/search。

三组一相SiO6规范输入各5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。每组nativeownedPID/title7atoms6bonds1polyhedron/所属短监听及整文件baseline门，均在getApp前通过。中心原子Si、faceRGBA27/59/250/204、全局opacity204、Preview1。分别setValue+Tab改红27→64、绿59→64、蓝250→64；原生UI对应通道变化，另外两个通道及alpha/globalopacity保持。

三组on文件仅所选Si1的SITET面RGB与Si的ATOMT面RGB同一通道改变；O及其他字节保持。三个真正改动状态Cancel均恢复完整SITET，却留下Si ATOMT的相应通道64，整份规范基线相等均false，恰是本轮事前登记预期。九份整文件/SITET/ATOMT预期全部符合。R121原始六份整文件预期仅3通过/3失败、69项原始报告false和实际exit1继续冻结，另建观察后79项差异核对的边界保持，不能把R122事前成功回写为R121旧预测成功。

复用静态路径：constructor605c8绑定dialog46c、事件2c8f/2c91；Update6121c/230/244直接写所选元素8c/d/e后，再经612b8调用Scene80df4；Scene80e7c写元素8c四字节，Crystal6ac50按元素匹配写位点ec四字节。项目writer ae908..914写SITET，af484..490写ATOMT两层；CopyParam/vectorassignment覆盖Scene5c0和Crystal1c8。所选元素在COW前被直接写入与观察到的ATOMT残留一致；实际指针别名/动态callback/唯一Cancel因果仍未证明。三个新RGB独立上下文验证了保留值模式，并非完整渲染或全软件回滚验收。

95项限定核对、9阶段文件/9原生UI通过；自有PID79535/83043/85600正常0退出，90.559/85.168/68.964秒，均<120；原生功能动作均在180前。源及三个输入hash/mtime、0.1秒抽样保持，六个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。中心元素切换、多相/多中心、颜色选择器、实际像素、边界非法值和通用回滚仍待核。

新增18原生ID全部为9UI+9IPC，完整核验派生1495；raw1247/not_loaded248，248次原生按ID补读成显式并集。1477旧记录/12问题头、257原件、121历史manifest/32175文件及78稳定样本保持；只扩展VF58/143/159，其余251保持，入口级待核104不变，多个partial仍未完成。254功能/32权威、计划10完成19未完成；before匹配头部并由实际provenance/coverage复核，R115/R121及全部旧失败保留。Codex自验；dsh/S08和完整Study未完成，未进入产品实现。


[95项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-002146-s04-polyhedra-face-rgb/poly-face-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-002146-s04-polyhedra-face-rgb/poly-face-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-002146-s04-polyhedra-face-rgb/README.md)。


## R123 Polyhedra 材质红与shininess（2026-11-10）

R123独立研究Polyhedra材质红255→128及shininess100%→50%。十份native core全为复用：R120完整constructor/Colour/Spin/Killfocus、R107完整Update/reader、已有XStyle writer/CopyProperties、R115共享MaterialTool.Create155d8（796字节199指令）及OnSpinChanged159b8（52/13）；没有重复native function study/search。

constructor5ff28/2c绑定dialog478/47c，5ff34/38事件2c89/2c8a，5ff3c调用共享MaterialTool。Update61178将ID减2c89、与2比较，落入611cc后611d4写Scene3dc、611e0写3e0，即XStyle1b4/1b8。原生POLYM writer b9278..84读取1b4..1b7 RGBA，b929c读取1b8 rawshininess；CopyProperties ba31c/320复制1b4，ba34c/350复制1b8。该分支与全局opacity及所选faceRGBA分支分离，静态路径不当动态callback或实际渲染验收。

共享转换按float32常量0x3fa3d70a（约1.28），Create先raw/constant再floor(+0.5)显示百分数；OnSpinChanged把整数百分数乘相同float32写raw。GUI前独立记录原生float32预期，50%得到raw64.000，原raw128.000对应100%。两份一相SiO6规范输入5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56；各nativeownedPID/title7atoms6bonds1polyhedron/所属短监听及整文件baseline门，均在getApp前通过。初始材质255/255/255/100%，Preview1。

红组只setValue128+Tab，原生UI红128/其他255/255、高光100%，文件仅POLYM红变128。独立shininess组setValue50，CUA批次报frontmostApplicationChanged；fresh AX显示控件185已为50且聚焦，故仅补Tab，fresh AX选中204后继续nativeon/IPC，不重复setValue。该错误具体子调用未独立跟踪，保留cua-interruption-observation.json，不删去中断。nativeUI高光50%，文件仅POLYMraw128.000→64.000。两组真正改动状态Cancel都恢复整份5144字节规范基线；六份事前整文件预期符合，全部SITET及其他字节保持。不能用本轮材质成功改写R121/R122面RGBA的ATOMT保留失败。

64项限定核对、6阶段文件/6原生UI通过。PID95619/96208正常0退出，79.893/94.467秒，均<120，原生功能动作在180前。源及两个输入hash/mtime、0.1秒抽样保持，四个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。仅所选材质值/UI/编码/Cancel；材质像素、lighting/physics、G/B其他输入、颜色选择器、边界非法值、多相及通用回滚仍待核。

新增12原生ID全部为6UI+6IPC，完整核验派生1507，raw1252/not_loaded255，255次原生按ID补读成显式并集；1495旧记录/12问题头、257原件、122历史manifest/33190文件、78样本保持。仅扩展VF58/143/153，其余251保持，精确入口级待核104不变（另有多个partial）。254功能/32权威、计划10完成19未完成。before匹配头部并按实际provenance/coverage复核；R115头部失败、R121原始false69项/实际exit1及观察后79项边界、R122保留值等旧失败保持。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。


[64项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-003609-s04-polyhedra-material/poly-material-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-003609-s04-polyhedra-material/poly-material-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-003609-s04-polyhedra-material/README.md)。


## R124 General 晶胞线绿色与蓝色（2026-11-10）

R124独立补核General晶胞线颜色的G/B通道0→64。八份native core全部复用R112：General constructor4104字节1026指令、reader80/20、完整UpdateCanvas428/107、原生UCOLP writer/CopyProperties及三个killfocus/colour/spin handler44/11、8/2、8/2。完整已知范围逐份hash与连续指令核对，没有重复native function study/search；既有builtin unresolved_indirect_jumps=1与virtualtail限制保留，不把静态路径当动态callbacktrace。

constructor495b8绑定dialog488，495c0/5c4注册事件2b5c/2b5d，495c8指定3通道。reader4a2e4从Scene230回读RGB；killfocus48f0c、colour48b3c、spin48b34转入Update4a33c；Update4a3e0/3ec/3f8分别写Scene230/231/232，即XStyle8/9/a。原生writer b8f30/34/38读8/9/a写UCOLP零基token3/4/5；CopyProperties ba11c..130复制三字节。本轮只改绿/蓝，不涉及General其他分支、颜色选择器或SaveDefault。

两份独立一相SiO6规范输入各5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。各nativeownedPID/title7atoms6bonds1polyhedron/所属短监听及整文件baseline门，均在getApp前通过；General初始Singleunitcell/Solidlines、width1/RGB0/0/0，Preview1/Showcompass1/Showaxislabels1/Showedges1。绿组只setValue64+Tab改变中间RGB，蓝组独立改最后RGB，native UI保持线宽1及其他颜色0。

两组on保存文件仅UCOLP绿或蓝项0→64，完整其他字节保持。两个真正改动状态Cancel均恢复整份5144字节规范基线，六份事前整文件预期全部符合。与R112已验width1→2、red0→64合起来，仅覆盖当前输入的所选线宽和RGB值，不把它扩成任意范围、实际线条像素或通用回滚；R103FORMP/R104mark/SBOND/ATOMT保留值等旧失败继续保持。

60项限定核对、6阶段文件/6原生UI通过；PID5129/5606正常0退出，75.715/74.194秒，均<120，原生功能动作均在180前。源及两个输入hash/mtime、0.1秒抽样保持，四个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。本轮没有前台切换错误；R123原中断记录及其恢复证据仍在历史包内保留。

新增12原生ID全部为6UI+6IPC，完整核验派生1519；raw1260/not_loaded259，259次原生按ID补读成显式并集。1507旧记录/12问题头、257原件、123历史manifest/34174文件及78样本保持；只扩展VF55/143/152，其余251保持，入口级待核104不变，另有多个partial。254功能/32权威、计划10完成19未完成。before匹配头部并按实际provenance/coverage复核，R115头部失败/R121原始false69项及exit1/观察后79项边界等全部旧失败保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。


[60项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-004705-s04-cell-gb/cell-gb-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-004705-s04-cell-gb/cell-gb-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-004705-s04-cell-gb/README.md)。


## R125 所选Si原子半径与红色（2026-11-10）

R125独立验证所选Si的Atoms半径1.18→1.5及原子红色27→64。事前固定六份完整5144字节文件预测，均符合：on仅对应Si1 SITET与Si ATOMT的半径或第一个RGB红色项改变；真正改动状态Cancel恢复SITET，却保留ATOMT改动，整份规范基线相等为false，符合事前预测。第二组RGB（多面体面色）及所有O/其他字节保持。此结论限定当前一相SiO6和两个选定正值，绿蓝/范围/颜色选择器/多相/实际像素仍待核，旧保留字段失败不被覆盖。

原生Atoms constructor8432字节2108指令：dialog498/event2bca为半径，490/event2bcb/2bcc为3通道原子RGB；reader684/171从element1c、88..8a回读。复用原生paged objdump的182连续指令Update范围2e148..2e420，原builtin截断输出仍保留。新增原生数据word4d73b2的前三字节56/26/26，按base2e194导向半径2e2ec/颜色2e22c。2e300直接写element1c后调用Scene::UpdateElementRadius；2e240/258/270直接写element88/89/8a后调用Scene::UpdateElementColor，phase=-1。没有动态callbacktrace。

新增完整Scene半径helper80bf8/252字节63指令、颜色80cf4/256/64及Crystal半径6aa60/204/51、颜色6ab2c/212/53。Scene路径具有clone/refcount，写element1c或88，负phase遍历Crystal；Crystal匹配site d4所选element，写site34或e8。原生project save_data10176/2544将这些值分别写SITET与ATOMT零基token2..5，faceRGBA token6..9另列。既有CopyParam/向量赋值各220/55，复制Scene5c0与Crystal1c8的引用计数指针。提前写入与共享快照污染在静态上相容，但实际动态指针别名、唯一Cancel成因、32bitcolor第四字节语义及范围外fallthrough元数据都没有被当作已证事实。

两个正式实验PID22463/27549正常0退出，65.791/59.562秒，均<120；nativeownedPID/title7atoms6bonds1polyhedron/所属监听与整文件baseline在getApp前通过。源和两个工作输入SHA/mtime及0.1秒抽样保持，四个所属偏好文件归档；初始两种Atoms模式radio均0，未强制修正或操作disabled控件。红组启动前发现radius/red短名碰撞，仅将红组preferences路径改为独立新目录；原计划、修改收据、hash及改动范围完整保留，功能预测不变。

本轮生命周期检查有真实失败：红组Quit后的getAXState返回空VESTA，正式PID已退出而另有PID28229于同秒启动。原71项报告仅“no remaining target”为false，实际--verify exit1及原分析器/hash/mtime冻结保留。随后新增一份PID绑定原生UI确认空实例、无研究文件，再经CUA仅Quit且不跟随界面读取；进程扫描确认消失，无强停。精确内部启动调用和额外实例的偏好环境/退出码未追踪，不把清理后通过说成原报告通过。由此后续Quit后必须先走原生存活检查，不能把Quit与getAXState批在同一调用；只有原生证实同一仍存活PID及所属保存提示时才允许后续CUA。

清理后限定报告72项通过，6阶段文件/6正式原生UI，另1份仅清理用原生UI。新增18原生ID=4function+1data+7UI+6IPC；完整核验派生1537，raw1261/not_loaded276，通过276次原生按ID补读形成显式并集，未重写canonical证据。1519旧记录/12问题头、257原件、124历史manifest/35168文件及78样本保持；只扩展VF56/143/154，其余251保持，入口级待核104→103（另有多个partial，不是全部未完成数）。254功能/32权威，计划10完成19未完成；R115头部失败及R121原始false/exit1等旧失败保留。Codex自验；dsh/S08与完整Study未完成，未进入产品实现。


[清理后72项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-010041-s04-atom-radius-red/atom-radius-red-final-crosscheck.json) / [原始71项失败](../../output/vesta-study/20261108-113622/reviews/20261110-010041-s04-atom-radius-red/atom-radius-red-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-010041-s04-atom-radius-red/atom-radius-red-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-010041-s04-atom-radius-red/README.md)。


## R126 所选Si原子绿色与蓝色（2026-11-10）

R126独立补核所选Si的原子绿色59→64、蓝色250→64。六份事前完整5144字节文件预测全部符合：on仅对应Si1 SITET与Si ATOMT的原子RGB零基token4或5改变；真正改动状态Cancel恢复SITET，却保留ATOMT对应通道64，整份规范基线相等均为false，符合事前预测。半径1.18、红色27、第二组多面体面色RGBA、所有O与其他字节保持。结合R125半径/红色，仅覆盖此一相SiO6所选正值的半径/RGB/UI/编码及Cancel行为，不覆盖任意输入、元素、像素、颜色选择器或通用回滚。

十四份原生静态来源全部复用：完整Atoms constructor8432字节2108指令、reader684/171、原生paged Update相关182连续指令（原builtin截断仍保留）、Scene/Crystal半径与颜色helper、dispatchword、project save_data10176/2544、CopyParam与向量赋值及handler。dialog490/事件2bcb/2bcc经word前三字节56/26/26中颜色项26转到2e22c；Update2e258/270写element89/8a后调用Scene80cf4，phase=-1，经Crystal6ab2c匹配所选element写site e8起32bit颜色。writer把element89/8a与sitee9/ea分别写ATOMT/SITET的原子绿色/蓝色，faceRGBA字段另列。引用计数/提前写入是静态候选，动态pointeralias/callbacktrace、唯一Cancel成因和32bitcolor第四字节语义仍未证明；没有新增重复functionstudy/search。

两份独立输入使用不同全新短preferences；各nativeownedPID/title7atoms6bonds1polyhedron/所属监听与完整baseline门在getApp前通过，所选Si/Preview1、Atoms两种mode radio初始均0并保持。绿色只setValue64+Tab改208，蓝色独立改212，native UI与IPC文件互核。PID48169/51267正常0退出，61.217/64.205秒，均<120，功能动作均在180前；源与两个工作输入SHA/mtime及0.1秒抽样保持，四个所属偏好文件归档，无强停。

本轮Quit均单独调用，之后先原生存活检查；deadPID不再getApp/AX，观察到无剩余VESTA，未出现额外空实例。R125原71项noRemainingTarget false/实际exit1/原分析器/hash/mtime及另立72项清理后通过报告全部保留；本轮成功不能覆盖前轮失败，也不证明任何环境下都不会重启。R121原始false预测/实际exit1及观察后characterization边界继续保留。

71项限定核对、6阶段文件/6原生UI通过。新增12原生ID全部为6UI+6IPC，完整核验派生1549；raw1271/not_loaded278，通过278次原生按ID补读形成显式并集，未重写canonical旧记录。1537旧记录/12问题头、257原件、125历史manifest/36247文件及78样本保持；只扩VF56/143/154，其余251保持，入口级待核103不变，另有多个partial。254功能/32权威，计划10完成19未完成；before匹配更新当前头部，R115头部失败及所有旧反例继续保留。Codex自验；dsh/S08和完整Study未完成，未进入产品实现。


[71项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-011949-s04-atom-gb/atom-gb-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-011949-s04-atom-gb/atom-gb-before-functional-targets.json) / [前轮失败保留](../../output/vesta-study/20261108-113622/reviews/20261110-011949-s04-atom-gb/reused-r125-lifecycle-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-011949-s04-atom-gb/README.md)。


## R127 原子半径类型（2026-11-10）

R127验证Atoms Radii type的Atomic→Ionic和Atomic→van der Waals。六份事前5144字节整文件预测全部符合：Ionic把Si/O半径置0.2600/1.4000，van der Waals置2.1000/1.5200；每组更新全部7条SITET及2条ATOMT半径，并仅将ATOMS首项0→1/2，颜色及所有其他字节保持。两次真正改动状态Cancel均恢复整份5144字节规范基线，分别重新打开Properties/Atoms均读到Atomic/Si半径1.18。最初所选Si并不限制此选择器只改Si，实际文件包含两种元素的全部9个半径项。这里仅验证当前一相SiO6，不接受所有元素/离子电荷配位/科学半径正确性/实际像素/多相或通用回滚。

复用完整constructor8432/2108、reader684/171及既有Atoms handler catalog：choice2bcd/dialog340、3选项/初始dialog478=Scene5b8。新增完整OnRadiiType2b2ac464字节116指令，index0/1/2分别取ElementData10/18/14，遍历Scene5c0全部元素，直接调用Scene::UpdateElementRadius80bf8，phase=-1，再Crystal6aa60匹配site d4写site34；选择器没有手工半径分支那种先直接改element1c再调用helper的写入顺序。既有project save_data10176/2544把element1c/site34分别写ATOMT/SITET半径。XStyle writer2588/647的b8a80读390（Scene5b8）写ATOMS首项；CopyProperties728/182的ba13c/140明确复制390，CopyParam220/55还复制elements/sites向量。此复制字段和序列化映射是在登记预期前核实，未把“类型不复制”猜测登记为先验；也不把静态顺序差异当动态pointeralias或唯一Cancel成因证明。

新增Element::GetData1270完整88/22，按Object+c的Z匹配52d680向量；新增loadDefPar de0完整676/169，读取elements.ini的8个字段，前三浮点分别进入10/14/18。原版packaged elements.ini原字节复制并保留SHA e77dd5dafe975347cf587d553bd26067961f9b954efd08e6bc33745b39104d99及mtime，Si行1.18/2.10/0.260、O行0.74/1.52/1.400与运行值吻合。原版MachO的arm64 slice/loadcommands/对应地址原字节另作明确derived UTF32数据提取，证实三标签Atomic/Ionic/van der Waals顺序；这不是新增native evidence，也没有捕获运行时打开该资源的调用。17份既有native source复用；两次限定函数目录查询和3份新完整函数研究不重复旧functionstudy。

两组nativeownedPID/正确标题7atoms6bonds1polyhedron/所属短listener与完整baseline均在getApp前通过；工作源/输入SHA、mtime和0.1秒抽样保持，四个所属偏好文件归档。初始Atoms两种mode radio均0并保持，未强制disabled控件。Ionic菜单点击曾返回“The user changed ...”状态提示，fresh AX仍Atomic/1.18且菜单展开后才以新索引重试，随后Ionic/0.26原生UI及文件证实；提示原文/恢复观察保留，不归因于某个人或某个未instrument子调用。该组PID77385正常0/119.665秒，达到<120目标但余量仅0.335秒，不应描述为宽裕。

van der Waals组首次Quit返回frontmostApplicationChanged；原生PID81342读到仍活的正确所属主窗口、没有保存提示，原“首次Quit后deadPID不查询”检查因此false。只在此原生身份门后查询既有bound活实例（未getApp），fresh AX仍是同一主窗口，再单独Quit正常0退出；最终原生guard证实deadPID后没有应用查询，没有No/重启/强停。实际146.442秒未达120目标、仍<180。此失败Quit恢复是有记录的有限清理裁定，不能把无保存提示的原guard false改成true，也不能接受原报告hardcoded noPostQuitCUAQuery=true作为“所有Quit尝试后从未查询”的广义主张；仅接受没有post-death查询。后续短实验需为退出重试留余量。

原82项报告80true/2false（vdw under120、vdw deadPIDnoquery），实际运行及--verify均exit1；原计划/分析器/false报告/收据/guard/提示/hash/mtime全部冻结。另立14项观察后证据与恢复复核通过，明确不是替换事前预测，六份原文件预测和8份功能UI均符合，另1份仅清理原生UI。新增20原生ID=3function+2search+9UI+6IPC；完整核验派生1569，raw1273/not_loaded296，通过296次原生按ID补读显式并集，canonical旧记录不重写。1549旧记录/12问题头、257原件、126历史manifest37304文件及78样本保持；只扩VF56/143/155，其他251保持，入口级待核103→102，另有多个partial。254功能/32权威、计划10完成19未完成；R121原始false/exit1与观察后边界、R125生命周期false/exit1、R115头部失败及其他旧反例都保留。Codex自验，dsh/S08与完整Study未完成，未进入产品实现。


[原82项false报告](../../output/vesta-study/20261108-113622/reviews/20261110-013325-s04-atom-radii-type/atom-radii-type-crosscheck.json) / [原实际exit1](../../output/vesta-study/20261108-113622/reviews/20261110-013325-s04-atom-radii-type/original-verify-run.json) / [14项观察后限定复核](../../output/vesta-study/20261108-113622/reviews/20261110-013325-s04-atom-radii-type/atom-radii-type-scoped-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-013325-s04-atom-radii-type/README.md)。


## R128 正ADP椭球概率与轮廓（2026-11-10）

R128独立验证正ADP工程中的椭球概率50→75、主椭圆轮廓宽度2.0→3.0。来源是VS57非正交C/O正定各向异性U样本在R104合法切换后冻结的两个4180字节VESTA派生文件：概率组ellipsoid.vesta（mode1/principal0），轮廓组changed.vesta（mode1/principal1）。每组启动前登记完整source/hash/mtime/预期，nativeownedPID/正确标题2atoms0bonds0polyhedra/所属短listener与整文件baseline门在getApp前通过；CUA fresh AX确认已载入Showasdisplacementellipsoids1、概率可编辑，轮廓组主椭圆1/width可编辑。不新切换mode或principal、不强制disabled控件，不增加稳定样本ID。

六份事前4180字节整文件预测全部符合：概率on仅ATOMP第四项50→75，轮廓on仅第五项2.0→3.0；其余ADP、site/element半径颜色、DISPF37884866、ATOMS0/1/1及其他字节保持。两次真正改动状态Cancel均恢复各自整份冻结基线，包含原principal0/1。原CIF SHA dc52d2f42554ceb909a6d9ea129a3bfa54efa1ca708d4bb1c605b34f3d892840和两份冻结源/工作输入hash、mtime、0.1秒抽样保持；复算C1/O1的三个有理数顺序主子式均正，仅建立可用正ADP输入，不接受椭球概率区域/轴长/投影等科学或像素正确性。

完整constructor8432字节2108指令、reader684/171及fullpaged Update相关182连续指令复用，原builtin截断限制保留。概率dialog484/event2bc6、outline494/event2bc9；新增原生word4d73ae为[27,27,73,0]，对应已登记ID的首/末字节将事件导向2e200/2e194。Update2e20c写Scene270概率，2e1b8写Scene278轮廓，等于XStyle48/50。复用writer2588/647的b8b68、b8b54及参数栈顺序映射到ATOMP第四/第五项；copy728/182的q0块40..4f和d0块50..57包含两字段。Modehandler484/121与既有原生UI来源也复用，不重复study/search，不把静态绑定当动态callbacktrace。

各向同性策略仅新增静态OnChoiceScaleIso2b47c完整144/36：choice非零置Scene5b0的bit25，Preview下创建atomlist；builtin unresolved_indirect_jumps=1的virtualtail仍未解析。此策略本轮未操作、未接受运行语义，仍属VF156剩余工作。原生UI提取未提供enabled属性，实际可编辑状态来自CUA fresh AX及成功改动/文件对照，不能声称原生AXEnabled已恢复。

66项限定核对、6阶段文件/6原生UI通过。PID2099/3057正常0退出，69.684/72.493秒，均<120，功能动作均在180前；4份所属偏好文件归档。两次Quit均独立，随后原生确认deadPID才停止查询，无post-death应用查询/额外空实例/No/强停，本轮未观察到CUA状态切换错误。R127原82项80true2false/actualexit1、未达120目标、原guardfalse/被限定的hardcoded noPostQuit标志与另立14项观察后复核全部保持；本轮短上下文成功不覆盖其失败。

新增14原生ID=1function+1data+6UI+6IPC，14份既有native source复用。完整核验派生1583，raw1277/not_loaded306，306次原生按ID补读形成显式并集，canonical旧记录不重写。1569旧记录/12问题头、257原件、127历史manifest38453文件及78稳定样本保持；只扩VF56/143/156，其余251不变，入口级待核102→101，另有多个partial。254功能/32权威、计划10完成19未完成，before匹配更新当前头部。R121原始false/actualexit1及观察后边界、R125生命周期false/exit1、R115头部失败和所有旧保留字段反例继续保留。Codex自验，dsh/S08、完整Study未完成，未进入产品实现。


[66项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-015640-s04-ellipsoid-controls/ellipsoid-controls-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-015640-s04-ellipsoid-controls/ellipsoid-controls-before-functional-targets.json) / [R127原失败保留](../../output/vesta-study/20261108-113622/reviews/20261110-015640-s04-ellipsoid-controls/reused-r127-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-015640-s04-ellipsoid-controls/README.md)。


## R129 各向同性策略双向编码及保留失败（2026-11-10）

R129核对各向同性原子策略的两个方向：Scale isotropic atoms by Uiso（choice1）→Set radii of isotropic atoms as specified below（choice0），以及独立上下文的0→1。实际菜单标签/顺序与原版arm64 UTF32字节、完整constructor8432/2108的dialog348/event2bce/两choice/初始DISPFbit25一致。复用R128完整OnChoiceScaleIso2b47c144/36：取selection、非零cset、bfiScene5b0 bit25，Preview下CreateAtomList；unresolved_indirect_jumps=1的virtualtail仍未解析。复用writer2588/647 b8a74..88将bit25写为ATOMS第三项；copy728/182 mask02422800包含bit25，CopyParam/reader与既有mode证据复用，共15份native来源，无新function/search/data调用，不把静态链视作动态callbacktrace。

输入先从R104冻结合法mode1/principal0正ADP工程派生：保留C1正定张量，O1取VS55正Uiso.050661、删除其THERM行。原生headless预检wrapper0/目标255/exited、进程组verified，几何2atoms0bonds0polyhedra；实际writer补写O1六个0的THERM行，并将显示mode1重置0。原预检4项2true2false/实际exit1完整保留：C1-only THERM标签预期失败，Uiso检查器要求前导0而文件为.050661也失败。原计划/输入/检查器/报告/收据hashmtime冻结；观察后数值解析确认正Uiso，不能覆盖原false。正式GUI事前预期另行登记在此已观察规范化之后，从实际4186字节输出只派生已知R104 bit17/ATOMS第二项的mode1编码与两个初始policy。不新切换GUI mode，不强制disabled控件，不新增稳定样本；仅称名义混合ADP表示，O物理各向同性分类和科学几何未接受。

两组nativeownedPID/正确输入标题/2atoms0bonds0polyhedra/所属短listener和整文件baseline门均在getApp前通过。CUA freshAX观察ellipsoid1、principal0、概率50、outline2.0disabled、C半径.77/RGB128/73/41，policy可用。六份事前整文件预测全部符合：指定半径on只清DISPFbit25（37884866→4330434）和ATOMS第三项1→0，4186→4185字节；反方向只置这两处，4185→4186字节。ADP、site/element/颜色/其他字节不变，两次真正改动状态Cancel恢复各自整份基线。两个输入和三份父源hashmtime、0.1秒抽样保持；4份GUI偏好与1份预检偏好分别归档。

指定半径PID21414菜单出现3次CUA“The user changed...”提示；每次重读已有bound app freshAX并保留旧policy，期间键盘/两种坐标尝试均未建立成功，最终freshmenu71点击才观察到目标标签。提示/失败尝试/重试均留档，未归因用户/单一callback或唯一延迟原因。该组正常0/160.131秒，未达120目标；Uiso PID21941首次选择成功、正常0/62.598秒。两组功能调用均在180前、未强停。Quit均独立，随后nativePIDguard确认dead，不做post-deathgetApp/AX、无额外空实例/No。原正式67项66true1false/实际运行及重算exit1冻结，唯一false为specified prospective owned normal under120。另立14项观察后证据复核通过，明确不是替代事前预期，也不宣称all120通过。

新增13nativeID=6UI+7runtimeIPC（1预检+6GUI保存）；完整核验派生1596，raw1283/not_loaded313，经313次原生按ID补读显式并集，不是未修改的1596条原生export；1583旧记录/12问题头保持，不改canonical。257原件、128历史manifest39601文件、78稳定样本保持。只扩VF56/143/156，其余251不变，254功能/32权威，入口级待核101→101，另有多个partial。VF156仍partial，科学各向同性分类/策略几何/像素、参数域、动态callback、一般rollback未验。R127原82项80true2false/actualexit1+另立14项、R121/R125/R115及全部旧反例继续保留。计划10checked/19unchecked，Codex自验，独立复审仅dsh、S08/完整Study未完成，未进入实现。


[原67项66true1false](../../output/vesta-study/20261108-113622/reviews/20261110-021420-s04-isotropic-policy/isotropic-policy-crosscheck.json) / [另立14项观察后复核](../../output/vesta-study/20261108-113622/reviews/20261110-021420-s04-isotropic-policy/isotropic-policy-scoped-crosscheck.json) / [预检原始失败](../../output/vesta-study/20261108-113622/reviews/20261110-021420-s04-isotropic-policy/fixture-preparation-original-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-021420-s04-isotropic-policy/README.md)。


## R130 零张量策略静态路径及范围边界（2026-11-10）

R130以静态链澄清R129“正Uiso＋零THERM行”的程序分类，没有重跑已完成的策略GUI、没有启动VESTA目标执行。复用readVESTAfile19620字节4905行、readCIFfile12664/3166、Atomsconstructor8432/2108、reader684/171、writer2588/647和OnChoiceScaleIso144/36共6份原生来源。THERI1经reader a5160..6c使Crystal218=0，单个U值经float32→float64写site数据base+1；THERM按行写base+2..7。当前两行路径使Crystal210=1，这是Crystal级存在标志，不是逐原子零张量判定。

新增一次CreateAtomList精确搜索得到Scene79598和Crystal633d8，并新增4份functionstudy：Scene252/63、Crystal3908/977、Make_thermal_ellipse_matrix643bc2196/549、Matrix4Dconstructor6e084112/28。Scene传Scene228处XStyle，故XStyle388=Scene5b0。Crystal modebit17选择matrixmaker；maker645ec..64644逐项比较六个热参数，任一非零入64694各向异性/转换/SYEVR路径，全零才读XStyle38b bit1=DISPFbit25。R129实际C1首项.01走非零路径，O1六项0走此零张量分支。这是源绑定的程序静态分类推断，尚不是运行中站点/科学几何验收，不回改R129的原接受边界。

bit25清零：创建4×4单位矩阵并写object+a0=matrix(3,3)=0作为回退标志；bit25置位：读取base+1的Uiso，Crystal218=0选U值，其他单位路径除以程序常量78.95683520871486（8π²），取平方根传给cblas_dscal，再把M33恢复1。原版MachO字节派生验证constructor的16个double为单位阵，PROBABILITY[49]的原生float32值1.5381699800491333。CreateAtomList检查M33：0走普通站点半径；1走glMultMatrixd和probability索引50-1的缩放命令。SITET reader以%f写XSite34，当前O1站点半径.7400与ATOMT O值相同；不把元素表来源代替站点来源。

十进制参考模型：Ball-and-stick常量.4×.74≈.296，sqrt(.050661)≈.2250799858，乘原程序P50≈.3462112772。另立4项半径/float绑定检查确认实际%f来源及已知读写：按IEEE754 round-to-nearest假设，普通半径float32≈.29600000381469727，Uiso先float32→float64为.05066100135445595，sqrt再乘P50≈.3462112818691342。两组数字均为静态推演，不是现场矩阵/GL调用/像素/物理正确性证据；FPCR、BLAS数值实现、科学椭球与非正输入等未验。等列范数比较的程序阈值为1e-6，但不由此宣称最终primitive/像素几何已核对。

本轮首次37项核对34true3false、实际重算exit1冻结：检查器错把入口至末地址当作连续代码。原生body_ranges明确VESTAreader/CIFreader/matrixmaker的enclosing span分别排除2/4/4个地址；声明代码段内行数、字节数和指令覆盖逐一精确吻合，所引用5段分支另外证明连续。未重复相同native study、未把原始false改为通过。另立41项观察后复核通过，明确保留body边界及未解析calls；3函数unresolved_indirect_jumps0不代表全部虚调用解析，原OnChoiceScaleIso virtualtail1保持。另保留本轮两版派生计算helper的StopIteration/实际复现exit1和最终exit0：先是ATOMT子串提前截断，再是修正时计算仍位于收集循环内，原失败脚本和收据均归档。

新增5原生ID=1functions_search+4function_study，无新data/runtime/UI记录；完整核验派生1601，raw1263/not_loaded338，经338次native按ID补读显式并集，不是未修改的1601条原生export。1596旧记录/12问题头、257原件、129历史manifest40794文件、78稳定样本保持。当前R130未运行GUI，不宣称达到GUI120目标；R129正式67项66true1false/actualexit1、160.131秒未达120、3次CUA提示、另立14项与预检2true2false/exit1原样保留，R127/R121/R125/R115及全部旧反例保持。只扩VF56/143/156，其余251不变；254功能/32权威，entry-only待核101→101，另有多个partial。计划10checked/19unchecked，Codex自验，独立复审仅dsh，S08未开始、完整Study未完成，未进入实现。


[原37项34true3false](../../output/vesta-study/20261108-113622/reviews/20261110-023914-s04-isotropic-classification/isotropic-classification-crosscheck.json) / [另立41项观察后复核](../../output/vesta-study/20261108-113622/reviews/20261110-023914-s04-isotropic-classification/isotropic-classification-scoped-crosscheck.json) / [半径/float4项](../../output/vesta-study/20261108-113622/reviews/20261110-023914-s04-isotropic-classification/static-radius-and-float-binding.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-023914-s04-isotropic-classification/README.md)。


## R131 概率域与合法端点（2026-11-10）

R131核对位移椭球概率的配置域与合法端点。新增vxSpinCtrlC1 1635c20字节5指令、C2 161f8284/71、OnSpinCtrl1617428/7、wxgeneric DoSetValue2c8558496/124，及一次精确vxSpinCtrl搜索（12匹配）。复用16份native来源，含完整Atomsconstructor/reader/writer/copy、fullpagedUpdate182指令、R128dispatchword和R130CrystalCreateAtomList，不重复已有functionstudy。caller2bc6/dialog484把min1/max99传给C2；C2将min/max/初始*intpointer转换为double，step1传wxgenericCreate，保存int指针于398并DoSetValue。genericSetter从360/368作上下界clamp，378标志及370step非零才走条件snap；OnSpinCtrl从358的double经fcvtzs写回398指向整数，再Skip事件。此为静态setter链，不是越界/小数/非数字文本路径已动态验证。C1 assembly36c的tail明确到C2；builtin callee元数据site370在C1 body外，未据此冒充真实callsite或动态callbacktrace，generic虚调用仍未全解。

原版lib MachO符号PROBABILITY位于50ef80，当前GUI可达1..99对应index0..98，R130绘制代码是tablebase+probability*4-4。本轮事前登记独立数学模型与绝对概率容差1e-5，提取99个原生float32字节后核对：全部正、有限、严格递增，与三个独立标准高斯坐标的径向CDF积分F(r)=erf(r/√2)-√(2/π)r·exp(-r²/2)吻合，最大绝对误差2.942649164061617e-6。P1=.33886998891830444、P50=1.5381699800491333、P99=3.3682100772857666。四项独立表检查通过，但只是标量模型，不接受现场GL/科学椭球/像素。下一同section符号50f110给出400字节间距上界；99项之后4字节dc68be40非零，未把它归类为padding或有效100%项，不宣称C++数组声明长度已恢复。

两组独立上下文均复用VS57/R104冻结的mode1/principal0正定C/O ADP工程，各4180字节。所有helpers、六份整文件oracle和输入/sourcehashmtime在GUI前冻结；每组nativeownedPID/正确标题2atoms0bonds0polyhedra/所属短listener/完整baseline门在getApp前通过，CUA freshAX看到概率50可编辑、outline2.0disabled。分别仅setValue+Tab到合法1和99，不动mode/principal/isotropicpolicy、不强制disabled控件。

47项限定核对及重算actualexit0，另含事前26项静态检查和独立表4项；六份prospective4180整文件预测全部符合：两组on仅ATOMP第四项50→1/99，%3i固定宽度保持总字节，全部ADP/site/element/颜色/DISPF37884866/ATOMS0/1/1/其他字节保持。两次真正改动状态Cancel均恢复整份各自基线。PID46246/46627正常0退出，66.952/70.132秒，均<120且功能动作180前；source与两个工作输入hashmtime及0.1秒抽样保持，4份所属偏好归档。Quit独立后native确认deadPID，无post-death应用查询/额外空实例/No/强停，本轮未观察到CUA状态切换提示。nativeUI未提供enabled属性，实际可编辑/disabled来自CUA和成功变更，不冒充nativeAXEnabled。

本轮施工helper首次因else50缺空格SyntaxError，原脚本/复现actualexit1留档，修正发生在GUI前，未改事前oracle或正式分析器。R130原37项34true3false/actualverifyexit1、body_ranges enclosing排除2/4/4地址、另立41+4及两个helper失败保持；R129正式67项66true1false/actualexit1、160.131秒未达120/3次CUA提示、另立14与预检2true2false/exit1保持，R127/R121/R125/R115及全部旧反例继续保留。

新增17nativeID=4functionstudy+1search+6UI+6IPC。完整核验派生1618，raw1269/not_loaded349，349次native按ID补读显式并集，不是未修改的1618条原生export，不改canonical。1601旧记录/12问题头、257原件、130历史manifest41950文件、78稳定样本保持。只扩VF56/143/156，其余251不变，254功能/32权威，入口级待核101→101，另有多个partial。参数越界/小数/非数字文本运行、原生文件异常值域、科学几何/像素及一般rollback仍待核。计划10checked/19unchecked，Codex自验，独立复审仅dsh，S08未开始，完整Study未完成，未进入实现。


[47项端点核对](../../output/vesta-study/20261108-113622/reviews/20261110-025855-s04-probability-domain/probability-domain-crosscheck.json) / [26项事前静态检查](../../output/vesta-study/20261108-113622/reviews/20261110-025855-s04-probability-domain/static-preflight.json) / [99项概率表标量核对](../../output/vesta-study/20261108-113622/reviews/20261110-025855-s04-probability-domain/probability-table-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-025855-s04-probability-domain/README.md)。


## R132 数值越界文本提交（2026-11-10）

R132补上概率数值越界文本提交的限定证据：两个独立正ADP上下文暂存0/100，Tab后分别归限为1/99，保存前都通过nativePID绑定的postcommitgate。raw阶段只读CUA/nativeUI，不发VESTA保存客户端；没有声称该阶段Scene或GL内部值已观察。六份事前4180整文件oracle全部符合：on只ATOMP第四项50→1/99，其他ADP/site/element/颜色/DISPF37884866/ATOMS0/1/1及全部字节不变；两次真正改动状态Cancel恢复各自整份基线。

复用20份native来源，包括R131vxSpinC1/C2/OnSpinCtrl/DoSetValue与旧constructor/reader/Update/writer/概率表链，不重复已有study。新增wxgeneric精确搜索31匹配，以及5份functionstudy：OnTextLostFocus2c81e4为76字节19指令、SyncSpinToText2c8434为196/49、AdjustToFitInRange2c850c为76/19、wxSpinCtrlDoTextToValue2c8c98为84/21、DoSendEvent2c8b28为320/80。LostFocus以SendEvent1调用Sync，返回changed才调用vslot760，再Skip事件。Sync先检查text modified虚方法，vslot768解析；成功就inline比较368最大/360最小并clamp，失败回取358原值，最后进入已研究DoSetValue。另一个AdjustToFitInRange有条件wrap分支，不把它替代本次Sync的inlineclamp。

R131C2最终vptr1006040a0绑定两个新nativeu64读取：+768=100604808→1002c8c98，+760=100604800→1002c8b28。已选整数解析器调用wxStringToLong，virtualbase仍未单独追完，不宣称全部语法/溢出行为已恢复；成功将long转换double。DoSendEvent用358数值转整数构建SpinEvent，再经virtualdispatch；旧vxOnSpinCtrl把358经fcvtzs写入398链接的dialog整数，Update/writer原链复用。控件最终slot已映射，但实际eventdispatch、modified/gettext等仍有未解虚调用，不冒充动态callbacktrace或全解析callgraph。

本轮25项事前静态检查、57项限定运行核对及实际重算exit0。所有helpers、六份文件预期和source/inputhashmtime在GUI前冻结；nativeownedPID/正确标题2atoms0bonds0polyhedra/所属短listener/完整baseline门在getApp前通过。CUA freshAX显示概率50可编辑、outline2.0disabled；不切mode/principal/isotropicpolicy，不强制disabled控件。暂存前允许集合已登记为rawtyped或已归限值，实际CUA及两次native typedUI均观察到raw0/raw100；Tab后nativeonUI精确1/99，gate记录在on保存客户端之前，未直接保存越界文件或注入表索引。

PID59440/61082正常0退出，93.935/90.309秒，均<120、全部功能动作180前，保留清理余量；源/两个工作输入hashmtime及0.1秒抽样保持，4份所属偏好归档。Quit独立后nativeguard确认dead，无post-deathgetApp/AX、额外空实例、No、强停；本轮未观察到CUA状态切换提示。nativeUI无enabled属性，实际editable/disabled来自CUA和成功变更，不能声称nativeAXEnabled已恢复。

接受边界只扩到选定0/100数值越界文本+Tab提交/UI/文件/Cancel；小数、非数字、空串、溢出、其他进制、Return提交、异常文件域、瞬态Scene/GL、科学椭球/像素与一般rollback仍待核。R13147项合法端点和99可达表项/CDF标量核对保持，400字节符号间距不是C++数组声明长度，extra4字节dc68be40仍未分类。R131施工helperSyntaxError/复现exit1，R130原37项34true3false/actualverifyexit1/body_ranges排除2/4/4、另立41+4及两版helper失败，R129原67项66true1false/actualexit1/160.131秒未达120/3次CUA提示、另立14及预检2true2false/exit1，R127/R121/R125/R115及全部旧反例都保留，不回改旧接受边界。

新增22nativeID=5functionstudy+1search+2data+8UI+6IPC。完整核验派生1640，raw1288/not_loaded352，352次native按ID补读显式并集，不是未修改的1640条原生export，不改canonical。1618旧记录/12问题头、257原件、131历史manifest43251文件、78稳定样本保持。只扩VF56/143/156，其余251不变；254功能/32权威，entry-only待核101→101，另有多个partial。计划10checked/19unchecked，Codex自验，独立复审仅dsh，S08未开始、完整Study未完成，未进入实现。


[57项提交核对](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/probability-typed-crosscheck.json) / [25项事前静态检查](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/static-preflight.json) / [提交前置检查](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/below-commit-contract.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/README.md)。

## S03 工具准备核验与独立复审（2026-11-10）

S03.1模式/边界、S03.2锁定CLI、S03.3真实命令/schema/隔离恢复、S03.4历史原生往返及Check前提均由Codex核验，dsh逐项pass/无硬阻断。[矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-053708-s03-tooling-audit/S03-MATRIX.md)、[dsh原文](../../output/vesta-study/20261108-113622/reviews/20261110-053708-s03-tooling-audit/dsh-review-retry.stdout)与[收尾校验](../../output/vesta-study/20261108-113622/reviews/20261110-053708-s03-tooling-audit/delivery-final-report.json)分别记录。只补只读CLI和Check help，未重建/启动目标/执行Check或builder。

原四项勾选保留，11完成18未完成、254功能/78样本/101入口及R132派生1640=raw1288+按ID352保持；正式调查12头10resolved2investigating未改。需求新rev5及文档摘要修订已登记，旧rev4残留S02.3机器字段仅在新版本纠正。首次自验命名冲突、首次dsh缺凭据与旧清单误选false均留原回执，不能拿原失败当通过。本卡完成后停止，未自动推进S04。

## S04 剩余步骤证据归并与键盘/焦点补证（2026-11-10）

S04.2 保持未勾选；S04.3/S04.4 已完成初步行为规格/遗漏登记；本轮 dsh 独立复审 partial、无硬阻断，S04 整卡未完成。Codex核12288项/5540功能引用/3023本地链接/137历史manifest45157条目通过，最终文档/身份收尾另核；[预验报告](../../output/vesta-study/20261108-113622/reviews/20261110-055459-s04-entry-interaction-audit/codex-evidence-report.json)与[最终验收](../../output/vesta-study/20261108-113622/reviews/20261110-055459-s04-entry-interaction-audit/delivery-authority-final-report.json)。VF0017误附R108/R109迁回VF0009；旧11数值行误识别章节已从当前候选语义中纠正；所有原失败与初版/前后本保全。新GUI62231正常0/125.369秒，源/工作字节不变，Quit提示No后确认dead，无退出后应用查询。

## S06 最终交付

S01–S06 已按各卡研究范围完成，S06 经 dsh 独立复审通过；Study 22 项完成、7 项未完成。S06 接受逐领域已证规则或明确未知、独立预期、真实失败和边界，不代表全部科学算法或格式兼容通过。本轮仅完成 S06 并停止，未进入 S07、P1 或产品实现。完整 Study 和产品实现未完成。 当前 254 个稳定功能 ID、102 个稳定样本、84 条入口级待核；2036 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open；S06 新增 21 条真实原生记录和一个科学规则问题头，来源 rev10/数据库 rev4保持。

[本卡矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/S06-MATRIX.md)、[dsh实际结论](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/dsh-review-resume.stdout)、[机器裁定](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/review-result.json)、[原生索引](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/native-current-index.json)、[预期](S06_ORACLES.md)、[科学行为](行为/04-科学计算与外部程序.md)。科学不完整/原失败和明确未知继续约束P1依赖安排；本卡已停止，无产品实现。

## S07 当前交付（2026-11-10）

Codex按原卡三项直接复用19旧定位器/23包6001封存文件/264runtime回执；唯一新运行是4案TIDY子完成/失败（0/29/0/29），有效报告与坏群0明确区分。官方16命令/11外部与OS协作/10完成分支逐项记录真实结果或未验证原因。dsh本机真实运行正常退出、未超时，三项及整卡pass；原请求/响应/退出与全部准备失败均保留。研究卡通过不使MADEL/RIETAN/ORFFE/真正无图形会话/Linux x86或全部科学成功。

稳定254功能、104样本、84精确入口待核；2037逐文件哈希磁盘派生原生索引（非raw全导出）；24问题头完全保持。rev18按SHA继承rev17，需求正文归档。原包/CLI/锁定对象/旧baseline/旧证据字节保持。完成S07后停止，没有S08/P1或产品实现。

## 历史：2026-11-14 P1设计续办实际交付（R1）

2026-11-14 P1设计续办部分交付：D3／I5／C3及31组领域、codec/G6计划细化已形成，待用户手动转交dsh独立复审。P1.1已接受保持；P1.2／P1.4仍有具名公共接口及高级任务缺口，P1.3设计稿待审；原整体partial和1项接受/3项未接受不变。Study29/29保持，未进入产品实现、部署、worker、模型builder或原生Check。当前资料基线rev26，证据为20261114-100546-p1-design-continuation。

I5补canonical相定位、带基底ADP、单步网格、格式loss及宿主查询恢复；C3补锁定工具构建/API/配置/kind/迁入/launcher/checks/恢复；模块计划保留254ID唯一主责，并细化31领域和应用/偏好/数据库入口。仍有7个具体设计问题，见R1/remaining-design-issues.json。自验只核schema子集、书面协议、独立数学与资料完整性；没有产品或原件运行。下一动作是用户手动将R1/DSH-REVIEW-HANDOFF.md转交dsh；本会话不调用dsh、不自动推进P2。


历史R5主体设计续办：D7/I9/C6，资料rev30；31个有界操作、85格式路由/6类字段政策/28入口绑定、15项具体资格阻塞。P1.2/P1.4/整体partial，P1.3仅C4设计接受，C6增量待审；不进入P2–P5、产品实现或运行验证。证据与手动复审入口：[R5](../../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/README.md)。


历史R6同卡整改：2026-11-14 R6：D8/I10按dsh R5复审整改D1–D14及σ公式，Codex自验待用户手动转dsh。P1.1继承接受、P1.3 C4设计级接受保持，C6原字节及未审增量身份不变；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料rev31；不进入P2–P5、产品实现、部署或运行验证，真实worker／Check／launcher未授权、未验证。 施工回执与手动复审入口：[R6](../../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/README.md)。


当前R7勘误：2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。 [回执/复审入口](../../output/vesta-study/20261108-113622/reviews/20261114-152228-p1-design-r7-errata/README.md)。
