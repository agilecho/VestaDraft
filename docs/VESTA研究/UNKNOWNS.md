# VESTA Study 未解决问题

当前任务对齐（2026-11-10）：S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 当前 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open，即14个未解决；来源 revision10、数据库 revision4 保持。本次状态对齐没有新增目标证据或产品代码。 既有结果与失败边界保持；从 [HANDOFF](HANDOFF.md) 接续。

权威状态：`output/vesta-study/20261108-113622/vesta.cany`，读取方法见 [METHOD.md](METHOD.md)。下列是阅读摘要，更新以真实回执为准。

| 原生 ID / revision | 状态 | 问题和影响 | 下一探针 | 证据 |
|---|---|---|---|---|
| `unk_42e315593eabb9ce` / 2 | high / resolved | About 实测 3.90.9a、64-bit、Aug 8 2026；2022 手册须与新版本逐项核差 | 继续 S04 版本差异盘点 | [更新](../../output/vesta-study/20261108-113622/s04/unknown-version-resolve.stdout)、[验证](../../output/vesta-study/20261108-113622/s04/unknown-version-verify.stdout) |
| `unk_3ee7a6c51afa307a` / 4 | high / investigating | 默认HTTP空白已解释；HTTPS COD查询/导入/保存、Back/Forward/Reload、404/dirty保留已验证；Stop实际取消仍未证明 | 读[DATABASE](DATABASE.md)，继续实际取消/认证SSL超时/非CIF及其它可能路由；不凭Menu名假定提供者选择 | [新头](../../output/vesta-study/20261108-113622/reviews/20261109-071658-s04-database-controls/controls-unknown-update.stdout)、[实际核对](../../output/vesta-study/20261108-113622/reviews/20261109-071658-s04-database-controls/database-controls-crosscheck.json) |
| `unk_af18a557833fcedb` / 3 | medium / resolved | 默认first取得66×20附属窗口；当次同PID元数据绑定主窗口ID，显式原生截图860×728/实际视觉确认 | 读[WINDOW_CAPTURE](WINDOW_CAPTURE.md)；以后用当次ID，AX仍进程范围；全视觉/模态/缩放覆盖另验 | [解决](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-unknown-resolve.stdout)、[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-unknown-verify.stdout)、[显式主窗口图](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-main-explicit.png) |
| `unk_f5b7e4d674823079` / 4 | high / resolved | 密度引用诊断闭合：读取端非空目的串同时作append来源，构造重复的路径/系数字符串，fopen失败/返回-4；不是工程往返通过或程序已修复 | 读[READER_PATHS](READER_PATHS.md)，保留原件往返失败；不再重复路径变体，继续其它格式/状态契约 | [解决](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/density-unknown-resolve.stdout)、[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/density-unknown-verify.stdout)、[原库对照](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/native-original-append.stdout) |
| `unk_b84f44506e209f4a` / 2 | high / resolved | nogui返回机制闭合：parse返回GUI启动标志0，OnInit/CallOnInit false，wxEntry返回对象中预设255；不转发命令结果。有效/失败同255的原件记录保留 | 读[CLI_COMPLETION](CLI_COMPLETION.md)，逐命令联合产物/内容/错误/信号判定；不把此题解决当S07完成 | [解决](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/cli-unknown-resolve.stdout)、[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/cli-unknown-verify.stdout)、[返回轨迹](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/cli-return-events.jsonl) |
| `unk_a1948891885cdc8d` / 2 | high / resolved | 两个既有接受机制已解释：pgrid短读510次仍复用标量填数据；CIF -2被词法转换并原样进入场景/矩阵/保存，未拒绝正长度条件违例 | 读[INPUT_VALIDATION](INPUT_VALIDATION.md)；不重跑旧矩阵，不当科学有效性；其它约束/溢出/资源/dirty仍待S05覆盖 | [解决](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/validation-unknown-resolve.stdout)、[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/validation-unknown-verify.stdout)、[短读核对](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/grid-short-read-crosscheck.json)、[负晶胞核对](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/negative-cell-crosscheck.json) |
| `unk_039a493a20033dec` / 4 | medium / resolved | VS-0033失败诊断闭合：大写路径未进入07.dat分支；小写按10字符切9字符坐标，首z误读7894748，短整型折回/32位索引越界。原件小写SIGSEGV；补空格变体保存300位点 | 保留原件失败；见[READER_PATHS](READER_PATHS.md)，不把诊断变体当规范/全格式通过；继续GUI取向和其它输入契约 | [解决](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/mxd-unknown-resolve.stdout)、[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/mxd-unknown-verify.stdout)、[因果核对](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/mxd-cause-crosscheck.json) |
| `unk_239d524d3a4b9d25` / 2 | high / resolved | 当前MADEL加载阻塞已定位：原包x86_64程序引用缺失的绝对GNU Fortran库，原件直接执行SIGABRT(-6)/无.madel；GUI忽略wxExecute返回且无文件时静默清理 | 读[MADEL_COMPLETION](MADEL_COMPLETION.md)；不重跑旧GUI输入，不当零能量通过，不安装/修补原环境；继续领域数值与完整外部执行契约 | [解决](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-unknown-resolve.stdout)、[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-unknown-verify.stdout)、[真实子进程](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-original-observation.json) |
| `unk_e1a118c694cc99ea` / 10 | high / investigating | R134的22项相关来源经dsh复核通过；专项收尾验收已过、S02三项完成；完整格式/科学与目标嵌入版本另待S05/S06，旧question保留创建时历史文字 | 保留MOLDA单位、商业CNX/二进制变体、未读旧论文/全表和目标绑定；来源阅读不作原件科学通过 | [更新](../../output/vesta-study/20261108-113622/reviews/20261110-041200-s02-public-professional-sources/source-unknown-update.stdout)、[读回](../../output/vesta-study/20261108-113622/reviews/20261110-041200-s02-public-professional-sources/source-unknowns-after.stdout)、[dsh](../../output/vesta-study/20261108-113622/reviews/20261110-041200-s02-public-professional-sources/dsh-final-source-review.stdout) |
| `unk_3f5d08b5dc777b37` / 3 | high / resolved | 所选CUBE原因查明：坐标换bohr、scalar保持内部值；原版读回scalar也不换算。Gaussian电子密度单位兼容仍未通过 | [FORMATS](FORMATS.md)；保留14/94.476674差异，不重跑已答往返；其它Cube变量/消费者继续 | [解决](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/cube-unit-unknown-resolve.stdout)、[native校验](../../output/vesta-study/20261108-113622/reviews/20261109-003405-s05-cube-units/cube-unit-unknown-verify.stdout) |

