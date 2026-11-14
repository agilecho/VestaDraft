# 原版命令行与外部交接契约

当前任务对齐（2026-11-10）：S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 当前 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open，即14个未解决；来源 revision10、数据库 revision4 保持。本次状态对齐没有新增目标证据或产品代码。 既有结果与失败边界保持；从 [HANDOFF](HANDOFF.md) 接续。

锁定macOS arm64 VESTA3.90.9a，Deep研究。命令依据VM17完整正文和本地原库，源码发现、独立先验、运行结果分开。无界面退出255的机制见[CLI_COMPLETION](CLI_COMPLETION.md)；255、wrapper0和文件存在都不能单独当成功。本机nogui尚未证明无图形会话或Linux/x86。

以下是R65时期的历史入门摘要；当前逐项结论见本页末尾S07收束及S05最终矩阵。

| 手册命令/参数 | 当时证据及剩余范围 |
|---|---|
| `-i file` | 既有34格式读取/R11/R33与R65选定输入成功；错误/保存空工程/lned崩溃见READER_PATHS/INPUT_VALIDATION，非全部格式通过。 |
| `-o file` | R66同源别名保存与-save逐字节相同，选定原生VESTA格式通过；其它格式待。 |
| `-save [format=rietan] [option=cartesian,pcell,reduced,as_displayed] file` | R69已核VASP默认/Cartesian常规胞、pcell/reduced原胞、as_displayed当前场景；SHELX默认.ins与RIETAN覆盖字段有实物。完整格式、隐形/其它选项及外部精修仍待。 |
| `-calcf source=N lambda=A resolution=d anomalous=N out=FILE` | R67显式source0中性X-ray/1neutron两波长108字段符合、中文空格out成功；anomalous1仅变化观察，空HKL无文件/失败日志但255及后续Save继续。R68默认组合(0/.709317/1/.7)与显式逐字节对照、默认相对cif.txt及缺失目录失败已核；一般文件名/反常科学待。 |
| `-list bond,angle,poly` | R66非空样本12双向键、15键角、1多面体及单字符串组合均具证，76科学字段符合。`cell/site`在空/非空样本均无额外报告；一般几何/错误待。 |
| `-merge_split_site [dmin=f]` | R65默认1e−6Å/.0001Å/.001Å阈值保留3/3/2site；GUI去重/完整metadata/PBC待，关联VF0106。 |
| `-rotate_x angle` / `-rotate_y angle` / `-rotate_z angle` | R65 x30/y40/z−25保存标准矩阵，x后y为RyRx、反序RxRy，输入结构保持。 |
| `-scale factor` | R65乘法缩放，2后.5返回zoom1，结构保持。 |
| `-scale_width_to size` | R65 nogui无变化；R70已运行GUI中设8后Overall Appearance的物理Width实读8Å，SCENE/图像另验。 |
| `-open file` / `-reopen file` / `-close [path]` | R70短共享VESTA_PREF下-open中文路径7/210、外改-reopen8/240、-close指定第二文件保留first均实证；长socket路径失败原因对照，dirty/其它分支仍待。 |
| `-export_img [scale=n] file` | R70已运行GUI原入口：scale1 RGB1112×940、transparent/scale2 RGBA2224×1880/alpha0..255；SVG/PDF/vector失败且发送仍0，必须看接收端与最终内容。相对路径以当前结构目录解析（本案）。 |
| `-flush` | R70在旋转/宽度/PNG序列实际执行并核后续状态；静态刷新调用已定位，独立同步渲染屏障语义未证明。 |

本地parse_cui_command0x1fb34的内部名字已在R71核分派：15个clear/add/del/import名字实际保存别名，4个set_*在首字母s分派返回NULL，add_bond是8bytes返回0的stub；25案与真实完整工程具证。remove确实调用C文件删除，只测试本轮新建可弃载荷。R72 v2site已核个体箭头转位点并幂等；powder普通CIF空DataSet在Powder+28空指针SIGSEGV，原生工具仅报code1/exited，完整参数/粉末科学仍待。

