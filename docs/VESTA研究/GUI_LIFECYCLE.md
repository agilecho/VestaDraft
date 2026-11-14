# GUI工程取向与所选文件状态

本轮补原件GUI的取向读回、Save As字节、dirty取消关闭和一次外改覆盖行为；没有完成S05全部格式/失败/并发路径，计划仍10完成/19未完成。

## GUI保留取向，不能沿用CLI结论

原件位置参数打开既有`temporary-C1.vesta`，Orientation读回：

```text
+1.000000 +0.000000 +0.000000
+0.000000 +0.707107 -0.707107
+0.000000 +0.707107 +0.707107
```

[原生树](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/gui-orientation-readback.stdout)完整未截断；[产物/矩阵交叉核](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/gui-artifact-crosscheck.json)核其与保存的SCENE一致。GUI Save As到本轮新路径所得文件与源完整字节相同，SHA为2128427f4b5aa7e7dd5bd93ffaa93b90a51fe820538fb6b0f071cd0e9cc3f1b1；原件/源工程没有被覆盖。因此当前GUI读入/另存保留Rx45，而早前nogui读入/保存重写为I的失败仍成立，两种路径不合并成“全部工程往返失败/通过”。

## dirty与关闭

工作副本晶胞a由4改5 Å，体积64→80；窗口标题没有新增星号，但File/Close弹出“Save chages to a file?”并有Yes/No/Cancel。第一次提示之后状态在原生读取期间转为保存a5且关闭，触发者未观察；[该原生读取](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/gui-dirty-close-prompt.stdout)实际是空白主窗，不能凭回执名称宣称抓到提示。随后Cancel点击被CUA状态保护拒绝；不当取消关闭通过或归因读工具。

重新打开a5并改a6后再次触发提示，立即点击Cancel，工程仍打开、内存a6/体积96，磁盘仍a5。此分支由[CUA操作摘要](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/cua-observation.md)记录，不伪造CA提示回执。保存后File/Close不再提示；No/Yes明确点击、写失败及并发编辑仍未核。标题文本不是可靠dirty指示，至少本次没有星号变化。

## 一次外改与保存

仅修改本轮工作副本：磁盘a5→a7，并设置外改标题，GUI内存仍为a6。[外改前记录](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/external-change-observation.json)和分别保留的[外改文件](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/gui-external-a7.vesta)、[保存后文件](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/gui-after-save-a6.vesta)可复核。外改后CUA树无变化，File/Save未见冲突提示；结果为a6、原标题New structure，外改a7/标题被覆盖：[结果](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/external-change-result.json)。这是单次顺序操作，不证明所有文件监听、竞态、系统弹窗或冲突处理。

同实例Close后Open Recent重开，CUA输出a6、体积96，与落盘一致。最后原生UI读取超时，没有Evidence；[失败监督回执](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/gui-saved-reopen-a6.json)保留。重开结果不能冒充成功的CA捕获，也尚非新进程重开。

## 生命周期与剩余

[原件受控生命周期](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/native-lifecycle-gui.stdout)/[子进程结果](../../output/vesta-study/20261108-113622/reviews/20261108-194753-s05-gui-lifecycle/lifecycle-gui-lifetime.json)：PID9454，exit0、564.104秒、未期限终止；CUA Quit后仅清单确认未运行，未重新读AX。CUA可见取向截图没有窗口ID，显式CA截图正控制依然未通过。

新增3条原生证据到172；9个问题头逐字保持6resolved/3investigating/0open。254功能、37样本和既有34探针不变，当前只是5个稳定功能ID的选定行为补证。继续完整读写/导出、No/Yes/写盘失败/并发、磁性/准晶/多相/多面体、数据库查询/导入及S06/S07；完整Study未完成，Codex自验，独立复审归dsh。

## R18退出的反例记录

原子几何研究中Quit后出现保存提示，No点击前拥有进程450秒期限已到，监督SIGTERM(-15)，CUA拒绝动作；[原始生命周期](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/geometry-gui-lifetime.json)和[操作摘要](../../output/vesta-study/20261108-113622/reviews/20261108-202012-s06-atomic-geometry/cua-observation.md)保留。不能由最终未运行倒推No通过，也不改R16正常退出记录。No/Yes明确执行仍待核。

## R19有名工程的No/Yes关闭（当前选定结果）

各用既有C1的独立研究副本，a4改5、体积64→80后菜单Quit，明确点击提示按钮；这次两目标均正常退出，未到监督期限。原源工程保持。

| 分支 | 目标结果 | 工作文件结果 | 原生文件变化 |
|---|---|---|---|
| No | PID27835，exit0，97.767秒 | 仍a4/4/4，完整字节等于初始C1 | unchanged；before/after均2128427f… |
| Yes | PID27893，exit0，101.596秒 | a5/4/4，只有CELLP的a一行变化 | modified；2128427f…→1af8c190… |

[联合核对](../../output/vesta-study/20261108-113622/reviews/20261108-203811-s05-close-choices/close-choice-crosscheck.json)、[No原生生命周期/文件](../../output/vesta-study/20261108-113622/reviews/20261108-203811-s05-close-choices/native-no-gui.stdout)、[Yes原生生命周期/文件](../../output/vesta-study/20261108-113622/reviews/20261108-203811-s05-close-choices/native-yes-gui.stdout)、[两个文件的差异](../../output/vesta-study/20261108-113622/reviews/20261108-203811-s05-close-choices/yes-artifact.diff)以及[CUA明确按钮记录](../../output/vesta-study/20261108-113622/reviews/20261108-203811-s05-close-choices/cua-observation.md)分开。两个编辑后原生树证明各自内存已a5/80，文件before仍a4。Yes对该有名VESTA文件直接保存退出，未出现新保存面板；不推广所有格式/未命名/多文档情况。

R16已核Cancel保留，此处补No/Yes；R18失败的No尝试仍是未执行/期限退出，不覆盖历史。尚需写失败、保存面板取消、多文档与并发/恢复契约；源样本和原包未改，192 evidence、9问题头不变，完整Study未完成。


R20[图像导出](IMAGE_EXPORT.md)从R19 Yes文件逐字副本启动独立新进程，原生树读回a5/体积80；补新实例重开证据。两PNG/取消Vector后工程摘要未变，Quit无保存提示、正常0，限此路径。

R24四原子XYZ两文档经导入/测量/切换后Quit依次提示保存，两个临时输入选择No均保持；该链dirty触发点未分离，不能定为单测量脏。[结构几何](行为/02-结构与几何.md)。

R26原件37980测量后关闭未赶上570秒期限，-15/No未执行；随后AppAX自动重启默认偏好空白38111已菜单关闭。不得以父wrapper0记目标GUI0；[清理](../../output/vesta-study/20261108-113622/reviews/20261108-222640-s06-nonorthogonal-images/cleanup-final.json)。原工作CIF不改，R19等成功No案例不受此监督失败替代。

R27用ownedPID/UTC softstop360、harddeadline570明确预留210秒，实际354.149秒No关闭/exit0，退出后仅清单，无AX自动重启；不是通过改原件绕过生命周期问题。

R46四次打开LineProfile控制均重新为(000)→(100)/N7，选定输入未保留；导出四文件后Quit无提示，源不变。非一般偏好/脏状态结论。见[体数据规格](行为/03-体数据与二维.md)。

R60多窗口实测：主窗口Quit后数据库WebView窗口可继续存活。先ps确认原拥有PID尚在，再读AX/关闭实际剩窗，最后ps无目标、末次不AX；不能在父进程未完成时读取lifetime冒称退出。首例隐藏freshAX切到主窗导致误close也保留。详见[DATABASE](DATABASE.md)R60。

R61未保存a8/V120结构经数据库浏览/404失败/关浏览器后，Output全文保持；Quit仍有保存提示，No后磁盘源SHA保持。末次No后只ps无AX；说明浏览窗口关闭没有清除选定dirty状态，见[DATABASE](DATABASE.md)R61。

R62内层UnitCellTransform完成可回主窗口前景，但外层EditData仍存在且暂存新参数；重新进入UnitCell会聚焦既有编辑器，外层OK后主场景才更新。第一案原生前后树保留，不用内层OK当完整提交。见[结构规格](行为/02-结构与几何.md)R62。

R63 det0错误确认、det−1左右手警告No均回Transform编辑器，后续有效变换仍对应原始源结构；没有接受负det或完整取消矩阵测试。两案内层OK后重聚焦外层/外层OK才保存，终Quit后ps无AX，见[结构规格](行为/02-结构与几何.md)R63。

## R70 已运行GUI的命令交接、路径限制与图像导出

继续复用VS70/P1/cell4×5×6Å/V120、C(.125,.2,.3)/O(.625,.4,.5)、occ1/.5与Uiso.01/.02；不新增样本。首target前固定两个文件、a7→8的外部工作副本修改、单文件关闭、Rx30、8Å显示宽度、PNG两倍缩放/透明及SVG假设。[初始登记](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/gui-cli-before-hosts.json)。不传-nogui的发送实例与已运行原版GUI通过IPC交接；parse_gui_command0x100019b98/280bytes、vxConnection::OnExec0x100091e50/772bytes的CWD前缀/事件排队，及Open/Reopen/Close/ExportImg/Flush/ScaleWidth原生处理函数已静态定位。原发布包不修改。

**长路径失败与短共享目录对照**：长偏好目录对应实际服务名148 UTF8字节（日志原文保留）；发送方出现existing instance may be too busy提示，主窗口无打开对话框。进程sample指向OnInit→wxMessageBox→NSAlert模态循环；先独立偏好目录、再共用原长目录均未交接。两个原发送target达到idle18秒、SIGTERM/记录code1；独立诊断子进程16.186秒后归属明确地终止−15，外层正常0，不把其当成功。长接收实例正常退出0/364.394秒，**超过360秒软时限4.394秒**，未触发570秒硬清理。

观察后、短目录target前登记[对照假设](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/short-pref-transport-before-host.json)：分别用新归属短目录/tmp/v70oc-0929与/tmp/v70rd-0929，发送方与接收方共用同一个VESTA_PREF，服务名30字节。实际socket/listener由lsof及socket类型确认。冻结OnInit证据中_prefdir+vesta_+wxGetUserId构造服务名；dyld GOT0x1005F80A8绑定libVESTA/_prefdir，本机SDK sys/un.h的sun_path容量104。长失败/短成功与静态链**强支持所测长路径超限导致IPC失败**，没有测精确边界，也没有证明所有busy提示都由此引起。更改研究环境与修复原软件是不同事项；原包保持。

**打开、重新加载、指定关闭**：短路径发送-open含空格中文绝对路径后，原first页保留并出现second页/a7/V210。只改新工作副本a7→8，-reopen前UI最后报告仍7/210；-reopen同绝对路径后8/240。-close只关second，first保留；随后-save导出first仍cell4/5/6、V120、原位点/占位/Uiso。四个发送实例均正常0，实际窗口/输出另验，未把客户端退出当状态完成。

**视图、flush和PNG**：接收端执行-rotate_x30/-scale_width_to8/-flush/-export_img scale=1，保存项目SCENE的3×3矩阵与独立Rx30先验在六位小数半单位内相符；Overall Appearance→Projection/Width实读**8Å**。scale1普通PNG **1112×940/RGB**；option=transparent/scale2得到**2224×1880/RGBA、alpha0..255**，宽高各2倍。flush仅确认在此命令序列中可执行和随后实际图像状态，不推断独立渲染屏障语义。

**两个原假设不成立**：format=vector scene.svg不生成文件且接收端报Failed to saved an image。观察后先登记再做PDF/vector、SVG无选项和PNG/vector对照；PDF/SVG仍无文件/明确失败，PNG/vector有效RGB。ExportImg0x100018930/1012bytes始终走CaptureImage→ExportRaster0x1000a9bcc/1352bytes，没有矢量导出函数调用；这解释所测CLI路径不能完成矢量导出，不能推翻R21所测GUI矢量导出成功，也不声称所有CLI图像后缀都已覆盖。第二个补充先验曾预测相对输出落在发送cwd，但实际relative image 中文.png落在**当前结构文件目录**；原生处理函数在Scene已有绝对文件名时以其为base转绝对路径，不能由CWD协议推导所有输出都按发送cwd解析。两项原失败均保留。

**异步完成可观测**：普通PNG发送进程退出0时，其after快照仍为0字节/空SHA；稍后接收端实际写出PNG，父进程完整after快照与独立Pillow解码确认1112×940及最终SHA。另有长共享pref的VESTA.ini在接收端正常退出时更新。这两处快照变化由[后续归属父证据核对](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/watch-lineage-analysis.json)，不回写先前快照、不把即时fileexists/0返回当完成。短目录偏好文件另做归档且与原路径SHA一致；三个host均正常0，短host88.790/144.845秒（实际lifetime为准），无遗留进程。

[27项选定校验及原失败](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/gui-ipc-crosscheck.json)区分首先验、观察后解释与后续先验。38新至714Evidence、旧676及12头10resolved/2investigating保持；254功能/72样本、32权威、10完成19未完成。首次证据导出因探针名与导出stem相同被wrapper回执覆盖，KeyError在写索引/权威文档前检出，原回执保留；重新只读导出为native-evidence-final.json，不重跑产品target。完整CLI/IPC/Study未完成，Codex自验，dsh独立复审未开始。

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


## R86 GUI样式保留、Cancel崩溃与数值隔离未成（2026-11-09）

首target前 [登记GUI5/传入7对照](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/gui-style-before-target.json)：当前一相X矢量(1,0,0)/全局5，传入新相Y矢量(0,1,0)/全局7，经短共享VESTA_PREF发送-i/-save。客户端0/exited，另核父进程仍活及新5347byte文件稳定；保存两相标题current_style_five/incoming_style_seven、全局VECTS5，前后原生AX比例5、后相下拉表含两名。所测“仅追加相数据、保留当前全局样式”得到GUI反例控制，非总重置1/非取传入7。未发送-refresh，未验第二相主模型渲染。

**原两项预期失败**：保存第二相rawY为.10000，预期1.00000，原始分量一致性失败；尚未证明重开后的物理磁矩损坏。另，父进程18317在Vectors的Cancel恢复预览时SIGSEGV11/−11，146.850秒<180且无残留，但正常退出/正常cleanup失败。先前对话称“Quit时”已被系统栈修正，不能把工具序列中的Quit当已证触发动作。[11项证据核对及原失败](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/gui-style-crosscheck.json)。

具体上下文：先在1相时打开/Cancel Vectors，再CLI追加为2相，重开Vectors查看相列表后Cancel。[系统报告原档（历史文件名含quit）](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/VESTA-18317-quit-crash.ips) 与 [地址/UUID/寄存器绑定](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/vector-cancel-crash-binding.json)：main UUID cd806142…，PC0x1000214e8执行ldp x1,x2,[x20]，x20/FAR=0x300；调用栈OnCancel→SetPreviewMode(false)→VectorBase赋值。还原循环以现有Scene相数为界，却索引backupScene+0x4a0的相数组，无旧长度检查，再加箭头向量偏移0x300；空源指针与报告吻合。旧备份实际长度未读内存，1→2相造成旧备份不匹配是有据推断，未宣称全部根因/所有Cancel场景。

崩溃后另登记两个无界面数值隔离对照；第一个Y-only在18秒idle后被SIGTERM终止（native code1/reason idle_timeout、wrapper0），无命令输出/无文件。研究脚本错误地先读预期文件而报FileNotFoundError；raw失败/脚本保留，第二个X/Y对照未调用，不能形成数值结论。[停止记录](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/headless-control-stop-record.json)。下轮先用新有界拥有实例检查是否有启动恢复提示；超时原因尚未确认，不原样盲重跑，也不重放已知两相Cancel崩溃。

新增7至878原生来源记录；raw原生导出876/not_loaded2，[完整核验派生快照878](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/verified-complete-evidence.json) 保留旧871及12头、77样本、原件257、85历史manifest/11873文件。两个旧记录仍经原生ID解析补齐，无记录重写。254功能/32权威、10完成19未完成；Codex自验，dsh独立复审/S08待，无产品实现。


## R87 恢复提示、命令行对照与持久化矢量回查（2026-11-09）

R87先登记新进程计划，实际原生AX捕获macOS“unexpectedly quit while reopening windows”提示及Reopen/Don’t Reopen按钮；对先前拥有的研究窗口选择Don’t Reopen后，新单相主窗口正常加载并退出0。随后重新登记两组短偏好目录nogui对照：单Y、X再Y均原版正常255/exited，保存Y=1，分别1/2相，VECTS仍按R85命令行契约写1。此前PID25030的idle原因为何仍未唯一确定：当时没有UI证据，新的恢复选择和偏好目录也同时改变，不能把旧失败回写成通过。

两个全新GUI分别回读原incoming输入和R86已保存双相输出。原输入为Polar/Cartesian(0,1,0)/Modulus1.00/Scale7；输出第二相实际为Polar/Cartesian(0,.1,0)/Modulus.10/Scale5。因此原先保存Y身份失败已落实为持久化后GUI显示数值差异，不再只是一种尚未解释的序列化表示；仍未定位是导入、缓存场景还是保存阶段改变，不推广为一般磁矩科学损坏。原输入与回读工作副本哈希保持。两个回查均只Cancel内层Edit a Vector，外层Vectors用OK退出，Quit时对研究副本选No，不重放已知“缓存单相→IPC追加双相→外层Cancel”崩溃。外层OK即使无数值编辑也在这两次流程触发保存提示，此观测不外推到所有对话框。

