# VESTA Study 报告与设计输入

当前状态：2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

2026-11-10。范围为原 S08.1–S08.4；原S08.1–.4已获本机dsh明确pass，无硬阻断；3项非阻断文档发现由Codex核实澄清，最终自验/封存另列。Study研究交付29/29，P0.2/P0.3按研究范围验收；P1和产品未开始。需求依据为 [D01 §1/5/6](../VESTA全功能重写要求.md)、[Study原卡与交付判据](../superpowers/plans/2026-11-08-vesta-study.md)、[D02](../接入技术要求.md)。Codex 研究、自验和回写；独立复审统一 dsh。原 D02 末尾的旧复审称呼受[根AGENTS](../../AGENTS.md)覆盖。全部功能和 Deep 已确认，本卡不再询问范围或模式。

本报告交付可定位的研究与设计问题，不是产品架构裁定、具体代码计划或产品通过报告。功能的唯一规格仍是 [functions.json](functions.json) / [FUNCTIONS](FUNCTIONS.md)；本报告不建立第二套功能覆盖清单。254 个稳定 ID 不代表已穷尽一切潜在功能，84 条精确 `entry-observed / behavior-pending` 及其它 partial 状态原样保留。科学、格式、Web、平台、真实模型与目标环境分别判断，未运行的 Build/Check/Ready 不记通过。

## 1. 基线、证据与恢复

正式证据根 `E=/Users/agilecho/code/Craft/output/vesta-study/20261108-113622`；正式调查 `E/vesta.cany`。本卡目录为 [R：20261110-155840-s08-study-closure](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/)，其 `before/` 保存实际预编辑字节，[pre-edit-files.json](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/pre-edit-files.json)保存摘要；[progress.md](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/progress.md)是本卡施工台账。接手先读本报告、[HANDOFF](HANDOFF.md)、[BASELINE](BASELINE.md)当前头和 [METHOD](METHOD.md)已核命令，不从长历史中的旧“当前”计数恢复。

- 目标只读：`/Users/agilecho/Documents/VESTA/VESTA.app`；主程序 SHA256 `b4835068478ec1852f732c93ab25768f2b8af0166a92fc68dfd85704fe71c9e9`，libVESTA `c2c677facf6c67783cc7bbe64e92b75beb1a567fc373eacabee58b11fb81a77f`。真实版本为 3.90.9a / 64-bit / Aug 8 2026，来自 `s04/about-read-summary.json` 的 AX；66×20 About 图不能作证。Mac arm64 不是 Linux/x86 验收。
- 方法固定 `1f8508a104134c436cbba5ea2f675619ef47702f`；Deep 允许内部分析，仍区分原件、同 TEXT 诊断副本、未改原库直接调用、静态与数学推导。只读锁定 git 对象或 METHOD 指定快照，活动上游 HEAD 不替换方法。
- CLI 0.4.0：`/Users/agilecho/code/Craft/output/tooling/craft-anything-1f8508a1/debug/craft-anything-cli`，SHA256 `99ad8c99cc5d7b35dfd85eb9b1a126ab7c074f3a5adbc7453b7a3d457d4a8869`。复用此运行件，不默认重建/升级、不启用 rebuild v4、不迁移调查。
- 继承rev19；执行中rev20与本次验收rev21均按前序SHA追加并归档需求正文，旧baseline/rev2–20字节不改。rev20的status文字继承开工授权，实际待验状态见其status_correction/next_card；rev21给出当前明确验收状态。`roundtrip.cany`、checkpoint-roundtrip 只代表历史往返测试，不取代正式调查。
- 当前 2037 条是逐文件 SHA 核对的磁盘派生原生索引，见 [native-current-index](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/native-current-index.json)。不是完整 raw export。本轮没有新增目标记录。R132 [完整核验派生1640](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/verified-complete-evidence.json)=[raw1288](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/native-evidence-final.json)+按ID352，实际路径和SHA关联见[原始来源链](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/complete-evidence-provenance.json)，继续独立保留。锁定 CLI 有64MiB/5000条加载限制，export的 `not_loaded` 必须检查；按ID补读未加载项，不把 CLI 0 当完整。

