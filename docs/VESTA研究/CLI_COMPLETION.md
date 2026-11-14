# 无界面命令的返回状态

锁定macOS arm64 VESTA 3.90.9a，选定的`-nogui -i … -save …`返回链。它解释已经观察到的有效保存/明确读取失败都退出255；不是全部命令、平台或科学正确性的验收。原件不变，完整Study仍未完成。

## 原始代码与数据

| 层次 | 确认的路径 | 证据 |
|---|---|---|
| 库parse_cmd_line 0x20480 | 识别`-nogui`后GUI启动标志为0；0x20528调用split_command_and_exec，0x2052c把保存的标志x21写回x0，没有转发命令结果 | [原库dossier](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/parse-command-study.stdout) |
| split_command_and_exec 0x1fed4 | 解析输入/命令、更新类型/Build并执行保存；这里不是最终进程返回码的提供者。间接命令调用仍有未解析限制，未宣称完整命令契约 | [dossier](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/split-command-study.stdout) |
| VESTA::OnInit 0x1000901d8 | 0x100090334调用parse；返回0时0x100090338跳到0x100090398，设返回0并经0x100090934/94c退出 | [原主程序dossier](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/app-init-study.stdout) |
| wxApp::CallOnInit 0x100147388 | 保存并返回OnInit的布尔结果；false不继续普通GUI事件循环 | [dossier](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/wx-call-init-study.stdout) |
| wxEntry 0x10036c8b0 | CallOnInit返回false时0x10036ca20转到ca5c，读取应用对象+0x224，最后返回该值 | [dossier](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/wx-entry-study.stdout) |
| wxAppConsoleBase构造体0x10030d6a0 | 0x10030d738从全局0x1006c7268取32位值，0x10030d73c存入对象+0x224 | [构造体](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/app-base-constructor-study.stdout) |
| 原主程序数据0x1006c7268 | 4字节`ff000000`，小端整数255 | [真实字节](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/app-default-exit-bytes.stdout) |

这里的255是此目标中预设且实际返回的255，不能描述为已经证明“-1被POSIX截断”。所读汇编和运行寄存器均支持直接255；不从低质量寄存器伪码推断。

## 受控返回轨迹

临时调试副本继续使用上轮派生身份；两架构TEXT和库完整字节不变。以已有C1工程为选定保存控制，只读返回寄存器、调用者返回地址和应用字段，不执行目标表达式，不改原件/全局设置。[逐事件记录](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/cli-return-events.jsonl)、[真实CA运行](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/native-cli-returns.stdout)、[核对摘要](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/cli-return-crosscheck.json)：

| 观察点 | 实际值 |
|---|---|
| parse入口的对象+0x224 | 255 |
| parse_cmd_line返回 | 0 |
| VESTA::OnInit返回 | 0 |
| wxApp::CallOnInit返回 | 0 |
| wxEntry返回 | 255 |
| 子进程退出 | 255，已生成选定工程 |

原件证据仍独立保留：此前[有效CIF转换](../../output/vesta-study/20261108-113622/s05/native-cli-cif-authorized.stdout)与明确失败均返回255；R10[只补空格变体](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/startup2-width10-observation.json)返回255并保存300位点，原样本大写路径255却保存空工程。副本返回轨迹与原件这些结果交叉支持机制，但不能改写为原件直接被附加调试。

原生问题`unk_b84f44506e209f4a`现为revision2/resolved，[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-174304-s07-cli-return/cli-unknown-verify.stdout)valid=true。resolved只表示这条返回路径已解释，未修复返回行为。

## 后续完成判定的研究约束

逐命令保留返回码/信号/超时、错误文本、产物存在与内容/科学字段的不同维度。不能由255单独判断成功，也不能由存在输出单独认可有效，因为失败可能写空工程。原件崩溃-11、AppKit恢复弹窗超时等不经过上述正常返回链，分别记录；不把255规则外推为所有命令永远返回255。

完整CLI命令矩阵、命令顺序效果、外部程序和协议仍待S07后续研究；当前254功能ID不因这条机制新增命令全集。S07只进入选定问题补证，步骤仍未勾选。