三个拥有的GUI PID29107/29811/30583正常0，64.226/76.968/108.440秒均<180，无截止终止/残留。新增6个原生来源ID至884；raw原生导出882/not_loaded2，另两条经原生按ID接口补齐的完整884为明确标注派生快照。旧878记录、12问题头、旧77样本、原件257文件及86历史manifest/12021文件保持；新增稳定VS0078为独立规定的单相PolarY单位输入，样本增78。R86的raw失败、Cancel SIG11和旧headless idle记录不改。254功能/32权威文件、计划10完成19未完成；Codex自验，dsh独立复审/S08未开始，无产品实现。

[15项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-142400-s07-recovery-vector-controls/recovery-crosscheck.json) / [封存目录](../../output/vesta-study/20261108-113622/reviews/20261109-142400-s07-recovery-vector-controls/README.md)。


## R88 单相直接保存与取消对照（2026-11-09）

R88预先登记同一VS0078单相PolarY1/Scale7输入的两种GUI上下文：①不开Vectors直接IPC-save；②单相打开Vectors/Preview on/Scale7后直接外层Cancel，不改字段、不增加相，再IPC-save。有效目标PID37035/41032的客户端均正常0，父进程另核实际文件两次稳定；两份3836字节输出均与输入SHA逐字节相同，Y1/Scale7完整保留。有效GUI正常0，45.565/123.453秒<180，无残留。因此在此样本中，单相打开/取消矢量窗口及单独IPC-save不足以触发R86第二相Y.1；尚需没有预开Vectors的双相追加对照，未把GUI追加阶段或完整根因宣称已定位。

初次nativeUI参数错误加入不支持的max_depth，工具bad_input/1未形成Evidence；按已知schema改为accessibility/max_nodes，原失败保留。首次第二组PID39128退出255/51.067秒，原生UI只depth0根节点，未执行取消/保存，对照不可用。同期另见空白研究实例PID37165，已通过CUA正常Quit清理；其创建及启动退出的精确因果未唯一证明，不把它当科学结果或正常GUI验收。新目录/偏好/唯一输入名重新登记后，先原生确认41032与已载入窗口再CUA操作；退出后通过拥有进程生命周期核实，避免对已退出app再次调用可能重新启动它的入口。中途一次在native读取未结束时尝试JSON读取空stdout发生JSONDecodeError，未触发产品动作，后续等待真实完成再读，不能把文件存在当完成。

14项限定核对通过；新增6原生ID至890，其中包含保留的空根节点启动观测。raw批量888/not_loaded2，完整890为原生raw加2条原生ID补读的明确派生快照。旧884/12问题头/78样本/原件257文件及87历史manifest/12184文件保持。R86持久化Y身份失败、Cancel SIG11和R87实证数值差异不撤销。254功能/32权威文件、计划10完成19未完成；Codex自验，dsh独立复审/S08未开始，无产品实现。

[14项核对](../../output/vesta-study/20261108-113622/reviews/20261109-145500-s07-single-phase-save-controls/save-controls-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-145500-s07-single-phase-save-controls/README.md)。


## R89 无预开Vectors的GUI双相追加（2026-11-09）

R89在首target前登记全新GUI：直接载入current X单位/Scale5，只通过共享短VESTA_PREF的CLI客户端执行-i incoming Y单位/Scale7及-save新文件，整个过程没有打开Vectors窗口。原生AX先绑定PID41723与唯一输入窗口；客户端正常0，另核父进程输出5347字节的两次稳定读数，确有两个不同TITLE、全局VECTS5，但实际vectors仍为(1,0,0)/(0,.1,0)，独立预期第二相Y1失败。新输出与R86原输出逐字节相同，SHA0eea1bd2c41b423d19d2dca370e361477dff1f250fb9d36e16183980e0c46214。

因此R86曾预开/取消Vectors并不是此数值变化的必要条件；R88单相Cancel/save两份逐字节保留输入，R89没有预开Vectors仍复现双相Y.1。数值问题已与已知VectorsCancel恢复崩溃的触发条件区分。R89没有重新读取Edit Vector数值或验证主模型第二相渲染；R87对逐字节相同R86文件的独立GUI回查已实证CartesianY.1/modulus.10，不能把R89称为又一次实际GUI字段回读。导入与保存之间的精确变化阶段仍未定位，下一步可在只-i尚未-save时，以全新双相对话框回查第二相数值，保持先验Y1。

父GUI正常0，47.116秒<180，无期限强制终止/残留；CUA仅在原生读取全部结束后绑定已有窗口并Quit，未调用Vectors/Cancel，不重放缓存单相→追加双相→Cancel崩溃。12项限定核对通过，但原数值身份预测失败保留。新增3原生ID至893，raw891/not_loaded2、完整893为明确派生的原生raw+2条按ID补读；旧890/12问题头/78样本/原件257文件及88历史manifest/12348文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完成，无产品实现。

[12项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-150700-s07-bare-gui-append/bare-append-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-150700-s07-bare-gui-append/README.md)。


## R90 保存前的第二相矢量读取（2026-11-09）

R90在首target前登记GUI current X1/Scale5与IPC incoming Y1/Scale7，只发送-i，不发送-save。原生先绑定PID44794与唯一主窗口，导入客户端正常0/父进程仍活；首次Vectors窗口是在双相导入之后才创建，Phase2/incoming_style_seven实际选中，Edit a Vector为Polar、Cartesian(0,.1,0)、Modulus.10、全局Scale5。独立保留原Y1/modulus1预期，此预测失败。主日志有Reading incoming、没有Saved data，工作目录始终只有两份未改输入，没有保存输出；因此差异已在任何保存之前的第一次GUI读取中出现，保存不是此显示数值变化的必要条件。对话框读取本身是否会改变内部状态没有被直接内存检查排除，不能把该观测推广成精确导入指令根因。

补充原生AX观测：该活体第二相的晶格、沿晶轴、倒易与Cartesian四组字段均为(0,.1,0)，而R87原单相输入为晶格Y.1/晶轴Y1/倒易Y10/CartesianY1，保存输出新开时为晶格Y.01/晶轴Y.1/倒易Y1/CartesianY.1。坐标基底未初始化成为有依据的待验证假设，尚未直接读取矩阵内存、也未证明初始化遗漏。旧R85 CLI Read函数已见临时Scene读取后只搬Crystal指针/文件名，无显式CoreInit调用，但其它间接/调用上下文仍需核，不据此单独宣布完整因果。

结束时只Cancel内层Edit Vector、外层Vectors OK，Quit对拥有研究副本选No，未执行任何Save/未重放缓存单相→追加双相→外层Cancel崩溃。原版GUI正常0/160.717秒<180，无强制终止/残留。13项限定核对通过，原数值保留失败不改写成成功。新增3原生ID至896，raw892/not_loaded4、完整896为明确派生原生raw+4条原生按ID补读；旧893/12问题头/78样本/原件257及89历史manifest/12476文件保持。254功能/32权威、10完成19未完成，Codex自验，dsh/S08与完整Study待，无产品实现。

[13项核对](../../output/vesta-study/20261108-113622/reviews/20261109-151300-s07-before-save-vector/before-save-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-151300-s07-before-save-vector/README.md)。


## R92 读取优先顺序与flush恢复失败（2026-11-09）

R92在首target前登记两项新对照。无界面参数故意写为-nogui -save OUT -i X -i Y：原版正常255/exited，实际先形成两相，保存X1/Y1、全局VECTS1，整个输出与R87常规-i X -i Y -save的两相文件逐字节相同。事前R91读取优先→Build→其余命令的顺序预测通过，这不是按参数文本从左至右逐条保存的契约。

GUI新进程载入X1/Scale5，第一次IPC-i Y1/Scale7/-save before输出复现两相Y.1/global5。第二次明确发送-flush/-save after；两个客户端均0、父进程仍活，两个不同输出文件分别经两次稳定内容检查，after仍Y.1、与before及R86完整5347字节逐字节相同。因此事前“flush后恢复Y1”的预测失败，不能宣称flush修复初始化，也不把它回写成预期.1。未运行时追踪三个虚调用回调是否逐一执行，不由此推广-flush一般无效。主日志有两次Saved路径；没有Vectors窗口/字段修改，主模型第二相渲染未另验。坐标基底初始化的完整因果仍待，不由无界面顺序成功替代GUI恢复失败。

GUI父57445正常0/64.178秒<180，无期限强制终止/残留；CUA仅在原生读完后绑定实际主窗口并Quit，无已知Cancel崩溃重放。15项证据范围核对通过，原flush恢复预期及originalAllPredictionsPass均false。新增5原生ID至918；本轮实际raw896/not_loaded22，22条经原生按ID补读构成明确派生完整918，旧913/12问题头/78样本/原件257及91历史manifest/12872文件保持。254功能/32权威、计划10完成19未完成；Codex自验，dsh/S08与完整Study未完，无产品实现。下一步转入全功能覆盖缺口盘点并补未观察入口，数值失败/静态假设作为明确边界保留。

[15项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-153900-s07-read-order-flush/read-order-flush-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-153900-s07-read-order-flush/README.md)。


## R93 输出文本、清空与目录覆盖回写（2026-11-09）

R93转向全功能缺口：修改前254功能中162条canonical status仍为entry-observed/behavior-pending，不能据此断言162条从未运行。重新核VM04 §4.2.1/.3及VM18 §18.5.4，并Deep定位OnSaveLog(main100098a08/456)调用wxTextAreaBase::SaveFile@100098ba8；OnClearLog(main100099568/20)尾跳br x1未解析，静态不冒称该虚目标已知。符号查询42匹配只返回40，目标两项已找到，不称全目录穷尽。

事前登记当前Output标签测试：File/Save Output Text保存到中文空格路径，首次1062字节与保存前原生AX文本的UTF8逐字节相同；View/Clear Text Area后原生AX为空，再保存另一文件为0字节，首次文件哈希保持。输入模型源不变。首GUI58637正常0/209.600秒，超过180交互目标29.600秒、在240软截止前退出；末尾模型检查被deadline断言在生成参数/调用工具之前阻止，未执行、未伪装成功。另事前登记只含清空→模型保存的新短流程，GUI59217正常0/69.892秒；原生确认空Output，IPC客户端0与父进程稳定文件另核，模型3834字节与输入逐字节相同。没有重放已知Cancel崩溃或修改模型参数。

19项限定证据核对通过，首轮originalAllGUIUnder180=false及originalFirstModelCheckExecuted=false保持。VF0006只核Command-Q选定正常退出、VF0017只核Output非空/空导出、VF0036只核Output清空及另流程模型保存保持；其余Summary/Comment、取消/覆盖/写失败/Undo及一般退出分支待。三条catalog记录回写具体前提/动作/持久化/失败边界，其他251记录不变；入口级待核由162降159，不把这三条称全分支完成。稳定ID仍254、样本78。

新增8原生ID至926，本轮实际raw898/not_loaded28，28条经原生按ID补读构成明确派生完整926；旧918/12问题头/78样本/原件257及92历史manifest/13075文件保持。32权威、计划10完成19未完成；Codex自验，dsh/S08与完整Study未完，无产品实现。

[19项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-154800-s04-output-text/output-text-crosscheck.json) / [覆盖回写](../../output/vesta-study/20261108-113622/reviews/20261109-154800-s04-output-text/coverage-after.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-154800-s04-output-text/README.md)。


## R102 常规属性关闭与No保存控制（2026-11-09）

R102先静态追关闭链，再事前登记两个短偏好目录/独立研究副本，禁止退出后按bundle盲查。Properties/OK main0x1000206f0在0x10002072c调用Document::PushHistory(2)；lib0x20a2c在非空历史分支把Document+0x1f8所指对象+0x131写1。OnFileClose main0x1000a71e8从Document::GetCurrent取得Scene，检查Scene+0x131，提示调用位于0x1000a7374；消息返回8跳过保存、2走0x1000a7538调用OnFileSave，其它走0x13ed取消。Quit事件OnExit0x100098db8转wxWindowBase::Close(true)，OnDestroy0x1000a14e4遍历文档调用OnFileClose并在13ed停止。所读函数内未见无条件回写工程分支，但含未解析虚调用，不作全程序不存在自动保存的断言；数字返回值不是动态回调追踪。

两个新当前格式副本均来自R93规范3834字节源d4c508851f6fe92c09e2931e31079dff6d9468fe748d950250887dab16bc9f48，Properties Preview1，仅线型改Dashed或轴标签关闭后OK。分别IPC另存到新文件：虚线SHA02cbf587b308922e808910690916b4ee743d7e73475504c5e5647b5bfbdbd85b，COMPS2为52cfb649b8e6b8549c67efc2c7c3c4ede511d372d807c2b4b3a5e5a1183a015c；整文件精确符合目标前指定字段预期，源与输入工作副本仍原字节。Quit前、按原PID捕获提示前后、退出后均记录输入哈希/mtime；两个实际原生提示均含所属文件名及Yes/No/Cancel。只有原生守卫确认提示且所属PID仍活才查询CUA并明确点击No，输入完整哈希与mtime均未变。0.1秒轮询未观察到变化，不冒充系统调用/写入者追踪。

拥有GUI33485/34014正常0、112.338/104.157秒，均<180且无期限强停/残留；8次原生UI/2个IPC客户端分别核验。退出后再执行compass原PID守卫，确认lifetime已记，既不原生读取也不查询CUA，不会因研究查询主动重开空白实例。首线型CUA选择器错误地要求No后行尾，而实际有ID后缀，未点击即抛错；刷新结果明确按钮4后点击No成功，选择器失误保留。两流程没有新空实例。

这两个单次字段变更控制中没有观察到R101工作副本异常写入，并非完整重放R101全部切换序列，不能据此把R101原只读预期失败改成通过或归因某组件。R101异常COMPS2工作副本、mtime区间与全部原生旧快照保持，写入者和精确触发仍未隔离。58项限定核对通过；本轮所选No丢弃和守卫契约有证，Cancel/Yes/多文档/写失败/完整关闭协议未因此全部验收。

新增19原生ID至1119，实际raw1031/not_loaded88；88条原生ID补读组成明确派生完整1119。旧1100/12问题头/78稳定样本、原件257及101历史manifest/16588文件保持。六条旧记录扩展、其它248条不变，入口级待核128不变；254功能/32权威、10完成19未完成，Codex自验，dsh/S08与完整Study未完成，无产品实现。

[58项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-185900-s05-close-write-semantics/close-write-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-185900-s05-close-write-semantics/README.md)。


## R103 边框Cancel与默认属性选择性复制（2026-11-09）

R103事前登记同一新单相晶形副本及后续复用短偏好目录的新CIF进程；原规范源为R99 current3.5.4/3752字节/SHA0fedd1bdef2efef374149c334a7842e22e83cfeeeb6b0a9050e49ef5a42aaa0d，含六个晶面。General Show edges的Scene+0x3c2对应XStyle+0x19a，原生写入器0xb8b04打印FORMP首值。边框关/开输出仅该值0/1，开启精确恢复整源；各阶段完整文件符合目标前字段预期。

两个宽泛假设均被否定，保留原登记而不改成通过：关闭边框和轴标签后点击Save as Default，再Cancel，当前Scene保存为FORMP0/COMPS1，指南针恢复而边框仍关闭，所以“Cancel恢复全部改变字段”不成立。全局默认最终为FORMP1/COMPS2，指南针变更进入默认而边框未进入，所以“按钮保存所改全部General字段”亦不成立。这是所选字段与路径结果，不擅称所有Cancel/默认行为均坏。原生CopyPropertiesTo lib0xba0ec复制+c的COMPS，直接块0x148..0x197在FORMP的0x198..0x1a3之前结束，后续直接字段由0x1a4起；R101 CopyParam/Cancel与按钮都调用它。静态选择性复制与两种实际不对称吻合，未动态追踪回调。

按钮main0x100020750将Scene+0x228属性复制到_defParam，处理器无直接文件写入。原生GetPrefvar lib0x21198读取VESTA_PREF并赋_prefdir，存在变量时跳过HOME回退；FinalizeLibrary0x22890转save_default_pref0x2219c，在0x22810调用DefaultParam::save_style0xb9cc0，拼_prefdir与style/default.ini并调用既有XStyle写入器。运行明确设/tmp/v103d-1930，按钮前/后和Quit前的归属快照均无default.ini，正常退出才出现50724字节文件SHA080fcc15316eee9ccf493d8a9ad5637f4e52aad45cc2343a49bd33b7c8155ac2、FORMP1/COMPS2。随后新进程直接读既有VS0070的533字节CIF副本，原生General读到Showedges1/axislabels0，另存3975字节项目字段一致；第二次退出default.ini整字节哈希仍相同，mtime正常更新。外部用户偏好没有独立前后清点，不能把归属路径证据扩大为已核所有外部文件。

GUI44013/58052正常0，159.555/67.855秒均<180；六个IPC输出各独立核发送结果及父文件稳定，八次原生UI完整。两个Quit后归属Popen已结束，清理守卫跳过原生与CUA查询，未重开空白实例。两工作输入及规范源哈希/mtime保持，稳定样本78未增。61项限定检查通过、静态选定字段预测通过；broadCancelAllFieldsHypothesisPass/broadSaveDefaultAllFieldsHypothesisPass=false。不改R101副本异常写入的旧失败、也未隔离旧写入者。

新增25原生ID至1144；实际raw1022/not_loaded122，122个ID原生补读组成明确派生完整1144，旧1119及12问题头保持。原件257和102历史manifest/17117文件校验，六条功能扩展、其余248不变，入口级待核128→126；254功能/32权威、10完成19未完成。Codex自验，dsh独立复审/S08与完整Study待，无产品实现。

[61项限定检查与两个被否定假设](../../output/vesta-study/20261108-113622/reviews/20261109-191500-s04-general-defaults/general-defaults-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-191500-s04-general-defaults/README.md)。


## R104 Atoms字段与选择性Cancel（2026-11-09）