准确的只读恢复命令（无需目标应用处于运行状态）：

```sh
CRAFT_ANYTHING_CONFIG_DIR=/Users/agilecho/code/Craft/output/vesta-study/20261108-113622/config /Users/agilecho/code/Craft/output/tooling/craft-anything-1f8508a1/debug/craft-anything-cli --deep --project /Users/agilecho/code/Craft/output/vesta-study/20261108-113622/vesta.cany core-unknowns-show
```

`CRAFT_ANYTHING_CONFIG_DIR` 传入不等于 Settings 已落盘；保留真实 HOME。正式研究子进程由 `E/run_probe.py` 过滤供应商 Key，按 argv、有界期限记录 cwd/stdout/stderr/真实退出和清理；可选参数放在新探针名前，报告文件名与 wrapper 回执名分开。本轮实际命令见 R 下 `unknown-current.json`、`help-unknown*.json`、`verify-unk_*.json`，不是手写原生回执。工具 ID 是下划线形式，CLI 名是连字符形式；需 target 的工具每次重新提供 target-path，不能假定跨 CLI 保持活动目标。更新 unknown 前先读当前 revision，以真实 `core-unknown-edit --based-on-revision` 操作并验证；本轮无实质新闭合，未执行 edit。

未来原件探针只在新目录和工作副本上：事前输入/独立预期/单位/容差/完整 watch、新 cwd/VESTA_PREF、内外期限及清理余量；只驱动本轮拥有 PID/窗口，工具结束后再关窗，PID 死后不 getApp/AX 重启。255、sender0、CA/监督器0、文件存在都不能单独证明成功。本卡未启动原件新探针、外部科学计算、rebuild/chat/builder、服务或产品生成。

## 2. S08.1：全部目录、上下文与样本如何直接定位

[逐引用核对](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/reference-audit.json)检查254条记录的5,733个原 evidenceRefs，涉及1,768个不同文件；原生片段ID均存在于正式调查且见于被引文件；104个稳定样本文件/SHA及功能ID均有效。资料 ID 回到[资料索引](../VESTA资料索引.md)中的原章节；V03旧目录、180小节、脚注与135书目条目的实际阅读效力见 [SOURCE_NOTES §S02.2–.3](SOURCE_NOTES.md)，书目条目不当全文阅读或科学验收。

每个 `functions.json.records[]` 保留 `entry/preconditions/entryConditions/inputs/action/stateEffects/viewEffects/persistence/errors/sourceRefs/evidenceRefs/status/pendingEntries`。新增 `studyHandoff` 直接指向域规格、资料锚点、注册样本、原实验输入计划、S06规则/未知/下一探针。61个功能有 `samples.json` 显式反向关联；剩余193个中7个应用/系统入口无需领域文件，186个仅给候选上下文，明确未验证。没有把候选编成新稳定样本，也没有把样本表未列功能推定已实测。对候选分支，先核实际 evidenceRefs 内输入是否足够；不足时须先准备适当新样本和独立预期。用户可直接搜索一个 VF-ID 获得这条接手链。

可复用的样本与效果例：VS9非正交、VS52所选周期像、VS53反演/占位、VS54多面体、VS55误差/散射控制、VS57各向异性ADP、VS59三轴Fourier、VS-0062/63非正交及零负阈值、VS-0064/65约化、VS70变胞/粉末、VS-0074–78磁矢量、VS79轴哨兵、VS-0080–101格式补证、VS102有界规模、VS-0103/104 TIDY输入。准确路径、hash、来源与独立预期从 [SAMPLES](SAMPLES.md) → `E/s05/samples.json` 读取，不能把编号连续当作各域全通过。

