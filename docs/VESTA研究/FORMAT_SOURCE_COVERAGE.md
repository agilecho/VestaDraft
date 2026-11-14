# 格式来源覆盖表

**当前任务状态（2026-11-10）：** S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 来源unknown revision10/high/investigating保留目标绑定边界；下文旧 revision/未勾选续记为历史，来源阅读不等于能力通过。接续见 [HANDOFF](HANDOFF.md)，阅读边界见 [SOURCE_NOTES](SOURCE_NOTES.md)。

来源阅读与原件契约分开。旧手册47类输入及S04已观察23类数据输出逐项列在下表；这些编号只作资料定位，不能替代VF/VS功能与样本ID。准确版本/章节见[SOURCE_SUPPLEMENT](SOURCE_SUPPLEMENT.md)、[本轮续读](SOURCE_FORMATS_SCIENCE.md)、[体数据/科学续读](SOURCE_VOLUME_SCIENCE.md)、[字典/历史格式续读](SOURCE_DICTIONARIES_LEGACY.md)、[历史生产者/绑定续读](SOURCE_LEGACY_PRODUCERS.md)、[MOLDA/C04D/XTL续读](SOURCE_REMAINING_FORMATS.md)、[梯度/参数/MEM续读](SOURCE_SCIENCE_PARAMETERS.md)和[资料索引](../VESTA资料索引.md)。

未完成目标契约不表示不能打开或导出；已有实测样本仍以[FORMATS](FORMATS.md)/[SAMPLES](SAMPLES.md)为准。表中选读、实现说明和VESTA自述均不等于完整格式兼容。结构15项与体数据11项扣除共同4项、加原生工程1项，共23个数据输出入口；不含图像导出的完整清单。

R133的S02专项标准澄清见[SOURCE_NOTES](SOURCE_NOTES.md)末节：本表“原厂商规范/全文/目标子集待核”不自动等于S02阻断；固定消费者或后续专业来源须如实标注身份与范围。R134的22项相关正文已经dsh明确来源复核通过；专项收尾验收实际通过后S02.3已勾选，完整格式语义/目标契约仍以本表边界为准。

## 旧手册输入