R65的[17案先验及准确argv](../../output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/independent-cli-before-runs.json)、[逐字段比较](../../output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/cli-geometry-crosscheck.json)、[17进程完成](../../output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/cli-run-completions.json)、[原生索引](../../output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/native-evidence-index.json)。每案20秒native期限/独立偏好/工作副本，17target正常255/无超时。339先验字段符合；空列表和宽度的无变化只作观测。

新VS71中C1/C2同点、C3沿a差.0001frac=.0004Å。较大阈值后C1x.125033/occ.5/U.01，只能写为结果后平均/不求和观察，不能替代完整去重物理规则。源/副本SHA和元数据均在先验/比较中；旧70样本记录保持。

外部程序：MADEL原x86_64因绝对gfortran依赖缺失在main前中止，见[MADEL_COMPLETION](MADEL_COMPLETION.md)；TIDY可实际运行、.stin文件名参数协议及标准化/报告差异见ALGORITHMS R56；RIETAN-FP/ORFFE完整调用/输出/失败/许可仍待。本轮不安装依赖、不删除最终功能。

研究接续：R66–R70已补非空列表、calcf显式/默认/错误、选定Save选项和GUIIPC/PNG及失败边界；继续内部命令与其它上下文、外部程序/许可，未核项仍需实际结果或明确不支持原因。完整S07及Study均未完成，独立复审归dsh，尚未开始。

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

## R79 ORFFE结果读取与链接（不改勾选）

实际入口为.ffe查看器，所读直接链未启动外部ORFFE。人工7位点fixture故填9.8765±.0123/123.456±.0045（模型2Å/90°）。首显式identity/码1案读后空表，原成功预测失败；零额外identity/码0联合诊断变体成功，表值9.8765/123.4560目视确认，选行原生详情9.877(13)/123.456(5)和对应原子。主图高亮未目视。SaveAs仅加IMPORT_ORFFE相对链接，尚未重开。GUI正常0/136.708、144.014秒无遗留。新增VS73人工.ffe，旧72保持；13新/830Evidence、12头保持、254功能/73样本/32权威/10完成19未完成。完整外部协方差计算/ORFFE/Study不通过，详见ALGORITHMS R79，Codex自验，dsh复审未开始。

## R80 ORFFE重开/高亮与坏读（不改勾选）

新实例相对IMPORT_ORFFE侧文件自动恢复，无手动Read；原生详情9.877(13)/Si1/O1一致。指定拥有PID91883/主窗口70518两张860×728原图目视确认选择行在O1/Si1产生黄圈，模型区黄色像素0→298，非全渲染验收。只删O6条目7→6后表空、无错误提示，顶部旧详情仍留；失败后原生UI读被180截止阻止，没有回执。坏读SaveAs清IMPORT_ORFFE，逐字节回到原R66无链接5144bytes；原输入/有效侧文件保持。GUI正常0/227.666<soft240，但超180目标47.666且清理晚，allTimingTargetsMet=false。6新/836Evidence、12头/73样本保持，254功能/32权威/10完成19未完成；详见ALGORITHMS R80，Codex自验，dsh复审未开始，完整ORFFE/Study待。


## R81 磁矩反演与时间反演（2026-11-09）

新增VS0074/75：10Å立方，Fe1(.1,.2,.3)，磁矩(1,2,3)μB；两个完整操作只在反演的时间符号±1上不同。首CLI前登记 [独立期望](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-before-targets.json)。一手Shinohara2023 §2.4 Eq7给m′=εdet(W)AW A⁻¹m；本案A=10I、W=−I，故普通反演保持磁矩、带时间反演则变号；magCIF字典只选读坐标/单位和xyz时间符号，全文阅读未完成。

原版两个-nogui导入/Save正常255，完整watch；实际MAGNETIC_CRYSTAL、SYMOP时间列±1、单site分配均保留。VECTR保存Cartesian(1,2,3)，末列type0/Axial（R83修正原坐标模式解释），原先预测晶格(.1,.2,.3)失败并留原计划；没有把表示差异当科学失败。原件自行识别2.4/P−1和2.6/P−1′，这些编号是目标观测，非本轮独立群号验算。[17项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-crosscheck.json)。

