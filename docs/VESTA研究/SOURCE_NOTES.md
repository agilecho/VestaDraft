# VESTA Study 资料阅读记录

**当前任务状态（2026-11-10）：** S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 来源unknown revision10/high/investigating保留目标绑定边界；下文旧 revision/未勾选续记为历史，来源阅读不等于能力通过。接续见 [HANDOFF](HANDOFF.md)，阅读边界见 [SOURCE_NOTES](SOURCE_NOTES.md)。

初次复核日期：2026-11-08；最新状态对齐：2026-11-10（Asia/Shanghai）。资料 ID 见[资料索引](../VESTA资料索引.md)。本页区分文档自述、源码静态核验、真实运行和未知；阅读通过不登记功能运行通过。

最新来源专项为R134（2026-11-10）：22项指定专业正文范围（19项本地字节、3项web正文），dsh已明确来源阶段通过、无硬阻断；来源问题实际revision10/high/investigating。专项完整性验收已实际通过，S02.3现已勾选；目标绑定/完整格式与数值仍由S05/S06继续核。最新完整原件证据仍R132，未执行新目标探针。

上轮接续见[SOURCE_REMAINING_FORMATS](SOURCE_REMAINING_FORMATS.md)：MOLDA数据集作者说明、C04D联动文档、GULP/CP2K XTL指定范围已补，ASSE/eb原作者页未得。该轮来源问题为revision7/high/investigating，S02.3仍未完成。

上轮接续见[SOURCE_LEGACY_PRODUCERS](SOURCE_LEGACY_PRODUCERS.md)：FDAT生产者说明、CSSR/CRYSTIN消费文档和WinGX/MCE集成有补读；rho的wien2venus三维来源与LAPW5二维参照已澄清。该轮来源问题为revision6/high/investigating，既有目标/科学状态不变。

上轮字典接续见[SOURCE_DICTIONARIES_LEGACY](SOURCE_DICTIONARIES_LEGACY.md)：四字典的指定定义、USPEX输出和CMTX版本条目补读；AMCSD仍只有入口/索引摘要。8条记录仅6处正文范围，该轮来源问题更新为revision5/high/investigating。未执行字典方法，不登记目标格式/科学通过。

上轮接续见[SOURCE_VOLUME_SCIENCE](SOURCE_VOLUME_SCIENCE.md)：本轮9项续记中6项补正文范围，涉及WIEN2k rho、CASTEP charg_frm、Elk writer、GSAS fmp及Chantler1995/Tsirelson2002选段。其余为定位/摘要或重复版本参照，非9篇新全文；该轮来源问题更新为revision4/high/investigating，原question是历史文本。原件功能/科学通过状态不变。

第一轮补证见 [SOURCE_SUPPLEMENT](SOURCE_SUPPLEMENT.md)及[25项历史台账](../../output/vesta-study/20261108-113622/reviews/20261108-133154-s02-sources/source-reading-ledger.json)：XSF官方HTTP正文和CIF等已补。最新续读见[SOURCE_FORMATS_SCIENCE](SOURCE_FORMATS_SCIENCE.md)及[15项续作台账](../../output/vesta-study/20261108-113622/reviews/20261108-135713-s02-formats-science/source-reading-ledger.json)，新增CTfile/SHELX/X-PLOR/Chem3D原始资料与Baur原文选读；[覆盖表](FORMAT_SOURCE_COVERAGE.md)逐项列47类旧手册输入和23个已观察数据输出。S02.3仍未勾选，该轮来源问题更新为revision3/high/investigating；原question中的XSF失败为创建时历史表述，现状以新台账/probes为准。

生产者追加选读已补FEFF、FHI-aims、Elk、GSAS、WIEN2k、CASTEP、GAMESS/MacMolPlt，另有[8项台账](../../output/vesta-study/20261108-113622/reviews/20261108-135713-s02-formats-science/producer-reading-ledger.json)，与本轮前15项合计23项。各版次、实际章节及rho/charg_frm/GEOMETRY.OUT等剩余缺口已回写来源续读与覆盖表；来源问题继续revision3/high/investigating。

## 历史S02独立复核结论（2026-11-08）

| 项目 | 当时裁定 | 依据与边界 |
|---|---|---|
| S02.1 | 保留 [x] | 本次逐项重读锁定源码，补齐模式、步骤顺序、默认参数及 Ready/检查能力边界。 |
| S02.2 | 补证后保留 [x] | 本次重读第 1–18 章叙述、LaTeX 公式文本、快捷键表和书目目录；与官方目录逐项对齐。没有由文本读取推定公式正确、图片已检查或新版功能通过。 |
| S02.3 | 保持 [ ]，部分完成 | 独立复核时五入口不足；续作已补指定规范及外部程序章节，其他核心正文和目标绑定版本仍有缺口。原验收“逐项核相关规范/论文/外部程序的准确版本与章节”尚未达到。 |

上表为 2026-11-08 裁定；R133 的 dsh 专项复核与标准澄清见页末。目标运行契约仍由 S04–S07 核，不把它们全部提升为 S02 阅读条件。

独立[复核目录](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/)包含当时修改前副本、源码身份与行号摘录、正文获取时间/摘要、目录检查、外部来源登记及验证。该轮新建 `unk_e1a118c694cc99ea` revision 1/high/open 登记 S02.3 来源覆盖缺口；[创建回执](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/review-source-unknown-add.stdout)和[当时正式调查读回](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/review-unknowns-after.stdout)均来自 `vesta.cany`。续作更新见页首；curl/web读取未包装成CA evidence，evidence_for/evidence_against仍为空。

## S02.1 Craft Anything 原始方法

锁定提交仍为 `1f8508a104134c436cbba5ea2f675619ef47702f`。本次实际读取 CA02/03/06/07/09 及关联实现，定位见[源码身份与摘录](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/source-identities.json)。以下是源码结论，不表示本卡执行了原生 rebuild worker。

