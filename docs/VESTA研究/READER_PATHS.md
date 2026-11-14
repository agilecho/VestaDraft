# 文件读取路径与失败机制

2026-11-08，目标为锁定的VESTA 3.90.9a/arm64；原主程序/库保持原摘要。这里记录S04/S05继续取证的结果，**诊断已解决不等于原程序已修复、工程往返通过或Study完成**。[本轮证据](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/)与[原有格式结果](FORMATS.md)配套。

## 密度工程：错误的拼接结果被当作文件名

既有原件观察：生成的C1工程、绝对/短名引用及官方KOD工程重开无体数据；网格单开正常。这些结果保留，不再重复同类路径探针。

| 层次 | 本轮证据 | 能够证明的范围 |
|---|---|---|
| 原库静态 | `readVESTAfile` 0xa185c，在0xa391c调用`vx_append_all` 0xaaa60，再在0xa3934调用`read_density_data` 0xbb5b4；另读`IO::toAbsolutePath` 0x287f0 | 真实CA dossier/汇编。内置反编译存在寄存器失真、未展开间接跳转；不把伪代码中的xzr当空指针事实。拼接器可见reserve及三次append，没有清空目的串。 |
| 临时调试副本 | [逐事件轨迹](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/density-debug-events.jsonl)、[CA运行回执](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/native-debug-density.stdout) | 格式ID53；传入的204字节字符串是`绝对pgrid路径 + +1.000000 + 空格`重复两次，`fopen(...,"rb")`收到相同内容，密度函数返回-4。该具体字符串不是已存在网格路径。副本运行退出255，保存结构而无体数据。 |
| 未改原库的独立调用 | [原库拼接观察](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/append-original-library-observation.json)、[真实CA回执](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/native-original-append.stdout) | 自有最小调用器加载原始签名库，用已分析的0xaaa60函数做三种受控输入。非空目的串又作最后来源时，结果逐字等于副本轨迹中的204字节串；空目的串/独立来源得到`+1.000000 空格 绝对路径`。它是原库函数观察，不是重写的字符串实现。 |

由上述证据确认，这个已复现C1工程的失败点在读取端构造文件名：目的串原有路径未清空，又作为后续append的来源，最终把路径和系数拼成错误文件名。原网格、相对路径或CWD本身不解释此例失败。该诊断没有证明所有IMPORT_DENSITY变体相同，也未修改原件或实现产品修复。

原生问题`unk_f5b7e4d674823079`真实更新为revision4/resolved，并经`core-unknown-verify`得到valid=true。[解决回执](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/density-unknown-resolve.stdout)、[校验](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/density-unknown-verify.stdout)。resolved表示此失败机制已解释，既有工程往返失败不撤销。

## MXDORTO：格式识别、名称分支与内容/建模失败分开

[原库识别函数观察](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/format-export-native-observation.json)实际调用`check_format(const char*)`，仅处理名字，不打开文档：

| 名字 | 原库返回 |
|---|---|
| FILE07.DAT、FILE06.DAT、file07.dat、带目录FILE07.DAT、synthetic.dat | 26 |
| synthetic.pgrid / synthetic.ggrid | 53 / 52 |
| synthetic.vesta / synthetic.cif / synthetic.xyz | 0 / 3 / 23 |
| 无扩展名FILE07 / synthetic.unknown | -1 / -2 |

`readdata` 0x982f8对结构ID查表。CA保存的86字节表配合汇编中的16位读数/乘4/加0x98380，可导出43项分支地址；ID26进入0x991b4。内置反编译器没有展开这些分支，后续用已安装LLVM补读原arm64符号，只保留有关地址摘录，完整反汇编留临时目录。该分支对路径中的`06.dat/07.dat`做区分大小写的字节匹配，不能从识别函数对`.DAT`宽容推出后续读取同样宽容。

[同 inode 对照](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/mxd-case-comparison.json)使用VS-0033的相同字节、同目录/选项，仅改变输入路径的大小写；APFS上两种名字指向同一个文件：

| 实测输入路径 | 原件结果 |
|---|---|
| FILE07.DAT | 退出255，明确Failed to read，仍保存2272字节空工程，无STRUC块。 |
| file07.dat | 退出1，没有输出工程；不是读取通过。原始头部300原子/200O+100Si不能当作已验证目标内容。 |