R104使用既有VS0057 P1非正交各向异性CIF（774字节，SHAdc52d2f42554ceb909a6d9ea129a3bfa54efa1ca708d4bb1c605b34f3d892840），两个U张量的Sylvester主子式均正，作为有用的正ADP输入，不等同椭球几何验收。先静态追Atoms控件，再登记GUI元变形规则：球体0/椭球1对应DISPF bit17和ATOMS第二值，主椭圆对应Scene+0x274/XStyle+0x4c、ATOMP第三值；隐藏非成键原子为bit11/ATOMP第六值，手性标记为bit23/LABEL第四值。其余所有工程字节不变作为目标前检验规则，基于首个稳定GUI基线而非独立科学渲染oracle。

原生Atoms UpdateCanvas10002e148返回35条指令、两处间接跳转未解，coverage complete1/1与未截断只表示该结果有一个函数，不证明所有分支覆盖；原始限制明确说可能缺代码。首次native objdump以地址范围调用，/usr/bin/objdump实际忽略范围，exit0却从100001630开始且共378601行，不能当范围取证成功。保留该记录，另用offset40000/limit12000原生分页，从其输出按地址派生182条连续指令，完整覆盖10002e148..420，补147条遗漏；派生文本不是原生记录原样。两个原生u8读取表1004d73aa/ab均66，事件2bc2/2bc3从基址10002e194加66×4均到10002e29c，0x10002e2b0 bfi写bit17，证明模式分支字段。R97写入器支持各重复编码，R103 CopyPropertiesTo mask0x02422800含11/13/17/22/25而不含23；静态映射不是动态执行追踪。

首个modes GUI53175未先确认监听即发送基线保存，客户端255/exited且无文件，wrapper0不代表成功；parent仍活，后续原生主窗证明模型已加载。controller还在client完成前试读缺失文件而报FileNotFoundError，脚本失误保留。该GUI未改属性，正常Quit0/86.503秒；发送时实际IPC监听未取证，原因不唯一归为启动竞态。另登记新modes_ready，先原生绑定主窗、lsof证实拥有PID的短socket，再发客户端0且稳定文件；后续短实例沿用这个守卫。

模式往返GUI72670初始两个模式单选框AX均0，工程ATOMS第二值0，异常初值保留，未唯一归因。手动选椭球后选中1且启用概率/主椭圆控件，principal开/关、球体恢复取得5个4180字节文件；最终整文件恢复基线SHAbc14120712f5edbfece81aca0689b2805fabfbf6cb9498c703513e3c99ff8577。该实例正常0/185.547秒，超过180目标5.547；未执行原计划改变状态后的Cancel/after_cancel，不从无编辑收尾Cancel倒推回滚通过。原toggles长实例未启动，之后重新事前登记三个最小上下文。

新mode_cancel88866将椭球+principal开启后Cancel，整文件恢复基线；hide_cancel95156开启Hide后Cancel亦完整恢复；mark_cancel96110开启Mark后Cancel，bit23/LABEL末值仍1，afterCancel与changed文件整字节相同（SHAf27eebd6581c0c781732de15623f3cb1c802b03037e1e4397b145ea191ff6eff），宽泛“Cancel恢复所有改动”在手性标记上不成立。三实例正常0/94.999、88.677、95.001秒，均<100；与选择性copy掩码一致。五个实例均正常退出，原输入及规范源哈希保持；所有退出后的原PID守卫跳过原生/CUA查询，无空白新实例。14个阶段输出精确符合元变形规则，11次原生UI完整，95项限定核对通过；初始保存失败/旧180目标失败/mark完整回滚假设失败保留。椭球/主椭圆几何、成键/孤立视觉对照及手性配对着色尚未验收。

本轮新增37原生ID至1181，raw1052/not_loaded129，129个ID原生补读组成明确派生完整1181，旧1144/12头保持。汇总脚本在完整快照/来源/加载审计均完成后，索引阶段把commands数组当dict而失败；原脚本/错误保留，仅补isinstance检查并独立重验union/旧记录，补完37-ID索引，不重导出或重跑原生。原件257、103历史manifest/17748文件及78稳定样本保持；7条功能扩展、其余247不变，入口级待核126→121。254功能/32权威、10完成19未完成，Codex自验，dsh/S08与完整Study待，无产品实现。

[95项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-193400-s04-atom-properties/atom-properties-crosscheck.json) / [分支覆盖限制与补证](../../output/vesta-study/20261108-113622/reviews/20261109-193400-s04-atom-properties/dispatch-coverage-followup.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-193400-s04-atom-properties/README.md)。


## R105 Bonds与SBOND耦合及不完整Cancel（2026-11-09）

R105以旧R66真实成键工程positive-project.vesta为规范源（5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，原有七原子/六条2Å键/一个SiO6多面体证据保持）。三个新GUI各先原生绑定PID/窗口/7atoms6bonds1polyhedra，再lsof证明所属短socket，然后发送基线IPC；三个基线均整字节等源。Properties/Bonds六个样式索引为Unicolor0/Bicolor1/Colorline2/Gradient3/Dotted4/Dashed5；原生六单选框的唯一选中均符合所测阶段，初始Bicolor1假设通过。

静态OnBondMode main0x10002ea34读组选中索引到dialog+0x41c，转event0x2c28/UpdateCanvas，在0x10002fd50调用Scene::UpdateBondsStyle。lib0x8134c写Scene+0x28c，并循环每相调用Crystal::UpdateBondsStyle0x6b534，后者对Crystal+0x238向量对象进行引用/复制并在0x6b57c写byte+0x40。原始计划曾把这些视为可能只供绘制的对象，实际文件证明它们还对应被序列化的SBOND规则，不能回写旧计划制造事前知道。复用R97 XStyle写入器从+0x64打印BONDS，Scene+0x228+0x64=0x28c；R103默认样式文件SBOND格式的第十值（零基token9，含行号）在0xb9ffc读byte+0x40。后者是style/default.ini写入器的静态布局证据，未冒充已追踪本轮工程保存调用。

初始圆柱组单色0实际同时改变BONDS0与唯一Si/O SBOND规则样式0，原“只改BONDS、其余工程字节保持”失败；手动改回Bicolor1精确恢复整源。再选Unicolor0、原生捕获后Cancel，保存结果BONDS回1，但SBOND仍0，原完整回滚预期失败，afterCancel SHA4dcfd80f917f53ed3de489aa138c628c2f98d1d4abb7b743592e93245a26108e，输入研究副本仍原哈希。与CopyParam仅恢复所选XStyle/其它向量、未见Crystal+238恢复的静态路径一致，但没有动态恢复调用/唯一原因追踪。

观察后、后续目标前另登记耦合字段预期：彩色/渐变组2/3，点/虚线组4/5，两级样式一起变化；Cancel后预测全局回1、SBOND保留最后3/5。原始单字段/完整回滚预期保持失败，不把后验规则追溯成通过。两后续组8个文件逐字节满足新先验，其余字节、结构/规则距离/半径/线宽/RGB等保持。三个afterCancel实际为(1,0)/(1,3)/(1,5)，后两文件SHA859e1e750407ae8162600903b6e86660aaeb7c653d9498e1321de74d7f35bb8e和65b731abfca2973635acb6a4c7c303687b6d7fdfde8cdfadc2b715b49d088f39。重新打开的优先级、实际颜色/圆柱/虚线像素和完全内存回滚未验，不能因BONDS1或文件存在宣称Cancel成功。

三拥有PID45983/54319/57263正常0，190.894/122.949/121.544秒，无期限强停。120目标全部未达，分别超70.894/2.949/1.544秒；圆柱亦超180目标10.894，其余两组<180，所有功能native发送/读取在180前、关闭均<240。退出后原PID守卫直接跳过原生/CUA查询，无空白新实例，规范源及三个输入哈希/mtime保持，六个归属偏好文件归档。首次后续计划准备脚本因原计划缺actions键抛KeyError，未写新计划或改目标，错误保留；修正的新计划明确写动作且先于后续GUI。原计划未补写actions。

72项限定核对通过、12阶段文件与11原生UI完整；originalOnlyBONDSPredictionsPass/originalFullProjectCancelRestorePass/originalAllExpectationsPass=false，followupCoupledPredictionsPass=true。新增31原生ID至1212，实际raw1074/not_loaded138，138个ID原生补读组成明确派生完整1212；旧1181/12头、原件257、104历史manifest/18538文件及78稳定样本保持。八条功能扩展/其余246不变，入口级待核121→115；254功能/32权威、10完成19未完成。Codex自验，dsh/S08与完整Study待，无产品实现。

[72项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-200500-s04-bond-properties/bond-properties-crosscheck.json) / [初始失败保留](../../output/vesta-study/20261108-113622/reviews/20261109-200500-s04-bond-properties/cylinders-observed-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-200500-s04-bond-properties/README.md)。


## R106 键规则冲突重开与手动同步（2026-11-09）

R106以R105冻结失败后像为源，分别新建全局BONDS1/规则SBOND0及1/3的工作副本，短独立偏好、原版进程，先原生PID/窗口/7atoms6bonds1polyhedra及lsof所属监听就绪，再执行初始IPC。三个实际baseline（梯度初次与重试、单色）均5144字节整源一致；重新打开未自动归一化被Cancel留下的规则，R105原始单字段/完整回滚预期仍false。

单色冲突副本原生Properties/Bonds六单选仅Bicolor1选中，与全局BONDS一致而非规则0；未编辑、直接Cancel后的新输出整字节保持(1,0)。梯度独立重试同样原生仅Bicolor1选中、规则仍3；再次点击已选Bicolor，after_same整字节仍(1,3)。先Unicolor0、再Bicolor1，每步CUA确认唯一选中，随即Cancel，新输出仅SBOND规则3→1，BONDS仍1，其余字节保持，整个5144字节精确等于R66规范源SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。原生读框与CUA动作记录分别标识，不冒充手动中间状态都有原生捕获。

复用R105完整130指令UpdateCanvas：0x10002fd40读Scene+0x28c、fd44读dialog+0x41c，fd48比较，fd4c相等跳至fdd0、绕过fd50的UpdateBondsStyle；后者经Scene8134c/Crystal6b534向规则byte+0x40写样式。此静态路径与同选项点击不归一化、异选项往返归一化的实际文件一致，但本轮没有事件/回调追踪；已选单选也可能根本不发事件，不能宣称动态唯一原因。界面选中值与文件值分歧不等于已验证实际绘图优先级；颜色、圆柱/虚线像素、多规则覆盖、几何科学验收仍未完成。新文件仅是研究行为控制，不修补/覆盖R105失败后像、原件或产品。

初次梯度PID66525完成baseline后遇上下文恢复，恢复时已过180功能窗及240关闭目标；仅读取既有主窗口后Quit，未打开Bonds或执行后续交互。实际正常0/251.587秒，120/180/240目标与完整动作执行皆false，两个计划输出不存在，失败单独留存。另立、先登记的新梯度重试PID76692正常0/82.804秒，单色PID72362正常0/58.586秒，均<120，未用新成功抹去原超时。所有实际功能native读取/IPC在各180窗前，退出后无应用重查/空白新实例；初次过清理读取截止后未补做原生读取。三工作输入哈希/mtime和每0.1秒抽样单一状态保持，三源为冻结旧证据，六个所属偏好常规文件已归档。未宣称捕捉每次系统调用或写入者。

54项限定核对通过、6个阶段文件/5个PID原生界面；新增11原生ID至1223，raw1082/not_loaded141，141个原生按ID补读组成显式派生完整快照1223，原raw不改。旧1212记录/12问题头、原件257、105历史manifest/19284文件及78稳定样本保持。扩展五条VF57/131/132/134/143，其余249不变；入口级待核115不变，254功能/32权威文档、计划10完成19未完成。Codex自验；dsh独立复审、S08与全Study仍未完成，未开展产品实现。

[54项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-203400-s04-bond-reopen/bond-reopen-crosscheck.json) / [初次超时留存](../../output/vesta-study/20261108-113622/reviews/20261109-203400-s04-bond-reopen/gradient-time-window-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-203400-s04-bond-reopen/README.md)。


## R107 多面体边线与线宽（2026-11-09）

R107继续Properties/Polyhedra，复用R66原版5144字节真实SiO6工程SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56；七原子/六条键/一个多面体为本轮原生就绪前提，未以其存在替代新科学几何验收。两份工作副本与短独立偏好，各先绑定原生PID/窗口/7/6/1及lsof所属监听，再发baseline IPC；两个基线整字节等源。两组动作、整文件变化预期、Cancel预测均先于GUI登记。

native函数目录DlgPolyPage共15项。OnChkShowLine main0x10005fb8c（272字节/68指令）读checkbox控件到dialog+0x464，在0x10005fc44存Scene+0x2a8、fc4c存线宽float+0x2ac；UpdateCanvas0x1000610f4（540字节/135指令）在611f0/611f8存相同字段。GetPolyParam0x100060e80（572字节/143指令）从Scene+2a8/+2ac回读到dialog+464/+474。三个已知范围均逐4字节连续包含尾部，保留builtin indirect_jumps_unresolved等局限；虚拟调用/尾跳并非已动态追踪。Scene内XStyle始于+0x228，故边线int+0x80、线宽float+0x84。复用R97 XStylewrite_style0xb89ac：b8c10读+80、b8bfc读+84，格式500e8c为“ %3d %i %6.3f %3i %3i %3i\n”，前两个实参为opacity与edge，第三float为width；POLYS的+7c为另一整体样式字段，不能混淆。复用R103 CopyPropertiesTo在ba18c/190复制+7c..83（包含edge+80），ba194/198复制width+84，为Cancel预期的静态依据。

边线组原生初始Show polyhedral edges=1/width1.0，关闭后原生edge0，CUA线宽与三个RGB输入均禁用；新文件仅POLYP第二项1→0，SHAcec92f4931944c92e055f8b0ee50b39022f352e6ef19391eb8ed55072b5dcb78，其余工程字节保持。手动开启后整源恢复，控件重新可编辑。再次关边线、原生确认edge0后Cancel，新输出整源恢复，不能把这个通过推广到R103边线形状、R104mark或R105键规则失败字段。

独立线宽组实际Line width输入1.0→2.0，Tab失焦后CUA和原生均显示2.0；新文件仅POLYP第三项1.000→2.000，SHA92b0f5529395072226bc9844593b7f0951afa60a389bd8349572f4a8ca7b70b6。Cancel后又精确等于整份5144字节规范源。两个真正改动状态的Cancel控制通过，DISPF/POLYS/SBOND及其余字节均不变。没有依靠关闭后AX重查启动空白实例，也未按disabled控件强行操作；实际屏幕边线/GL线宽像素、透明度颜色、不同中心原子、全状态回滚仍待。

拥有PID81001/88127均正常0，118.728/80.457秒，120与180目标均满足；所有功能nativeIPC/读取在180窗前，退出后原PID守卫直接跳过native/CUA查询。两个输入的哈希/mtime在0.1秒抽样和退出前后保持，规范源及四个所属偏好常规文件保持/归档。

49项限定核对通过，7个阶段文件/7次原生UI；新增18原生ID至1241，raw1100/not_loaded141，141次按ID原生补读组成显式派生完整快照，raw不改。旧1223记录/12问题头、原件257、106历史manifest/19945文件、78稳定样本保持。扩展VF58/137/143三条、其余251不变；入口级待核115→114，254功能/32权威、计划10完成19未完成。Codex自验；dsh独立复审/S08与完整Study未完成，无产品实现。

[49项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-205300-s04-polyhedra-properties/polyhedra-properties-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-205300-s04-polyhedra-properties/polyhedra-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-205300-s04-polyhedra-properties/README.md)。


## R108 等值面属性与资源重开失败（2026-11-09）

R108先研究Isosurfaces原生路径：DlgIsosurfPage目录16项，OnTextCtrl main0x10004c408范围1304字节/326指令，constructor0x10004c928为5260/1315，InitDialogParam4e2e0为88/22，ResetScene4e3b0为180/45，均逐4字节连续覆盖已知范围，保留间接调用等限制。constructor给+3a8的checkboxID2cee，对应前后绘制；OnTextCtrl在4c4c8清bit13、4c4cc用0x2000并在4c4dc写Scene+5b0（XStyle+388，DISPF）。反色checkbox+408/ID2cf6，constructor4d440按signed Scene+2f1的负号选中；handler4c610写ff（-1）、4c914写1。复用R97写入器b8cfc从XStyle+c9符号扩展，500ed7格式“%3i %12.5E %12.5E\n”写TEX3P首项；Scene+2f1=+228+c9。CopyPropertiesTo掩码含bit13，ba1bc/ba1c0半字复制+c8包含+c9，先登记对应Cancel预期。

首次采用R98冻结current3.5.4项目baseline.vesta（3539字节/SHA9f60f6acdb72c9641e0ddbee3942c870b585479bce52b50ecb7c7c45caf31838），同时复制其IMPORT_DENSITY引用的相邻visibility-volume.cube（1519字节/SHA04a77cbbe05ff93df31612dfc25d5a38c8f69bb4d1fd853277e63aa098b13f68）。原生PID/窗口及文件哈希正确，但实际等值面0polygons/0vertices，CUA体数据控件禁用；正体数据16/16的事前就绪预测失败。readiness断言返回1，未发任何baselineIPC，未开Properties或更改控件，原front三阶段输出不存在，原inverse未启动。PID93347正常0/29.244秒后退出，输入及资源不改。原项目重开失败保留，不能以有文件或相邻Cube就宣称加载成功，也未隔离读取原因或推广为所有VESTA项目失效。

