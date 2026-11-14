# S02.3 来源补读（2026-11-08）

**当前任务状态（2026-11-10）：** S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 来源unknown revision10/high/investigating保留目标绑定边界；下文旧 revision/未勾选续记为历史，来源阅读不等于能力通过。接续见 [HANDOFF](HANDOFF.md)，阅读边界见 [SOURCE_NOTES](SOURCE_NOTES.md)。

本页保留第一轮补证的阅读范围与当时接续清单；最新进展见[格式与科学来源续读](SOURCE_FORMATS_SCIENCE.md)和[完整覆盖表](FORMAT_SOURCE_COVERAGE.md)。来源问题当前revision3，原第一轮台账/回执不回写；SHELX/MDL/X-PLOR/STL/Chem3D及Baur的新增已读范围以后续记录为准。

承接 [SOURCE_NOTES](SOURCE_NOTES.md) 的独立复核，不改写旧获取记录或降低 S02.3 验收标准。本轮只研究资料、只读本地附带资源并更新调查问题；没有运行外部科学程序或把资料阅读登记为功能通过。

本轮[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-133154-s02-sources/)保留 4 批获取元数据、逐来源阅读台账、资源身份、原生更新与文档差异。外部完整 HTML/PDF/ZIP 和提取文本只在 `/tmp/20261108-133154-s02-sources-bodies/`；永久记录不能在临时文件丢失后恢复原字节。HTTP 错误页摘要不当文献摘要；web 阅读通道没有原始 HTML 字节摘要。

## 格式规范