两幅归属窗口原生PNG（872×558）已逐一目视：反演行−x/−y/−z、T+1显示mx/my/mz；T−1显示−mx/−my/−mz，符合独立规则。[普通反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/inversion-equivalent-ui.png) / [时间反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/anti-repeat-equivalent-ui.png)。wx表行不暴露于AX，此为原图视觉读数；没有提取实际展开原子上的数值磁矩，也未校准3D箭头方向/长度。polar、非正交、特殊位置、磁超空间与New-dialog编辑仍待，不把VF0091/VF0177或完整磁性科学标作完成。

首截图native调用尚未完成时Codex提前Close/Quit，实际screenshot_window_not_found/单AXApplication节点；失败与 [原因记录](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/native-capture-failure.json) 保留。随后两个新归属进程分别33.382/30.084秒正常0退出，成功取证；首进程75.711秒也正常0，全部<180且无残留。CIF reader0xa671c已有静态证据ID去重，不重复增加记录；新增8条至844，12问题头/旧836证据/原件257/旧73样本不变，稳定样本增至75。80个历史manifest/11112文件完整。研究仍10完成19未完成、Codex自验，dsh独立复审未开始，无产品实现。


## R84 Polar 时间反演、视角与CLI比例（2026-11-09）

新增VS0077：由VS76单位磁矩CIF只把反演时间+1改−1，首target前登记 [独立Polar时间不变预期](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-before-targets.json)。原件CLI导入后，按R83证实的类型位/Scale格式制作新受控Polar1/Scale3副本；其与R83正时间Polar工程仅GROUP2.4→2.6及SYMOP反演时间列不同。GUI新进程回读实际Polar、Cartesian1/0/0、Modulus1.00、Scale3。

**两项原预期失败保留**：①CLI无界面回读/Save [只把VECTS3写回1](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/roundtrip-actual.diff)，其余字节与受控源相同、Polar类型保留；字节一致先验失败，不能推广为全部场景字段/一般读取丢失。首GUI准备断言在启动前停止，实际 [准备失败记录](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/roundtrip-preparation-failure.json) 保留，后续GUI明确选择未改动Scale3受控源。②首GUI原图a轴沿视线、b右c上，箭头正对视线，原注册ROI与参考差18590RGBbytes；不能由此读出方向或宣称时间反演失败。[初始原图](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-minus-main-ui.png)。

结果后另登记 [新进程显式Viewalongc](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/projection-before-target.json)，原ROI[305,80,859,550]没有移动。[沿c观察原图](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-minus-c-view-ui.png) 已目视：a右b上，两个箭头(+a,−a)，与T+1参考相同；该554×470模型ROI逐RGB字节相等，0差。独立物理预期来自VM09 §9.1.1，图像等值是按该不变量检验的原件间一致性，不是外部绝对渲染oracle。限定Polar时间不敏感有证，首失败不回写成通过。[12项核对与失败状态](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-crosscheck.json)。

GUI14360/14733正常0，75.529/37.637秒均<180无残留；一处CUA差分索引解析未找到Edit而中止，刷新完整AX后正常继续，未发额外编辑动作。所有native读取等待结束后才Cancel/Quit。新增5至864Evidence，旧859/12头/旧76样本/原件257与83历史manifest/11566文件保持，稳定样本增77。下一步可追CLI为何未保留VECTS及非正交/一般矢量；完整Study10完成19未完成，254功能/32权威，Codex自验，dsh独立复审/S08未开始，无产品实现。


## R85 -i按相导入而不接管全局样式

[先验与准确argv](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/style-before-targets.json)：已有Polar单位矢量工程仅改全局VECTS为7或.125，另做7后.125两次-i。三案均正常255/无信号/完整watch；输出VECTS1，相数1/1/2，各相VECTR(1,0,0)/末列Polar1及site1分配保持。因此原R84字节一致失败不是矢量类型丢失，属于所测CLI场景样式未接管。