另立且先登记两个直接Cube控制；两份同名visibility-volume.cube新副本/独立短偏好，原生各16等值面多边形/16顶点、0atoms0bonds0polyhedra及lsof所属监听就绪后才发送baseline。实际3539字节序列化基线均精确等于旧R98冻结项目，输入Cube与预期序列化VESTA的哈希分别核验。两组初始原生front0/inverse0/Preview1；前后绘制开启时唯一front1，文件只DISPF37753794→37761986（bit13）；反色组唯一inverse1，文件只TEX3P首项1→-1（固定三字符字段），两on文件SHA43456ba264d9f397dcd4651447f390a49a2228eb12615d4584888355bffe2a0b及7140454e644b52fe145d90270e674d9191d6bbb65053434e0d7f1b3c0bee30e5。两个真正改动状态Cancel后整份3539字节恢复旧基线，其余字节包括引用声明不变。

两个直接Cube拥有PID93593/96562正常0/86.752、96.687秒，三拥有实例皆<120/180，无强停/退出后应用查询/空白新实例，源/工作输入哈希mtime保持，六个所属偏好常规文件归档。一次只读旧UI缓存文件名FileNotFound留存，已枚举正确缓存替代，不冒充目标异常。前述成功只是直接Cube属性控制，不覆盖原项目加载失败；实际深度混合/颜色像素、Surface coloring启用、科学重建及动态回滚调用追踪仍未完成。

56项限定核对通过，6阶段文件/7次原生UI（含一次未就绪上下文）。原originalProjectResourceReadinessPredictionPass/originalAllPlannedStagesExecuted=false；后登记selectedCubeControlPredictionsPass与两次Cancel整基线相等true。新增18原生ID至1259，raw1118/not_loaded141，经141原生按ID补读得到明确派生完整快照，raw不改。旧1241记录/12问题头、原件257、107历史manifest/20608文件、78稳定样本保持。扩展VF17/59/138/139/143五条、其余249保持，入口级待核114→112；254功能/32权威，计划10完成19未完成。Codex自验；dsh/S08与完整Study待，无产品实现。

[56项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-210600-s04-isosurface-properties/isosurface-properties-crosscheck.json) / [原资源就绪失败](../../output/vesta-study/20261108-113622/reviews/20261109-210600-s04-isosurface-properties/original-resource-readiness-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-210600-s04-isosurface-properties/README.md)。


## R109 体数据重开路径与打开方式控制（2026-11-09）

R109不重做已有函数study，复用readVESTAfile0xa185c（旧R1/4905指令）、check_format0x8d83c、IO::toAbsolutePath0x287f0、read_density_data0xbb5b4与readCUBEfile0xbeb7c五份已存在原生记录。IMPORT_DENSITY分支实际先在a38f4做格式识别、a3904转绝对路径、a391c拼路径/倍率描述符，a3934调用密度读取器。原生u8地址4fa6b3=32，拼接分隔符为空格；不可凭猜测写成换行。已有路径函数对前导/提前返回，不是原生调用追踪。

现有read_density_data返回493条指令/size1972、body_ranges不连续，仍带一个未解析间接跳转；虽然函数级coverage显示1/1完整，不能据此称整个Cube分支已覆盖。check_format在8dc04/8dc08比较little-endian65627563（cube）、8dc18转8e090返回11。密度分发bbb40减10，bbb54基PC=bbb64，bbb58读u16；本轮原生读取50fd12=113，故Cube目标bbb64+113×4=bbd28。按事前上限12指令/遇首个bl停止的计划，五次原生instruction-show取得bbd28..bbd38，设置栈参数后调用readCUBEfilebeb7c；这五个地址确实不在旧493条body中、原生报告function=None。已补到首个调用，未声称完整read_density_data或动态调用/文件访问链都已恢复，原文件和工具未修补。

随后事前登记两个有区分度、只读的恢复预测。绝对引用副本只把R98项目IMPORT_DENSITY中的相对文件名替换为拥有目录中同哈希Cube的绝对路径，其余字节精确保持；原生确认absolute-density.vesta标题/4Å晶胞，实际仍0polygons/0vertices，正16/16恢复预测false。另开空白原版实例，通过真正GUIFileOpen选取与R98整字节相同的相对引用项目和相邻Cube；原生先证空白窗口VESTA，后证manual_open-density.vesta标题/晶胞，结果同为0/0，第二个正恢复预测false。与R108同Cube直接打开16/16的既有控制对照，仅在这些成对上下文中，相对路径写法与启动argv打开路线都不是观察失败的必要条件；唯一读取失败原因仍未隔离，不能推广为所有IMPORT_DENSITY或所有VESTA项目失效。未开Properties、未编辑/保存/发送IPC，更未覆盖冻结旧失败。

两个拥有PID1595/1715均正常0，25.277/170.985秒，后者120目标超50.985秒、两者均<180；所有功能native读取结束在180前，退出后原PID守卫跳过应用查询。手动打开时typeText和paste后的GoToPath实际值丢失下划线，Return仍停在路径sheet、未打开错误文件；最终setValue原样填入后核对正确路径，才点击Open并取得指定项目原生后像。输入问题及120目标失败留存，未声称其唯一原因。两输入哈希/mtime的0.1秒抽样保持，规范源及两资源哈希/mtime保持，四个拥有偏好常规文件归档。

34项限定核对通过，两个真实只读控制/3次原生UI；bothPositiveRestorePredictionsPass=false、all120GoalPass=false，bothRegisteredControlsExecuted/all180GoalPass=true。新增10原生ID至1269，实际raw1126/not_loaded143，143次原生按ID补读组成显式派生完整快照，raw不改。旧1259记录/12问题头、原件257、108历史manifest/21293文件及78稳定样本保持。只扩展VF17，其余253条保持；入口级待核112不变，254功能/32权威，计划10完成19未完成。Codex自验；dsh/S08及完整Study未完成，无产品实现。

[34项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-212400-s05-import-density-readback/import-density-crosscheck.json) / [事前控制](../../output/vesta-study/20261108-113622/reviews/20261109-212400-s05-import-density-readback/import-density-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-212400-s05-import-density-readback/README.md)。


## R110 截面标志与绝对值关联范围（2026-11-09）

R110以直接打开的VS67 signed4×4×4 Cube（1519字节/SHA04a77cbbe05ff93df31612dfc25d5a38c8f69bb4d1fd853277e63aa098b13f68）研究Sections三个控件，未用仍失败的VESTA资源重开作正上下文。三份同名visibility-volume.cube独立副本/短偏好，每组原生PID/标题/16等值面多边形16顶点及lsof监听就绪后才发baseline，三个3539字节序列化基线均精确等于冻结R98 SHA9f60f6acdb72c9641e0ddbee3942c870b585479bce52b50ecb7c7c45caf31838，Cube输入哈希与VESTA基线哈希分别核验。

DlgSectionPage目录19项。Absolute handler main79204在7922c写dialog+48a bit0，并先调用SetSatulationLabel7b7e0(arg1)；Recursive handler7924c在79260/64写bit1；DrawContour handler79270在79284/88写bit6。UpdateCanvas7baa4在7bbb8写Scene+354半字，Scene内XStyle+228，故字段+12c。参数读取器7b6fc也从Scene354回读dialog48a。复用R97 writer b8a34读取XStyle+12c、打印SECTS第一项，b8d40读取XStyle+b8/+bc、打印SECTP最小/最大值。7bba0/7bb80分别写Scene2e0min/2e4max，对应XStyle+b8/+bc。

Absolute不能按只改flag预期：652字节/163指令SetSatulationLabel在普通分支使用数据绝对最大值6及当前百分比0/100，预计把物理下限-6改成0、上限仍6；Scene+5f0==2有另一分支，其输入谓词未单独追踪，事前明确若实际不符保留失败，不后验扩大预期。本轮实际原生/GUI绝对值单选1、物理Max6/Min0，on文件同时SECTS32→33、SECTPmin-6→0，SHAf1cb9c8a97f05c8dcc021a7950963d674a95cb99ce7909f123abf4aaa1a9a74c，整份文件仅这两处变动，耦合先验通过。

独立Recursive和DrawContour组各只有该原生checkbox为1，其余两项0/Preview1/物理range[-6,6]。on文件只SECTS32→34或96，其余字节精确保持，SHA459ff76968d4813cdba5fc0743ec8de73083d919e5c996598d983efebc53db18与04223267a517009fe35bba26683dabf62cb5701151e63d75f9b3e00845a26344。三组均在真正改动状态Cancel，三个after_cancel整字节恢复3539字节冻结基线。静态CopyPropertiesTo ba1cc/1d0半字包含+12c，ba1d4/1d8从+b8复制16字节，包含min/max；与所测恢复相符，未声称动态调用追踪或所有Properties字段都能回滚。

七个函数已知范围连续：72/18、36/9、36/9、Update1060/265、Reader216/54、SetSatLabel652/163、getminmax56/14（字节/指令）；保留Update虚拟尾跳的间接告警，不推广为动态科学算法验收。三个拥有PID7830/8226/8479正常0、95.182/84.336/82.167秒，皆<120/180，所有native功能动作在180前；退出后守卫直接跳过应用查询，三输入哈希/mtime及0.1秒抽样保持，六个所属偏好文件归档。所测只含checkbox/UI数值、序列化及选定Cancel，不含截面像素、递归颜色周期、等高线数值/科学绝对值计算、DataType2谓词唯一原因或资源读回修复。R108/R109失败保持。

71项限定核对通过、9阶段文件/9原生UI，全部事前整文件预期及三次Cancel通过。新增26原生ID至1295；raw1140/not_loaded155，经155次原生按ID补读组成明确派生完整快照，raw保持原样。旧1269记录/12问题头、原件257、109历史manifest/21897文件、78稳定样本保持。扩展VF60/140/141/142/143五条、其余249保持；入口级待核112→109，254功能/32权威、计划10完成19未完成。Codex自验；dsh/S08与完整Study未完成，无产品实现。

[71项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-214400-s04-section-properties/section-properties-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-214400-s04-section-properties/section-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-214400-s04-section-properties/README.md)。


## R111 Properties Preview 隔离与应用（2026-11-09）

R111补足 Properties Preview-off 隔离与 Preview-on 应用的单相轴标签控制。先读已有 SetPreviewMode main100021264（308字节/77指令）、CopyParam 100021398（220/55）、OnPreview 1000204a0（520/130）及轴标签 handler 100048c88（176/44），逐4字节连续覆盖已知范围，不重复同一study。SetPreviewMode关闭分支在两个Scene引用相等时调用虚拟clone(mode2)，随后CopyParam把基线参数送回活跃Scene；开启分支CopyParam(编辑Scene,活跃Scene)后把编辑引用切回活跃Scene。CopyParam从Scene+228的XStyle边界调用CopyPropertiesTo，OnPreview把各页面Scene引用指向选定Scene。此为静态控制流解释，未捕获动态指针/调用追踪。

两份独立R66规范SiO6工程副本，均5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。各组先原生PID/标题/7atoms6bonds1polyhedron、所属短Unix监听和整文件baseline门，再开General：Show compass1、Show axis labels1、Show edges1、Preview1。关闭Preview后再取消轴标签，原生对话框为Preview0/labels0，但IPC保存的活跃Scene仍整份等基线、COMPS1；两组相同。第一组直接Cancel后亦整份基线。第二组重新开启Preview，原生Preview1/labels0，on_apply文件唯一COMPS1→2，5144字节/SHA73ae84b72c4ba06f2d86d2c8faf7edc35935a5edf7f51a76286383a0223a7f1b；真正已应用改动后Cancel整份恢复基线。七个新阶段输出全部符合事前整文件预期。未据此推广所有属性的隔离/回滚，R103边、R104标记、R105键规则的保留字段失败均保持。

拥有PID15614/15762正常0退出，66.527/81.938秒，均<120/180；无追加相、Vectors、SaveDefault、强停或退出后应用查询。两输入/规范源哈希mtime及0.1秒抽样保持，四个所属偏好文件归档。50项限定核对通过，7阶段文件/7次原生UI；不含轴标签像素、动态Scene指针追踪、所有Properties字段/预览/回滚或体数据资源修复验收。

新增14原生ID至1309，raw1149/not_loaded160，经160次原生按ID补读形成显式派生完整快照；raw不修改。旧1295记录/12问题头、原件257、110历史manifest/22656文件与78样本保持。仅扩展VF55/143，其余252保持；入口级待核109不变，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08及完整Study仍待，未进入产品实现。

[50项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-220943-s04-properties-preview/preview-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-220943-s04-properties-preview/preview-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-220943-s04-properties-preview/README.md)。


## R112 晶胞线宽与红通道（2026-11-09）

R112补General晶胞线宽与颜色字段。目录原生搜索返回34项（含GeneralPage2），本轮只研究GeneralPage三个尚无旧study的handler：OnKillFocus main100048ef8为44字节/11指令，OnColour100048b38及OnSpinCtrl100048b30各8/2；都把event+28的ID送入UpdateCanvas10004a33c。复用完整constructor、GetGeneralParam、UpdateCanvas、XStyle写入器和CopyPropertiesTo，未重复相同study。constructor49320绑定dialog+474浮点线宽/ID2b61；495b8绑定dialog+488 RGB数组、ID2b5c/2b5d、3组件。UpdateCanvas4a3cc/3d0写Scene22c线宽，4a3e0/3ec/3f8写Scene230/231/232三颜色字节；Scene的XStyle基址228，分别对应+4和+8..a。库b8f28..38从这些字段读取，500f49为UCOLP、500f50为固定宽度格式；CopyPropertiesTo ba114..130复制线宽及RGB。静态映射与运行序列化相互支持，无动态callback跟踪。

两份独立R66规范SiO6副本5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。原生各证自有PID/标题/7atoms6bonds1polyhedron、短监听及整文件baseline门，General初始线宽1、RGB0/0/0、Preview1。width组仅setValue线宽2后Tab失焦，原生首四文本字段2/0/0/0，保存只UCOLP第三项1.000→2.000（SHAac1f6767380611bbfc7f9d9bf0bac7fca72c646ef7ab74044f7e9b5dbbd4aa30）；red组仅红值64后Tab失焦，原生字段1/64/0/0，保存只UCOLP第四项0→64（SHA4dd3c6c855bc488a509a6cffc66bdcbf22c84e17c6390865b685973436c142e1）。每组均从真正改动状态Cancel，after_cancel整份恢复规范基线，六个阶段文件均符合事前整文件预期；没有强制点击禁用控件、SaveDefault或更改相。

拥有PID18574/19296正常0退出，用时74.861/61.892秒，均<120/180；所有原生功能阶段在180前，退出后死PID守卫跳过应用查询，未重启空白实例。规范源/两个输入哈希mtime及0.1秒抽样保持，四个所属偏好文件归档。56项限定核对通过、6阶段文件/6原生UI。本轮只测线宽2与红值64的UI/序列化/Cancel，不包含绿色蓝色非零、范围/非法输入、颜色选择器、绘制像素、动态callback或所有属性回滚；R103/104/105保留字段反例不变。

新增16原生ID至1325，raw1152/not_loaded173，经173次原生按ID补读形成显式派生完整快照，原raw不改。旧1309记录/12问题头、原件257、111历史manifest/23367文件和78样本保持。扩展VF55/143/152三条，其余251保持；入口级待核109→108，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08和完整Study仍待，未进入产品实现。

[56项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-221913-s04-cell-line-properties/cell-line-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-221913-s04-cell-line-properties/cell-line-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-221913-s04-cell-line-properties/README.md)。


## R113 原子标签类型与偏移（2026-11-09）

R113研究Atoms标签类型和z偏移。复用R104目录、GetAtomsParam、只有35条可达指令/140字节的内置UpdateCanvas记录，以及R104原生objdump分页offset40000/limit12000；新提取2e148..2e420共182条连续指令，保持原内置分支覆盖缺口和当年objdump忽略range失败，不重跑同一study或修补工具。新study为OnLabelType main10002b594（144字节/36指令）、constructor10002b660（8432/2108）、OnKillFocus10002b634（44/11），各已知范围连续，保留间接调用/尾跳限制。

OnLabelType在2b5c4按choice是否非零设置Scene+5b0的bit22。constructor2d094为2bd0/两选项，2d4fc把bit22作为当前selection；偏移2d180绑定dialog4a8、2d190为ID2bd1。GetAtomsParam从Scene280回读dialog4a8。2bd1-2bc0=17，对应原生u8地址1004d73b9=137，故dispatch base2e194+137×4=2e3b8；原页2e3b8..3c0将偏移写Scene280，即XStyle+58。原生writer b8f78取DISPF bit22、b8f7c读取XStyle54/58、b8f88取bit23，格式500f76为LABEL %i %5g %6.3f %i。CopyPropertiesTo ba154/158复制58，mask02422800含bit22而不含bit23；这解释本轮选定标签类型/偏移可恢复与R104 markbit23保留的差别，仍非动态调用跟踪。

两份独立R66规范SiO6工程副本，5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56；原生每组证自有PID/标题/7atoms6bonds1polyhedron、短监听及整文件baseline门。Atoms标签类型初始Names of sites、offset1、Preview1。type组实际展开两选项Names of elements/Names of sites，选择第0项Names of elements；原生on为元素名称/offset1，保存仅DISPF37753794→33559490（清bit22）及LABEL首项1→0，SHA899850b2c5b59fd6754ff24796bc80e02335b6a39f52464fe931175976571c18。offset组保持Names of sites，将z偏移setValue1.5后Tab失焦，原生offset1.5，文件唯一LABEL第三项1.000→1.500，SHA74975df24e7e1efcdebf78a0882d9cfa5dae4fbe6e31fd692cb178a74e57edbe。两组均从真正改动状态Cancel，输出整份恢复规范基线，六个阶段文件均符合事前整文件预期。