| 资料定位 | 手册条目 | 来源ID | 覆盖与边界 |
|---|---|---|---|
| VM18-I01 | VESTA format (*.vesta) | VM18 | VESTA格式说明已读：VM18相关布局/说明；工程块、压缩载荷及目标版本仍有待核部分 |
| VM18-I02 | VICS format (*.vcs) | VM18, EXT-VENUS-LEGACY, EXT-VICS-VEND-MANUAL20060130, EXT-VENUS-EXAMPLES20040727 | 生产者旧文档/原例指定范围已读：2006-01-30手册附录A及2004原例vcs全文；完整关键字/目标子集未核 |
| VM18-I03 | American Mineralogist Crystal Structure Database [99] (*.amc) | VM18, EXT-AMCSD-FORMAT, EXT-AMCSD-PAPER2003 | 原作者2003论文印247含图1/248承接句已读；书目/晶体记录、原点/列头/位移说明，非当前数据库全格式 |
| VM18-I04 | asse (*.asse) | VM18, EXT-ASSE, EXT-ASSE05-AUTHORARCHIVE | 原作者0.5输入章节/指定读取代码已读：begin/end、lattice/atoms与坐标模式；长度单位/目标子集未核 |
| VM18-I05 | Chem3D (*.cc1) | VM18, EXT-CHEM3D-OB231 | 实现/消费者资料：2.3.1实现代码；厂商规范/cc1绑定待核 |
| VM18-I06 | Crystallographic Information File (CIF; *.cif) [100] | VM18, EXT-CIF1-SYNTAX, EXT-CIF1-SEMANTICS, EXT-CIF-CORE245, EXT-CIF-CORE330, EXT-CIF-MAG, EXT-CIF-MS | 生产者资料指定范围已读：CIF1语法/语义及四字典指定条目；全字典/导入依赖/目标CIF2及磁调制支持未核 |
| VM18-I07 | CrystalMaker text file (*.cmt, *.cmtx) | VM18, EXT-CMTX-PRODUCER | 生产者版本说明/导航：11.0.0 CMTX字段变更、FAQ/规格选读；完整CMT/CMTX语法及目标子集待核 |
| VM18-I08 | Crystal Structure Search and Retrieval (CSSR; *.cssr) | VM18, EXT-ZEOPP-CSSR, EXT-CERIUS2-FORMATS | 实现/消费者资料指定范围已读：Zeo++及MSI1999 CSSR字段说明；原SERC规范/目标约定未核 |
| VM18-I09 | Cambridge Structural Database [102] (CSD/FDAT; *.csd, *.fdt)) | VM18, EXT-FDAT-RPLUTO | 生产者资料指定范围已读：RPluto13.1及ConQuest2022.2 FDAT选读；配套TXT为验证页/例文件404，字段边界及目标待核 |
| VM18-I10 | DL_POLY [103] format (CONFIG, REVCON, *.config) | VM18, EXT-DLPOLY | 生产者资料指定范围已读：5.3.0 CONFIG选读；旧版与目标待核 |
| VM18-I11 | FEFF input file (feff.inp) | VM18, EXT-FEFF | 生产者资料指定范围已读：9.6.4的4.1/4.3指定结构卡；倒空间/单位选项分开，目标版待核 |
| VM18-I12 | FHI-aims input file (*.in) | VM18, EXT-FHI-AIMS | 生产者资料指定范围已读：261002的2.1及三个结构标签；版次晚于目标，剩余标签待核 |
| VM18-I13 | Output file of the Elk FP-LAPW Code (GEOMETRY.OUT) | VM18, EXT-ELK, EXT-ELK-WRITEGEOM | 生产者资料指定范围已读：11.2.3输入块/task2加官方writer选读；writer发行版及GEOMETRY.OUT调用绑定待核 |
| VM18-I14 | GSAS [18] format (*.EXP) | VM18, EXT-GSAS | 生产者资料指定范围已读：2004 EXP指定结构记录；非全章，U顺序/目标子集待核 |
| VM18-I15 | Inorganic Crystal Structure Database [87] (ICSD; *.ics) | VM18, EXT-CERIUS2-FORMATS, EXT-ICSD-CONSUMERS | 消费者说明/生产者导航：厂商读取说明及当前ICSD入口已读；1991原手册和历史ics变体未得 |
| VM18-I16 | ICSD-CRYSTIN: (*.cry) | VM18, EXT-ICSD-CONSUMERS | 消费者转录指定范围已读：Diamond5转录1986 CRYSTIN字段；原手册/转录疑点及目标兼容未核 |
| VM18-I17 | MDL Molfile (*.mol) | VM18, EXT-CTFILE2020 | 生产者资料指定范围已读：2020版V2000/V3000/SDF指定范围；非全规范 |
| VM18-I18 | MINCRYST (Crystallographic Database for Minerals; *.min) | VM18, EXT-MINCRYST-PRODUCER | 生产者说明/单卡选读：旧路径404，现站单卡与BDM/CPDS区别已核；非完整min语法/目标通过 |
| VM18-I19 | MOLDA [106] (*.mld) | VM18, EXT-MOLDA-DATA, EXT-MOLDA65-AUTHOR-GUIDE | 数据集字段及原作者Windows6.5 User Guide G30/G60已读：mld默认/可配置mol，与XYZ分开；本次相关页未给坐标单位，不从数字例子推定，目标契约另核 |
| VM18-I20 | Protein Data Bank (PDB; *.pdb) [107] | VM18, EXT-PDB33 | 生产者资料指定范围已读：3.3指定四章；非全规范 |
| VM18-I21 | Input file of RIETAN-FP/2000 [12, 108] (*.ins) | VM18, EXT-RIETAN-MANUAL | 生产者资料指定范围已读：2021-12-16手册Ch17选读 |
| VM18-I22 | Output file of RIETAN-FP [12] (*.lst) | VM18, EXT-RIETAN-MANUAL | 生产者资料指定范围已读：2021-12-16手册输出选段 |
| VM18-I23 | SHELXL [97] (*.ins) | VM18, EXT-SHELX97, EXT-SHELXL-COMMANDS | 生产者资料指定范围已读：97版和当前说明分开；指定字段 |
| VM18-I24 | Output file of STRUCTURE TIDY (*.sto) | VM18, EXT-RIETAN-COOP | 生产者资料指定范围已读：联动说明选读；TIDY原论文正文未读 |
| VM18-I25 | Structure data files output by USPEX (gatheredPOSCARS, BESTgatheredPOSCARS) [109, 110] | VM18, EXT-USPEX-MANUAL, EXT-POSCAR | 生产者资料指定范围已读：9.4.4/10.5/2023指定输出说明；多块与order余列分开，目标读取仍待核 |
| VM18-I26 | WIEN2k [41] (*.struct)[109, 110] | VM18, EXT-WIEN2K | 生产者资料指定范围已读：23.2的4.3字段选读；表4.4矩阵未逐式核定，目标绑定待核 |
| VM18-I27 | XMol XYZ (*.xyz) | VM18, EXT-OPENBABEL-XYZ | 实现/消费者资料：OpenBabel实现说明；无统一正式XYZ规范 |
| VM18-I28 | F01 for SCAT [58, 111] and C04D for contrd | VM18, EXT-DVXA-CONTRD, EXT-DVXA-BULLETIN33, EXT-CONTRD-README20050202, EXT-CONTRD-SOURCE20040202, EXT-F01-INPUT2014 | 生产者2014完整F01输入/单位标志、2012 writer实例、CONTRD原Readme全文及指定input/writer已读；2005说明/2004源码差别保留，全部变体/目标配套未核 |
| VM18-I29 | MXDORTO/MXDTRICL [112, 113] (FILE06.DAT, FILE07.DAT) | VM18, EXT-MXDORTO, EXT-MXDORTO-MANUAL101, EXT-MXDORTO-SOURCE20251120, EXT-MXDTRICL-SOURCE20251119 | 官方正文已恢复：NIMS手册1.0.1 §3.2及两版serial writer/FORMAT选读；网页/源码日期差、头布局区别保留，FILE06全布局/目标契约仍待；原件失败沿用原ID |
| VM18-I30 | XTL format (*.xtl) | VM18, EXT-GULP-XTL, EXT-CP2K-XTL | 生产者输出说明/消费者代码：GULP5.1输出段及CP2K2025.1静态读取子集；MSI原规范/目标兼容未核 |
| VM18-I31 | MEM densities in binary format (*.pri, *.prim) | VM18, EXT-VENUS-LEGACY, EXT-VICS-VEND-MANUAL20060130, EXT-PRIMA-MANUAL20051021 | 旧生产者手册§3.3/§8与VEND附录指定范围已读：二进制/平台/单位与非对称像素语义；印文本FORMAT和binary叙述矛盾保留，完整字节布局/目标未核 |
| VM18-I32 | MEM densities in text format (*.den) | VM18, EXT-VENUS-LEGACY, EXT-PRIMA-MANUAL20051021, EXT-VENUS-EXAMPLES20040727 | 原生产者§8旧文本密度字段、VEND den说明及两例前24记录已读；单/双符号密度分开，非所有变体或目标通过 |
| VM18-I33 | Energy Band (*.eb) | VM18, EXT-ARAI-EB, EXT-VICS-VEND-MANUAL20060130, EXT-ASSE05-AUTHORARCHIVE | 旧VEND生产者附录A ID9正文已读，近似rho/能量padding；Arai0.5同后缀能带列表/VTK明确排除，能量单位/分带及目标另核 |
| VM18-I34 | General volumetric-data (text format) (*.?ed) | VM18 | VESTA格式说明已读：VM18相关布局/说明；工程块、压缩载荷及目标版本仍有待核部分 |
| VM18-I35 | Periodic volumetric-data (text format) (*.grd) | VM18 | VESTA格式说明已读：VM18相关布局/说明；工程块、压缩载荷及目标版本仍有待核部分 |
| VM18-I36 | General volumetric-data (binary format) (*.ggrid) | VM18, EXT-ERIS-MANUAL | 同作者系列指定范围已读：ERIS2026二进制网格/载荷说明；伪代码与fType混写疑点、目标变体未核 |
| VM18-I37 | Periodic volumetric-data (binary format) (*.pgrid) | VM18, EXT-ERIS-MANUAL | 同作者系列指定范围已读：ERIS2026 pgrid章节补头/索引/对称操作自述；非目标全变体验收 |
| VM18-I38 | Compressed volumetric-data format (*.m3d) | VM18 | VESTA格式说明已读：VM18相关布局/说明；工程块、压缩载荷及目标版本仍有待核部分 |
| VM18-I39 | SCAT volumetric-data files (*.sca, *.scat) | VM18, EXT-DVXA-CONTRD, EXT-CONTRD-README20050202, EXT-CONTRD-SOURCE20040202, EXT-VICS-VEND-MANUAL20060130 | 原Readme/SCA writer和旧VEND单位指定正文已读：头/轴序/场单位，非全部SCAT版本/目标兼容 |
| VM18-I40 | WIEN2k (*.rho) | VM18, EXT-WIEN2K, EXT-WIEN2VENUS, EXT-WIEN2VENUS-20111124, EXT-VICS-VEND-MANUAL20060130 | 作者2011-11-24脚本及说明全静态读，旧VEND补rho单位；与LAPW5二维rho分开，非立方分支/目标契约未验 |
| VM18-I41 | WinGX 3D Fourier map (*.fou) | VM18, EXT-WINGX-MAP, EXT-MCE-SVN145-FOU | WinGX/MCE集成和固定消费者SVN145 ReadFouFile全文已读，补头/矩阵/网格/原子变体；非WinGX全规范/目标通过 |
| VM18-I42 | X-PLOR/CNX (*.xplor) | VM18, EXT-XPLOR-MAP, EXT-CNS11-MAP, EXT-CHIMERAX19-CNS-READER | XPLOR3.12、Yale CNS1.1地图命令及ChimeraX1.9 ASCII消费者指定正文已读；商业CNX/历史差异/目标子集仍待 |
| VM18-I43 | CASTEP [124, 125] (*.cell, *.charg_frm) | VM18, EXT-CASTEP, EXT-CASTEP-GUIDE2024 | 生产者资料指定范围已读：cell两节及Materials Studio2024的charg_frm列/归一化选读；尺寸/晶胞头及目标兼容待核 |
| VM18-I44 | GAMESS [126] input and volumetric data files output by MacMolPlt [127] | VM18, EXT-GAMESS, EXT-MACMOLPLT | 生产者资料指定范围已读：GAMESS2016指定DATA/CONTRL；MacMolPlt网格说明选读，非整体契约 |
| VM18-I45 | Gaussian Cube format (*.cube, *.cub) | VM18, EXT-CUBE | 生产者资料指定范围已读：G16RevC.01指定小节；03/09版未核 |
| VM18-I46 | VASP (*.vasp, CHG, CHGCAR, PARCHG, LOCPOT, ELFCAR, POSCAR, CONTCAR) | VM18, EXT-POSCAR, EXT-CHGCAR | 生产者资料指定范围已读：现代wiki两格式；其他VASP场/目标子集待核 |
| VM18-I47 | XCrySDen XSF format (*.xsf) | VM18, EXT-XSF | 生产者资料指定范围已读：2025-08-04正文；无数字格式版号 |

