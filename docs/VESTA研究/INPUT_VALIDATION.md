# 选定输入验证边界

本节解释既有VS-0013截断网格和VS-0014负晶胞为什么进入读取/保存。只针对这些输入、当前macOS arm64路径；不宣称完整健壮性、资源边界或科学有效性验收。原件没有修复，完整Study仍未完成。

## VS-0013：短读后复用最后一个标量

本例是二进制pgrid，160字节，不是文本数据。原函数读取152字节头部，声明8×8×8=512个载荷浮点数，文件实际只剩2个。原库`readGRIDfile` 0xbd1d8的载荷循环在0xbda74调用`fread_LE`后没有根据短读返回值中止；`fread_LE` 0x27acc只有4字节，直接尾跳`_fread@stub`，自身也没有补齐/报错逻辑。[读取dossier](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/grid-reader-study.stdout)、[辅助函数](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/fread-helper-study.stdout)。

相同TEXT/库字节的临时副本只读观察真实参数、返回寄存器和目标数据：[逐事件](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/grid-short-read-events.jsonl)、[交叉核对](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/grid-short-read-crosscheck.json)。

| 观察项 | 实测 |
|---|---|
| 载荷读取调用 / 实际返回项 | 512 / 2 |
| 返回0的载荷读取 | 510次 |
| 第一、第二个输入值 | 11.202439308166504、2.7015416622161865 |
| 物理512体素中的值 | 第一值1个，第二值511个；缺失510个都复用了上次标量 |
| 内部数组 | 729个值，包含(8+1)³的周期边界副本；不当729个独立体素 |
| 函数/进程 | 读取返回x0=0；进程255，保存工程引用原始截断输入 |

既有[原件读取记录](../../output/vesta-study/20261108-113622/s05/native-VS-0013.stdout)及保存工程保持；没有声称新建了可验证完整的网格输出，保存的VESTA文件仍引用原始160字节pgrid。本轮内存值是副本直接捕获，原库静态及旧原件结果独立保留，不能标为原件被附加调试。未认可重复值补全为有效输入。

## VS-0014：数值转换成功，没有执行正长度约束

[干净的最终轨迹](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/negative-cell-final-events.jsonl)、[核对摘要](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/negative-cell-crosscheck.json)显示：

1. CIF行`_cell_length_a -2`先按`%127s %127s`取两个文本token；数字token`-2`进入`getValueAndEsd` 0x27658，`%f(%f`实际转换1项，得到float -2。[数字转换器dossier](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/cif-number-parser-study.stdout)。
2. `readCIFfile` 0xa671c返回x0=0，场景有一个晶体、两个位点，晶胞参数仍`[-2,3,4,90,90,60]`。[CIF dossier](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/cif-reader-study.stdout)。
3. 解析阶段没有命中SetC2Ortho；随后`Scene::BuildOrientMatrix`在0x713a0调用它，参数仍为该负长度数组，返回后也未修改。[场景矩阵dossier](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/scene-cell-matrix-study.stdout)、[SetC2Ortho](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/cell-to-ortho-study.stdout)。不把未执行的CIF分支当本例实际路径。
4. 本轮保存参数与既有原件`VS-0014.vesta`一致，保留a=-2。显示体积20.784610，与独立算得的绝对体积幅值20.7846096908在6位小数舍入内；正体积不证明负长度符合既有样本的正长度前置条件。

数字词法允许负数本身适用于坐标等字段；本例问题是执行的晶胞导入/建模路径没有拒绝不满足既有正长度条件的值。这里不外推所有CIF字段都无校验，也不修改原程序的兼容行为。

## 仪器失败与证据边界

第一次按文本读pgrid失败后改按二进制/原读取代码核对，未改样本。早期负晶胞轨迹只覆盖CIF解析，SetC2Ortho未命中；后移范围才确认它在建模阶段调用。一个在启动前建立广泛scanf断点的版本在进入CIF前超时，具体原因未定，不能记为输入失败或套用先前AppKit原因；[该回执](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/native-negative-cell-flow.stdout)保留。

后续作用域限定版本捕获了正确转换，但返回断点再命中时出现19个未匹配pending的KeyError，并把字符串目标错误解读为float；[旧轨迹](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/negative-cell-scoped-events.jsonl)保留，不能用于完整/无错误轨迹结论。最终新脚本只在CIF内放scanf断点，返回后禁用且容忍无pending，只对`%f`格式读取数值，0 callback_error，完整走到保存/255。原生回执中的debugger0、超时版本1及目标255分别记录。