两组Atoms初始Show as balls/Show as displacement ellipsoids都为0，修改标签后亦为0；这个已知异常原样记录，不强制更改radio，也不把正7/6/1模型或标签控制通过当Atoms模式正确性验收。Hide nonbonding/Mark enantiomorphic为0不动，Preview1。拥有PID21226/21371正常0退出，74.568/67.464秒，均<120/180；所有原生功能动作在180前，退出后死PID守卫跳过查询，规范源/工作输入哈希mtime及0.1秒抽样保持，四个所属偏好文件归档。61项限定核对通过，6阶段文件/6原生UI。未验标签绘制像素、动态callback、偏移边界或所有LABEL/Properties回滚；R104 markbit23及其它保留字段失败不变。

新增16原生ID至1341，raw1140/not_loaded201，经201次原生按ID补读形成显式派生完整快照，原raw不改。旧1325记录/12问题头、原件257、112历史manifest/24120文件与78样本保持。扩展VF56/143/157三条，其余251保持；入口级待核108→107，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08及完整Study仍待，无产品实现。

[61项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-222917-s04-atom-label-properties/atom-label-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-222917-s04-atom-label-properties/atom-label-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-222917-s04-atom-label-properties/README.md)。


## R114 原子 stacks 与 slices（2026-11-09）

R114研究Atoms tessellation的stacks/slices，未研究材质或Bonds控件。复用R113完整constructor、R104 GetAtomsParam/内置不完整UpdateCanvas及原生完整182指令分页、R113失焦handler、XStyle writer和CopyPropertiesTo。新原生OnSpinCtrl main10002b62c仅8字节/2指令，把event+28的ID尾跳到UpdateCanvas2e148。constructor2bab0绑定dialog47c、2bac0为ID2bc4/stacks、2bac8为下限2；2bb14绑定dialog480、2bb24为ID2bc5/slices、2bb2c下限3，栈上界均10000。这些只说明静态构造器，不当上下限运行验收。

2bc4/2bc5相对分发表基ID2bc0的索引4/5；新原生u8地址1004d73ac/ad均为0，故跳转2e194。复用原页2e198/1a0将dialog47c写Scene268，2e1a4/1ac将dialog480写Scene26c；Scene的XStyle基址228，故对应40/44。writer b8b64读取40/44，500e3b为ATOMP、500e42格式为%3i %3i %3i %3i %4.1f %3i；CopyPropertiesTo ba144/148复制40..4f，包括这两个整数。保持原内置Update缺少分支和旧objdump忽略range失败，不重跑相同study、不修补工具或声称动态调用追踪。

两份独立R66规范SiO6工程，5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。原生各证自有PID/标题/7atoms6bonds1polyhedron、短所属监听及整个基线序列化门。Atoms初始stacks24/slices24/Preview1，两个样式radio均0；本轮保持异常原样，不强制修复也不据此验收模式。stacks组只setValue32后Tab，原生32/24，文件仅ATOMP首项24→32，SHA7a42d2ea6d435a8642c1246c0fe033cac3043bda7fbfe8eafb85b1dd12c38f27；slices独立组只第二字段32后Tab，原生24/32，文件仅ATOMP第二项24→32，SHA0bd926b86141fcd4c375f562589ed64464ceca19fdd33363c51f9888d87fd9f8。每组真正改动状态Cancel后，整份5144字节恢复规范基线；六个阶段文件均符合事前整文件预期。

拥有PID22918/23312正常0退出，72.749/65.879秒，均<120/180；所有原生功能动作在180前，无Vectors/追加相/SaveDefault/强停/退出后应用查询。规范源/工作输入哈希mtime及0.1秒抽样保持，四个所属偏好文件归档。55项限定核对通过、6阶段文件/6原生UI。只验所选32值UI/序列化/Cancel，未验实际球面网格/像素、材质、Bonds参数、范围/非法值、性能/科学算法或通用属性回滚；原R103/104/105保留字段反例保持。

新增15原生ID至1356，raw1151/not_loaded205，经205次原生按ID补读形成显式派生完整快照，原raw不改。旧1341记录/12问题头、原件257、113历史manifest/24954文件和78稳定样本保持。扩展VF56/143/153三条，其余251保持；入口级待核107→106，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08与完整Study未完成，产品实现未开始。

[55项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-223928-s04-atom-tessellation/atom-tess-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-223928-s04-atom-tessellation/atom-tess-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-223928-s04-atom-tessellation/README.md)。


## R115 原子材质与百分比换算（2026-11-09）

R115研究Atoms材质红通道与shininess百分比，保持单相、无Vectors/追加相/SaveDefault或实现。新原生vxMaterialTool::Create main1000155d8为796字节/199连续指令、OnSpinChanged1000159b8为52/13，目录返回2函数；先核1356旧记录未有Create study再调用，未重复相同研究。复用Atoms constructor/reader/失焦与spinhandler、不完整内置Update和原生182指令分页、XStyle writer/copy。

Atoms constructor2b898/89c把dialog49d RGBA和4a4 rawshininess交共享MaterialTool，事件ID2bc0/2bc1。Create1562c/630拼float32位模式3fa3d70a（约1.2799999713897705），15638将raw除以该常量、转double后加0.5、fcvtms取floor生成界面整数百分比。OnSpinChanged159bc读取spin缓存double，159c0转整数，159d4按同一float32常量相乘，159dc写回raw指针。事前Python按相同float32舍入计算40%，得到固定三位字符串51.200；不把百分比数值直接写进ATOMM。Create给spin静态范围1..100，未做范围/非法值运行验收。

新原生表字节1004d73a8/a9均78，2e194+78×4=2e2cc；原页2e2d8/2e2e4写Scene3cc/3d0，即XStyle1a4/1a8。writer b91c0..cc读RGBA、b91ec读shininess，ATOMM按所选原始格式输出；CopyPropertiesTo ba30c/310和ba33c/340分别复制1a4和1a8。此为静态控制/字段说明，无动态callback跟踪。

两份独立R66规范SiO6工程副本5144字节/SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，原生各证自有PID/标题/7atoms6bonds1polyhedron、短监听及整文件baseline门。Atoms材质初始Specular204/204/204、shininess20%、Preview1。red组setValue红128后Tab，原生128/204/204/20，文件仅ATOMM红204→128（SHA bd8369495e389f620fb411d8939de972b76f700fef8d6dbbb716d14b1f2ce781）；shine组独立setValue40后Tab，原生204/204/204/40，文件唯一ATOMM原始shininess25.600→51.200（SHA1b61525a82ca81468f01d04e1d87dbb2e06064d79e1b3bcebc7778978ea76dc0）。两组真正改动状态Cancel后整份恢复规范基线，六个阶段文件符合事前整文件预期，float32换算先验通过。

拥有PID25034/25438正常0退出，71.869/70.911秒，均<120/180；所有原生功能动作在180前。两个初始模式radio均0仍原样保留，不强制改动或声称模式正确。规范源/工作输入哈希mtime及0.1秒抽样保持，四个所属偏好文件归档，无退出后应用查询。59项限定核对通过、6阶段文件/6原生UI。未验其它颜色/通道/页面、范围/非整数输入、实际材质光照/像素、科学数值或通用Properties回滚；R103/104/105保留字段反例不变。

新增17原生ID至1373，raw1168/not_loaded205，经205次原生按ID补读形成显式派生完整快照，raw保持原样。旧1356记录/12问题头、原件257、114历史manifest/25796文件与78样本保持。只扩展VF56/143/153三条，其余251保持；入口级待核106不变，254功能/32权威、计划10完成19未完成。Codex自验，dsh/S08及完整Study未完成，未进入产品实现。

[59项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-224835-s04-atom-material/atom-material-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-224835-s04-atom-material/atom-material-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-224835-s04-atom-material/README.md)。


## R116 Bonds 材质与规则隔离（2026-11-09）

R116以两个独立R66规范SiO6副本研究Bonds材质，先读R105 GetBondParamFromS/完整UpdateCanvas、共享R115 MaterialTool换算以及原生writer/copy；新study为Bonds constructor main10002ea84（3500字节/875连续指令）和OnTextCtrl2ea7c/OnColour2ea6c/OnSpinCtrl2ea74（各8/2），未重复相同study。constructor2ecb0/4绑定dialog414 RGBA与418 rawshininess，2ecbc/2ecc0给事件ID2c24/25，调用共享155d8。UpdateCanvas2fc00按ID减2c24、比较2，2fc08转2fd08；2fd10/1c写Scene3d4/3d8，即XStyle1ac/1b0。2c28样式分支在2fd50调用SetBondStyle，半径/线宽/颜色有其它分支，不把Atoms父层绑定直接当Bonds证据。

writer b9224..30读XStyle1ac..1af RGBA、b9248读1b0，501021为BONDM；CopyPropertiesTo ba314/318及ba344/348复制两材质字段。共享float32常量3fa3d70a约1.2799999713897705，界面floor(float(raw/constant)+0.5)，反向integerpercent*constant。事前float32预期50%输出64.000，非将50直接当raw。静态控制链说明未声称动态callback或SBOND调用追踪。

两份5144字节规范源SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，各在原生自有PID/标题/7atoms6bonds1polyhedron及所属短监听后，通过整文件baseline门。Bonds初始specular255/255/255、shininess100%、Preview1，唯一选中Bicolor cylinder。red组setValue红128后Tab，原生128/255/255/100，文件仅BONDM红255→128（SHA24ae9079bfcdb02650a0a4888c4473920cd9e9bda343193fd53b0fe2c6ef19ed）。shine独立组setValue50后Tab，原生255/255/255/50，文件唯一BONDM raw128.000→64.000（SHA9cabd3113a123f112fdfe9616835f62be975c50489f61b59228c289557f9e637）。六个阶段文件符合事前整文件预期，完整SBOND块全部保持，样式也保持Bicolor；两个真正改动状态Cancel均恢复整份规范基线。此成功不覆盖R105样式改动后SBOND保留的Cancel失败。

拥有PID31735/32425正常0退出，98.987/76.028秒，均<120/180；所有原生功能动作在180前。规范源/输入哈希mtime与0.1秒抽样保持，四个所属偏好文件归档，无Vectors/追加相/SaveDefault/强停/退出后应用查询。65项限定核对通过、6阶段文件/6原生UI。只验两所选材质控制的UI/序列化/恢复和SBOND不变，不含实际光照像素、其它通道/范围/非整数输入、通用Bonds或Properties回滚。

新增16原生ID至1389，raw1182/not_loaded207，经207次原生按ID补读形成显式派生完整快照，raw保持。旧1373记录/12问题头、原件257、115历史manifest/26658文件与78样本保持。仅扩展VF57/143/153，其余251保持，入口级待核106不变，254功能/32权威、计划10完成19未完成。头部更新从before快照核对实际旧文本后生成，并以本轮round/provenance/coverage核对，迁移旧R115当前头部guard以避免下一轮叠加过期要求。Codex自验；dsh/S08及完整Study未完成，无产品实现。

[65项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-230440-s04-bond-material/bond-material-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-230440-s04-bond-material/bond-material-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-230440-s04-bond-material/README.md)。


## R117 Bonds stacks 与 slices（2026-11-09）

R117以独立R66规范SiO6副本研究Bonds stacks/slices，直接复用R116完整constructor和Text/Spin handler、R105 reader/完整UpdateCanvas、原生BONDP writer及CopyPropertiesTo，不重复同一函数study。constructor2eb50令x20=dialog40c，2eed8给stacks事件2c26，2eedc把该指针传入spin控件，静态min1/max10000；slices在2ef30绑定dialog410，2ef40事件2c27，min3/max10000。reader从Scene284/288回读40c/410；Update2fc0c将ID减2c26、与2比较，小于2时落入2fc1c，2fc28/34写Scene284/288，即XStyle5c/60。后续所选分支只重建显示列表，样式2c28有另外的SetBondStyle分支；这是静态控制说明，无动态callback或规则更新跟踪。

原生writer b8bbc读取XStyle5c/60，500e5d是BONDP，按其前两整数写出；CopyPropertiesTo ba15c/160复制5c..6b，涵盖两字段。两份5144字节规范源SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，各在原生自有PID/标题/7atoms6bonds1polyhedron、所属短监听后通过整文件baseline门。Bonds初始stacks1/slices16、Preview1、唯一Bicolor cylinder选中。

stacks组只setValue1→2后Tab失焦，原生2/16，文件仅BONDP第一项1→2（SHA304ae948b68203cf575595586fc8208e64b304a5323d72c2a37ef2b1e9400d54）；slices独立组16→24后Tab，原生1/24，文件仅第二项16→24（SHAee3694e7b8a60a4ba2c6c38208d7660f72955f6978d6f4c2522b5b881c8a6c24）。六份阶段文件符合事前整文件预期，完整SBOND块全部保持，Bicolor样式未变；两个真正改动状态Cancel后整份恢复规范基线。此成功不替代R105样式改动后规则保留的Cancel失败。

拥有PID46915/47294正常0退出，70.527/72.120秒，均<120/180，原生功能动作均在180前。规范源/两个输入哈希mtime和0.1秒抽样保持，四个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。63项限定核对通过、6阶段文件/6原生UI。仅验所选resolution值的UI/序列化/Cancel与SBOND不变，未验实际网格、像素、性能、边界/非法输入或通用回滚。静态构造器范围不当运行范围验收。

新增12原生ID（全为6UI+6IPC，没有新static study）至1401；raw1194/not_loaded207，经207次原生按ID补读形成显式派生完整快照，原raw保持。旧1389记录/12问题头、原件257、116历史manifest/27515文件与78样本保持。仅扩展VF57/143/153，其余251保持，入口级待核106不变，254功能/32权威、计划10完成19未完成。before快照匹配后更新当前头部，验证读取本轮实际round/provenance/coverage；原R115头部失败和修订包仍保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。

[63项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-231655-s04-bond-tessellation/bond-tess-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-231655-s04-bond-tessellation/bond-tess-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-231655-s04-bond-tessellation/README.md)。


## R118 Bonds 半径与线宽（2026-11-09）

R118研究Bonds圆柱半径及线宽。复用完整constructor、reader、UpdateCanvas、Text handler、XStyle writer/CopyProperties和已有完整save_data项目保存函数。新增Scene/Crystal两个半径与两个线宽helper的原生study：Scene81294写全局29c并遍历各相调用6b3ac，Scene812f0写298并调用6b470；Crystal COW路径分别在6b3f8/6b4bc写规则+34/+38。保留未解虚拟clone/destructor及cold边，不把内建callee落入下个函数的元数据当实际调用。VESTA名writer正则零结果仅是有限负搜索，实际save_data已有证据，未重复study。

constructor2f428绑定dialog424、2f438事件2c29；2f598绑定420、2f5a8事件2c2b。Update2fd6c/2fc98调用对应Scene半径/线宽import，比较同值时跳过更新。save_data ae7e4/e8读取当前相规则向量240/238，ae82c读取规则34/38，ae850依次传给4ff628完整SBOND格式，落在第11/12个token（零基10/11）；这与只含前10项的default-style SBOND writer不同。全局writer b8ba4读取XStyle70(linewidth)/74(radius)，写BONDP第三/四项；CopyProperties ba16c/170复制两全局字段。静态路径与观察相符，无动态callbacktrace、虚拟目标解析或Cancel唯一因果证明。

两份规范SiO6副本各5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，独立短偏好目录。原生自有PID/标题/7atoms6bonds1polyhedron/所属监听后，整文件baseline门通过；Bonds初始radius0.25/width2.0、Preview1及唯一Bicolor1。半径组只setValue0.5并Tab，BONDP半径及唯一SBOND半径同时0.250→0.500；线宽组只setValue3.0并Tab，BONDP线宽及SBOND线宽同时2.000→3.000，其他字节保持。两个真正改动状态Cancel都仅恢复全局BONDP，SBOND规则仍保留编辑值。六份阶段文件全部符合事前精确预期；两份after_cancel都不等于完整规范基线，不能表述为整体回滚成功。R105旧样式Cancel失败继续保留。

67项限定核对、6阶段文件/6原生UI通过。自有PID54746/56113均正常0退出，74.559/65.646秒，均<120；原生功能动作在180前。源/输入哈希及mtime、0.1秒抽样不变，四个所属偏好文件已归档，无Vectors/追加相/SaveDefault/强停/退出后应用查询。未验RGB、实际几何/像素/性能、多规则/多相、边界非法值或通用回滚。新增18原生ID（4helper study+2search+6UI+6IPC）至1419，raw1201/not_loaded218，经218次原生补读形成显式派生完整快照；1401旧记录、12问题头、原件257、117历史manifest/28357文件及78样本保持。

只扩展VF57/143/158，其余251记录保持；VF158为所选半径/线宽partial，RGB及更广语义待核，精确入口级待核106→105并不代表全部未完成功能。254功能/32权威文档、计划10完成19未完成。当前头部以before快照匹配并由实际provenance/coverage验证；R115旧头部失败/修订包保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。

[67项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-232635-s04-bond-radius-width/bond-size-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-232635-s04-bond-radius-width/bond-size-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-232635-s04-bond-radius-width/README.md)。


## R119 Bonds RGB三通道（2026-11-09）

R119独立验证Bonds的R/G/B三通道127→64。复用既有完整constructor、reader、UpdateCanvas、Text/Colour handler、XStyle writer/copy以及项目save_data；旧原生search已定位Scene81234及Crystal6b2e8，因此没有重复search。新增两个helper完整已知范围study：Scene96字节24指令，在81234/38把输入32位值写入全局2a0并在8126c遍历相调用Crystal；Crystal196字节49指令，COW路径6b330/334把输入32位写入规则3c。第四拷贝字节的含义、虚拟clone/destructor及cold目标仍未解决，不能扩张为alpha契约或动态callbacktrace。

