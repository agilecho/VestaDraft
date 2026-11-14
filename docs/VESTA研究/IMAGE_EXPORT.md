# 原件图像导出：实物、格式兼容与失败

File/Export Raster Image和Export Vector Image已经分别观察格式菜单；已实际执行十种Raster与四种Vector入口；JPEG2000成功提示却无产物，RGB无效，TIFF透明开关本例无效，不能宣称全格式通过。尚未完成全部图像格式/显示选项，不勾选S05。

## 实际格式入口

| 入口 | 本轮实际菜单 | 已运行范围 |
|---|---|---|
| Raster | BMP、EPS、JPEG、JPEG2000、PNG、PPM、RAW、RGB、TGA、TIFF | 十项已执行，具体文件/失败范围见R20/R22；全参数未核 |
| Vector | EPS、PDF、PostScript、SVG | R20仅入口；R21四种实物，PDF/SVG独立渲染、EPS/PS结构检查 |

相同EPS扩展名分别出现在两入口，尚未核文件内容/生成机制，不能按后缀把它们视作同一导出行为。[CUA完整动作与菜单](../../output/vesta-study/20261108-113622/reviews/20261108-204840-s05-image-export/cua-observation.md)。这些图像菜单另于既有结构/体数据格式清单统计，不改变47输入/23数据输出来源表的范围。

## Scale与透明组合

保存PNG路径后，Export image显示Scale x和透明背景复选框，无直接宽高输入；与Preferences的栅格tessellation factor（VF0171）分别记录。

| 组合 | 实际文件 | 独立PNG解码 |
|---|---|---|
| Scale1、透明=1 | [carbon.png](../../output/vesta-study/20261108-113622/reviews/20261108-204840-s05-image-export/carbon.png) | 1112×940、8-bit RGBA，26138字节 |
| Scale2、透明=0 | [carbon-scale2.png](../../output/vesta-study/20261108-113622/reviews/20261108-204840-s05-image-export/carbon-scale2.png) | 2224×1880、8-bit RGB，62429字节 |

宽高均翻倍；这两组选项组合没有分离全部交互/所有透明配置。透明图alpha最小0/最大255：1024167个全透明、18771个全不透明、2342个部分alpha像素，合计1045280。第二图无alpha通道。[独立文件检查](../../output/vesta-study/20261108-113622/reviews/20261108-204840-s05-image-export/png-file-inspection.json)和[联合核对](../../output/vesta-study/20261108-113622/reviews/20261108-204840-s05-image-export/image-export-crosscheck.json)给SHA/参数/尺寸。绝对1112×940是当次视口导出的观察基准，不是所有设备/窗口默认尺寸或独立预定值。

第二次进入时文件类型仍PNG，但选项对话框复位Scale1/透明0。本轮未核跨实例持久化，也未绑定PNG编码库/标准具体版本；当前结论为实际PNG输出和独立解码兼容性。

## 实际画面与身份

两文件已逐张查看：[视觉记录与原图](../../output/vesta-study/20261108-113622/reviews/20261108-204840-s05-image-export/visual-qa.md)。可见一颗着色碳球、晶胞线和罗盘，无应用菜单/侧栏/Output文字；晶胞线到图边，未宣称全部对象/裁切/透视/字体与视口逐像素相同。Scale2预览被查看器缩放，文件原尺寸由解码核实。白底预览不能证明是否透明，alpha来自文件像素。

这是File/Export Raster Image的产物，不能替代`runtime-ui-read --window-id`正确窗口截图正控制；截图unknown仍revision2/investigating。原生[parent](../../output/vesta-study/20261108-113622/reviews/20261108-204840-s05-image-export/native-export-gui.stdout)watch记录了carbon.png与工程输入；Scale2新路径未在watch清单，作为独立本地文件/SHA/解码产物留存，二者归属不得混称。

R19 Yes文件逐字复制到本轮后，新原件GUI读回a5、体积80，[独立新实例树](../../output/vesta-study/20261108-113622/reviews/20261108-204840-s05-image-export/image-export-new-instance-ui.stdout)。本轮源/工作工程SHA均1af8c190…，导出未改工程，Quit无保存提示，PID28169正常exit0/466.398秒、非deadline。原包/最初C1源保持。

## 样本和剩余

首次实际图像格式代表登记VS-0038=carbon.png；原37个ID/文件保持，Scale2只是该样本的诊断变体，不新分VS0039。截至R20为254功能、38样本、196 evidence；9个原生问题头不变。继续实际Vector与其它Raster格式、全部参数/失败、输入格式/磁性准晶/PBC、领域及CLI；完整Study未完成，Codex自验，独立复审归dsh。

## R21四种Vector实物与兼容性边界