[遗漏再核回执](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/omission-audit.json)复用S04菜单/手册/快捷键/对话框映射和20条pendingEntries。未发现有新证据但未归属稳定ID的新增入口；此结论仅针对已有发现输入，不能证明永远没有遗漏。重要的新版本/触发差异继续留在唯一规格中：

- VF68新版内置Powder与旧手册RIETAN外部流程分开；Import Data/Export Graph只观察入口，不当真实文件内容通过；反射/profile输出沿R73实际证据。
- VF47只有primary Cube时disabled；同一工作Cube导入secondary、默认Add/Raw后enabled/checked已实测。旧R98/R100的activation pending属历史，着色数值、保存及渲染依然未验。
- VF-0179 best plane的UI门是当前行，不能以“至少选3原子”推定启用条件；best fit科学仍未知。VF-0220 Slices alpha原件前台浮窗/AXSlider192阳性与诊断副本回调分开，slider编辑/范围/持久化未通过。
- VF-0204/VF-0213/VF-0224/VF-0225修饰键、动画、选择和删除有选定焦点/条件；物理键位全平台、IME、遮挡/列表等剩余分支不外推。
- VF80锁定arm64菜单路由是空handler，已证无动作不自创积分；Settings别名不重复建ID；48类输入含本版mcif与旧手册47类差别。

S04接受的是入口/条件及初步规格、遗漏登记；S05/S06/S07补上各自研究事实，既不倒填原轮成功，也不把这次可定位性审计当全行为验收。

## 3. S08.2：全部原生 unknown 与失败的结论

[本轮实际读取](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/unknown-current.stdout)24头与S07逐项相同：10 resolved /3 investigating /11 open，来源 `unk_e1a118c694cc99ea` revision10、数据库 `unk_3ee7a6c51afa307a` revision4、科学 `unk_522695d0039b3283` revision1均不变。[全部逐头裁定](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/unknown-dispositions.json)及 [UNKNOWNS 当前24头](UNKNOWNS.md)给出每个ID/revision、设计影响和下一探针。14个未解决头没有足以闭合其剩余问题的新材料，不为阶段计数强行resolved。

10个既有resolved均本轮真实调用 `core-unknown-verify`，返回 `valid=true / problems=[]` 且带回同一头和revision。锁定CA05 `session_tools.rs::core_unknown_verify` 检查resolved、resolution及证据存在/完整，不判断科学真值。About、截图选择、密度路径、MXD列布局、255返回、MADEL加载、短读/负晶胞、CUBE标量、lned摄入/崩溃、Raster三异常是所选问题得到解释；故障或兼容失败没有因此变成成功。旧source question中的“S02未完成”等创建时措辞不覆盖rev10实质进展。

本轮同时复核S05/S06/S07已接受的原始失败/限制和回执，入口为 [failure-and-limit-review.json](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/failure-and-limit-review.json)。保留以下设计必须消费的事实：

- 文件/资源：PDB把U写入B列；Elk物种/空字符串重开丢失；AMCSD KA(m)→Ka；ASSE移列；GAMESS首原子/网格跨度；作者DEN拒绝、TIDY sto方言、m3d差值；PRI其它flag/ABI、XPLOR尾部及商业方言未知。atom-free VASP失败，无体结构CUBE有inf/nan，STL有16非有限法向。密度工程资源恢复失败及CUBE电子密度标量外部单位不正确均保留。
- 图像：JP2无产物、SGI RGB无效、TIFF透明被丢失；Vector PDF/SVG选定解析/渲染与EPS/PS仅结构核验分开。图像成功提示/非空文件不代表内容正确。
- 科学：R64原1e−6三项失败与后续打印政策并存；R74固定左移模型被否定，R77模型/误读失败，粉末首点实际0与独立非零预期不符；投影归一偏差、连续导数与26邻点兼容模型不等价。磁矩降对称丢失、GUI追加相Y缩为.1及Vectors Cancel崩溃不删。
- 完成/工具：空产物、读取/工具拒绝、截断、默认cif.txt覆盖且首表SHA丢失、内外超时/软限越界、监督器提前断开、强制清理/AX额外重启等按原记录保留。R101写入原因、R121/R125生命周期失败、S04 alpha首wrapper超时/原件SIGTERM清理均不改成正常成功。S07 TIDY四child退出是runner-reported/Popen.wait，CA直接监督主体是外层Python，未冒称逐child独立waitpid。