R52将原件转换输出.lned作-nogui -i/-save输入，target−11/SIGSEGV且0byteggrid；同字节改.led后target255/1916byte有效zero441网格。文件存在和wrapper退出0均不是有效完成；unknown unk_41a422ce94027042 rev2 investigating，详见ALGORITHMS R52。

R53 PID60060原件crash报告UUID绑定原二进制，故障lib0xb1b9c的NULLScene对象数组（x8=0/idx0），在exportGridFile前；cmdRead失败后后续Save仍执行且无objects guard，fopen(w)/close预检先于dispatch留下0byte。选定原因resolved，不是CLI错误契约修复或全路径验收，见ALGORITHMS R53。

R65新增[CLI权威规格](CLI.md)，17有界target正常255但另以339字段/产物验收，三轴旋转/顺序、乘法scale与去重阈值具证；空列表及nogui宽度无变化保持局部限制。返回链本文件职责不扩为全命令通过。

R66八个正几何/别名target均正常255、无信号/期限，以内容/76数值字段和-o字节一致另验收，不由255直接认成功。无界面重建7/6/1和各列表具证，见[CLI](CLI.md)R66。

R67 calcf空HKL失败无表与成功有表同为target正常255，后续-save仍执行，必须逐命令验产物/领域内容；显式中性/中子两波长108字段另通过。见[CLI](CLI.md)R67。

R68有效输入但out父目录不存在时，无反射表/Failedtowrite仍target255并执行后续Save；默认cif.txt可覆盖共享cwd前表。默认组合运行对照已核，采集偏差/恢复和原失败假设保留，见[CLI](CLI.md)R68。

R69九个保存target正常255、各具体VASP/INS内容另验；as_displayed初始闭盒假设失败不以255掩盖，修正场景模型/新先验变体后核对。reduced实际TIDY产物和变换输出不由简单文件存在推定，见[CLI](CLI.md)R69。

## R70 已运行GUI的命令交接、路径限制与图像导出

继续复用VS70/P1/cell4×5×6Å/V120、C(.125,.2,.3)/O(.625,.4,.5)、occ1/.5与Uiso.01/.02；不新增样本。首target前固定两个文件、a7→8的外部工作副本修改、单文件关闭、Rx30、8Å显示宽度、PNG两倍缩放/透明及SVG假设。[初始登记](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/gui-cli-before-hosts.json)。不传-nogui的发送实例与已运行原版GUI通过IPC交接；parse_gui_command0x100019b98/280bytes、vxConnection::OnExec0x100091e50/772bytes的CWD前缀/事件排队，及Open/Reopen/Close/ExportImg/Flush/ScaleWidth原生处理函数已静态定位。原发布包不修改。

**长路径失败与短共享目录对照**：长偏好目录对应实际服务名148 UTF8字节（日志原文保留）；发送方出现existing instance may be too busy提示，主窗口无打开对话框。进程sample指向OnInit→wxMessageBox→NSAlert模态循环；先独立偏好目录、再共用原长目录均未交接。两个原发送target达到idle18秒、SIGTERM/记录code1；独立诊断子进程16.186秒后归属明确地终止−15，外层正常0，不把其当成功。长接收实例正常退出0/364.394秒，**超过360秒软时限4.394秒**，未触发570秒硬清理。

观察后、短目录target前登记[对照假设](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/short-pref-transport-before-host.json)：分别用新归属短目录/tmp/v70oc-0929与/tmp/v70rd-0929，发送方与接收方共用同一个VESTA_PREF，服务名30字节。实际socket/listener由lsof及socket类型确认。冻结OnInit证据中_prefdir+vesta_+wxGetUserId构造服务名；dyld GOT0x1005F80A8绑定libVESTA/_prefdir，本机SDK sys/un.h的sun_path容量104。长失败/短成功与静态链**强支持所测长路径超限导致IPC失败**，没有测精确边界，也没有证明所有busy提示都由此引起。更改研究环境与修复原软件是不同事项；原包保持。