新原件PID28517读取同一工程逐字副本，依次实际保存PDF、SVG、EPS、PostScript，均成功提示、无额外参数对话框。工程SHA仍1af8c190…；退出0/337.508秒、非deadline，无dirty提示。[CUA动作](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/cua-observation.md)和[交叉核对](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/vector-export-crosscheck.json)分别说明实际操作和原生/文件证据。

| 实物 | 独立结构与渲染范围 |
|---|---|
| [carbon.pdf](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/carbon.pdf) | 10498字节，PDF1.4，1页，MediaBox1112×940点；Poppler与pypdf可读，无image/XObject，三组shading、Helvetica文本，已逐页渲染查看 |
| [carbon.svg](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/carbon.svg) | 89294字节，width1112px/height940px、相同viewBox；1086polygon/5polyline/3text、无image；XML解析与librsvg渲染查看 |
| [carbon.eps](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/carbon.eps) | 105350字节，PS-Adobe-3.0 EPSF-3.0、LanguageLevel3，BoundingBox0 0 1112 940；尚无独立渲染 |
| [carbon.ps](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/carbon.ps) | 105406字节，PS-Adobe-3.0、LanguageLevel3、Portrait和同BoundingBox；尚无独立渲染 |

[文件结构与SHA](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/vector-file-inspection.json)。元数据均自报GL2PS1.3.8/VESTA，未据此直接认定完整实现库版本。1112×940点与px分别按文件原单位记录，不宣称物理尺寸换算相同。本机没有独立PS解释器，cupsfilter查询亦无转换过滤器；保留失败结果，不把成功提示/DSC结构当视觉通过。Raster入口EPS仍未实际生成，不能合并两个EPS行为。

PDF/SVG渲染显示一颗碳球、晶胞线、罗盘，但高光/标签与R20 PNG明显不同。PDF操作符和SVG文本属性均编码蓝色Helvetica12；球心半径50选区7833像素中RGB和>600：PNG668、PDF0、SVG0。[视觉记录](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/visual-qa.md)与[量化](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/appearance-comparison.json)。这是观察，不证明内部原因或全设置相同，不签栅格/矢量画面等价。SVG色带、PDF平滑表现也保留；色彩、字体、光照和复杂场景继续待研究。

原生parent实际watch PDF/SVG/EPS和输入，PostScript新路径不在watch，PS的生成归属为CUA选择与本地实物。新增2条native至198，9问题头不变；四个不同格式代表依次VS0039 PDF、VS0040 SVG、VS0041 EPS、VS0042 PS，原38记录完全保持，渲染PNG不分VS号。当前254功能/42样本，10完成19未完成；所有图像选项与完整Study仍未完成，独立复审归dsh尚未开始。


## R22其它九种Raster实际执行与三项异常

原件PID28787读取同一工程副本，九个入口各Scale1，另加TIFF透明=1。十次均成功提示；正常退出0/258.724秒、工程不改。[CUA摘要](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/cua-observation.md)。PNG之外八个实际格式产物登记VS0043—0050，旧42条保持，JPEG2000无文件不编样本，透明TIFF是诊断变体。

| 输出 | 文件/独立结果 |
|---|---|
| BMP | 3135894字节，RGB1112×940，严格解码可读 |
| Raster EPS | 6359072字节，Level2，Creator为bitmaplib/PaulBourke，固定hex流包含3135840字节RGB，与BMP解码像素相同；不等于Vector EPS的Level3图元，也未签通用PS解释器通过 |
| JPEG | 24707字节，RGB1112×940可读，与BMP像素不相等，未核编码质量映射 |
| JPEG2000 | 实际保存carbon.jp2且成功提示，目录无该文件，不接受成功 |
| PPM | P6，RGB1112×940可读，像素与BMP相同 |
| RAW | 3135840字节无头，恰等BMP的top-down交错RGB；自动识别失败是无头预期，不与SGI坏文件混称 |
| RGB | 3135880字节，magic474/1112×940/3通道，但严格SGI解码截断；40字节前缀后是top-down交错RGB，与SGI规范布局不符，不接受有效格式 |
| TGA | RGB1112×940可读，解码像素与BMP相同 |
| TIFF | RGB1112×940可读；透明0与1两文件逐字节相同，均3样本/像素、无alpha/ExtraSamples；透明选项未生效 |

[完整文件/像素/SHA报告](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/raster-file-inspection.json)和[视觉范围](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/visual-qa.md)。只有TIFF（及R20 PNG）在本次选项显示透明复选框，其余八项只有Scale，未见JPEG质量参数。TIFF透明=1另有[原生AX](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/tiff-transparent-options-ui.stdout)。JPEG2000的无文件、RGB坏文件和TIFF透明无效是原件行为，不进行产品修复，也不改变科学/格式正确性要求。