## R22 Raster产物验证

新问题`unk_ff152c8e4008160b` revision2 investigating，范围是JPEG2000无文件/RGB无效/TIFF透明无效的原件写出机制与现场路径绑定。原件静态stub、短头及RGB自定义路径和实物一致，未完成现场调用证明；见[IMAGE_EXPORT](IMAGE_EXPORT.md)。

## R23 Raster所选诊断关闭

`unk_ff152c8e4008160b` revision3 resolved：空JP2转换返回被忽略，SGI自定义短头/布局，TIFF自定义RGB写出丢alpha，真实同TEXT副本调用轨迹与原件实物对应。仅指所选原因查明，三项功能仍失败；详见[IMAGE_EXPORT](IMAGE_EXPORT.md)。

当前10个原生问题：7resolved、3investigating、0open，全部头不变。248证据（原241保持新增7）；254功能/54样本，新VS0054规则八面体体积控制，旧53记录保持。完整畸变/ECN公式、非规则/退化、组成与其它领域仍待；Utilities/ORFFE没有测试。完整Study未通过，独立复审归dsh未开始。

R29新增4至252证据，10头逐项不变，7resolved3investigating。手册五项公式与轴向畸变样本按显示精度一致，不作Robinson/Hoppe全文验读、来源问题关闭或S06全覆盖；[算法补证](ALGORITHMS.md)。

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

## S04 入口/焦点复核（2026-11-10）