没有重放已知崩溃或为填表重复科学/CLI问题；没有伪造旧成功回执。S05完整格式兼容、S06全部科学、S07全部CLI变体/外部程序/平台仍未接受。

## 4. S08.3：领域表示、算法和依赖的确定输入

下列是由已读规格支持的设计约束；产品数据结构、API名称与实现栈仍待P1设计。

**结构与场景分离。** 工程包含相的顺序/身份、晶胞/对称、非对称位点（元素/label、fractional坐标、occupancy、U/B/beta）、磁/极矢量、体数据层/资源、晶形面；显示image、选择/隐藏、对象style、相机SCENE、文本/偏好不能混成同一个原子集合。CLI Read会追加Crystal但不转移临时Scene全局style，GUI追加保留全局style且有Y分量异常，因而“文件读入”不是全场景替换的通用契约。见[行为01](行为/01-界面与交互.md)、[行为02](行为/02-结构与几何.md)、[FORMATS](FORMATS.md)R85–97。

**几何基础。** `r=Hf, G=HᵀH, V=|detH|`；所选周期image与全局最短像分开。变胞 `H'=HP, f'=P⁻¹(f-p+n)` 及ADP基变换有选定证据，不推广到全部群/setting/协方差。法向 `g=H⁻ᵀh`、间距 `1/|g|`，晶形面D不是d_hkl；特定位点去重/重数与显示边界计数分开。键区间、递归范围、多面体规则及无符号二面角按[ALGORITHMS](ALGORITHMS.md)、[S06_ORACLES](S06_ORACLES.md)逐样本的精度与先后关系消费；R17晶形是事后独立重算，不改称事前先验。

**场与科学算子。** 周期core N和闭合N+1网格、轴顺序、场量及单位必须显式。CUBE x外/z最快，ggrid x最快；坐标bohr不代表scalar已换单位。散射 `F=Σocc·f·T·exp(+2πihf)`，Fourier `ρ=ΣF·exp(-2πihf)/V`，Patterson模平方，反射集合/Nyquist和F000单独规定。电子密度14e、核密度2.064fm、核/电子Patterson 4.260096fm²/81e²等仅所选DC/不变量，不能替全频谱。线剖面周期三线性、2D切片/投影、密度26邻点兼容规则都有明确已证/反例。见[行为03](行为/03-体数据与二维.md)、[行为04](行为/04-科学计算与外部程序.md)、[31组规则/206领域ID](S06_DOMAIN_RULES.json)。该索引每组均有unknown/nextProbe，`all_scientific_rules_proven=false`。

**依赖顺序的设计输入。** 单位/坐标/对象身份与修订支撑对称、变胞、成键、测量和ADP；其上叠加体数据/反射/FFT、晶形、渲染/选取；codec与资源身份贯穿各层，页面与无头都消费同一业务语义。科学操作、预览、Apply、Cancel和撤销的事务范围要按每条实际行为决定：Properties若干Cancel有残留，Boundary Apply后Cancel保留，不能把任一窗口的规则泛化。此处仅交付依赖，不指定模块代码、排期或最终栈。

## 5. Web、正式命令与平台接缝

D06–D12的实际资料入口在[资料索引](../VESTA资料索引.md)；本轮所读章节及路径重定位见 [source-reading-ledger.json](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/source-reading-ledger.json)。本轮本地指导资料为D06–D12的16份、D02及P01接入指南，合计18份；这与P01–P07选读的18份外部文件是不同集合。其中本地链接中，20处继承的相对路径按字面位置已失效；均已在上述路径重定位回执中给出实际存在的IntegrateCraft原上下文目标。未改写锁定D02，未把失效字面路径报可用。方法/契约模板中的示例类型不是VESTA已实现接口。原VESTA为只读原生发布包、未发现可直接移植的开源内核；CA Web是轻量分析查看器，不能当可编辑VESTA Web。