constructor2f694绑定dialog428、2f69c/6a0注册2c2c/2d；Update2fcb8/fcc8/fcd8写Scene2a0/1/2，比较RGB后在2fcf4调用Scene import。同值分支不调用该helper；这是静态路径，不证明本次具体回调或Cancel唯一因果。项目writer ae838/3c/40读取规则3c/3d/3e，按ae848/4c参数排布写SBOND零基token12/13/14；全局BONDP writer b8bb0/b4/b8读取XStyle78/79/7a，CopyProperties ba174..188包含三全局字节。

三个独立规范SiO6输入各5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56，一相一规则，原生自有PID/标题/7atoms6bonds1polyhedron/所属监听及整文件baseline门均先通过。每组只用setValue+Tab把一个通道127改64；原生UI保持另外两通道127及Bicolor1，保存文件仅相应BONDP和SBOND两处通道改变。三个真正改动状态Cancel均恢复全局RGB127/127/127，却保留规则的相应64；三份after_cancel整体基线相等均false，九份阶段文件符合事前整文件预期。R118半径/线宽和R105样式的旧保留规则反例保持。

91项限定核对、9阶段文件/9原生UI通过。PID59511/62448/62733正常0退出，72.252/69.671/69.951秒，均<120，原生功能动作均在180前。源及三个输入hash/mtime、0.1秒抽样保持，六个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。仅验三通道所选值与UI/持久化，颜色选择器、实际渲染、第四字节、多规则/多相、非法边界、通用回滚仍待核。

新增20原生ID（2helper study+9UI+9IPC），完整核验派生快照1439；原生raw1220/not_loaded219，按ID补读219次形成显式并集，raw和canonical均未重写。1419旧记录、12问题头、257原件、118历史manifest/29248文件及78样本保持。仅扩展VF57/143/158，其余251保持；VF158所选半径/线宽/RGB partial，精确入口级待核105不变，不能称所有功能已验。254功能/32权威，计划10完成19未完成；before匹配头部并按实际provenance/coverage复核，R115旧失败保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。

[91项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-234135-s04-bond-rgb/bond-rgb-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-234135-s04-bond-rgb/bond-rgb-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-234135-s04-bond-rgb/README.md)。


## R120 Polyhedra 边线RGB（2026-11-09→10）

R120独立验证Polyhedra边线R/G/B三通道180→64。先查1439个既有native记录，确认所选constructor/Colour/Spin/Killfocus四个函数尚无native_function_study，再补读完整已知范围3572/893、8/2、8/2、44/11（字节/指令）。复用R107完整UpdateCanvas540/135、reader572/143和已有XStyle writer/CopyProperties。constructor609d0绑定dialog470，609d8/9dc事件2c8e/2c90，609e0设3通道；Colour/Spin尾调用6110f4，Killfocus在5fd30调用同一Update。

Update61124将ID减2c8c后构造bit，61144的mask0x14选择2c8e/2c90，61150..170把dialog470/471/472写Scene2b0/2b1/2b2，即XStyle88/89/8a。reader61040/44从Scene2b0回读dialog470。writer b8c04/08/0c读XStyle88/89/8a，经格式500e8c写POLYP零基token3/4/5；CopyProperties ba19c..1b0复制三个全局字节。所选分支没有调用Bonds的规则更新helper；这是静态路径，不能声称动态callback、写入actor或唯一回滚因果已跟踪。既有virtualcall/tail及builtin警示保留。

三份规范一相SiO6输入各5144字节，SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。每组原生自有PID/标题/7atoms6bonds1polyhedron/所属短监听和整文件baseline门均在getApp前通过。Polyhedra初始Showedges1/Preview1/width1.0，边线180/180/180；每组仅setValue+Tab把一个边线通道180改64。原生控件与阶段文件匹配，仅POLYP相应通道变化，完整SITET及其他字节保持；三个真正改动状态Cancel均恢复整份5144字节规范基线。九份事前整文件预期通过，不替代R105/R118/R119 Bonds规则保留等旧失败。

82项限定核对、9阶段文件/9原生UI通过；PID67039/67297/67686正常0退出，76.781/76.980/75.141秒，均<120，原生功能动作在180前。源及三个输入hash/mtime、0.1秒抽样保持，六个所属偏好文件归档，无Vectors/追加相/SaveDefault/强停/退出后应用查询。面RGBA、透明度、颜色选择器、实际渲染、边界非法输入/多相或通用回滚仍待核。新增22原生ID（4函数study+9UI+9IPC），完整派生1461；raw1220/not_loaded241，241次原生按ID补读构成显式完整并集。1439旧记录/12问题头、257原件、119历史manifest/30182文件、78样本保持。

仅扩展VF58/143/159，其余251保持；VF159为所选边线RGB partial，并补链R107已验edge/width证据，opacity/faceRGBA仍待核。精确入口级待核105→104不等于总未完成功能减少到104。254功能/32权威，计划10完成19未完成。头部以before匹配并按实际provenance/coverage核验，R115旧失败保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。

[82项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-235637-s04-polyhedra-edge-rgb/poly-edge-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261109-235637-s04-polyhedra-edge-rgb/poly-edge-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-235637-s04-polyhedra-edge-rgb/README.md)。


## R121 Polyhedra 全局透明度与面alpha（2026-11-10）

R121分别研究Polyhedra全局opacity与所选Si面alpha204→128。四个新增native study为Scene80ef4（244/61）、Scene80df4（256/64）、Crystal6acd4（184/46）、Crystal6ac00（212/53），均已知范围完整连续；复用R120 constructor/handlers、R107完整Update/reader、原生project save_data、CopyProperties、CopyParam及refcountedvectorassignment21474。没有重复旧study/search。保留builtin已知函数边界外错误fallthrough callee条目、虚拟clone/destructor/cold目标未解，不把它们当实际执行。

原生全局路径：constructor60070 dialog468/60080事件2c8b，Update611a8调用Scene80ef4；80f0c写Scene2b3，80f40遍历Scene5c0元素alpha8f，80fbc调用Crystal6acd4，6ad18写全部Crystal1c8位点alphaef。所选面路径：constructor605c8 dialog46c/2c8f/91，Update先在6121c/230/244写所选元素RGB8c/d/e，alpha变化时6129c直接写8f，再经612b8调用Scene80df4及Crystal6ac00；后者以位点d4匹配元素，6ac50写位点ec..ef四字节。

原始事前整文件预期漏记ATOMT联动。冻结原计划、expected文件、分析器及poly-opacity-crosscheck.json：69项检查中4项false，6整文件预期仅3通过/3失败，实际重算exit1保留。3个失败整文件分别是global/on、alpha/on、alpha/after_cancel；另外bothCancelwholebaseline断言失败。全部baseline、源输入及native调用正确，并非运行调用失败。

观察后另建显式characterization文件及独立分析器/79项核对，不能把它称事前预测。Global204→128实际只改POLYP首项、全部7个SITET alpha和2个ATOMT alpha；Cancel整份5144规范基线精确恢复。独立Si facealpha204→128实际只改Si1 SITET alpha及Si ATOMT alpha，O两层alpha与全局POLYP204保持；Cancel后所有SITET恢复204，却留下ATOMT Si alpha128。选中Si的原生UI始终正确，global组编辑时界面global/facealpha128/128，alpha组为204/128。

原生项目writer同时区分两层：ae908..914读取Crystal1c8位点ec..ef写SITET token6..9；af448/44c遍历Scene5c0，af484..490读取元素8c..8f写ATOMT token6..9，格式4ff85b。CopyParam213c4复制Scene5c0、213f8复制各Crystal1c8，经21474拷贝指针并增加引用计数；全局XStyle8b由CopyPropertiesba1b4/1b8复制。所选Update在Scene COW helper前直接写元素，符合共享快照提前变化的候选解释；实际指针别名、动态callback及唯一Cancel因果仍未证明。未修改原版或修复失败输出，也不据全局成功推翻选中alpha反例。

79项观察后限定核对、6阶段文件/6原生UI完成；原始false报告及exit1通过sha/mtime冻结保护。自有PID71101/71647正常0退出，74.390/77.164秒，均<120；所有原生功能动作在180前。源及两个输入hash/mtime、0.1秒抽样保持，四个所属偏好文件归档，无Vectors/追加相/SaveDefault/强停/退出后应用查询。仅一相SiO6所选值；faceRGB、颜色选择器、实际透明度像素、边界非法输入、多相或通用回滚仍待核。

新增16原生ID（4helper study+6UI+6IPC），完整派生1477，raw1229/not_loaded248，248次原生按ID补读成显式并集。1461旧记录/12问题头、257原件、120历史manifest/31184文件、78样本保持；仅扩展VF58/143/159，其余251保持，精确入口级待核104不变，不等于所有未完成功能。254功能/32权威、计划10完成19未完成；before匹配当前头部/实际provenance与coverage复核，R115旧失败和所有旧反例保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。


[79项观察后限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-000803-s04-polyhedra-opacity/poly-opacity-scoped-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-000803-s04-polyhedra-opacity/poly-opacity-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-000803-s04-polyhedra-opacity/README.md)。


## R122 Polyhedra 面RGB与ATOMT残留（2026-11-10）

R122将R121所选面alpha反例的线索，转为操作前登记的三个独立面RGB检验。全部native core直接复用：R120 constructor/handlers、R107完整Update/reader、R121 Scene/Crystal颜色helper、项目save_data、CopyProperties/CopyParam/refcountedvectorassignment；15份复用源逐份hash/extent核对，没有新增或重复static study/search。

三组一相SiO6规范输入各5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。每组nativeownedPID/title7atoms6bonds1polyhedron/所属短监听及整文件baseline门，均在getApp前通过。中心原子Si、faceRGBA27/59/250/204、全局opacity204、Preview1。分别setValue+Tab改红27→64、绿59→64、蓝250→64；原生UI对应通道变化，另外两个通道及alpha/globalopacity保持。

三组on文件仅所选Si1的SITET面RGB与Si的ATOMT面RGB同一通道改变；O及其他字节保持。三个真正改动状态Cancel均恢复完整SITET，却留下Si ATOMT的相应通道64，整份规范基线相等均false，恰是本轮事前登记预期。九份整文件/SITET/ATOMT预期全部符合。R121原始六份整文件预期仅3通过/3失败、69项原始报告false和实际exit1继续冻结，另建观察后79项差异核对的边界保持，不能把R122事前成功回写为R121旧预测成功。

复用静态路径：constructor605c8绑定dialog46c、事件2c8f/2c91；Update6121c/230/244直接写所选元素8c/d/e后，再经612b8调用Scene80df4；Scene80e7c写元素8c四字节，Crystal6ac50按元素匹配写位点ec四字节。项目writer ae908..914写SITET，af484..490写ATOMT两层；CopyParam/vectorassignment覆盖Scene5c0和Crystal1c8。所选元素在COW前被直接写入与观察到的ATOMT残留一致；实际指针别名/动态callback/唯一Cancel因果仍未证明。三个新RGB独立上下文验证了保留值模式，并非完整渲染或全软件回滚验收。

95项限定核对、9阶段文件/9原生UI通过；自有PID79535/83043/85600正常0退出，90.559/85.168/68.964秒，均<120；原生功能动作均在180前。源及三个输入hash/mtime、0.1秒抽样保持，六个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。中心元素切换、多相/多中心、颜色选择器、实际像素、边界非法值和通用回滚仍待核。

新增18原生ID全部为9UI+9IPC，完整核验派生1495；raw1247/not_loaded248，248次原生按ID补读成显式并集。1477旧记录/12问题头、257原件、121历史manifest/32175文件及78稳定样本保持；只扩展VF58/143/159，其余251保持，入口级待核104不变，多个partial仍未完成。254功能/32权威、计划10完成19未完成；before匹配头部并由实际provenance/coverage复核，R115/R121及全部旧失败保留。Codex自验；dsh/S08和完整Study未完成，未进入产品实现。


[95项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-002146-s04-polyhedra-face-rgb/poly-face-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-002146-s04-polyhedra-face-rgb/poly-face-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-002146-s04-polyhedra-face-rgb/README.md)。


## R123 Polyhedra 材质红与shininess（2026-11-10）

R123独立研究Polyhedra材质红255→128及shininess100%→50%。十份native core全为复用：R120完整constructor/Colour/Spin/Killfocus、R107完整Update/reader、已有XStyle writer/CopyProperties、R115共享MaterialTool.Create155d8（796字节199指令）及OnSpinChanged159b8（52/13）；没有重复native function study/search。

constructor5ff28/2c绑定dialog478/47c，5ff34/38事件2c89/2c8a，5ff3c调用共享MaterialTool。Update61178将ID减2c89、与2比较，落入611cc后611d4写Scene3dc、611e0写3e0，即XStyle1b4/1b8。原生POLYM writer b9278..84读取1b4..1b7 RGBA，b929c读取1b8 rawshininess；CopyProperties ba31c/320复制1b4，ba34c/350复制1b8。该分支与全局opacity及所选faceRGBA分支分离，静态路径不当动态callback或实际渲染验收。

共享转换按float32常量0x3fa3d70a（约1.28），Create先raw/constant再floor(+0.5)显示百分数；OnSpinChanged把整数百分数乘相同float32写raw。GUI前独立记录原生float32预期，50%得到raw64.000，原raw128.000对应100%。两份一相SiO6规范输入5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56；各nativeownedPID/title7atoms6bonds1polyhedron/所属短监听及整文件baseline门，均在getApp前通过。初始材质255/255/255/100%，Preview1。

红组只setValue128+Tab，原生UI红128/其他255/255、高光100%，文件仅POLYM红变128。独立shininess组setValue50，CUA批次报frontmostApplicationChanged；fresh AX显示控件185已为50且聚焦，故仅补Tab，fresh AX选中204后继续nativeon/IPC，不重复setValue。该错误具体子调用未独立跟踪，保留cua-interruption-observation.json，不删去中断。nativeUI高光50%，文件仅POLYMraw128.000→64.000。两组真正改动状态Cancel都恢复整份5144字节规范基线；六份事前整文件预期符合，全部SITET及其他字节保持。不能用本轮材质成功改写R121/R122面RGBA的ATOMT保留失败。

64项限定核对、6阶段文件/6原生UI通过。PID95619/96208正常0退出，79.893/94.467秒，均<120，原生功能动作在180前。源及两个输入hash/mtime、0.1秒抽样保持，四个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。仅所选材质值/UI/编码/Cancel；材质像素、lighting/physics、G/B其他输入、颜色选择器、边界非法值、多相及通用回滚仍待核。

新增12原生ID全部为6UI+6IPC，完整核验派生1507，raw1252/not_loaded255，255次原生按ID补读成显式并集；1495旧记录/12问题头、257原件、122历史manifest/33190文件、78样本保持。仅扩展VF58/143/153，其余251保持，精确入口级待核104不变（另有多个partial）。254功能/32权威、计划10完成19未完成。before匹配头部并按实际provenance/coverage复核；R115头部失败、R121原始false69项/实际exit1及观察后79项边界、R122保留值等旧失败保持。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。


[64项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-003609-s04-polyhedra-material/poly-material-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-003609-s04-polyhedra-material/poly-material-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-003609-s04-polyhedra-material/README.md)。


## R124 General 晶胞线绿色与蓝色（2026-11-10）

R124独立补核General晶胞线颜色的G/B通道0→64。八份native core全部复用R112：General constructor4104字节1026指令、reader80/20、完整UpdateCanvas428/107、原生UCOLP writer/CopyProperties及三个killfocus/colour/spin handler44/11、8/2、8/2。完整已知范围逐份hash与连续指令核对，没有重复native function study/search；既有builtin unresolved_indirect_jumps=1与virtualtail限制保留，不把静态路径当动态callbacktrace。

constructor495b8绑定dialog488，495c0/5c4注册事件2b5c/2b5d，495c8指定3通道。reader4a2e4从Scene230回读RGB；killfocus48f0c、colour48b3c、spin48b34转入Update4a33c；Update4a3e0/3ec/3f8分别写Scene230/231/232，即XStyle8/9/a。原生writer b8f30/34/38读8/9/a写UCOLP零基token3/4/5；CopyProperties ba11c..130复制三字节。本轮只改绿/蓝，不涉及General其他分支、颜色选择器或SaveDefault。

两份独立一相SiO6规范输入各5144字节、SHA308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。各nativeownedPID/title7atoms6bonds1polyhedron/所属短监听及整文件baseline门，均在getApp前通过；General初始Singleunitcell/Solidlines、width1/RGB0/0/0，Preview1/Showcompass1/Showaxislabels1/Showedges1。绿组只setValue64+Tab改变中间RGB，蓝组独立改最后RGB，native UI保持线宽1及其他颜色0。

两组on保存文件仅UCOLP绿或蓝项0→64，完整其他字节保持。两个真正改动状态Cancel均恢复整份5144字节规范基线，六份事前整文件预期全部符合。与R112已验width1→2、red0→64合起来，仅覆盖当前输入的所选线宽和RGB值，不把它扩成任意范围、实际线条像素或通用回滚；R103FORMP/R104mark/SBOND/ATOMT保留值等旧失败继续保持。

60项限定核对、6阶段文件/6原生UI通过；PID5129/5606正常0退出，75.715/74.194秒，均<120，原生功能动作均在180前。源及两个输入hash/mtime、0.1秒抽样保持，四个所属偏好文件归档；无Vectors/追加相/SaveDefault/强停/退出后应用查询。本轮没有前台切换错误；R123原中断记录及其恢复证据仍在历史包内保留。

新增12原生ID全部为6UI+6IPC，完整核验派生1519；raw1260/not_loaded259，259次原生按ID补读成显式并集。1507旧记录/12问题头、257原件、123历史manifest/34174文件及78样本保持；只扩展VF55/143/152，其余251保持，入口级待核104不变，另有多个partial。254功能/32权威、计划10完成19未完成。before匹配头部并按实际provenance/coverage复核，R115头部失败/R121原始false69项及exit1/观察后79项边界等全部旧失败保留。Codex自验；dsh/S08及完整Study未完成，未进入产品实现。