| 来源 ID / 直接位置 | 已读范围与版本 | 对后续研究的约束 |
|---|---|---|
| EXT-XSF，[XCrySDen 官方格式](http://www.xcrysden.org/doc/XSF.html) | HTTP 两次 200、原字节摘要一致；全部正文及示例已读。正文修订时间 2025-08-04，无数字格式版号；文中 1.4 指注释功能引入的 XCrySDen 版本。HTTPS 两次 TLS 失败仍保留。 | 坐标 Å、力 Hartree/Å；`PRIMVEC` 拼写与 VM18 的 `PRIMEVEC` 不同。一般 datagrid 包含边界、x 最快/Fortran 次序；bandgrid 为倒易原胞一般网格、z 最快/C 次序，不能混用。AXSF 区分固定/可变晶胞；未由坐标单位推断标量密度单位，也未证明目标全部支持。 |
| EXT-PDB33，[简介](https://www.wwpdb.org/documentation/file-format-content/format33/sect1.html)、[晶胞/变换](https://www.wwpdb.org/documentation/file-format-content/format33/sect8.html)、[坐标](https://www.wwpdb.org/documentation/file-format-content/format33/sect9.html)、[连接](https://www.wwpdb.org/documentation/file-format-content/format33/sect10.html) | 四页正文/示例已读，Atomic Coordinate Entry Format 3.3/3.30；简介给 2011-07-13 初版、2012-11-21 更新。其他 PDB 章节和字典未读。 | 80 列固定字段，坐标 Å；altLoc、占位、MODEL/ENDMDL、TER 不能丢成同一概念。ANISOU 为同坐标系的 U 张量整数、按 10⁴ 缩放；CRYST1、SCALE、ORIGX 与非晶体占位晶胞需区别。CONECT 不包含所有隐含标准连接或缺坐标原子。 |
| EXT-CUBE，[Gaussian cubegen](https://gaussian.com/cubegen/) | 只读 Output File Formats / Reading Cube Files；页面标 Gaussian 16 Rev. C.01、2023-08-03。没有把它当 Gaussian03/09 原手册全文。 | 一般 cube 原子单位；任意点输入 `npts=-5` 的坐标例外为 Å。网格 N1 最慢/N3 最快；NVal 多值与负原子数 MO 附记录须分别核，不能以单值例子概括。 |
| EXT-DLPOLY，[DL_POLY CONFIG](https://ccp5.gitlab.io/dl-poly/UserManual/data_files/input_files.html) | 只读 The CONFIG File 至 Further Comments；网站为 DL_POLY 5.3.0，不是目标使用版本证明。 | 自由格式且区分大小写；levcfg 控制坐标/速度/力，imcon 控制晶胞记录及周期边界；长度 Å、速度 Å/ps，REVCON 同格式。不能把当前文档的实现限制套成 VESTA 限制。 |
| EXT-OPENBABEL-XYZ，[Open Babel XYZ](https://openbabel.org/docs/FileFormats/XYZ_cartesian_coordinates_format.html) | 格式正文已读，当前站标 openbabel-3-2-0；旧手册外链是 2.3.1，版本分开。 | 文档明确 XYZ 无已发布正式规范；计数/标题/元素与 Å 坐标、多帧是该实现说明，不能声称存在统一严格 XYZ 标准。 |
| EXT-OPENBABEL-CC1，[Chem3D Cartesian 1](https://openbabel.org/docs/FileFormats/Chem3D_Cartesian_1_format.html) | 页面已读，但只有标题/导航，没有字段规范。 | 不能据 HTTP200 或格式名称宣称 Chem3D `.cc1` 规范已核；与 c3d1 的对应也仍待验证。 |
| EXT-ZEOPP-CSSR，[Zeo++ 输入页](http://www.zeoplusplus.org/input.html#cssr) | 页面正文/示例已读；未标格式版号，响应 Last-Modified 为 2017-06-20。 | 这是使用者程序的输入约定，非原始 CSSR 规范。说明分数坐标、完整晶胞及忽略字段；另称 Open Babel 变体用 `.obcssr`。Zeo++ 的 P1 要求不能套成 VESTA 要求。 |
| EXT-VRML97，[ISO/IEC 14772-1:1997](https://www.web3d.org/documents/specifications/14772/V2.0/index.html)、[Annex A](https://www.web3d.org/documents/specifications/14772/V2.0/part1/grammar.html)、[节点](https://www.web3d.org/documents/specifications/14772/V2.0/part1/nodesRef.html) | 目录、Annex A 全文，以及 §6.23/.24/.27/.41/.52/.53 已读。未通读其余节点、概念/光照公式或 Part2 EAI。 | -1 分隔面/线、颜色/法线索引与每顶点开关、Material transparency=1 为全透明、Transform 次序均有规范约束；VRML97 不代表 VESTA 的实际导出版本与节点子集。 |
| EXT-CIF1-SYNTAX / SEMANTICS，[语法](https://www.iucr.org/what-we-do/digital-standards/cif/cif1/file-syntax)、[语义](https://www.iucr.org/what-we-do/digital-standards/cif/cif1/common-semantic-features) | 续读语法 ¶25–64（含 Appendix A）及语义 ¶26–38；与前轮 ¶3–24 / ¶1–25 连续。curl 仍403，正文来自 web 通道。 | 1.1 行长2048、名称75、单层 loop；global_ 禁用，save frame 限字典；引号/分号/注释有上下文条件。长行折叠有专门标记，不能随意删反斜杠；DDL1/DDL2 字典声明、CIF 标记与 Unicode 不是一个层次。尚未读所用字典全文、CIF2/磁性扩展或证明目标符合性。 |

## 外部程序与论文

旧 RIETAN URL `fujioizumi.verse.jp/download/download_Eng.html` 本轮 301 至无关域名后超时，不能继续当权威下载入口。[JP-Minerals 恢复页](https://jp-minerals.org/rietan/)的维护者说明日期为 2023-03-18，列 RIETAN-FP 3.12 与 2021-12-20 文档包；恢复页正文已读。下载包身份不证明本地配置或程序使用同一版本。

| 来源 ID | 已读精确范围 | 已核内容与边界 |
|---|---|---|
| EXT-RIETAN-DOCUMENTS / MANUAL，[官方恢复文档包](https://jp-minerals.org/rietan/documents.zip) 中 `RIETAN-FP_manual.pdf` | 234页，封面 2021-12-16；封面/引言，§17.1–.2、§17.3.1–.15、§17.3.16开头，§17.7.2–.7、§17.8.1/.9，Appendix D 全文（PDF209–212/印刷208–211）；另读PDF184–185的gnuplot片段，未读完整§17.7.1。 | 18 个位置文件参数；Tink 输入、编码和宏限制与晶体设置。gnuplot `.gpd` 分块、残差由 LRES 控制，不能固定理解为简单差值。原子散射/轮廓科学公式未全读，不能当234页全文或全算法核验。 |
| EXT-RIETAN-COOP，同包 `RIETAN_VESTA_cooperation.pdf` | 38页、封面 2021-12-16；PDF1–7（§2.4至该页）、PDF20（§3.1及§3.2开头）。 | TIDY 标准设置、标签6字符及必要数据文件；RIETAN 输入模板须匹配实验类型/参数。ORFFE 产物与 `.ins/.lst` 联动；外部文档的调用方式与本地脚本仍分别登记。 |
| EXT-MADEL-MANUAL，同包 `MADEL.pdf` | 8页全文；英文部分注明 1991-03 翻译/修订，日文附录 2021-12-16 标 MADEL 2.1。 | 固定列 `.pme`、RADIUS Å / REGION Å⁻¹、占位加权电荷与能量求和/晶胞倍数；输出以 e/Å 转 V。两张关键页实际渲染检查：附录物理第6页印刷光速指数为10²³，下一页用10⁸，属来源内部印刷矛盾，不能照抄。附录推荐无部分占位超胞，不等于已证明目标处理占位的方式；本地 madel 版本仍未知。 |
| EXT-BIB-045，[ORNL-TM-306 原报告](https://www.iucr.org/__data/iucr/software_manuals/ORFFE_ORNL-TM306.pdf) | 1964，web可读91页PDF；读物理6–13、19–23、31页，覆盖摘要/数值求导和误差传播、指令/前三种几何函数及错误指示。没有读全部程序清单。curl两种协议均403。 | 原版指令 I3、原子编码100C+S；RIETAN 附录是修改版 I5/1000C+S，不能互换。原版有完整协方差项不证明 VESTA GUI 也这样传播；`***NG` 是计算失败指示。 |
| EXT-BIB-046，[Gelato/Parthé 1987](https://journals.iucr.org/paper?buy=yes&cnor=a27733) | 出版社元数据/摘要：J. Appl. Cryst.20,139–143，DOI10.1107/S0021889887086965；付费正文未读。 | 只支持标准化目的和来源身份；完整算法及本地 TIDY 版本待核。 |
| EXT-BIB-094，[Waasmaier/Kirfel 1995](https://journals.iucr.org/paper?buy=yes&cnor=sh0059) | 出版社元数据/摘要：Acta Cryst.A51,416–431，DOI10.1107/S0108767394013292；付费正文未读。 | 摘要给五高斯加常数、0–6 Å⁻¹；未据摘要验证全部参数、本地 asfdc 来源或电子密度实现。 |
| EXT-BIB-048，[Tsirelson 2002](https://doi.org/10.1107/S0108768102005517) | 出版社目录与其 Crossref 元数据核身份：Acta Cryst.B58,632–639，正文未读。 | 手册起页632正确。曾误把相邻 CSDSymmetry 的640–646归到本文，随后以题名/DOI/文章链接排除；没有据错误猜测改正文。身份核定不算密度转换算法已核。 |
| EXT-DYSNOMIA / MANUAL，[程序页](http://jp-minerals.org/dysnomia/en/)、[手册](https://jp-minerals.org/dysnomia/archives/Dysnomia_Manual.pdf) | 程序页全文含日志/许可，版本1.0构建2018-12-08；手册封面2014-12-02，实际读封面/目录/许可、Ch4–5（印刷11–16、PDF14–19）；Ch2算法未登记为完整阅读。 | pgrid 输出、不输出 den；重启与非均匀先验不同；`.prf` 命令行入口。程序页记1.0修复P1 pgrid格式，旧手册不覆盖该修复；未安装/运行/再分发。 |
| EXT-VESTA-PDF2019，[官方PDF](https://jp-minerals.org/vesta/archives/VESTA_Manual.pdf) | 177页，仅查封面/元数据/目录定位，封面2019-01-27；两个官方主机响应摘要相等。 | 与2022-08-10 HTML版分开。初次下载队列误标 EXT-MADEL-MANUAL，获取原记录不改；阅读台账明确更正为本ID，MADEL另来自RIETAN包。 |

`New_features.pdf` 只读封面、引言及§2开头；`Readme_Mac.pdf` 仅提取/核身份，不登记正文已读。没有因为解包成功把同包20个文件计为全部已读。

## 本地资源对照与未完成项

对原件 Resources 中9个文件做了只读身份登记，未执行二进制。[身份表](../../output/vesta-study/20261108-113622/reviews/20261108-133154-s02-sources/bundled-resource-identities.json)固定路径、字节数、SHA-256和文件种类。`RIETAN.sh`/`STRUCTURE_TIDY.sh` 全文读过：前者向用户指定 RIETAN 可执行件传18个文件参数并处理绘图/查看器，后者在样本目录以 .stin 文件名参数调用附带 TIDY，并把 stdout 重定向到临时 .sto（R56纠正输入协议措辞）；这不是调用成功证明。`madel`/`structure_tidy` 只识别 x86_64 Mach-O，未由架构或文档版本冒充二进制版本。

`bvparm2020.cif` 读头部、全部数据标签和更新记录，数据块为2020-11-25、声明 cif_core.dic 2.2；R0/B 为Å，存在多条参数、可靠性顺序和 unchecked 标记。未核全部表值和目标选参逻辑。`template.ins` 只读头部/版本引用，注释还提 bvparm2013，不能证明实际调用2013；`asfdc` 只读前30行，不宣称完整参数验证。

S02.3仍为 **[ ]/部分完成**。来源问题保持 high，更新为 investigating；evidence_for/against为空，84条原生evidence及原8个问题不变。待办按用途推进：

1. 对本地实际输入/输出及47类旧手册输入逐项补来源，优先 SHELX、MDL Mol、X-PLOR/CNX、STL 与 Chem3D；CSSR 当前只有使用者说明，MXDORTO 官方旧页TLS失败。其余不能由已读几种格式概括。
2. 补核心几何/科学依据：Baur/Robinson/Hoppe、Brown/Altermatt、原子散射/色散、Tsirelson及ITC相关章节正文；付费摘要、Crossref元数据和135条书目不等于原文全读。FFAST猜测路径404保留，不替代已核NIST入口。
3. 外部程序剩余章节与目标绑定版本分别核；本地实际调用/CWD/完成、MADEL输出缺口继续原 `unk_239d524d3a4b9d25`，不重开已resolved的About身份问题。

当前只是资料证据的边界收窄；S04–S07入口、格式保真、科学数值和运行联动仍各自需要原件证据。

R56补充：原附带TIDY在本机三案实际输出标准化/约化报告；标准化应用、Niggli仅报告，外部子退出码未单独捕获。源脚本身份和选定二进制条件导入均记录于[ALGORITHMS](ALGORITHMS.md)R56，原件保持只读。

R57以锁定本地原件构造/事件初始化/Mach-O常量确认Peak Search与Voronoi菜单178/179共用OnPkSearch；旧手册14.14有峰积分，但没有以该手册替代本地新增菜单绑定证据。见[ALGORITHMS](ALGORITHMS.md)R57。

R59以原件arm64宽字符串、菜单ID185、事件182..186范围及R57缓存的4bytes/ret处理体绑定“逐多面体积分”为当前分发空处理；不是从旧手册推定数值实现。其它架构/版本未验，见[ALGORITHMS](ALGORITHMS.md)R59。

## R75 粉末峰形一手依据补充

[Ida作者托管1998全文](https://www.crl.nitech.ac.jp/~ida/research/reprints/efficient_convolution.pdf)三页文本已读，前两页本地渲染目视；采用Eq1/4直接积分，Eq15符号疑点未使用。PDF仅临时目录保存，不复制全文。 [ITC2019第3.3章](https://onlinelibrary.wiley.com/iucr/itc/Ha/ch3o3v0001/)读至§3.3.2.4，用于TCH/不对称峰形依据，后续TOF未声称已读。[TCH1987](https://onlinelibrary.wiley.com/doi/abs/10.1107/S0021889887087090)只读摘要，不以摘要冒充公式全文。新增来源仍不代表来源全覆盖、许可证已解决或完整算法验收；source未知头rev8保持。二进制默认输入与独立积分范围见ALGORITHMS R75。

## R79 ORFFE读取与计算边界

重读[VM14§14.2](https://jp-minerals.org/vesta/en/doc/VESTAch14.html)：描述.ffe结果读取、选择联动与自动侧文件发现，外部ORFFE协方差计算与VESTA几何估算须区分。并核[RIETAN恢复页](https://jp-minerals.org/rietan/)概览，未下载安装执行系统或改许可。R79只用人工数据检验本版查看器；不把本文档宣称的完整协方差传播当目标本机实测，原EXT-BIB-045的选读边界仍保持。


## R81 磁矩反演与时间反演（2026-11-09）

新增VS0074/75：10Å立方，Fe1(.1,.2,.3)，磁矩(1,2,3)μB；两个完整操作只在反演的时间符号±1上不同。首CLI前登记 [独立期望](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-before-targets.json)。一手Shinohara2023 §2.4 Eq7给m′=εdet(W)AW A⁻¹m；本案A=10I、W=−I，故普通反演保持磁矩、带时间反演则变号；magCIF字典只选读坐标/单位和xyz时间符号，全文阅读未完成。

原版两个-nogui导入/Save正常255，完整watch；实际MAGNETIC_CRYSTAL、SYMOP时间列±1、单site分配均保留。VECTR保存Cartesian(1,2,3)，末列type0/Axial（R83修正原坐标模式解释），原先预测晶格(.1,.2,.3)失败并留原计划；没有把表示差异当科学失败。原件自行识别2.4/P−1和2.6/P−1′，这些编号是目标观测，非本轮独立群号验算。[17项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-crosscheck.json)。

两幅归属窗口原生PNG（872×558）已逐一目视：反演行−x/−y/−z、T+1显示mx/my/mz；T−1显示−mx/−my/−mz，符合独立规则。[普通反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/inversion-equivalent-ui.png) / [时间反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/anti-repeat-equivalent-ui.png)。wx表行不暴露于AX，此为原图视觉读数；没有提取实际展开原子上的数值磁矩，也未校准3D箭头方向/长度。polar、非正交、特殊位置、磁超空间与New-dialog编辑仍待，不把VF0091/VF0177或完整磁性科学标作完成。

首截图native调用尚未完成时Codex提前Close/Quit，实际screenshot_window_not_found/单AXApplication节点；失败与 [原因记录](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/native-capture-failure.json) 保留。随后两个新归属进程分别33.382/30.084秒正常0退出，成功取证；首进程75.711秒也正常0，全部<180且无残留。CIF reader0xa671c已有静态证据ID去重，不重复增加记录；新增8条至844，12问题头/旧836证据/原件257/旧73样本不变，稳定样本增至75。80个历史manifest/11112文件完整。研究仍10完成19未完成、Codex自验，dsh独立复审未开始，无产品实现。