[SGI自述Version1.00规范（原SGI站归档镜像）](https://ftp.zx.net.nz/pub/archive/ftp.sgi.com/graphics/grafica/sgiimage.html)要求512字节头、非RLE通道平面和左下原点；本件比该固定头+所需像素少472字节，且40字节起载荷是交错top-down。本地[获取记录](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/sgi-spec-fetch.json)固定正文，未以扩展名当规范。[静态写出路径](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/writer-path-analysis.json)：原件write_JPEG2000_file在0x1000c1138只有mov w0,0/ret；ExportRaster索引3保留先前wxImage保存返回值并移除临时路径，未把转换返回值复制到结果；自定义WriteBitmap SGI分支只写短前缀，TIFF索引9转code6自定义RGB writer不接alpha。与实物一致，但本轮未做现场断点把GUI各调用逐个绑定，机制诊断仍investigating，不写完整动态证明。

parent watch九个预定路径但文件字节预算达到上限、file_list_cut_short=true，大文件没有原生完整SHA；本地完整实物另行独立SHA/解码。新增透明TIFF路径未watch。不能把本地完整报告冒称原生捕获完整，也不能因预算省略把缺失JP2单独当原生无文件证明；实际目录查验与GUI操作共同说明当前结果。

新增9条native至207，新问题unk_ff152c8e4008160b revision2 investigating；旧9头不变，当前10问题=6resolved/4investigating/0open，254功能/50样本。后续绑定三异常现场路径、全部Scale参数/错误与复杂场景；S05、完整Study均未完成，独立复审归dsh且尚未开始。


## R23三项Raster失败的现场路径（诊断已关闭，功能仍失败）

使用既有R14签名/包标识副本，主代码TEXT/库与原件一致；[本轮前置摘要](../../output/vesta-study/20261108-113622/reviews/20261108-215250-s05-raster-paths/debug-copy-preflight.json)核已注册派生主程序和原库SHA，[CUA动作](../../output/vesta-study/20261108-113622/reviews/20261108-215250-s05-raster-paths/cua-observation.md)只控制该副本。不是直接附加未改原件。原生[运行](../../output/vesta-study/20261108-113622/reviews/20261108-215250-s05-raster-paths/native-raster-paths.stdout)记录[逐事件轨迹](../../output/vesta-study/20261108-113622/reviews/20261108-215250-s05-raster-paths/raster-path-events.jsonl)，无表达式求值/显式业务数据写入，但使用软件断点插桩。

1. JPEG2000实际formatIndex3：wxImage::SaveFile(type1)返回1；转换函数收到存在的`trace.jp2~`，返回0；随后删除同一临时文件，ExportRaster仍返回1、菜单收到1，成功提示但无JP2/临时文件残留。静态两条指令的空转换器由真实调用证明，忽略转换结果导致假成功。
2. RGB实际index7→WriteBitmap(type4)，收到RGB指针及1112×940。输出SHA与R22原件RGB完全相同，仍40字节头/交错top-down载荷、严格SGI解码失败。通用writer确实用于本轮格式，短头/布局原因与实物对应。
3. TIFF透明=1先创建含alpha的wxImage：实际读取1045280个alpha字节、min0/max255，非空或全不透明伪缓冲；index9随后转WriteBitmap(type6)，传入同一RGB指针及尺寸，没有传alpha。输出RGB TIFF逐字节等于R22原件透明/不透明两文件。故透明信息在最终自定义RGB写出路径丢失，不归咎于复选框未勾或图像解码器。

[实物/轨迹交叉核对](../../output/vesta-study/20261108-113622/reviews/20261108-215250-s05-raster-paths/raster-path-crosscheck.json)。0回调错误，副本目标exit0，原生watch本轮未截断，研究工程未改。派生实物只是原失败样本的诊断复现，不新增VS编号；原50记录不变。

`unk_ff152c8e4008160b`从revision2/investigating到revision3/resolved，[原生更新](../../output/vesta-study/20261108-113622/reviews/20261108-215250-s05-raster-paths/raster-validation-resolve.stdout)和[真实验证器](../../output/vesta-study/20261108-113622/reviews/20261108-215250-s05-raster-paths/raster-validation-verify.stdout)valid=true。resolved指三项所选原因查清，不指编码器补齐/SGI修复/TIFF透明通过；没有修改原件和格式正确性要求。当前208证据、10问题=7resolved/3investigating/0open，其余9头不变，254功能/50样本，10完成19未完成。剩余Scale参数/错误/批量、复杂场景和字体光照、Vector EPS/PS独立渲染、向量外观原因，以及其余S04—S08全范围继续研究；独立复审归dsh且尚未开始。

## R70 已运行GUI的命令交接、路径限制与图像导出

继续复用VS70/P1/cell4×5×6Å/V120、C(.125,.2,.3)/O(.625,.4,.5)、occ1/.5与Uiso.01/.02；不新增样本。首target前固定两个文件、a7→8的外部工作副本修改、单文件关闭、Rx30、8Å显示宽度、PNG两倍缩放/透明及SVG假设。[初始登记](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/gui-cli-before-hosts.json)。不传-nogui的发送实例与已运行原版GUI通过IPC交接；parse_gui_command0x100019b98/280bytes、vxConnection::OnExec0x100091e50/772bytes的CWD前缀/事件排队，及Open/Reopen/Close/ExportImg/Flush/ScaleWidth原生处理函数已静态定位。原发布包不修改。

**长路径失败与短共享目录对照**：长偏好目录对应实际服务名148 UTF8字节（日志原文保留）；发送方出现existing instance may be too busy提示，主窗口无打开对话框。进程sample指向OnInit→wxMessageBox→NSAlert模态循环；先独立偏好目录、再共用原长目录均未交接。两个原发送target达到idle18秒、SIGTERM/记录code1；独立诊断子进程16.186秒后归属明确地终止−15，外层正常0，不把其当成功。长接收实例正常退出0/364.394秒，**超过360秒软时限4.394秒**，未触发570秒硬清理。

观察后、短目录target前登记[对照假设](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/short-pref-transport-before-host.json)：分别用新归属短目录/tmp/v70oc-0929与/tmp/v70rd-0929，发送方与接收方共用同一个VESTA_PREF，服务名30字节。实际socket/listener由lsof及socket类型确认。冻结OnInit证据中_prefdir+vesta_+wxGetUserId构造服务名；dyld GOT0x1005F80A8绑定libVESTA/_prefdir，本机SDK sys/un.h的sun_path容量104。长失败/短成功与静态链**强支持所测长路径超限导致IPC失败**，没有测精确边界，也没有证明所有busy提示都由此引起。更改研究环境与修复原软件是不同事项；原包保持。

**打开、重新加载、指定关闭**：短路径发送-open含空格中文绝对路径后，原first页保留并出现second页/a7/V210。只改新工作副本a7→8，-reopen前UI最后报告仍7/210；-reopen同绝对路径后8/240。-close只关second，first保留；随后-save导出first仍cell4/5/6、V120、原位点/占位/Uiso。四个发送实例均正常0，实际窗口/输出另验，未把客户端退出当状态完成。

**视图、flush和PNG**：接收端执行-rotate_x30/-scale_width_to8/-flush/-export_img scale=1，保存项目SCENE的3×3矩阵与独立Rx30先验在六位小数半单位内相符；Overall Appearance→Projection/Width实读**8Å**。scale1普通PNG **1112×940/RGB**；option=transparent/scale2得到**2224×1880/RGBA、alpha0..255**，宽高各2倍。flush仅确认在此命令序列中可执行和随后实际图像状态，不推断独立渲染屏障语义。

**两个原假设不成立**：format=vector scene.svg不生成文件且接收端报Failed to saved an image。观察后先登记再做PDF/vector、SVG无选项和PNG/vector对照；PDF/SVG仍无文件/明确失败，PNG/vector有效RGB。ExportImg0x100018930/1012bytes始终走CaptureImage→ExportRaster0x1000a9bcc/1352bytes，没有矢量导出函数调用；这解释所测CLI路径不能完成矢量导出，不能推翻R21所测GUI矢量导出成功，也不声称所有CLI图像后缀都已覆盖。第二个补充先验曾预测相对输出落在发送cwd，但实际relative image 中文.png落在**当前结构文件目录**；原生处理函数在Scene已有绝对文件名时以其为base转绝对路径，不能由CWD协议推导所有输出都按发送cwd解析。两项原失败均保留。

**异步完成可观测**：普通PNG发送进程退出0时，其after快照仍为0字节/空SHA；稍后接收端实际写出PNG，父进程完整after快照与独立Pillow解码确认1112×940及最终SHA。另有长共享pref的VESTA.ini在接收端正常退出时更新。这两处快照变化由[后续归属父证据核对](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/watch-lineage-analysis.json)，不回写先前快照、不把即时fileexists/0返回当完成。短目录偏好文件另做归档且与原路径SHA一致；三个host均正常0，短host88.790/144.845秒（实际lifetime为准），无遗留进程。

[27项选定校验及原失败](../../output/vesta-study/20261108-113622/reviews/20261109-092000-s07-gui-cli-ipc/gui-ipc-crosscheck.json)区分首先验、观察后解释与后续先验。38新至714Evidence、旧676及12头10resolved/2investigating保持；254功能/72样本、32权威、10完成19未完成。首次证据导出因探针名与导出stem相同被wrapper回执覆盖，KeyError在写索引/权威文档前检出，原回执保留；重新只读导出为native-evidence-final.json，不重跑产品target。完整CLI/IPC/Study未完成，Codex自验，dsh独立复审未开始。