[60项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-004705-s04-cell-gb/cell-gb-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-004705-s04-cell-gb/cell-gb-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-004705-s04-cell-gb/README.md)。


## R125 所选Si原子半径与红色（2026-11-10）

R125独立验证所选Si的Atoms半径1.18→1.5及原子红色27→64。事前固定六份完整5144字节文件预测，均符合：on仅对应Si1 SITET与Si ATOMT的半径或第一个RGB红色项改变；真正改动状态Cancel恢复SITET，却保留ATOMT改动，整份规范基线相等为false，符合事前预测。第二组RGB（多面体面色）及所有O/其他字节保持。此结论限定当前一相SiO6和两个选定正值，绿蓝/范围/颜色选择器/多相/实际像素仍待核，旧保留字段失败不被覆盖。

原生Atoms constructor8432字节2108指令：dialog498/event2bca为半径，490/event2bcb/2bcc为3通道原子RGB；reader684/171从element1c、88..8a回读。复用原生paged objdump的182连续指令Update范围2e148..2e420，原builtin截断输出仍保留。新增原生数据word4d73b2的前三字节56/26/26，按base2e194导向半径2e2ec/颜色2e22c。2e300直接写element1c后调用Scene::UpdateElementRadius；2e240/258/270直接写element88/89/8a后调用Scene::UpdateElementColor，phase=-1。没有动态callbacktrace。

新增完整Scene半径helper80bf8/252字节63指令、颜色80cf4/256/64及Crystal半径6aa60/204/51、颜色6ab2c/212/53。Scene路径具有clone/refcount，写element1c或88，负phase遍历Crystal；Crystal匹配site d4所选element，写site34或e8。原生project save_data10176/2544将这些值分别写SITET与ATOMT零基token2..5，faceRGBA token6..9另列。既有CopyParam/向量赋值各220/55，复制Scene5c0与Crystal1c8的引用计数指针。提前写入与共享快照污染在静态上相容，但实际动态指针别名、唯一Cancel成因、32bitcolor第四字节语义及范围外fallthrough元数据都没有被当作已证事实。

两个正式实验PID22463/27549正常0退出，65.791/59.562秒，均<120；nativeownedPID/title7atoms6bonds1polyhedron/所属监听与整文件baseline在getApp前通过。源和两个工作输入SHA/mtime及0.1秒抽样保持，四个所属偏好文件归档；初始两种Atoms模式radio均0，未强制修正或操作disabled控件。红组启动前发现radius/red短名碰撞，仅将红组preferences路径改为独立新目录；原计划、修改收据、hash及改动范围完整保留，功能预测不变。

本轮生命周期检查有真实失败：红组Quit后的getAXState返回空VESTA，正式PID已退出而另有PID28229于同秒启动。原71项报告仅“no remaining target”为false，实际--verify exit1及原分析器/hash/mtime冻结保留。随后新增一份PID绑定原生UI确认空实例、无研究文件，再经CUA仅Quit且不跟随界面读取；进程扫描确认消失，无强停。精确内部启动调用和额外实例的偏好环境/退出码未追踪，不把清理后通过说成原报告通过。由此后续Quit后必须先走原生存活检查，不能把Quit与getAXState批在同一调用；只有原生证实同一仍存活PID及所属保存提示时才允许后续CUA。

清理后限定报告72项通过，6阶段文件/6正式原生UI，另1份仅清理用原生UI。新增18原生ID=4function+1data+7UI+6IPC；完整核验派生1537，raw1261/not_loaded276，通过276次原生按ID补读形成显式并集，未重写canonical证据。1519旧记录/12问题头、257原件、124历史manifest/35168文件及78样本保持；只扩展VF56/143/154，其余251保持，入口级待核104→103（另有多个partial，不是全部未完成数）。254功能/32权威，计划10完成19未完成；R115头部失败及R121原始false/exit1等旧失败保留。Codex自验；dsh/S08与完整Study未完成，未进入产品实现。


[清理后72项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-010041-s04-atom-radius-red/atom-radius-red-final-crosscheck.json) / [原始71项失败](../../output/vesta-study/20261108-113622/reviews/20261110-010041-s04-atom-radius-red/atom-radius-red-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-010041-s04-atom-radius-red/atom-radius-red-before-functional-targets.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-010041-s04-atom-radius-red/README.md)。


## R126 所选Si原子绿色与蓝色（2026-11-10）

R126独立补核所选Si的原子绿色59→64、蓝色250→64。六份事前完整5144字节文件预测全部符合：on仅对应Si1 SITET与Si ATOMT的原子RGB零基token4或5改变；真正改动状态Cancel恢复SITET，却保留ATOMT对应通道64，整份规范基线相等均为false，符合事前预测。半径1.18、红色27、第二组多面体面色RGBA、所有O与其他字节保持。结合R125半径/红色，仅覆盖此一相SiO6所选正值的半径/RGB/UI/编码及Cancel行为，不覆盖任意输入、元素、像素、颜色选择器或通用回滚。

十四份原生静态来源全部复用：完整Atoms constructor8432字节2108指令、reader684/171、原生paged Update相关182连续指令（原builtin截断仍保留）、Scene/Crystal半径与颜色helper、dispatchword、project save_data10176/2544、CopyParam与向量赋值及handler。dialog490/事件2bcb/2bcc经word前三字节56/26/26中颜色项26转到2e22c；Update2e258/270写element89/8a后调用Scene80cf4，phase=-1，经Crystal6ab2c匹配所选element写site e8起32bit颜色。writer把element89/8a与sitee9/ea分别写ATOMT/SITET的原子绿色/蓝色，faceRGBA字段另列。引用计数/提前写入是静态候选，动态pointeralias/callbacktrace、唯一Cancel成因和32bitcolor第四字节语义仍未证明；没有新增重复functionstudy/search。

两份独立输入使用不同全新短preferences；各nativeownedPID/title7atoms6bonds1polyhedron/所属监听与完整baseline门在getApp前通过，所选Si/Preview1、Atoms两种mode radio初始均0并保持。绿色只setValue64+Tab改208，蓝色独立改212，native UI与IPC文件互核。PID48169/51267正常0退出，61.217/64.205秒，均<120，功能动作均在180前；源与两个工作输入SHA/mtime及0.1秒抽样保持，四个所属偏好文件归档，无强停。

本轮Quit均单独调用，之后先原生存活检查；deadPID不再getApp/AX，观察到无剩余VESTA，未出现额外空实例。R125原71项noRemainingTarget false/实际exit1/原分析器/hash/mtime及另立72项清理后通过报告全部保留；本轮成功不能覆盖前轮失败，也不证明任何环境下都不会重启。R121原始false预测/实际exit1及观察后characterization边界继续保留。

71项限定核对、6阶段文件/6原生UI通过。新增12原生ID全部为6UI+6IPC，完整核验派生1549；raw1271/not_loaded278，通过278次原生按ID补读形成显式并集，未重写canonical旧记录。1537旧记录/12问题头、257原件、125历史manifest/36247文件及78样本保持；只扩VF56/143/154，其余251保持，入口级待核103不变，另有多个partial。254功能/32权威，计划10完成19未完成；before匹配更新当前头部，R115头部失败及所有旧反例继续保留。Codex自验；dsh/S08和完整Study未完成，未进入产品实现。


[71项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-011949-s04-atom-gb/atom-gb-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-011949-s04-atom-gb/atom-gb-before-functional-targets.json) / [前轮失败保留](../../output/vesta-study/20261108-113622/reviews/20261110-011949-s04-atom-gb/reused-r125-lifecycle-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-011949-s04-atom-gb/README.md)。


## R127 原子半径类型（2026-11-10）

R127验证Atoms Radii type的Atomic→Ionic和Atomic→van der Waals。六份事前5144字节整文件预测全部符合：Ionic把Si/O半径置0.2600/1.4000，van der Waals置2.1000/1.5200；每组更新全部7条SITET及2条ATOMT半径，并仅将ATOMS首项0→1/2，颜色及所有其他字节保持。两次真正改动状态Cancel均恢复整份5144字节规范基线，分别重新打开Properties/Atoms均读到Atomic/Si半径1.18。最初所选Si并不限制此选择器只改Si，实际文件包含两种元素的全部9个半径项。这里仅验证当前一相SiO6，不接受所有元素/离子电荷配位/科学半径正确性/实际像素/多相或通用回滚。

复用完整constructor8432/2108、reader684/171及既有Atoms handler catalog：choice2bcd/dialog340、3选项/初始dialog478=Scene5b8。新增完整OnRadiiType2b2ac464字节116指令，index0/1/2分别取ElementData10/18/14，遍历Scene5c0全部元素，直接调用Scene::UpdateElementRadius80bf8，phase=-1，再Crystal6aa60匹配site d4写site34；选择器没有手工半径分支那种先直接改element1c再调用helper的写入顺序。既有project save_data10176/2544把element1c/site34分别写ATOMT/SITET半径。XStyle writer2588/647的b8a80读390（Scene5b8）写ATOMS首项；CopyProperties728/182的ba13c/140明确复制390，CopyParam220/55还复制elements/sites向量。此复制字段和序列化映射是在登记预期前核实，未把“类型不复制”猜测登记为先验；也不把静态顺序差异当动态pointeralias或唯一Cancel成因证明。

新增Element::GetData1270完整88/22，按Object+c的Z匹配52d680向量；新增loadDefPar de0完整676/169，读取elements.ini的8个字段，前三浮点分别进入10/14/18。原版packaged elements.ini原字节复制并保留SHA e77dd5dafe975347cf587d553bd26067961f9b954efd08e6bc33745b39104d99及mtime，Si行1.18/2.10/0.260、O行0.74/1.52/1.400与运行值吻合。原版MachO的arm64 slice/loadcommands/对应地址原字节另作明确derived UTF32数据提取，证实三标签Atomic/Ionic/van der Waals顺序；这不是新增native evidence，也没有捕获运行时打开该资源的调用。17份既有native source复用；两次限定函数目录查询和3份新完整函数研究不重复旧functionstudy。

两组nativeownedPID/正确标题7atoms6bonds1polyhedron/所属短listener与完整baseline均在getApp前通过；工作源/输入SHA、mtime和0.1秒抽样保持，四个所属偏好文件归档。初始Atoms两种mode radio均0并保持，未强制disabled控件。Ionic菜单点击曾返回“The user changed ...”状态提示，fresh AX仍Atomic/1.18且菜单展开后才以新索引重试，随后Ionic/0.26原生UI及文件证实；提示原文/恢复观察保留，不归因于某个人或某个未instrument子调用。该组PID77385正常0/119.665秒，达到<120目标但余量仅0.335秒，不应描述为宽裕。

van der Waals组首次Quit返回frontmostApplicationChanged；原生PID81342读到仍活的正确所属主窗口、没有保存提示，原“首次Quit后deadPID不查询”检查因此false。只在此原生身份门后查询既有bound活实例（未getApp），fresh AX仍是同一主窗口，再单独Quit正常0退出；最终原生guard证实deadPID后没有应用查询，没有No/重启/强停。实际146.442秒未达120目标、仍<180。此失败Quit恢复是有记录的有限清理裁定，不能把无保存提示的原guard false改成true，也不能接受原报告hardcoded noPostQuitCUAQuery=true作为“所有Quit尝试后从未查询”的广义主张；仅接受没有post-death查询。后续短实验需为退出重试留余量。

原82项报告80true/2false（vdw under120、vdw deadPIDnoquery），实际运行及--verify均exit1；原计划/分析器/false报告/收据/guard/提示/hash/mtime全部冻结。另立14项观察后证据与恢复复核通过，明确不是替换事前预测，六份原文件预测和8份功能UI均符合，另1份仅清理原生UI。新增20原生ID=3function+2search+9UI+6IPC；完整核验派生1569，raw1273/not_loaded296，通过296次原生按ID补读显式并集，canonical旧记录不重写。1549旧记录/12问题头、257原件、126历史manifest37304文件及78样本保持；只扩VF56/143/155，其他251保持，入口级待核103→102，另有多个partial。254功能/32权威、计划10完成19未完成；R121原始false/exit1与观察后边界、R125生命周期false/exit1、R115头部失败及其他旧反例都保留。Codex自验，dsh/S08与完整Study未完成，未进入产品实现。


[原82项false报告](../../output/vesta-study/20261108-113622/reviews/20261110-013325-s04-atom-radii-type/atom-radii-type-crosscheck.json) / [原实际exit1](../../output/vesta-study/20261108-113622/reviews/20261110-013325-s04-atom-radii-type/original-verify-run.json) / [14项观察后限定复核](../../output/vesta-study/20261108-113622/reviews/20261110-013325-s04-atom-radii-type/atom-radii-type-scoped-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-013325-s04-atom-radii-type/README.md)。


## R128 正ADP椭球概率与轮廓（2026-11-10）

R128独立验证正ADP工程中的椭球概率50→75、主椭圆轮廓宽度2.0→3.0。来源是VS57非正交C/O正定各向异性U样本在R104合法切换后冻结的两个4180字节VESTA派生文件：概率组ellipsoid.vesta（mode1/principal0），轮廓组changed.vesta（mode1/principal1）。每组启动前登记完整source/hash/mtime/预期，nativeownedPID/正确标题2atoms0bonds0polyhedra/所属短listener与整文件baseline门在getApp前通过；CUA fresh AX确认已载入Showasdisplacementellipsoids1、概率可编辑，轮廓组主椭圆1/width可编辑。不新切换mode或principal、不强制disabled控件，不增加稳定样本ID。

六份事前4180字节整文件预测全部符合：概率on仅ATOMP第四项50→75，轮廓on仅第五项2.0→3.0；其余ADP、site/element半径颜色、DISPF37884866、ATOMS0/1/1及其他字节保持。两次真正改动状态Cancel均恢复各自整份冻结基线，包含原principal0/1。原CIF SHA dc52d2f42554ceb909a6d9ea129a3bfa54efa1ca708d4bb1c605b34f3d892840和两份冻结源/工作输入hash、mtime、0.1秒抽样保持；复算C1/O1的三个有理数顺序主子式均正，仅建立可用正ADP输入，不接受椭球概率区域/轴长/投影等科学或像素正确性。

完整constructor8432字节2108指令、reader684/171及fullpaged Update相关182连续指令复用，原builtin截断限制保留。概率dialog484/event2bc6、outline494/event2bc9；新增原生word4d73ae为[27,27,73,0]，对应已登记ID的首/末字节将事件导向2e200/2e194。Update2e20c写Scene270概率，2e1b8写Scene278轮廓，等于XStyle48/50。复用writer2588/647的b8b68、b8b54及参数栈顺序映射到ATOMP第四/第五项；copy728/182的q0块40..4f和d0块50..57包含两字段。Modehandler484/121与既有原生UI来源也复用，不重复study/search，不把静态绑定当动态callbacktrace。

各向同性策略仅新增静态OnChoiceScaleIso2b47c完整144/36：choice非零置Scene5b0的bit25，Preview下创建atomlist；builtin unresolved_indirect_jumps=1的virtualtail仍未解析。此策略本轮未操作、未接受运行语义，仍属VF156剩余工作。原生UI提取未提供enabled属性，实际可编辑状态来自CUA fresh AX及成功改动/文件对照，不能声称原生AXEnabled已恢复。

66项限定核对、6阶段文件/6原生UI通过。PID2099/3057正常0退出，69.684/72.493秒，均<120，功能动作均在180前；4份所属偏好文件归档。两次Quit均独立，随后原生确认deadPID才停止查询，无post-death应用查询/额外空实例/No/强停，本轮未观察到CUA状态切换错误。R127原82项80true2false/actualexit1、未达120目标、原guardfalse/被限定的hardcoded noPostQuit标志与另立14项观察后复核全部保持；本轮短上下文成功不覆盖其失败。

新增14原生ID=1function+1data+6UI+6IPC，14份既有native source复用。完整核验派生1583，raw1277/not_loaded306，306次原生按ID补读形成显式并集，canonical旧记录不重写。1569旧记录/12问题头、257原件、127历史manifest38453文件及78稳定样本保持；只扩VF56/143/156，其余251不变，入口级待核102→101，另有多个partial。254功能/32权威、计划10完成19未完成，before匹配更新当前头部。R121原始false/actualexit1及观察后边界、R125生命周期false/exit1、R115头部失败和所有旧保留字段反例继续保留。Codex自验，dsh/S08、完整Study未完成，未进入产品实现。


[66项限定核对](../../output/vesta-study/20261108-113622/reviews/20261110-015640-s04-ellipsoid-controls/ellipsoid-controls-crosscheck.json) / [事前预期](../../output/vesta-study/20261108-113622/reviews/20261110-015640-s04-ellipsoid-controls/ellipsoid-controls-before-functional-targets.json) / [R127原失败保留](../../output/vesta-study/20261108-113622/reviews/20261110-015640-s04-ellipsoid-controls/reused-r127-failure.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-015640-s04-ellipsoid-controls/README.md)。


## R129 各向同性策略双向编码及保留失败（2026-11-10）

R129核对各向同性原子策略的两个方向：Scale isotropic atoms by Uiso（choice1）→Set radii of isotropic atoms as specified below（choice0），以及独立上下文的0→1。实际菜单标签/顺序与原版arm64 UTF32字节、完整constructor8432/2108的dialog348/event2bce/两choice/初始DISPFbit25一致。复用R128完整OnChoiceScaleIso2b47c144/36：取selection、非零cset、bfiScene5b0 bit25，Preview下CreateAtomList；unresolved_indirect_jumps=1的virtualtail仍未解析。复用writer2588/647 b8a74..88将bit25写为ATOMS第三项；copy728/182 mask02422800包含bit25，CopyParam/reader与既有mode证据复用，共15份native来源，无新function/search/data调用，不把静态链视作动态callbacktrace。