**打开、重新加载、指定关闭**：短路径发送-open含空格中文绝对路径后，原first页保留并出现second页/a7/V210。只改新工作副本a7→8，-reopen前UI最后报告仍7/210；-reopen同绝对路径后8/240。-close只关second，first保留；随后-save导出first仍cell4/5/6、V120、原位点/占位/Uiso。四个发送实例均正常0，实际窗口/输出另验，未把客户端退出当状态完成。

**视图、flush和PNG**：接收端执行-rotate_x30/-scale_width_to8/-flush/-export_img scale=1，保存项目SCENE的3×3矩阵与独立Rx30先验在六位小数半单位内相符；Overall Appearance→Projection/Width实读**8Å**。scale1普通PNG **1112×940/RGB**；option=transparent/scale2得到**2224×1880/RGBA、alpha0..255**，宽高各2倍。flush仅确认在此命令序列中可执行和随后实际图像状态，不推断独立渲染屏障语义。

**两个原假设不成立**：format=vector scene.svg不生成文件且接收端报Failed to saved an image。观察后先登记再做PDF/vector、SVG无选项和PNG/vector对照；PDF/SVG仍无文件/明确失败，PNG/vector有效RGB。ExportImg0x100018930/1012bytes始终走CaptureImage→ExportRaster0x1000a9bcc/1352bytes，没有矢量导出函数调用；这解释所测CLI路径不能完成矢量导出，不能推翻R21所测GUI矢量导出成功，也不声称所有CLI图像后缀都已覆盖。第二个补充先验曾预测相对输出落在发送cwd，但实际relative image 中文.png落在**当前结构文件目录**；原生处理函数在Scene已有绝对文件名时以其为base转绝对路径，不能由CWD协议推导所有输出都按发送cwd解析。两项原失败均保留。

**异步完成可观测**：普通PNG发送进程退出0时，其after快照仍为0字节/空SHA；稍后接收端实际写出PNG，父进程完整after快照与独立Pillow解码确认1112×940及最终SHA。另有长共享pref的VESTA.ini在接收端正常退出时更新。这两处快照变化由[后续归属父证据核对](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/watch-lineage-analysis.json)，不回写先前快照、不把即时fileexists/0返回当完成。短目录偏好文件另做归档且与原路径SHA一致；三个host均正常0，短host88.790/144.845秒（实际lifetime为准），无遗留进程。

[27项选定校验及原失败](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/gui-ipc-crosscheck.json)区分首先验、观察后解释与后续先验。38新至714Evidence、旧676及12头10resolved/2investigating保持；254功能/72样本、32权威、10完成19未完成。首次证据导出因探针名与导出stem相同被wrapper回执覆盖，KeyError在写索引/权威文档前检出，原回执保留；重新只读导出为native-evidence-final.json，不重跑产品target。完整CLI/IPC/Study未完成，Codex自验，dsh独立复审未开始。

## R71 CLI命令名与真实分派：保存别名、死分支和删除

复用VS54家族/R66独立规则SiO6工程：10Å立方、Si居中、±2Å六O、Si/O1.9..2.1Å规则，7sites/6bonds/1polyhedron；源SHA为308e23fd073960bbf173585382110bb33f0af3de3f14b089c51346efd9a92c56。每案复制源到新cwd，所有偏好/输出亦在归属目录，原档保持；72样本记录完全不变。**首target前**固定25案argv、输入/被删测试文件SHA、同源-save输出应逐字节相同的预期及20秒target期限。[先验/25案](../../output/vesta-study/20261108-113622/reviews/20261109-094200-s07-cli-parser-aliases/aliases-before-targets.json)。

R65的libVESTA parse_cui_command0x1fb34/928bytes含许多编辑命令字符串，本轮逐跳读汇编，最初推断19项都指Save。实际结果纠正为**15保存别名+4未识别set项**，不能按字符串出现赋予编辑功能。