可供P1比较的技术候选是原创Web内核、可移植领域内核配Web绑定、或受控后端处理确需原生的计算。纯数值/codec的浏览器可行性由依赖和测量决定，系统进程、原生绘图/字体/文件API必须分离。当前没有验证任何候选的WASM、Worker、GPU、字体、静态托管或目标容器性能，故不在S08替用户确定Rust/egui、TS、WASM或服务端方案。最终目标是Web可手改、Agent操作当前页面，不能以远端图片或桌面默认栈缩减。

原件的16条正式CLI合同、实际argv、GUI/nogui/IPC区别、退出与输出请直接读 [CLI](CLI.md)当前收束及 [official-command-contracts.json](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/official-command-contracts.json) / [argv索引](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/official-command-argv-index.json)。这些是原件参考能力，不是重写产品已有命令服务。内部保存别名、不可达set_*、add_bond stub、remove删除工作文件与powder空DataSet崩溃均不能自动暴露为产品可用编辑API。GUI只有x30旋转实测，y/z GUI未验；flush不是已证渲染屏障，sender0不是host完成。

本轮只读重登记 IntegrateCraft 参考：HEAD `9dc541249dbea68fa12b6bffc3e62e4ce365af90`；工作区有72条既有状态，未归属为本卡改动。P01–P07选取18份实际文件均与当次HEAD字节相同，副本/摘要/符号行位于 [platform-reference-baseline.json](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/platform-reference-baseline.json) 和 `R/platform-snapshot/`。这只锁本轮参考材料，与CA方法锁定是两回事；不承诺整个活跃仓库干净或产品接通，不恢复/清理任何外部改动。

已核代码接缝及必须保留的差异：

- P02 `catalog.mjs::SOFTWARE`、startup/service和client负责注册/启停/工作台；选读catalog没有VESTA注册。本轮没有添加适配器或生成bundle。最终接口必须从产品真实注册表/schema生成，不能把模板名当已有工具。
- P03 `PageChannel::claim/send/poll/reply/release`已有页面身份、队列/超时和断开拒绝；当次源码send上限32、默认60秒、claim新鲜期15秒，client在1698行100ms轮询。这些是参考代码事实，既不是目标延迟测量，也不是为VESTA重新确定预算的裁定。`SoftwareCoordinator::serial`串行且有保存/交接，避免在既有队列内等待自己。
- P04 `SpaceFiles`、`SoftwareFiles::verify/write/reload/newPath/newExportPath`及native-paths已有路径、外改SHA基线与命名接缝。需复用这些职责，另核VESTA多资源工程提交、导出扩展/格式竞争与下游接受范围；没有自动支持VESTA格式。权限、配额与模型额度继续由实际平台入口强制。
- P05 `createWordPage`中的snapshot/ticket/ack和 `WordCoordinator::changeMode('agent')`是真实Word样例。当前后者保存后转MCP，通用coordinator也会按mode换owner；D02对VESTA明确要求三视图只改布局、页面不卸载/不丢编辑/不打断任务，不能逐行照抄这一行为。真正关页才按已核CLI/命令交接同一已保存作品，页面持有编辑权时必须走页面命令。
- P06锁文件/补丁/字体/制品组织可参考，Word工具链、预算和“保存即下载”等历史说明不沿用。P07 api-browser/native-runtime/save-status-browser选读的是测试组织：真实页面/官方CLI、内容/同稿身份和409/507/网络/200确认；未运行这些测试，不把其旧通过或受控HTTP当VESTA真实平台验收。