12原生头保持，来源rev10、数据库rev4继续investigating，不为措辞修改原生记录。本轮新增10evidence至1650磁盘派生索引（非CLI raw）；剩余入口/焦点/文档候选已挂254稳定功能父ID内pendingEntries及[20组待办](../../output/vesta-study/20261108-113622/reviews/20261110-055459-s04-entry-interaction-audit/remaining-gaps.json)，并非12问题计数覆盖全部未答功能。S04.2仍未勾选，S04.3/.4接受初步规格/查漏，不升级科学或产品通过。

## S04触发补证063443当前记录

本轮仅接续S04触发补证：新增90条原生至1740磁盘派生索引；254稳定ID/78样本，90条入口级待核。S04.2仍partial-unchecked，S04.3/.4既有接受保持；13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 [本轮矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-063443-s04-trigger-completion/S04-MATRIX.md)保留真实边界。原件根目录269个is_file路径（263普通+6symlink）不变；VESTA.app257是app内非symlink普通文件数，不是整个根目录计数。12原生头保持，90新增证据不冒称raw。

## S04 Objects/焦点072741当前记录

本轮继续S04 Objects/焦点：新增45条原生至1785磁盘派生索引，254稳定ID/78样本/12头保持，87条入口级待核。S04.2保持partial-unchecked，S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 [矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-072741-s04-objects-focus/S04-MATRIX.md)区分CUA转录、native端点、文件diff与static机制。原件根269个is_file路径（263普通+6symlink），app内257非symlink普通文件口径保持；CLI/12头/78样本/旧原生字节不改，无配置Settings或凭据读取，无新增引擎/重建/Check/builder。

## alpha路由075808

本轮继续S04 alpha事件路由：新增9条原生至1794磁盘派生索引，254稳定ID/78样本/12头/87入口级待核保持。S04.2保持partial-unchecked；S04.3/.4既有接受保持，Study13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 原生头保持，不为措辞更新；新分析为Deep静态函数/一次新焦点条件，仅同格singleclick区别，不重复已有科学或成功探针。[证据](../../output/vesta-study/20261108-113622/reviews/20261110-075808-s04-alpha-dispatch/routing-report.json)。

## alpha运行时081848

本轮S04 alpha入口已有原件阳性：同PID激活后28×128浮窗与AXSlider192；诊断副本实测点击门槛/显示链。新增32原生至1826派生磁盘索引，254ID/78样本/12头/87entry-only保持。dsh复核pass、无硬阻断；S04.2按入口/条件登记范围接受，S04整卡收尾，Study14完成/15未完成。未进入S05或产品实现。 原件同PID：inactive时C右半点击后无屏上popup；访达Open既有应用激活后出现28×128/layer101浮窗，native AXSlider=192；再次CUA主窗观察后inactive且浮窗隐藏。诊断副本另实测mode2/column2/adjustedX25>16及alpha/Popup/Show链，不能冒充原件回调值。 原生unknown头未更新；截图not_found/首wrapper timeout/原件SIGTERM清理均保留。[证据](../../output/vesta-study/20261108-113622/reviews/20261110-081848-s04-alpha-runtime/runtime-report.json)。

当前dsh复核裁定：本轮S04 alpha入口已有原件阳性：同PID激活后28×128浮窗与AXSlider192；诊断副本实测点击门槛/显示链。新增32原生至1826派生磁盘索引，254ID/78样本/12头/87entry-only保持。dsh复核pass、无硬阻断；S04.2按入口/条件登记范围接受，S04整卡收尾，Study14完成/15未完成。未进入S05或产品实现。 只接受登记范围，保留全部失败与科学/平台未验。[实际复审](../../output/vesta-study/20261108-113622/reviews/20261110-081848-s04-alpha-runtime/review-result.json)。

## S05 新实质差异登记（2026-11-10）

本轮旧12头完整字节不变（来源rev10、数据库rev4继续investigating），新增3个真实open头；当前15头=10resolved/2investigating/3open。名称/原报告false和三项下一探针均不冒称原因闭合或程序修复。