VESTA_cmd_Read0x1e45c/484bytes：0x1e4fc构造临时Scene，0x1e534 ReadFile；tempScene基址sp+0x10，其Crystal vector+0x138位于sp+0x148，0x1e564取begin/end，0x1e574插入currentScene+0x138，只另复制filename+0x28，0x1e5c8销毁临时Scene。readVESTAfile0xa185c将Scene+0x228作为XStyle传给0xa62fc；XStyle::read_style0xb782c的VECTS分支写XStyle+0x98。未见该全局样式转移到current，三个新比例holdout符合。没有读出实际临时内存值，不声称每个输入格式/样式字段都已证。readdata0x982f8 dossier只有204bytes/51指令并有1个未解间接跳转，不能因truncated=false把全部格式switch当已恢复。


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


## R91 构造矩阵与命令调度初始化路径（2026-11-09）

R91仅静态研究，未启动VESTA运行实例。28次原生函数/符号查询成功，17个新增ID、11个已有身份复用。Crystal构造lib0x35f68/472bytes在+150/+178/+1a0创建MatrixDD，并在36074/7c/84调用MatrixD::Initialize0x33b0；该函数_bzero后按stride+1写IEEE double1，故构造期矩阵为单位阵。MatrixDD数据指针+10与旧writer读取Crystal+160吻合。此为静态对象布局证据，不是已读取活体第二相矩阵。

无界面split_command_and_exec(lib0x1fed4/1336)在20264构造VESTA_cmd_Read地址1e45c，202a4比较函数指针，202bc优先执行并从待执行数组移除所有Read；随后20358调用Scene::Build(NULL,-1)，2037c再执行余下保存等命令。GUI事件经execVSTXfile(main100019cb0/568)，100019e68解析、100019e80直接调用处理函数，所见路径无等价Build前置。旧Read函数仅临时Scene读取后插入Crystal指针/复制文件名，无显式Scene::Build调用。正常GUI文件打开OnFileReadImpl(main1000a7680/2280)则在a77f0 ReadFile后有a7d6c Scene::Build。三条执行路径差异已绑定指令与调用点。

Scene::Build(lib71a64/2096)在71b34调用BuildOrientMatrix(712a0/1836)；后者713a0 UnitCell::SetC2Ortho，并715d8/71600等写Crystal+150基底，71880/718a8处理其余矩阵。此前文档泛称“CoreInit”不是本次发现的原生符号，当前应使用这些已绑定函数名。构造单位阵、GUI-i缺少同样的前置Build、R90活体各坐标均Y.1及无界面Y1之间形成具体机制假设；尚无运行时矩阵内存和新holdout，完整导入根因不宣称已验证。

GUI解析器的实际令牌是flush，映射VESTA_cmd_gui_Flush(main100018dc0/92)：虚表槽310/e8/230各调用一次，三个目标尚未解析，不能只由名称宣称能修复数值；不把不存在于该解析器的refresh令牌当已验证命令。下一组应事前登记无界面-save先于-i的参数顺序holdout，以及GUI-i/-save在-flush前后两个新输出，观察是否恢复Y1；原件不修复、旧失败保留。

15项静态限定核对通过。完整证据913，原生raw893/not_loaded20，20条均通过原生按ID接口取回、与旧896逐项一致，明确派生完整快照；不是按上一轮4条遗漏推定。旧896/12问题头/78样本/原件257和90历史manifest/12627文件保持。254功能/32权威、10完成19未完成；Codex自验，dsh/S08与完整Study未完，无产品实现。

[15项静态核对](../../output/vesta-study/20261108-113622/reviews/20261109-152400-s07-import-initialization/initialization-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-152400-s07-import-initialization/README.md)。


## R92 读取优先顺序与flush恢复失败（2026-11-09）

R92在首target前登记两项新对照。无界面参数故意写为-nogui -save OUT -i X -i Y：原版正常255/exited，实际先形成两相，保存X1/Y1、全局VECTS1，整个输出与R87常规-i X -i Y -save的两相文件逐字节相同。事前R91读取优先→Build→其余命令的顺序预测通过，这不是按参数文本从左至右逐条保存的契约。