页面、模型、文件的完成时刻必须可区分：命令接收/排队、内核提交、对应修订渲染就绪、宿主个人空间真实写入确认。D02的工程/导入源/另存目标/导出副本身份、保存期间再编辑、切稿后的迟到回执、外改冲突、失败保留dirty、不自动下载及无浏览器作品持久副本都是已确认要求；产品的实际字段/取消/幂等/batch语义尚待设计并实测。原件的无限路径能力不能当平台授权；不复制裸文件守卫或另建一套运输/计费系统。

## 6. codec、资源、无头、中文与性能

S05 [输入48类](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-format-matrix.json)、[数据输出23类及其它输出](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/output-format-matrix.json)、[FORMAT_SOURCE_COVERAGE](FORMAT_SOURCE_COVERAGE.md)、[FORMATS](FORMATS.md)已分别列读取/编辑/保留/导出事实或失败/未知，不能由“有样本/自读回”接受全格式。原创codec须明确字段、版本、单位、缺失/未知块、坏输入后旧工程及资源路径；允许保留原始块不能代替语义理解。PRI flag/ABI、m3d及DEN/STO方言等影响相应codec设计的部分保持前置补证。

体数据资源相对引用、另存/重开与已有路径拼接故障说明主工程与资源应共同校验。原件失败如何兼容、是否拒绝/提示/提供修正选项是后续设计裁定，S08没有默认修复或默认复刻科学错误。保存必须到本人空间确认对应快照；多文件原子提交/资源打包形式尚未确定，不能主文件替换成功就宣称全工程事务。

11类外部/关联入口和10类完成分支见S07 [external-program-contracts](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/external-program-contracts.json)、[completion-boundaries](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/completion-boundaries.json)。MADEL缺绝对GNU Fortran库在main前SIGABRT，无科学成功；TIDY选定P1/约化与四案子返回有证，坏群退出0仍失败；RIETAN真实外部计算、ORFFE精修/全协方差及绘图器未运行。原分发文件再分发书面许可未取得，专业来源可读不等于实际运行件资格。参数表、字体、样本、模板、原库和工具不能自动打进产品。最终相关功能保持，可评估获准依赖或原创实现，不能因缺依赖删功能。

CA08锁定源码 `CheckParams` 为project可显式或由inbox推导、wait_minutes 1..60/default20；`rebuild_check`把请求投给运行中build worker，并非独立离线检查器，同进程API builder自查拒绝。当前只有Study调查、没有产品app/worker/inbox接线；S06 `research/MECHANICS.md` / `mechanics.json`只是迁移输入，不能当原生Deep mechanics检查已过。P1须先决定Web/无头构建与worker职责，再按真实接口接线；本卡不空调用制造超时。

本机nogui仍经wx/AppKit、有WindowServer登录会话；无窗口/清DISPLAY不证明真正displayless。Linux x86/容器的准确运行件、依赖/字体/沙箱、冷启动/失败/信号/进程树均未验。Mac工具及附带x86_64 TIDY的局部结果不跨环境追认。

中文已有VS12中文空格文件路径与选定读取结果，数据库/Comment文本有各自范围；Web中文命令、IME、画布/导出字形、字体来源/许可/加载与无头环境均未验。候选验证先用VS12及包含“晶胞α、占位½”的新命名/文本输入，分别核文件名、结构字段、显示、导出、重开；此文本是未来样本建议，没有编成已运行稳定样本。

性能已有VS102两规模：4896/35904唯一值→5814/39270闭合点，共45084值逐点独立精确符合；仅证明所选格式/布局与有界规模，未测RSS/OOM、最大网格、GPU、拖动、冷启动或p95。未来按模型小样本VS54、域计算VS59/62、体数据VS102及新较大有界fixture分开测解析/计算/传输/首次可见/交互/导出，记录实际设备、DPR、资源与分位；不自动采用开发机24GiB或旧平台1GiB作为已测产品预算。

## 7. P1前置缺口与下一建议

以下是设计依赖，不是新增产品缩范围或“必须全部unknown关闭”的S08验收门：