| 原生ID / revision | 状态 | 实际范围与下一探针 | 回执 |
|---|---|---|---|
| `unk_87375544dae1b5e6` / 1 | high / open | PDB温度列实际写U而标准要求B；自读回掩盖差异。补独立PDB B-positive/ANISOU与reader/writer参数模式绑定。 | [新增](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/unknown-pdb-add.stdout) |
| `unk_bc1524075a819b53` / 1 | high / open | 原官方AMCSD KA(m)被存为Ka；坐标占位保持。补原作者标签解释与K/K1/KA(m)区别样本，未把推测当完整机制。此头evidence_for为空，实际既有产物/独立比较在question具名引用；不虚造native目标回执。 | [新增](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/unknown-amc-add.stdout) |
| `unk_f1ffb0ab9d8ee9c2` / 1 | medium / open | m3d对VS5复读最大差1.0147520750761032、VS79整数控制精确。绑定原编码/允许损失/负值分数控制，不将目标自读回当独立codec。 | [新增](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/unknown-m3d-add.stdout) |

[逐格式未答项](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/input-format-matrix.json)与[输出未答项](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/output-format-matrix.json)保留全范围；17行缺具名稳定代表不记不支持，全部GUI读/显/编辑/保留及剩余资源/选项继续S05。

## S05 本次继续执行结果（dsh复审通过）

新增实际unknown及m3d真实revision2（无手写native头）：

| ID | revision/状态 | 问题 |
|---|---|---|
| unk_11b1b64c9b62dc3d | 1/open | GUI VASP export from an atom-free VS79 periodic4/5/6 grid writes blank species/count lines, Direct, grid2/3/4 and density values. Fresh target read saves empty cell1/1/1 and exits1 before producing the expected density grid. Does atom-free VASP output violate the producer header contract or require an associated structure? |
| unk_14217952ef1b0bb6 | 1/open | Selected actual STRUCTURE TIDY .sto output from the earlier Niggli/orthogonal control is rejected by GUI Open despite the .sto classifier. Is the target limited to another TIDY output dialect? Editing was blocked; this must remain a failed selected output/input contract. |
| unk_43e6146944e74705 | 1/open | Selected official Elk writegeom syntax with species C.in/O.in reads correct cell/coordinates but garbage species/labels; GUI project saves blank strings, and fresh .vesta read shifts coordinates and creates wrong strings. What input-species/serialization contract prevents this source-field and project loss? |
| unk_a71701d6a425e081 | 1/open | Byte-exact author VENUS KTiLiO.den legacy text is rejected by target GUI/reader. A separate CRCRLF/comment-normalized diagnostic throws vector length_error. Which author DEN dialect or parser constraint is required? Independent10091 indexed densities and cell metadata exist; no restored grid was observed. |
| unk_cc2918c874342362 | 1/open | Actual volume STL export has144 binary facets with finite vertices but16 nonfinite normals (NaN). Saved level is1, not the failed intended160.5 setup. Independent raw-byte audit also distinguishes136 boundary-coplanar triangles and8 non-boundary triangles; non-boundary vertices match level1. What degeneracy/capping contract produces invalid normals? |
| unk_d5dfb7e0b2c0dda0 | 1/open | Paired GAMESS C1 .inp + .mmp source contains C and O; target retains only O. Grid3/4/5 with spacing2,5/3,1.5 produces cell6,6.666667,7.5, versus source endpoint spans4,5,6. Is first C skipped by a symmetry-card assumption, and is n*spacing an intentional cell convention? Standalone .inp rejection is separately recorded. |
| unk_f1ffb0ab9d8ee9c2 | 2/investigating | Locked target default m3d export of VS5 and new-instance decoding to independently parsed ggrid differ from source by maxAbs1.0147520750761032; asymmetric positive integer VS79 is exact. Is this declared lossy precision, negative-value handling or another codec limitation? Full independent m3d layout/precision contract remains unverified. |
| unk_ffd5079cd743b4b7 | 1/open | Source-defined ASSE0.5 cell4/5/6 and C(.125,.2,.3), O(.625,.4,.5) reads as C(.2,.3,0), O(.4,.5,0), despite correct cell. Is this target atom-column loss or a legacy ASSE dialect requirement? Correct .asse suffix and GUI edit/save retained; initial .inp misdispatch is a separate setup failure. |