临时调试副本用小写路径时，观察到`EXC_BAD_ACCESS → vector<int>::push_back → BuildCrystal2 → Scene::Build`，而非此前大写名字的立即读取失败。[副本栈回执](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/native-debug-mxd-exit.stdout)。这是副本的具体栈，原件回执只记录退出1/无产物，不能擅自把该栈标为原件直接捕获。

原生问题`unk_039a493a20033dec`接续revision3/investigating；识别ID已回答，字段解释及BuildCrystal2失效数据/索引仍待核，不能把文件改小写写成兼容性修复或输入有效。[更新](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/mxd-unknown-followup.stdout)。

## 工具、签名与调用器边界

原件启用hardened runtime且没有get-task-allow。本轮未附加调试原件、重签原件或更改系统调试策略。临时副本在`/tmp/vesta-debug-20261108-162528/VESTA.app`，仅其主程序改为自签名调试配置；[派生身份](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/debug-copy-derivation.json)核两架构全部TEXT区节不变，[库/原件复核](../../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/debug-copy-library-validation.json)核库完整字节相同、原包257个普通文件不变。副本不是基线替换。

LLDB密度回调只读寄存器及有限内存，不执行目标表达式；所有目标进程由有界CA walkthrough拥有并清理。LLDB的系统日志很多，终端/输出有边界，不把截取终端视为全部执行记录；关键密度事件另即时写JSONL。调试进程退出/被杀与原件正常运行返回码分别记录。

两个Python直接载库尝试在进入识别函数前因`@executable_path`依赖libpng失败，原回执保留。随后自有arm64 C调用器建立所需Frameworks目录链接并成功；C++调用器通过已知导出地址校验库布局，再调用已分析的内部拼接器。调用器是研究工具，未复用原代码为产品实现；不新增全局VS样本，仍37个。

锁定CA的`apple_calls_observe`帮助虽说有参数寄存器，实际所读实现只保存命中函数名，也未在回调中读取参数；本轮没有用它伪造文件路径轨迹。`native-bytes-show`的参数是length，早先误用count的失败回执保留。LLVM范围/参数失败也保留，成功的符号反汇编与失败不混合。

另读到`ReadBVParm` 0xd6bc引用`bvparm2013.cif`，原包只见2020文件；当前库直接callers和主程序导入查询均未建立调用关系。它可能是未使用路径，本轮不宣称价键功能缺资源、固定B或采用某版参数，继续按来源问题与后续实际入口核对。

目前真实原生状态：104条evidence、9问题（2 resolved/2 investigating/5 open），来源问题仍revision8。254功能/37样本保留，S02.3及S04/S05剩余步骤不因此勾选。

## R10：MXDORTO坐标字段与越界机制已解释

本节接续上轮revision3的待查项，当前问题为revision4/resolved，真实[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/mxd-unknown-verify.stdout)valid=true。[原函数逐事件](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/mxd-parse-events.jsonl)、[BuildCrystal2 dossier](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/build-crystal-study.stdout)、[交叉核对](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/mxd-cause-crosscheck.json)分别保存原始观察、静态代码及算术推导。

原例首行三坐标各占9字符；07.dat分支在0xa0184以`%10c%10c%10c%10c%10c%10c`分割，再以`%lf`转换。临时副本的库完整字节仍与原库相同，未改输入/代码，实际捕获：

| 坐标 | 第一行实际转换文本 | 实际转换结果 |
|---|---|---|
| x | ` .4943653 ` | 0.4943653 |
| y | `.5406929 .` | 0.5406929 |
| z | `7894748 5.` | 7894748.0 |

首3行z依次7894748、1552740、9973490，Build入口容器实际有300个位点。BuildCrystal2 0x165f0–0x16608的整数折回使用16位平移：首z留下7864320。0x167c4–0x16800用7×7×7分区形成32位索引，无符号2697461784/有符号-1597505512，以24字节步长计算出的地址逐字等于捕获的vector目标地址；push_back访问该地址+8时EXC_BAD_ACCESS。该算术解释明确为对观察汇编与地址的推导，现场值见JSONL。