**15项保存别名**：clear_bond/clear_isosurf/clear_slice/clear_vector，add_isosurf/add_slice/add_vector，del_bond/del_isosurf/del_slice/del_vector，import_phase/import_struc/import_vol_isosurf/import_vol_coloring。各给一个新.vesta绝对输出路径，随后显式-save另一文件；前后两个输出均与标准-save及源工程完全相同，15×2+基线/其它案共**40个工程文件**同为5144bytes/上述SHA，保留原6bonds/1polyhedron，未导入或修改对象。解析器共同尾0x1fe84..0x1fe88实际返回cmdSave0x1e654指针。这是所测本地版本的真实分派，不宣称这些名字实现了编辑能力；等值面/切片/矢量各非空状态未由本轮构造。

**4项原别名预期失败**：set_bond/set_isosurf/set_slice/set_vector都没有生成alias-output，只有随后显式-save保存不变工程。汇编首字符分派把s带到0x1fce4，只比较save与scale；set_*两者不符，经0x1fd08→0x1fe7c返回NULL。字符串set_*虽出现在后面的比较，却位于首字符d的分支，不能被首字符s的输入走到。原19条文件生成预期中**15通过、4失败**，四行失败留在[完整比较](../../output/vesta-study/20261108-113622/reviews/20261109-094200-s07-cli-parser-aliases/aliases-crosscheck.json)，结果后控制流解释单独标注；不将修正后的结论写成19原先验全部通过。

**add_bond**单独分派到0x1e314，原函数仅8bytes：mov w0,#0/ret，无调用。给Si/O/dmin0/dmax100后7/6/1和工程仍完全不变，不能把正常退出当添加成键成功。**clear_bond无参数**同样未清六键，后续-save继续；对参数不足的Save别名不猜测编辑语义。

**remove是真实文件操作**：0x1f948/24bytes先检查argc≥2，否则返回−1；满足时取argv[1]尾跳C remove。只用本轮创建的disposable 中文.txt/固定载荷（另保留原始载荷文件）做三案：存在文件的before SHA真实、after消失；缺失文件保持缺失；无参数不删任何文件。三个案后续-save均写出原工程，原件正常255，说明内部remove失败不自动中止后续命令或改变最终退出码。未触及原包、源工程或任何历史证据，未验证所有目录/权限/相对路径错误。

25个target均正常255/20秒内、无信号/超时、完整cwd watch/noGUI/无遗留。分析helper正常0/完整watch；两次新静态研究分别确认stub和remove。首次静态调用误用Frameworks/libVESTA路径而not_found，无Evidence/未运行产品target，失败回执保留，读取既有R65权威argv后用实际Contents/MacOS/libVESTA.dylib新名修正，不覆盖失败。28新至742Evidence、旧714/12头10resolved2investigating保持。前70包9501文件/原257保持，254功能/72样本、32权威、10完成19未完成。完整CLI/编辑/Study未验，Codex自验，dsh独立复审未开始。Powder/v2site及外部联动继续研究，不按内部名称虚构功能。

## R72 单原子箭头转位点与内部powder空数据集崩溃

复用VS53反演样本：10Å立方/P−1，C(.1,.2,.3)/occ.5和O原点/occ1/B0；独立周期唯一C2+O1，闭0..1显示C2+O8=10。首GUI前登记源SHA、单C原子轴矢量(1,2,3)、位点/单原子区别和有界归属进程，[运行前GUI先验](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/vector-before-gui.json)。复读此前已取得的VM09 §9.1.1–9.1.3，仅取单原子/位点分配与axial TP M/polar M公式，记录本地正文SHA；不把本机二进制算法当独立科学预期。

**GUI源与输入精度变化**：原版GUI只给IndividualAtom首C1(.1,.2,.3)分配箭头1，另一个C1(.9,.8,.7)初始Vector空。创建框实际设置Cartesian(1,2,3)，Tab后Modulus显示3.74，点OK后列表/文件成为**(.99956,1.99911,2.99867)**、晶格表示(.09996,.19991,.29987)。因此初始请求分量精确保留的假设未成立；结果后数学解释v×3.74/√14符合五位打印半单位，不伪称首运行前精度oracle。源VECTR实际个体行**1 1 0 0 0**，单独SaveAs individual-vector.vesta后源SHA2bc29cc38688cc9c2253eb9a6495b97e4fbe246e2d952c3eb177ce5300ef92fa，后续CLI运行前另登记其实际分量/文件身份。此处研究既有界面提交序列的行为，没有修改原版。