全部22头及evidence关联见[unknown-heads](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/unknown-heads.json)。来源rev10/数据库rev4及其它原头保持，仅m3d因新证据1→2/investigating。Resolved旧读取/单位/图片问题只指机制解释，原失败持续保留。

### 既有结构-only CUBE输出追加核对

结构-only CUBE追加核对：已有337字节GUI实物含两原子、grid0/0/0、9个inf/nan步进向量，无密度payload；实际fresh文件有非有限空CRYSTAL和坐标正确的MOLECULE，不能认证有效体数据输出。独立原字节报告structure-cube-degeneracy-analysis.json；实际新unknown unk_dc25854c16d1dde6。分析层初版Python NaN JSON保留analysis-initial，当前JSON显式字符串NaN/+Infinity，不改原件或原始文件。研究范围不变，当前23头=10resolved/3investigating/10open。

## S06 科学规则缺口原生登记

新增 `unk_522695d0039b3283` revision1/high/open，真实[创建回执](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/science-unknown-add.stdout)与[读回](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/unknown-heads.json)。它跟踪31组规则中明确的科学未知：多相R/t、一般对称/退化、键价/ADP几何、等值面/映射/运算、精确投影、Nyquist/网格、反常表和资源边界；每组具体影响和下一探针在[S06_DOMAIN_RULES](S06_DOMAIN_RULES.json)，不是只写笼统“待核”。新核/Patterson DC通过不关闭全谱问题；MADEL依赖阻塞未修复。

当前24头：10resolved/3investigating/11open。原23头实质内容和revision保持，来源rev10/数据库rev4不动；不为措辞对齐修改既有原生头。resolved的旧缺陷仍按原缺陷保留。所有影响模块的缺口先补研究，独立模块可继续，最终全部功能不缩减。

S06验收记录：本卡实际dsh独立复审pass，研究卡四项完成；已证/失败/未知的科学边界保持，完整Study和产品实现未完成。见[裁定](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/review-result.json)。本轮停止。

## S07 已知运行边界与原生问题头

本轮读取当前24头，逐项与S06相同，来源revision10/数据库revision4保持；不为措辞修订原生头。TIDY新增失败合同已有明确观测，不虚构一个尚未回答的问题。RIETAN运行件/绘图器、ORFFE生产者、真正无图形会话和Linux/x86是明确未验证能力，原因/影响/下一探针见[外部合同](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/external-program-contracts.json)和[完成边界](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/completion-boundaries.json)。MADEL resolved仍只诊断加载阻塞；科学unk_522695d0039b3283、TIDY-reader unk_14217952ef1b0bb6等未被卡状态改为通过。

