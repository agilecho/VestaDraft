# S02.3 格式与科学来源续读（2026-11-08）

**当前任务状态（2026-11-10）：** S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 来源unknown revision10/high/investigating保留目标绑定边界；下文旧 revision/未勾选续记为历史，来源阅读不等于能力通过。接续见 [HANDOFF](HANDOFF.md)，阅读边界见 [SOURCE_NOTES](SOURCE_NOTES.md)。

承接[SOURCE_SUPPLEMENT](SOURCE_SUPPLEMENT.md)。本轮补主要格式的一手说明和核心论文的可读部分；S02.3仍为[ ]，不把读过规范等同于原件支持、完整算法恢复或科学验收。完整外部PDF、网页、源码及源图只在临时阅读目录；永久[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-135713-s02-formats-science/)保存身份、章节、简短笔记与原生更新。

本页记录前轮范围；最新的rho/charg_frm/Elk writer与科学原文补读见[SOURCE_VOLUME_SCIENCE](SOURCE_VOLUME_SCIENCE.md)，原生来源问题接续至revision4，未将本页旧待办当成本轮仍完全未读。

## 文件格式来源

| 来源ID / 直接入口 | 版本与实际阅读范围 | 可用于后续验证的区别及边界 |
|---|---|---|
| EXT-CTFILE2020，[BIOVIA原始规范](https://discover.3ds.com/sites/default/files/2020-08/biovia_ctfileformats_2020.pdf) | 封面BIOVIA Databases 2020、版权2019，105页。读封面/目录，PDF7–8、11–14、38–40、47–55、63、74–75、81–82；含Ch2前段、Ch3、Ch6指定字段、Ch7、Ch10。未读完整Ch2/6或其余章节。 | V2000固定列与V3000块/关键字结构不同；V3000有80列续行/字符串规则。V2000电荷编码不直接等于电荷值；M CHG/RAD/ISO优先于旧字段，M END与SDF的$$$$终止不同层级。实际渲染5页：印刷49页电荷范围确为“15 to +15”，与本书负电荷例子不一致，保留印刷疑点，不静默补负号。目标是否支持V3000/SDF、多记录与各属性仍待核。 |
| EXT-SHELX / EXT-SHELXL-GUIDE，[官方索引](https://shelx.uni-goettingen.de/wikis.php)、[当前指南](https://shelx.uni-goettingen.de/shelxl_user_guide.pdf) | 导航/索引已读；49页指南只读Introduction及§1.1–1.9（PDF1–5）。正文说基于SHELX-97并更新，未给可据此绑定目标的数字发行版；PDF元数据日期不是应用版本。 | `.ins/.hkl/.res/.lst/.cif`用途不同；几何s.u.可用完整协方差，但平面夹角另作近似。它描述SHELXL，不能据此替VESTA保证误差传播；未运行该程序。 |
| EXT-SHELXL-COMMANDS，[官方HTML指令](https://shelx.uni-goettingen.de/shelxl_html.php)、[PDF列表](https://shelx.uni-goettingen.de/shelxl_comlist.pdf) | HTML读General/原子语法及CELL、FVAR、LATT、PART、RESI、SFAC、SYMM、UNIT条目；39页PDF只核身份/页数，不宣称已读PDF全文。HTML无明确发行版号，按获取摘要固定身份。 | 分数坐标，U分量顺序为11/22/33/23/13/12；sof中的11不是占位11，21/-21及特殊位点倍数要解码。PART/RESI与FVAR会影响含义，不能逐行当普通XYZ；源文字的负自由变量记号与例子须结合历史版复核，未登记为目标算法。 |
| EXT-SHELX97，[官方历史手册](https://shelx.uni-goettingen.de/shelx97.pdf) | 154页，版本SHELX-97明确；读封面/目录及§2.2–2.3（PDF7）、§7.1（PDF47–49并读50页END说明）、§7.3原子/SPEC/RESI开头（PDF54–56）。未读整章/全手册。 | 与当前说明分开；END可用于include文件并可在HKLF之后出现，负U可绑定前原子Ueq。旧版自由变量公式的记号与当前HTML不同；采用明确的互补占位示例作后续探针线索，不照抄含糊记号。 |
| EXT-XPLOR-NIH / EXT-XPLOR-MAP，[包入口](https://nmr.cit.nih.gov/xplor-nih/)、[地图页](https://nmr.cit.nih.gov/xplor-nih/doc/current/xplor/node333.html)、[上级说明](https://nmr.cit.nih.gov/xplor-nih/doc/current/xplor/node330.html) | 入口只核当前3.12和文档导航；两张地图页正文/Fortran读取例已读，页脚2026-07-10。`current`可变，不冒充1993年X-PLOR3.1原书或CNX规范。 | 9I8给全轴网格数与局部最小/最大索引，不能把NA与局部点数混同；晶胞Å/度。ZYX例中z切片最慢、x最快，正文同时区分formatted/unformatted。代码片段并非可直接复制的完整解析器；未从unformatted推断固定字节序或Fortran记录头。 |
| EXT-STL-BURNS，[作者书节摘录](https://www.fabbers.com/tech/STL_Format) | Marshall Burns《Automated Fabrication》§6.5，页标版权1993/1999、技术来源1989；全文及4张原图/语法表实际读过。属于作者说明，不是假称取得3D Systems原规范。 | 面法向与顶点绕序、相邻面顶点规则；ASCII语法与二进制80字节头/4字节计数/每面50字节布局分开。原摘录称坐标任意单位/无尺度信息，并有正象限等旧约束；未将其强加为目标实现或现代所有STL的保证，也未把属性字节当通用颜色标准。 |
| EXT-STL-LOC，[格式族](https://www.loc.gov/preservation/digital/formats/fdd/fdd000504.shtml)、[二进制页](https://www.loc.gov/preservation/digital/formats/fdd/fdd000505.shtml) | 仅读格式描述/来源定位等选段，页面显著更新2025-02-25，Preliminary。 | 属LOC策展说明，非原始厂商规范；用于追溯原规范缺口，其编者也称未找到原始3D Systems规范。不能把LOC描述、书节及原规范合成一个版本。 |
| EXT-CHEM3D-OB231，[Open Babel固定提交源码](https://github.com/openbabel/openbabel/blob/ef02552aa0d9a6d87561aad806c315fcdc3b925a/src/formats/chem3dformat.cpp) | 2.3.1标签经只读ls-remote解引用到`ef02552aa0d9a6d87561aad806c315fcdc3b925a`；该文件全读，标签/提交原字节相等。源码只留临时目录，未编译/执行/复用。 | `c3d1`用MM2类型，`c3d2`用C3D；首行7/8项分数坐标模式按角度、长度及可选缩放指数解释。读原子行忽略输入编号，按行建原子；连接先作单键再推断键序。它只证明Open Babel的实现，未证明VESTA `.cc1`对应关系和行为相同。空白文档页的缺口因而只收窄至“有实现资料、缺厂商规范/目标绑定”。 |
| EXT-CNS，[官方入口](https://www.cns-online.org/v1.3/) | curl403，web一个主机不可达、另一个无可读正文。 | CNS/CNX正文及其与X-PLOR格式的确切兼容版本未核，不由消费者文档或NIH当前页面替代。 |

Chem3D的GitHub连接器报Unknown tool，公开API返回403限流，均保留；只读公开源码与ls-remote是实际成功通道，没有克隆仓库或建立工作树。通用HTML提取器会损失C++尖括号，源码阅读/行号以原始UTF-8字节为准，不用该提取文本作源码身份。

## 核心几何论文

| 来源ID / 原出版物 | 本轮证据与读取范围 | 结论边界 |
|---|---|---|
| EXT-BIB-032，Baur 1974，B30:1195–1215，[DOI](https://doi.org/10.1107/S0567740874004560) | 出版社/Crossref核身份；出版社PDF401/403。另从[作者上传原论文](https://www.researchgate.net/profile/W-Baur/publication/250905022_The_Geometry_of_Polyhedral_Distortions_Predictive_Relationships_for_the_Phosphate_Group/links/58e27507a6fdccf442032df0/The-Geometry-of-Polyhedral-Distortions-Predictive-Relationships-for-the-Phosphate-Group.pdf)的web通道读印刷1195–1196及1197的形状分类/Table2/图注选段；21页原文未全读。curl作者PDF403，web截图超时，不能宣称原字节或公式视觉核验。 | 已读畸变指数定义：距离、角度、边分别围绕各自均值取绝对偏差并归一；正文讨论特定形状比较基准。不能把它与平方伸长、键角方差或全部多面体指标混为同一个量；目标选邻居/退化/精度仍属S06。 |
| EXT-BIB-033，Robinson/Gibbs/Ribbe 1971，Science172:567–570，[DOI](https://doi.org/10.1126/science.172.3983.567) | 出版者交存的Crossref元数据核题名/卷页/DOI；Science正文/PDF403。 | 仅身份核定，未读原文公式；不借引用它的论文填成原文已读。 |
| EXT-BIB-034，Hoppe 1979，Z.Kristallogr.150:23–52，[出版页](https://www.degruyterbrill.com/document/doi/10.1524/zkri.1979.150.14.23/html) | web可读出版社引文/访问限制，印刷1979-12-01、上线2015-09-18；正文受限，curl202空体。Crossref同时有`.14.23`与`.1-4.23`两条题名不同的交存记录；出版页采用前者。 | 原先DOI拼写未知收窄为出版社采用`.14.23`，不把2015当研究年份，也不武断宣称另一个交存DOI不存在。ECoN/MEFIR正文和公式仍未核。 |
| EXT-BIB-038，Brown/Altermatt 1985，B41:244–247，[出版页](https://journals.iucr.org/paper?buy=yes&cnor=a24419) | 出版页核题名/卷页/DOI10.1107/S0108768185002063；PDF401/403，Crossref请求429留存。 | 正文未读；不能用邻篇240–244、现有bvparm文件或其他论文中的公式代替本文原文。 |

## 生产者资料追加选读

以下8项另存[生产者阅读台账](../../output/vesta-study/20261108-113622/reviews/20261108-135713-s02-formats-science/producer-reading-ledger.json)，与前15项续作台账合计23项；台账项数不等于全文数或格式通过数。

| 来源ID / 直接入口 | 版本与实际阅读范围 | 字段区别与剩余边界 |
|---|---|---|
| EXT-FHI-AIMS，[版本目录](https://fhi-aims.org/uploads/manual/Ch2/index.html)、[文件说明](https://fhi-aims.org/uploads/manual/Ch2/S1.html)、[结构标签](https://fhi-aims.org/uploads/manual/Ch3/S2.html) | 目录261002；§2.1正文/例子及§3.2的atom、atom_frac、lattice_vector完整条目。未读整个§3.2或全手册。 | 默认长度Å、能量eV；自由格式、大小写敏感，物种名须与control.in匹配；atom_frac按有序晶格向量组合，仅周期体系可用。该版晚于目标构建日期，不能倒推目标采用它；原子附加标签及版本绑定待核。 |
| EXT-ELK，[官方手册](https://elk.sourceforge.io/elk.pdf) | web通道203页，封面明确11.2.3；读§5.2 atoms、5.7 avec、5.8 avecref、5.72 molecule及5.128中task2/3条目。curl手册/主页403，未取得PDF原字节摘要。 | avec为Bohr；原子默认晶格坐标，molecule为真则用Cartesian；task2写GEOMETRY.OUT。原子还含物种文件及外加场。输入字段不能替代输出文件的完整记录契约，GEOMETRY.OUT具体布局/目标版本仍待核。 |
| EXT-WIEN2K，[官方用户指南](http://www.wien2k.at/reg_user/textbooks/usersguide.pdf) | 23.2，Release07/24/2024，316页。读§4.3 case.struct的PDF62–66字段；表4.4矩阵未逐式核定。另读PDF62的case.rho用途及§5.10.9–10的rhoplot/XSF选段（PDF140–142）。PDF64实际渲染核字段。HTTPS超时，HTTP成功，分别留证。 | `.struct`固定列，NAT为不等价原子数，晶格长度用原子单位；分数位置、MULT重复、局部旋转与空间群记录分层。R晶格的晶格参数和位置表示约定不同，不能只改单位。`.rho`目前只核平面密度图用途，完整网格字段/轴顺序未核；未把XSF导出说明当rho规范。 |
| EXT-FEFF，[官方9.6指南](https://feff.phys.washington.edu/FEFF/Docs/feff9/feff90/feff90_users_guide.pdf) | 封面feff9.6.4、2013-02-02，150页。读§4.1全文（PDF75）及§4.3的ATOMS/LATTICE/CIF/POTENTIALS/TITLE、REAL/RECIPROCAL/TARGET、COORDINATES/RMULT/SGROUP指定条目（PDF78–82选段）。未读全§4.3。 | 实空间ATOMS以Å输入，ipot由POTENTIALS定义，不是元素Z；倒空间另有单胞/目标原子约定。COORDINATES支持不同单位/分数位置选项，手册对默认项留有“FIX check this”字样，不能据此硬定默认。首页超时不影响该历史指南成功获取；目标采用版/子集待核。 |
| EXT-GSAS，[官方分发手册](https://subversion.xray.aps.anl.gov/EXPGUI/gsas/all/GSAS%20Manual.pdf) | LAUR86-748、Larson/VonDreele，版权至2004，页眉9/26/04，231页。读封面/目录；EXP说明PDF186，晶体记录PDF190–193，文件末尾记录PDF218；PDF191实际渲染核字段。未读整个Files章。 | EXP为80字符记录、12字符键及各记录固定格式，不是普通XYZ。晶胞/角度及其s.u.分列；原子记录含类型、分数位置、占位、多重性与U及精修代码，宏分子相另有布局。版本记录/结束标记有校验意义；U分量顺序、完整对称/磁性记录及GRD体数据契约未核。猜测NIST路径404保留。 |
| EXT-CASTEP，[项目文档cell页](https://castep-docs.github.io/castep-docs/documentation/Input_Files/cell_file/) | 阅读Lattice Vectors/Ionic Positions两节；未建立与目标对应的数字发行版。不是Materials Studio某一历史版手册全文。 | LATTICE_CART与LATTICE_ABC互斥，POSITIONS_ABS与POSITIONS_FRAC互斥；可给单位，默认长度Å/角度度，支持表达式与可选自旋字段。未核`.charg_frm`体数据、其余cell块或目标支持子集。Cambridge旧猜测路径404保留。 |
| EXT-GAMESS，[官方输入说明](https://www.msg.chem.iastate.edu/GAMESS/GAMESS_Manual/input.pdf) | 2016-07-01，523页。读封面概述、$CONTRL中COORD/UNITS（PDF15–16指定条目）、$DATA标题/群/坐标框架及U原子定义（PDF40–43）、PDF44负核电荷说明、C序列与ZMT前段（PDF45末–47）。基组段/其余序列未全读。 | UNIQUE只输入对称独立原子，CART给所有原子，ZMT是内坐标；重定向可改变顺序。Cartesian原子名仅用于打印，核电荷决定身份，负值另有基组中心含义。UNITS默认ANGS也可BOHR；不能把所有$DATA都按元素名+XYZ解析。目标解析/其它输入组未核。 |
| EXT-MACMOLPLT，[开发者主页](https://brettbode.github.io/wxmacmolplt/)、[Files](https://brettbode.github.io/wxmacmolplt/Manual_pages/MacMolPlt_Files.html)、[Surfaces](https://brettbode.github.io/wxmacmolplt/Manual_pages/MacMolPlt_Surfaces.html) | 主页7.7.3/2024-01；手册未明确同版。读手册目录、Files的格式识别及GAMESS/XYZ/2D和3D Surface指定段，Surfaces的General3D文字；未读整份手册或核示意图。 | 自定义3D网格有标签、nx/ny/nz、原点、三个轴向增量和nx×ny×nz值，标作grid(x(y(z)))；2D另给向量。坐标单位措辞为通常Å，不能提升成无条件规范；每行值数自由。输入器会丢部分GAMESS选项，这不是GAMESS本身的契约，也不证明目标读写相同。 |

追加4份PDF原字节/页数身份与2张实际查看页另有记录；Elk只有web正文读证，不混入本地PDF摘要。不同来源的版本/单位/原子身份和网格轴序分别保存，未合成一个通用格式。

## 覆盖范围与接续

[格式来源覆盖表](FORMAT_SOURCE_COVERAGE.md)逐项列旧手册47类输入和已观察23类数据输出；编号只是资料定位，不是功能通过ID。它分别标作者规范选读、实现/消费者说明、VESTA自述和仅入口/失败，防止读过几种就概括全部。目标版本/格式子集均未因此整体核定；不把已有样本通过抹成完全未知，也不把样本通过扩成全格式兼容。

本轮来源问题沿用`unk_e1a118c694cc99ea`，真实更新至revision3/high/investigating；其余8问题、84条evidence不变，来源问题evidence_for/against仍为空。当前待办：

1. FEFF、FHI-aims、Elk、GSAS、WIEN2k、CASTEP、GAMESS/MacMolPlt已补上述选定范围，仍需核Elk输出布局、WIEN2k rho、CASTEP charg_frm及剩余字段/目标绑定。VICS/AMCSD/数据库旧格式与MOLDA/XTL、SCAT、Energy Band等仍有来源/版本缺口。
2. 核CIF所用字典/磁性与超空间扩展、各输出的确切版本及剩余字段；Chem3D、CSSR、STL原始厂商规范和CNS不能据替代来源提前结项。
3. 对受限原文保留准确状态，不重复相同失败下载；补Robinson/Hoppe/Brown、散射/色散、Tsirelson、ITC相关正文与章节。Baur定义选读不等于整篇或S06计算验收。
4. 来源版本与原件绑定、调用/CWD/完成继续S05–S07既有问题；本轮不运行原件或外部科学程序。

## R29配位指标后续

[VM11公式与选定轴向畸变对照](ALGORITHMS.md)已核mean/V/BaurD/quadraticelongation/ECoN五项显示精度；预登记重复加权候选与实际区分。Robinson/Hoppe原文全文未获得、不重复受限下载、不把手册误称两原论文。S02.3/S06保持未完成，来源问题rev8不改。

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