## 已观察的数据输出入口

| 资料定位 | 项目 | 来源ID | 阅读边界 |
|---|---|---|---|
| OBS-O01 | VESTA | VM18 | 原生工程说明；样本文件头3.5.4不是应用版本 |
| OBS-O02 | CUBE | EXT-CUBE | 指定producer小节 |
| OBS-O03 | VASP/POSCAR | EXT-POSCAR, EXT-CHGCAR | 两个格式wiki；输出类型/归一化待核 |
| OBS-O04 | STL | EXT-STL-BURNS | 作者摘录；原厂商规范未得 |
| OBS-O05 | VRML | EXT-VRML97 | 97版AnnexA及指定节点 |
| OBS-O06 | CIF | EXT-CIF1-SYNTAX, EXT-CIF1-SEMANTICS, EXT-CIF-CORE245, EXT-CIF-CORE330, EXT-CIF-MAG, EXT-CIF-MS | 相关字典指定条目已读；实际输出标签/版次及扩展支持待核 |
| OBS-O07 | PDB | EXT-PDB33 | 3.3指定章节 |
| OBS-O08 | SHELX | EXT-SHELX97, EXT-SHELXL-COMMANDS | 历史与当前说明指定字段 |
| OBS-O09 | RIETAN | EXT-RIETAN-MANUAL | 2021手册指定章节 |
| OBS-O10 | XYZ | EXT-OPENBABEL-XYZ | 实现说明；无统一规范 |
| OBS-O11 | Chem3D | EXT-CHEM3D-OB231 | OpenBabel2.3.1实现；目标对应待核 |
| OBS-O12 | DL_POLY CONFIG | EXT-DLPOLY | 5.3.0CONFIG小节 |
| OBS-O13 | MADEL pme | EXT-MADEL-MANUAL | 8页手册；本地程序版未知 |
| OBS-O14 | STRUCTURE TIDY stin | EXT-RIETAN-COOP | 联动资料；原始输入合同未全核 |
| OBS-O15 | P1 | VM18 | VESTA自述 |
| OBS-O16 | 分数坐标 XTL | VM18, EXT-GULP-XTL, EXT-CP2K-XTL | XTL生产者用途/消费者子集已读；当前显示原子/P1仅VM18自述，原规范及目标未核 |
| OBS-O17 | 3ed | VM18 | 通用文本布局自述 |
| OBS-O18 | grd | VM18 | VESTA周期文本布局；GSAS fmp选读未建立其与通用grd的对应关系 |
| OBS-O19 | m3d | VM18 | 压缩载荷具体合同未补 |
| OBS-O20 | pri | VM18, EXT-PRIMA-MANUAL20051021, EXT-VICS-VEND-MANUAL20060130 | 原PRIMA/VEND指定生产者说明已读；binary/text叙述差别与完整字节布局/目标输出仍待 |
| OBS-O21 | ggrid | VM18, EXT-ERIS-MANUAL | ERIS2026二进制网格指定章节已读；伪代码疑点及目标变体仍待核 |
| OBS-O22 | pgrid | VM18, EXT-ERIS-MANUAL | ERIS2026二进制网格指定章节已读；伪代码疑点及目标变体仍待核 |
| OBS-O23 | Xplor | EXT-XPLOR-MAP | 当前生产者读取例；输出版本待核 |

输入来源是2022旧手册，实际过滤器和新版磁性/准晶/超空间范围仍需S04–S07；图像格式来源不因本表未列而排除。机器可核的同值表在[覆盖JSON](../../output/vesta-study/20261108-113622/reviews/20261108-135713-s02-formats-science/format-source-coverage.json)，只是来源覆盖图，不是产品验收表。