GUI新进程载入X1/Scale5，第一次IPC-i Y1/Scale7/-save before输出复现两相Y.1/global5。第二次明确发送-flush/-save after；两个客户端均0、父进程仍活，两个不同输出文件分别经两次稳定内容检查，after仍Y.1、与before及R86完整5347字节逐字节相同。因此事前“flush后恢复Y1”的预测失败，不能宣称flush修复初始化，也不把它回写成预期.1。未运行时追踪三个虚调用回调是否逐一执行，不由此推广-flush一般无效。主日志有两次Saved路径；没有Vectors窗口/字段修改，主模型第二相渲染未另验。坐标基底初始化的完整因果仍待，不由无界面顺序成功替代GUI恢复失败。

GUI父57445正常0/64.178秒<180，无期限强制终止/残留；CUA仅在原生读完后绑定实际主窗口并Quit，无已知Cancel崩溃重放。15项证据范围核对通过，原flush恢复预期及originalAllPredictionsPass均false。新增5原生ID至918；本轮实际raw896/not_loaded22，22条经原生按ID补读构成明确派生完整918，旧913/12问题头/78样本/原件257及91历史manifest/12872文件保持。254功能/32权威、计划10完成19未完成；Codex自验，dsh/S08与完整Study未完，无产品实现。下一步转入全功能覆盖缺口盘点并补未观察入口，数值失败/静态假设作为明确边界保留。

[15项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-153900-s07-read-order-flush/read-order-flush-crosscheck.json) / [证据目录](../../output/vesta-study/20261108-113622/reviews/20261109-153900-s07-read-order-flush/README.md)。

## S07 当前命令与运行边界收束（2026-11-10）

本次 Codex 施工、自验及本机 dsh 独立只读复审已完成；S07三项按研究标准通过，保留以下全部能力边界。正式16命令来自 VM17，逐条实际参数索引、GUI依赖、返回/内容/错误和未验证原因分别见[命令合同](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/official-command-contracts.json)、[准确历史argv](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/official-command-argv-index.json)与[原卡矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/S07-MATRIX.md)。这些JSON是研究台账，不是产品API/schema。先前本页34格式是早期历史；当前格式依据为S05最终48输入/23数据输出的逐项结果，full_format_compatibility_accepted=false。