[只补空格的诊断变体](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/width-variant-derivation.json)把300行的前三个9字符字段各左补1空格至10字符；900个数值token及其余输入字节不变，原VS-0033未改，不新增VS编号。副本对照首z恢复0.7894748，首分区索引269落在343个容器内，继续后保存工程。

随后未修改原件的受控执行得到：

| 原件输入/执行 | 原件结果 |
|---|---|
| 原始小写字节，stdout/stderr写入本轮文件 | [实际子进程-11/SIGSEGV、无产物](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/startup2-raw-observation.json)，不把此前CA归一化1冒充信号原值 |
| 只补空格的小写变体，同样文件输出方式 | [退出255并实际保存](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/startup2-width10-observation.json)；[300位点/200O+100Si、900坐标比较](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/width-output-site-comparison.json)，最大误差5.00000000014e-7，在6位小数序列化容差5.0000001e-7内 |

原件成功产物的Output报377 atoms/421 bonds/99 polyhedra，是显示/边界扩展计数，不等于300个独立位点。这次只验数值token保留及序列化的坐标/元素/位点数量，没有宣称所有键/多面体/物理量正确，也没有接受完整MXD规范或产品修复。大写原样本的名称分支失败仍保留。

## 本轮启动状态和工具失败边界

原件前几次安静超时，文件输出重定向也未改善。[原件系统采样](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/sample-original-stack.txt)明确停在`NSPersistentUIRestorer promptToIgnorePersistentStateWithCrashHistory → NSAlert runModal`，尚未进入OnInit的数据读取。因此不能推为补空格变体失败、输出管道阻塞或CPU计算慢。后续startup2原件自行不再被该弹窗阻塞，转变过程未被观察；本轮未点击弹窗或修改全局恢复/安全设置。CUA仅读应用清单，看到VESTA未运行，已completion_summary，没有自动重启app。

CLI两次错误UI命令名、一次检索mode=contains、一次漏单次allow-run均保留失败。实际命令表查询后确认runtime-ui-walkthrough-record，但未调用动作；授权后的runtime-ui-read只返回depth0节点，不能证明弹窗文本/布局。首版调试回调在读无效地址时未处理None，发生callback_error；新脚本保留旧事件并修正为记录读取失败，第二次轨迹完整捕获。原件重定向前两次supervisor超时异常同样保留，不伪造为已完成报告。

当前118条原生证据、9问题（3 resolved/1 investigating/5 open）；来源问题revision8不变。254功能、37样本、10已勾选/19未勾选；完整Study仍未完成。


## 返回255的后续解释

R10受控变体有效保存却返回255的外层机制现已在[CLI_COMPLETION](CLI_COMPLETION.md)解释：正常nogui路径返回预设255，没有转发命令结果。原样本信号11、读取失败/空工程、有效保存仍是不同结果，不能合并成通过或全部失败。

R53选定新扩展名lned被check_format判-2；当前三字节xed分支判45。ReadFile0x981f0/98234对负type回-2，cmdRead0x1e45c失败回-1且不追加objects；后续CLI Save不防空数组，原件PID60060 postmortem绑定NULL解引用。此为原库查询+新/缓存静态+原已发生崩溃证据，未再运行reader/crash。见ALGORITHMS R53。

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


## R85 CLI全局样式与完整证据快照（2026-11-09）

新target前登记VECTS7、.125及7→.125双文件append三案；原版均正常255/完整watch，输出全局VECTS1、相数1/1/2，每相矢量(1,0,0)/Polar1/site1保留。静态cmdRead临时Scene读取后仅转移Crystal列表及文件名，未转移Scene全局XStyle，解释所测比例重置；未读取临时内存或验证所有样式字段/格式。[15项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/style-crosscheck.json)。

锁定工具首次触发64MiB加载预算：原生export实际869/not_loaded2，磁盘871条均存在。两旧记录经原生按ID接口完整读取，与旧包/磁盘一致；[原始部分导出](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/native-evidence-final.json)保留，不改成全量；[完整核验派生快照871](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/verified-complete-evidence.json)明确为原生导出+原生ID解析的核验合并，[来源与身份](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/complete-evidence-provenance.json)、[预算重放](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/method-load-budget-audit.json)可审。不得将wrapper0或导出记录数当完整性；后续须核not_loaded并按需补齐。

