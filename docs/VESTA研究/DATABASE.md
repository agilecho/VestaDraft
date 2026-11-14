# 数据库入口与空白原因

默认数据库空白的机制已解释；完整数据库问题现revision4/high/investigating；R60已验证选定HTTPS COD公共查询/原生导入/保存，提供者切换与错误/取消/dirty矩阵尚未完成。原件没有被修补，Study步骤保持10完成/19未完成。

## 原件与网络对照

既有构造体`DlgSearch` 0x100077a04直接传入`http://www.crystallography.net/cod/search.html`给`wxWebView::New`，见[原构造体](../../output/vesta-study/20261108-113622/s04/database-constructor-body-study.stdout)。本轮原件空白GUI PID6258中Search Database仍为空HTML/空URL；手工输入同路径HTTPS后真实显示COD搜索页；先清空至about:blank再输入HTTP则保持空白：[默认树](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/database-default-ui-authorized.stdout)、[HTTPS树](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/database-https-ui.stdout)、[HTTP对照树](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/database-http-from-blank-ui.stdout)。HTTPS原生树截断在500节点，足以核标题/表单存在，不是完整页面遍历。

当前公共HTTP和HTTPS经cURL均返回200、12116字节、零重定向，见[获取记录](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/endpoint-acquisition.json)。这不是服务端强制HTTP→HTTPS的证据，也不能用cURL成功替代WebView策略。远程页面提供COD ID、文本、文献、晶胞/元素等表单；它是外部网页，不是已证明的原生查询API。

## 实际错误码

[WebView失败代理](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/webview-provisional-error-study.stdout)在0x100137bf0接收NSError并调用`nsErrorToWxHtmlError` 0x100137864；[转换函数](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/webview-nserror-map-study.stdout)先比较`NSURLErrorDomain`，再取错误码。[原件ObjC stub/selector交叉核对](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/objc-error-selector-static-crosscheck.json)由原arm64字节解出domain、isEqualToString:、code，支持域相等判断的语义。

原包不能被直接调试时，使用此前已派生的R14唯一标识副本。副本只改Info三项标识与调试签名；两架构TEXT/库完整字节等同原件。[原生调试运行](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/native-webview-error.stdout)、[逐事件轨迹](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/webview-error-events.jsonl)记录默认请求和about:blank后再次HTTP请求两次回调：域比较为true；0x100460dc8读取到实际selector为`code`且receiver等于回调NSError；0x1001378ac读到signed64=-1022。没有求值表达式或显式改业务数据，软件断点插桩仍须计入派生观察边界。HTTPS在副本中亦显示表单，最后exit_status0、无回调异常。