1. **DPRE-01 科学表示与兼容策略**：依赖 `unk_522695d0039b3283` 与31组nextProbe。冻结相应模块前补一般群/多相R/t、键价表绑定、ADP几何、等值面/颜色、投影/Nyquist/反常表等会改变该模型/算法的证据；可先基于明确H/G/单位/身份约束讨论不依赖未知的边界。
2. **DPRE-02 codec与缺陷处理**：13个格式/来源/网络等未解决头中的实际格式项、已resolved但仍失败的CUBE/密度/Raster等分别进入依赖模块；先核原始字段/机制和适用标准，再在设计中明确兼容/提示/拒绝。不可悄悄把自读回或原因解释当格式正确。
3. **DPRE-03 Web路线与资源**：没有产品Web内核/正式API、没有依赖可移植性和真实渲染/资源测量；先比较候选并核选定依赖/许可、客户端资源和目标部署条件，不直接写已接通方法或执行构建。
4. **DPRE-04 平台保存/编辑权**：P01–P07参考与D02间的mode→owner差异、多资源提交/迟到确认/外改和不下载必须设计清楚；路径与只读/配额复用现有守卫，新增差异留届时真实整链验证。
5. **DPRE-05 无头/Check/目标环境**：Linux x86运行件、无图形依赖和build worker/inbox接线未成立；没有准确接线前不调用Check或报Ready。独立的数学/codec接口设计不因此被删，但相应部署结论保持未验证。
6. **DPRE-06 外部计算与资产资格**：MADEL/RIETAN/ORFFE科学与运行件、TIDY通用群、再分发/参数/字体/模板/样本资格逐项核；不得用缺环境删除最终功能或自动安装/修补原包。
7. **DPRE-07 中文与交互性能**：VS12仅路径证据；IME/字体/图像/目标浏览器和资源上限必须先补相关样本与测量，再定受影响的方案和阈值。
8. **DPRE-08 参考版本续验**：IntegrateCraft的本次只读参考在新施工前重新核当次HEAD/工作区/选用符号与制品；CA方法继续锁定，不因外部参考重登记自动升级。缺失历史材料（如R68首表SHA、未归档CUA截图）明确不可复查，不补造。

建议下一授权动作是以本报告和这些前置证据进入P1设计讨论/必要补证，先确定依赖的设计问题；完成设计后再写精确到代码、接口和命令的执行计划。没有为P1/Build/Check授权、未定预算与环境的内容，本会话不执行、不自动发新会话。

## 8. S08.4交接与验收记录

[HANDOFF](HANDOFF.md)保存当前头、准确恢复命令、已回答/失败/未知和下一动作；[资料索引](../VESTA资料索引.md)提供D01/D02、CA05/CA08、D06–D12/P01–P07与专业来源。机器状态 `E/reviews/current-task-status.json`、原S08四项和总体P0.2/P0.3已在真实自验与dsh裁定后同步。本轮原始证据、旧封存和基线不覆盖，新的需求正文另归档。

自验范围为本地路径/章节、功能/样本/原生证据/unknown ID、目标/CLI/锁定源码身份、参考源码快照、前后文档/基线链及历史封存；它不替独立复审或科学/产品验收。R下保留真实自验输出、dsh请求/全部响应与退出/超时、裁定、发现处理、最终摘要和封存清单。dsh使用正常启动环境，不套CA的Key过滤、不读/打印凭据；只读审查，不施工。

S08是否完成按原“新会话可以从记录直接找到每个功能的资料、规格、样本与证据，知道哪些已证、哪些仍需研究，且无需依赖聊天猜参数或范围”的标准判断。原文标准见本报告开头链接；样本缺口仍是研究前置，不伪装成实测样本。卡完成与科学全正确、所有unknown闭合、全格式通过、P1已执行、Build/Check/Ready及最终产品交付分别记载。最终全部功能范围不缩减。

### 本轮实际验收结果

