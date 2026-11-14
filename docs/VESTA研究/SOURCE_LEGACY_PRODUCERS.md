# S02.3 历史格式与生产者绑定续读（2026-11-08）

**当前任务状态（2026-11-10）：** S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 来源unknown revision10/high/investigating保留目标绑定边界；下文旧 revision/未勾选续记为历史，来源阅读不等于能力通过。接续见 [HANDOFF](HANDOFF.md)，阅读边界见 [SOURCE_NOTES](SOURCE_NOTES.md)。

承接[SOURCE_DICTIONARIES_LEGACY](SOURCE_DICTIONARIES_LEGACY.md)。本轮补公开历史格式资料，同时纠正`.rho`来源关联过宽的问题；S02.3仍[ ]。原文/PDF/派生阅读页仅在临时目录，永久[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-152746-s02-legacy-producers/)保留身份、范围、差异及新校验。

后续MOLDA/C04D/XTL选读见[SOURCE_REMAINING_FORMATS](SOURCE_REMAINING_FORMATS.md)，来源问题现接续revision7；本页保留前轮范围与rho裁定。

## WIEN2k同后缀格式的来源绑定

重读VM18 §18.5.1第40项后确认，该条指向Arai的`wien2venus.py`生成三维电子密度，手册所述单位为bohr⁻³。前轮读到的WIEN2k23.2 LAPW5 `case.rho`是二维平面输出，相关字段说明本身保留；**它不能仅凭`.rho`后缀作为VM18该条的格式契约**。前轮未登记目标读取通过，本轮进一步把来源关联分开。

| 来源 | 实际证据与范围 | 当前裁定 |
|---|---|---|
| VM18，[原手册](https://jp-minerals.org/vesta/en/doc/VESTAch18.html) | 本轮只重读第40–41项以核生产者归属，记为上下文复核，不计新增正文成果。 | 第40项为转换脚本的三维数据；LAPW5二维头/轴序仅作同后缀参照，二者等价关系未建立。 |
| EXT-WIEN2VENUS，[NIMS作者入口](https://www.nims.go.jp/cmsc/staff/arai/wien/venus.html)、[WIEN附加软件页](https://susi.theochem.tuwien.ac.at/reg_user/unsupported/)、[Blaha 2020-01-02邮件](https://zeus.theochem.tuwien.ac.at/pipermail/wien/2020-January/030087.html) | NIMS HTTPS404/HTTP超时，未得脚本；WIEN官方页面仍将它作为外部附加程序链接。读该页相关条目与作者邮件正文，未读整页其它软件。 | 作者在2020年回复中认为脚本可能不完全兼容，并建议3ddens的XSF路线。这是当时的作者判断，邮件内用户报错不是本机重现。未取得转换后文件完整布局，也未运行脚本或目标；保留脚本来源/版本缺口。 |

[绑定复核](../../output/vesta-study/20261108-113622/reviews/20261108-152746-s02-legacy-producers/rho-binding-review.json)记录依据与变更范围；修改前检查真实失败、修改后另存检查结果。原LAPW5阅读记录与历史证据不回改为脚本规范，不撤销已有实际样本结果。

## 其它历史格式

| 来源 / 入口 | 版本与实际阅读范围 | 可用于后续验证的区别及边界 |
|---|---|---|
| EXT-FDAT-RPLUTO，[CCDC RPluto指南](https://downloads.ccdc.cam.ac.uk/rpluto/rpluto.pdf)、[ConQuest指南](https://www.ccdc.cam.ac.uk/media/Documentation/C82017ED-FAE4-4D93-BA5A-8D841F1E4314/ConQuest-UserGuide_2020_1.pdf) | RPluto68页；封面未见数字发行号，PDF元数据2006-06-27只作身份。读封面和§13.1叙述/表/两例（PDF46–52），实际查看47/48/50/51/52。ConQuest文件名含2020，封面实为2022.2、229页；仅读封面及FDAT导出/词条（PDF41/205）。 | FDAT为80字节定列，目录记录必需，其余记录可缺；晶胞按各自PREC缩放，坐标乘10⁵，AT/SAT与CENT/中心化分开。连接表有隐含FROM及后续成对索引，不能当普通XYZ。原文在总原子数=100的格式分界留空，未静默补规则。两个配套TXT的HTTP200实为验证页，例文件404，不能称完整字段规范取得；PDF内嵌表补读不替代缺失配套。ConQuest称FDAT仍可导出但不推荐。 |
| EXT-CERIUS2-FORMATS，[MSI厂商手册档案](https://www.chem.cmu.edu/courses/09-560/docs/msi/modenv/D_Files.html) | CMU托管的Molecular Simulations文档，页标更新1999-04-08；只读CSSR和ICSD两节及日期，未读其它格式。未建立具体Cerius2应用发行号。 | CSSR四条头记录加原子记录，0表示分数、1表示Å笛卡尔，连接最多8项；这是该厂商的读写约定，不是原SERC规范。ICSD节仅说明其读取内容并指向1991用户手册§9；原手册未取得。表中Fortran字样有排版疑点，未照抄成可执行解析器，也未把XTLGRF当成VESTA的XTL格式。 |
| EXT-ICSD-CONSUMERS，[Diamond5格式手册](https://www.crystalimpact.de/download/diamond5/docs/Manual_structure-file-formats.htm)、[ICSD生产者首页](https://icsd.products.fiz-karlsruhe.de/en) | 读Diamond的CRYSTIN Download字段段落全段，明确其转录自1986年ICSD-CRYSTIN手册；不是取得原手册。生产者首页只读身份/相关新闻版次，列2026-10-30的Web5.7.0，不读成历史文件版号。 | 消费者说明含多条目、分类/延续记录、晶胞缺值-1.0、位置/标准差和B/U/Beta等分别记录；占位还关联多重性。保留转录中待核字样，未从消费者表保证所有历史ICSD/CRYSTIN或目标兼容。1986/1991原手册、`.ics`与其它导出变体仍待核。 |
| EXT-WINGX-MAP，[作者主页](https://www.chem.gla.ac.uk/~louis/software/wingx/)、[修改记录](https://www.chem.gla.ac.uk/~louis/software/wingx/modif.html)、[MCE集成说明](https://sourceforge.net/p/mce-marching-cube-eld/wiki/Integration%20with%20different%20crystallographic%20packages./) | 读作者主页/相关功能条目、修改记录2023.1及1.70.00/1.64.02相关段、MCE的SHELX/WinGX及邻近格式说明，未读完整修改史或MCE源码。记录页首2023.1/2023-07-23，不绑定目标年代。 | WinGX的MapView用`.mpf`与MarchingCubes的`.fou`路线分开。MCE作者说明WinGX≥1.62 beta的Write MarchingCubes File可供MCE读取；它证明集成用途，未给`.fou`完整网格字段/字节布局。MCE同时列多种GRD来源，也不能据此合并成同一GRD规范。 |
| EXT-VENUS-LEGACY，[旧生产者页](http://fujioizumi.verse.jp/visualization/VENUS.html) | 原链接410；推测的JP-Minerals `/venus/en/`为404，推测路径明确保留。没有获得VICS/PRIMA格式正文，也未下载/执行旧程序。 | VICS `.vcs`、PRIMA `.pri/.prim/.den`仍为未读来源缺口；已读Dysnomia的pgrid输出章节不代替这些旧格式。 |

[覆盖表](FORMAT_SOURCE_COVERAGE.md)按上述证据更新来源类型与边界，保留47类旧手册输入/23个已观察数据输出。未读的原始规范、完整字段/边界和目标实际读写仍待核；样本/功能/科学状态不因本轮阅读提升为通过。