| VM17入口 | 实证与保留边界 |
|---|---|
| `-i` | S05最终48输入/23输出逐格式取具体结果；R85将phase而非完整Scene导入，全局VECTS回默认1；R92 -save放在-i前仍先Read→Build→其余命令。GUI直接handler分派，第二相矢量Y=.1失败保留。 不能由48类研究卡通过推成全格式/全字段兼容；GUI-i通用样式/矢量归属与任意命令排序仍未知。最小补证须用新输入并固定完整Scene/phase预期，已答探针不重跑。 |
| `-o` | R66同源-o与-save输出5144字节逐字相同；真实保存别名。 仅选定VESTA文件实测；其它格式/GUI-o分支未单独验证。若需要该组合，消费S05同格式样本后做别名对照。 |
| `-calcf` | R67显式X-ray/Neutron/两波长108字段；anomalous1只验变化。R68组合默认0/.709317/.7/1与显式逐字对照；CIF默认输出是cwd/cif.txt，原stem假设失败保留。中文空格out=必须是一个argv。 全反常F、其它source/输入后缀默认名/一般LP与资源域未核；沿S06具体unknown/新独立预期补。 |
| `-list` | R66非空SiO6：12有向键/6物理边、15角、1多面体，76科学字段；组合单字符串有效。cell/site在空和非空案无额外报告。 一般几何/PBC/非零SU与其它无效选项未核，不能把内部token当可用功能。 |
| `-merge_split_site` | R65默认1e-6Å、.0001Å、.001Å保留3/3/2site；大阈值坐标/occ/U仅结果观察。 完整metadata/PBC/GUI Remove duplicates仍未知；需不同元素/占位/周期边界的新独立合并规则。 |
| `-rotate_x` | R65 x30/y40/z-25为标准矩阵；x后y是RyRx，反序RxRy；结构保持。R70 GUI x30保存矩阵另验。 任意旋转与渲染像素域未核；不能从矩阵通过推断完整显示通过。 |
| `-rotate_y` | R65 x30/y40/z-25为标准矩阵；x后y是RyRx，反序RxRy；结构保持。R70 GUI x30保存矩阵另验。 任意旋转与渲染像素域未核；不能从矩阵通过推断完整显示通过。 |
| `-rotate_z` | R65 x30/y40/z-25为标准矩阵；x后y是RyRx，反序RxRy；结构保持。R70 GUI x30保存矩阵另验。 任意旋转与渲染像素域未核；不能从矩阵通过推断完整显示通过。 |
| `-close` | R70只关指定second，first仍开且保存cell4/5/6。 无path关闭当前页、dirty Yes/No/Cancel的CLI入口未单独实测；不由GUI R19替代该入口。下次须冻结两页与dirty预期。 |
| `-export_img` | R70 scale1 PNG RGB1112×940；scale2透明RGBA2224×1880/alpha0..255。相对路径落在当前结构文件目录。 其它后缀/无结构时相对路径/全像素未核；GUI vector成功是另一入口。 |
| `-flush` | R70旋转/宽度/PNG序列中已执行；R92显式flush前后保存5347字节相同，未修复Y=.1。静态三虚调用仍未现场全绑定。 独立同步/渲染屏障语义、三callback动态因果未证；不能给适配器杜撰flush完成信号。 |
| `-open` | R70短共享pref下打开中文空格绝对路径second，7Å/V210，first保留。 GUI新tab/当前tab偏好全部组合和无图形会话未知；须具体host另测。 |
| `-reopen` | R70外改工作副本a7→8；reopen前内存7/210，后8/240。 无参/dirty/多相资源所有分支未知；下一探针必须保留导入源和外改前副本。 |
| `-save` | R69 VASP Direct/Cartesian/pcell/reduced/显示场景有具体实物；SHELX默认.ins与format=rietan不同。reduced调用TIDY。omit_hidden/nitech只静态未核。S05最终23数据输出各有成功/失败。 全格式全选项与外部精修未通过。按S05逐格式和S06领域预期消费，不能汇总成全兼容。 |
| `-scale` | R65乘法2后.5回zoom1；R84选定矢量比例与视角分开。 任意因子、GUI所有模式/资源未核；需单独范围预期。 |
| `-scale_width_to` | R65 nogui不变；R70 GUI Overall Appearance Width读8Å，并核后续保存/PNG。 非法宽度、无host或无图形会话均未知；不得将nogui几何保存外推无显示服务。 |

本地内部名字的R71分派结论仍有效：15个clear/add/del/import名字是保存别名，4个set_*在s分派不可达，add_bond是stub；remove实际删除仅测可弃工作文件，v2site是箭头归属转换，powder空DataSet崩溃。名字发现不成为编辑能力通过；这些不属于16条官方命令的新增承诺。

### 完成、失败、归属和真正无图形环境

[完成边界表](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/completion-boundaries.json)逐项对齐D02§4.2/D10。正常nogui的255、CA/监督器0、信号和错误文本、每个命令的产物及接收host完成分别记录。R70发送退出时0字节PNG，后来同一拥有GUI写完后才解码有效；不能固定sleep或sender0冒充完成。R67空HKL、R68坏out目录、R70无SVG/PDF与旧崩溃/超时直接复用，不为矩阵再制造故障。原软期限偏差、默认文件覆盖丢SHA、R86Cancel崩溃及恢复/额外实例事实保持。

**图形环境缺失未运行**：本机是有WindowServer的macOS登录会话；-nogui不是无图形会话证据，清DISPLAY也不能移除macOS图形会话。没有独立无会话主机/已绑定Linux运行件，未改系统会话、安全策略或安装依赖。Linux/x86、真正displayless/container以及GUI不存在时的完整初始化诊断、产物与清理仍未验证；下一探针要用可用的隔离无图形会话与该平台准确运行件，先冻结预期和期限。当前仅有Mac/arm64事实及其中x86_64附带工具运行，不证明Mac原程序x86或Linux/x86可用。

### 外部程序与交付资格

[逐项外部清单](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/external-program-contracts.json)区分真实调用、文件生产者和OS查看器；[本机有界路径核查](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/collaborator-discovery.json)只覆盖PATH及脚本指定位置，不宣称全盘未安装。