问题已更新后的一次读回误用了缺失锁定目录的CLI路径，启动失败回执保留；随后[正确绝对路径读回](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/validation-unknowns-after-absolute.stdout)和导出/校验完成，未更改全局工具配置。

原生问题`unk_a1948891885cdc8d`为revision2/resolved，[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/validation-unknown-verify.stdout)valid=true。它表示两个既有失败接受机制与已观察边界已解释，不表示输入验证已经充分、原件已修复或S05已完成。维数/计数溢出、极大资源、其它字段约束、错误后的dirty/恢复状态及其它格式版本仍未验。

目前151条原生evidence，9问题（6 resolved/1 investigating/2 open）。254功能、37样本、10勾选/19未勾选保持；模型/产品实现均未开始。

R46LineProfile输入N0/N1均在接受SavePanel后归一2，两个有限端点；native实际字段0/1与最终header2分开。OnLProfile局部count<=1写2、同slot传export，静态/运行相符。负N/文本/hugeN未运行，不能泛化所有解析。见[体数据规格](行为/03-体数据与二维.md)。

## R72 单原子箭头转位点与内部powder空数据集崩溃

复用VS53反演样本：10Å立方/P−1，C(.1,.2,.3)/occ.5和O原点/occ1/B0；独立周期唯一C2+O1，闭0..1显示C2+O8=10。首GUI前登记源SHA、单C原子轴矢量(1,2,3)、位点/单原子区别和有界归属进程，[运行前GUI先验](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/vector-before-gui.json)。复读此前已取得的VM09 §9.1.1–9.1.3，仅取单原子/位点分配与axial TP M/polar M公式，记录本地正文SHA；不把本机二进制算法当独立科学预期。

**GUI源与输入精度变化**：原版GUI只给IndividualAtom首C1(.1,.2,.3)分配箭头1，另一个C1(.9,.8,.7)初始Vector空。创建框实际设置Cartesian(1,2,3)，Tab后Modulus显示3.74，点OK后列表/文件成为**(.99956,1.99911,2.99867)**、晶格表示(.09996,.19991,.29987)。因此初始请求分量精确保留的假设未成立；结果后数学解释v×3.74/√14符合五位打印半单位，不伪称首运行前精度oracle。源VECTR实际个体行**1 1 0 0 0**，单独SaveAs individual-vector.vesta后源SHA2bc29cc38688cc9c2253eb9a6495b97e4fbe246e2d952c3eb177ce5300ef92fa，后续CLI运行前另登记其实际分量/文件身份。此处研究既有界面提交序列的行为，没有修改原版。

**v2site真实转换**：lib0x1e31c/320bytes遍历各Crystal的AtomArrow集合，个体分配按5整数一组取首siteID去重加入siteIDs，再清空个体分配；AtomArrow构造0xd314/124bytes和ResetArrowIDforSite0x6c824/408bytes交叉定位字段，未由函数名猜成体数据操作。CLI基线-save与GUI源逐字节相同。-v2site后仅VECTR个体行由**1 1 0 0 0→1 0 0 0 0**，其余所有工程字节保持（含组件、类型、cell、结构/占位/B/视图）；连续两次与一次产物逐字节相同，SHA**dfb204be2c964ccc1bbfee001d7f35d861893fe9e891c0582425eb2b4b75ad50**。这是个体分配转换为位点分配的幂等性，不是重新计算向量分量。

新归属GUI重开转换工程：CrystallographicSites里C1的Vector=1、O空；IndividualAtom两个C1的Vector列均空，主渲染实际有**两个同方向红箭头**。初始回读补充先验曾预测IndividualAtom两行也显示1，实测不成立；该表显示直接个体分配，不能把空列当位点箭头未生效。CUA截图直接观察网格和两箭头，原生AX保留对话框/控件状态但wx网格行未暴露，未声称它能机器提取行内容或对3D角度作数值校准。轴矢量经反演同向符合独立TP M公式，本轮未测polar/磁性时间反演或所有非正交表示。

**内部powder不是可直接用普通CIF完成的命令**：0x1f8e0/104bytes在Scene非空后取Scene+0x150的数据集集合首指针，然后调用GetFormFactor、DataSet::PrepareCalc0xceb50、Simulate0xcec38/784bytes。它不读命令argv也没有导出调用；静态存在计算链不证明本次到达。首CLI前固定6案，其中四个有效基线/v2site案正常255。第五案为VS55普通CIF（读取本身正常、两site/10Å立方）执行-powder再-save；原生工具记录**code1/reason=exited**且无Save文件，运行器断言停止，第六powder-extra-argument案**未执行**，不继续碰同一崩溃。

