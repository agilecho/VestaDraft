# 原生截图的窗口选择

当前截图问题`unk_af18a557833fcedb` **revision3/resolved**。锁定CLI已有window-id；默认截图首个同PID零层正尺寸窗口可为66×20附属窗口。R30用当次只读同PID窗口元数据取得真实主窗口ID，显式截图并解码视觉确认通过。只解决截图归属，完整布局/渲染等价与其它窗口覆盖仍待。

## 当次正控制

原版拥有PID39830/独立偏好，既有C1工程副本window-working.vesta。默认capture实际60844、66×20、scaled=false；AX有Window/WindowSharingSessionButton及主窗口，说明AX与截图范围不同。[默认回执](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-default-ui.stdout)。[只读诊断源码](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/list_owned_windows.swift)调用ScreenCaptureKit取屏上窗口，只打印同PID元数据，不注入事件/激活窗口/取图/请求授权；既有ScreenCapture权限只做preflight。编译产物及源码由[真实native运行](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/native-owned-window-list.stdout)监视，未修改锁定CLI、原件或全局设置。[派生列表](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/owned-window-metadata.json)给出60844/Window/66×20与60826/window-working.vesta - VESTA/860×728；终端换行提取过程见[笔记](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/diagnostic-extraction-notes.md)。

向原CLI传pid39830/window_id60826，返回同ID、PNG860×728、scaled=false，[显式原生回执](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-main-explicit-ui.stdout)、[PNG原字节](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-main-explicit.png)。解码和实际view_image检查包含主窗口工程标题/工具栏、C球/轴、Style侧栏、Output与底部标签。[完整交叉核对](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-positive-crosscheck.json)。尺寸与SCK逻辑frame860×728相同；不把CUA高分辨率画面与此未经校准地逐像素比较。原始base64与PNG字节逐字一致；这是原生CA截图，非CUA会话图或导出图。

GUI正常0/147.53秒、softstop前，Quit后不读AX、无重启，parent-watch输入SHA保持；[CUA与退出](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/cua-observation.md)。未测试此例之外的模态框/遮挡/最小化/多屏/缩放条件，也未查附属窗口创建者。获取正确ID的方法明确，但不改默认first行为，不以此宣称重写视觉等价。

## 锁定源码与历史失败

[原锁定源码摘录/旧图核对](../../output/vesta-study/20261108-113622/reviews/20261108-184003-s04-window-selection/window-selector-source-review.json)确认Rust ObserveUiParams传window_id，Swift筛同PID/layer0/正宽高及可选exactID后取first；省略ID不按主窗口、标题或CUA焦点选。AX始终遍历进程窗口，window_id只限制截图。

旧PID11307/window53350的66×20原图保留，scaled=false，不改判为About/布局通过，也不复用旧ID。R14以UINT64_MAX[负控制](../../output/vesta-study/20261108-113622/reviews/20261108-184003-s04-window-selection/invalid-window-selector.stdout)得screenshot_window_not_found/0AX/无图、不回退；R30当前正控制与之分别保留。

R14[临时副本派生](../../output/vesta-study/20261108-113622/reviews/20261108-184003-s04-window-selection/ui-copy-derivation.json)仅包标识/显示名/调试签名，TEXT/原库相同/原257文件不改，但唯一BundleID没有隔离应用内部IPC。busy提示后副本255退出、截图timeout，无有效正图，退出原因未观察。用户后已确认旧81929是可关闭研究实例，R15复查已退出；此授权不扩大为关闭无关实例或重试碰撞。

## 当前状态与接续

[原生解决](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-unknown-resolve.stdout)、[原生校验valid](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-unknown-verify.stdout)、[读回](../../output/vesta-study/20261108-113622/reviews/20261108-234404-s04-window-positive/window-positive-unknowns-current.stdout)。本轮新增4至256证据，旧252逐记录不改；只截图头2→3/resolved，其它9头保持，当前8resolved/2investigating/0open。254功能/54样本全部保持、10勾选19未勾选，完整Study未完，Codex自验，dsh独立复审未开始。

以后视觉证据使用当次拥有PID/窗口标题与尺寸/当次ID，明确区分AX进程范围和图像窗口范围；只改Path.name生成探针路径并检查父目录、退出后先查拥有PID再决定是否读AX。无需重跑已回答的主窗口正控制；继续实际未验功能与领域数值。


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