本机Apple SDK常量[选读记录](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/apple-sdk-error-constant.json)将-1022定义为`NSURLErrorAppTransportSecurityRequiresSecureConnection`；[Apple官方错误定义](https://developer.apple.com/documentation/foundation/nsurlerrorapptransportsecurityrequiressecureconnection-c.enum.case)说明它表示ATS拒绝缺少安全连接的请求。[WebView ATS官方说明](https://developer.apple.com/documentation/bundleresources/information-property-list/nsapptransportsecurity/nsallowsarbitraryloadsinwebcontent)解释WebView豁免键；原件及副本都没有NSAppTransportSecurity配置，目标arm64构建SDK为26.2。[来源获取和实际阅读范围](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/apple-source-acquisition.json)区分网页JS外壳、搜索摘要与实际取得的短Markdown正文。

据此，默认HTTP被当前WebView的ATS策略拒绝可解释空白；HTTPS原件对照提供独立行为支持。并未直接在未改原件抓取NSError，不能抹去派生观察这层区别，也不能由此宣称所有网络环境均有同样结果。

## 失败控制与启动边界

Apple nscurl ATS诊断自动改用HTTPS，默认/允许任意加载都超时，其它配置有TLS错误；35秒监督终止，见[诊断观察](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/ats-diagnostic-observation.json)。该不完整诊断本身不能证明HTTP被ATS拒绝，结论依据后续实际WebView回调。

用户已明确81929为研究遗留实例，可正常关闭；复查时它已退出，见[用户裁定与状态](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/legacy-process-user-decision.json)。原件首个带-i GUI尝试PID6157在VESTA_cmd_Read/execVSTXfile崩溃；保留[崩溃选段](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/original-gui-crash-selected.json)，不将其混作数据库错误。随后空白GUI需点击Don’t Reopen恢复提示后进入；stderr仍含Failed to create an IPC service，不能宣称已完整理解IPC生命周期。

[CUA操作摘要](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/cua-observation.md)与CA原生树分开。原件PID6258正常退出0；Quit后误读AX导致默认空白实例自动重启，已菜单关闭并确认未运行。副本正常退出0。没有修改原包、网络代理、ATS例外或系统安全设置；未取得显式WindowServer ID正控制图。

## 查询与导入仍待核

[导航函数0x1000785b4](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/database-navigation-study.stdout)取URL最后一个斜线起的后缀交给原`check_format`。返回非负时，manager的虚表+0x8d0收到URL，随后清事件对象+0xb8；间接callee及事件字段含义未解析，不能把它写成已证实的CIF下载/导入。识别器并非只识别CIF。[URL输入](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/database-url-entry-study.stdout)与[状态更新](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/database-update-state-study.stdout)给后续路径，尚未验证完整返回/取消/错误传播。

接续先核提供者菜单与一个公共查询，再抓下载/交接/取消/dirty、格式识别及产物；不上传私人文件。既有254功能ID与37样本保持，VF-0011只从入口待测更新为所选行为部分观察。原生[问题更新](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/database-unknown-update.stdout)、[读回](../../output/vesta-study/20261108-113622/reviews/20261108-190718-s04-database/database-unknowns-after.stdout)：169 evidence，6 resolved/3 investigating/0 open；其余8问题字节不变，来源8/截图2仍调查中。

## R60 COD 公共查询、原生导入和保存

锁定未改原件从独立VS64工作CIF进入File/Search Database，默认HTTP仍为空，原URL/ATS没有修补。手工在既有URL栏输入 **https://www.crystallography.net/cod/search.html**，实际COD搜索表单完整载入；公开编号**1000000**只提交一次，结果页显示1条结构、公式C5H17AlN2O8P2、P1 21/n 1及7.8783/10.4689/16.068Å、β95.147°。原生[表单](../../output/vesta-study/20261108-113622/reviews/20261109-072000-s04-database-import/database-https-form-ui.stdout)与[查询结果](../../output/vesta-study/20261108-113622/reviews/20261109-072000-s04-database-import/database-query-result-ui.stdout)均未截断。toolbar Menu点击未显露提供者列表，不冒称提供者切换已验。

结果表CIF链接在CUA截图中定位后点击；CUA随后仍显示网页缓存树，但进程范围原生AX已显示 **1000000.cif - VESTA**、新标签及实际结构Output，原工作标签仍保留：[第一次原生导入](../../output/vesta-study/20261108-113622/reviews/20261109-072000-s04-database-import/database-after-cif-link-ui.stdout)。随后隐藏的freshAX切到了主窗口，泛化close选择误关主窗口而非数据库窗口；[关闭后树](../../output/vesta-study/20261108-113622/reviews/20261109-072000-s04-database-import/database-after-window-close-ui.stdout)只剩查询页，计划首保存文件确实不存在。此方法失误、导入成功但首保存未完成分开保留，未覆盖失败或重提交查询。

第一实例正常结束后，从官方[COD CIF](https://www.crystallography.net/cod/1000000.cif)另作一次公共GET，状态200、**12580bytes/SHA892c59ffee1b218f0059ca86df287056ffed34a68d5426a5be6a5c00a31933bc**，保留headers/命令/退出码；web打开工具不能直接取CIF，原始获取由原生记录的cURL完成。[获取记录](../../output/vesta-study/20261108-113622/reviews/20261109-072000-s04-database-import/cod-source-acquisition.json)。该源另登记VS0069，COD的[CC0声明](https://www.crystallography.net/cod/search.html)及源文件文献保留：Phan Thanh,Marrot,Renaudin,Maisonneuve，ActaCrystC56(2000)1073–1074。它是在第一次原生导入后取得，并不声称与临时wxWebResponse数据流逐字节同一；本轮没有捕获那条临时流的SHA。

新原件实例通过SearchDatabase的可编辑URL栏直接交入同一CIF URL，只补原生导入/保存、不重跑查询。显式读取当前标题1000000.cif后SaveAs；系统面板的Where最初指向历史R56，先GoToFolder切到本轮R60，回读Where和文件名，再写 **database-retry-imported.vesta**。原目录未污染，原工作CIF仍保持，原基线标签和新保存标签并存。[保存后原生树](../../output/vesta-study/20261108-113622/reviews/20261109-072000-s04-database-import/database-retry-saved-ui.stdout)。新文件12065bytes，GROUP14/setting2，保存晶胞7.878300/10.468900/16.068001、β95.147003；与另取CIF源字段一致。

在补证GUI前解析公共源的35位点标签/元素、坐标、占位及晶胞（该解析晚于首次导入，不是首次目标前的物理预测）。SaveAs中**35对标签/元素、146个数值字段**全部在预定cellÅ/angle1e−5、fractional1e−6、occ1e−5内相符；空间群号14一致。[逐字段一致性](../../output/vesta-study/20261108-113622/reviews/20261109-072000-s04-database-import/database-import-crosscheck.json)。这是来源数据一致性和功能链路验证，不是独立科学精度/ADP/对称性算法验收；GUI显示262atoms/266bonds/12polyhedra是显示边界展开后的场景计数，不误当源35位点数。

新静态OnFileRead0x100092338（624bytes）区分http/https/ftp，wxWebSession::CreateRequest/Start异步请求；事件回调0x1000a8144（624bytes）GetStream→**_tmpfile 0x1000a8184**→流拷贝→OnFileReadImpl0x1000a7680（2280bytes），其0x1000a77f0调用ReadFile，再建立文档/标签及RecentFiles。与R15导航后缀识别/vcall和本轮真实URL导入相互支持；没有运行寄存器追踪来冒称精确证明每次虚调用目标，也没有把静态HTTP支持写成ATS已修复。

GUI68095/68595分别正常0、220.658/162.645秒，soft360前。第二次Quit关闭主窗口后数据库窗仍活着；清理脚本过早读取尚未写出的lifetime报FileNotFoundError，保留方法说明。先ps核同一拥有PID仍活着，才继续AX观察并关闭实际剩余数据库窗，最后只ps确认无目标，末次关闭后不读AX。[清理记录](../../output/vesta-study/20261108-113622/reviews/20261109-072000-s04-database-import/gui-cleanup.json)。未出现新实例重启，原包与稳定源保持只读。

数据库问题仅从revision2更新为**revision3/high/investigating**：选定COD公共查询/原生下载交接/保存已覆盖，提供者切换、Stop/Reload/Back、非CIF后缀、认证/网络错误/取消/dirty等矩阵仍待核；R15“尚未查询”是历史记录，当前状态由新头覆盖。13新至546 Evidence，旧533逐记录保持，其它11问题头不变，总12头10resolved/2investigating。前59包7468文件及原发布包257保持；新VS69、旧68记录完全保持。254功能/69样本，10完成19未完成，完整数据库与Study继续，Codex自验/dsh独立复审未开始。

## R61 数据库导航、404 与未保存状态

复用VS64工作CIF，在GUI将a7→8，V105→120，不保存源。打开数据库窗口后使用HTTPS搜索页与公开tips.php，依次链接到帮助页、Back回search.html、Forward回tips.php、Reload当前tips.php；每步当前URL及原生完整树保存于本轮，未重提交公共结构查询。[逐项核对](../../output/vesta-study/20261108-113622/reviews/20261109-071658-s04-database-controls/database-controls-crosscheck.json)。Back/Forward按钮的CUA启用状态随历史位置切换；部分时刻窗口标题为空或仍为前页，而URL和HTML已反映目标，不以标题单项替代导航结果。没有为这种瞬态标题现象声称完整事件根因追踪。

Stop在上一帧显示enabled时点击，点击后页面已经完整载入。原生poststate保留，但**实际传输取消未证明**，不能由按钮调用成功标作网络取消通过。静态OnBack/OnForward/OnStop分别48bytes，调用WebView虚表slot0x828/830/810；OnReload52bytes调用slot0x7b0、参数0，随后均UpdateState。R15已存的UpdateState292bytes更新导航状态/标题/URL，新处理体与源身份均保留。该静态路径不替代实际网络取消证据。

选定DlgSearch构造体2552bytes只硬编码COD HTTP默认页，创建Back/Forward/Stop/Reload/URL及Menu；Menu在0x1000780dc以ID−1创建，保存到对象+0x460。23项DlgSearch符号目录没有独立OnMenu/提供者切换处理函数，本轮点击Menu也未显示提供者弹层。只据此记录**此构造/目录/GUI路径未建立提供者切换**，不推断全应用或其它版本绝无其它入口。R60“提供者矩阵待核”因此缩小为需要证实的其它路由，不凭控件名创造提供者清单。

URL输入已识别的 **https://www.crystallography.net/cod/999999999.cif** 后，原件实际出现“Could not load the file: Error: not found (404)”错误提示。[原生错误与文档状态](../../output/vesta-study/20261108-113622/reviews/20261109-071658-s04-database-controls/controls-missing-cif-result-ui.stdout)。OK关闭提示后没有新增导入标签，网页仍为帮助内容、URL栏为失败CIF地址。该404来自实际原件提示，不是独立HTTP测量或伪造服务返回；没有把这次失败当CIF解析/下载通过。

显式核对实际数据库窗口标题/URL后关闭它，原始工作结构仍唯一标签，a8/V120的Output全文与打开数据库前完全相同；之后Quit实际给出未保存提示，明确No，工作CIF与稳定VS64仍为原SHA。原生[关闭后状态](../../output/vesta-study/20261108-113622/reviews/20261109-071658-s04-database-controls/controls-after-browser-close-ui.stdout)及[未保存提示](../../output/vesta-study/20261108-113622/reviews/20261109-071658-s04-database-controls/controls-dirty-quit-prompt-ui.stdout)证明dirty没有因浏览/错误/关窗被清掉。未测试此轮Yes/Cancel（之前文件关闭选项另有R19证据），也未把保存失败/认证/SSL/超时/FTP/非CIF/实际Stop中止矩阵写成全验。

原GUI69289正常0/328.770秒，soft360前；No后只ps，无AX重启/目标遗留。DB头revision3→**4/high/investigating**，仅追加选定控制/404/dirty观察与Stop未证范围，其它11头保持；19新至565 Evidence，旧546逐记录保持。前60包7636文件及原发布包257保持，全部69样本记录保持。254功能、10完成19未完成，完整数据库和Study继续，Codex自验/dsh独立复审未开始。