| 来源、符号与行号 | 核实结果及本项目对应 |
|---|---|
| CA02 `brief.rs:7–37`，`RULES/steps` | 用户指定的语言/平台/范围优先于默认值，不静默删除功能。先 README/摘要，再聚焦追证、复用已答探针，登记 unknown/严重度/探针，解决时给证据；随后维护 3–7 步清单，在实现区构建，调用真实 `rebuild_check` 修差异，回复前重读清单和 unknown。原生新建工作区限制实现写入 `app/`；不是禁止当前获授权研究目录写文档。 |
| CA03 `study.rs:13–131`；`mod.rs:732–737` | 新研究先 Study，再进入 Build。Deep 本地目标先定位主程序并 triage，按条件准备/等待 Ghidra，适用时列资产/映射 JS；允许 runtime study 时，控制台采运行记录、窗口程序采窗口；写 capture/README。读取失败写入 notes，不能当成功。没有自动穷尽全部 VESTA 功能的保证。 |
| CA03 `study_from_outside:157–170`，`picture_window:174–193`；`clean_room.rs:116,135–149` | Clean room 不读内部文件、目录或代码，工具按白名单拒绝内部分析；但 Worker 在 Settings 允许 runtime study、直接选择可执行文件时，可启动一次并拍外观。不能写成“任何情况下绝不运行”。`picture_window` 固定等 15 秒并请求 window.png/UI tree；只按调用和 tree 存在拼 notes，未核图片内容，因此不能由成功回执追认 About 66×20 小图有效。 |
| CA06 `brief.rs:161–184`，`DECOMPILE_FIRST/decompile_note` | Deep 本地程序必须找相关功能、字符串/引用和函数并等待读取，先查后估；常量、公式、规则和地址记 MECHANICS.md，再原创实现。Ghidra 不在时内置反编译更粗糙，值还须第二函数或原程序文件交叉核实。`mod.rs:809,892–896` 是一次补提醒机制，不是所有功能均已研究的证明。 |
| CA06 `MECHANICS_CONTRACT`、`MechanicsCheck::run` | mechanics.json（version 1）与人读表同值，每项给 source 或 estimated。实际检查读表及扫描实现/数据文件的数值字面量，非 estimated 数字按绝对值七位有效数字比较，忽略正负号；最多给 60 条 claims，有扫描大小/深度限制。缺少 source 只进入 note，最终 passed 取 missing 数字是否为空，不证明来源真实、文字/布尔规则已使用、科学公式或数值计算正确。 |
| CA07 `AppKind::default_stack/goal/checks` | 默认窗口栈 Rust+egui；Deep 检出控制台时为 Rust console；goal 只是桌面核心功能。默认检查列表为 Build/Runs/Screenshot/DeepOnly Mechanics，具体是否适用另看 applies，不能说每项总是实际运行。本项目仍是全功能 Web/Agent，并遵守 D02 保存、当前页面、权限与续作契约；技术栈留设计阶段。 |
| CA07 `RunOptions::default:155–173`；CA04 `RebuildArgs:44–47/run:139–148` | 内部 max_rounds=6、good_enough=90；CLI `rebuild --rounds` 默认 2，显式传给 max_rounds，good_enough 来自实际 Settings。两者都是首次构建后的修正轮上限，不能混成总共构建两次/六次，也不是本项目预算或全功能完成标准。 |
| CA09 `mod.rs:899–974`，`CheckResult::good:308–313` | 每轮 Check，good 同时要求 passed、无 check unknown、claims 全真及满足相似度门值；还有停滞/连续失败错误。达到修正上限会退出迭代。Ready 前 usable 只看最后 check 的 passed，再摘要最后一轮各检查并报告 open questions；Ready 可带差异/未知，不能替代本项目全功能/科学/平台验收。 |

本轮采用 Deep；既有许可证、包元数据、内部分析不包装为纯 Clean room。仅继续资料研究与原生问题登记，没有启动 rebuild/chat、builder、VESTA GUI 或科学探针。运行参数与恢复方式仍以 [METHOD](METHOD.md) 为准。

## S02.2 官方目录与正文核验

