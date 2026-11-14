# 领域公式与数值：规则、证据和未知

当前任务对齐（2026-11-10）：S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 当前 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open，即14个未解决；来源 revision10、数据库 revision4 保持。本次状态对齐没有新增目标证据或产品代码。 既有结果与失败边界保持；从 [HANDOFF](HANDOFF.md) 接续。

历史早期说明（后续R节已大量补证；当前整卡裁定见末尾S06本卡归档）：S06已开始选定补证，所有S06计划步骤仍未勾选。以下是从已登记官方样本独立计算的预期与原件显示比较，不是目标内部算法重建；其余密度、键价、散射/电子、畸变、衍射、磁性等仍需来源、函数和数值证据。

## 晶形的独立几何模型

入口/单位背景沿用[VM10官方晶形资料](../VESTA资料索引.md#vm10)与已有VS-0018，样本原字节提供晶胞和有向面。本轮数学模型假设各面形成有界凸半空间交；不假设该模型覆盖所有坏输入、非凸体或目标实现。

令实空间基A、B、C（Å）满足晶胞长度与夹角；V=A·(B×C)。无2π的倒易基a*=(B×C)/V、b*=(C×A)/V、c*=(A×B)/V，满足直接/倒易基内积为Kronecker delta。对于(h,k,l)，g=h a*+k b*+l c*，晶面间距d_hkl=1/|g|（Å）；单位法向n=g/|g|，晶形面距离D（Å）约束n·x≤D。不能混淆D和d_hkl，界面倍数为D/d_hkl。

独立脚本枚举3个不共面的约束平面交点，保留满足全部半空间的点；在每个面内按角排序顶点，以1/2|Σ n·(v_i×v_(i+1))|求面积（Å²），v已减面中心。没有调用VESTA/反编译算法/模型builder。[实际脚本](../../output/vesta-study/20261108-113622/reviews/20261108-200510-s04-multiphase-shapes/independent_shape_geometry.py)和[完整数学产物](../../output/vesta-study/20261108-113622/reviews/20261108-200510-s04-multiphase-shapes/shape-geometry-crosscheck.json)保留顶点、48面、原输入与原生输出ID；有界凸体、非奇异晶胞是显式前提。

R17先后关系明确：该轮GUI为2026-11-08 20:03:45–20:09:05（北京时间），独立数学产物为20:10:11，脚本与显示舍入校对属**事后独立复核**，不是预注册先验；几何公式没有调用目标或使用目标面积构造预期，数值独立性与时间上的事前性分别判断。旧证据原字节保留。

## 48面显示交叉核

8相共48面。第一相三对面积分别为333.1748950653、81.8699800881、240.6778843599 Å²；原件显示333.175、81.87、240.678。所有48面按六位有效数字格式化后与原件字符串逐项相同，[严格显示核对](../../output/vesta-study/20261108-113622/reviews/20261108-200510-s04-multiphase-shapes/shape-display-rounding-check.json)。早期产物另保留了较宽的数值容差比较；当前结论采用逐字显示舍入检查，不以宽容差掩盖差异，也不证明目标所有数值都用该格式。

第8相首面D=10.000028 Å（原文件），独立D/d_hkl=1.5103401012543785，格式化为1.51034，与原生选面字段相同。界面距离显示10是显示精度，不应回写为原文件恰等于10；未改变晶面或实际调用对称展开。

## 晶胞体积与表示控制

精确使用原十进制参数8.56、13、7.19、90、116、90得到V=719.1279925849117 Å³；原件显示719.128034，差4.1415088e-5，不能称只差最后一位舍入。把六参数先舍入IEEE754 binary32后再用同一双精度晶胞公式，得到719.1280335600926，六小数显示恰为719.128034：[表示控制](../../output/vesta-study/20261108-113622/reviews/20261108-200510-s04-multiphase-shapes/cell-volume-representation-control.json)。这是与binary32输入一致的确定性控制，本轮没有直接追此样本解析存储/内部算术，故不作完整因果证明或高精度体积算法验收。

## 历史：早期下一补证（当前按S06索引接续）

继续磁性/调制/准晶、几何测量、原子配位/畸变、键价/散射参数/电子密度、二维/衍射与外部程序；逐项补公式来源、单位、常数、精度和异常边界。已解释的原件缺陷不记科学结果通过，来源未得不复制受限全文。完整Study仍未完成。

## R18笛卡尔测距与选定成键区间

使用已有VS0011 C=(0,0,0)、O=(2,0,0) Å，独立范数sqrt((2-0)^2)=2 Å。原件Distance工具两次选点确认C1/O1，Output `l(C1-O1) = 2.00000(0) Å`：[原生测量](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/geometry-distance-pair-ui.stdout)、[独立/实际交叉核](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/atomic-geometry-crosscheck.json)。XYZ虚拟晶胞1 Å不使本次直接选点距离周期折回；不由此推断所有格式/PBC规则。

同样本手工C/O搜索规则，提交字段后，区间[1.9,2.1]、[1.9,2.0]、[2.0,2.1]均1键，[1.9,1.99]为0键；[已提交范围](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/geometry-bond-committed-ui.stdout)、[最大等于](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/geometry-bond-max-equals-ui.stdout)、[最大略小](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/geometry-bond-max-below-ui.stdout)、[最小等于](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/geometry-bond-min-equals-ui.stdout)分开。这里的“键”是用户几何范围生成的图边，不是独立化学键判断。

Deep [函数定位](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/geometry-bond-functions-search.stdout)找到SearchBonds2 0x13a58、CheckBond 0x14c88。[SearchBonds2 dossier](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/geometry-search-bonds-study.stdout)在0x13e0c加载相邻float界限，0x13e10先单精度平方、0x13e14升为double，换序保存。[CheckBond dossier](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/geometry-check-bond-study.stdout)以double坐标差累积平方距离d8，0x14da4的b.gt拒绝高于准备上界，0x14db0的b.mi拒绝低于准备下界；有序有限值相等不在此局部谓词被拒绝。上述准备/比较段未见显式epsilon；不能推广为全部上游筛选、精度邻域/非有限值或完整搜索实现。没有动态追本次GUI的该调用链，静态机制与原件有限样本观察分开。

首个Apply后原生记录仍0键，显式Tab/再次Apply/OK后为1，实际顺序保留，不把最初字段文本当模型已提交。4个有效区间比较以binary32输入平方再转double的准备规则计算，均与原件结果一致；边界2在此可精确表示。下一步继续非正交/PBC、角/扭转角、配位/畸变与异常边界。所有S06步骤仍未勾选。


## R24四原子角度、二面角与镜像

独立生成Cartesian C(0,0,0)、O(2,0,0)、N(2,2,0)、F(0,2,2)Å，不调用目标或反编译算法。令u=A−B，v=C−B，键角为acos(u·v/|u||v|)=90°；中央轴ON为+y，投影向量的atan2带符号参考为+45°；仅Fz改−2则参考−45°，无符号平面夹角均45°。C–F距离=√8Å。[先于结果的独立脚本](../../output/vesta-study/20261108-113622/reviews/20261108-220228-s06-angle-torsion/independent_geometry.py)和[期望产物](../../output/vesta-study/20261108-113622/reviews/20261108-220228-s06-angle-torsion/independent-geometry.json)。

原件GUI点选原子顺序/坐标由Output核实：phi(C1-O1-N1)=90.0000(0)deg，正向和镜像omega(C1-O1-N1-F1)均45.0000(0)deg；镜像报告Fz=−2，排除误开同文件；l(C1-F1)=2.82843(0)Å。[逐项原生显示核对](../../output/vesta-study/20261108-113622/reviews/20261108-220228-s06-angle-torsion/geometry-crosscheck.json)按4/5小数精度逐字格式化匹配；距离误差2.8753e−6Å，小于5e−6Å半末位。无坐标Esd的(0)只按观察记，不签误差传播正确。镜像结果表示本组目标使用无符号角值，不将常见带符号torsion约定硬套为其需求，也不由两点宣称全部角范围/退化行为。

Deep定位原库CalcAtomAngle0x75a78、CalcAtomTorsAngle0x75f84，分别传点数3/4到CalcGeomWithSUE0x74950，再调用CalcGeometry0x74198（1080字节）。核心2/3/4分支分别求距离/向量角/两个叉积法向夹角，归一化点积在0x7453c..0x74564按−1/+1限制后调用acos，再乘180/π；π立即数字节0x400921fb54442d18。[核心原生研究](../../output/vesta-study/20261108-113622/reviews/20261108-220228-s06-angle-torsion/geometry-core-study.stdout)。静态链与镜像均45的现象一致，本轮未做该GUI链现场断点绑定，不把寄存器不全的伪码当完整源码；零范数/共线/非有限、周期坐标、特殊位置和SUE仍待实际样本。

[CUA完整动作/边界](../../output/vesta-study/20261108-113622/reviews/20261108-220228-s06-angle-torsion/cua-observation.md)：Quit分别对两研究文档提示保存，依次No，正常exit0/300.248秒、非deadline；两输入未改。保存提示只归于完整导入/测量/切换链，dirty触发点未分离。镜像不是parent watch路径，用本地SHA和原生负坐标说明。新VS0051，旧50保持、镜像不重复编号；220证据/10头不变、7resolved3investigating，254功能/51样本，10完成19未完成，完整Study未完。


## R25周期平移像、点选距离与边界往返

VS0052独立P1晶胞a=b=c=10Å、角90°，C(0.05,0.5,0.5)、O(0.95,0.5,0.5)，占位1/Biso0。体积1000Å³，主像Cartesian为(0.5,5,5)/(9.5,5,5)Å，相距9Å；O加整数平移(−1,0,0)变(−0.05,0.5,0.5)，距C1Å。[先于原件的独立晶格枚举](../../output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/independent-periodic-expectation.json)枚举27平移，最近像为该唯一x平移；只用正交胞线性换算，不把逐坐标round推广为三斜最短像。

原件初始2个独立site实际显示4图像原子/2条键。点选C/O主像，原生l(C1-O1)=9.00000(0)Å；点选同C与左侧O图像，原生报告O fractional−0.05、Cartesian−0.5Å以及(−1,0,0)，l=1.00000(0)Å。[两种点选与字段交叉核](../../output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/periodic-crosscheck.json)。这组Distance工具按实际选中图像测量，不能在重写时悄悄把主像9Å结果替换为全局最短1Å。显示的跨边界连线和测量所选图像是两个行为维度。

Boundary默认xyz均0..1，x(max)2经Tab/Apply/OK提交后显示6原子/3键，重新打开实际读回2；改回1/Apply/OK恢复4/2。[初始边界](../../output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/periodic-initial-boundary-ui.stdout)、[两晶胞主树](../../output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/periodic-two-cells-ui.stdout)、[提交值读回](../../output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/periodic-two-cells-boundary-ui.stdout)、[恢复主树](../../output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/periodic-restored-ui.stdout)。Output中的Apply/OK重复计数保留，不误当重复原子。

用有理数枚举1Å的C–O边，若保留至少一端在x范围内的边并包含外端点，则0..1/0..2分别得到4/2和6/3；详见交叉核内conditionalImageModels。这是基于初始连接策略观察的条件算术一致性模型，不是测试前预注册的通用计数规则或完整算法恢复；其它边界模式/键规则/对称特殊位置未核。1Å只是几何探针，不作真实化学成键断言。

[CUA动作](../../output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/cua-observation.md)明确原子截图点选、原生树归属及退出：研究原件36492正常0/267.323秒、非deadline，Quit保存提示选No、CIF未改。新增7条native至227，10问题头不变，7resolved3investigating；新VS0052、旧51记录保持，254功能/52样本，10完成19未完成，完整Study仍未完成。继续非正交PBC、等价/特殊位置、配位多面体、退化/SUE及全部领域工具；Codex自验，独立复审归dsh尚未开始。


## R26既有VS0009非正交像与真实搜索模式

沿用原VS0009及先前独立预期，不重跑34格式矩阵、不新增样本号。a=2,b=3,c=4Å，γ=60°、其余角90°，C(0,0,0)、O(.5,.5,0)，O占位/Biso均.5。直接基a=(2,0,0)、b=(1.5,3√3/2,0)、c=(0,0,4)，主O=(1.75,3√3/4,0)Å，距离√19/2；邻O(−.5,.5,0)为(−.25,3√3/4,0)Å，距离√7/2。独立27平移枚举有两个等距最近像(−1,0,0)/(0,−1,0)，没有唯一性误判。[期望与精确metric](../../output/vesta-study/20261108-113622/reviews/20261108-222640-s06-nonorthogonal-images/independent-nonorthogonal-expectation.json)。

原件实际主像l=2.17945(0)Å、(−1,0,0)像l=1.32288(0)Å，原生分数/Cartesian/平移字段一致。两O图像围绕C夹角的cos=5/√133，独立64.3066190955°，原件phi(O1-C1-O1)=64.3066(0)deg。[先于测量的角预期](../../output/vesta-study/20261108-113622/reviews/20261108-222640-s06-nonorthogonal-images/independent-nonorthogonal-angle.json)、[原生数字核对](../../output/vesta-study/20261108-113622/reviews/20261108-222640-s06-nonorthogonal-images/nonorthogonal-crosscheck.json)按末位半单位逐字舍入相符。两端标签同O1，但来自不同平移像，不要求三种独立site才能测角。这个单一非正交例子不完成全部三斜最近像/误差传播/特殊位置。

原件初始122图像原子/116键；选中唯一Bonds行才读到真实C/O0..1.97249Å、recursiveextraatoms=1，[原生规则](../../output/vesta-study/20261108-113622/reviews/20261108-222640-s06-nonorthogonal-images/nonorthogonal-recursive-rule-ui.stdout)。未选行灰色默认C/C1.6/非递归不是现行规则。CUA改禁止越界后10/4；再缩显示范围到x[−.6,.6]、y[−.1,.6]、z[−.1,.1]后3/1。最终原生主树保留完整计数序列；修改后rule/range的再次原生对话框读回未完成，不能冒称已核全部字段持久化。Showpolyhedra=1而Output0，仅是规则开关，不签生成多面体通过。

首次Angle点组残留，后来先完成了另一中心/顺序phi(C1-O1-O1)=79.1066；该记录保留，目标结果使用随后完整新组的64.3066及各坐标。期间Wireframe状态变化原因未观察，已恢复Ball-and-stick；CUA保护拒绝与首次未命中不当目标算法失败。动作边界见[CUA摘要](../../output/vesta-study/20261108-113622/reviews/20261108-222640-s06-nonorthogonal-images/cua-observation.md)。

**生命周期本轮未通过正常关闭。** 原PID37980到570秒期限SIGTERM−15，No未成功执行；随后AX自动重启空白PID38111已菜单关闭，[清理和实际退出](../../output/vesta-study/20261108-113622/reviews/20261108-222640-s06-nonorthogonal-images/cleanup-final.json)。原始/工作CIF SHA均e661819c…保持，测量原生证据先于期限结束，不能把父包装器exit0混作GUI正常0。无需重测已回答的这三项数值；关闭案例沿用已通过R19/R24/R25而不伪称本轮。

新增5条native至232，10问题头完全不变，7resolved3investigating；52样本身份/摘要全部保持，仅VS0009补本轮几何证据/边界，254功能、10完成19未完成。完整Study仍未完成，Codex自验，独立复审归dsh尚未开始。


## R27反演一般位置、特殊位置与显示不对称单元

独立CIF显式给出identity与inversion，P−1、10Å正交胞，C(.1,.2,.3)占位.5，O原点占位1。对每点施加±坐标、模1归一并有理数去重：C得到(.1,.2,.3)/(.9,.8,.7)两唯一像，O只一个不动点。[先于GUI的独立产物](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/independent-inversion-expectation.json)。加权组成C=2×.5=1、O=1×1=1，中性原子Z的条件F000=14电子仍为独立预期，未从目标读回，不签散射/化学组成算法通过。

原件初始C标2i、O标1a，空间群#2，显示10图像原子/0键；这个10像与闭胞O八角平移像加2个C的独立计数相符；没有逐一点击八个O角点。显示平移像与特殊位置单重不是同一计数。将显示范围xyz0..0.99后为3像，恰等本例唯一位置集合；不是声称一般上限0.99等于半开1。[初始](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/symmetry-initial-ui.stdout)、[范围后](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/symmetry-halfopen-ui.stdout)。

点选两个C，原生实际报告C主fractional(.1,.2,.3)/Cartesian(1,2,3)Å，反演像(.9,.8,.7)/(9,8,7)Å，操作(1,1,1)+−x,−y,−z，同C1/occupancy.5。[等价像报告](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/symmetry-equivalent-images-ui.stdout)。Show only asymmetric unit勾选/取消时，原生checkbox1→0，CUA画面3→2→3对象，逆像恢复时仍是选中glyph；Output没有新计数行。只签显示过滤观察，不从旧日志推断内部数组始终3或无重建；样本文件最终未改。[交叉核对](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/inversion-crosscheck.json)、[CUA动作和视觉归属](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/cua-observation.md)。这些CUA图片不是正确window-id原生截图，截图问题未关闭。

[Summary页](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/symmetry-summary-ui.stdout)仅给空间群/晶胞/体积，不提供此处composition/F000读回。Deep定位并选读UnitCell::getWyckoffLetters原库0x89470/2360字节，[原生研究](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/wyckoff-sites-study.stdout)有矩阵/向量处理；未完成容差/完整去重算法和GUI现场调用绑定，不由函数名补写内部机制。

按拥有PID/时间预留关闭，本轮38361正常exit0/354.149秒、非deadline，早于360秒softstop；Quit保存提示No、输入未改，无AX重启。[生命周期](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/symmetry-gui-lifetime.json)。新增9条native至241，10问题头不变，7resolved3investigating；新样本VS0053、原52记录完全保持，254功能/53样本、10完成19未完成。更多空间群/设置/特殊位置、近重合容差、占位/ADP/组成、配位多面体等仍待，完整Study未完，Codex自验、dsh独立复审尚未开始。


## R28规则SiO6配位八面体

独立输入P1/10Å胞，Si(.5,.5,.5)，六O沿±轴偏离.2分数坐标（半径2Å），占位1/Biso0。中心与六顶点形成规则八面体：8个直角四面体各2³/6=4/3Å³，体积32/3=10.6666666667Å³；每面等边边长2√2Å、面积2√3Å²，总面积16√3Å²。[GUI前独立几何](../../output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/independent-octahedron-expectation.json)。这是有限几何探针，不是对这种键长/配位的化学稳定性断言。

原件初始Bonds列表为空、7原子/0键/0多面体；New明确Si–O1.9..2.1Å、Showpolyhedra1，[提交前原生选项](../../output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/octahedron-bond-options-ui.stdout)。Tab/Apply/OK后7原子/6键/1多面体，再选全局Polyhedral风格并旋转+x/+y。生成规则与显示风格分别经过实际动作，未将初始空列表或灰色默认Si/Si字段误当目标默认Si/O阈值。

点中心面实际命中POLYHEDRON报告，[原生结果](../../output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/octahedron-volume-report-ui.stdout)：六个不同O标签各l(Si1-Oi)=2.00000(0)Å，平均2.0000Å，体积10.6667Å³。独立体积按4小数恰同字符串，误差3.3333e−5Å³小于半末位5e−5Å³。[完整数值/归属核对](../../output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/octahedron-crosscheck.json)。

报告还给distortionindex0.00000、quadraticelongation1.0000、effectivecoordination6.0000；它们只构成规则几何的标称基准。未绑定完整畸变/伸长/有效配位公式，非规则样本/阈值/迭代/近退化仍待。表面积16√3仍是独立预期，本报告无实际表面积字段，不签目标已算。动作是3D选择→Output报告，**没有测试Utilities/GeometricalParameters或ORFFE流程**。

Deep原库定位_calc_Poly_Vol0xe770/704字节，_MakePolyhedraThread0x1179c/4148字节。[体积体](../../output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/polyhedron-volume-study.stdout)中0xe9b0把d0舍入s0、0xe9b4再拓回double，末尾0xea04除3、0xea08再转s0；不按伪码的returnx0裁定真实浮点返回或据此恢复完整算法。[生成体](../../output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/polyhedron-build-study.stdout)调用dgesdd于0x120c8，但本轮没有现场断点绑定GUI路径/中间矩阵；不由函数名声称通用凸包/非凸/全部精度契约。

[CUA动作/视觉边界](../../output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/cua-observation.md)和[生命周期](../../output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/octahedron-gui-lifetime.json)：原件38984正常0/309.52秒，softstop360前No关闭、源未改，无AX重启；CUA图不是正确window-id截图正控制。新增7条native至248，10头不变，7resolved3investigating；新VS0054、旧53记录保持，254功能/54样本、10完成19未完成。继续畸变多面体/组成/ADP/磁性准晶/体数据和全部领域工具，完整Study未完，Codex自验，dsh独立复审尚未开始。

## R29轴向畸变与手册公式控制

沿VS0054仅把O5/O6的z改.25/.75，四条Si–O2Å、两条2.5Å。作为同一几何代表的诊断变体，不新增VS号；原54身份/路径/SHA保持、其它53记录完全保持。目标运行前独立算得V=4×2×2×2.5/3=40/3Å³、平均13/6Å、Baur D=4/39、同体积规则八面体半径l0=10^(1/3)Å，λ=(4×4+2×6.25)/(6l0²)=1.023356477765。依据[官方11.4.3](https://jp-minerals.org/vesta/en/doc/VESTAch11.html)，ECoN先以最短长2Å的指数权重求加权长度2.0145449273763Å，再求权重和4.314817079115873。[先验计算](../../output/vesta-study/20261108-113622/reviews/20261108-232231-s06-distorted-octahedron/independent-distorted-expectation.json)、[公式/HTTP/摘要身份](../../output/vesta-study/20261108-113622/reviews/20261108-232231-s06-distorted-octahedron/manual-formula-source.json)。手册原HTML SHA与S02一致，仅存六数学公式/元数据，无外部全文复制；Robinson/Hoppe原论文全文未取得，手册是VESTA行为的一手说明，不代替原论文验读。

精简重试原件39507：初始Bonds空，Si/O1.9..2.6Å、Showpolyhedra1经Tab/Apply/OK后7原子/6键/1多面体。[选项](../../output/vesta-study/20261108-113622/reviews/20261108-232231-s06-distorted-octahedron/distorted2-bond-options-ui.stdout)。Polyhedral风格/+x/+y/选择中心，原生报告六不同O：O1..4=2.00000(0)Å、O5..6=2.50000(0)Å；平均2.1667Å、V13.3333Å³、D0.10256、λ1.0234、ECoN4.3148。[完整实际报告](../../output/vesta-study/20261108-113622/reviews/20261108-232231-s06-distorted-octahedron/distorted2-polyhedron-report-ui.stdout)、[五项显示精度核对](../../output/vesta-study/20261108-113622/reviews/20261108-232231-s06-distorted-octahedron/distorted-crosscheck.json)。五值都按4/5小数逐字相同且误差不超过半末位。

预登记重复加权至收敛的诊断候选ECoN4.3423836003，与实际4.3148明显区分；本例支持手册一次加权值，不把重复加权当重写目标。仅签选定有限正长度轴向变形，不推出所有构型/数值稳定性、完整Hoppe实现、原子电荷/键价、角方差/表面积、SUE、非凸/退化或Utilities/ORFFE工作流通过。

[CUA过程](../../output/vesta-study/20261108-113622/reviews/20261108-232231-s06-distorted-octahedron/cua-observation.md)如实保留首轮392.93秒/softstop超过、未Apply/未得报告、读取漏--allow-run的拒绝以及退出后AX误重启空白39437/正常关闭。重试报告后先核拥有PID，再读保存提示选No，不在退出后读AX；正常0/132.653秒、softstop前、无期限强杀、两CIF未改、ps无残留。[清理](../../output/vesta-study/20261108-113622/reviews/20261108-232231-s06-distorted-octahedron/cleanup-final.json)。CUA图片不代替正确window-id截图正控制。

新增4条native至252，原248逐记录保持；10问题头完全不变，7resolved3investigating。254功能/54样本，仅VS54补变体，不重跑34格式探针、不新增原论文验读或关闭来源问题。10完成19未完成、S06仍全部未勾选，完整Study未完，Codex自验、dsh独立复审尚未开始。

重试native parent的watched_paths生成误改了目录名为不存在的s06-distorted2-octahedron，实际before/after文件清单均空；不声称重试CIF由parent-watch核字节。首轮正确watch、独立输入期望SHA、重试本地同SHA、拥有记录argv以及原生报告实际路径/七点坐标分别保留，只有这些证据支持输入一致性；不为补监视重复数值测量。专项校验最初假定watch存在而KeyError，失败回执/脚本保留，已更正证据口径。后续路径变体只改Path.name并断言父目录存在。

## R31测距标准不确定度与显示向上取整

VS0055独立P1 CIF：a10.000(10)Å，C1 fractionalx.1000(10)、O1 .3000(20)，y=z=.5；对应σa=.01Å、σxC=.001、σxO=.002。在目标前，以d=a(xO−xC)=2Å和独立对角一阶传播，偏导(−10,10,.2)，方差=.0001+.0004+.000004=.000504Å²，σ=.02244994432064365Å；只坐标不计晶胞的诊断参考为.022360679774997897Å。[先验](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/independent-distance-sue.json)。[VM11](https://jp-minerals.org/vesta/en/doc/VESTAch11.html)说明所示SUE仅用对角误差估计、忽略未提供的协方差非对角项；此例P1/轴向差值不涉及对称约束、角误差或非零协方差，未验原引用70全文。

原件实际选中主像C/O，Cartesian(1,5,5)/(3,5,5)Å，Output **2.00(3)Å**。[原生测距](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-distance-report-ui.stdout)。真正选中C/O行后读回x/s.u.x分别.1/.001、.3/.002，Unitcell读a10/s.u.a.01000；未选行灰色0不是已解析值。[C1](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-C1-fields-ui.stdout)、[O1](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-O1-fields-ui.stdout)、[cell](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-cell-fields-ui.stdout)。仅从括号3无法区分接近的σ，不以此直接验传播通过。

原库CalcAtomDist0x75604调用CalcGeomWithSUE0x74950再在0x756f0调用getPrecision0xe5f0，[测距静态体](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-distance-study.stdout)。利用已登记R14同TEXT/原库调试副本现场只读：getPrecision参数σ=.02245001638122369Å、最大位数5；栈为getPrecision→CalcAtomDist→OnMouseLeftUp；返回小数位2、括号整数3，copy原生报告与原件相同。[native轨迹](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/native-sue-trace.stdout)、[事件](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-trace-events.jsonl)、[副本报告](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-trace-report-ui.stdout)。0EvaluateExpression/0显式业务写入、0回调错误，软件断点仍是仪器化；不冒称在不可变原件进程直接读到σ。

σ与精确解析值相差7.206058e−8Å，而非原先疑似.03−.02245的算法错误。既有[CalcGeomWithSUE体](../../output/vesta-study/20261108-113622/reviews/20261108-220228-s06-angle-torsion/geometry-sue-study.stdout)坐标+0.1σ于0x74e40、差平方×100于0x74d88..90；晶胞a+0.1σ于0x751c8再经0x751cc转binary32/0x751d0存回，差平方×100于0x75158..60。独立一维表示控制把临时a存为binary32：10.001→10.00100040435791；重新计算三项扰动差平方之和得σ=.022450016381224083Å，与现场仅差3.9205e−16Å。[控制脚本](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/representation_control.py)、[控制数值](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-representation-control.json)。这是依据静态实现的次级表示解释，独立解析oracle保持不变，不把目标算法反抄为独立真值。

[getPrecision静态体](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-precision-study.stdout)所选分支将σ×100后0xe6ec FCVTPS向+∞取整数，故ceil(2.2450016)=3、显示2.00(3)，不是强制五小数或普通四舍五入成(2)。本例原件/副本/原输入字段/原库机制和独立解析+表示控制已对应；全σ阈值/负非有限值、全部构型/角与扭转/特殊位置/协方差仍未覆盖。[数值及边界核对](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-crosscheck.json)。

[CUA动作与退出](../../output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/cua-observation.md)：原件40500正常0/214.409秒，副本43219正常0/98秒内；两次Quit后先核拥有PID再读No，不在退出后取AX，parent-watch两CIF字节保持，无VESTA残留。dirty触发只归完整导入/测量/对话框链，未分离；CUA图不冒充CA图。新增12native至268，原256逐记录保持；10头完全不变，8resolved2investigating。新VS0055为带误差CIF代表，原54记录完全保持，trace副本不再分号；254功能/55样本、10完成19未完成，完整Study未完，Codex自验、dsh独立复审尚未开始。

## R32占位、特殊位置、密度DC与反演

沿用VS0053，不重编ID。P−1 C一般重数2×占位.5=1C，O反演中心重数1×占位1=1O，中性Z预期F000=6+8=14，V1000Å³，平均密度.014e/Å³。目标前登记总积分容差1e−4电子、完整网格反演差1e−5e/Å³；只作本例binary32数值控制，不当普适FFT误差界。网格点数未预猜为10/2=5，手册称受对称/分辨率约束。[独立先验](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/independent-density-f000.json)与[VM14 14.7/14.9](https://jp-minerals.org/vesta/en/doc/VESTAch14.html)占位结构因子/1/V逆Fourier公式、[VM18网格](https://jp-minerals.org/vesta/en/doc/VESTAch18.html)周期重复边界分开。

原件Resolution2Å字段经Tab/OK，实际C/O占位/2i/1a、散射参数、**F000=14**和PeakSearch完整Output归档：[原生报告](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/f000-density-peaks-ui.stdout)。Summary实际6/6/6、V1000：[原生摘要](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/f000-grid-summary-ui.stdout)。峰搜索CUA参数max0/all、negativeoff，报告5峰、Voronoi边界216/216、两数28.284113与−14.284113；后续独立正/负标量求和逐一按6小数相同，**它们是正、负部分积分，不是实部/虚部**。5峰不能据此等同3原子像或独立化学原子。

保存新VESTA+相邻pgrid，另ExportData实际选ggrid和CUBE，没有重开/重测既有密度引用失败。pgrid头(周期1/fType1/nVal1/dim3/6³/nAsym112)，ggrid头(一般0/fType1/7³/nAsym112)，byte128后完全相同。基本头后120字节元数据、112个int/float对；本例索引指内部7³一般网格，解点后mod6/I−I可填满216无冲突，6³反演固定点8，(216+8)/2=112。初按6³索引猜测产生46冲突/只填171已拒绝。[解码边界](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/binary-decode-notes.md)。这是本例解释，尚非全部fType1/中心化/有理平移格式规范。

完整216值CUBE独立读取（x最慢/z最快），反演差实际0；不以已强制反演补值的binary数组自证。两种导出标量按CUBE的.5E精度全部相同。独立binary网格积分**13.999999978598984e**，误差−2.1401e−8e，在预登记1e−4内；正28.2841125208/负−14.2841125422，两部分符合原件显示。最低−.1871961504、最高.8650593162e/Å³，共102负体素；粗分辨率截断可有噪声，未据此证明全部物理密度或峰拓扑正确。对输出C/O拟合系数的零角和为13.996904，明显不同于实际DC14；只登记这一区别，不由输出反推完整F000特殊处理机制。[独立脚本](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/analyze_density_f000.py)、[原生运行/文件SHA绑定](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/native-density-file-controls.stdout)、[全数值](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/density-f000-crosscheck.json)。该native运行的是自己的解析/算术脚本，不冒作目标原生算法本体。

**CUBE单位兼容尚未通过。** 坐标步长3.149544、6步是约bohr表示的10Å胞，但标量按导出精度等于内部e/Å³值；依据先前已读Gaussian cubegen的一般原子单位约定，若用CUBE自身坐标体素体积直接积分，得**94.476674238654e**而非14。CUBE三原子行核电荷和20也不编码C的.5占位，不能据行电荷求本例电子数。[原Gaussian来源阅读](SOURCE_SUPPLEMENT.md)；本次刷新timeout保留，未重试/冒称新全文获取。新问题`unk_3f5d08b5dc777b37` rev2/high/investigating，精确writer/消费者单位与格式语义待核；nativeverify valid=false的原因正是not marked resolved/no resolution text，不当异常或已解决。[添加](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/cube-unit-unknown-add.stdout)、[更新](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/cube-unit-unknown-edit.stdout)、[未解决校验](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/cube-unit-unknown-verify.stdout)。

[CUA动作](../../output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/cua-observation.md)保留guard重复匹配被拒绝、误快捷键打开Phase/Cancel、后按菜单导出。拥有50858正常0/326.522秒、softstop360前、Quit无保存提示观测，之后无AX/重启/进程残留。parentwatch源CIF/工程/pgrid，ggrid/CUBE在独立分析native watch绑定，不混称最初parent监视它们。

新增5至273native，旧268逐记录保持；旧10问题头完全保持，新CUBE单位问题使11头=8resolved3investigating。55样本身份保持，仅VS0053补密度控制，其它54记录完全不变；254功能、10完成19未完成。密度DC/占位特殊位置/反演有限控制通过，完整散射/ADP/Fourier/PeakVoronoi及CUBE格式科学兼容未过。完整Study未完，Codex自验、dsh独立复审尚未开始。

## R33 CUBE标量单位行为与本程序往返

R32单位问题`unk_3f5d08b5dc777b37`现rev3/resolved，指选定原因解释闭合，**不是单位兼容修复或全CUBE验收**。R32模型电子密度内部积分约14，而同导出按Gaussian一般原子单位解释为94.476674；原件行为是长度坐标转bohr，标量保持数据集原值。[选定交叉核](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/cube-unit-crosscheck.json)、[解决](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/cube-unit-unknown-resolve.stdout)、[nativevalid](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/cube-unit-unknown-verify.stdout)。这并不等于宣称通用CUBE语法无效；任意标量场、Gaussian电子密度的单位语义及本程序自有往返应区分。

原库export_data0xb1a80分派含计算跳转，短dossier只有300字节并未覆盖case。[初始分派](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/cube-export-dispatch-study.stdout)和外部objdump未按范围裁剪的限制保留。改用只读LLDB加载不可变原库arm64，按b1b24..b6300读4599指令、未启动VESTA目标；[native加载/读指令运行](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/native-offline-export-inspection.stdout)、[全定址指令](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/export-body-arm64.json)、[诊断源码](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/inspect_export_lldb.py)。唯一CreatedbyVESTAv%s字符串0x500117由0xb2bcc取，分支在b2b80..b2f24。

晶格/步长在0xb2c30..58等用常数bits0x3fe0ef050a9737de=**0.5291772086Å**换算长度；原子坐标在0xb2d98..b2e00用bits0x3ffe3c51782fcacd=**1.8897261328499326**倍数。这些是原件常数，不冒称当前CODATA值。标量循环0xb2ef8从数据数组加载s0，0xb2efc仅float→double，0xb2f00作fprintf参数，0xb2f0c按`%12.5E`打印，中间无密度体积单位因子。它与R32原件输出216标量按该精度逐项一致；没有把坐标换算误读为密度单位也已换算。

[原库readCUBEfile核心0xbeb7c](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/cube-core-reader-study.stdout)对正网格数的向量另作bohr→Å换算，而标量fscanf%f后直接存float，最后补周期边界。原版-nogui读取R32存档CUBE，再显式-saveggrid，得到gType0/fType0/nVal1/dim3/7³/343的raw网格；全部343值与原CUBE解析为binary32并补周期边界**逐位相同**，maxdifference0。不是外部Gaussian执行结果。[原件直接ggrid导出](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/native-cube-ggrid-roundtrip.stdout)。

读回晶胞10.000000953674316Å，恰等binary32(6×3.149544×0.5291772086)；这独立确认坐标换算。排除127重复边界点后，按本程序Å胞体积积分13.999996719928912电子，CUBE文本舍入和坐标舍入保留，不冒称完全回到原binary值13.9999999786。按原CUBE原子单位坐标直接积分的94.476674仍然存在；**原程序的保标量往返不证明Gaussian电子密度单位正确**。R32CUBE三原子行核电荷和20不编码.5占位，仍不据此宣称原结构组成错。

首次-saveVESTA只保存IMPORT_DENSITY对原Cube的引用，没有自动pgrid，[首次实际产物](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/native-cube-roundtrip.stdout)保留；之后按VM17实际支持的明确扩展-saveggrid才得到数值网格，没有重开VESTA工程/重跑已回答引用失败。两次原件255、约.25/.19秒是已解释nogui返回，实际验收依产物/字段/内容，不能用driver0证明操作成功。

[方法限制](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/method-limitations.md)明确无现场writer断点绑定、无外部Gaussian消费者执行，负维数单位/非零origin/MO/NVal多值/其它场单位/坏输入/03与09版本/所有消费者仍待。未来重写的科学单位政策与1:1行为选择未在此作实现裁定，先如实记录原行为。

本轮新增9至282native，旧273逐记录保持；仅此问题2→3/resolved，其它10头完全保持，11头=9resolved2investigating。254功能/55样本记录全部保持，无新样本；10完成19未完成，完整Study未完，Codex自验、dsh独立复审尚未开始。

## R34非零复数结构因子与预设波长

沿VS0055 P1 C(.1,.5,.5)/O(.3,.5,.5)、占位1/Biso0；坐标/胞的Esd不改变本次中心值期望，未验F误差传播。先从[OASYS/DABAX维护数据](https://raw.githubusercontent.com/oasys-kit/DabaxFiles/main/f0_WaasKirf.dat)独立读取中性C/O各11参数，按f0(s)=c+Σaᵢexp(−bᵢs²)、s=|h|/(2a)和**F=Σg f0 exp(+2πi h·r)**自行计算，不导入目标代码/库。dmin9Å/10Å胞枚举六±轴反射；Friedel对若合并有3组，但没有预先把目标行数定为3。[独立数据身份/选行](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/independent-scattering-source.json)、[先验](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/independent-structure-factors.json)。完整表仅临时目录；HTTP200/42473bytes/SHA8294958f…，没有以维护数据代替1995原论文全文。C b3外表42.086842、原件先前打印42.086843，差1e−6照实保留，未将整组系数宣称逐字相同。

原件Structuralmodel/Xray/CuKα1、dmin9/Ignoreanomalous1实际提交，[原生选项](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/structure-factor-options-ui.stdout)。Calculate的CUA结果表实际六行，AX不暴露格子；Export保存[原件文本](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/structure-factors.txt)，由parentnative watch取SHA。±100的F(real)同2.241675，F(imag)分别±10.795576，|F|11.0259；±010/±001 F(real)−13.547204、imag按6小数±0、|F|13.5472。d全10.000000、M全1，未作Friedel减半。[完整数值交叉核](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/structure-factor-crosscheck.json)。正100先验2.241674612+10.795576012i，模11.025858992、相位78.269375762°；实/虚/模、d与同角I比例均按输出精度符合。

**相位无直接输出列**，只由实际已舍入复数分量推导，误差以分量半末位传播界核对；不能写成直接读取phase字段通过。相对I为本例同d/P1M1下100|F|²/max|F|²，100反射66.24079、其它100.00000；共同角因子在这个比中抵消，不据此恢复所有极化/多重性/强度政策。

波长初按AX文本1.54059Å作精确输入，独立Bragg2θ=8.8356830496°，而原件8.83570，超过5小数半末位；[初失败差值](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/initial-display-comparison-failures.json)、[失败脚本](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/analyze_structure_factors.failed-display-wavelength.py)保留。没有改Fh的独立外部Oracle。原件退出后，用已登记同TEXT/原库调试副本专门读取**计算前输入**：DlgStrucFac::OnCalculate调用CalcI的0x10007c638，d0=9、d1及this+0x440模型double=**1.5405929Å**、this+0x448float dmin=9，AX仍显示1.54059。[现场事件](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/wavelength-model-events.jsonl)、[真实native轨迹](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/native-wavelength-model-trace.stdout)、[副本相同选项](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/wavelength-trace-options-ui.stdout)。0EvaluateExpression/0显式业务写、0回调错误，软件断点仍仪器化；不冒称直接读原件内存，也未从2θ反拟合λ。

据实际预设输入作独立Bragg计算得到**8.8356997148672°**，按5小数恰为8.83570。30个非θ数值与独立数据公式符合，6个θ通过额外实际输入诊断才解释；合计36字段逐字舍入相同且误差均≤半末位。显示舍入值不能替代模型真值；其它预设/Custom手输/失焦提交行为未测。[OnCalculate静态体](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/structure-factor-calculate-handler.stdout)确认从+0x448 float/+0x440 double传CalcI，[OnXray体](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/structure-factor-xray-handler.stdout)把表值存+0x440再设置文本。静态与现场分开，不由不全伪码补写完整界面数据绑定。

[CUA归属与退出](../../output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/cua-observation.md)：原件52347正常0/160.954秒、诊断副本52640正常0/92.683秒，均softstop前，无Quit保存提示观测、退出后不AX、不重启/无残留；CIF字节保持。原件Export后自动回主界面，副本未重导出原件数值。只有指定低阶P1/B0/忽略反常散射，完整ADP/有序无序/其它晶胞/消光/中子/反常/波长/不确定度/所有输出仍待。

新增11至293native，旧282逐记录保持；11问题头完全不变，9resolved2investigating。55样本身份保持，仅VS0055补本轮控制，其它54记录完全保持，未给诊断副本/文本输出新ID。293最终导出保存native-evidence-final.json；先前285条的native-evidence-after.json作为波长调查前的不可变中间导出保留，不覆写。254功能、10完成19未完成，完整Study未完，Codex自验，dsh独立复审尚未开始。

## R35各向同性位移参数与部分占位的复数贡献

沿VS0055诊断衍生P1/10Å胞/C(.1,.5,.5)、O(.3,.5,.5)，坐标/胞Esd保留但只验中心值。先依据[IUCr B/U定义](https://www.iucr.org/__data/iucr/cifdic_html/3/CORE_DIC/Iatom_site.B_iso_or_equiv.html)与[2023论文§3–4式9/14/17](https://journals.iucr.org/j/issues/2023/03/00/dv5004/index.html)，登记 **F=Σg f0(s)exp(−Bs²)exp(+2πih·r)，B=8π²U，s=1/(2d)**。f0沿R34独立DABAX中性C/O数据，未取目标系数/库作Oracle。三控制分开：占位1、B=(2,4)Å²；占位1、U=(.02533029591058444,.05066059182116889)Å²；B0、占位=(.5,.75)。[先于全部GUI的独立期望](../../output/vesta-study/20261108-113622/reviews/20261109-012100-s06-isotropic-occupancy/independent-isotropic-occupancy.json)。

原件分别读三份CIF，Output确认为B2/4、U约.025/.051与占位.5/.75；未在EditData里修改/提交字段。三次均Xray/CuKα1、dmin9Å、Ignoreanomalous1，native计算前选项及计算后Output保存。每表均完整六±轴反射，M1。原件B/U两份导出**原字节完全相同**；100的F=2.242453+10.704929i、模10.9373，其它轴模13.4409，100同角I66.21546。独立期望100=2.242452518+10.704928974i、模10.937280176，B衰减exp(−.005)/exp(−.01)各自乘原子振幅，不把B误乘在强度或整体结构因子上。

部分占位100原件.518421+7.251833i、模7.27034，独立期望.518421472+7.251833301i、模7.270340229，同角I69.46590；其它轴模8.72306。三例18反射、**108数值字段**误差均≤各自输出半末位；|F|随量级显示不同小数位，本轮按实际精度核而非一律4小数。±0保留文本，科学核对以数值零。λ采用R34已查明同CuKα1预置1.5405929Å的已知输入，未声称本轮直接读取原件内存。[全部独立核对与原件文件名](../../output/vesta-study/20261108-113622/reviews/20261109-012100-s06-isotropic-occupancy/isotropic-occupancy-crosscheck.json)。

[取证限制/生命周期](../../output/vesta-study/20261108-113622/reviews/20261109-012100-s06-isotropic-occupancy/method-observations.md)：B文件保存框输入被拼接为iso-b-working.txtiso-b-factors.txt，原parent watch仅预登记短名，**未监视真实结果**。保留原名及缺口，不重算；后续独立分析native程序显式watch实际文件/SHA，属于稍后取证，不倒写为GUI父进程监视成功。U及占位两次用setValue并回读文件名，原parent watch覆盖实际输出。AX不暴露六结果行，只CUA截图现场见表，科学文本来自真正Export。occupancy中辅助选择器遇diff找不到checkbox，完整刷新仍0后实际点击，再native确认1后才计算。

三个原版拥有PID53063/53127/53176均正常0，分别75.362/72.107/89.558秒，均softstop前，无Quit保存提示观测/退出后AX重启、无剩余进程，输入字节保持。没有新LLDB/调试副本或产品实现。只选定正交P1、两元素与低阶反射；各向异性/交叉项/非正交变换/负参数校验/误差传播/全散射仍待，不勾选完整ADP或S06。

新增10至303原生证据，旧293逐记录保持，11问题头完全不变9resolved2investigating。55样本路径/SHA/ID保持，仅VS0055追加控制，其它54记录不变，无新样本号。前34轮3906历史文件保持；254功能、10完成19未完成，完整Study未完，本轮Codex自验、dsh独立复审尚未开始。

## R36各向异性U张量与三个交叉项

沿VS0055正交P1/10Å胞的诊断衍生，C/O各占位1，非零U张量(Å²)分别为C[[.01,.005,−.003],[.005,.02,.002],[−.003,.002,.03]]、O[[.04,−.01,.004],[−.01,.05,.005],[.004,.005,.06]]；先用Sylvester三个顺序主子式各正确认本例正定，不把它冒称原件参数校验。CIF含Uani/完整Uij loop，scalar Ueq=.02/.05。沿R35一手IUCr式15/17、R34外部DABAX系数，自行登记 **T(hkl)=exp(−2π² ΣᵢⱼUᵢⱼhᵢhⱼ/100)，F=Σ f0 T exp(+2πih·r)**；正交10Å胞下a*=b*=c*=.1Å⁻¹，双重求和自然含交叉项2U12hk等。[先于GUI的张量/18反射先验](../../output/vesta-study/20261108-113622/reviews/20261109-013800-s06-anisotropic/independent-anisotropic.json)。dmin6.9Å只保norm²1/2两组，未预设全部I跨组公式。

原版Xray/CuKα1、Ignoreanomalous1、dmin6.9实际提交，原件Export保存18行；**real/imag/|F|/d/2θ共90字段误差均≤实际输出半末位**。λ沿R34同预置输入1.5405929Å，没有本轮直接读原件内存。100=2.251453+10.730587i、|F|10.9642，010/001模分别13.448/13.4214，显示方向不同。110=−2.112708−10.352694i、模10.5661；1−10=−2.148463−10.309323i、模10.5308，实际模差.0353、独立期望.03525449227885，差在合并半末位.0001内。若把两个张量各用trace/3替换为等效球，同|h|两反射的模会相同；实际数据能区分交叉项贡献，但不推广为所有张量/非正交证明。[全部核对](../../output/vesta-study/20261108-113622/reviews/20261109-013800-s06-anisotropic/anisotropic-crosscheck.json)。跨两种d的I仍留原表，未用简单|F|²全局比错误验收其角因子。

Calculate/Export后进入EditData仅选C和O行，native分别读回六U字段，**12个导入值与输入准确一致**，含U12/U13/U23的符号；未选行灰零不作数据，未写字段/Apply，Cancel回主窗。[CUA与生命周期](../../output/vesta-study/20261108-113622/reviews/20261109-013800-s06-anisotropic/cua-observation.md)。原件53508正常0/134.861秒，softstop前，无Quit提示观测，退出后不AX/无自动重启或残留；parent watch完整覆盖源CIF和真实导出文件，输入SHA保持。后续native分析再监视数值文件、两份树、源码及结果。

静态[CalcStructureFactors0x3542c](../../output/vesta-study/20261108-113622/reviews/20261109-013800-s06-anisotropic/anisotropic-core-function.stdout)1256bytes/314指令，0x35480调用[UnitCell::get_U_Beta_trans_coefficients0x8c474](../../output/vesta-study/20261108-113622/reviews/20261109-013800-s06-anisotropic/anisotropic-U-beta-conversion.stdout)340bytes/85指令；按原arm64三sincos/胞体积和六store可导出helper输出顺序为2π²[a*²,b*²,c*²,a*b*,a*c*,b*c*]，常量位0x4033bd3cc9be45de=19.739208802178716。胞长乘积先以binary32执行再提升double，角先float提升double。这里只是静态helper，未现场调用或全部非正交数值验收。core含OpenMP worker数据引用0x373f8/0x37558；[已读后者](../../output/vesta-study/20261108-113622/reviews/20261109-013800-s06-anisotropic/anisotropic-structure-worker.stdout)在0x37640按vtable+0x40间接调用，目标/实际分支未绑定，未宣称恢复全部核心算法。[静态边界](../../output/vesta-study/20261108-113622/reviews/20261109-013800-s06-anisotropic/static-path-scope.json)。

新增8至311原生证据，旧303逐记录保持、11问题头不变9resolved2investigating。55样本身份保持，仅VS0055追加anisotropicControl，其它54记录不改，无新样本号；前35轮4054历史文件/原包257保持。只指定正交P1正定张量与反射，不覆盖各向异性非正交转换/负定参数校验/椭球渲染/不确定度/全散射。254功能、10完成19未完成、Study未完，Codex自验、dsh独立复审尚未开始。

## R37中子结构因子、负散射长度与手输波长

沿VS0055正交P1/C(.1,.5,.5)、O(.3,.5,.5)/B0占位1；第二份只将C改H作为诊断。先读[NIST历史表](https://www.ncnr.nist.gov/resources/n-lengths/list.html)表头/自然元素H/C/O三行，单位fm，页自述取NeutronNews3(3)(1992)29–37；bC=6.6460、bO=5.803、bH=−3.7390。依据VM14.7中子和式 **F=Σg bc T exp(+2πih·r)**、本例B0/T1，独立登记六±轴反射，目标Neutron、λ手输2Å/Tab、dmin9Å/Tab。另一元素URL重定向的[NIST表](https://www.ncnr.nist.gov/resources/activation/scattering_table.html)选定值为H−3.7409(11)/C6.6472(9)/O5.8037(29)，日期未建立；两份预测都在GUI前登记，历史值是兼容性对照，不以它替代当前推荐数据或完整版本裁定。[来源与读取失败](../../output/vesta-study/20261108-113622/reviews/20261109-014900-s06-neutron/independent-neutron-source.json)、[双版本独立先验](../../output/vesta-study/20261108-113622/reviews/20261109-014900-s06-neutron/independent-neutron.json)。web无www表open超时、www表选段可读；urllib raw403保留，无rawSHA/HTTP200声明，不重复请求绕过。

原件切Neutron后CUA完整树见Xray靶和Ignoreanomalous灰掉，λ可编辑；实际输入2与9各Tab提交，两份native预计算树确认SourceNeutron/λ2/d9。Calculate/Export每份六反射、M1；AX不暴露结果行，实际文本导出作为核对。C/O正100=3.583501+9.425402i、模10.0836，其它轴模12.449，100同角I65.60927；独立历史值期望3.583501326+9.425401751i、模10.083634261。H/O正100=−4.818140+3.321252i、模5.85194，其它轴模2.064，同角I12.43998，独立期望−4.818140160+3.321251906i、模5.851938895。符号符合负bH参与复数和，未把散射长度取绝对值。所有2θ=11.47834，与2asin(2/20)=11.4783409545°符合；这里只核整数2/Tab提交，不推广为全部文本精度/无效值/Xray Custom绑定。

两例12反射**72数值字段**均在实际打印半末位内符合历史表值；原件Output bc亦实际C6.646/O5.803/H−3.739。原表F无单位列，这一数值尺度与所用fm散射长度和式相符，不冒称文件头声明了fm。另一NIST表预测的real/imag/模中**28字段超出输出舍入**，完整差值保留；这是两资料数值区别与所选原件兼容性事实，不能反推出原包完整表来源/版本，也不以计算通过证明当前物理推荐参数正确。[原版两表及完整双版本对照](../../output/vesta-study/20261108-113622/reviews/20261109-014900-s06-neutron/neutron-crosscheck.json)。

[CUA归属/退出](../../output/vesta-study/20261108-113622/reviews/20261109-014900-s06-neutron/cua-observation.md)：原件53840/53952正常0，117.822/105.512秒，均softstop前；无Quit保存提示观测，退出只inventory/ps不AX，无自动重启/残留。两parent watch完整含实际源/导出，源SHA保持；后续native分析监视数据/树/脚本/结果。未写内存/LLDB、未执行外部中子程序。只有P1/B0自然元素三种选定值，全部同位素/吸收复数bc/磁散射/角度全域/误差仍未验。

新7至318原生证据、旧311逐记录保持，11头不变9resolved2investigating。55样本身份仅VS0055加neutronControls，其它54记录不改，H/O诊断不新编ID；前36轮4177历史文件/原包257保持。254功能、10完成19未完成、完整Study未完，Codex自验、dsh独立复审未开始。

## R38单斜P21螺旋消光、反射轨道与强度分字段

新VS0056独立CIF：a10/b12/c14Å、β100°、其余90°，P21/#4，给定两操作I与(−x,y+1/2,−z)，C(.1,.2,.3)/occ1/B0生成(.9,.7,.7)。独立倒易metric由直接metric求逆；V=1680sin100°=1654.4770250605Å³。两像直接求和得F=f exp(2πik·.2)[exp(2πi(.1h+.3l))+(−1)^k exp(−2πi(.1h+.3l))]，故0k0/kodd严格零。dmin8Å枚举完整12HKL，两个±010禁戒、10允许点；空间群旋转轨道(h,k,l)/(−h,k,−l)共7含禁戒、5允许轨道。[先验](../../output/vesta-study/20261108-113622/reviews/20261109-020000-s06-monoclinic-screw/independent-monoclinic-screw.json)。未预定软件输出完整点或轨道代表、禁戒省略或零行，外部f0仍R34独立DABAX。

原件CuKα1/d8/Ignoreanomalous1实际五行001/100/0−11/011/10−1，各M2，恰覆盖10允许点；±010省略。d/real/imag/模/2θ共**25个先验字段**误差≤实际输出半末位；原件volume1654.477025、C1 2a/2atoms符合。[全对照](../../output/vesta-study/20261108-113622/reviews/20261109-020000-s06-monoclinic-screw/screw-crosscheck.json)。0−11 F=10.306426+3.348761i、011=−10.306426+3.348761i，实际关系F(0,−1,1)=−conj(F(0,1,1))，不把螺旋平移关联的复F视为完全相同；本例没有额外Friedel合并。既有P1 M1/保留±h与本例P21 M2均保留，软件表M不可直接替换为文献Laue粉末M口径。

**强度是事后补充解释**，先验不改：[IUCr2023论文§7.2式29面积L/式32非偏振P](https://journals.iucr.org/j/issues/2023/03/00/dv5004/index.html)组合给I∝M|F|²(1+cos²2θ)/(sin²θ cosθ)，再max100归一，本例5I分别26.11276/87.15022/100/100/9.82866均匹配半末位。式36逐点无cosθ分母的另模型并非全部匹配，不将面积公式称为所有物理情形通式；所有M2在本例比中抵消，单靠数值不能隔离M乘法。故本轮明确25先验字段+5事后I字段，不倒写成30先验或全部强度政策通过。

[CalcI0x60ff4](../../output/vesta-study/20261108-113622/reviews/20261109-020000-s06-monoclinic-screw/screw-CalcI-function.stdout)1040bytes/260指令：CreateReflectionList0x61184、SortEquivHKL0x6118c；参数p为raw w4==1取0、其余.5，LP=[(1−p)+p cos²2θ]/[2sin²θ cosθ]存ref+88，ref+90=1。先ref+78存|F|²、求max(|F|²LP M)，再ref+78乘100/max。**该字段本身不是最终I**。[主OnExport](../../output/vesta-study/20261108-113622/reviews/20261109-020000-s06-monoclinic-screw/screw-export-handler.stdout)调用[Reflection::ExportData0x6f81c](../../output/vesta-study/20261108-113622/reviews/20261109-020000-s06-monoclinic-screw/screw-reflection-export-function.stdout)，其0x6f964/974/990/994再乘ref+78、+88、+90、轨道计数及额外export scale；main把grid+380值作为scale传入，本轮未新读该内存值，也未全测source/scale分支。[静态布局/边界](../../output/vesta-study/20261108-113622/reviews/20261109-020000-s06-monoclinic-screw/intensity-static-scope.json)。所选原件数值与该组合相符，但虚调用/所有生成分支未全绑定。

本次默认十列文本d/real/imag为固定6小数、模**%12.6g即六有效数字**、2θ/I固定5小数、M整数；静态还含Phase/ID(λ)十二列分支，未实测，不扩为已通过。[CUA归属](../../output/vesta-study/20261108-113622/reviews/20261109-020000-s06-monoclinic-screw/cua-observation.md)：原件54201正常0/116.167秒，softstop前，无Quit提示观测，parent watch覆盖真实源/导出、CIF保持，退出后不AX/无残留。新9至327证据、11头保持9resolved2investigating；旧318逐记录保持。新增VS0056，其它55样本记录完全保持；前37轮4310历史文件/原包257保持。254功能/56样本，10完成19未完成，Study未完，Codex自验、dsh复审未开始。

## R39非正交晶胞的常规CIF Uij与自动等效U

新VS0057独立P1单斜胞10/12/14Å、β100°，C(.1,.5,.5)/O(.3,.5,.5)各占位1；沿R36两个正定U张量，输入Uani/完整Uij，**不提供Uiso/equiv列**。独立解倒易Cartesian q=(h/10,k/12,(l/14−h cosβ/10)/sinβ)，|q|/2用于外部DABAX f0；常规CIF U二次型是ΣUij hi hj ai*aj*，a*=1/(10sinβ)、b*=1/12、c*=1/(14sinβ)，不是直接把同一数字视为Cartesian分量。T=exp(−2π²二次型)，复F沿正指数。dmin6.9得到22完整反射；本轮在GUI前已把R38查明的面积LP归一I也纳入先验，不再是事后I模型。[132字段先验与Cartesian另模型](../../output/vesta-study/20261108-113622/reviews/20261109-021600-s06-nonorthogonal-adp/independent-nonorthogonal-adp.json)。

原件CuKα1/d6.9/Ignoreanomalous1，实际22行、M1；d/real/imag/模/2θ/I共**132个先验字段全部在实际打印半末位内**。100先验F=2.247656738+10.718452799i，模10.951583959、I32.506790896；同一U数字作Cartesian分量的另模型在42个real/imag/模字段超舍入，区分坐标约定，而非调整Oracle追原件。[完整核对](../../output/vesta-study/20261108-113622/reviews/20261109-021600-s06-nonorthogonal-adp/nonorth-adp-crosscheck.json)。λ沿已查明同CuKa1预置1.5405929Å、未新读原件内存；全部M1，M效应仍不能单靠本例I比隔离。

原版在输入未给Uiso的情况下自动给C/O粗输出.021/.051；观察后另登记追加诊断，**不冒称原132先验部分**，并在读选中精确字段前写期望。依据[VM06的Ueq式](https://jp-minerals.org/vesta/en/doc/VESTAch6.html)，Ueq=(1/3)ΣUij ai*aj* ai·aj；本胞化为[(U11+U33+2U13cosβ)/sin²β+U22]/3，独立C=.0207726436055532、O=.0505589145145792。EditData仅选择C/O后native读**.0207726/.0505589**，误差均≤7小数半末位，区别于简单trace/3的.02/.05；两份树六U分量亦保持。未编辑/Apply，Cancel回主窗。[追加诊断登记](../../output/vesta-study/20261108-113622/reviews/20261109-021600-s06-nonorthogonal-adp/secondary-ueq-expectations.json)、[缓存资料身份/选段](../../output/vesta-study/20261108-113622/reviews/20261109-021600-s06-nonorthogonal-adp/ueq-source-record.json)，不把手册引文56/57原论文记成全文已读。

[CUA归属/退出](../../output/vesta-study/20261108-113622/reviews/20261109-021600-s06-nonorthogonal-adp/cua-observation.md)：原件54624正常0/200.133秒，softstop前，parent watch完整含真实输入/导出、源SHA不变；无Quit提示观测、退出后只inventory/ps不AX，无重启或残留。没有LLDB/业务内存写；实际结果为原Export文本，AX无格子行，后续native分析另监视文件/树/源码/结果。只特定单斜P1与两张量，所有三斜/约束/重定向/表示转换/负定/椭球/不确定度等仍未验。

新增6至333原生证据、旧327逐记录保持，11头不变9resolved2investigating。新VS0057、原56记录完全保持；前38轮4436历史文件/原包257保持。254功能/57样本，10完成19未完成、Study未完；Codex自验，dsh复审未开始。

## R40解析Fourier合成：Fc、Fo与差密度

新VS0058仅一行通用8列：h/k/l/|Fo|/|Fc|/Fc(real)/Fc(imag)/sigmaF = **1 0 0 8 5 3 4 1**，用既有VS55 P1/10Å胞作晶胞载体。按VM14.8指定唯一反射而不提供−100；三种输入复系数独立先验为Fc3+4i、Fo4.8+6.4i、Fo−Fc1.8+2.4i；F000均明确手输8、Resolution2Å。ρ(x)=1/1000[8+2Re(F100 exp(−2πix))]，故Fc=.008+.006cos2πx+.008sin2πx，另两种分别正弦/余弦系数乘1.6/.6。[GUI前的全部先验/容差](../../output/vesta-study/20261108-113622/reviews/20261109-022700-s06-analytic-fourier/independent-analytic-fourier.json)。逐点绝对容差1e−7 e/Å³、DC积分容差1e−4 e只用于本小幅单模/float FFT及6有效位文本，不冒称通用FFT误差预算；未预定网格数。显式±h配对诊断文件仅准备、未运行。

三份独立原件各实际选择Fc/Fo/Fo−Fc、字段F0008/Resolution2，各导出6×6×6 CUBE。native分别留mode单选框、参数、主Output；格子行AX不可读，CUA截图实见3+4i、4.8+6.4i、1.8+2.4i。**648唯一体素逐点全符合**，max误差Fc3.2303e−9、Fo2.51685e−8、差2.19382e−8；积分7.999999999999993/7.999989999999987/7.999993333333318均符8。单独正100输入得到与实密度共轭补全相符的幅度，负指数相位给正sin项，反号模型及不除体积模型明显不符。[完整核对/每x层覆盖所有yz体素](../../output/vesta-study/20261108-113622/reviews/20261109-022700-s06-analytic-fourier/analytic-fourier-crosscheck.json)。本例y/z常量，不独立证明两轴一般顺序。

独立离散正向求和（离线分析，**未操作原件StructureFactors/Volumetric分支**）恢复F100：Fc3+3.999998135i、Fo4.8+6.399994129i、差1.8+2.399993107i；差来自文本舍入量级。三个F000都8，故ρdiff=ρFo−ρFc+.008，实际最大线性残差1e−8，在三份逐点容差和3e−7内；不是无DC偏置直接相减。CUBE按R33已查明的**raw内部密度标量**解释，Gaussian原子单位兼容失败不因此消失；没有外部Gaussian消费声明。

输出区三次均“F000(+)=0/F000(−)=0”，并非这三张密度积分。[后续独立静态绑定](../../output/vesta-study/20261108-113622/reviews/20261109-022700-s06-analytic-fourier/fourier-static-label-scope.json)：DlgFourier::OnImport0x10004661c在文件框之前0x10004667c/668c调用当前原子模型CalcF000EE/NE；NE0x37338按元素+78的核散射长度×多重性×占位分正负加和，打印后返回总和。所以它早于随后用户输入F0008；初始元素参数为何为零尚未现场绑定，不补写推测。分析JSON生成时标签尚未绑定，原文件false标志按native监视SHA保持，之后静态说明单列。便利汇总曾对callee.name=null过滤报TypeError，只读解析错误已记录/修正，原native调用exit0不重跑。

OnCalculate0x10004641c实际从dialog+428(float)取resolution→UnitCell::CalcGridNo，+420(double)取编辑F000→FourierSynthesis0x5f214，之后检查等值面、重建Scene、PushHistory2。核心2516bytes/629指令含pocketfft c2r<double>调用0x5f9f8及基矢矩阵行列式0x5fa04；不把不全虚调用/第三方核版本标成全验。Calculate后Showsections/isosurfaces实际启用，Quit出现保存提示，三次明确No，输入字节保持。

[生命周期/方法限制](../../output/vesta-study/20261108-113622/reviews/20261109-022700-s06-analytic-fourier/method-observations.md)：Fc55020正常0/275.970秒、Fo55185正常0/171.913秒、差55269正常0/155.396秒，均softstop前，退出后仅inventory/ps、不AX，无重启/残留。因Fc已用较久，后两种另启原件而未重算Fc；第一parent只生成Fc，后两parent分别覆盖Fo/差真实文件，所有实际源/结果watch完整。完整Fourier/其它空间群、消光/相位缺失/零幅度、fcf/mem/fos、第四Fo−Fo、编辑/异常输入/可视化均待。

新20至353原生证据，旧333逐记录保持，11头不变9resolved2investigating。新增VS0058，其它57记录完全保持；前39轮4554历史文件/原包257保持。新增行为03实质规格承接本轮体数据合同；254功能/58样本，10完成19未完成，Study未完，Codex自验、dsh复审未开始。

## R41解析体数据Patterson自相关

本轮复用VS0058/R40已独立验证的Fc CUBE作输入，不新增样本ID；**目标算子的Oracle另行先验推导**：ρ=.008+.006cos2πx+.008sin2πx、F0008、F1003+4i、V1000Å³，P(r)=∫ρ(u)ρ(u+r)du=1/V Σ|Fh|²exp(−2πih·r)=**.064+.05cos2πx**，连续范围.014..114，积分64。GUI前另对原始216个文本标量独立循环卷积，给出六个x位移.11399997016/.08899998508/.03900001492/.01400002984/.03900001492/.08899998508。预登记逐点容差1e−6 e²/Å³、积分.002e²；名义V1000未按目标输出拟合。[先验及输入身份](../../output/vesta-study/20261108-113622/reviews/20261109-025400-s06-patterson-volume/independent-patterson-volume.json)。

原件55761读入CUBE后Output分两层：体数据6³/10Å胞、体积1000.000286Å³；独立原子层为1Å胞、C(1,5,5)/O(3,5,5)，体积1。Utilities/PattersonDensities/FromVolumetricData打开SelectLayer，两个同名radio只有首个体数据层可选；明确选首项后OK。native留两层Output及首项value1。不能拿原子层1Å³替代体数据体积。

File/ExportData→CUBE，setValue后回读patterson-density.cube，再Save。本轮导出6³、0原子记录；**全部216体素匹配解析式**，最大解析差2.08167e−17（输出文本恰为.114/.089/.039/.014/.039/.089），相对逐点独立离散卷积最大差2.98400000126e−8；V1000积分64、反演偶对称残差0。离线独立正向DFT回收P00064、P10025.000000000000075+1.15648e−15i，吻合|3+4i|²25、相位消失。没有操作原件StructureFactors/Volumetric分支。本例y/z恒定，仍不证明一般三轴顺序/三维非平凡路径。[逐点/积分/静态摘要](../../output/vesta-study/20261108-113622/reviews/20261109-025400-s06-patterson-volume/patterson-volume-crosscheck.json)。CUBE按已查明raw内部Å密度标量解释，Gaussian原子单位互操作失败结论保持。

[静态核心](../../output/vesta-study/20261108-113622/reviews/20261109-025400-s06-patterson-volume/patterson-core-function.stdout)Crystal::CalcPattersonDensity0x61850，1512bytes/378指令：61880/6188c两次MatrixD::Determinant；61904..61910以det/N将内部float密度送double；61afc pocketfft r2c；61c28..61c38复系数Re²+Im²存实部、虚部清零；61c70 c2r；61c90倒det、61d24..61d30输出乘1/det转float；输出端点按网格数取模写周期padding。静态与选定运行结果分别支持归一/模方，不声称所有优化分支或第三方核版本已绑定，原件未插桩。

**方法偏差保留**：owned softstop360/hard570秒；恢复上下文时已靠近softstop，导出/取证/关闭至392.461秒，超softstop32.461秒，未标作softstop内通过。进程正常exit0、deadlineTerminated=false、parent进程组verified且文件清单未截断；Quit保存提示明确No、源字节不变。无退出后AX/getApp重启，无遗留原件。[完整方法边界](../../output/vesta-study/20261108-113622/reviews/20261109-025400-s06-patterson-volume/method-observations.md)。后续单实例在进入软限前应结束新探查并预留关闭时间，已完成数值不重跑。

新7至360原生Evidence；旧353逐记录保持、11未知头不变9resolved2investigating。样本仍58，前57记录完全保持，只给VS58追加自相关派生证据；前40包4738文件/原包257保持。254功能、10完成19未完成；仅选定体数据Patterson成立，电子/核模型分支、一般三维/空间群、异常输入、视觉/状态持久化仍待，Study未完、Codex自验、dsh独立复审未开始。

R41来源限定：VM14.11的FromVolumetricData离散F式印作Σρexp、未显式写dV；本轮事先采用物理自相关积分/直接卷积Oracle，原件静态实际先乘det/N、末除det，数值符合该带体元归一化合同。这里记录手册省略与实际算子的区别，不称逐字符照搬手册离散式。

## R42体数据正向结构因子

本轮复用VS0058/R40 Fc CUBE，**实际运行原件StructureFactors的VolumetricData分支**，不再仅离线求和。GUI前逐216个独立体素登记Fh=V/N Σρexp(+2πih·r)，V1000、dmin9Å、10Å立方胞应含六个±轴/P1M1；+100独立DFT=3.000000000000002+3.9999981349995655i，−100共轭，其余四轴约0，不含000。[全部先验](../../output/vesta-study/20261108-113622/reviews/20261109-030800-s06-volume-structure-factors/independent-volume-sf.json)。预设绝对容差d2e−5Å、复部/模长各1e−4，涵盖先前R33已知Cube文本/float晶胞与FFT/Export舍入；没有从本次目标输出拟合容差。角度/归一I另作已登记次级预测，非24个主数值字段。

原件56123加载volume/atom两层，StructureFactors默认phase1为volume，CalculateFrom下拉禁用且值VolumetricData；SourceXray/CuKa1/λ1.54059仍可见，IgnoreAnomalous禁用。只设dmin9/Tab，native留实际控制后Calculate，CUA实见六行。ExportsetValue并回读volume-sf-factors.txt，Save后自动回主窗口。[真实导出](../../output/vesta-study/20261108-113622/reviews/20261109-030800-s06-volume-structure-factors/volume-sf-factors.txt)六行d10.000001/M1；+100=**3.000001+3.999999i/|F|5**、−100共轭，四轴复部/模/强度全0，±100 I100，2θ8.83570。**24个预登记数值全符**，六个多重性与反射集合相符，F000不列；I/角度次级预测均符。[核对](../../output/vesta-study/20261108-113622/reviews/20261109-030800-s06-volume-structure-factors/volume-sf-crosscheck.json)。

事后另用R33已经查明的导入cell binary32值10.000000953674316、体积1000.0002861023222，原始密度文本转binary32再作独立DFT：全部实/虚部均在6小数末位半单位内。此解释单独标为**postGUI诊断**，不挪作24个主先验；不是原件插桩，也没有重算/修改结果。选定正向/逆向环节都各自对独立数学核过，不能只以往返相等自证。y/z常量，完整三轴/非平凡3D及Nyquist边界仍未验。

[静态](../../output/vesta-study/20261108-113622/reviews/20261109-030800-s06-volume-structure-factors/volume-sf-core-function.stdout)Crystal::CalcStructureFactorsVol0x359b4，1296bytes/324指令；359e8取MatrixDdet，35a4c..35a58算det/(NxNyNz)，35b18..35b28输入float转double并乘该体元；35c44 pocketfft r2c<double>。取反射部分先检查abs(h/k/l)≤floor(Nx/y/z/2)，packed索引按h符号翻转k/l并取周期余数、选择虚部符号（35cd8..35d28，双条路径35dd4..35e1c/35e74..35ebc）；本轮±100验证实际正相位，越Nyquist处理仅静态、未现场。未验证全kernel版本/SIMD支路或“改变其它选项无影响”的完整手册断言。

原件56123正常exit0/**91.373秒**，soft360前；Export自动关dialog，Quit无保存提示，输入字节与parent监视SHA保持。无退出后AX/getApp重启、无遗留；没有一般脏状态/所有设置持久化结论。[方法边界](../../output/vesta-study/20261108-113622/reviews/20261109-030800-s06-volume-structure-factors/method-observations.md)。

新6至366原生Evidence，旧360逐记录保持；11头仍9resolved2investigating。58样本不新增，前57记录保持，仅VS58追加原件正向分支取证；前41包4858文件/原包257保持。254功能、10完成19未完成，Study未完，Codex自验、dsh独立复审未开始。

## R43非等轴四谐波Fourier与三轴布局

新VS0059为四行通用8列Fc输入，P1晶胞10/12/14Å、90°载体：F100=3+4i、F010=−2+i、F001=1−3i、F111=2+2i，均仅给唯一正反射；F0008、resolution2。GUI前登记ρ(r)=[8+2Σ(ReFh cos2πh·r+ImFh sin2πh·r)]/1680、积分8、逐点1e−7/积分1e−4容差。**未预定网格数**；对原件实际NxNyNz按(i/Nx,j/Ny,k/Nz)逐点核，非拟合密度/维度。[先验](../../output/vesta-study/20261108-113622/reviews/20261109-031600-s06-fourier-three-axes/independent-three-axis-fourier.json)。各轴系数不同且带111混合项，用来区分此前单x例无法区分的布局。

原件56372实际选择Fc导入，CUA表实见四行上述复系数/M1及d10/12/14/6.73489，native留Fc单选值1、F0008/res2字段。Calculate后Close再ExportData→CUBE、setValue回读three-axis-density.cube，真实导出**6×6×8/288独立体素**。全部符合解析式，max误差**4.8437046765853475e−8**；V1680积分**8.000000450866658**在先验容差内。[全部288逐点结果](../../output/vesta-study/20261108-113622/reviews/20261109-031600-s06-fourier-three-axes/three-axis-fourier-crosscheck.json)。CUBE按x外层/y中间/z最快解释，六种坐标排列只有identity符合，另外五种max误差0.0118022/0.0121383/0.0203691/0.0203121/0.0167407，均远超1e−7；本轮明确支持该非等轴四模输入的XYZ输出顺序与负指数相位，未泛化所有格式/内存布局/维数。

独立离线正向DFT恢复四复系数均在1e−4内；没有本轮操作原件正向分支（R42才是真实singlex正向），不以原件往返自证。一般空间群/消光、Nyquist奇偶/高频、非正交体数据/其它文件格式/渲染及Gaussian原子单位互操作仍待或既有不兼容保持。此次不重复调用已经留存R40的核心静态体，不宣称新增核路径/版本证据。

原件56372正常exit0/**153.565秒**，soft360前，Quit提示明确No；CIF/反射文本/稳定样本三份SHA均保持，parent进程组verified/文件清单未截断。无退出后AX重启/遗留。[方法限制](../../output/vesta-study/20261108-113622/reviews/20261109-031600-s06-fourier-three-axes/method-observations.md)。新5至371原生Evidence、旧366逐记录保持，11头不变9resolved2investigating。新VS0059，旧58条记录全保持；前42包4975文件/原包257保持。254功能/59样本、10完成19未完成，Study未完，Codex自验/dsh独立复审未开始。

## R44三轴体数据正向与Patterson算子

复用VS0059/R43独立逐点核过的6×6×8 CUBE，另在GUI前登记两个算子Oracle，**不是拿原件往返互证**。正向Fh=1680/288 Σρexp(+2πih·r)，按10/12/14Å正交度量/dmin6.5枚举28个完整P1/M1反射；包含±100、±010、±001、±111八非零项，余20项应近零（包括±002），d/实/虚/模共112个预先数值。Patterson P(r)=[64+2(25cosX+5cosY+10cosZ+8cos(X+Y+Z))]/1680，288个位置全先验、积分64，逐点1e−6/积分.002容差；正向d2e−5Å、复部/模1e−4。[完整先验](../../output/vesta-study/20261108-113622/reviews/20261109-032400-s06-three-axis-operators/independent-three-axis-operators.json)。

原件56723默认volumephase1/VolumetricData、dmin6.5/Tab后native回读，Calculate/Export真实[28行文本](../../output/vesta-study/20261108-113622/reviews/20261109-032400-s06-three-axis-operators/three-axis-forward.txt)，**112数值全符**、反射集合与M1全符；随后FromVolumetricData打开SelectLayer，明确选首个体数据radio，再导出[三轴Patterson CUBE](../../output/vesta-study/20261108-113622/reviews/20261109-032400-s06-three-axis-operators/three-axis-patterson.cube)。仍6×6×8/0原子记录，**全部288体素符合解析式**，max差7.23449101181e−8；另逐位移用原输入288点直接周期卷积，max差5.38451594492e−8；V1680积分64.00000873、反演偶对称残差0。[全部算子核对](../../output/vesta-study/20261108-113622/reviews/20261109-032400-s06-three-axis-operators/three-axis-operators-crosscheck.json)。

Patterson原点.0952381，范围−.0190476..+.0952381；带符号输入密度允许负自相关，(x,y,z)=(.5,.5,.5)先验−32/1680≈−.019047619，不是失败/人为截零。体数据导入volume1680.000141Å³与独立atomlayer1Å³继续区分，先验保持nominal1680并按已登记float/文本容差验收。沿用R41/R42静态算子解释，无新增重复函数取证；不称全kernel/所有空间群/非正交/高频Nyquist奇偶/显示/persistence已验，Gaussian单位互操作结论保持。

原件56723正常0/**152.875秒**、soft360前；两个输出均setValue/readback路径，Quit保存提示明确No、输入字节保持，parent进程组verified/清单未截断，退出后无AX重启/遗留。[方法](../../output/vesta-study/20261108-113622/reviews/20261109-032400-s06-three-axis-operators/method-observations.md)。新5至376原生Evidence、旧371逐记录保持，11头仍9resolved2investigating。仍59样本，前58记录全保持，只给VS59追加算子取证；前43包5090文件/原包257保持。254功能、10完成19未完成，Study未完、Codex自验/dsh独立复审未开始。

## R45线剖面：三线性插值、距离与文本输出

复用VS0059/R43已核过的6×6×8 raw内部密度，GUI前按VM14.13线性插值说明独立登记：Point1=(0,.125,.25)、Point2=(1,.625,.75)、N13，t=i/12包含两端；每点以周期8角tensor-product线性权重插值原始288个文本标量，**不重建连续解析场代替网格插值**。不等轴10/12/14Å晶胞给总长sqrt(10²+6²+7²)=**sqrt185=13.601470508735444Å**，而非分数坐标欧氏长度。预定d绝对容差1e−4Å、密度1e−6；13点/26字段全先验。[推导](../../output/vesta-study/20261108-113622/reviews/20261109-033200-s06-line-profile/independent-line-profile.json)。

原件56998在volumephase下Utilities/LineProfile打开ExportLineProfile，CUA初见两端(0,0,0)/(1,0,0)、N7；本轮明确填上述两端/N13，Tab后native回读七字段。OK→SavePanel，setValue/readback three-axis-line.txt后Save。[真实输出](../../output/vesta-study/20261108-113622/reviews/20261109-033200-s06-line-profile/three-axis-line.txt)为title、Point1/Point2三分数坐标、13及13行distanceA/density。**26数值全符**，最大距离差5.36699250908e−6Å、密度差4.9999999991e−9；连续解析场模型max差.00137514380413，明确区分网格三线性与直接再合成。第13点x1周期corner与两端包含得到选定运行证据。[逐点核对](../../output/vesta-study/20261108-113622/reviews/20261109-033200-s06-line-profile/line-profile-crosscheck.json)。

[exportLineProfile0xc45f8](../../output/vesta-study/20261108-113622/reviews/20261109-033200-s06-line-profile/line-export-function.stdout)736bytes/184指令：c4744 N−1、c476c..c47b8以binary32 start+i*(end−start)/(N−1)构造点；c47cc调用Calc_Physical_Value_at；c4688/c4804 cblas_dgemv用当前晶胞矩阵变Cartesian，c4808..c4830双精度差/平方根，c4834降float再升double输出。Point1/2格式`%8.5f`三项，数据行`%14.6E %14.6E\n`。fopen失败静态回调“Permissiondenied”且返回false，但没有现场失败路径验证。

[Calc_Physical_Value_at0x32150](../../output/vesta-study/20261108-113622/reviews/20261109-033200-s06-line-profile/line-interpolator-function.stdout)672bytes/168指令：binary32坐标×网格数后转double/modf，负小数段调整为floor式正fraction，索引周期余数；322e4..32334八float角点，32350..323c8双精度tensor-product权重/FMA，323cc转float返回。负坐标域外、N1/零/负N、非正交、文件写失败/路径冲突和其它volume层仍未运行，不能把静态枝节当现场行为全验；render/外部绘图软件也未验。

**方法失败保留**：首次native只读参数取证命令的project和@params均漏`20261108-113622`路径段，exit2，stderr为cannotreadparams，未得到Evidence；原回执/空stdout/error保留。新名字line-options-ui-correct-project使用正确参数exit0、实际七字段核对，不重跑GUI计算。原件56998正常0/**73.401秒**、soft360前，Quit无Saveprompt，源parentSHA保持，清单未截断/进程组verified，无AX重启/遗留。[方法记录](../../output/vesta-study/20261108-113622/reviews/20261109-033200-s06-line-profile/method-observations.md)。

新6至382原生Evidence，旧376逐记录保持、11头不变9resolved2investigating。59样本不增，前58记录保持，仅VS59追加线剖面取证；前44包5205文件/原包257保持。254功能、10完成19未完成，Study未完/Codex自验/dsh独立复审未开始。

## R46线剖面负周期、反向与点数归一

复用VS0059网格。GUI前对两个新案例独立周期8角插值/晶胞度量先验：negative从(−1,.125,.25)→(0,.625,.75)，reverse反向；各N13、各26数值，合计**52字段**，相同d1e−4Å/密度1e−6容差。另预先列N1/N0为**输入校正探索，没有数值科学先验**，不把i/(N−1)底层代码推成GUI必定除零。[先验](../../output/vesta-study/20261108-113622/reviews/20261109-034000-s06-line-boundaries/independent-line-boundaries.json)。

原件57369四次打开LineProfile，每次控制重置(0,0,0)/(1,0,0)、N7；负/反向各填写七字段/native回读，导出[negative](../../output/vesta-study/20261108-113622/reviews/20261109-034000-s06-line-boundaries/negative-line.txt)/[reverse](../../output/vesta-study/20261108-113622/reviews/20261109-034000-s06-line-boundaries/reverse-line.txt)。两个13点**52字段全符**，两案max密度差均5e−9、距离5.36699250908e−6Å；反向密度序列与负周期正序倒排仅差2.5e−9，距离仍从新起点0正向递增至sqrt185Å。负坐标模数/方向行为有选定现场证据，不泛化全部大数/轴/非正交。

N1和N0分别在默认(000)→(100)线段上输入/native回读（字段仍1/0），OK均进入SavePanel，未提示拒绝；真实[single-point](../../output/vesta-study/20261108-113622/reviews/20261109-034000-s06-line-boundaries/single-point-line.txt)/[zero-point](../../output/vesta-study/20261108-113622/reviews/20261109-034000-s06-line-boundaries/zero-point-line.txt)却**headerN2、两行有限数**，两文件byteidentical。距离0/10Å、密度两端.009523810。这里是实际N0/N1归一化合同，不是接受未定义单点除零值；输出两端的数值仅事后解释，不并入52个GUI前字段。[全部核对](../../output/vesta-study/20261108-113622/reviews/20261109-034000-s06-line-boundaries/line-boundaries-crosscheck.json)。

沿导入stub0x10045ae0c的唯一caller找到[VestaFrame::OnLProfile0x10009ca64](../../output/vesta-study/20261108-113622/reviews/20261109-034000-s06-line-boundaries/line-bound-gui-handler.stdout)，4196bytes/1049指令。**接受文件框后**10009d8cc读取局部N(slotx29−84)，d8d0 cmp1/d8d4 b.gt跳过，否则d8d8 mov2/d8dc回写；d99c从同slot载入w2，d9a8调用exportLineProfile。明确绑定GUI→max(N,2)→底层N−1的合同，R45“单看export不能推GUI校验”的限制得到回答。首轮宽regex搜进wxListLineData、结果47/返回30，原结果保留；窄literal找到stub，再沿caller定位，未声称宽搜为完整功能枚举。静态N负数同枝，但**负N/非数字/超大N未现场**，完整wx解析/校验仍待。

四个输出均setValue/readback不同文件名，原件57369正常0/**202.487秒**、soft360前；Quit无保存提示，输入parentSHA保持、清单未截断/进程组verified，无退出后AX重启/遗留。[方法](../../output/vesta-study/20261108-113622/reviews/20261109-034000-s06-line-boundaries/method-observations.md)。新10至392原生Evidence、旧382逐记录保持；11头仍9resolved2investigating。59样本不增、前58记录保持，只给VS59追加线边界取证；前45包5325文件/原包257保持。254功能、10完成19未完成，Study未完/Codex自验/dsh独立复审未开始。

## R47双向量半格二维切片与导出

复用VS0059 6×6×8体数据，按VM15.5.2/15.7在GUI前登记two-vectors/001、V1[100]/V2[010]、center(.5,.5,.3125)、distance0。平面覆盖x/y0..1、物理10×12Å，z=.3125位于原格2/8与3/8中点；独立Oracle为同xy的两个原始gridplane值取平均。手册NxNy6的“点数/区间数/闭合端点”措辞预先列歧义，**49点闭合候选**及全部密度先登记，允许实际header证实或保留不符，不把候选默认为既定事实。[先验](../../output/vesta-study/20261108-113622/reviews/20261109-035000-s06-vector-slice/independent-vector-slice.json)。

原件57701 Utilities/2DDataDisplay→Slice→two-vectors，实际001/V1[100]/V2[010]/center(.5,.5,.3125)/offset0与xd0全部native回读，OK绘图后File/Export2DData。真实[vector-slice.txt](../../output/vesta-study/20261108-113622/reviews/20261109-035000-s06-vector-slice/vector-slice.txt)header**7 7**、49行，确认本例NxNy6对应区间、包含两端；x外层/y内层，localXY为0..10/0..12Å。**49密度/98坐标均符合GUI前候选**，max密度差5.00000000171e−10、几何差9.99999999252e−7Å，容差分别1e−6/2e−5。所选z两格平均成立，闭合边界重复保留；不泛化全部切片模式/斜胞。[逐点核对](../../output/vesta-study/20261108-113622/reviews/20261109-035000-s06-vector-slice/vector-slice-crosscheck.json)。

UI range两节点分别Zmin−.008029/Zmax.011477，与实际min−.008028975/max.011477的6小数显示符合。**分析解析误判保留**：早期分析把native节点用换行拼接，却按单空格字符串匹配，所以UIRangeLabelMatchesSelectedSixDecimal=false；原JSON已被native监视，保持其SHA。[后续逐节点修正](../../output/vesta-study/20261108-113622/reviews/20261109-035000-s06-vector-slice/ui-range-label-scope.json)以两个AXStaticText独立匹配/数值舍入确认true，没有GUI重算/原文件覆写。General默认FillPolygons1/BGR/Absolute0/Recursive0，原全3D saturation上下限.0188454/−.00932158在新slice后仍保留；未验精确颜色/光栅/轮廓视觉一致。

本例真实平面z=.3125×14=**4.375Å**，但UI及export“Distance fromorigin”均0（两个数0/0）；它对应本模式从centerprojection的**偏移**，不是把平面放在z0。输出header记录title/(hkl)/V1/V2/distancepair/gridcounts，**没有保存centerprojection**，单凭该header无法完整恢复本例z定位；这是选定导出合同的边界，不自行修补/扩大实现。

[vx2DWindow::OnExport2DData0x1000b726c](../../output/vesta-study/20261108-113622/reviews/20261109-035000-s06-vector-slice/slice-export-handler.stdout)1564bytes/391指令：76b8/76c0两计数各+1，77a8打印`%5i %5i`；77d4..7838闭合双循环，77f8..780c顶点XYfloat→double，7810..7818 scalarfloat→double，行格式`%10.6f %10.6f %14.6E`。另有[hkl]*projection header静态分支，本轮未运行；手册boundingbox无2D输出也未现场。双向量采样构造整个静态核未绑定，本轮数值推导/导出体分开，不冒称全slice库已验。

原件57701正常0/**150.244秒**、soft360前；二维FileClose回main，再Quit无Saveprompt，源parentSHA不变，文件清单未截断/进程组verified，无AX重启/遗留。[方法](../../output/vesta-study/20261108-113622/reviews/20261109-035000-s06-vector-slice/method-observations.md)。新8至400原生Evidence、旧392逐记录保持、11头不变9resolved2investigating。仍59样本，前58记录保持，仅VS59追加二维取证；前46包5457文件/原包257保持。254功能、10完成19未完成，Study未完/Codex自验/dsh独立复审未开始。

R47专项验收初次失败为取证窗口范围错误：runtime-ui-read包含CreateNewSlice、后台2D和main全部窗口，校验把全进程24个文本框误与15字段比较。实际CreateNewSlice的15字段逐值正确；保留verify_slice.py/失败报告/实际exit1，另存verify_slice_scoped.py按depth1AXWindow标题限定当前dialog再验，主验收此前pass无需重跑，原件GUI/数学分析无重算。

## R48完整/半周期投影：近似权重解释与积分差异

VS0059网格，Projectalong[001]、V1[100]/V2[010]、center(.5,.5,.5)。GUI前按VM15.5.3“cumulative sum”但未明写权重/端点的歧义，独立列五种候选：unique-node sum/closed-node sum、各自average、对原三线性场的physical trapezoid integral。full范围−7..7Å对应z0..1、half0..7对应z.5..1；各7×7面内点。**原五候选在两案例全部不符**；没有把目标输出缩放后重写先验。[原先验/全候选](../../output/vesta-study/20261108-113622/reviews/20261109-040100-s06-density-projection/independent-density-projection.json)。

原件58155先生成[full](../../output/vesta-study/20261108-113622/reviews/20261109-040100-s06-density-projection/full-projection.txt)，看到首个值.08627452后提出新假设：投影轴按原格两倍频率采样，包含两端全部等权平均后乘厚度。**在half Calculate/导出前**另登记[新假设/全部49半周期预测](../../output/vesta-study/20261108-113622/reviews/20261109-040100-s06-density-projection/after-full-before-half-hypothesis.json)，断言当时half文件不存在，保留fullSHA；full16subinterval/17samples、half8/9，预测用原网格三线性值独立构造。随后[half](../../output/vesta-study/20261108-113622/reviews/20261109-040100-s06-density-projection/half-projection.txt)49个留出点全部符合，max差4.44444444303e−8；full49诊断点max5.64705882472e−8。**full参与假设产生，不能追称GUI前独立先验；half49才是新假设预先留出验证**。两个7×7的localXYÅ坐标亦符10×12平面。[核对](../../output/vesta-study/20261108-113622/reviews/20261109-040100-s06-density-projection/density-projection-crosscheck.json)。

原网格ρ为piecewise-linear，沿z格点对齐的trapezoid是该离散场厚度积分的独立数学参考。本目标与它max差**full.00365888/half.00320333375**，远超1e−6容差，因此`scientificExactIntegralAccepted=false`明确保持。此轮解释所选目标近似累计算法，不把吻合其离散采样公式标为精确积分正确，也不修改产品。选定输出量纲数值为rawscalarunit×Å，本raw e/Å³输入给近似e/Å²，但Gaussian原子单位互操作结论不变。

[vx2DPanel::CalcSlice0x1000b6350](../../output/vesta-study/20261108-113622/reviews/20261109-040100-s06-density-projection/projection-panel-calculate.stdout)404bytes/101指令：1000b6388测modebits0xc，6390读Slice+28、6394读Prop+60，639c经stub45ae48调用Gen_Periodic_Slice；普通boundingbox另走Create_slice2d。向量slice与projection共享GenPeriodic，projection额外测bit3。[Gen_Periodic_Slice0x323f0](../../output/vesta-study/20261108-113622/reviews/20261109-040100-s06-density-projection/projection-periodic-function.stdout)3068bytes/767指令：329d0..32a08按投影方向加权绝对gridfrequency加.5向下取整，再lsl1加倍；32a1c..a40计算rawsubintervalR并取floor(R+.5)作循环数。**权重32a44..a54是float[(high−low)/(Rraw+1)]，并非直接用rounded count**；初始样本32928、后续i1..count经32bd0 CalcPhysical累加float(32bdc..be4)，最后32c64..c6c统一乘float权重。选定R=16/8时得到17/9等权闭合样本，静态和两case数值相符；未读运行寄存器，不声称任意非整范围R也已验。

最初搜到make_plane2并取2190指令发现它处理普通网格mesh/contours，没有投影累加call；进一步找GenPeriodic并沿mainimport唯一caller定位CalcSlice，相关成功原生取证都保留，不把make_plane2误当投影算子。Create_slice2d469指令对应普通分支，完整渲染/轮廓/SIMD/所有轴/斜胞仍待。

两个实际Export2D文件header仅[hkl]*/V1/V2/“Distancefromorigin”下界对（full−7/−.5，half0/0）与7×7，**未保存center或上界7Å**，不能单凭header恢复projection范围。原件58155正常0/**217.670秒**、soft360前，full创建后再进SliceProperties调整half，OK重算；2Dclose回main再Quit无提示，源parentSHA保持、清单未截断/进程组verified，无AX重启/遗留。[方法](../../output/vesta-study/20261108-113622/reviews/20261109-040100-s06-density-projection/method-observations.md)。

新13至413原生Evidence，旧400逐记录保持、11头不变9resolved2investigating。59样本不增/前58记录保持，只给VS59追加投影取证；前47包5584文件/原包257保持。254功能、10完成19未完成；精确积分差异明确未通过，Study未完/Codex自验/dsh独立复审未开始。

## R49恒定密度投影：非整数R归一化偏差隔离

新VS0060为**独立构造**CUBE，6×6×8、cell10/12/14Å、0原子、288标量全部1；正轴几何按Bohr写入，标量用R33已查明的VESTA raw内部语义，不声明Gaussian密度互操作。GUI前分开两套Oracle：物理常数积分∫1dz=**L**；兼容预测引用R48已查明的采样规则**L×[floor(Rraw+.5)+1]/(Rraw+1)**，Rraw=2Nz8×rangeD。[先验/源身份](../../output/vesta-study/20261108-113622/reviews/20261109-041600-s06-constant-projection/independent-constant-projection.json)。所有两案预测在本GUI前登记，未用本次目标输出拟合。

001轴/V1[100]/V2[010]/center(.5,.5,.5)。noninteger范围0..0.3d=4.2Å，Rraw4.8→loop5、含初始共6samples，兼容预测4.2×6/5.8=**4.344827586206897**，精确积分4.2；aligned范围0..0.3125d=4.375Å，Rraw5、同6samples，预测与精确积分均4.375。两个原件[noninteger文本](../../output/vesta-study/20261108-113622/reviews/20261109-041600-s06-constant-projection/noninteger-constant-projection.txt)/[aligned文本](../../output/vesta-study/20261108-113622/reviews/20261109-041600-s06-constant-projection/aligned-constant-projection.txt)均7×7；第一49标量全**4.344828**、第二49全**4.375**，98兼容标量预期及196面内坐标全部符预设1e−5/2e−5容差。[逐项核对](../../output/vesta-study/20261108-113622/reviews/20261109-041600-s06-constant-projection/constant-projection-crosscheck.json)。

noninteger对独立精确积分差+.144828、相对**+3.448285714%**，超容差；aligned差0。两案均6个相同常数样本，排除了原有Fourier相位、曲线插值或端点值不同的影响，支持R48静态看到的**整数循环数与未取整分母不一致**这一因果解释。此为选定R4.8/5对照，未读运行counter寄存器，非任意范围/方向的一般保证。`scientificNonintegerExactIntegralAccepted=false`、aligned=true、generalProjectionIntegralAccepted=false分别保持；不把兼容规则通过冒成物理积分正确，不实现修复。

原件58585读此0atom文件后ShowModels等结构按钮disabled/0、volume可用；Output实际6×6×8、V1680.000000Å³。两组实际range/向量/center字段native回读，第二从SliceProperties改range、OK再导出。正常0/**158.282秒**、soft360前，2DFileClose回main再Quit无提示，输入/稳定样本SHA保持、parentverified/清单未截断、无AX重启/遗留。[方法](../../output/vesta-study/20261108-113622/reviews/20261109-041600-s06-constant-projection/method-observations.md)。本轮沿用R48静态证据，没有重复函数提取充数；min=max恒定图没有验精确颜色/视觉。

新5至418原生Evidence，旧413逐记录保持、11头不变9resolved2investigating。新VS0060，旧59记录完全保持；前48包5724文件/原包257保持。254功能/60样本、10完成19未完成，精确非整数积分失败仍保留，Study未完/Codex自验/dsh独立复审未开始。

## R50包围盒切片：导出禁用与模式切换

复用VS0059已验证三轴Cube，GUI前登记001、distance=.5d/7Å、xyz各0..1。独立z=.5原始网格平面范围[-.00320971,.0151145]；原件首次bbox及由vector切回后均显示[-.0032097,.015115]，以十进制原值比较误差1e−8/5e−7，均在预登记5e−7内。先验Python六位小数候选[-.003210,.015114]的**字面格式预测不符**，并非全部预测通过；具体显示格式规则仍未绑定。[先验与源身份](../../output/vesta-study/20261108-113622/reviews/20261109-042500-s05-bbox-export/independent-bbox-export.json) / [核对](../../output/vesta-study/20261108-113622/reviews/20261109-042500-s05-bbox-export/bbox-export-crosscheck.json)。

直接CUA观察File菜单：fresh bbox的Export2DData为disabled；切双向量/V1[100]/V2[010]/center(.5,.5,.3125)/offset0后enabled、正常保存7×7/49行vector-control.txt；改回bbox并恢复全部范围后再次disabled。两个bbox候选输出不存在，未强制执行禁用动作。符合VM15.7“不支持bbox导出二维数据”选定路径，并未观察到可用的旧vector导出入口残留。[CUA观察转录](../../output/vesta-study/20261108-113622/reviews/20261109-042500-s05-bbox-export/cua-menu-observations.json)明确是工具交互记录的人工作证；原生AX仅证明菜单项存在和参数/范围，**未暴露enabled属性**，不将它单独当禁用证据。native函数搜索仅找到三个Export handler，menu enable条件未静态绑定。

两种模式共用的六个数值字段会按新模式解释：bbox→vector时继承为[0,1,0]/[1,0,1]，vector→bbox时继承为[1,0,0,0,1,0]。此次每次OK前均重新填写预定字段；这是实际状态转换，不推定系统自动修正。vector控制文件只用于验证可导出状态，数值契约R47已回答，不重算为新覆盖。

原件58948正常0/242.779秒、soft360前；关闭二维窗回主窗Quit无保存提示，源与60稳定样本保持、parent进程组verified/清单未截断、无重启/遗留。第一次setup编码解析失败后误发依赖调用，缺params退出2/noGUI/noEvidence，保留setup-failure；本轮analysis第一次使用不存在的--params-file选项退出2/noEvidence，正确@params新名重试成功，两次原生失败不计成功证据。菜单Escape未关闭时唯一行检查安全终止该动作，随后用公开Cancel动作关闭。编码根因未隔离，非产品计算失败。

7新至425原生Evidence，旧418/11头保持。前49包5839文件和原包257保持；254功能/60样本、10完成19未完成。选定bbox导出禁用及状态切换有证据，一般hkl、边界/视觉/失败输入仍待核，Study未完、Codex自验/dsh独立复审未开始。

## R51二维等高线、绘图开关与重开恢复

复用VS0059/R47平面：two-vectors/001、V1[100]/V2[010]/center(.5,.5,.3125)/offset0。选定二维密度计算此前已答，本轮聚焦显示控制。初始Contours：DrawContour0、Linear、Max1/Min−1/Interval.1/Step5，正负各major宽2/other宽1。填Max.02/Min−.01/Interval.005/Step2并开启后，[原生线性截图](../../output/vesta-study/20261108-113622/reviews/20261109-044000-s06-2d-properties/contours-linear.png)显示相应曲线；切Logarithmic后字段改为A/B/Nmin/Nmax/Step，本轮.003/4/−2/2/2，[原生对数图](../../output/vesta-study/20261108-113622/reviews/20261109-044000-s06-2d-properties/contours-log.png)明显多出曲线。同一308×370内部图域有50424像素改变，只证明渲染响应，未以像素反推所有等高值或验收重写视觉等价。[全部先验](../../output/vesta-study/20261108-113622/reviews/20261109-044000-s06-2d-properties/independent-2d-properties.json) / [读回、图像身份及核对](../../output/vesta-study/20261108-113622/reviews/20261109-044000-s06-2d-properties/2d-properties-crosscheck.json)。

当前本地界面在线性及对数模式均给出正/负值各“每s条/其余”的两套线宽；旧VM15.6的简单L1solid/L2dashed说明不足以覆盖该版本。major/other颜色选框、精确dash/zero样式未验证。main OnContourType0x1000b1dcc(236bytes/59insns)切bit2、隐藏/显示sizer并通过vxTextCtrl复制共享参数，再在DrawContour bit6开启时CalcSlice；OnChkDrawContour0x1000b1b8c(108bytes/27insns)设置bit6并计算/刷新。

library calc_contour_num0x31240(1008bytes/252insns)静态绑定：31314..3132c以binary32 floor((Max−Min)/Interval)分配条数，31404..3146c循环i<count、level=fma(Interval,i,Min)，不分配上端点。本案静态6条，约−.01/−.005/0/.005/.01/.015；GUI前列出的7个含上端点候选不是已验证运行输出。Log31334..31350 count=(Nmax−Nmin)*Step，输入范围跨零时count×2；31488..314ac每个幅度A*B^(Nmin+i/Step)，31470..3155c交替存正负对。本案静态8个幅度.0001875/.000375/.00075/.0015/.003/.006/.012/.024、共16正负候选，Nmin/Nmax是指数界而非先验误读的5个离散N值。**原件运行counter/levelbuffers未读取**，这些为静态推导，不能把GUI读回/曲线变化算数值科学全验；先验5level解释不提升为通过。

General里关闭FillPolygons、开启DrawGridEdges、width1.5，[实际图](../../output/vesta-study/20261108-113622/reviews/20261109-044000-s06-2d-properties/general-grid.png)显示白底网格与等高线，较填色对数图内部113736像素改变。设置Zscale100、Bird1后x/y旋转按钮从disabled变enabled；点击+x一步45°后[鸟瞰图](../../output/vesta-study/20261108-113622/reviews/20261109-044000-s06-2d-properties/general-bird.png)呈垂直裁切/近edge-on，不能称精确美观/无遮挡渲染通过。

FileClose二维窗→主窗Utilities重新开2D：切片label、grid1/width1.5/fill0/bird1、log.003/4/−2/2/2与DrawContour1保留，**Zscale字段100回1**。重开图仍呈裁切线，底层Z几何是否同步重置未绑定。[重开General](../../output/vesta-study/20261108-113622/reviews/20261109-044000-s06-2d-properties/reopened-general.png)为800×726，而前五图800×674，不作逐像素“完全恢复”断言。只证明同进程选定字段恢复，不是新进程默认设置持久化测试。六图均同次只读SCK元数据选择当前原件PID59383正确二维窗：前63802、重开63933；原始PNG与native base64逐字相符，windowID未跨重开复用。general-baseline命名先于读回完成，实际保存时已切Contours页且Draw0，此名称不作General页证据。

原研究GUI59383正常0/267.864秒、soft360前，源/60样本保持、parentverified/清单未截断。**方法偏差**：Quit后立即CUA.getAXState触发空白原件59599，违反既有退出后不用AX约束；该空白实例正常Quit，随后只用pgrep核无遗留。新实例默认偏好上下文未instrument，不声称无全局偏好写入；没有重跑研究或把重启算观察成功。[清理记录](../../output/vesta-study/20261108-113622/reviews/20261109-044000-s06-2d-properties/post-quit-cleanup.json)。

20新至445原生Evidence，旧425/11头保持。前50包5969文件/原包257保持；254功能/60样本全部保持ID、10完成19未完成。Study未完，Codex自验/dsh独立复审未开始。

## R52恒定电子密度转换：单位、五类输出与lned读取失败

复用VS0060独立constantρ1、6×6×8/10×12×14Å/0atom Cube，两份字节相同工作副本。GUI前按VM14.15分别登记：constant梯度/Laplacian=0；TF=(3/10)(3π²)^(2/3)=**2.871234000188191**。若输入1e/Å³，则rhoBohr=a0³、g=TF*a0^5=**.119144569756351**；若输入1e/a0³，g=TF。能量均Eh/a0³，v=−2g、he=−g。使用手册a0=.529177211Å，能量tol=2e−6+2e−5|expected|、Laplacian/logtol1e−6。ln分别登记raw1→0与Å案转Bohr后ln(a0³)=−1.90929573235两个候选，未预先假定UI length解释。[全部先验](../../output/vesta-study/20261108-113622/reviews/20261109-045200-s06-density-conversion/independent-density-conversion.json)。

两个隔离实例中Utilities/ConversionofElectronDensities均勾五类led/ked/ped/ted/lned，分别选择Angstrom/Atomicunit，native scoped回读。五输出各general ASCII grid、cell仍10/12/14Å、**7×7×9=441闭合点**。Å案441个g=.1191446、v=−.2382891、he=−.1191446，Lap=1.595946e−16、ln=0；atomic案g=2.871234、v=−5.742468、he=−2.871234，Lap=−4.250073e−17、ln=0。**4410字段全部符GUI前预期容差**，每文件441值相同。Å案ln(raw1)=0候选符、转Bohr后ln候选不符；atomic案两ln候选相同，不把它算额外区分。[原值/逐文件核对](../../output/vesta-study/20261108-113622/reviews/20261109-045200-s06-density-conversion/density-conversion-crosscheck.json)。这是constant场单位/TF项控制，不覆盖非恒定差分算子与任意单位变换。

library _calc_electronic_energy0xde0c(2020bytes/505insns)：de68..de88载a0 bits0x3fe0ef050a9737de=**.5291772086**，pow(a0/lengthScale,5)；e294..e2a8载TF同独立公式值，e278..e290梯度项系数1/72；df98..dfcc另选Laplacian系数1/12或1/24（本次未切另系数/未绑定GUI选择，非运行通过）。e2b4..e2c0形成v=Lap/4−2g、he=g+v，e2fc/e320/e344将各能量乘上述单位因子。e234..e25c在rawrho<=1e−6时令g/v/he零，e368..e37c为fabs(rawrho)后log；**零、负、阈值和非恒定量尚未运行**，不凭静态分支保证有限/科学适用。df70..df90各维+1输出，scalar格式%14.6E；本轮未重复提取R33等已回答路径。

**写出与自动载入分开**：两案led/ked/ped/ted四新标签实际出现，但lned各弹“Invaliddata/Thisfiletypeisnotsupported”且不成新标签。lned文件实际已完整写出441个有限0，不能把报错解释为未计算或没有输出。二次解释对照在GUI之后、CLI之前另登记[对照先验](../../output/vesta-study/20261108-113622/reviews/20261109-045200-s06-density-conversion/lned-reader-control-prior.json)：取Å案lned同字节副本仅改为.led。原件-nogui -i原.lned -save .ggrid实际**SIGSEGV/−11**，输出存在但**0bytes**；相同字节.led读后保存exit255（既有wx启动退出语义），输出**1916bytes、header7×7×9/441、441个binary32零**。两child无超时，源SHA相同。[原始CLI结果](../../output/vesta-study/20261108-113622/reviews/20261109-045200-s06-density-conversion/lned-reader-control.json)。只改变suffix即改变选定摄入结果，但具体classifier及失败导入后的save崩溃控制流未静态/栈绑定，不宣称已隔离全部根因，更不实现修复。

新增真实unknown **unk_41a422ce94027042**，create rev1/open→edit rev2/medium/investigating，证据含两GUI alert、原CLI对照、全部核对；待核classifier及错误传播/崩溃路径。12头为9resolved/3investigating/0open，旧11头逐项保持。

Å GUI59859正常0/**82.906秒**；atomic59979正常0/**70.055秒**，均soft360前。两次Quit后只用ps，不读AX，不误重启；源/60稳定样本保持、parentverified/未截断、最终无目标进程。CLI实验SIGSEGV不算GUI生命周期失败，也不被outer Python0掩盖。[清理](../../output/vesta-study/20261108-113622/reviews/20261109-045200-s06-density-conversion/gui-cleanup.json)。

12新至457原生Evidence，旧445逐记录保持。前51包6154文件及原包257保持，254功能/60样本保持ID，10完成19未完成。非恒定/零负值/离散导数、输出位置失败/覆盖、完整转换及Study仍未验；Codex自验/dsh独立复审未开始。

## R53lned摄入与失败后CLI保存：分类缺口及NULL路径

不重跑R52已回答的转换和崩溃。读取仅本轮拥有的原CLI PID60060系统crash报告，captureTime2026-11-09 04:57:12.4626+0800，EXC_BAD_ACCESS/SIGSEGV、FAR0；报告内main/lib arm64 UUID分别cd806142-939a-3478-b4e5-30519f8339a0与5d1ec64c-a09f-3f1d-97a2-79f51cd3dce2，独立解析锁定MachO LC_UUID逐一一致。原主/库SHA与基线保持；完整报告只复制本地存证，源不改。[证据接收先验](../../output/vesta-study/20261108-113622/reviews/20261109-050500-s05-lned-ingestion/postmortem-intake-prior.json) / [选定栈、寄存器及UUID](../../output/vesta-study/20261108-113622/reviews/20261109-050500-s05-lned-ingestion/lned-postmortem-observation.json)。

分类函数沿用原生s05已提取check_format0x8d83c(2388bytes/597insns)，不重复函数提取充数。实际只读诊断仅调用原库check_format(dummyfilename)：**.lned/.LNED→−2**；.led/.ked/.ped/.ted/.ned/.xed→**45**；.ggrid→52。[原库实际查询](../../output/vesta-study/20261108-113622/reviews/20261109-050500-s05-lned-ingestion/suffix-classification-observation.json)。静态8d934..8d948仅比较后缀bytes1/2为ed、byte3=0，8deb0回45；即这里接受三字节xed家族而非四字节lned，lowercase路径说明大写LNED仍不支持。这与R52同字节仅改后缀即可有效读回、lned自动导入unsupported alert相互独立支持；不是lned写出缺失或constant0数据不合法。

新ReadFile0x981f0(264bytes/66insns)在9822c调用check_format，98234负type直接9825c回−2、不进入密度reader。新VESTA_cmd_Read0x1e45c(484bytes/121insns)在1e534调用ReadFile，1e538非零失败→1e554返回−1，跳过1e55c起临时objects向当前Scene的追加。实际CLI参数仍含后续-save，crash栈确认已到VESTA_cmd_Save→export_data，并非停在解析lned内容中。

新VESTA_cmd_Save0x1e654(852bytes/213insns)1e8e0..1e8f4直接传当前Scene给export_data，不检查objects数组是否非空。R33完整离线arm64解码已有0xb1b98 `ldr x8,[x1,#0x138]`，接着**0xb1b9c `ldr x1,[x8,w21,sxtw#3]`**；本次报告故障PC确为lib+**0xb1b9c=export_data+284**，寄存器**x8=0/x21=0/FAR=0**，故障在Scene对象数组NULL索引0，尚未进入b1ba8的exportGridFile调用。前导export_data原生dossier b1aec先fopen(w)、b1af4 fclose，之后才dispatch，解释R52**文件已存在但0bytes**。这里用实际postmortem数据+缓存/新静态路径交叉定位，未把静态猜测冒为新崩溃运行。[完整因果核对与旧证据身份](../../output/vesta-study/20261108-113622/reviews/20261109-050500-s05-lned-ingestion/lned-ingestion-crosscheck.json)。未覆盖其它读失败、非空既有Scene、多文件/其它导出格式，也没有修复原件或建立新产品错误策略。

诊断方法限制明确：首次Python CDLL尝试虽预载原PNG，原库仍按Python executable_path解析依赖而失败，**未执行分类函数**，错误和false保留于postmortem observation。另建隔离arm64 C诊断及本地MacOS/Frameworks布局，Frameworks仅放原PNG的同SHA副本；原库按绝对路径加载，仅调用已研究的check_format，不调用reader/writer，不启动原GUI或CLI，不改install-name、不设全局symlink/不改系统路径。clang实际命令/退出码、原库/依赖/二进制SHA与parent watch均保留。postmortem诊断与实际库函数执行证据类型分开。

**unk_41a422ce94027042 revision3/resolved**，原生verify valid=true且unknown与当前头一致。仅R52选定lned分类缺口和失败后save崩溃原因已解释；lned自动读仍不支持、原CLI仍会崩溃，未升级文件互操作/完整错误处理或科学数值验收。12头现10resolved/2investigating/0open，其余11头保持。7新至464 Evidence，旧457逐记录保持；前52包6336文件和原包257保持，60样本记录完全不变、254功能/10完成19未完成。无原GUI/CLI重跑/无目标遗留，Study未完/Codex自验/dsh独立复审未开始。

## R54非恒定正密度：26邻点离散导数与单位

新VS0061独立Cube，6×6×8、10×12×14Å、0atoms、ρ=**1+.2cos(2πx)**，six rawx值1.2/1.1/.9/.8/.9/1.1（读取binary32身份有登记），所有值>0。GUI前分别登记连续解析导数和周期中心差分：L_exact=−.2(2π/10)²cos；L_center=(ρ−−2ρ+ρ+)/h_x²，h_x=10/6；G_center²=[(ρ+−ρ−)/(2h_x)]²。g={TFρ^(5/3)+G²/(72ρ)+L/12}×s，v=L*s/4−2g，he=g+v，rawlnρ；Å能量s=a0^5，Atomic能量s=1而物理导数L/G²×a0²。Atomic另两种几何数字解释列竞争候选，未默认其正确。[独立先验](../../output/vesta-study/20261108-113622/reviews/20261109-051800-s06-nonconstant-conversion/independent-nonconstant-conversion.json)。

在**首次转换之前**新提取make_laplacian_grid0xdb60(684bytes/171insns)并另登记兼容候选：[计算前静态候选](../../output/vesta-study/20261108-113622/reviews/20261109-051800-s06-nonconstant-conversion/static-candidate-before-calculation.json)。本例a≠b触发全部26非零位移δ=(−1,0,1)³\{0}，权重wδ=(6/26)/|Hδ|²，center=−Σw；dcdc..dce4先将距离除lengthScale再取逆平方，Atomic对应a0²系数；dd5c..dd64按6/count归一。对仅x变化，Kx=(6/26)Σ_y,z1/[h_x²+(yh_y)²+(zh_z)²]=**.32400554356492023**，普通中心差分1/h_x²=.36。该候选的L=Kx(ρ−−2ρ+ρ+)；g另分别列L/12、L/24并暂用普通中心梯度，明确只是代码兼容候选，不是独立连续物理Oracle。

两独立GUI各五文件7×7×9/441值，总**4410字段**。[全部原值/模型核对](../../output/vesta-study/20261108-113622/reviews/20261109-051800-s06-nonconstant-conversion/nonconstant-conversion-crosscheck.json)。Å的Lx约[−.06480112,−.03240056,.03240058,.06480109,.03240058,−.03240056]，Atomic约[−.01814616,−.009073081,.009073087,.01814615,.009073087,−.009073081]，比例符a0²。连续解析L最大差Å .0141557452/Atomic .00396401576；普通中心L差Å .00719889717/Atomic .00201589829，均超1e−5。**exact/ordinarycentral两种整体模型在两案均不通过**；这是选定粗网格离散近似合同的区别，不自动推成软件物理缺陷或修复请求。计算前26邻点L候选两个单位全符，max差Å4.462e−9/Atomic2.935e−9，unitLength作用与物理cellÅ解释一致；ln(rawρ)两案全符，max4.215e−8。

计算前“26邻点L+中心梯度+L/12”的能量候选Å在既定容差内，Atomic有字段超出：gmax3.724536e−6(tol3e−6)、vmax7.177353e−6(tol6e−6)、hemax3.452817e−6(tol3e−6)。L/24候选明显更差。**保留初始候选失败，不改容差、不将全部4410算先验通过**。

读完两案输出后沿既有R52_calc_electronic_energy静态e174..e210发现梯度项并非单轴中心梯度平方：对相反邻点ρ(r+δ)−ρ(r−δ)先binary32相减/平方，再加权1/8Σwδ*diff²。本例G²=q*Kx*binary32((ρ+−ρ−)²)/4，qÅ1/Atomic a0²。另登记**事后解释**[gradient假设](../../output/vesta-study/20261108-113622/reviews/20261109-051800-s06-nonconstant-conversion/after-output-gradient-hypothesis.json)，代入L/12后4410字段均符原设各字段容差；**没有独立holdout**，不冒称GUI前科学验证。权重L系数与相反邻点梯度平方、单位、能量组合相互吻合；三维变场/非正交/特殊a=b且γ120分支、零负/1e−6阈值与一般数值精度仍待。

启动首案遇到操作系统因R52 CLI崩溃提供窗口恢复提示，CUA选Don’tReopen后正常加载本轮已登记source，未恢复未知窗口；这是OS启动附带状态，记录于gui-cleanup。两案lned仍弹已知unsupported告警、文件已写出，四类新页出现；不重跑已回答的CLI崩溃。60733正常0/**153.211秒**、60813正常0/**79.435秒**，均soft360前；Quit后仅ps，无AX重启/遗留，输入parentSHA保持、当前parent清单未截断/组verified。[清理](../../output/vesta-study/20261108-113622/reviews/20261109-051800-s06-nonconstant-conversion/gui-cleanup.json)。

8新至472 Evidence，旧464/12头保持10resolved2investigating。新VS0061、旧60样本记录完全不变；前53包6513文件及原包257保持。254功能/61样本、10完成19未完成，完整转换与Study未完，Codex自验/dsh独立复审未开始。

## R55三维非正交与零/负/阈值：新输入预测检验

本轮三案的全部模型在**首个GUI启动前**固定，无本轮输出拟合；复用R54已查明的26邻点兼容规则，同时为正密度案分列独立连续物理Oracle。[完整先验/生成器身份](../../output/vesta-study/20261108-113622/reviews/20261109-053500-s06-3d-threshold-conversion/independent-3d-threshold-holdout.json)。这是用原件既有证据学得规则对新输入的holdout，不称独立推导出原件公式或独立复审。

新VS0062：6×6×8、10/12/14Å、β100°、0atom，ρ=**1+.1cosX+.15sinY+.2cosZ+.05cos(X+Z)**，X/Y/Z=2π分数坐标，raw范围.6200962..1.4799038。混合XZ项和非正交metric区分轴/周期/夹角作用。独立连续梯度/拉普拉斯用M=H^−1H^−T，Mxx=1/(a²sin²β)、Mzz=1/(c²sin²β)、Mxz=−cosβ/(acsin²β)，其余相应；兼容模型预先用所有26 displacement的Cartesian距离权重6/26、R54对向差平方G²，分别Angstrom/Atomicunit。新VS0063为正交同dims，sixx raw=[−1,0,1e−7,1e−6,1.000001e−6,1]，binary32后1e−6实际**9.999999974752427e−7**、上邻**1.0000010206567822e−6**，预先登记rawrho>1e−6才算能量，其他g/v/he零、L照算、logabsρ/zero→−Infinity。该signed/zero样本只验兼容行为，不声明处于正电子密度物理域。

每案5文件各7×7×9/441闭合点（288核心+153周期副本），共**6615字段**。[逐点实际/预期/判定](../../output/vesta-study/20261108-113622/reviews/20261109-053500-s06-3d-threshold-conversion/3d-threshold-crosscheck.json)。正密度非正交两案**4410字段全部符合原始绝对容差**，覆盖三维变化、XZ耦合、闭合面/边/角及两单位的选定合同；g/v/he与26邻点规则有新输入预测证据。独立连续metric物理模型仍不符该粗网格输出，`continuousPhysicalExactDerivativeAccepted=false`保持，不能用兼容模型通过代替物理精确导数或一般收敛精度验收。

阈值案315个closedsites（三维核心192个）g/v/he三项均精确0，126closedsites进入能量分支；rawρ=−1的log=0，零点**63个−Infinity**，其他近阈值log为有限负数。`ln(absρ)`及阈值比较均得到选定运行证据；已分开预期的非有限值，不把它漏检成全finitepass。超过阈值极小正ρ时g约**46.68471**、v约−93.36607、he约−46.68135，而下邻三能量恰0，此选定不连续行为保留，不实施修复。

**两套精度准则各自保留**：原始固定tol为L2e−5/g4e−6/v8e−6/he4e−6/log1e−6。阈值计算之前，从先验幅度可知能量到46..93且原格式%14.6E，另登记[打印精度补充](../../output/vesta-study/20261108-113622/reviews/20261109-053500-s06-3d-threshold-conversion/before-threshold-print-precision.json)：仅threshold案采用补充max(原tol,1.5×10^(floor(log10|expected|)−6))，原始判定不改；此登记早于threshold-options native回读/首个阈值输出。结果原固定tol共有**189失败**：g63(max5.64048e−6)、he63(max4.41474e−6)、log63(max4.36073e−6)。g/he幅度及手册a0=.529177211与原码.5291772086末位差会作用于打印末位，log小密度的七位输出也超过固定1e−6；未重拟合数值或抹掉失败。阈值全部2205符合前述补充准则，含63个有正确负号的Infinity；原绝对容差整体false、补充适用准则整体true分列，不宣称6615原始tol全通过。

三原GUI61189/61257/61335分别正常0/**72.712/78.012/134.372秒**，soft360前；工作副本与稳定源保持，全部Quit后只ps、无AX重启/目标遗留。每案五文件写出/四新标签，lned仍因已解释的后缀分类缺口unsupported，未重跑CLI崩溃。[生命周期](../../output/vesta-study/20261108-113622/reviews/20261109-053500-s06-3d-threshold-conversion/gui-cleanup.json)。原静态171/505insn沿用，没有重复提取凑证据；并非所有单斜/三斜、a=bγ120特殊stencil、任意dims/大网格/NaN输入或导出失败全验。

10新至482 Evidence，旧472/12头保持10resolved2investigating。新VS0062/63、旧61记录全保持；前54包6661文件及原包257保持。254功能/63样本、10完成19未完成，完整转换与Study未完，Codex自验/dsh独立复审未开始。

## R56 标准化与 Niggli：报告结果和当前结构分开

新增独立 P1 样本 VS0064（7/3/5 Å 全直角，V105）和 VS0065（4/5/5 Å、γ=atan2(3,4)=36.869897645844°，V60）。C/O/N 原始分数坐标分别(−.89,.23,.37)、(1.41,−.47,.61)、(.71,.19,−.17)，占位1/.5/.75、Uiso .01/.02/.03。首 GUI 前固定两晶胞度量、三对最短周期距离、8种带符号基变换候选和容差；不从目标输出拟合约化晶胞。[先验](../../output/vesta-study/20261108-113622/reviews/20261109-055000-s06-tidy-niggli/independent-tidy-niggli.json)与[实际对照](../../output/vesta-study/20261108-113622/reviews/20261109-055000-s06-tidy-niggli/tidy-niggli-crosscheck-scoped.json)。生成器标题曾写成字面量+name+，在首 GUI 前修正并保留原稿/身份，不改数学输入。

原附带 structure_tidy 是 x86_64，SHA e32bc47a8c24f4b987a3295fc4968c7f45dc9512dadaebef9cfa541f6a7a7b4a，依赖系统 libgcc_s.1/libSystem；本轮在原环境实际产出结果，与已查明的 MADEL 缺失 gfortran 情形分开。STRUCTURE_TIDY.sh SHA 558e12e79a35422629e409b71a94a1ad8a410d6b5920b65006fc7107920fed39，真实协议是在样本目录执行 **structure_tidy $f.stin > $f-tmp.sto**，输入是文件名参数，输出重定向；不是把输入文件重定向到 stdin。主程序随后将临时输出改为 data.sto。未修补任何原件、全局依赖或脚本；没有独立捕获外部子程序退出码，只有文件、数值、界面及原 GUI 父进程退出证据。

三案临时 data.stin 均把坐标归一到[0,1)：C(.11,.23,.37)、O(.41,.53,.61)、N(.71,.19,.83)，标准化第一行为 P 1，Niggli 为 **\*P 1**，随后0、晶胞、标签/坐标、end/end；临时输入没有占位和U字段。临时文件按拥有的 PID 只读快照，拷贝与原临时文件 SHA 相符；原始工作 CIF 保持。

**标准化更新当前结构**：最终 SaveAs 晶胞3/5/7全90°，C(.30,.24,.30)、O(0,0,0)、N(.34,.78,.70)，与 TIDY 最终 ATOM 行一致；体积105与三对最短周期距离保持。报告 b,c,a，setting −x,−y,−z、origin(.47,.39,.59)。独立不加占位权的 Σsqrt(x²+y²+z²)=1.5892589859453032，对应输出γ1.5893；仅验证该选中解的参数，未证明所有原点/设置的全局最优。SaveAs 保留占位1/.5/.75及 THERI 1 原始 .01/.02/.03（本案U家族）；场景显示原子3→10是原点O的8个闭合胞角副本加C/N，不能解释为不对称位点变成10。

**Niggli 只报告约化解，不写回当前结构**：正交案报告3/5/7全90°、b,c,a，坐标C(.23,.37,.11)、O(.53,.61,.41)、N(.19,.83,.71)；斜晶胞案报告3/4/5全90°、−a+b,a,−c，坐标C(.23,.34,.63)、O(.53,.94,.39)、N(.19,.90,.17)。两案均为 reduced form32、metrically Orthorhombic oP，体积和三对最短周期距离符合先验，分别命中候选符号(+,+,+)/(+,+,−)。但 SaveAs 分别仍为7/3/5和4/5/5、γ36.869896°，原始超胞外坐标原样保留；占位/U不变。手册14.4说约化解写入输出窗，本轮行为与该范围一致。最初“约化将变换当前结构”的假定被拒绝，不能将报告数值正确冒充已应用。正交案后、斜案启动后但操作结果前另登记“报告约化、当前结构不变”的状态预期；该登记22:10:26早于斜案 data.sto 22:10:46，斜案支持此预期，不称它在 GUI 启动前登记。

静态主程序 Standardize_by_ST 0x100099eac、3920bytes/980insn：0x10009a378 export_data(type20,mode0/1)，0x10009a598 wxExecute，确认输出存在后0x10009a5ec比较事件mode 0xa1，0x10009a5f0另一分支跳到报告路径；只有选中路径0x10009a634调用 readdata(type33)，后续 SceneBuild 0x10009adc0。此静态条件导入与标准化应用/Niggli报告的运行差异相互支持，未做运行寄存器追踪。保留调用/脚本/临时输入输出证据，不移植反编译代码。

首分析 child1 被保留：对未归一的原始坐标仅搜±2图像，错判斜案C/O距离；正确先取模再搜±2，恢复2.1931712 Å。另把斜案状态预期误标为启动前，现按实际时序更正。原报告/代码/失败 Evidence 不覆盖，新 scoped 报告只重算已有数据，没有重跑目标。[修正说明](../../output/vesta-study/20261108-113622/reviews/20261109-055000-s06-tidy-niggli/analysis-scope-correction.json)。原 GUI61789/61887/61944均正常0，266.113/129.089/230.839秒，soft360前；Quit后只ps无AX重启/目标遗留。

13新至495 Evidence，旧482和12头不变（10resolved/2investigating）；旧63样本记录保持、新64/65。前55包6830文件、原发布包257文件保持。254功能/65样本、10完成19未完成，完整标准化/约化、全空间群及失败分支和Study未验收，Codex自验、dsh独立复审未开始。

## R57 正负峰、Voronoi 菜单共用与最大峰数的计算作用

独立新 VS0066 为4×4×4、4Å立方晶胞、常量ρ2、Cube含C/O两原子；VS0067为同网格无原子 signed 场 **ρ=cos(2π(x−.25))+2cos(2π(y−.5))+3cos(2π(z−.75))**。首 GUI 前登记原始binary32标量、正/负/总积分、唯一正最大/负最小的格点和容差。[先验](../../output/vesta-study/20261108-113622/reviews/20261109-063000-s06-voronoi-peaks/independent-voronoi-peaks.json) / [原件实际核对](../../output/vesta-study/20261108-113622/reviews/20261109-063000-s06-voronoi-peaks/voronoi-peaks-crosscheck.json)。每体素体积1Å³；常量总积分128，signed场正/负部分68/−68、总和0，峰值±6在(.25,.5,.75)/(.75,0,.25)，均由独立输入计算。

选择含原子 Cube 的 Voronoi Integration 菜单后实际出现 Max.NumberOfPeaks 对话框，而无原子 Cube 中该菜单disabled、Peak Search仍enabled。[CUA观测转录](../../output/vesta-study/20261108-113622/reviews/20261109-063000-s06-voronoi-peaks/cua-observations.json)明确是Codex对工具输出的转录，不伪造原生AX enabled属性。初设的按C/O原子中心划分32/32Å³、64/64电子的普通周期Voronoi预期**不适用于此处理路径**，没有拿未适用预期冒充通过。Cube分子结构部分输出1Å晶胞、原子坐标(.5,2,2)/(2.5,2,2)，体数据部分仍4Å晶胞/64Å³；这是选定结构/体数据双几何观察，未由此判原件科学bug。常量计算实际只追加128/0的整胞积分，没有峰数或峰表；不把“没有打印峰表”改写成工具明确报告0峰。

菜单共用已静态绑定，并非仅凭同对话框推测：构造函数在0x1000a44f8/0x1000a4558分别赋 Peak Search ID **0xb2/178** 和 Voronoi ID **0xb3/179**；全局初始化0x1000aa8dc在0x1000ab63c和0x1000ab670均将同一x28=**OnPkSearch 0x10009dedc**存入回调。只读解析原始arm64 Mach-O得到常量0x1004e69a0/9a8分别(b2,−1)/(b3,−1)，初始化0x1000ab64c/680读取；处理函数没有读取传入事件参数。完整原件SHA与文件切片/常量字节留在[事件绑定](../../output/vesta-study/20261108-113622/reviews/20261109-063000-s06-voronoi-peaks/menu-event-binding.json)。该主程序大于30MB，没有放进本轮analysis父进程watch冒充完整hash；完整SHA在结果中、原发布包257文件由专项校验另验。未启动静态分析目标进程，也未修改原件。

signed场 Max0、negativeOff：报告检测1峰，正峰坐标/值符合先验，Voronoi边界0/64，integral0、volume64。Max0、negativeOn：报告检测2峰，负峰先列，再正峰，坐标/值符合先验；两峰 integral−60/+60、volume32/32、边界**63/64**。每次整胞正负积分仍68/−68。重开参数每次默认max0、negativeOff；没有把此前checkbox状态当成持久设置。

在负峰all输出后、max1计算前另登记“总检测数2、仅列1行、整胞积分不变”的控制预期；原始两案数值先验不改。[max1预期](../../output/vesta-study/20261108-113622/reviews/20261109-063000-s06-voronoi-peaks/before-max-one-hypothesis.json)。实际max1/negativeOn确实仍报2个检测峰，只列首个负峰；但它的integral从−60变为**0**、volume32变为**64**，边界0/64。故最大峰数会改变参与Voronoi分配的峰集合，不能当作仅限制打印行数。库 Search_Peak_from_3Ddata 0x34924/1676bytes/419insn 在0x34e1c先打印全检测数，0x34e20..30按正max取min，0x34e4c把裁剪数传给 VoronoiIntegration 0x33690，0x34e58调用后再打印列表；与该运行变化相符。主处理函数1540bytes/385insn先调用Integrate_Data 0x34804，再显示对话框/调用搜索。

事后独立欧氏最近峰/周期距离、等距节点半分的数学对照给两峰积分+60/−60和体积32/32，符合全峰输出；此分配模型本轮没有预注册，明确是**post-output comparison**，不是独立预测检验。其几何严格等距节点只有**20/64**，与原件所谓Voronoi boundary63/64不等，不能把输出计数解释成几何等距点数。库VoronoiIntegration4468bytes/1117insn已留，确切边界计数语义、一般边界/非正交/大网格仍待核，未据一例认定全算法正确或修复。

原 GUI62318/62390正常0、73.988/177.836秒，均soft360前；源/工作副本不变、Quit后只ps无AX重启、无目标遗留。静态搜索一次Pk宽模式匹配大量PK类型名，保留原始搜索结果，随后用VestaFrame目录和精确源地址绑定；没有用截断宽搜索推断函数不存在。30新至525 Evidence，旧495/12头保持10resolved2investigating。前56包7018文件及原发布包257保持；新VS66/67、旧65记录保持。254功能/67样本、10完成19未完成，完整Peak/Voronoi/Study未验收，Codex自验/dsh独立复审未开始。

## R58 Voronoi 边界计数与细分积分：新网格预测

R57的“boundary63/64”现有选定路径解释：库 VoronoiIntegration 0x33690 对每个周期格点先按到峰的Cartesian距离选单个最近标签；严格减小才改标签，正好相等保留较早峰。0x34350设邻点数26，0x34358..a4检查中心以外的3³−1个周期邻格点标签，0x34390/94把标签差异OR到flag，0x343a8在无差异时直接整体分配；否则进入细分并在0x3468c计数加一，0x347b0打印。**boundary数是邻域标签不一致的格点数，不是几何严格等距点数。** 原件常量装载0x340e0..3419c的312bytes表由只读Mach-O切片重建，确为26个(−1,0,1)三元组、排除(0,0,0)，字节、地址、库SHA及缓存函数SHA均在[静态规则](../../output/vesta-study/20261108-113622/reviews/20261109-065000-s06-voronoi-boundaries/static-boundary-rule-before-gui.json)。没有重复提取同一函数增加证据数。

该调用细分数2，每个有flag格点按x/y/z各±**1/4格距**的8个子位置分配，各带该原格点体积1/8；源标量保持中心格点值，不对子位置插值ρ。候选峰来自中心+26邻格点出现的标签；距离差≤1e−6时均分子份额。0x344ec/4f0、0x34544/548、0x3457c/580给出半步/四分之一步坐标，0x34660比较1e−6（bits3eb0c6f7a0b5ed8d），0x34724..754将原始float标量与份额体积累加。未标记节点按单标签积入完整体积。复算R57旧数据得到boundary63、积分−60/+60、体积32/32；此旧数据复算仍是事后解释，不能改称预测。

新增稳定VS0068，6×8×10/480核心点、5/7/11Å全直角、V385、无原子。定义X=2π(x−1/6)、Y=2π(y−2/8)、Z=2π(z−3/10)，**ρ=.3+cosX+2cosY+3cosZ+.4cos(X+Y+Z)**。将前述兼容规则用于新输入，全部原始binary32、每节点标签/flag/份额、峰坐标/值、整胞及每峰积分/体积和容差在本轮首GUI前登记：[独立输入与兼容预期](../../output/vesta-study/20261108-113622/reviews/20261109-065000-s06-voronoi-boundaries/independent-boundary-holdout.json)。几何严格等距节点为**0**，26邻点flag预测**342/480**，可以清楚区分两种计数含义。此为早期原件学得规则对新输入的holdout，不宣称公式从独立物理原理推出；本轮没有输出拟合。

原件Max0/negativeOn实际报告两峰：(.16667,.25,.3)、值6.7、积分467.483、体积192.5；(.66667,.75,.8)、值−6.1、积分−351.983、体积192.5。全部符合计算前坐标1e−5/峰1e−4/积分与体积.002容差；先验未舍入积分分别**467.4832005689971/−351.9832000894239**。峰表原件排序正峰先，本轮按坐标匹配，不假定原始数组顺序或由两案声称完整排序规则。整胞正/负部分实际479.799172/−364.299172，分别符合先验479.79917237711686/−364.29917189754394，合计115.50000047957292。实际boundary正是**342/480**。[逐字段对照](../../output/vesta-study/20261108-113622/reviews/20261109-065000-s06-voronoi-boundaries/boundary-holdout-crosscheck.json)。整体积分不等于局部ρ插值的精确连续Voronoi积分；独立物理精度、收敛、非正交/加权/退化峰及任意采样尚未验收。

原GUI62754正常0/58.122秒，soft360前，稳定源和工作副本不变，Quit后只ps无AX重启/无目标遗留。4新至529 Evidence（GUI、两UI、analysis），旧525/12头保持10resolved2investigating。前57包7232文件及原发布包257保持；新VS68、旧67记录完全保持。254功能/68样本、10完成19未完成；一般Peak/Voronoi与Study继续，Codex自验、dsh独立复审未开始。

## R59 逐多面体积分菜单：锁定 arm64 的空处理路径

“Integrate Voxel Data in Each Polyhedron”入口在锁定原件的arm64切片中**没有执行数值积分**。构造函数0x1000a4594从宽字符串0x1004e71e0构造标签，0x1000a45b4赋菜单ID **0xb9/185**；只读原Mach-O的UTF32LE标签及原字节、原件SHA登记在[GUI前证据](../../output/vesta-study/20261108-113622/reviews/20261109-070500-s05-polyhedron-integration-stub/poly-stub-before-gui.json)。全局初始化0x1000ab7a0读常量0x1004e69d0，其两个int32为**182/186**，故185落在事件范围中；0x1000ab788/790将 **VestaFrame::OnMulti 0x10009fedc** 存为回调。R57缓存的原生函数研究给该处理体**4bytes/1insn，仅ret**。函数无条件返回，不能由菜单名推定有多面体积分实现，也不为重写阶段自行补出新的科学语义。

本轮复用VS0066常量2/4³含C/O的Cube，未新增或修改稳定样本。启动前登记该路径预期为无计算/无对话框/无输出追加。GUI实际显示2atoms、0polyhedra及体数据；Utilities菜单该项enabled，点击后仍为同一主窗口，无新增对话框，Output全文SHA前后完全相同：[前后对照](../../output/vesta-study/20261108-113622/reviews/20261109-070500-s05-polyhedron-integration-stub/poly-stub-crosscheck.json)。本运行没有多面体，所以只作该场景的GUI行为观察；对无条件空处理的判断来自事件绑定及函数体，不用0polyhedra场景冒称已执行科学算法。

原生AX完整树并非字节相等：353→360nodes，三项AXTabGroup的opaque观测指针变化，窗口管理菜单缓存增添分屏/移往显示器条目及一处AXGroup差异；[完整差异](../../output/vesta-study/20261108-113622/reviews/20261109-070500-s05-polyhedron-integration-stub/ax-difference-scope.json)保留，不把环境AX缓存当文档修改，也不宣称全树相同。CUA enabled观察单独转录，原生AX没有该enabled属性。输入/工作副本SHA保持。原件主程序>30MB未放进analysis父进程watch冒称完整hash，完整原件SHA在先验且专项校验全部257文件；本轮没有再次提取相同函数凑证据。

原GUI66161正常0/48.459秒，soft360前，Quit后只ps/无AX重启/无目标遗留。结论只覆盖锁定本地版本arm64菜单分发与选定GUI观察，不声称x86_64/其它版本也是空实现，更不记数值积分通过。4新至533 Evidence，旧529/12头保持10resolved2investigating；前58包7350文件及原发布包257保持，全部68样本记录不变。254功能、10完成19未完成，Study继续，Codex自验/dsh独立复审未开始。

## R62 晶胞矩阵、超胞两种存储与原点平移

新独立VS0070：P1、4/5/6Å全直角、V120；C1(.125,.2,.3)、occ1/Uiso.01，O1(.625,.4,.5)、occ.5/Uiso.02。三案输入/原始SHA、H′=HP、f′=P⁻¹(f−p+n)、Gram PᵀGP/体积行列式、整数平移陪集及容差在首GUI前固定。[独立先验](../../output/vesta-study/20261108-113622/reviews/20261109-073141-s06-unit-cell-transform/independent-transform-before-gui.json)。手册6.2.6明确P各列为新基矢、p为旧分数坐标下新原点；此处原生内部分析只绑定执行路径，不复制反编译实现，也不从结果拟合数值Oracle。

三原件均从已加载CIF的Edit/EditData/UnitCell进入，保持 **Update structure parameters to keep 3D geometry**、NormalizeOn；并不声称完整File/NewStructure创建流程也经过同样运行。前两案P=diag(2,1,1)、p0，均出现体积变化警告并Yes。第一选择Add new equivalent positions，第二选择Search atoms/add new sites；实际选项另有原生树。

**增加对称操作**：保存晶胞8/5/6全90、V240，存储C(.0625,.2,.3)/O(.3125,.4,.5)两位点，SYMOP为identity和(.5,0,0)平移，均单位旋转，GROUP仍1/setting1/P1，TRANM1，矩阵行保存P2a。位点标为2a，界面显示4atoms。**增加独立位点**：相同晶胞V240，SYMOP仅identity，TRANM0；存储C(.0625,.2,.3)、C(.5625,.2,.3)、O(.3125,.4,.5)、O(.8125,.4,.5)。原件标签保留**C1,C1,O1,O1**，没有擅自当作唯一命名或预设自动改名。两案实际对称展开后的4个位置、占位/U与同一独立超胞预期相符，不能只凭相同屏幕计数认为底层表示相同。[逐项对照与原始矩阵/操作](../../output/vesta-study/20261108-113622/reviews/20261109-073141-s06-unit-cell-transform/unit-cell-transform-crosscheck.json)。

**剪切+平移**：P的列为a,a+b,c，即行矩阵[[1,1,0],[0,1,0],[0,0,1]]，p=(.1,.2,.3)。保存a4、b**6.403124**（独立√41=6.4031242374328485）、c6，γ**51.340191°**（独立atan2(5,4)=51.34019174590991°），V约120。坐标C(.025,0,0)、O(.325,.2,.2)，SYMOPidentity、两源位点。界面5atoms来自C在y/z闭合面上的4副本+O，不是源位点变5。存储TRANM行的原点和平移矩阵与输入完全一致。

三案**111个先验数值字段**（晶胞/体积、存储及实际对称展开后坐标/占位/Uiso）均符合预定tol：Å1e−5、角度1e−4、体积.002、分数模1/occ/Uiso各1e−5；存储位点数2/4/2、操作数2/1/1及P/p另有精确合同检查。THERI1原始U .01/.02分别保持。由保存后的展开组成和中性核Z推导F000为20/20/10，本轮**没有原版F000计算**，不能写作其运行验证。一般各向异性张量变换/不确定度尚未由此验收。

内层Transform确认后主窗口回前、Output仍旧4/5/6/V120，但进程原生树显示外层EditData仍存在；Cmdgrave/窗口菜单未把它聚焦。重新从Edit/UnitCell进入，原有编辑器仍保留待提交a8（或剪切6.40312/γ51.3402），再外层OK才真正更新主场景。这是本轮实际modeless事务/焦点边界，不由内层OK或菜单尝试冒称整个操作完成。第一案[待外层提交树](../../output/vesta-study/20261108-113622/reviews/20261109-073141-s06-unit-cell-transform/new-symmetry-before-parent-apply-ui.stdout)保留；后两案也显式复核外层标题/值后OK。三份SaveAs的Where与文件名均回读为本轮路径，CIF不覆盖。

主程序DlgTransform::OnOK0x100081f70/1492bytes、CheckDet0x100083de8/508bytes与库Crystal::TransformCell0x69038/2660bytes已读；OnOK0x100082368调用核心，核心0x69200转换SymOP、0x6923c转换位点、0x69260调用各向异性位移参数转换，0x696b8复制XSite。只据这些调用绑定已测分支，不把未测张量/所有det分支当通过。KeepParametersUnchanged、NormalizeOff、初始化不可取消行为、第三个DoNothing、减胞/去重、奇异/负行列式、其它SG/ADP/误差传播仍待核。

原GUI69661/69787/69869正常0，259.468/182.020/105.459秒，均soft360前；Quit后只ps，无AX重启或目标遗留。18新至583 Evidence，旧565和12头10resolved/2investigating保持（DB4/source8）；前61包7809文件及原发布包257保持，新VS70/旧69记录不变。254功能/70样本、10完成19未完成，完整变换及Study继续，Codex自验/dsh独立复审未开始。

## R63 关闭归一化、保持参数与行列式拒绝分支

沿用VS0070，70样本记录全不变；同一剪切P的列a,a+b,c、原点p=(.4,.3,.6)。输入SHA、手册6.2.2/6.2.6身份、H′=HP、f′=P⁻¹(f−p)及两种模式预期在首GUI前固定，[独立先验](../../output/vesta-study/20261108-113622/reviews/20261109-080000-s06-transform-modes/independent-modes-before-gui.json)。没有从实际结果回填预期。

**Update keep3D + NormalizeOff**：保存cell4/6.403124/6、γ51.340191、V约120；C1(**−.175,−.1,−.3**)、O1(**.125,.1,−.1**)按原始分数坐标比较，**没有模1掩盖未归一化错误**。**Keep structure parameters unchanged + NormalizeOn**：实际选择模式的原生树已存，同一P/p保存于TRANM0行，晶胞仍4/5/6全90、V120，C1(.125,.2,.3)、O1(.625,.4,.5)保持；occ1/.5及Uiso.01/.02两案都不变。总计34先验数值字段全部符合原定tol（Å1e−5/角1e−4/V.002/原始分数及occ/U各1e−5），保存位点各2、identitySYMOP各1，矩阵/原点另作精确核对。[保存文件与逐字段比较](../../output/vesta-study/20261108-113622/reviews/20261109-080000-s06-transform-modes/transform-modes-crosscheck.json)。P1的恒等操作在任意基底共轭后仍恒等，本轮不据此声称非平凡SG的仅变换对称操作分支已验收；没有原版F000计算。

在NormalizeOff案有效矩阵之前，先将P00=0（其余单位阵），OK实际弹出 **“Warning ! The determination of the matrix you have given is 0. Change the rotational matrix part.”**；确定后仍是Transform编辑器。随后P00=−1，OK实际警告从right-handed变left-handed，并询问是否继续；选择**No**返回编辑器，再输入登记的有效P/p。后续保存的晶胞与坐标符合未被前两次尝试修改的源结构先验。零行列式提示保留原件拼写determination，未擅改determinant；这里只覆盖拒绝/取消，没有接受负行列式变换或边界阈值。底层CheckDet0x100083de8沿用R62已读证据，|det|与1e−6/finite判断、负号警告和返回支路可见；本轮不重复计静态新证据。

内层OK后再次从Edit/UnitCell聚焦既有modeless外层，确认新参数/保持模式后外层OK。两案SaveAs回读Where为本轮目录及新文件名，源CIF不覆盖。第一次奇异警告关闭后，按值匹配唯一0文本框的防护因有7个候选而抛错，发生在setValue前；改为已观察12个矩阵字段的有序定位，**没有把错误字符串x输入原件**，CUA记录保留。最终Quit后只ps，不调用AX；70139/70275正常0、146.559/113.463秒，均soft360前，无目标遗留。

前62包7997文件及原发布包257保持；10新Evidence至593，12头10resolved/2investigating保持（DB4/source8），254功能/70样本、10完成19未完成不变。一般SG、各向异性张量/误差传播、初始化/取消不可逆范围、减胞/去重、接受负行列式仍待核，完整Study继续；Codex自验，dsh独立复审未开始。

## R64 各向异性位移张量随基底变换

复用独立VS0057（P1、10/12/14Å、β100°、C/O两位点Uani及不同含正负非对角分量U），70记录不变。首GUI前登记原CIF/手册身份、P/p、H′=HP、f′=P⁻¹(f−p)和两案完整张量先验。独立公式为β=2π²D*UD*、Q=P⁻¹、β′=QβQᵀ、G′=PᵀGP，再以新倒易长度D*′转换回U′；Ueq=ΣGij ai*aj*Uij/3。小型线性代数计算不引入原件代码，见[先验](../../output/vesta-study/20261108-113622/reviews/20261109-081000-s06-transform-anisotropic/independent-tensor-before-gui.json)和[独立数学](../../output/vesta-study/20261108-113622/reviews/20261109-081000-s06-transform-anisotropic/independent_tensor_math.py)。均为Updatekeep3D/NormalizeOn、det+1，无体积变化或超胞选项。

**右手换轴**P的列b,a,−c：cell12/10/14、α80°、βγ90°，C(.5,.1,.5)/O(.5,.3,.5)。保存U六分量顺序11,22,33,12,13,23，C=(.02,.01,.03,.005,−.002,.003)、O=(.05,.04,.06,−.01,−.005,−.004)，交换与反向轴的符号均符合先验。**非正交剪切**P的列a,a+b,c、p=(.05,.15,.25)：保存b15.620500、α96.382584、β100、γ50.194429，C(.7,.35,.25)/O(.9,.35,.25)。C的先验U11=.009120595399、U12=−.008822750363、U13=−.003587819137，O的先验U11=.053832354799、U12=−.039449659413、U13=−.000079902940；其余U22/U33/U23对应.02/.03/.002与.05/.06/.005。两案V约1654.477025不变，occ均1，identitySYMOP各1/存储各2，TRANM0行P/p精确核对。

**精度边界完整保留**：原首GUI前U分量tol=1e−6。第一案保存文件显示THERT0/THERM六分量为固定5小数、THERI1/Ueq为6小数；在第二案GUI启动前另登记[半个末位政策](../../output/vesta-study/20261108-113622/reviews/20261109-081000-s06-transform-anisotropic/printed-tensor-policy-before-shear.json)，只对U分量以5.000001e−6作保存格式比较，不改原JSON或原标准。58数值字段中原标准有**3项不通过**，均在剪切：C U12(−.00882 vs −.008822750363)、C U13(−.00359 vs −.003587819137)、O U11(.05383 vs .053832354799)。全部原比较行/差异保留，**不能写成58原定容差全通过**。按第二案运行前的固定打印精度政策，58项全部通过；该政策从第一案格式学得，不是整个实验前的独立格式先验。其余tol为Å1e−5、角1e−4、V.01、frac/occ1e−5、Ueq1e−6。[逐字段双标准](../../output/vesta-study/20261108-113622/reviews/20261109-081000-s06-transform-anisotropic/tensor-transform-crosscheck.json)。

从原张量独立得到Ueq .020772643606/.050558914515，两案均保存**.020773/.050559**，符合换基不变量和原1e−6tol。只是保存字段验证，不把舍入后THERM再次重构的物理协方差当更高精度结果；也没有重开或StructureFactors再次计算、nativeF000运行或一般误差传播。

libVESTA的Transform_anisotropic_B0x8c758/692bytes、U_Beta_trans0x8c5c8/400bytes新读；get_U_Beta_trans_coefficients0x8c474/340bytes与历史原生ID一致，归复用而非新增证据。前者0x8c8b8/0x8c908调用U_Beta_trans，0x8c8d8/0x8c8f8矩阵乘法用于绑定执行；不以静态全函数扫描声称各模式/所有输入通过。

内层OK后重聚焦既有外层EditData、回读新cell后外层OK；两SaveAs的Where/新名确认。GUI70513/70612正常0、97.809/81.206秒，均soft360前，Quit后只ps无AX/无遗留。原发布包257和前63包8143文件保持。10次原生结果中1复用旧ID、**9新Evidence至602**，旧593及12头10resolved/2investigating保持（DB4/source8），254功能/70样本、10完成19未完成。一般SG/张量类别/误差传播、减胞/去重、初始化取消/负det接受仍待核；完整Study继续，Codex自验，dsh独立复审未开始。

## R65 CLI旋转顺序、缩放和去重阈值

对照总计划后，CLI全命令契约和166个仅见入口条目仍有实质缺口；本轮从局部晶胞测试转到S07系统命令研究，没有用新覆盖矩阵替代规格。VM17列16个命令（含-i/-o/-calcf/-list/-merge_split_site、三轴旋转、GUI交接/导出、两种缩放）；原库parse_cui_command0x1fb34/928bytes另见clear/add/del/set/import等名字，只作静态发现，未执行/不宣称全命令通过。新增[CLI权威记录](CLI.md)统一入口（相对链接见各域文档），原CLI_COMPLETION仅保留返回链职责。

首原版CLI前固定[17案/原始argv和独立预期](../../output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/independent-cli-before-runs.json)。VS70原P1两位点为基线，新增VS71是在副本上独立构造的C1/C2同坐标、C3沿a差.0001分数即**.0004Å**、另O1远离，输入先于任何本轮targetCLI；合并后的label/occupancy/平均坐标**没有预设**。每案独立VESTA_PREF、20秒native期限、输入/输出路径、正常退出和内容合同；不运行原34格式探针。

**旋转**：-rotate_x30/-rotate_y40/-rotate_z−25分别保存标准右手旋转矩阵，原始SCENE4×4保持齐次结构；x30后y40为**Ry(40)Rx(30)**，反序为Rx(30)Ry(40)，均与首run前的主预期符合，另行登记的右乘/局部轴替代矩阵不符合。不能交换CLI操作顺序，也不把相同晶胞/位点当view未变。**缩放**：-scale2保存SCENEzoom2；-scale2 -scale.5 -rotate_z−25保存zoom1及预期Rz，晶胞/原子坐标不受缩放影响。

**去重**：-merge_split_site默认1e−6Å、dmin=.0001Å、dmin=.001Å分别保存**3/3/2**位点，与0/.0004Å独立距离判别相符。前两案保留C1(.125,.2,.3)/C3(.1251,.2,.3)/O1，较大阈值案实际保留C1(**.125033,.2,.3**)/O1，occ仍.5，U.01/.02，未将占位求和。该.125033与三源C的算术平均.12503333相符是**结果后诊断**，不是预设平均公式、所有元素/周期/占位/不确定度通过。

**列表/绝对宽度限界**：-list cell/site为选定parser中发现，bond/angle/poly为手册项；源结构没有bond/poly。五案terminal去路径/CPU行后与基线相同，保存字节亦与基线相同，未产生额外列表。-scale_width_to8在本次nogui同样terminal无额外诊断、保存逐字节等基线，不作GUI宽度成功或全命令非法判断。正bond/angle/poly列表和GUI模式仍待；旧手册分组/本地模式的准确分派须继续核。

全部17原件进程**正常退出255、无deadline/信号**，每案实际snapshot/字段核验；339个先验数值字段（矩阵/zoom、晶胞、原始coord/occ/U及sitecount）均符合登记tol（矩阵1e−6、zoom半末位.0005、cell/frac/occ/U1e−5、count精确）。返回255依R11既有解释，不单独当成功。[逐字段与metadata观测](../../output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/cli-geometry-crosscheck.json)。本机nogui不等于无图形会话或Linux证明，未使用GUI/无遗留target。cmdRotateX/Y/Z0x1f960/1f9fc/1fa98各156bytes、Scale1e9f0/124bytes、Merge1f7c0/288bytes、ListGeom1f474/788bytes已读；Scale读取scene+0x120后相乘写回，Merge0x1f89c调用removeDuplicateAtoms6c1a0，只绑定已测路径。

新增CLI.md为第32份权威文档，31既有文件及新增前不存在状态跟踪。旧70记录完全保持、新VS71；27新Evidence至629，旧602与12头10resolved/2investigating保持（DB4/source8）。原发布包257及前64包8289文件保持，254功能/71样本、10完成19未完成。calcf/Saveformatoptions/GUIopen-reopen-close-export-flush/正几何列表/所有外部程序和新发现命令仍待，完整CLI/Study未完成；Codex自验、dsh独立复审未开始。

## R66 非空CLI几何列表与-o别名

复用VS0054规则SiO6/10Å立方/Si居中±2Å六O，71样本记录完全不变。首GUI前固定源CIF SHA、六独立距离2Å、15配体对中12个90°/3个180°、V32/3、D0/λ1/ECN6。原版GUI71806从工作CIF新增Si/O1.9..2.1Å规则、ShowpolyhedraOn；原生树保存输入规则和**7atoms/6bonds/1polyhedron**。SaveAs回读本轮Where及positive-project.vesta新名，未覆盖CIF；派生工程供CLI读取，不将GUI生成文件本身当科学预期。GUI正常0/102.784秒，Quit后ps无AX/无遗留。

GUI退出后、首次CLI前登记工程SHA和[8个准确argv](../../output/vesta-study/20261108-113622/reviews/20261109-083000-s07-cli-positive-geometry/positive-cli-before-runs.json)，每个20秒native期限/独立偏好/新输出。**-list bond**实际12条有向记录，6个Si→O和6个O→Si，去方向覆盖6条物理边、每条2.00000(0)Å；双向顺序/条数为原行为观察，不冒充首GUI前行排序预期。**-list angle**给15个不同O–Si–O组合，12×90.0000(0)°和3×180.0000(0)°，每个组合均以源坐标内积独立核对。**-list poly**给1份POLYHEDRON、六距离2.00000、均长2.0000、体积**10.6667Å³**、畸变0.00000、二次伸长1.0000、ECN6.0000。单参数字符串**bond,angle,poly**可组合，含三类完整报告。两组独立/组合合计**76个原GUI前科学数值预期全部符合**，tol距离1e−5Å、角1e−4°、V1e−4Å³/λ/ECN1e−4、D1e−5；没有非零SU传播验收。[逐报告/字段比较](../../output/vesta-study/20261108-113622/reviews/20261109-083000-s07-cli-positive-geometry/positive-cli-crosscheck.json)。

**-o别名**此处实际原版保存成功，结果与同源-save基线逐字节相同；8案所有工程也逐字节相同，列表无持久化编辑。各案读取后均明确7/6/1，无界面可重建该非空几何，不能由旧空列表推定功能缺失。相反，本地parser出现的**-list cell/site**两案即使有非空几何也无额外报告，仅保留原读取日志；不把发现命令名字当实现支持。所有8target正常退出255/无信号、无期限，wrapper0与内容/完成分别核。

静态ListBonds0x1f128/268bytes、ListAngle0x1f234/344bytes、ListPoly0x1f38c/232bytes已读，分别到CalcAtomDist/CalcAtomAngle/Output_Poly_info。CLI-CalcF0x1ea6c/1236bytes另已读：出现source=/lambda=/resolution=/anomalous=/out=，整数/双精度解析后0x1edb0调用Crystal::CalcI、0x1edd0调用Reflection::ExportData；默认输出由源stem+.txt，准确默认值/参数意义仍需下轮核定，**本轮没有执行-calcf**，不得写成数值通过。未测试GUIIPC、一般PBC/坏输入/完整ORFFE/其它列表/所有选项。

前65包8531文件、原发布包257保持；16新至645Evidence、旧629与12头10resolved/2investigating保持（DB4/source8），254功能/71样本、10完成19未完成。32权威文件跟踪（含R65新CLI.md）；完整S07/Study继续，Codex自验，dsh独立复审未开始。

## R67 CLI结构因子的显式参数与空结果

复用VS0055/P1/10Å立方、C(.1,.5,.5)/O(.3,.5,.5)、occ1/B0，71样本记录不变。首targetCLI前固定5案准确argv、原始CIF/已有DABAX与NIST历史参考SHA、独立六个±轴HKL的F/2θ/相对I；外部系数与数学均独立于原件代码，未重新下载/替换既有来源。F=Σg f exp(+2πi h·r)，X-ray f0用R34外部五Gaussian，neutron用R37历史bC6.646/bO5.803fm（不是当前推荐值）。d10，六反射同d使LP在相对强度归一化时抵消，I=100|F|²/max|F|²，不声称一般LP复核。[运行前先验/参考身份/argv](../../output/vesta-study/20261108-113622/reviews/20261109-084000-s07-cli-structure-factors/independent-calcf-before-runs.json)。

原版实际语法为 `-nogui -i FILE -calcf source=0 lambda=1 resolution=9 anomalous=0 out=FILE -save PROJECT`，参数分别作为完整argv项。**source=0/anomalous=0**在λ1与λ2两案均输出6条HKL，(−100)复F为2.241675−10.795576i、|F|11.0259，(010)/(001)为−13.547204+0i；所有F/d/相对I在两波长逐打印数相同，而2θ为**5.73197°/11.47834°**。**source=1/anomalous=0/λ2**匹配独立中子期望，(−100)为3.583501−9.425402i，|F|10.0836，(010)为−12.449000+0i。三案共**108数值字段**（每案6HKL×d/Re/Im/|F|/2θ/I），全部在运行前登记的实际打印半末位容差内；每行M1另精确核。原十进制外部系数为主、binary32系数控制另保留，没有改原Oracle。[逐字段/原始token/副控制](../../output/vesta-study/20261108-113622/reviews/20261109-084000-s07-cli-structure-factors/calcf-crosscheck.json)。

λ2案`out=`使用本轮**中文 输出/波长2 反射.txt**完整含空格/中文路径，真实文件和成功日志均存在。不能拆分argv中的out路径。**anomalous=1**另案实际改变F，±100分别(2.254524,−10.818890)/ (2.226221,10.816609)，|F|11.0513/11.0433不再相同；该案未套中性f0数值标准，只证明开关影响和产物，FFAST/反常项准确性仍待。

**resolution=10.1Å**无HKL满足d≥10.1，原件没有生成反射文件，日志实际 **Failed to write structure factors to ...**，随后`-save`仍生成有效两位点工程，target依然正常**255**。与另四案成功同码，故退出码/后续工程保存不能充当反射计算成功。该空集合诊断在静态0x1edb4..0x1edd8条件分支可见，不能将它误报为权限/磁盘错误或“文件写成功”。native terminal180列产生普通换行，output_truncated=false，失败路径最后字符在下一行的表示保留。

五个target均20秒native期限、独立偏好/新输出、正常255/无信号或deadline、无GUI/无遗留；原输入和source/refSHA保持。R66读的CalcF0x1ea6c/1236bytes为静态绑定，参数source/anomalous用整数，lambda/resolution用double。默认source0/anomalous1/resolution.7/lambda**.709317Å**/stem+.txt由初始指令常数/拼路径推导，**本轮全部显式参数，没有运行默认值验收**。未核坏out路径/默认输出/其它source、所有散射/完整命令。

前66包8710文件、原发布包257保持；6新至651Evidence、旧645与12头10resolved/2investigating保持（DB4/source8），254功能/71样本、32权威文件、10完成19未完成。S07默认/输出错误/保存选项与GUIIPC及其它Study范围继续；Codex自验，dsh独立复审未开始。

## R68 默认参数、默认文件名勘误与输出失败

复用VS0055/71记录全不变。首5个target前登记默认/显式两套参数、输入SHA、9Å与.7Å独立整数HKL枚举（**6/12196**个）、半末位d/2θ/M核法；另有缺失输出父目录受控失败。原始计划[defaults-before-runs](../../output/vesta-study/20261108-113622/reviews/20261109-085000-s07-cli-defaults-errors/defaults-before-runs.json)和[独立全HKL](../../output/vesta-study/20261108-113622/reviews/20261109-085000-s07-cli-defaults-errors/independent-allowed-hkl.json)均保持。

**必须勘误R66/R67的静态“源stem+.txt”推断**：本轮两个默认调用的原版日志实际都是 **Structure factors were saved to cif.txt**，工作目录为E/s04，因此默认写的是工作目录内的cif.txt，而不是预期的输入绝对路径主名。R66已读CalcF代码0x1eb64..0x1eb7c向后找点，0x1ec20/0x1ec28从最后点之后复制后缀，0x1ec64再追加.txt，与所测CIF产生cif.txt相符。没有现场寄存器追踪，未对其它后缀/无点/点在目录中/对象标题fallback作一般运行验收。R67各案显式out数值结论不受这条默认名勘误影响。

**研究采集偏差原样保留**：原default-limited和default-full共用cwd，后者覆盖了前者的cif.txt；默认路径当时未在watched_paths中。首份limited表已不可取得其原SHA，不能拿重跑文件冒认原始文件。原5次计划/参数/日志未改。后一个full文件仍在E/s04，复制到本轮discovered-default-full-cif.txt，SHA **3072765578612b1da122dd1b487de8ae0380a104d24ab73b779fbe86fc50c967**，与显式full输出一致；该路径不属于旧清单文件，前67包仍核。见[发现/恢复](../../output/vesta-study/20261108-113622/reviews/20261109-085000-s07-cli-defaults-errors/default-output-discovery-recovery.json)。

在观察默认名字后、两次新target前，登记[隔离后续先验](../../output/vesta-study/20261108-113622/reviews/20261109-085000-s07-cli-defaults-errors/isolated-defaults-before-runs.json)，每个独立新cwd且监控整个cwd（输入/偏好/项目/cif.txt），不用共享s04。新limited仅覆写resolution=9，新full不提供任何calcf选项；二者真正产生本cwd/cif.txt，并与source=0、lambda=.709317、anomalous=1、resolution9或.7的对应显式输出**逐字节相同**。因此本选定P1输入的组合默认参数得到运行对照，不再只是静态常数；但这个文件名预期是**原观察后登记的后续验证**，不能把原stem假设写作通过。

四份有完整监控身份的表（显式limited/full、隔离默认limited/full）共**24404行**，每一行HKL集合与预先独立枚举相同、无重复；**73212个d/2θ/M字段**按d=10/|h|、2θ=2asin(.709317/(2d))、P1的M1在打印半末位内符合。full最小d由n²≤204边界确定，12196行；max|d误差|4.9987909645e−7Å、max|2θ误差|4.9785540455e−6°。这里是网格/角度/多重度及组合默认值的兼容核，**不是12196个反常散射F的独立准确性验收**。[比较/身份/偏差边界](../../output/vesta-study/20261108-113622/reviews/20261109-085000-s07-cli-defaults-errors/defaults-crosscheck.json)。

**缺失父目录输出**：同一有效输入、source0/lambda1/resolution9/anomalous0，out指向运行前后均不存在的missing-output-parent/cannot-write.txt。原件实际报Failedtowritestructurefactors、无反射文件，却继续-save写有效工程/正常255。对照R67同参数显式可写输出，这支持输出路径失败的受控解释；没有errno/系统调用捕获，不臆报具体errno。与空HKL同样需要逐命令内容合同，不能依255或后续工程判断计算成功。

总计7个target（原5+隔离后续2）正常255/20秒内/无信号期限、无GUI/无遗留。8新至659Evidence、旧651和12头10resolved/2investigating保持（DB4/source8），旧71样本保持、254功能/32权威、10完成19未完成。原发布包257及前67包8846文件保持；原文件名假设明确失败，恢复后选定合同通过；一般命令/默认文件名/反常F/GUIIPC/其它Study范围继续。Codex自验，dsh独立复审未开始。

## R69 CLI保存选项、居中晶胞与显示场景

新增独立VS0072/NaCl/SG225 Fm−3m/4Å立方、Na原点/Cl(.5,.5,.5)、occ1/B0。首target前固定四个F平移及其展开、常规各4原子/V64、原胞各1/V16、Cartesian转换、闭0..1盒14Na/13Cl、原始SHA与8案argv；旧71记录保持。[初始先验](../../output/vesta-study/20261108-113622/reviews/20261109-090000-s07-cli-save-options/save-options-before-runs.json)。每target独立cwd并watch整个目录，VESTA_PREF亦归本轮，无共享s04输出。

**VASP**：默认Direct保存cell4³/V64、Na4Cl4与独立F展开8位置；option=cartesian同胞/组成，位置分别乘4Å。option=pcell原胞基矢(0,1.9999999658,1.9999999658)等，option=reduced基矢为同组的循环置换；两案V=**15.999999179200016Å³**、Na/Cl各1、分数0与(.5,.5,.5)，周期最近NaCl距离**1.9999999658Å**，符合V16/2Å先验（tolV1e−5、距离1e−6）。基矢分别属于原F布拉维格点、index4，未把某个矩阵朝向当唯一约定。所用tol并非输出拟合。

**原显示范围假设失败**：cell4的原生基线实际57atoms/84bonds/14polyhedra，as_displayed保存**14Na/43Cl**，而非原闭盒27/14+13；两项原预测失败行保留。原生SBOND默认Na/Cl0..**3.39412Å**、A1在边界内时搜索边界外A2。根据14个闭盒Na的六个±.5分数Cl近邻构造并集得到43Cl，原文件43个原始坐标全部相符，其中**30Cl在闭盒之外**。这是[结果后独立几何解释](../../output/vesta-study/20261108-113622/reviews/20261109-090000-s07-cli-save-options/displayed-boundary-postdiagnosis.json)，不是首实验前oracle。

在观察后、再次target前固定[8Å变体先验](../../output/vesta-study/20261108-113622/reviews/20261109-090000-s07-cli-save-options/wide-displayed-before-run.json)：只把晶胞4改8，最近NaCl变4Å、大于原默认成键阈值，其余分数/SG保持。新原件实际27atoms/0bonds/0polyhedra，as_displayed保存**14Na/13Cl、V512**，所有27原始坐标符合事先闭盒预期，未用模1把重复边界折叠。这区分as_displayed导出整个当前场景与只输出常规胞；一般过滤/隐形/递归/标签条件仍待。[全案比较与原失败](../../output/vesta-study/20261108-113622/reviews/20261109-090000-s07-cli-save-options/save-options-crosscheck.json)。

**.ins默认与覆盖**：不传format的.ins是真实SHELX，CELL为4/4/4全90、LATT4、SFACNa/Cl、UNIT4/4、两个不对称位点分数0/.5。SOF原文**10.02083/10.02083**保留，未将其当直接occ1或声称特殊位置SOF/实际精修正确。传format=rietan后另产40538字节模板RIETAN-FP输入，VNS1 A-225-1、CELLQ4/4/4全90，Na1/Na与Cl1/Cl的occ1、0/.5坐标/B0符合源字段。只有独立字段/格式判别，**没有运行RIETAN-FP或SHELXL**，也未把模板注释复制为项目正文。

静态cmdSave0x1e654/852bytes沿用R53原证据，option标志分别cartesian2/pcell4/reduced8/as_displayed16（另有omit_hidden/nitech待）；writerVASP0xb647c/2840bytes到convertToPrimitiveCell0x72fc8/1096bytes、convertToReducedCell0x734a0/2544bytes。后者用_prefdir全局0x52d780的tmp/data.stin及data-tmp.sto走TIDY。此次实际报告含Niggli变换、2.8284Å/60°及体积比.25，位于归本轮的preferences/tmp，不改原发布包。这里是**CLI约化导出应用变换**，不能与R56选定GUI Niggli报告后活动结构不变的结论混淆。SHELX0xb006c/2056bytes、RIETAN0xb0874/3640bytes另读，绑定格式分派，未复制反编译实现。

9个target正常255/20秒内、无GUI/无遗留；原发布包257和前68包8999文件保持。17新至676Evidence、旧659和12头10resolved/2investigating保持（DB4/source8）。新VS72/旧71不变，254功能/72样本、32权威、10完成19未完成。原闭盒假设不通过、补证后选定合同符合；全保存选项/其它输出/GUIIPC/Study继续，Codex自验、dsh独立复审未开始。

R69验收补记：原九个target的监控均完整；初次分析辅助进程watch_depth=2在空preferences/style与tmp目录处置file_list_cut_short=true，专项验收如实失败。原报告/执行未覆盖；独立递归20个实际文件、全部与原snapshot哈希一致，再以显式文件/深度6/上限1000做native完整监控审计，child0/cutfalse/23节点。科学比较和原件均未重跑。规范native-evidence-final.json为676证据/17新；原675导出仅中间态。使用新的scoped主/专项验收与实际运行，保留初失败。

## R72 单原子箭头转位点与内部powder空数据集崩溃

复用VS53反演样本：10Å立方/P−1，C(.1,.2,.3)/occ.5和O原点/occ1/B0；独立周期唯一C2+O1，闭0..1显示C2+O8=10。首GUI前登记源SHA、单C原子轴矢量(1,2,3)、位点/单原子区别和有界归属进程，[运行前GUI先验](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/vector-before-gui.json)。复读此前已取得的VM09 §9.1.1–9.1.3，仅取单原子/位点分配与axial TP M/polar M公式，记录本地正文SHA；不把本机二进制算法当独立科学预期。

**GUI源与输入精度变化**：原版GUI只给IndividualAtom首C1(.1,.2,.3)分配箭头1，另一个C1(.9,.8,.7)初始Vector空。创建框实际设置Cartesian(1,2,3)，Tab后Modulus显示3.74，点OK后列表/文件成为**(.99956,1.99911,2.99867)**、晶格表示(.09996,.19991,.29987)。因此初始请求分量精确保留的假设未成立；结果后数学解释v×3.74/√14符合五位打印半单位，不伪称首运行前精度oracle。源VECTR实际个体行**1 1 0 0 0**，单独SaveAs individual-vector.vesta后源SHA2bc29cc38688cc9c2253eb9a6495b97e4fbe246e2d952c3eb177ce5300ef92fa，后续CLI运行前另登记其实际分量/文件身份。此处研究既有界面提交序列的行为，没有修改原版。

**v2site真实转换**：lib0x1e31c/320bytes遍历各Crystal的AtomArrow集合，个体分配按5整数一组取首siteID去重加入siteIDs，再清空个体分配；AtomArrow构造0xd314/124bytes和ResetArrowIDforSite0x6c824/408bytes交叉定位字段，未由函数名猜成体数据操作。CLI基线-save与GUI源逐字节相同。-v2site后仅VECTR个体行由**1 1 0 0 0→1 0 0 0 0**，其余所有工程字节保持（含组件、类型、cell、结构/占位/B/视图）；连续两次与一次产物逐字节相同，SHA**dfb204be2c964ccc1bbfee001d7f35d861893fe9e891c0582425eb2b4b75ad50**。这是个体分配转换为位点分配的幂等性，不是重新计算向量分量。

新归属GUI重开转换工程：CrystallographicSites里C1的Vector=1、O空；IndividualAtom两个C1的Vector列均空，主渲染实际有**两个同方向红箭头**。初始回读补充先验曾预测IndividualAtom两行也显示1，实测不成立；该表显示直接个体分配，不能把空列当位点箭头未生效。CUA截图直接观察网格和两箭头，原生AX保留对话框/控件状态但wx网格行未暴露，未声称它能机器提取行内容或对3D角度作数值校准。轴矢量经反演同向符合独立TP M公式，本轮未测polar/磁性时间反演或所有非正交表示。

**内部powder不是可直接用普通CIF完成的命令**：0x1f8e0/104bytes在Scene非空后取Scene+0x150的数据集集合首指针，然后调用GetFormFactor、DataSet::PrepareCalc0xceb50、Simulate0xcec38/784bytes。它不读命令argv也没有导出调用；静态存在计算链不证明本次到达。首CLI前固定6案，其中四个有效基线/v2site案正常255。第五案为VS55普通CIF（读取本身正常、两site/10Å立方）执行-powder再-save；原生工具记录**code1/reason=exited**且无Save文件，运行器断言停止，第六powder-extra-argument案**未执行**，不继续碰同一崩溃。

本次PID**2124**系统诊断文件完整归档，[PID/故障摘录](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/powder-crash-source.json)及原始powder-2124-crash.ips：EXC_BAD_ACCESS/**SIGSEGV11**、KERN_INVALID_ADDRESS0，故障lib imageOffset **0x1f8fc=Powder+28**，原生指令**ldr x20,[x9]**、x9=0，绝对PC−imageBase相符，原库UUID **5d1ec64c-a09f-3f1d-97a2-79f51cd3dce2**由dwarfdump核一致。具体原因是有Scene但**DataSet集合为空，解引用begin未判空**，发生在GetFormFactor/PrepareCalc/Simulate调用之前；不能称为完成衍射计算后的数值错误，也不把GUI粉末计算功能一概判坏。无需再执行崩溃或安装外部程序。

后续归属GUI出现“上次意外退出/是否恢复窗口”系统提示，原生AX保留后选择Don’t Reopen，不恢复旧窗口，随后实际打开研究副本。两个GUI均正常0/各小于360秒（lifetime原件为准），Quit后ps无进程/无AX重启；短偏好目录文件另归档。**native的reason=exited/code1不能独立证明无信号**，此处必须结合系统SIGSEGV记录，不改本研究锁定工具或伪造其退出字段。

[20项选定校验、三项拒绝先验与边界](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/powder-v2site-crosscheck.json)和[停止记录](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/cli-run-failure-recovery.json)区分4正常255、1系统SIGSEGV、1未运行。22新至764Evidence、旧742/12头10resolved2investigating保持；前71包9798文件/原257保持。254功能/72不变样本、32权威、10完成19未完成。完整粉末科学、axial/polar及CLI/Study未完成，Codex自验，dsh独立复审未开始；未将查明原因当修复原件。

## R73 内置GUI粉末正向计算、导出入口差异与峰形位置

复用VS70/4×5×6Å正交P1、C(.125,.2,.3)/occ1/U.01，O(.625,.4,.5)/occ.5/U.02，V120，72样本记录不变。**首GUI前**用正交倒易度量与Bragg定律枚举整数HKL：λ1Å、2θ9..10°时只允许±001，d=6Å、2θ=**9.560383694398318°**；反射角tol.0001°、d tol.00001Å，采样峰最大值最近网格tol.0051°均先登记。[完整先验/计算/源SHA](../../output/vesta-study/20261108-113622/reviews/20261109-101000-s06-powder-positive/powder-before-gui.json)。没有使用原件实现形成这组几何数值。

**当前版本具有内置路径**：VM14旧手册§14.6描述RIETAN-FP→绘图外部链，原包有RIETAN.sh/template.ins而无RIETAN二进制。本版OnPowderPattern0x10009bde4/2112bytes按偏好字符串是否为空分两路：非空有RIETAN.sh/wxExecute，另一支构造DlgPowder。本轮新短偏好目录实际打开Conditions/Reflections/Plot内置窗口；DlgPowder构造0x100066cd4/2752bytes创建DataSet的ExpCond与ObsData，OnCalculate0x100063c80/156bytes到PrepareCalc/Simulate/ResetList。此正向GUI创建数据集后计算，不能混同R72裸内部CLI在空数据集首指针处崩溃，也不能将旧手册外部路线当本轮真实执行。外部RIETAN联动仍单独待核。

实填No.ofλ=1、λ1Å、相对强度1、范围9..10°/步长.01°，CTHM1/PCOR.5、Z/Ds/Ts全0，Calculate前原生AX保留输入。由于减少波长数后动态重建控件，AX创建顺序将λ与weight放在末尾，语义仍由相邻标签确认。两条反射实际±001/d**6.000000**/2θ**9.56038**，符合独立几何先验。其F为(−5.343395,−5.076222)/|F|7.3702和(−5.349935,5.061493)/7.36482，归一I100/99.85387，M1、IDλ1、Phase1。主输出实报X-ray dispersion λ.100000nm及C/O反常系数，实际为X-ray含反常，不声称已切换成中子。

**反射表按钮与菜单不可混同**：Reflections页Export...按钮通过AXpress与可见坐标点击各一次，都无Save对话框/可见变化；第一次后原生全窗口树也无Save。File→Export Reflection Table却立即打开Save，在已核Where=本轮cwd后保存reflections-single-lambda.txt，真实2行/完整字段。静态DlgPowder::OnExportTable0x100066750/8bytes是**尾跳PanelReflection::OnExport0x1000656e8/1128bytes**，后者有文件对话框与Reflection::ExportData，不能因8bytes叫stub或猜剪贴板路线；按钮无效的具体绑定原因本轮未证，菜单成功不算所有入口通过。File→Export Data另保存profile-single-lambda.int；其Save面板标题竟写Export Graph，但后缀与产物为数据，标签差异保留。

**原采样峰先验失败**：.int以GENERAL$/100开头，实际100个三列行，x=9+.01i、i0..99，末9.99而非包含10；全部有限。反射位置9.56038正确，但曲线最大行是**x9.55/y100/第三列0**，9.56处只有93.91134；相对独立Bragg峰差**−.010383694398318°**，超过先验.0051。Plot亦显示约9.55的峰线和约9.56的反射标记，不能把表峰通过写成峰形准确。结果后“最近网格点再左移一个bin”的模型吻合本案，**只是待检假设**；下一次先登记步长.02等留出对照，当前不以拟合解释关闭该问题。完整峰形、积分强度、Lorentz/偏振/位移/背景等仍未独立验收。

GUI结果之后、专门CLI target之前另登记[跨入口比较先验](../../output/vesta-study/20261108-113622/reviews/20261109-101000-s06-powder-positive/calcf-control-before-target.json)。同源-nogui -calcf source=0 lambda=1 resolution=5.9 anomalous=1 out=新绝对路径，之后-save；正常255/完整watch/有效工程，反射2行前10token共**20个原始token与GUI导出完全相同**（不含GUI专有IDλ/Phase）。这是跨入口一致性，反常F精度并非独立通过；不把GUI原件输出当外部散射基准。

**时限偏差**：GUI归属PID15780正常0，lifetime**384.496秒**、未到570秒硬清理，超过360秒软时限**24.496秒**。末次原生读取在软期限前5.9秒发起却耗11.734秒，随后仅做退出清理；不改回执或伪称软期限达标。Quit后ps无VESTA，无AX重开；偏好文件另归档。后续研究应在软截止前至少60秒开始收尾，不在临近截止新增长读取；出现入口调查分支就保存现有产物、换新归属实例继续。

[19项几何/跨入口/证据完整性校验](../../output/vesta-study/20261108-113622/reviews/20261109-101000-s06-powder-positive/powder-positive-crosscheck.json)全部符合其限定范围，但**原曲线峰最近网格假设不通过、页面按钮无效和软期限偏差均保留**；独立粉末科学、所有入口与Study不完成。12新至776Evidence、旧764/12头10resolved2investigating保持；前72包10006文件/原257保持，254功能/72样本、32权威、10完成19未完成。Codex自验，dsh独立复审未开始。

## R74 粉末采样步长/起点留出：一格左移模型被否定

为检验R73结果后提出的“最近Bragg网格点再左移一格”模型，在新原件GUI前登记两个独立实例/短偏好目录/源SHA与具体预测，继续复用VS70，λ1/weight1、CTHM1/PCOR.5、Z/Ds/Ts0，其余结构相同。[运行前留出计划](../../output/vesta-study/20261108-113622/reviews/20261109-103000-s06-powder-step-holdout/step-holdout-before-targets.json)。首登记脚本路径拼接错误，只创建32份before和首输入副本后中断；一次只读kernel目录查询先于成功写出的计划，修正后明确标afterInitialKernelCatalogOnly，**所有产品target均在计划写出之后**，未偷改预测或样本。失败/顺序另保留preparation-failure-recovery.json。

**A/step.02**：xmin9、xmax10，预测半开50行。独立Bragg仍为±001/d6/2θ9.5603836944；最近网格9.56，一格左移模型和floor后减一格模型均事先预测峰**9.54**。实际50行、x9..9.98、最大值却为**9.56/100**，9.54只有97.61353。**注册的一格左移预测失败**；不能把R73单案9.55拟合当普遍索引错误。

**B/起点9.005**：xmin9.005、xmax10.005、step.01，预测100行。最近Bragg网格9.565，一格左移模型预测**9.555**，floor后减一格另预测**9.545**。实际100行、x9.005..9.995，max为**9.555/100**，9.545为98.71818。此案单独吻合第一模型，却不能挽救A的反例；floor−1两案皆被否定。反射表两案与R73逐字节相同，仍±001/d6/2θ9.56038/F/I/M/λID/Phase相同，独立几何保持。B采样峰相对Bragg差−.0053836944，略超step/2+.0001=.0051；A差−.0003836944，符合其.0101近格容差。各容差/预测均在本案GUI前登记，没有结果后放宽。

本轮结论是**否定单bin全局模型，峰形原因未解决**。连续位移、峰形不对称、初始化网格及其它因素仍待证；采样最大值也不必天然等于反射Bragg位置，不能仅据此宣布软件缺陷或已恢复完整粉末科学。原库ExpPhaseParamProfileTCH::CalcProfile0xd2234/1472bytes含Lorentz/Gaussian混合及ObsDataPowder::FindIndex0xd7384/72bytes；后者对实际x数组作二分定位。仅已定位原生路径，未现场绑定使用TCH还是Ida分支，也未形成全峰形独立oracle，不能强行从局部汇编补写因果。

两案新GUI只用已证File菜单导出.int与反射TXT，跳过R73已知无动作页按钮；参数计算前各取原生UI读证，控件值与计划相同。使用完整AX树与唯一匹配核Save的Where和filename后保存，未写历史目录。一次AX差分正则未覆盖前置+而提前停止，改读取完整树后完成；没有继续凭错误索引写入。

**生命周期与交互目标分开**：A正常0/**176.577秒**，B正常0/**202.842秒**，都在soft240/hard360内退出，无强制终止和遗留。两个原生UI读取都在180秒前发起且完成，但B总时长超180交互目标**22.842秒**；没有逐CUA动作独立时间戳，**不声称所有GUI交互在180秒前结束、或清理一定提前开始**。保留差异，不能用软240内退出覆盖交互目标偏差；结束后不再AX重开，短pref文件归档。

[20项限定证据/几何校验与预测判定](../../output/vesta-study/20261108-113622/reviews/20261109-103000-s06-powder-step-holdout/step-holdout-crosscheck.json)明确A第一模型false/B true、备选两false、causeResolved=false。8新至784Evidence、旧776/12头10resolved2investigating保持，前73包10154文件/原257保持；254功能/72不变样本、32权威、10完成19未完成。后续核实际峰形/坐标与修正链及独立参考；完整粉末/Study仍待，Codex自验，dsh独立复审未开始。

## R75 默认不对称峰形与新波长留出（不改勾选）

原版arm64工厂链已绑定：DataSet::UpdatePhases0xce968→ExpParamDataPowder虚表+0x18/GenPhaseData0xd0878→ExpParamPhaseBase构造0xd1a00→GenPhaseParam0xd18f0；后者调用TCH构造0xd2cf4，并写入TCH_Ida虚表0x522270，其CalcProfile槽0x522290指向0xd3020。不是凭符号存在推定GUI分支。只读Mach-O解析明确选择universal库的arm64切片，库SHA保持c2c677fa…；默认参数名U,V,W,P,X,Xe,Y,Ye,rs,rd,h,k,l对应构造常量，rd=.1、rs=0。Z/Ds/Ts默认0；CalcX返回独立Bragg位置，不存在已证明的统一−.01修正。

实际Ida路径给出zmin=−rd²·cot(2θ)/2，2θ按度转弧度取三角函数。λ1/d6时zmin=−.02968659136°，λ2时−.01436762233°。因此峰顶可以偏离反射Bragg位置；R73最近网格假设与R74固定一格模型的失败记录继续保持。TCH宽度多项式/eta见ITC2019§3.3.2；实际本版把计算出的H直接传给Lorentz/Gaussian核作为gamma_L/gamma_G。**不能把它无条件称为标准FWHM约定**：独立按通常FWHM换算的备选模型对旧三谱内部强度最大差27.53–29.14/100，已否定该绑定假设；这也不能仅凭命名宣布原版公式错误。

采用Ida1998 Eq1/4的Howard窗口卷积，令z=zmin·v²消除端点奇性，用2000段复合Simpson独立积分，不调用原版P_G/P_LH_sym；原版Gaussian调用点数为5。宽度默认输入来自二进制，积分数学独立，两种独立性不混淆。结合样本点LP修正后，旧三曲线最大内部强度差≤.003306/100、三个采样峰全部匹配；旧结果是模型开发数据，非留出。论文Eq15存在gamma符号疑点，未用该式或静默改写。

**新波长留出在任何新产品target前登记**：同VS70、λ2Å/权重1、19..20°/step.01、CTHM1/PCOR.5、ZDsTs0，预报±001/d6/Bragg19.18813645372°、100行半开网格、采样max19.18°；内部index1..99误差容差事先固定.01/100，完整CSV和oracle SHA先写出。原版反射2行19.18814/d6与独立几何吻合；实际max19.18/100，内部最大误差**.0016398951/100**，通过预定限值；2000/4000段积分收敛差<1e−8。

**首点科学失败保留**：λ2首行x19原版y0，独立卷积**1.94239831685/100**；旧三谱亦首行0而积分正值。Ida左循环0xd32e4递减后以b.gt回环，写到index1即止，右循环从峰索引+1开始，解释本组index0未写。该已知边界例外单独测试，不能把内部检查通过称整谱科学验收。仅当前P1/单反射族/单波长归一化峰形得到因果解释，完整多反射、多波长、异常散射绝对强度/外部RIETAN及全Study仍待。

一个拥有的GUI/PID46731正常0退出，**123.251秒**，低于180交互目标/soft240，无强制终止与遗留。保存校验首次误用AX差分而提前抛错，读取完整树重核Where/filename后保存，未向错误目录写入。一次CLI相对--project产生独立自有临时研究工程，原件不受影响；该工程完整移入本轮misaddressed-project，随后绝对路径在权威工程重取，同一静态内容ID去重为一条。初次Mach-O读取误按thin而断言中止，后改universal arm64切片；这些准备失败不删、不冒充成功。

[24项限定检查](../../output/vesta-study/20261108-113622/reviews/20261109-105000-s06-powder-profile-cause/profile-crosscheck.json) / [工厂与参数绑定](../../output/vesta-study/20261108-113622/reviews/20261109-105000-s06-powder-profile-cause/static-default-profile-binding.json) / [预登记预测](../../output/vesta-study/20261108-113622/reviews/20261109-105000-s06-powder-profile-cause/lambda2-before-target.json) / [原始曲线](../../output/vesta-study/20261108-113622/reviews/20261109-105000-s06-powder-profile-cause/lambda2-gui-cwd/profile-lambda2.int) / [一手来源及阅读边界](../../output/vesta-study/20261108-113622/reviews/20261109-105000-s06-powder-profile-cause/primary-sources-ledger.json)。24新至808Evidence、旧784及12问题头保持，前74包10293文件和原257保持；254功能/72不变样本、32权威、10完成19未完成。Codex自验，dsh独立复审未开始，未开展产品实现。

## R76 双波长、相对权重与条件峰形（不改勾选）

新拥有GUI前登记VS70、λ=[2,2.1]Å、相对权重[1,.5]、19..21°/step.01、CTHM1/PCOR.5、ZDsTs0，独立±001/d6各一族，总4条反射/λID1、2。独立Bragg为19.18813645372°、20.15731621558°，各分量采样峰预测19.18°、20.15°。实际反射4条/M1/Phase1/d6，对应2θ19.18814、20.15732均在预定.0001°内，曲线200有限三列行/半开19..20.99。

预先登记的组合式是LP(x)·Σλ[wλ·Σ±001|Fλ|²·Howard(x;λ)]，max归一到100。**振幅取原版独立导出的反射表|F|²作为条件输入**；这检验积分和相对权重叠加，不独立证明F/绝对反常散射正确。没有把已含其它修正的显示I当振幅，也未结果后拟合权重。复用R75封存的独立2000段Simpson代码（与本轮副本SHA相同）；原版先运行、反射表后取得是条件计算的明示范围，不能称完整预报绝对强度。

实际双峰为**19.18/100、20.15/44.54929**。条件积分预报第二峰44.54943383，index1..199最大差**.001645282164/100**，通过target前固定.01；2000/4000段积分收敛差<1e−8。事先登记的忽略权重备选把第二峰预报89.00245180，内部最大差44.45316180，明确被否定。首点原版0而条件独立积分1.96803397724，科学失败继续保留，**不宣称整谱/完整多反射/全部粉末通过**。

PID51848正常0/**94.347秒**，早于180交互/soft240/hard360，无强制终止和遗留。计算前原生AX完整读取13输入字段与登记相同；保存用完整树核Where=dual-gui-cwd及两个文件名，未写旧目录。3新至811Evidence，旧808/12头保持；前75包10485文件/原257保持，254功能/72不变样本/32权威，10完成19未完成。Codex自验，dsh独立复审未开始，未产品实现。

[运行前登记](../../output/vesta-study/20261108-113622/reviews/20261109-112000-s06-powder-dual-lines/dual-before-target.json) / [15项限定检查](../../output/vesta-study/20261108-113622/reviews/20261109-112000-s06-powder-dual-lines/dual-crosscheck.json) / [实际曲线](../../output/vesta-study/20261108-113622/reviews/20261109-112000-s06-powder-dual-lines/dual-gui-cwd/profile-dual.int) / [实际反射表](../../output/vesta-study/20261108-113622/reviews/20261109-112000-s06-powder-dual-lines/dual-gui-cwd/reflections-dual.txt) / [条件oracle与备选逐点结果](../../output/vesta-study/20261108-113622/reviews/20261109-112000-s06-powder-dual-lines/conditional-dual-oracle.csv)。

## R77 峰位修正反例与360分母纠读（不改勾选）

target前方案v1/60°的LP备选分离仅.00679，低于固定.01容差，因实验区分力不足而在任何产品运行前改v2/80°，v1与预测完整保留。最终预登记λ7.713451316238471Å、range79.5..80.5/step.01、CTHM.8/PCOR.999、Z=.05/Ds=.04/Ts=−.03，VS70仅±001/d6。使用当时误读的Z+Ds cosθ+Ts sin2θ，预测反射80.05109754513°、采样峰80.05°、内部容差.01/100。

**原预期失败**：反射2行实际80.06830°、峰80.07/100，原模型最大内部差**11.83108048/100**。不是通过，也未因完整性15检查passed而改称科学通过。计算前原生AX11字段同输入，波长失焦显示7.71345，偏振/位移值均符合。GUI/PID55886正常0/**141.195秒**，早于180/soft240/hard360，无遗留。

逐指令重读R75已封存CalcX0xcf5cc：d8=asin(qλ/2)/(π/180)=θ度；0xcf634..638建立**0x4076800000000000=360.0**，随后cos参数d8·π/360，sin参数2d8·π/360。**Codex之前把360读作180，导致原预登记的三角参数错误**。以物理T=2θ表示，本版实际映射是 **Tcorr=T+Z+Ds cos(T/4)+Ts sin(T/2)**（三角角度均按度），不是先前模型。此为本版指令契约，不替原版给出一般物理正确性背书；默认ZDsTs全0的R73–R76结论不受影响。

新映射只作**事后解释**：完整精度λ给反射80.06830407654°，落在实测打印80.06830的半单位内，曲线内部最大差**.005690446968/100**；若把显示7.71345视为重写计算值，则预测80.06828767295，超出打印半单位，曲线差.01226636827。更支持显示舍入未损失内部精度，**尚无新输入验证**。把修正后的模型再换回默认LP，事后误差.91952635；不能因此把原始LP假设记作前瞻成功。首点仍原版0/独立积分正值，整谱科学失败保持。

[原方案及失败预测](../../output/vesta-study/20261108-113622/reviews/20261109-113000-s06-powder-corrections/corrections-before-target.json) / [完整原预测CSV](../../output/vesta-study/20261108-113622/reviews/20261109-113000-s06-powder-corrections/corrections-prospective-profile.csv) / [15项完整性与如实报告检查](../../output/vesta-study/20261108-113622/reviews/20261109-113000-s06-powder-corrections/corrections-crosscheck.json) / [实际反射](../../output/vesta-study/20261108-113622/reviews/20261109-113000-s06-powder-corrections/corrections-gui-cwd/reflections-corrections.txt) / [实际曲线](../../output/vesta-study/20261108-113622/reviews/20261109-113000-s06-powder-corrections/corrections-gui-cwd/profile-corrections.int)。3新至814Evidence，旧811/12头保持，前76包10597文件/原257保持。254功能/72样本/32权威、10完成19未完成；下步新精确有限小数λ/新ZDsTs检验半角模型，完整粉末/Study仍待。Codex自验，dsh复审未开始，未产品实现。

## R78 半角位移模型的新输入留出（不改勾选）

GUI前登记精确有限小数λ7.7Å、VS70、range79.3..80.3/step.01、CTHM.6/PCOR.997、Z=−.02/Ds=−.06/Ts=.08。独立Bragg T=79.83242239646°、±001/d6；本版指令半角映射Tcorr=T+Z+Ds cos(T/4)+Ts sin(T/2)预报位移−.02506321385°、反射79.80735918261°及采样max79.81。原R77错角备选预报反射79.84514703/峰79.84，另登记默认LP及未修正宽度/不对称两备选；完整CSV/oracle SHA均在任何产品target前保存，容差仍.01/100。

原版实际反射2行79.80736/d6，采样峰**79.81/100**，100行有限半开网格，内部index1..99独立主文献积分最大差**.005668704972/100**，均通过原定容差。三备选内部误差依次**25.51843939、.7432170404、.02131746091**，均被预定.01限值否定。这是R77纠读所得映射的真正新输入验证，**不改写R77原预测失败，也不把本版契约等同于一般物理修正公式正确性**。非默认LP和修正后角度进入宽度/不对称的归一化形状在本案成立，绝对反常F/多反射族尚未独立验全。

首点原版仍0，独立积分**3.07070227202/100**，整谱科学失败保留；显示舍入精度因果本轮不另称已前瞻验证。PID57657正常0/**124.951秒**，早于180/soft240/hard360，计算前原生AX11参数一致，完整保存树核新目录/文件名，无遗留。3新至817Evidence，旧814/12头保持；前77包10711文件/原257保持，254功能/72样本/32权威/10完成19未完成。Codex自验，dsh复审未开始，未产品实现。

[运行前计划](../../output/vesta-study/20261108-113622/reviews/20261109-114000-s06-powder-shift-holdout/shift-holdout-before-target.json) / [冻结预测及三备选](../../output/vesta-study/20261108-113622/reviews/20261109-114000-s06-powder-shift-holdout/shift-holdout-prospective.csv) / [15项限定检查](../../output/vesta-study/20261108-113622/reviews/20261109-114000-s06-powder-shift-holdout/shift-holdout-crosscheck.json) / [实际曲线](../../output/vesta-study/20261108-113622/reviews/20261109-114000-s06-powder-shift-holdout/holdout-gui-cwd/profile-shift-holdout.int) / [实际反射](../../output/vesta-study/20261108-113622/reviews/20261109-114000-s06-powder-shift-holdout/holdout-gui-cwd/reflections-shift-holdout.txt)。完整粉末/外部程序/格式和上下文/Study继续。

## R79 ORFFE结果读取、选择详情与工程链接（不改勾选）

本版OnORFFE0x100099e4c只ShowDlg(11)，OnBrowse0x1000519a8打开文件选择并调用readORFFE0x100052a84；对所读直接调用链未见外部ORFFE启动。原件包257文件也无ORFFE命名可执行件；不能据此称整个用户机器未安装ORFFE。官方VM14§14.2说明这是读取RIETAN/ORFFE的.ffe结果并联动模型的查看器，不是该页现算完整协方差。源码复原/外部科学验收不冒称。

**人工构造数据，非真实ORFFE产物**：复用VS54/R66的7位点/6键/1多面体工程，独立几何Si1–O1=2Å、O1–Si1–O3=90°；.ffe故意填9.8765±.0123Å和123.456±.0045°，用来区分文件结果权威与几何重算。native解析要求原子条目数一致、额外对称行数+1一致；距离/角记录分别13/19转换，原子索引减1、对称码按1000拆分。完整语法/真实ORFFE诸版本未验全。

**首预期失败保留**：GUI前构造的显式identity行/码1文件741bytes，读后两表目视空白、详情空，没有错误提示；未继续Save。原“读取成功”预测false。PID65946正常0/**136.708秒**。静态读取器先初始化identity，诊断变体在其新GUI前登记：移除额外identity行并把五个原子引用码改0，原文件不改。两处联合改动，不能断言只有某一处独立造成首案失败。

**诊断变体成功**：733bytes的零编码文件，距离表目视一行Si1/O1/**9.8765**；角表目视一行O1/Si1/O3/**123.4560**。wx表行未暴露AX，表值证据为CUA截图观察；选中后详情有原生AX归档，分别 **l(Si1-O1)=9.877(13) Angstrom**、**phi(O1-Si1-O3)=123.456(5) deg.**，并列正确原子序号/分数坐标、(0,0,0)+x,y,z。详情采用SUE相关打印舍入，不把9.877与原文件9.8765差异冒称丢失全部精度；两者明显不同于模型2Å/90°。只证明选行详情和数据绑定，**本轮未目视主图高亮，不宣称该视觉验收**。

SaveAs写出orffe-identity0-linked.vesta；与源逐字节比较仅新增 **IMPORT_ORFFE / controlled-identity0.ffe** 相对链接，依赖实际存在，未嵌入几何表值或改结构。**尚未新实例重开**。第二PID66992正常0/**144.014秒**；两案都早于180/soft240/hard360、无遗留。首案文件面板自动状态变化使旧索引动作被CUA拒绝，重读当前树后用明确绝对路径打开，没有继续旧索引或写用户文件。

新增稳定样本**VS-0073**为已接受的人工.ffe，旧72记录完全保持；并不把首失败样本也计成功。13新至830Evidence，旧817/12头保持；前78包10823文件/原257保持，254功能/73样本/32权威、10完成19未完成。[两阶段计划](../../output/vesta-study/20261108-113622/reviews/20261109-114600-s07-orffe/identity0-before-target.json) / [16项限定检查及失败/限制](../../output/vesta-study/20261108-113622/reviews/20261109-114600-s07-orffe/orffe-crosscheck.json) / [原失败输入](../../output/vesta-study/20261108-113622/reviews/20261109-114600-s07-orffe/orffe-gui-cwd/controlled-results.ffe) / [VS73零编码输入](../../output/vesta-study/20261108-113622/reviews/20261109-114600-s07-orffe/identity0-gui-cwd/controlled-identity0.ffe) / [已存链接工程](../../output/vesta-study/20261108-113622/reviews/20261109-114600-s07-orffe/identity0-gui-cwd/orffe-identity0-linked.vesta)。完整ORFFE/实际外部计算/协方差、重开/坏输入/多相/其它格式上下文和全Study继续；Codex自验，dsh复审未开始，未产品实现。

## R80 ORFFE相对链接重开、主图选择与坏输入状态（不改勾选）

新GUI前固定R79已存工程及733bytes依赖，一起复制到新cwd，工程相对IMPORT_ORFFE不改。新实例打开Geometrical Parameters即见9.8765距离行，**未点Read *.ffe即自动恢复**；选择后原生AX详情仍9.877(13)/Si1/O1，证明相对侧文件新进程读取，非旧窗口缓存。通过R30已封存的只读SCK helper按拥有PID91883和精确主窗口ID70518取两个PNG，均860×728，原生base64解码原字节相等；先后原图均已目视。选择前模型区域无黄色标记，选择距离行且查看器保持开启时，左O1/中央Si1出现黄色圈/十字和连线；ROI黄色像素0→298。该条目→两原子主图高亮得到目视与原生截图支持，**不是全渲染等价**。首次关闭查看器后的主图无此标记，因此再次打开/选择并在查看器仍开启时拍指定主窗口，未把无高亮图当成功。

**单变量坏输入**：只删除未被所选距离/角引用的O6原子参数行，7→6，其余字节/identity0/码0不改；698bytes诊断文件不另加稳定样本ID。读取后距离表目视空白、无错误提示；顶部却仍显示此前9.877(13)及Si1/O1详情。这个旧详情残留已由CUA截图/AX观察，**失败后没有原生UI快照**：新捕获被180秒截止保护在参数文件写出/工具调用前阻止，异常与没有.params/.stdout明确保留，不伪造回执或重开已退出实例。只读UpdateList0x100053370/956bytes直接清两列表后再填充，未见直接清详情控件调用，与现象相容；有间接调用，不能冒称恢复完整源码。

原件OnBrowse先把路径写入Crystal+0x830，再调用readORFFE；失败0x100051cb0转清字符串，**并非保留上次有效链接**。运行前已按此登记：坏读后SaveAs写orffe-after-invalid.vesta，实际**逐字节等于原R66无链接5144bytes工程**，IMPORT_ORFFE消失；原5183bytes输入和733bytes有效依赖均未改。于是一次被拒绝的ORFFE读取可同时清结果表/工程链接，却保留顶部旧详情。这个失败状态/持久化语义属于1:1兼容合同，不当作科学计算通过。

**时间偏差不掩盖**：GUI91883正常0/**227.666秒**，软240/硬360未触发，无遗留；但总时长超180目标**47.666秒**，截止后才开始最后保存/关闭清理，beginCleanupBy180=false/allTimingTargetsMet=false。实际三次原生UI捕获在16.371、53.641、126.201秒开始；被拒绝的第四次没有工具调用。不能以软240内退出改称全交互按时，也不为已无原生快照的坏读状态补造数据。后续缩短单实例任务拆分。

[运行前预期](../../output/vesta-study/20261108-113622/reviews/20261109-120700-s05-orffe-reopen/reopen-before-target.json) / [17项限定证据检查](../../output/vesta-study/20261108-113622/reviews/20261109-120700-s05-orffe-reopen/reopen-crosscheck.json) / [选择前原生PNG](../../output/vesta-study/20261108-113622/reviews/20261109-120700-s05-orffe-reopen/main-before-selection-ui.png) / [选择中原生PNG](../../output/vesta-study/20261108-113622/reviews/20261109-120700-s05-orffe-reopen/main-while-distance-selected-ui.png) / [迟到捕获阻止记录](../../output/vesta-study/20261108-113622/reviews/20261109-120700-s05-orffe-reopen/blocked-ui-capture-record.json) / [坏读后工程](../../output/vesta-study/20261108-113622/reviews/20261109-120700-s05-orffe-reopen/reopen-gui-cwd/orffe-after-invalid.vesta)。6新至836Evidence，旧830/12头保持；前79包10984文件/原257保持，254功能/73不变样本/32权威、10完成19未完成。其它ORFFE格式/多相/真实外部协方差、其它功能上下文和全Study继续；Codex自验，dsh复审未开始，未产品实现。


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


## R91 构造矩阵与命令调度初始化路径（2026-11-09）

R91仅静态研究，未启动VESTA运行实例。28次原生函数/符号查询成功，17个新增ID、11个已有身份复用。Crystal构造lib0x35f68/472bytes在+150/+178/+1a0创建MatrixDD，并在36074/7c/84调用MatrixD::Initialize0x33b0；该函数_bzero后按stride+1写IEEE double1，故构造期矩阵为单位阵。MatrixDD数据指针+10与旧writer读取Crystal+160吻合。此为静态对象布局证据，不是已读取活体第二相矩阵。

无界面split_command_and_exec(lib0x1fed4/1336)在20264构造VESTA_cmd_Read地址1e45c，202a4比较函数指针，202bc优先执行并从待执行数组移除所有Read；随后20358调用Scene::Build(NULL,-1)，2037c再执行余下保存等命令。GUI事件经execVSTXfile(main100019cb0/568)，100019e68解析、100019e80直接调用处理函数，所见路径无等价Build前置。旧Read函数仅临时Scene读取后插入Crystal指针/复制文件名，无显式Scene::Build调用。正常GUI文件打开OnFileReadImpl(main1000a7680/2280)则在a77f0 ReadFile后有a7d6c Scene::Build。三条执行路径差异已绑定指令与调用点。

Scene::Build(lib71a64/2096)在71b34调用BuildOrientMatrix(712a0/1836)；后者713a0 UnitCell::SetC2Ortho，并715d8/71600等写Crystal+150基底，71880/718a8处理其余矩阵。此前文档泛称“CoreInit”不是本次发现的原生符号，当前应使用这些已绑定函数名。构造单位阵、GUI-i缺少同样的前置Build、R90活体各坐标均Y.1及无界面Y1之间形成具体机制假设；尚无运行时矩阵内存和新holdout，完整导入根因不宣称已验证。

GUI解析器的实际令牌是flush，映射VESTA_cmd_gui_Flush(main100018dc0/92)：虚表槽310/e8/230各调用一次，三个目标尚未解析，不能只由名称宣称能修复数值；不把不存在于该解析器的refresh令牌当已验证命令。下一组应事前登记无界面-save先于-i的参数顺序holdout，以及GUI-i/-save在-flush前后两个新输出，观察是否恢复Y1；原件不修复、旧失败保留。

15项静态限定核对通过。完整证据913，原生raw893/not_loaded20，20条均通过原生按ID接口取回、与旧896逐项一致，明确派生完整快照；不是按上一轮4条遗漏推定。旧896/12问题头/78样本/原件257和90历史manifest/12627文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完，无产品实现。

[15项静态核对](../../output/vesta-study/20261108-113622/reviews/20261109-152400-s07-import-initialization/initialization-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-152400-s07-import-initialization/README.md)。

## S06 本卡归档、补证与独立预期（2026-11-10）

本卡复用45个选定科学研究定位器及其原始先验/报告，不为换任务卡全部重跑。完整逐功能已证规则/明确未知见[S06_DOMAIN_RULES.json](S06_DOMAIN_RULES.json)，206个领域相关ID/31组；这是唯一functions.json的研究索引，不另造产品覆盖门。31组均有未知/影响/下一探针，其余48个界面/文件/命令上下文ID仍保留全部最终范围。新[行为04](行为/04-科学计算与外部程序.md)首次依据真实材料创建；[S06_ORACLES](S06_ORACLES.md)区分物理先验、实现兼容模型、事后诊断和新输入留出。

新原件H/O核密度/模型核Patterson/模型电子Patterson：10Å P1、occ1/B0、resolution2Å；在三目标前冻结DC2.064fm、4.260096fm²、81e²及容差1e−4/2e−4/.002。三个6³网格的独立积分分别2.064003472222222、4.260089814814815、81.00019953703703；两Patterson偶对称残差0、原点最大。核density最小−.481814，核/电子Patterson最小−2.81868/−1.06596；负值未截零。**只验零频、有限值与列明不变量，全频谱/全体素科学仍未知**。CUBE标量按R33已证内部尺度，不宣称Gaussian原子单位互操作通过。详见[事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/density-oracle-before-target.json)、[逐案分析](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/density-independent-analysis.json)。

先读Deep函数：main OnCalcModelDensity10009ba54/460字节，lib Scene7666c/124、Crystal61588/668；61698取bc、616c8..d0分正负、61708..14按模式平方DC、61740建反射、61760间接核心尚未现场绑定、61774调用Fourier5f214。新增CalcGridNo88ebc/1456有对称/取整分支，**未从部分常量拼成完整格数政策**。新面夹角Scene76530/316字节在76598作两对象+48向量ddot，7659c acos，后乘180/pi；局部未见显式clamp，法向构建/GUI实际角范围与退化仍未知。符号检索Valence未返回匹配只是在该名称查询的有界负证，不宣称不存在键价功能。

VS102两种有界网格含零/负/整数scalar，原CUBE x外/z最快，输出ggrid x最快并补周期边界。4896/35904唯一点→5814/39270闭合点，共45084个值独立精确相等，首轮错误argv无输出保留，corrected-export-before-target仅纠正已核命令，原science oracle不变；未测RSS/OOM或最大容量。新离线复核R43四谐波288点max4.8437046766e−8，在原1e−7内，**没有新运行旧目标**。

旧R64原1e−6的3项失败/后续打印政策、R74固定左移模型否定、R77原模型失败、粉末首点、投影归一偏差、连续导数不符、P1磁矩丢失、GUI-i数值/Cancel崩溃、空handler、MADEL加载阻塞与所有S05科学单位/格式问题保持。来源旧论文未读范围/原件与同TEXT诊断副本/静态与运行均不合并。新问题unk_522695d0039b3283 high/open承接规则缺口，不把缺口解释成最终功能缩减。

本轮21个新原生ID，合计2036为逐文件哈希磁盘派生索引，**不是raw完整导出**；254稳定功能/102样本/84entry-only、24问题头，来源rev10/DBrev4。三GUI正常0，用时118.429/90.071/87.903秒，均soft240前、非强停、输入不改；四CLI原件正常255，首两无导出不记成功、后两按实际文件判。未改原件/上游/CLI、无Ghidra安装/全局设置变更/rebuild或产品实现。Codex自验与dsh独立复审分开；S06原四项在实际复审前仍未勾选。

S06验收记录：本卡实际dsh独立复审pass，研究卡四项完成；已证/失败/未知的科学边界保持，完整Study和产品实现未完成。见[裁定](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/review-result.json)。本轮停止。

## S07 TIDY 子进程完成合同（不扩大科学验收）

R56已证P1标准化/度量不变量直接复用；本轮新问题是没有单独捕获的子退出/失败。原附带x86_64 TIDY用文件名input.stin、cwd新工作目录、RIETAN原数据库目录，valid0输出最终三ATOM/3,5,7全90°/V105；NOT_A_SPACE_GROUP为0但输出Invalid space group symbol/Stop job/无ATOM，缺输入或空RIETAN目录缺Sets.daf均29/forrtl file not found。此处不是全群/最优γ或新科学公式通过；不倒填R56 GUI子返回。见[先验/实际内容](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/tidy-content-analysis.json)与[CLI](CLI.md)。