## S08 最新24头与设计影响
本轮真实 `core-unknowns-show` 与S07逐头相同；10个resolved另逐一真实 `core-unknown-verify` 均valid=true/problems=[]，仅验证resolution及所引证据存在/完整，不是独立科学验收。14个未解决头逐项仍缺实质证据，本轮不为文案改revision。原生question/probes保留历史措辞，当前设计影响如下；详见[全部裁定与真实回执](../../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/unknown-dispositions.json)。
| 原生ID / revision / 状态 | 当前设计影响 | 下一探针（未运行） |
|---|---|---|
| `unk_11b1b64c9b62dc3d` / 1 / open | 结构与体数据的可导出域必须分开；零原子header/再导入失败仍在。 | 先读VS79的已存空species/count header与CHGCAR来源；冻结一个合法结构+同网格作为候选对照，再核本版原子空缺约束/拒绝后旧场保持。 |
| `unk_14217952ef1b0bb6` / 1 / open | TIDY实际输出不能保证File/Open可摄入，影响codec方言和回链。 | 对既有R56 .sto逐字段绑定readSTOfile，先静态核所需header/记录类型，再预登记最小差异控制；S07的valid子退出0未回答reader方言，先复用不重跑外部程序。 |
| `unk_3ee7a6c51afa307a` / 4 / investigating | HTTPS导入等已证；Stop实际取消、认证/SSL/超时及非CIF路由仍未知。 | 新拥有页面发可控慢响应，Stop前后记录请求状态/回调/旧工程与保存；继续仅公共无凭据请求，原HTTP空白和HTTPS阳性不重跑。 |
| `unk_43e6146944e74705` / 1 / open | 正确cell/坐标并不接受species/label或.vesta持久化；模型需明确缺失字段。 | 读VS83 GEOMETRY.OUT与已存首工程/重开工程；绑定Elk物种token解析和writer空字符串边界，冻结C.in/O.in及显式species对照、全部字段后再新探针。 |
| `unk_522695d0039b3283` / 1 / open | 206领域ID/31规则组均未全部科学通过；影响P1模型、精度、依赖和兼容策略。 | 按31组的unknown/nextProbe先选择影响当前设计的最小新输入：冻结oracle/单位/容差/资源期限；保留R64/R74/R77/首点/磁矩反例。DC、字面量和自读回不代替全谱。 |
| `unk_87375544dae1b5e6` / 1 / open | PDB写tempFactor .01/.02与B=8π²U规范冲突，自读回掩盖错误。 | 读取已有VS70输入/GUI导出/独立PDB33列解析；静态绑定writer/parser及ANISOU；新非零/交叉U对照在运行前固定B与列宽预期，原失败保持。 |
| `unk_a71701d6a425e081` / 1 / open | 作者原字节DEN拒绝；正规化诊断length_error不是格式兼容。 | 先按已读VENUS作者语法、既有10091标量与readDEN分支逐字段核header/CRCRLF，事前冻结最小差异样本；不重复原失败字节。 |
| `unk_bc1524075a819b53` / 1 / open | KA(m)被判Ka；坐标/占位不变不能接受元素化学语义。 | 复用VS22源/旧导出与独立期待，定位label→species逻辑；新显式K/Ca、混合占位及split控制先固定字段，核读取/显示/保存不同层。 |
| `unk_cc2918c874342362` / 1 / open | 144面中16 NaN normals，level实为1；mesh几何/退化边界是P1前置。 | 用已存STL与level1工程追退化/cap生成路径；新2³/3³解析平面预计算交点/法向/面积，保持原NaN和失败level160.5设置记录。 |
| `unk_d5dfb7e0b2c0dda0` / 1 / open | 配套inp+mmp丢C且采用n*spacing；不能假定端点跨度科学正确。 | 绑定已存两文件中的对称卡/首原子行与grid起止；预登记0/1/2 symmetry卡、不同轴计数的最小新输入，分别检验species与cell，不把单独inp拒绝混为同一机制。 |
| `unk_dc25854c16d1dde6` / 1 / open | 0/0/0 voxel及inf/nan基矢是无效体元数据；两原子位置正确不洗白。 | 复用VS70结构CUBE实物与fresh读回；绑定没有grid时的writer路径；新结构+合法grid对照只补条件门，先确定有限header预期，不实现自动修复。 |
| `unk_e1a118c694cc99ea` / 10 / investigating | revision10的22来源已按范围接受，旧question中的S02未完成是历史；未读论文/全表、MOLDA单位/商业CNX/目标内嵌版本仍不够定科学实现。 | 先读R134来源台账与各专业页明确剩余项；只补设计实际消费的原始章节/版本或目标参数表绑定，不重试同样受限下载，不以当前官网版本推定目标。 |
| `unk_f1ffb0ab9d8ee9c2` / 2 / investigating | VS5 maxAbs1.014752075…与整数VS79精确两事实并存；codec/精度域未定。 | 先读已有m3d原字节、独立ggrid和先验；定位量化/符号范围，再冻结小非整数正/负/零/极值控制与容差，不从自读回臆造格式规范。 |
| `unk_ffd5079cd743b4b7` / 1 / open | VS81的晶胞正确但C/O坐标移列并补0；不能接受完整ASSE或GUI编辑持久化。 | 读取已有作者ASSE0.5样本与原件输出，绑定实际readASSE列扫描；先冻结各列唯一哨兵和不同空白分隔的最小新输入，单独核后缀分派、坐标和失败后旧状态。 |