输入先从R104冻结合法mode1/principal0正ADP工程派生：保留C1正定张量，O1取VS55正Uiso.050661、删除其THERM行。原生headless预检wrapper0/目标255/exited、进程组verified，几何2atoms0bonds0polyhedra；实际writer补写O1六个0的THERM行，并将显示mode1重置0。原预检4项2true2false/实际exit1完整保留：C1-only THERM标签预期失败，Uiso检查器要求前导0而文件为.050661也失败。原计划/输入/检查器/报告/收据hashmtime冻结；观察后数值解析确认正Uiso，不能覆盖原false。正式GUI事前预期另行登记在此已观察规范化之后，从实际4186字节输出只派生已知R104 bit17/ATOMS第二项的mode1编码与两个初始policy。不新切换GUI mode，不强制disabled控件，不新增稳定样本；仅称名义混合ADP表示，O物理各向同性分类和科学几何未接受。

两组nativeownedPID/正确输入标题/2atoms0bonds0polyhedra/所属短listener和整文件baseline门均在getApp前通过。CUA freshAX观察ellipsoid1、principal0、概率50、outline2.0disabled、C半径.77/RGB128/73/41，policy可用。六份事前整文件预测全部符合：指定半径on只清DISPFbit25（37884866→4330434）和ATOMS第三项1→0，4186→4185字节；反方向只置这两处，4185→4186字节。ADP、site/element/颜色/其他字节不变，两次真正改动状态Cancel恢复各自整份基线。两个输入和三份父源hashmtime、0.1秒抽样保持；4份GUI偏好与1份预检偏好分别归档。

指定半径PID21414菜单出现3次CUA“The user changed...”提示；每次重读已有bound app freshAX并保留旧policy，期间键盘/两种坐标尝试均未建立成功，最终freshmenu71点击才观察到目标标签。提示/失败尝试/重试均留档，未归因用户/单一callback或唯一延迟原因。该组正常0/160.131秒，未达120目标；Uiso PID21941首次选择成功、正常0/62.598秒。两组功能调用均在180前、未强停。Quit均独立，随后nativePIDguard确认dead，不做post-deathgetApp/AX、无额外空实例/No。原正式67项66true1false/实际运行及重算exit1冻结，唯一false为specified prospective owned normal under120。另立14项观察后证据复核通过，明确不是替代事前预期，也不宣称all120通过。

新增13nativeID=6UI+7runtimeIPC（1预检+6GUI保存）；完整核验派生1596，raw1283/not_loaded313，经313次原生按ID补读显式并集，不是未修改的1596条原生export；1583旧记录/12问题头保持，不改canonical。257原件、128历史manifest39601文件、78稳定样本保持。只扩VF56/143/156，其余251不变，254功能/32权威，入口级待核101→101，另有多个partial。VF156仍partial，科学各向同性分类/策略几何/像素、参数域、动态callback、一般rollback未验。R127原82项80true2false/actualexit1+另立14项、R121/R125/R115及全部旧反例继续保留。计划10checked/19unchecked，Codex自验，独立复审仅dsh、S08/完整Study未完成，未进入实现。


[原67项66true1false](../../output/vesta-study/20261108-113622/reviews/20261110-021420-s04-isotropic-policy/isotropic-policy-crosscheck.json) / [另立14项观察后复核](../../output/vesta-study/20261108-113622/reviews/20261110-021420-s04-isotropic-policy/isotropic-policy-scoped-crosscheck.json) / [预检原始失败](../../output/vesta-study/20261108-113622/reviews/20261110-021420-s04-isotropic-policy/fixture-preparation-original-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-021420-s04-isotropic-policy/README.md)。


## R130 零张量策略静态路径及范围边界（2026-11-10）

R130以静态链澄清R129“正Uiso＋零THERM行”的程序分类，没有重跑已完成的策略GUI、没有启动VESTA目标执行。复用readVESTAfile19620字节4905行、readCIFfile12664/3166、Atomsconstructor8432/2108、reader684/171、writer2588/647和OnChoiceScaleIso144/36共6份原生来源。THERI1经reader a5160..6c使Crystal218=0，单个U值经float32→float64写site数据base+1；THERM按行写base+2..7。当前两行路径使Crystal210=1，这是Crystal级存在标志，不是逐原子零张量判定。

新增一次CreateAtomList精确搜索得到Scene79598和Crystal633d8，并新增4份functionstudy：Scene252/63、Crystal3908/977、Make_thermal_ellipse_matrix643bc2196/549、Matrix4Dconstructor6e084112/28。Scene传Scene228处XStyle，故XStyle388=Scene5b0。Crystal modebit17选择matrixmaker；maker645ec..64644逐项比较六个热参数，任一非零入64694各向异性/转换/SYEVR路径，全零才读XStyle38b bit1=DISPFbit25。R129实际C1首项.01走非零路径，O1六项0走此零张量分支。这是源绑定的程序静态分类推断，尚不是运行中站点/科学几何验收，不回改R129的原接受边界。

bit25清零：创建4×4单位矩阵并写object+a0=matrix(3,3)=0作为回退标志；bit25置位：读取base+1的Uiso，Crystal218=0选U值，其他单位路径除以程序常量78.95683520871486（8π²），取平方根传给cblas_dscal，再把M33恢复1。原版MachO字节派生验证constructor的16个double为单位阵，PROBABILITY[49]的原生float32值1.5381699800491333。CreateAtomList检查M33：0走普通站点半径；1走glMultMatrixd和probability索引50-1的缩放命令。SITET reader以%f写XSite34，当前O1站点半径.7400与ATOMT O值相同；不把元素表来源代替站点来源。

十进制参考模型：Ball-and-stick常量.4×.74≈.296，sqrt(.050661)≈.2250799858，乘原程序P50≈.3462112772。另立4项半径/float绑定检查确认实际%f来源及已知读写：按IEEE754 round-to-nearest假设，普通半径float32≈.29600000381469727，Uiso先float32→float64为.05066100135445595，sqrt再乘P50≈.3462112818691342。两组数字均为静态推演，不是现场矩阵/GL调用/像素/物理正确性证据；FPCR、BLAS数值实现、科学椭球与非正输入等未验。等列范数比较的程序阈值为1e-6，但不由此宣称最终primitive/像素几何已核对。

本轮首次37项核对34true3false、实际重算exit1冻结：检查器错把入口至末地址当作连续代码。原生body_ranges明确VESTAreader/CIFreader/matrixmaker的enclosing span分别排除2/4/4个地址；声明代码段内行数、字节数和指令覆盖逐一精确吻合，所引用5段分支另外证明连续。未重复相同native study、未把原始false改为通过。另立41项观察后复核通过，明确保留body边界及未解析calls；3函数unresolved_indirect_jumps0不代表全部虚调用解析，原OnChoiceScaleIso virtualtail1保持。另保留本轮两版派生计算helper的StopIteration/实际复现exit1和最终exit0：先是ATOMT子串提前截断，再是修正时计算仍位于收集循环内，原失败脚本和收据均归档。

新增5原生ID=1functions_search+4function_study，无新data/runtime/UI记录；完整核验派生1601，raw1263/not_loaded338，经338次native按ID补读显式并集，不是未修改的1601条原生export。1596旧记录/12问题头、257原件、129历史manifest40794文件、78稳定样本保持。当前R130未运行GUI，不宣称达到GUI120目标；R129正式67项66true1false/actualexit1、160.131秒未达120、3次CUA提示、另立14项与预检2true2false/exit1原样保留，R127/R121/R125/R115及全部旧反例保持。只扩VF56/143/156，其余251不变；254功能/32权威，entry-only待核101→101，另有多个partial。计划10checked/19unchecked，Codex自验，独立复审仅dsh，S08未开始、完整Study未完成，未进入实现。


[原37项34true3false](../../output/vesta-study/20261108-113622/reviews/20261110-023914-s04-isotropic-classification/isotropic-classification-crosscheck.json) / [另立41项观察后复核](../../output/vesta-study/20261108-113622/reviews/20261110-023914-s04-isotropic-classification/isotropic-classification-scoped-crosscheck.json) / [半径/float4项](../../output/vesta-study/20261108-113622/reviews/20261110-023914-s04-isotropic-classification/static-radius-and-float-binding.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-023914-s04-isotropic-classification/README.md)。


## R131 概率域与合法端点（2026-11-10）

R131核对位移椭球概率的配置域与合法端点。新增vxSpinCtrlC1 1635c20字节5指令、C2 161f8284/71、OnSpinCtrl1617428/7、wxgeneric DoSetValue2c8558496/124，及一次精确vxSpinCtrl搜索（12匹配）。复用16份native来源，含完整Atomsconstructor/reader/writer/copy、fullpagedUpdate182指令、R128dispatchword和R130CrystalCreateAtomList，不重复已有functionstudy。caller2bc6/dialog484把min1/max99传给C2；C2将min/max/初始*intpointer转换为double，step1传wxgenericCreate，保存int指针于398并DoSetValue。genericSetter从360/368作上下界clamp，378标志及370step非零才走条件snap；OnSpinCtrl从358的double经fcvtzs写回398指向整数，再Skip事件。此为静态setter链，不是越界/小数/非数字文本路径已动态验证。C1 assembly36c的tail明确到C2；builtin callee元数据site370在C1 body外，未据此冒充真实callsite或动态callbacktrace，generic虚调用仍未全解。

原版lib MachO符号PROBABILITY位于50ef80，当前GUI可达1..99对应index0..98，R130绘制代码是tablebase+probability*4-4。本轮事前登记独立数学模型与绝对概率容差1e-5，提取99个原生float32字节后核对：全部正、有限、严格递增，与三个独立标准高斯坐标的径向CDF积分F(r)=erf(r/√2)-√(2/π)r·exp(-r²/2)吻合，最大绝对误差2.942649164061617e-6。P1=.33886998891830444、P50=1.5381699800491333、P99=3.3682100772857666。四项独立表检查通过，但只是标量模型，不接受现场GL/科学椭球/像素。下一同section符号50f110给出400字节间距上界；99项之后4字节dc68be40非零，未把它归类为padding或有效100%项，不宣称C++数组声明长度已恢复。

两组独立上下文均复用VS57/R104冻结的mode1/principal0正定C/O ADP工程，各4180字节。所有helpers、六份整文件oracle和输入/sourcehashmtime在GUI前冻结；每组nativeownedPID/正确标题2atoms0bonds0polyhedra/所属短listener/完整baseline门在getApp前通过，CUA freshAX看到概率50可编辑、outline2.0disabled。分别仅setValue+Tab到合法1和99，不动mode/principal/isotropicpolicy、不强制disabled控件。

47项限定核对及重算actualexit0，另含事前26项静态检查和独立表4项；六份prospective4180整文件预测全部符合：两组on仅ATOMP第四项50→1/99，%3i固定宽度保持总字节，全部ADP/site/element/颜色/DISPF37884866/ATOMS0/1/1/其他字节保持。两次真正改动状态Cancel均恢复整份各自基线。PID46246/46627正常0退出，66.952/70.132秒，均<120且功能动作180前；source与两个工作输入hashmtime及0.1秒抽样保持，4份所属偏好归档。Quit独立后native确认deadPID，无post-death应用查询/额外空实例/No/强停，本轮未观察到CUA状态切换提示。nativeUI未提供enabled属性，实际可编辑/disabled来自CUA和成功变更，不冒充nativeAXEnabled。

本轮施工helper首次因else50缺空格SyntaxError，原脚本/复现actualexit1留档，修正发生在GUI前，未改事前oracle或正式分析器。R130原37项34true3false/actualverifyexit1、body_ranges enclosing排除2/4/4地址、另立41+4及两个helper失败保持；R129正式67项66true1false/actualexit1、160.131秒未达120/3次CUA提示、另立14与预检2true2false/exit1保持，R127/R121/R125/R115及全部旧反例继续保留。

新增17nativeID=4functionstudy+1search+6UI+6IPC。完整核验派生1618，raw1269/not_loaded349，349次native按ID补读显式并集，不是未修改的1618条原生export，不改canonical。1601旧记录/12问题头、257原件、130历史manifest41950文件、78稳定样本保持。只扩VF56/143/156，其余251不变，254功能/32权威，入口级待核101→101，另有多个partial。参数越界/小数/非数字文本运行、原生文件异常值域、科学几何/像素及一般rollback仍待核。计划10checked/19unchecked，Codex自验，独立复审仅dsh，S08未开始，完整Study未完成，未进入实现。


[47项端点核对](../../output/vesta-study/20261108-113622/reviews/20261110-025855-s04-probability-domain/probability-domain-crosscheck.json) / [26项事前静态检查](../../output/vesta-study/20261108-113622/reviews/20261110-025855-s04-probability-domain/static-preflight.json) / [99项概率表标量核对](../../output/vesta-study/20261108-113622/reviews/20261110-025855-s04-probability-domain/probability-table-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-025855-s04-probability-domain/README.md)。


## R132 数值越界文本提交（2026-11-10）

R132补上概率数值越界文本提交的限定证据：两个独立正ADP上下文暂存0/100，Tab后分别归限为1/99，保存前都通过nativePID绑定的postcommitgate。raw阶段只读CUA/nativeUI，不发VESTA保存客户端；没有声称该阶段Scene或GL内部值已观察。六份事前4180整文件oracle全部符合：on只ATOMP第四项50→1/99，其他ADP/site/element/颜色/DISPF37884866/ATOMS0/1/1及全部字节不变；两次真正改动状态Cancel恢复各自整份基线。

复用20份native来源，包括R131vxSpinC1/C2/OnSpinCtrl/DoSetValue与旧constructor/reader/Update/writer/概率表链，不重复已有study。新增wxgeneric精确搜索31匹配，以及5份functionstudy：OnTextLostFocus2c81e4为76字节19指令、SyncSpinToText2c8434为196/49、AdjustToFitInRange2c850c为76/19、wxSpinCtrlDoTextToValue2c8c98为84/21、DoSendEvent2c8b28为320/80。LostFocus以SendEvent1调用Sync，返回changed才调用vslot760，再Skip事件。Sync先检查text modified虚方法，vslot768解析；成功就inline比较368最大/360最小并clamp，失败回取358原值，最后进入已研究DoSetValue。另一个AdjustToFitInRange有条件wrap分支，不把它替代本次Sync的inlineclamp。

R131C2最终vptr1006040a0绑定两个新nativeu64读取：+768=100604808→1002c8c98，+760=100604800→1002c8b28。已选整数解析器调用wxStringToLong，virtualbase仍未单独追完，不宣称全部语法/溢出行为已恢复；成功将long转换double。DoSendEvent用358数值转整数构建SpinEvent，再经virtualdispatch；旧vxOnSpinCtrl把358经fcvtzs写入398链接的dialog整数，Update/writer原链复用。控件最终slot已映射，但实际eventdispatch、modified/gettext等仍有未解虚调用，不冒充动态callbacktrace或全解析callgraph。

本轮25项事前静态检查、57项限定运行核对及实际重算exit0。所有helpers、六份文件预期和source/inputhashmtime在GUI前冻结；nativeownedPID/正确标题2atoms0bonds0polyhedra/所属短listener/完整baseline门在getApp前通过。CUA freshAX显示概率50可编辑、outline2.0disabled；不切mode/principal/isotropicpolicy，不强制disabled控件。暂存前允许集合已登记为rawtyped或已归限值，实际CUA及两次native typedUI均观察到raw0/raw100；Tab后nativeonUI精确1/99，gate记录在on保存客户端之前，未直接保存越界文件或注入表索引。

PID59440/61082正常0退出，93.935/90.309秒，均<120、全部功能动作180前，保留清理余量；源/两个工作输入hashmtime及0.1秒抽样保持，4份所属偏好归档。Quit独立后nativeguard确认dead，无post-deathgetApp/AX、额外空实例、No、强停；本轮未观察到CUA状态切换提示。nativeUI无enabled属性，实际editable/disabled来自CUA和成功变更，不能声称nativeAXEnabled已恢复。

接受边界只扩到选定0/100数值越界文本+Tab提交/UI/文件/Cancel；小数、非数字、空串、溢出、其他进制、Return提交、异常文件域、瞬态Scene/GL、科学椭球/像素与一般rollback仍待核。R13147项合法端点和99可达表项/CDF标量核对保持，400字节符号间距不是C++数组声明长度，extra4字节dc68be40仍未分类。R131施工helperSyntaxError/复现exit1，R130原37项34true3false/actualverifyexit1/body_ranges排除2/4/4、另立41+4及两版helper失败，R129原67项66true1false/actualexit1/160.131秒未达120/3次CUA提示、另立14及预检2true2false/exit1，R127/R121/R125/R115及全部旧反例都保留，不回改旧接受边界。

新增22nativeID=5functionstudy+1search+2data+8UI+6IPC。完整核验派生1640，raw1288/not_loaded352，352次native按ID补读显式并集，不是未修改的1640条原生export，不改canonical。1618旧记录/12问题头、257原件、131历史manifest43251文件、78稳定样本保持。只扩VF56/143/156，其余251不变；254功能/32权威，entry-only待核101→101，另有多个partial。计划10checked/19unchecked，Codex自验，独立复审仅dsh，S08未开始、完整Study未完成，未进入实现。


[57项提交核对](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/probability-typed-crosscheck.json) / [25项事前静态检查](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/static-preflight.json) / [提交前置检查](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/below-commit-contract.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261110-031830-s04-probability-typed-commit/README.md)。