本次PID**2124**系统诊断文件完整归档，[PID/故障摘录](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/powder-crash-source.json)及原始powder-2124-crash.ips：EXC_BAD_ACCESS/**SIGSEGV11**、KERN_INVALID_ADDRESS0，故障lib imageOffset **0x1f8fc=Powder+28**，原生指令**ldr x20,[x9]**、x9=0，绝对PC−imageBase相符，原库UUID **5d1ec64c-a09f-3f1d-97a2-79f51cd3dce2**由dwarfdump核一致。具体原因是有Scene但**DataSet集合为空，解引用begin未判空**，发生在GetFormFactor/PrepareCalc/Simulate调用之前；不能称为完成衍射计算后的数值错误，也不把GUI粉末计算功能一概判坏。无需再执行崩溃或安装外部程序。

后续归属GUI出现“上次意外退出/是否恢复窗口”系统提示，原生AX保留后选择Don’t Reopen，不恢复旧窗口，随后实际打开研究副本。两个GUI均正常0/各小于360秒（lifetime原件为准），Quit后ps无进程/无AX重启；短偏好目录文件另归档。**native的reason=exited/code1不能独立证明无信号**，此处必须结合系统SIGSEGV记录，不改本研究锁定工具或伪造其退出字段。

[20项选定校验、三项拒绝先验与边界](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/powder-v2site-crosscheck.json)和[停止记录](../../output/vesta-study/20261108-113622/reviews/20261109-095000-s07-powder-v2site/cli-run-failure-recovery.json)区分4正常255、1系统SIGSEGV、1未运行。22新至764Evidence、旧742/12头10resolved2investigating保持；前71包9798文件/原257保持。254功能/72不变样本、32权威、10完成19未完成。完整粉末科学、axial/polar及CLI/Study未完成，Codex自验，dsh独立复审未开始；未将查明原因当修复原件。


## R86 GUI样式保留、Cancel崩溃与数值隔离未成（2026-11-09）

首target前 [登记GUI5/传入7对照](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/gui-style-before-target.json)：当前一相X矢量(1,0,0)/全局5，传入新相Y矢量(0,1,0)/全局7，经短共享VESTA_PREF发送-i/-save。客户端0/exited，另核父进程仍活及新5347byte文件稳定；保存两相标题current_style_five/incoming_style_seven、全局VECTS5，前后原生AX比例5、后相下拉表含两名。所测“仅追加相数据、保留当前全局样式”得到GUI反例控制，非总重置1/非取传入7。未发送-refresh，未验第二相主模型渲染。

**原两项预期失败**：保存第二相rawY为.10000，预期1.00000，原始分量一致性失败；尚未证明重开后的物理磁矩损坏。另，父进程18317在Vectors的Cancel恢复预览时SIGSEGV11/−11，146.850秒<180且无残留，但正常退出/正常cleanup失败。先前对话称“Quit时”已被系统栈修正，不能把工具序列中的Quit当已证触发动作。[11项证据核对及原失败](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/gui-style-crosscheck.json)。

具体上下文：先在1相时打开/Cancel Vectors，再CLI追加为2相，重开Vectors查看相列表后Cancel。[系统报告原档（历史文件名含quit）](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/VESTA-18317-quit-crash.ips) 与 [地址/UUID/寄存器绑定](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/vector-cancel-crash-binding.json)：main UUID cd806142…，PC0x1000214e8执行ldp x1,x2,[x20]，x20/FAR=0x300；调用栈OnCancel→SetPreviewMode(false)→VectorBase赋值。还原循环以现有Scene相数为界，却索引backupScene+0x4a0的相数组，无旧长度检查，再加箭头向量偏移0x300；空源指针与报告吻合。旧备份实际长度未读内存，1→2相造成旧备份不匹配是有据推断，未宣称全部根因/所有Cancel场景。

崩溃后另登记两个无界面数值隔离对照；第一个Y-only在18秒idle后被SIGTERM终止（native code1/reason idle_timeout、wrapper0），无命令输出/无文件。研究脚本错误地先读预期文件而报FileNotFoundError；raw失败/脚本保留，第二个X/Y对照未调用，不能形成数值结论。[停止记录](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/headless-control-stop-record.json)。下轮先用新有界拥有实例检查是否有启动恢复提示；超时原因尚未确认，不原样盲重跑，也不重放已知两相Cancel崩溃。

新增7至878原生来源记录；raw原生导出876/not_loaded2，[完整核验派生快照878](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/verified-complete-evidence.json) 保留旧871及12头、77样本、原件257、85历史manifest/11873文件。两个旧记录仍经原生ID解析补齐，无记录重写。254功能/32权威、10完成19未完成；Codex自验，dsh独立复审/S08待，无产品实现。