| 程序 | 调用和结果 | 明确未验/交付边界 |
|---|---|
| MADEL | R12 PME→madel→.madel；原x86_64因绝对gfortran库缺失main前SIGABRT(-6)，无结果；GUI忽略子返回/无文件静默清理 | 不能记零能量、读取PME或数值成功；不安装/修补/重签原包 |
| STRUCTURE TIDY | R56脚本以.stin文件名参数/cwd样本目录/RIETAN数据库目录，stdout→临时.sto；GUI标准化应用、Niggli报告；R69 CLI reduced另会应用 | 本次补独立子返回：valid0；坏群0但有失败文本/无ATOM；缺输入或Sets.daf为29。4案均<.3秒/6秒限且组清理；未回填R56GUI的未捕获子返回。一般群/失败导入状态/旧.sto读入未知仍保留 |
| RIETAN-FP | 非空偏好走RIETAN.sh，18文件名/stdout.lst；空偏好走本版内置DlgPowder。脚本运行前rm同stem历史输出，后按.plt/.itx/.lst走绘图/查看 | PATH/指定目录没找到运行件；没有实际外部计算/坏路径执行。导出.ins与内置粉末成功均不算RIETAN成功；模板版本须匹配真实运行件 |
| ORFFE | R79/80人工.ffe读取、IMPORT_ORFFE相对链接、重开/选行高亮与坏读清链接；检查的查看器路径没有启动ORFFE | 人工9.8765/123.456与模型2Å/90°不同，证明读取权威；不是精修/完整协方差计算。producer和许可资格未绑定 |
| gnuplot/gnuplot-run.sh、pstopdf、pdfcrop | RIETAN脚本.plt分支到PDF/可选裁剪 | 本机有界查找未见，未运行；pipeline/末个open返回不是外部科学或PDF内容完成 |
| Igor Pro/PowderPlot/Viewer、open/Preview/文本编辑器/文件管理器 | RIETAN脚本.itx/其他分支的OS启动；open/bash可定位 | 没有配置/运行图形链。脚本有未引号的f/$2/路径展开，仅静态风险，不伪造实测故障或自动修补 |
| 其他生产者/浏览器关联 | S05格式生产者、S02Dysnomia等输入来源不是已证VESTA子程序；数据库/OS关联见DATABASE | 不因文件后缀、支持来源或字符串出现，宣称外部程序已调用 |

本地V01 `LICENSE` Version3、版权2006–2026已只读复核：允许使用与原分发文件再分发是不同问题，其限制明确涵盖MADEL/STRUCTURE TIDY。没有取得原文件再分发书面许可；原样本、参数表、模板、二进制及第三方库也不因研究读取而变成可打包资产。RIETAN/ORFFE专业手册可读及论文可引用，均不等于实际运行件版本绑定或再分发资格；各自资格尚待核。最终全部功能目标保持，后续应以获准依赖或原创实现完成，不把缺依赖当删功能依据。本卡未作产品打包或法律许可通过裁定。

[新先验](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/tidy-before-target.json)、[真实CA回执](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/tidy-native.stdout)、[四子进程](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/tidy-child-observations.json)和[内容解析](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/tidy-content-analysis.json)区分外层0与子返回。新增VS0103/0104及1条原生观察；旧24问题头逐项读回相同，无实质新未知头更新。旧2036记录逐字不变，新总2037是逐文件SHA磁盘派生索引，未冒称全量raw导出。

S07复审低优先级修订：TIDY子返回明确为runner-reported（Python Popen.wait/OS等待），CA主体是外层Python且保存回显/文件，未独立逐child捕获；原始0/29/0/29不改。官方16合同增加observed_exit与准确native回执、signal来源和host上下文；flush/save/width/image四个next_probe_plan细化输入/独立候选预期/容差/期限/完成判据，全部是未来未运行计划、不是本卡新增授权。GUI只有x旋转实测，y/z标明未验证。dsh原审查全文/6项发现保留，Codex仅作合同与文档修订，没有新运行。

S07实际dsh独立复审三项/整卡pass，无硬阻断；见[原文](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/dsh-review.stdout)及[裁定](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/review-result.json)。已按研究范围勾选三项，Study25/4，交付后停止。以上未验证原因不转为能力通过。