[S08原始dsh裁定](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/review-result.json)明确四项、Study研究交付、P0.2/P0.3均pass；[实际进程](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/dsh-review-run.json)exit0、未超时、868.099秒。S08.1接受逐功能导航及明确样本/行为缺口；S08.2接受24头/失败状态复核与下一探针；S08.3接受确定输入与P1前置分离；S08.4接受基线/恢复/ID/路径/历史证据交接。独立复审没有重跑VESTA、全部3272链接或全部25054自验，准确复核范围和未判断项见[dsh原文](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/dsh-review.stdout)。

[3项发现处理](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/review-finding-resolutions.json)：数据库/Undo/Redo改为特定未来探针，输入导航字段改为relatedEvidenceLocatorRefs并明确注册样本回退，当前未解决头误写13改14；另澄清资料集合计数、R132直链和稳定功能ID。全部是Codex文档修订，无新原件运行/原生记录/样本/unknown revision；原审查输入完整封存，没有重跑dsh或声称修订后二次独立复审。最终本轮自验与封存分别见本卡R下的 `packet-verification-accepted.json`及`seal-verification.json`，用于完整性/状态验收，不代替科学或产品测试。

历史S08交付时点：当时停止在S08，建议后续授权处理DPRE依赖并做P1设计；本次用户新授权见页首当前接续。

## 9. P1设计接续（2026-11-10）

Study原S08主体/封存仍表示当时交付；本轮已按§7消费DPRE-01–08并形成[架构](../superpowers/specs/2026-11-10-vesta-p1-design.md)、[接口](../superpowers/specs/2026-11-10-vesta-interfaces.md)、[CA接线](../superpowers/specs/2026-11-10-vesta-ca-wiring.md)及[模块计划](../superpowers/plans/2026-11-10-vesta-modules.md)。现已获dsh partial裁定，P1.1接受，其余三项未完成。用户两项兼容/原生保存裁定见需求§9；本轮probe仅本机最小WASM/Worker/WebGL2，不新增原件记录或科学/格式通过。尚未冻结的关键接口/门在设计§7与接口§7，不因Study完成抹掉。

### P1本轮验收与准确边界

P1正式dsh结果partial，仅P1.1路线/技术栈/目录接受，其余三项未完成；原Study29/29及全部失败/未知保持。证据R为E/reviews/20261110-p1-design：完整复审392.780秒exit0、同session差异复核210.355秒exit0，首轮510.316秒stream中断exit1没有裁定。I3主修正获差异复核接受，残余两处由Codex改I4；I4已于2026-11-11由dsh独立复审，R3-1/R3-2实质解决，P1仍partial。最终状态见[HANDOFF](HANDOFF.md)及R/review-result-effective.json。

G-CA真实worker/模型/产品联调未运行，亦不在本P1当前授权内；继续设计应先固定prepare输入/锁构建/调查迁入绑定/目标kind/launcher/恢复契约，不把“没启动产品”单独当设计失败理由。科学公共payload及高级精确计划仍有真实设计缺口，不能全P1勾选。


历史R5主体设计续办：D7/I9/C6，资料rev30；31个有界操作、85格式路由/6类字段政策/28入口绑定、15项具体资格阻塞。P1.2/P1.4/整体partial，P1.3仅C4设计接受，C6增量待审；不进入P2–P5、产品实现或运行验证。证据与手动复审入口：[R5](../../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/README.md)。


历史R6同卡整改：2026-11-14 R6：D8/I10按dsh R5复审整改D1–D14及σ公式，Codex自验待用户手动转dsh。P1.1继承接受、P1.3 C4设计级接受保持，C6原字节及未审增量身份不变；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料rev31；不进入P2–P5、产品实现、部署或运行验证，真实worker／Check／launcher未授权、未验证。 施工回执与手动复审入口：[R6](../../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/README.md)。


当前R7勘误：2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。 [回执/复审入口](../../output/vesta-study/20261108-113622/reviews/20261114-152228-p1-design-r7-errata/README.md)。