[官方目录](https://jp-minerals.org/vesta/en/doc/VESTA.html)日期 **2022-08-10**。锁定目标仍是 AX 实读的 **3.90.9a / 64-bit / Aug 8 2026**（见 [BASELINE](BASELINE.md)）；旧手册不是该开发版的完整实测清单。

原 S02 的 21 份记录保留原字节。本次重新获取目录及这 21 份 HTML：21/21 的原始 HTML SHA-256 与旧记录相等；旧临时文本 21/21 的字符/行数也相等，但各自摘要均不等于旧 sha256。因此旧 `bytes/sha256` 对应 HTML，`text_chars/text_lines/temporary_text` 对应提取文本，不能混比。新获取时间、响应、原 HTML 和新提取文本的不同摘要见[获取记录](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/manual-acquisition.json)，逐章对照见[阅读复核](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/manual-reading-review.json)。旧记录无获取时间，不能倒填为历史访问时间，也不能从文件存在反推历史已读深度；本次正文重读支撑当前裁定。

[目录核对](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/manual-directory-audit.json)按官方目录检查 118 个正文/附属页链接及锚点；另登记 180 个编号小节的标题与直达 URL（见阅读复核），这是定位数量，不是功能数量。下表各章状态均为**本次叙述正文已重读**；图片视觉、公式正确性、原件行为与当前版本适用性继续待证。

| 直接来源与已读范围 | 功能入口/摘要 | 后续待证问题 |
|---|---|---|
| [第1章](https://jp-minerals.org/vesta/en/doc/VESTAch1.html)，§1.1–1.4 | 背景、v3 新功能、数值框四则表达式和 Tab/Enter 焦点。 | 文档介绍不当运行通过；焦点和非法输入按原件核。 |
| [VM02](https://jp-minerals.org/vesta/en/doc/VESTAch2.html)，§2.1–2.7.2 | 三类数据叠加、选择信息、外部 ORFFE/TIDY/RIETAN/MADEL、工程外链、GUI/内核分离自述。 | §2.6 说 42 类输入，§18.5 列至 47；不得合成新版格式总数或可复用源码结论。 |
| [VM03](https://jp-minerals.org/vesta/en/doc/VESTAch3.html)，§3.1–3.6 | 各 OS 启动/拖放/关联、地区设置、缺库与图形故障。 | Intel-only 等旧要求不套到本地 universal 开发版或目标 x86 环境。 |
| [VM04](https://jp-minerals.org/vesta/en/doc/VESTAch4.html)，§4.1–4.5.3 | 六菜单、横/纵工具栏、Tools/Style/Objects、Output/Summary/Comment；保存与导出分开。 | §4.1 称两类文本，§4.5 实列三页；实际布局/禁用前提以 S04 为准。 |
| [VM05](https://jp-minerals.org/vesta/en/doc/VESTAch5.html)，§5.1–5.3.1 | 结构、椭球、点表面、等值面/截面、次级数据着色和晶形；各组无相应数据时禁用。 | 默认显示/半径因子/渲染和叠加需运行、保存与重开证据。 |
| [VM06](https://jp-minerals.org/vesta/en/doc/VESTAch6.html)，§6.1–6.5.1 | Edit Data 的相、晶胞/对称变换、磁群、占位/位移、外链、五种体数据运算及晶面。 | §6.3 写标签6字符，§6.3.1写9字符；图6.15自旋差式重复 up；矩阵、符号、单位和无效输入须 S05/S06 独立核。 |
| [VM07](https://jp-minerals.org/vesta/en/doc/VESTAch7.html)，§7.1–7.3 | Phase 定位/取向，layer0 为内部笛卡尔系，顺序防循环，直接/倒易方向配对。 | 多相依赖及 hu+kv+lw=0、不相容方向和保存结果待观察。 |
| [VM08](https://jp-minerals.org/vesta/en/doc/VESTAch8.html)，§8.1.1–8.2.5 | Bonds 三搜索模式、三边界模式、按标签、中心多面体、分子/氢键/笼状例。 | 递归有边界限制；分子模式无中心多面体。周期边界与异常距离留实际探针。 |
| [VM09](https://jp-minerals.org/vesta/en/doc/VESTAch9.html)，§9.1.1–9.2.2 | Vectors 的轴/极/时间反演、位点/单原子附着、长度选项；Lattice Planes/拟合平面。 | 预览取消、向量坐标换算、对称作用和退化拟合待验证。 |
| [VM10](https://jp-minerals.org/vesta/en/doc/VESTAch10.html)，§10.1.1–10.2.4 | Boundary 范围与截切、Orientation 方向/矩阵/晶轴。 | §10.1称边界变化重置选择/隐藏；V02 3.4.7称保留标签/选择，须以新版实测分项裁定。 |
| [VM11](https://jp-minerals.org/vesta/en/doc/VESTAch11.html)，§11.1–11.8 | 拖动/动画、缩放/平移、选择/隐藏、几何和多面体体积/畸变/ECoN/电荷/价键。 | s.u. 忽略非对角项；旧文价键 b 固定0.37 Å及2011/2009参数文件并不证明本地 bvparm2020 的使用规则。二面角符号/退化等待核。 |
| [VM12](https://jp-minerals.org/vesta/en/doc/VESTAch12.html)，§12.1.1–12.2.6 | Properties/Preview/Cancel/default；各对象材质/分辨率/双不透明度/多等值面；侧栏分层编辑。 | 饱和端点的两段文字与映射式疑似相反；不直接当色标算法。公式/归一化和负位移 cuboid 待核。 |
| [VM13](https://jp-minerals.org/vesta/en/doc/VESTAch13.html)，§13.1–13.4 | Overall Appearance 的背景/光照/平行透视/雾化，内部场景归一化。 | fog 公式排版错位，坐标归一化不可冒充科学数据单位；渲染待观察。 |
| [VM14](https://jp-minerals.org/vesta/en/doc/VESTAch14.html)，§14.1–14.15 | Utilities：几何/标准化/约化、势能/衍射、结构因子/傅里叶/模型密度/Patterson、二维/线剖面/峰与Voronoi/密度转换。 | 旧衍射流程依赖 RIETAN+绘图器，不直接套到新版窗口；§14.5 REGION解释却写RADIUS、§14.7对话框名等疑误需核。各式/常量是文档自述，未证明算法。 |
| [VM15](https://jp-minerals.org/vesta/en/doc/VESTAch15.html)，§15.1–15.7（含§15.5.1–.3） | 二维窗口/工具、三种Slice/投影、插值、鸟瞰、线性/对数等高线与数据导出。 | bounding-box 模式文档禁用二维数据导出；不能将该禁用当功能缺失；向量/采样式待数值核。 |
| [VM16](https://jp-minerals.org/vesta/en/doc/VESTAch16.html)，§16.1–16.8 | Preferences：RIETAN可执行件/Viewer/模板、字体、新开目标、动画、自动成键、图像细分、等值面默认和撤销历史。 | 模板版本须匹配；默认等值面公式与历史100是旧文值，待当前版本核。 |
| [VM17](https://jp-minerals.org/vesta/en/doc/VESTAch17.html)，§17.1–17.4及16个命令项 | 文档命令名/选项、几何列表、旋转、打开/重开/保存/输出。 | HTML中§17.2/.3为空，全部详细命令排在§17.4后；因此不能按排版给命令断言GUI模式归属。`-calcf`未展开选项；完整CLI契约留S07。 |
| [VM18](https://jp-minerals.org/vesta/en/doc/VESTAch18.html)，§18.1–18.5.4 | 使用文件/偏好、一般/周期网格、二进制头和对称扩展、47项输入列表、数据/几何输出、14图像项和输出文本。 | §18.4 的 fType/gType 混写、nAsym注释、赋值式if有疑误；STL项链到OBJ，输入/输出后缀亦有笔误。准确类型/轴序/字节序/资源行为继续S05；不能照抄当规范。 |
| [VMA](https://jp-minerals.org/vesta/en/doc/VESTAap1.html#x50-271000A)，表A.1 | 区分全局菜单快捷键与Graphics Area焦点；字母工具、方向旋转、Shift小步和Alt平移。 | Ctrl在macOS的对应、焦点/文本框/上下文冲突留S04，未把附录当新版实测快捷键表。 |

旧提取文本没有包含HTML分拆出的28个脚注正文。本次全部补读且28/28响应200、锚点可定位，见[脚注获取](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/footnotes-acquisition.json)和[逐条阅读](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/footnote-reading-review.json)。脚注确认macOS Preferences/About位于VESTA菜单，另给出ORTEP、gnuplot/Igor/WinPLOTR/RietPlot/PowderPlot、Gaussian03常量和图像格式入口；只读脚注不意味着这些目的页已读。

官方目录还含[许可页](https://jp-minerals.org/vesta/en/doc/VESTAli1.html)、[结语/反馈/致谢](https://jp-minerals.org/vesta/en/doc/VESTAli2.html)、[术语索引](https://jp-minerals.org/vesta/en/doc/VESTAli4.html)。本次补读前两页，索引只作定位导航核对；它们未被包装为额外功能章。书目135条已读条目，不等于135篇原文已读；见下节。

## S02.3 版本、规范、论文与外部程序

以下访问日期均为2026-11-08；准确获取时间见对应 JSON。旧五条[入口记录](../../output/vesta-study/20261108-113622/s02/primary-source-entries.json)没有 HTTP/正文摘要/时间，不追认其历史全文读取程度。本次[五入口获取](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/primary-acquisition.json)、[补充获取](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/supplemental-acquisition.json)、[版本/阅读范围](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/primary-reading-review.json)与[网页读取笔记](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/web-reader-notes.json)分别登记通道，403页面摘要不当规范摘要。

| 来源ID / 精确位置 | 本次实际读取与版本 | 用途及尚未核实 |
|---|---|---|
| V02 [版本历史](https://jp-minerals.org/vesta/en/changes.html) | HTML与旧摘要相同；重读3.4.6至3.5.8相关条目，页首最新为2022-08-11/3.5.8；更早条目本次未逐条重读。 | 3.4.7选择保留、3.5.0多相CIF/透明PNG、3.5.1生成体数据外存、3.5.6位移默认/能量符号、3.5.8 B_ij解释与保存修复，均只作版本差异线索。 |
| V02 [下载页](https://jp-minerals.org/vesta/en/download.html)，development/macOS | 正文已读；标3.90.9a、Aug8 2026、Intel/Apple Silicon universal，开发版可视化准晶、计划未来4.0；stable仍3.5.8。 | 支持目标身份与开发分支差异，不能补出3.90完整变更日志或证明准晶行为已测；不下载替换目标。 |
| EXT-CIF1，Conformance/Version history | curl403；web通道读入口正文。CIF1.1工作规范2002-12-10批准、2003-02-24上网，含到2005-03-04编辑变更。 | 核入口版本及语法/语义分工；目标读取子集/字典版本尚未核。 |
| EXT-CIF1-SYNTAX，[File syntax](https://www.iucr.org/what-we-do/digital-standards/cif/cif1/file-syntax) ¶3–64 | 原复核读¶3–24；续作补¶25–64和Appendix A；curl403保留，正文来自web。 | block/loop/引号/分号/注释/形式语法及长度限制；不宣称目标语法符合性，字典与扩展仍待核。 |
| EXT-CIF1-SEMANTICS，[Common semantic features](https://www.iucr.org/what-we-do/digital-standards/cif/cif1/common-semantic-features) ¶1–38 | 原复核读¶1–25；续作补¶26–38，正文来自web。 | 字典约束、单位、s.u.、空值及折叠/编码约定；所用字典正文与目标实现仍待核。 |
| EXT-POSCAR，[官方正文](https://vasp.at/wiki/POSCAR#Full_format_specification) | 本次读完整格式及精度小节；页面 oldid=37632，编辑时间2026-07-22 14:50（网页未标时区）。 | 负体积缩放、三因子、元素行、坐标/Selective dynamics、速度和MD附段须区分。此现代规范不能证明目标支持全部字段。 |
| EXT-CHGCAR，[Format/Magnetic calculations](https://vasp.at/wiki/CHGCAR#Format) | 本次读正文，oldid=37892，编辑2026-09-14 07:29，晚于目标构建。 | NX最快/NZ最慢、PAW与磁化附块；页内文字说除FFT体积和晶胞体积，但公式写data/Vcell，归一化表述有歧义，先保留S05独立校核，不擅定额外Ngrid因子。 |
| EXT-XSF，[官方格式](http://www.xcrysden.org/doc/XSF.html) | 原S02 502/S05超时及独立复核TLS/web失败保留；续作HTTP两次200且摘要一致，正文/示例已读，页面修订2025-08-04，无数字格式版号。 | datagrid与bandgrid轴序不同；PRIMVEC/单位/动画及网格约束见SOURCE_SUPPLEMENT；目标支持子集与标量单位仍未核。 |
| EXT-FFAST，[NIST SRD66入口](https://www.nist.gov/pml/x-ray-form-factor-attenuation-and-scattering-tables) | 读入口说明，不是全部数据/论文。入口同时写数据内容更新Jan2005和页面末Aug2005。 | f1/f2、吸收/衰减、Z1–92用途已定位；不能据网页日期推定本地参数版本。 |
| EXT-FFAST-HISTORY，[版本历史](https://www.nist.gov/physical-measurement-laboratory/x-ray-form-factor-attenuation-and-scattering-tables-version-history) | 正文已读，version2.2/May2026；2.1/Jan2005；2.0/May2003。 | 区分后端/显示更新与数据内容变化。2.2不代表目标嵌入数据；1995/2000论文全文及具体表值仍未核。 |

[VM18外链登记](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/vm18-external-source-inventory.json)给45个唯一外链分配 `EXT-VM18-001`–`045`，逐条保留原URL、所在小节/链接上下文、用途、访问状态和版本待核字段。大多是产品/数据库入口，3个Wikipedia链接及STL误链的OBJ页是二手入口，不能统称一手规范。另有同站Dysnomia入口见资料索引。没有访问的目的页明确写“仅在VM18发现入口”，访问日期不伪装成目的页读取日期。

[书目条目登记](../../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/bibliography-entry-inventory.json)给135条分配 `EXT-BIB-001`–`135`，保留书目锚点、刊卷年页/版次和用途入口。原登记是**手册所印书目信息**；下表保留独立复核时的重点缺口，续作中ORFFE选读、TIDY/Waasmaier摘要与Tsirelson身份核对覆盖在SOURCE_SUPPLEMENT/阅读台账，不回写原135条历史登记。原文/公式/目标采用版本均未因此整体通过：

| 书目来源ID | 手册定位与用途 | 明确缺口 |
|---|---|---|
| EXT-BIB-010 / 030 / 050 / 055 | VM06/14：Sears，ITC C第3版(2004)pp444–452；ITC A第5版(2005)；Arnold同卷(2006)pp78–85；位移约束C(2004)p695。 | 引用年/版差别照录，尚未向出版物核定，未读这些章节正文。 |
| EXT-BIB-021 / 022 / 023 / 031–040 | VM02/08/11：Voronoi/成键/多面体/畸变/ECoN/电荷/价键；Baur B30:1195(1974)、Robinson Science172:567(1971)、Hoppe150:23(1979)、Brown/Altermatt B41:244(1985)等。 | 数值算法、书目疑误与参数版本均未独立核，不能从手册复述成已核算法。 |
| EXT-BIB-045 | VM14.2：ORFFE报告ORNL-TM-306(1964)。 | 是书目入口，程序版本、输入/协方差契约和报告正文未读。 |
| EXT-BIB-046 / 085 / 086 / 089 / 090 | VM14.3/.4、VM18：Gelato/Parthé JAC20:139(1987)，Parthé/Gelato A40:169(1984)，LAZY PULVERIX JAC10:73(1977)，约化书章(2009)pp447–450、(2011)pp84–86。 | TIDY本地版本/分支/阈值、约化规则正文与样本仍待核。 |
| EXT-BIB-012 / 108 | VM14.6/16.1/18：RIETAN-FP SSP130:15(2007)与RIETAN-2000 MSF321:198(2000)。 | 论文年代不是本地可执行件/模板版本；正文和配套绘图器文档未核。 |
| EXT-BIB-091 | VM14.5：Kato/Izumi，unpublished work(2008)。 | 不能称公开论文已读。MADEL用户手册、目标参数/运行完成缺口仍关联 `unk_239d524d3a4b9d25`。 |
| EXT-BIB-047 / 048 / 098 | VM14.15：ELEN相关Izumi JCSJ44:380(2002)，Tsirelson B58:632(2002)，Kirzhnits JETP5:64(1957)。 | 原文、离散导数、边界和负密度处理未核；仅手册展示公式。 |
| EXT-BIB-093 / 094 / 095 / 096 | VM14.7/.9：Berant69:281(1977)、Waasmaier/Kirfel A51:416(1995)、Chantler JPCRD24:71(1995)/29:597(2000)。 | 五高斯/色散/Thomson项与本地参数不能由入口确认；NIST入口并不覆盖Waasmaier算法全文。 |
| EXT-BIB-097 / 100 / 107 / 123 / 128 / 129 / 130–135 | VM18对应SHELX/CIF/PDB/X-Plor3.1(1993)/Gaussian03(2003)/09A.1(2009)/VASP/ABINIT/XCrySDen/QE。 | 外部版本、格式子集和目标兼容范围尚未逐项核；其余书目也不隐含已核全文。 |

## 证据效力与接续

原[来源记录](../../output/vesta-study/20261108-113622/s02/manual-source-provenance.json)、[187个文档候选](../../output/vesta-study/20261108-113622/s02/manual-feature-candidates.json)与五入口记录不修改。187项全是 documentation-discovery/runtime unverified，不能计作187功能通过、完整功能数或运行验证。

新获取完整HTML/提取文本只在本次 `/tmp` 阅读目录；没有整篇外部原文复制进项目，也不宣称临时目录是永久原始归档。永久证据保留获取时间、HTTP/TLS结果、HTML/文本独立摘要、标题/锚点/链接元数据、简短阅读笔记及差异裁定；它可核身份和定位，**不能在远端变更、临时文件丢失后恢复每篇原字节**。web通道CIF选读只存笔记，未取得可与curl摘要相比的HTML原字节。

S02.3 继续以新原生来源问题追踪；版本身份已解决的 `unk_42e315593eabb9ce` 不重开。S04–S07仍分别负责当前版本入口、格式语义、科学规则和外部执行；不把S02阅读当这些步骤已完成。

## S05 继续读取的材料

已重新核 VM18 的支持格式、pgrid/ggrid 布局、工程相对引用及 MXDORTO 条目，VM17 的 `-nogui/-i/-save` 契约；并读 D08 的文件兼容与保存恢复正文。资料与实际结果对照在 FORMATS、SAMPLES、行为/05；不以旧格式清单代替当前实测。

EXT-EXAMPLES 官方 ZIP 已真实下载，摘要及成员记录保留在 s05，部分样本已用原件读取。S05当时XSF正文超时的记录保留；续作已用官方HTTP补读。官方 KOD 工程引用与网格单开的差异已复现，不能由官方样本身份推定当前版本往返通过。

## R29选定公式与目标样本绑定

重新获取[VM11 11.4.3](https://jp-minerals.org/vesta/en/doc/VESTAch11.html)，HTTP200、40498字节/rawSHA dba178c0928e8439010009322a50a20853fe2cc2773dad91828246bbe553f8dc，与S02相同。永久只保留数学公式/身份元数据，未复制外部全文。[公式元数据](../../output/vesta-study/20261108-113622/reviews/20261108-232231-s06-distorted-octahedron/manual-formula-source.json)。独立按手册公式先验计算轴向畸变样本，目标五值显示一致，见[ALGORITHMS](ALGORITHMS.md)；有限目标一致性不代替Robinson/Hoppe原论文全文、全部构型或源规范通过，既有来源问题rev8 investigating不改。

## R31 VM11不确定度选读

继续读取同SHA缓存VM11中选点距离/角/扭转SUE说明，明确只对角估计/忽略未提供的非对角协方差，不替代引用70原论文全文。独立解析传播、实现表示控制与原件/同TEXT现场读数分别见[ALGORITHMS](ALGORITHMS.md)，不更新来源问题rev8或扩大为全SUE证明。

## R32密度归一化与格式来源边界

选读VM14 14.7/14.9结构因子占位/逆Fourier1/V、14.14峰积分和VM18周期/一般网格。仅存哈希/范围与独立公式期望，无外部全文复制。Gaussian cubegen已有[来源选读](SOURCE_SUPPLEMENT.md)的一般原子单位、N1慢/N3快规则；本次刷新timeout、不重复访问/不冒称新全文。CUBE单位具体问题另有原生unk_3f5d08b5dc777b37，原广义来源问题rev8不改，未用替代来源取消原厂规范要求。

## R34独立散射系数资料

新增EXT-DABAX-WAAS，读取OASYS/DABAX维护库f0_WaasKirf.dat的中性C/O两行/五高斯式/列顺序，HTTP200、42473bytes、完整哈希与临时全文路径永久记录，项目仅两行科学数值/元数据。ESRF文件webtimeout、未重复下载；维护数据是独立数值对照的一手资料，不替代Waasmaier/Kirfel1995原文，旧来源问题rev8 investigating不改。[资料索引](../VESTA资料索引.md)及[ALGORITHMS](ALGORITHMS.md)保留C b3末位差和有限绑定范围。

R35新增EXT-IUCR-ADP-CIF/EXT-IUCR-INTENSITY，仅选读IUCr coreCIF定义和Dinnebier/Scardi(2023)§3–4式9/12/14/15/17；B=8π²U、exp(−Bs²)及占位乘入复数和用于独立控制，未复制全文，未宣称全篇/全部ADP通过。两一手直接链接见[资料索引](../VESTA资料索引.md)，DABAX系数沿用R34既有哈希，无重复下载。旧来源问题rev8 investigating保持。

R37新增EXT-NIST-NEUTRON，仅NIST两页面表头/单位与自然H/C/O数值选读。历史页明示1992来源；另一重定向表数值不同、日期未建立，分别预登记、保留28输出字段差值。web成功与raw403分开，不以三元素相等推出原包整表来源/版本，旧来源问题rev8保持；直接链接见[资料索引](../VESTA资料索引.md)。

R38沿EXT-IUCR-INTENSITY另选读§7.1–7.2：区分Laue粉末multiplicity与本程序表中P1 M1/本例P21 M2；式29面积Lorentz与32非偏振P支持事后强度模型，式36逐点因子不同，未宣称整篇或所有几何域通过。见[ALGORITHMS](ALGORITHMS.md)。

R40重新对照已缓存VM14.8/.9的8列输入、唯一反射、F000/分辨率及逆Fourier负指数式；未取得新原论文全文。单模解析Oracle是独立数学推导，648实际体素验证与pocketfft静态调用分开，不宣称核版本或全部FFT通过。见[ALGORITHMS](ALGORITHMS.md)。

R41按既有VM14.11 Patterson定义及FromVolumetricData保留分辨率描述，另以解析谐波和全216点直接周期卷积作独立Oracle；没有取得新原论文全文。参见[ALGORITHMS](ALGORITHMS.md)。

R41来源限定：VM14.11的FromVolumetricData离散F式印作Σρexp、未显式写dV；本轮事先采用物理自相关积分/直接卷积Oracle，原件静态实际先乘det/N、末除det，数值符合该带体元归一化合同。这里记录手册省略与实际算子的区别，不称逐字符照搬手册离散式。

R42按VM14.7带dV的离散正向积分先验，原件VolumetricData分支实测；手册“其它选项不影响”尚未作控变验收，kernel版本/完整三维/轴顺序仍待。见[ALGORITHMS](ALGORITHMS.md)。

R45对照既有VM14.13 lineprofile的分数坐标、等步长、线性插值和两列distanceÅ/scalar合同，选定13点运行符合。8角tensor-product推导独立于目标静态实现，静态调度及格式另留；没有新原论文全文。

R47读取既有缓存VM15 §15.1–15.7，selectedtwo-vector/linearinterp/Nvector公式及Export2D合同。手册Nx点数用语在本例实际导出为6区间/7闭合样本，先验候选歧义保留再核；projection/boundingbox/所有渲染未验。

R48对VM15.5.3“累计切片之和”未明确的端点/权重列五候选全部否定，随后分离full首值产生假设/half预先留出49点验证和staticGenPeriodic解释；不将手册简述冒作精确物理积分来源。selectedapprox与piecewise-linear精确积分不符的数值差异保留。

R56以原脚本和实际临时文件纠正旧stdin措辞为filename .stin argv/stdout重定向；不是引入新版本来源。三案原工具实际可用，子退出码未直接测量。见[SOURCE_SUPPLEMENT](SOURCE_SUPPLEMENT.md)和[结构规格](行为/02-结构与几何.md)R56。


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


## S02 专项接续 R133（2026-11-10）

用户要求继续直至 S02 完成。本轮只做来源补证与验收，不执行 VESTA/科学计算程序。锁定工具和原件身份不变；[台账](../../output/vesta-study/20261108-113622/reviews/20261110-110000-s02-source-closure/source-reading-ledger.json)为 8 条记录、7 处指定正文范围和 1 处出版社版次定位。不是 8 份全文、原件样本或功能通过。

| 来源与实际范围 | 准确版次/用途与限制 |
|---|---|
| EXT-MXDORTO-MANUAL101；[NIMS 手册](https://www.nims.go.jp/personal/mxdorto-mxdtricl/doc/mxdorto_manual_JP_1-0-1.pdf) 封面、§3.2 PDF11–14、§5标题 | 手册 1.0.1/17页，未标封面日期。file07 定列、物种范围、分数坐标、晶胞长度/角余弦已选读，PDF13–14目视；§3.2仍标编写中，§5输出标准备中。不是 FILE06 完整说明或目标读取验收。 |
| EXT-MXDORTO-SOURCE20251120 / EXT-MXDTRICL-SOURCE20251119；[ORTHO](https://www.nims.go.jp/personal/mxdorto-mxdtricl/src/mxdorto.zip) / [TRICL](https://www.nims.go.jp/personal/mxdorto-mxdtricl/src/mxdtricl.zip) serial 的指定 writer/FORMAT | 下载页标20251119，ORTHO源码内标2025-11-20-00、TRICL为2025-11-19-00。两者头部/BIN位置和H矩阵附记录不同。只读台账所列范围，未读MPI/完整FILE06、未编译运行。旧TLS失败保留，但不再说官方正文未得。 |
| EXT-BIB-030/050；[官方版次](https://www.iucr.org/who-we-are/commissions/commission-on-international-tables/editions-and-editors)、[Arnold §5.1.3](https://onlinelibrary.wiley.com/iucr/itc/Ab/ch5o1v0001/sec5o1o3/)、[大学托管原书](https://ncm.unn.ru/files/2021/01/International-Tables-for-crystallography.pdf) | A卷第五版2002、修订重印2005、首次在线2006。取得PDF910页，前置页印2005、章节页印2006，二者均保留。读身份页及§5.1印78–79/85指定段，目视78–79；换基/坐标/原点变换约定已核，不声称全卷/全部矩阵表已读。 |
| EXT-BIB-010；[Sears §4.4.4](https://onlinelibrary.wiley.com/iucr/itc/Cb/ch4o4v0001/sec4o4o4/)、[§4.4.4.6](https://onlinelibrary.wiley.com/iucr/itc/Cb/ch4o4v0001/sec4o4o4o6/) | C卷第三版2004/首次在线2006。只读引言、表头/H/C/O例和§4.4.4.6叙述；fm/barn、相干/非相干/吸收量区别及编表说明已核。书目444–452与在线节444–454不合并。不是全表比对；web正文没有成功原HTML摘要。 |
| EXT-BROWN-ITCC2021；[Brown 后续正式章节](https://onlinelibrary.wiley.com/iucr/itc/Cc/wf5155/) ch1.1 | 2021，DOI10.1107/S1574870720003110。读§1、§2公式1–7及叙述/表题、§3开头至Table4标题；同作者正式后续来源给指数价键式和拟合参数概念。1985原论文仍未读，完整参数表与本地选参仍待S06；不据此确认所有历史拟合细节。 |
| EXT-ITC-EDITIONS；官方A/C版次页、目录 | 核版本/章节定位；Prince/Finger/Konnert §8.3.1及表8.3.1.1印695–696已定位，但正文请求失败，不标ADP约束表已读。 |
| EXT-DVXA-BULLETIN33；[坂根2012原报告](https://www.center.ous.ac.jp/pdf/bulletin/bulletin_33.pdf) §3/3-1指定程序清单 | OUS Bulletin33/2012-03；d3d26n内标2012-02-16 Version1.0。PDF6/9/10（印3/6/7）逐页目视：unit8写F01的Z/NEQ/笛卡尔坐标，FORMAT 2X,I3,1X,I4,3(F10.5)，Unit标志0=Å、1=atomic，本例写0。unit7的3D20.10,I5写d3d26n.out，不能误当F01。仅八原子生成器实例，不是全部SCAT/目标契约。 |

[dsh首轮缺口复核](../../output/vesta-study/20261108-113622/reviews/20261110-110000-s02-source-closure/dsh-gap-review.stdout)判S02.3未通过；[单独的标准澄清](../../output/vesta-study/20261108-113622/reviews/20261110-110000-s02-source-closure/dsh-criteria-clarification.stdout)纠正了原厂商完整规范/精确历史版/全文的额外要求和I28/STL编号混淆。计划要求准确版本及相关章节，没有要求全部135篇书目全文。固定消费者实现、新版生产者文档、同作者正式后续专业章节可按其真实身份作为相关来源，不能冒充原厂商规范、旧论文已读或目标兼容。

据此，I05/OpenBabel2.3.1、I08/CSSR消费者、I15–16/ICSD转录、I30/XTL消费者及I42/XPLOR3.12已有相关来源；原始规范/旧版差异依然保留。MXDORTO“只有失效入口”已补。Brown2021补专业入口，原1985正文未读不回改。本轮F01新材料在上述dsh澄清之后取得，不宣称已获dsh内容签核。

**S02仍未完成。** 下一轮继续补VICS/AMCSD/ASSE、PRIMA pri/prim/den、eb、wien2venus三维rho、fou/SCA，以及选定几何/标准化/散射专业正文。先寻找公开作者稿、机构存档和能明确覆盖所需范围的版本化专业替代来源；不以摘要或同后缀替代。MINCRYST额外字段、MOLDA单位与ITC位移约束按实际相关范围逐项收窄，不凭“全文未读”泛化为全部阻断。用户没有机构访问入口并已要求自行解决，不再索要该入口。

[原生更新](../../output/vesta-study/20261108-113622/reviews/20261110-110000-s02-source-closure/source-unknown-update.stdout)及[读回](../../output/vesta-study/20261108-113622/reviews/20261110-110000-s02-source-closure/source-unknowns-after.stdout)确认来源问题revision9/high/investigating；其他11头逐项不变，evidence_for/against仍为空。最新完整原件证据快照仍为R132的1640条；本轮没有新增/重新核完整原件记录。原文与阅读页只在/tmp，永久只留[获取记录](../../output/vesta-study/20261108-113622/reviews/20261110-110000-s02-source-closure/acquisitions.json)、身份、范围、笔记、复核和差异；远端/缓存失效时不可由摘要重建原字节。


## R134 公开历史资料补证（2026-11-10）

用户没有机构订阅，要求自行解决。本轮通过原作者网页的 Internet Archive 存档、原发布包、作者/研究机构托管原印本及固定消费者源码补读。完整外文仅放临时目录，不复制到工程；[主台账](../../output/vesta-study/20261108-113622/reviews/20261110-041200-s02-public-professional-sources/source-reading-ledger.json)与[补充台账](../../output/vesta-study/20261108-113622/reviews/20261110-041200-s02-public-professional-sources/source-reading-addendum.json)给准确身份、访问日期、实际章节、字节摘要及局限。读取资料不增加原件样本、功能或 CA evidence。

| 关联范围 | 可定位的新正文 |
|---|---|
| I02 VICS；I31–32 PRIMA/MEM | EXT-VICS-VEND-MANUAL20060130：原发布包内89页手册，PDF创建2006-01-30，读VICS附录A印46–51、VEND附录A印81–86；EXT-PRIMA-MANUAL20051021：13页，读§3.3/§8印4/11–12及身份，非2010手册。EXT-VENUS-EXAMPLES20040727：原例包中111行vcs通读、两份den仅前24记录。原例包实际2004-07-27，不能套2005网页列的新版日期。 |
| I03 AMCSD | EXT-AMCSD-PAPER2003：Downs/Hall-Wallace原论文88:247–250，作者站历史副本；读印247全页/图1及248的U/b承接句。记录由标识/书目/注释和晶体数据两部分组成，非标准原点星号/平移、右对齐列头/原子与分子命名、占位及位移量已有原作者正文。2010帮助页只有amc/cif选项，不能算字段规范。 |
| I04 ASSE | EXT-ASSE05-AUTHORARCHIVE：0.5原包内标/mtime2003-07-09；读原作者2003首页、输入章节与指定section/lattice/atoms读写代码。EUC-JP正确解码，begin/end及abc/xyz/prm分开；未确定标量长度单位、未编译运行。0.5的eb列表/VTK输出与VEND三维eb分开。 |
| I28/I39 F01/C04D/SCA | EXT-CONTRD-README20050202全文、EXT-CONTRD-SOURCE20040202指定input/writer/FORMAT，水野原说明/泉修订来自原发布包。补齐头3I5+3F10.5和x最快/y行/z组、nx≥72时10E14.6；单位由旧VEND手册印83明确为bohr^-3、Ry、bohr^-3/2。Readme的X5、重复节号和源文件16I5读dummy的差别保留，不将作者尺寸上限当目标上限。 |
| I40 wien2venus三维rho | EXT-WIEN2VENUS-20111124：原作者脚本2011-11-24，2015存档；全静态读脚本及作者说明。cell/晶胞/角/网格头与k最快的作者说明已核；脚本原样收集lapw5平面，未独立验证所有非立方分支，不运行转换。旧VEND手册明确该rho密度用bohr^-3，不能与二维LAPW5同后缀合并。 |
| I33 eb | 原生产者VICS/VEND手册A.1 ID9（印85）正文选读：近似其三维rho布局、能量整体平移与等值面要考虑该平移。Arai0.5中的同名eb实际能带列表/VTK，不用来填充三维字段；能量单位、分带/偏移存储及目标读取另待核。 |
| I41 fou | EXT-MCE-SVN145-FOU：官方SVN145固定IO.cpp，ReadFouFile438–800通读，补矩阵/平移/晶胞弧度/网格和z外/x中/y内、原子记录变体等消费者字段。与已有WinGX→MCE集成说明关联，非WinGX完整原厂商规范，不合并mpf、JANA fou或其它GRD。 |
| CNS/XPLOR | EXT-CNS11-MAP：Yale官方1.1语法手册，读write/read map完整命令页，1999-12-08原修改/2005存档；ASCII默认/可选二进制、real/imag、缩放和Å/分数盒界。EXT-CHIMERAX19-CNS-READER：官方v1.9提交7048c533…，109行静态通读，明确CNS/XPLOR ASCII消费者。ZYX与-9999只是注释，代码跳过这两行而不校验；不能冒充强制格式检查、CNS1.3或商业CNX兼容证明。 |

科学补读见[SOURCE_SCIENCE_PARAMETERS](SOURCE_SCIENCE_PARAMETERS.md)R134。Fleet公开首论文页、Hoppe共同署名2001原文、原作者Waasmaier README、Chantler2000原论文、ITC H2019及IUCr2005指定正文为专业来源。保留1971/1979/1987/1995旧论文未读与1996完整勘误未得；同字节OASYS/ESRF文件不是两个独立数值数据集。

S02.3是否达到原标准由本轮dsh独立复核与实际收尾验证裁定；此处正文摘要不自行宣布通过。格式/算法完整契约、本地嵌入版本、所有参数和科学正确性仍由S04–S07继续核。历史HTTP/TLS/缺页、构造错误与其修正保留，不用成功获取覆盖原失败记录。

R134后续补证：EXT-F01-INPUT2014为坂根原报告35（2014-03）§6印15/PDF19的完整MAKEF05输入示例；头/原子记录/Unit标志已目视，见补充台账。CNS补充与本项共3记录，主台账18记录中15项有本地原字节、3项仅web正文且不编造哈希。MCE Last-Modified原回执实际在acquisition-metadata/acquisition03.json；dsh首轮报告中的未留痕疑问以该实际元数据澄清。

R134补充来源最终为4项（CNS两项、F01输入一项、MOLDA原作者6.5手册一项），连同18项主台账共22项指定正文范围。MOLDA准确程序版本和G30/G60已恢复；坐标单位仍未由本次章节给出。保留来源边界，待dsh对最终材料按原计划统一裁定。

R134 [dsh最终来源复核](../../output/vesta-study/20261108-113622/reviews/20261110-041200-s02-public-professional-sources/dsh-final-source-review.stdout)已实际exit0，明确22项身份/范围/限制可信，来源阶段达到原计划、无剩余硬阻断。它纠正了将I19单位与CNS1.3精确版另作入口条件的不一致；MOLDA/ASSE/eb单位、完整字节/商业CNX和Chantler完整旧勘误不因此已知，留S05/S06。原负面报告与修订依据保留。来源未知已实际9→10、其它11头未变；待专项完整性验收后才勾选S02.3。

R134收尾专项验收实际exit0：133历史manifest/44696文件、35实际预编辑副本、22来源/9目视页、12问题头/精确9→10更新、47输入/23输出关联及原件/工具身份检查通过。此后勾选S02.3，S02三项完成、计划11/18；本阶段只验资料入口/版本/章节，不声称全部书目全文、所有数值或目标科学行为已通过。

## S06 本卡来源复用

[范围台账](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/source-reading-ledger.json)重核VM05–VM11/VM14/VM15/VM18对应章节的S02准确raw SHA，继续使用2022-08-10手册；本轮浏览再次打开官方VM14，未复制全文到项目。新核/Patterson依据§14.10–14.11和R37已有历史NIST选定H/O，现代版本差异保留。CA06用锁定git对象；D04/D05/D11用于验证而非晶体学来源。原件最新3.90.9a不由旧手册代表；未新称未读论文/全表已读。