新增7至871原生来源记录，旧864/12头/77样本/原件257与84历史manifest/11713文件不变；raw导出869，完整核验871，两者须区分。未改工具二进制/锁定提交/工程历史记录，无产品实现。254功能/32权威、10完成19未完成，Codex自验，dsh独立复审/S08待。一般样式、非正交矢量及完整Study继续。


## R109 体数据重开路径与打开方式控制（2026-11-09）

R109不重做已有函数study，复用readVESTAfile0xa185c（旧R1/4905指令）、check_format0x8d83c、IO::toAbsolutePath0x287f0、read_density_data0xbb5b4与readCUBEfile0xbeb7c五份已存在原生记录。IMPORT_DENSITY分支实际先在a38f4做格式识别、a3904转绝对路径、a391c拼路径/倍率描述符，a3934调用密度读取器。原生u8地址4fa6b3=32，拼接分隔符为空格；不可凭猜测写成换行。已有路径函数对前导/提前返回，不是原生调用追踪。

现有read_density_data返回493条指令/size1972、body_ranges不连续，仍带一个未解析间接跳转；虽然函数级coverage显示1/1完整，不能据此称整个Cube分支已覆盖。check_format在8dc04/8dc08比较little-endian65627563（cube）、8dc18转8e090返回11。密度分发bbb40减10，bbb54基PC=bbb64，bbb58读u16；本轮原生读取50fd12=113，故Cube目标bbb64+113×4=bbd28。按事前上限12指令/遇首个bl停止的计划，五次原生instruction-show取得bbd28..bbd38，设置栈参数后调用readCUBEfilebeb7c；这五个地址确实不在旧493条body中、原生报告function=None。已补到首个调用，未声称完整read_density_data或动态调用/文件访问链都已恢复，原文件和工具未修补。

随后事前登记两个有区分度、只读的恢复预测。绝对引用副本只把R98项目IMPORT_DENSITY中的相对文件名替换为拥有目录中同哈希Cube的绝对路径，其余字节精确保持；原生确认absolute-density.vesta标题/4Å晶胞，实际仍0polygons/0vertices，正16/16恢复预测false。另开空白原版实例，通过真正GUIFileOpen选取与R98整字节相同的相对引用项目和相邻Cube；原生先证空白窗口VESTA，后证manual_open-density.vesta标题/晶胞，结果同为0/0，第二个正恢复预测false。与R108同Cube直接打开16/16的既有控制对照，仅在这些成对上下文中，相对路径写法与启动argv打开路线都不是观察失败的必要条件；唯一读取失败原因仍未隔离，不能推广为所有IMPORT_DENSITY或所有VESTA项目失效。未开Properties、未编辑/保存/发送IPC，更未覆盖冻结旧失败。

两个拥有PID1595/1715均正常0，25.277/170.985秒，后者120目标超50.985秒、两者均<180；所有功能native读取结束在180前，退出后原PID守卫跳过应用查询。手动打开时typeText和paste后的GoToPath实际值丢失下划线，Return仍停在路径sheet、未打开错误文件；最终setValue原样填入后核对正确路径，才点击Open并取得指定项目原生后像。输入问题及120目标失败留存，未声称其唯一原因。两输入哈希/mtime的0.1秒抽样保持，规范源及两资源哈希/mtime保持，四个拥有偏好常规文件归档。

34项限定核对通过，两个真实只读控制/3次原生UI；bothPositiveRestorePredictionsPass=false、all120GoalPass=false，bothRegisteredControlsExecuted/all180GoalPass=true。新增10原生ID至1269，实际raw1126/not_loaded143，143次原生按ID补读组成显式派生完整快照，raw不改。旧1259记录/12问题头、原件257、108历史manifest/21293文件及78稳定样本保持。只扩展VF17，其余253条保持；入口级待核112不变，254功能/32权威，计划10完成19未完成。Codex自验；dsh/S08及完整Study未完成，无产品实现。

[34项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-212400-s05-import-density-readback/import-density-crosscheck.json) / [事前控制](../../output/vesta-study/20261108-113622/reviews/20261109-212400-s05-import-density-readback/import-density-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-212400-s05-import-density-readback/README.md)。