**v2site真实转换**：lib0x1e31c/320bytes遍历各Crystal的AtomArrow集合，个体分配按5整数一组取首siteID去重加入siteIDs，再清空个体分配；AtomArrow构造0xd314/124bytes和ResetArrowIDforSite0x6c824/408bytes交叉定位字段，未由函数名猜成体数据操作。CLI基线-save与GUI源逐字节相同。-v2site后仅VECTR个体行由**1 1 0 0 0→1 0 0 0 0**，其余所有工程字节保持（含组件、类型、cell、结构/占位/B/视图）；连续两次与一次产物逐字节相同，SHA**dfb204be2c964ccc1bbfee001d7f35d861893fe9e891c0582425eb2b4b75ad50**。这是个体分配转换为位点分配的幂等性，不是重新计算向量分量。

新归属GUI重开转换工程：CrystallographicSites里C1的Vector=1、O空；IndividualAtom两个C1的Vector列均空，主渲染实际有**两个同方向红箭头**。初始回读补充先验曾预测IndividualAtom两行也显示1，实测不成立；该表显示直接个体分配，不能把空列当位点箭头未生效。CUA截图直接观察网格和两箭头，原生AX保留对话框/控件状态但wx网格行未暴露，未声称它能机器提取行内容或对3D角度作数值校准。轴矢量经反演同向符合独立TP M公式，本轮未测polar/磁性时间反演或所有非正交表示。

**内部powder不是可直接用普通CIF完成的命令**：0x1f8e0/104bytes在Scene非空后取Scene+0x150的数据集集合首指针，然后调用GetFormFactor、DataSet::PrepareCalc0xceb50、Simulate0xcec38/784bytes。它不读命令argv也没有导出调用；静态存在计算链不证明本次到达。首CLI前固定6案，其中四个有效基线/v2site案正常255。第五案为VS55普通CIF（读取本身正常、两site/10Å立方）执行-powder再-save；原生工具记录**code1/reason=exited**且无Save文件，运行器断言停止，第六powder-extra-argument案**未执行**，不继续碰同一崩溃。

本次PID**2124**系统诊断文件完整归档，[PID/故障摘录](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/powder-crash-source.json)及原始powder-2124-crash.ips：EXC_BAD_ACCESS/**SIGSEGV11**、KERN_INVALID_ADDRESS0，故障lib imageOffset **0x1f8fc=Powder+28**，原生指令**ldr x20,[x9]**、x9=0，绝对PC−imageBase相符，原库UUID **5d1ec64c-a09f-3f1d-97a2-79f51cd3dce2**由dwarfdump核一致。具体原因是有Scene但**DataSet集合为空，解引用begin未判空**，发生在GetFormFactor/PrepareCalc/Simulate调用之前；不能称为完成衍射计算后的数值错误，也不把GUI粉末计算功能一概判坏。无需再执行崩溃或安装外部程序。

后续归属GUI出现“上次意外退出/是否恢复窗口”系统提示，原生AX保留后选择Don’t Reopen，不恢复旧窗口，随后实际打开研究副本。两个GUI均正常0/各小于360秒（lifetime原件为准），Quit后ps无进程/无AX重启；短偏好目录文件另归档。**native的reason=exited/code1不能独立证明无信号**，此处必须结合系统SIGSEGV记录，不改本研究锁定工具或伪造其退出字段。

[20项选定校验、三项拒绝先验与边界](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/powder-v2site-crosscheck.json)和[停止记录](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/cli-run-failure-recovery.json)区分4正常255、1系统SIGSEGV、1未运行。22新至764Evidence、旧742/12头10resolved2investigating保持；前71包9798文件/原257保持。254功能/72不变样本、32权威、10完成19未完成。完整粉末科学、axial/polar及CLI/Study未完成，Codex自验，dsh独立复审未开始；未将查明原因当修复原件。


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
