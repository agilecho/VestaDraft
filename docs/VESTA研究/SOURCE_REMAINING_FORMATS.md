# S02.3 MOLDA、C04D与XTL来源续读（2026-11-08）

**当前任务状态（2026-11-10）：** S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 来源unknown revision10/high/investigating保留目标绑定边界；下文旧 revision/未勾选续记为历史，来源阅读不等于能力通过。接续见 [HANDOFF](HANDOFF.md)，阅读边界见 [SOURCE_NOTES](SOURCE_NOTES.md)。

承接[SOURCE_LEGACY_PRODUCERS](SOURCE_LEGACY_PRODUCERS.md)。本轮7条来源记录含4处新增正文选读、2处失效入口定位和1处VM18上下文复核，未取得全部历史规范，S02.3仍[ ]。[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-155158-s02-remaining-formats/)保存获取结果、身份、阅读范围和新检查；外部完整原文、PDF及阅读页只放临时目录。

后续梯度修正/参数/MEM与MINCRYST续读见[SOURCE_SCIENCE_PARAMETERS](SOURCE_SCIENCE_PARAMETERS.md)，来源问题接续revision8；本页原范围保留。

## MOLDA与旧OPT布局

EXT-MOLDA-DATA：[长尾/木原数据集说明](https://katakago.sakura.ne.jp/cc/mld/readme.html)读第3节至末尾注释/署名，第1–2节只核数据集来源；原Toyama入口TLS失败。该页保留1989年注记，当前修订页的数字发行号未建立。另读[吉田/松浦1997作者论文](https://www.sccj.net/CSSJ/jcs/v3n4/a2/text.html)的引言、§3.1 File菜单和参考文献3/9/10，确认MODRAST/MOLDA默认格式与XMol XYZ分开，没有全篇阅读或取得原1984/1995格式文献。

数据集说明把`.mld`组织为标题、原子数、每原子`x,y,z,原子序数`、键数及每行一对原子索引；示例6原子实际占第3–8行，第9行为键数。旧`.opt`则把原子数/键数/标题放同一头记录，原子记录多MMP2类型字段，连接记录每行最多8对；其Fortran定列说明不能挪给`.mld`。注释提到孤对电子的0/104编码、注册顺序与化学命名编号不同，这些是该数据集的历史约定，不能保证目标解析器接受。未从页面推出所有文件坐标单位、通用元素范围或错误恢复规则。

VM18第19项的通用行号公式与6原子例子不完全一致。本轮保留原文疑点，以例子的计数关系解释阅读结果，不静默改写上游手册，不把选读当解析实现或原件读取通过。

## SCAT、CONTRD与C04D

EXT-DVXA-CONTRD：[坂根公开手册](https://dvxa.org/hme/HidemaruDV/HidemaruDV.pdf)实际封面为2026-02-08、179页，搜索摘要中的2025不能作为版次。读封面与PDF/印刷99–103页的VENUS/CONTRD说明及C04D字段，实际查看封面、102、103页图像；未读整本、未下载/运行计算程序或受限程序。

C04D前三记录分别给三方向网格计数、盒子边长和中心；边长/中心用bohr，计数标作`3I5`，后两项标作`3F10.5`。第四记录给轨道数及轨道编号，标作`16I5`；自旋版第五记录给1=up、2=down，原文印作`X5,16I5`，其Fortran写法仍待原始程序核定。图中101/251计数与叙述中的100/250网格间隔应保留区别，不能据此宣布VESTA尺寸上限。

手册要求盒子容纳F01全部原子，C04D与F01配套；超界可能被当作周期等价位置。输出区分电荷密度、静电势和上下自旋波函数文件，补足使用语义，**没有给出SCA完整头、轴序或全部F01字段**。F01坐标单位在该文中表述为通常Å，不能升格为无条件格式规范。VM18第28/39项的同目录/叠加与场单位仅作目标手册自述；其波函数文件名单重复U项的疑点，结合此处U/D说明登记，不回改历史原文。

一条题为CONTRD的搜索结果实际下载成2014-08-06版、175页VESTA手册，本轮仅核封面，不把它算作取得`Readme_contrd.txt`。DVXA首页与目录为Shift_JIS，通用UTF-8文本提取含替换字符；原始字节仍保留，未据乱码作字段结论。原始CONTRD说明/程序版次、SCA完整布局及目标配套读取待核。

## XTL生产者说明与消费者代码

| 来源 | 实际范围 | 能支持的结论和限制 |
|---|---|---|
| EXT-GULP-XTL，[官方5.1命令说明](https://gulp.curtin.edu.au/help/help_51_txt.html)、[版次索引](https://gulp.curtin.edu.au/help/manuals.html) | 只读`output`中的xtl/xr/cssr/arc相关段及索引的版本映射；未读整个命令表。 | xtl写单结构晶体供BIOSYM读取，并自动补扩展名；xr、CSSR、archive分开。未给XTL全部字段，未取得原MSI规范，也未运行GULP。猜测的`/help/`目录404，正确索引由官方导航找到。 |
| EXT-CP2K-XTL，[固定提交源文件](https://github.com/cp2k/cp2k/blob/953180a62b30a8f3dc9eff4e1ed2aa9e1ef21684/src/topology_xtl.F) | 静态通读331行。`v2025.1`标记对象a5f79be7…，剥离提交953180a6…；两种raw URL字节相同，Git只读远端标签查询有记录。GitHub连接器返回Unknown tool后改用公开raw，无克隆、编译、执行或代码移植。 | 这是CP2K的消费者子集：大小写敏感检索，DIMENSION缺省3/其它值中止，CELL按Å/角度转换，原子分数坐标转内部实坐标；ATOMS下一行检查带特定空格的列头。代码寻找EOF，同时读取循环可在物理文件尾退出，不能说所有消费者强制EOF。SYMMETRY文字与SYM MAT操作分开处理；其内部矩阵/去重行为不提升为MSI或VESTA规范。 |

VM18第30项把XTL关联Cerius2/GULP/GSAS，输出项说明导出当前显示原子、分数坐标及P1。这是手册范围，不能从CP2K的标题空格检查推出VESTA相同行为。前轮MSI文档中的BGF/XTLGRF仍与此处XTL分开；`.xtl`同后缀不能建立等价。

## 仍未取得的Arai资料

EXT-ASSE：VM18所列[作者ASSE页](https://www.nims.go.jp/cmsc/staff/arai/asse/)404，未取得格式正文或版次。EXT-ARAI-EB：[Arai原主页](https://www.nims.go.jp/cmsc/staff/arai/)404，未取得`.eb`原始布局。VM18第33项称`.eb`用于费米面、与其所指`.rho`近似，并把本征值整体平移至非负；等值面应考虑该平移。此处仅复核手册上下文，**不能套用LAPW5二维rho格式，也未确定能量单位、偏移保存方式或每条能带布局**。

完整字典依赖、MINCRYST/VICS/PRIMA及其它原规范、核心论文/勘误和目标版契约仍待核。[覆盖表](FORMAT_SOURCE_COVERAGE.md)保留47输入/23数据输出，来源阅读不增加样本、CA evidence或目标/科学通过数。


## R133 F01 生产者清单补证（2026-11-10）

EXT-DVXA-BULLETIN33：坂根弦太2012-03发表的eduDV开发(3)，§3/3-1中的d3d26n内标2012-02-16 Version1.0。PDF6/9/10（印3/6/7）目视核定F01由unit8、2020格式写出Z/NEQ/三坐标，2X,I3,1X,I4,3(F10.5)；Unit标志0=angstrom、1=atomic，本八原子例写0。unit7另写d3d26n.out，格式3D20.10,I5；两种输出不能混淆。字体映射导致提取文本乱码，字段以PDF图像为准。来源、摘要和阅读边界见[SOURCE_NOTES](SOURCE_NOTES.md)R133与其台账；不是完整F01规范、所有SCAT变体或目标通过。原Readme及SCA完整布局缺口不回写成已读。


## R134 原始发布包与作者存档（2026-11-10）

此前失效的VENUS/ASSE/Arai来源已取回原始正文，当前身份/阅读范围以[SOURCE_NOTES](SOURCE_NOTES.md)R134及其台账为准，旧失败文字保留其时点。未运行旧程序，外部原文只在/tmp。

VICS/VEND手册89页，PDF创建2006-01-30；VICS附录A印46–51给导入/输出与旧vcs用途，原例包YBa2Cu4O8.vcs的111行显示TITLE/GROUP/SYMOP/CELLP/STRUC等及图形块；例文件不是完整关键字规范。ASSE0.5输入章及指定读取代码给begin/end、lattice及原子相对晶轴坐标，显式abc/xyz/prm从代码分别记录，长度单位未自行推定。

CONTRD原Readme实际全文已读，签署2002-09-30/修订2005-02-02；原contrd.f内末项更改2004-02-02。c04d盒边和中心为bohr，输出SCA头写3I5,3F10.5，体值按x最快、y行、z组排列，nx≤71用100E14.6、否则10E14.6。源文件首输入额外读IPRT，spin行用16I5读NDUM；不能把Readme印的X5当源程序原句。VEND印83补场单位密度bohr^-3、势Ry、波函数bohr^-3/2，皆不转换再绘制；这是该旧版本说明，非目标实测。

wien2venus作者页和2011-11-24脚本已静态通读，cell头、晶胞/角、网格三整数加长度和后续体值；作者说明k最快，脚本沿用lapw5平面次序。其字面拼写、非立方分支/计数校验局限保留；输出未运行，LAPW5二维rho不能替代。旧VEND eb段仅给近似三维rho与非负能量padding，Arai0.5同后缀能带列表/VTK明确不作为同一规范。

MCE固定SVN145 ReadFouFile438–800完整静态选读，为WinGX集成fou提供消费者实现：逐行矩阵/平移、晶胞角转度、网格范围/步长/尺寸、分片以及原子记录7/14/18等。其顺序是z外/x中/y内；identity矩阵分支另设P1。此处不把它等同于所有WinGX版或VESTA；没有源码移植。

R134补充EXT-F01-INPUT2014：坂根eduDV开发(5)，OUS信息处理中心研究报告35/2014-03，§6印15（PDF19）完整F01输入示例已目视。原文明确F01经MAKEF05生成F05，给Z/NEQ/XYZ、CHG/U-D/RD/VD节标题/分隔和Unit0=angstrom、1=atomic，另有Spin/M.P./Sample Point。与2012生产者writer交叉定位，未推广全变体。页顶bohr与Å括号换算与所印常数矛盾，不把它用于换算或目标预期值。

EXT-MOLDA65-AUTHOR-GUIDE：已从原作者Hiroshi Yoshida站取回Windows6.5 User Guide，G30 Opening/Importing、G60 Saving/Exporting全文及作者/版本页，原修改2001-02-02，2004存档；下载页另标6.5 rev2.0，不冒充同一手册修订号。明确MODRAST/MOLDA默认.mld、molda.prm可改.mol，XMol XYZ和其它导入/输出格式分开。相关两章未给坐标单位，原数据集1989说明中的字段仍沿用；未凭乙烯数值反推Å。
