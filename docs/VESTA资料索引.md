# VESTA 重写资料索引

当前状态：2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

## 使用规则

- 本项目目标以 [VESTA 全功能重写要求](VESTA全功能重写要求.md)为准，平台契约以[接入技术要求](接入技术要求.md)为准；参考代码不是业务裁定。
- Craft Anything 源码基线为 `1f8508a104134c436cbba5ea2f675619ef47702f`；IntegrateCraft 本轮参考 HEAD 为 `dbb09f78c8188ec2f9b531ac7f89fe87efe13c70`，存在用户文档迁移改动。只读参考，实施接入时重新锁定，不恢复或清理其现有改动。
- 外部源码链接为本机绝对路径，行号是本轮定位辅助；定位优先核提交和符号。旧报告中的 `/Volumes/Samsung_T5/AI/…` 已迁移，当前源码位于 T9，以下使用 `/Volumes/T9/Development/Sources/Artcraft/craft-anything`。
- 官方在线手册标注 2022-08-10。S02 已重读正文、快捷键和书目目录；S02.3 曾因来源不足撤销，R134 补证并经 dsh/验收后恢复完成。S04 About 已核本地为 3.90.9a（2026-08-08），不能用旧手册代表新版全部功能。完整格式/科学数值和目标绑定仍按 S04–S07 核对。
- 后续产生的功能、格式、算法和行为规格必须反向记录本页资料 ID、具体小节/符号及证据；新发现的规范、论文和外部程序资料在首次使用时加入本页。
- 未创建的研究产物在计划中标为“本步创建”或“前置产物”，不冒充现有资料。已开始执行 Study；当前真实产物从 HANDOFF 定位。

## 本项目文档

| ID | 直接入口 | 执行时读哪里、用来做什么 |
|---|---|---|
| <a id="d01"></a>D01 | [VESTA 全功能重写要求](VESTA全功能重写要求.md) | §1 全部功能；§2 基线；§3 方法；§4 默认行为差异；§6 完成标准 |
| <a id="d02"></a>D02 | [接入技术要求](接入技术要求.md) | §2 用户结果；§4 命令/协调；§5 保存；§6 隔离；§7 中文；§8 构建；§9 Agent；§10–11 验收和交付 |
| <a id="d03"></a>D03 | [AI 入口](AI入口.md)、[软件研究与功能边界](方法/02-软件研究与功能边界.md) | 任务理解、真实接口、研究基线、证据分类、未知项和格式边界 |
| <a id="d04"></a>D04 | [软件研究与重建验证](专题/14-软件研究与重建验证.md) | 观察计划、行为规格、同样输入对照、独立预期 |
| <a id="d05"></a>D05 | [领域模型与编辑算法](专题/01-领域模型与编辑算法.md)、[图形与三维](专题/10-图形CAD三维与材质软件.md) | 实体身份、单位、坐标、事务、撤销与科学/几何不变量；不代替晶体学专业依据 |
| <a id="d06"></a>D06 | [命令内核与多端架构](方法/03-命令内核与多端架构.md)、[契约模板](模板/命令与页面接口契约.md) | 同一内核、schema、对象/修订、batch、错误、完成时刻和接口文档 |
| <a id="d07"></a>D07 | [Web 与实时交互](方法/04-Web化与实时交互.md)、[原生内核到 Web](专题/05-原生内核到Web与WASM.md) | 当前页面接口、渲染就绪、浏览器依赖；视图要求采用 D02 的明确契约 |
| <a id="d08"></a>D08 | [文件格式与兼容](专题/04-文件格式与兼容工程.md)、[保存恢复](方法/06-工程文件与保存恢复.md) | codec、语义往返、工程/导入/导出身份、真实写入确认和资源路径 |
| <a id="d09"></a>D09 | [前端交互](专题/02-前端交互与可访问性.md)、[渲染与性能](专题/03-渲染图形与性能.md) | 选择/焦点、输入法、坐标、字体、资源就绪、DPR、旧渲染结果和实测性能 |
| <a id="d10"></a>D10 | [Agent 工具协作](方法/05-Agent工具与开发协作.md)、[安全边界](专题/13-安全扩展与协作边界.md)、[长任务](专题/06-模型接入与长任务系统.md) | 正式命令、受限路径、单写者、任务取消、模型归属及恢复 |
| <a id="d11"></a>D11 | [测试与交付方法](方法/07-测试验收与构建交付.md)、[测试诊断](专题/07-自动化测试与故障诊断.md)、[构建维护](专题/08-构建发布与升级维护.md) | 新增/关联测试、独立断言、真实链、版本/制品、目标环境和交接 |
| <a id="d12"></a>D12 | [软件方案模板](模板/单款软件Craft方案模板.md)、[新项目文档骨架](模板/新项目文档骨架.md) | Study 收束、设计输入、状态和新会话接手 |

## Craft Anything 当前源码

以下入口均已核路径和标出的符号；S02复核源码身份/行段见[记录](../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/source-identities.json)；S03 已构建锁定 CLI，并实际读取 commands/capabilities 与所需 help。运行回执见 METHOD，未执行的工具仍不宣称可用。

表内活跃源码路径只用于定位；上游 HEAD 后来已有推进。判断锁定方法时读取 `git show 1f8508a104134c436cbba5ea2f675619ef47702f:<repoPath>` 或 [METHOD](VESTA研究/METHOD.md) 指定快照，不以当前工作区源码替代历史证据。原 CLI 二进制仍按既有摘要锁定。

| ID | 直接入口 | 执行时读哪里、用来做什么 |
|---|---|---|
| <a id="ca01"></a>CA01 | [AGENTS](/Volumes/T9/Development/Sources/Artcraft/craft-anything/AGENTS.md)、[Cargo.toml](/Volumes/T9/Development/Sources/Artcraft/craft-anything/Cargo.toml:6)、[Cargo.lock](/Volumes/T9/Development/Sources/Artcraft/craft-anything/Cargo.lock) | 上游只读规则、版本 0.4.0、Rust 最低 1.97 与依赖锁；所引 `../craftrules/AGENTS.md` 本机缺失，不能推测其内容 |
| <a id="ca02"></a>CA02 | [brief.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/ai/src/rebuild/brief.rs:7) | `RULES`、`steps`（21 行）：用户要求优先、不得静默删减、先摘要、unknown、Build/Check 循环、原生工作区实现边界 |
| <a id="ca03"></a>CA03 | [study.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/ai/src/rebuild/study.rs:13)、[clean_room.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/engine/src/clean_room.rs:135) | `Worker::study`、`study_from_outside`（157 行）、`picture_window`（174 行）、`allows`：两种模式、目标观察和能力边界；Clean room在Settings允许且直接选择程序时，Worker可启动一次采外观，并非绝对禁止运行 |
| <a id="ca04"></a>CA04 | [CLI main.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/apps/craft-anything-cli/src/main.rs:30)、[rebuild_cmd.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/apps/craft-anything-cli/src/rebuild_cmd.rs:18) | `Cli`、`Command`、`RebuildArgs`：调查 `--project` 与 rebuild 项目参数的区别、模式、恢复和后端；CLI `--rounds` 默认 2 |
| <a id="ca05"></a>CA05 | [工具清单](/Volumes/T9/Development/Sources/Artcraft/craft-anything/docs/tools.md)、[session_tools.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/engine/src/tools/session_tools.rs:18) | `core_target_open`、证据导出、`core_unknowns_show/add/edit` 的真实注册和 schema；实际调用先查锁定运行件 help/schema |
| <a id="ca06"></a>CA06 | [brief.rs 的反编译规则](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/ai/src/rebuild/brief.rs:161)、[mechanics.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/ai/src/rebuild/kinds/mechanics.rs:12) | `DECOMPILE_FIRST`、`decompile_note`、`MECHANICS_CONTRACT`、`MechanicsCheck::run`；常量/公式来源记录及该检查的证明范围；字面量按绝对值比较，不核公式/来源真实性，缺source只写note |
| <a id="ca07"></a>CA07 | [app.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/ai/src/rebuild/kinds/app.rs:36)、[mod.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/ai/src/rebuild/mod.rs:155) | `default_stack/goal/checks`、`RunOptions::default`；默认桌面/核心功能与用户全功能 Web 要求的区别；Deep控制台默认Rust console；内部max_rounds=6、CLI默认2，均指首次构建后的修正轮数 |
| <a id="ca08"></a>CA08 | [rebuild tools](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/ai/src/rebuild/tools.rs:105)、[self_check.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/ai/src/rebuild/self_check.rs:47)、[workspace.rs](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/engine/src/workspace.rs:226) | `rebuild_check`、`CheckParams`（167 行）、`rebuild_check` 实现（458 行）、`open_questions/report`、`CheckResult::good`（308 行）；需要运行中的 build worker/inbox，不能独立离线调用；同进程 API builder 的自查拒绝按原协议处理 |
| <a id="ca09"></a>CA09 | [mod.rs 迭代与 Ready](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/ai/src/rebuild/mod.rs:919)、[workspace.rs 工作区](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/engine/src/workspace.rs:632) | 迭代停止、`Ready`（968 行）及 `Workspace::create/capture_dir/checks_dir`；状态不能替代全功能验收 |
| <a id="ca10"></a>CA10 | [MCP 实现](/Volumes/T9/Development/Sources/Artcraft/craft-anything/crates/mcp/src/lib.rs:355)、[Web 入口](/Volumes/T9/Development/Sources/Artcraft/craft-anything/apps/craft-anything-web/src/main.rs:1) | `restricted_refusal/call_engine` 的实际限制；Web 入口自述的轻量查看器范围，不把它当 VESTA 编辑内核 |

## VESTA 原件与官方资料

| ID | 直接入口 | 执行时读哪里、用来做什么 |
|---|---|---|
| <a id="v01"></a>V01 | [本地目录](/Users/agilecho/Documents/VESTA)、[LICENSE](/Users/agilecho/Documents/VESTA/LICENSE)、[Info.plist](/Users/agilecho/Documents/VESTA/VESTA.app/Contents/Info.plist) | 目标身份、许可和包版本字段；运行与内部读取按已记录研究模式进行 |
| <a id="v02"></a>V02 | [官方产品介绍](https://jp-minerals.org/vesta/en/)、[下载与许可](https://jp-minerals.org/vesta/en/download.html)、[版本历史](https://jp-minerals.org/vesta/en/changes.html) | 版本差异与查漏；本次下载页正文列开发版3.90.9a（macOS Aug8 2026）与稳定版3.5.8，changes仅到3.5.8，缺3.90完整变更记录；官网不替代本地身份 |
| <a id="v03"></a>V03 | [官方手册目录](https://jp-minerals.org/vesta/en/doc/VESTA.html)、[参考文献](https://jp-minerals.org/vesta/en/doc/VESTAli3.html#x51-272000A) | 全部章节与专业文献入口；2022-08-10目录已按章节/小节核对，书目135条仅条目信息，不代表原文已核；拆出的脚注页另补读。完整阅读范围见SOURCE_NOTES |
| <a id="vm02"></a>VM02 | [第 2 章：程序总览](https://jp-minerals.org/vesta/en/doc/VESTAch2.html#x7-70002) | §2.1–2.7（含2.2.1/2.7.1–.2）结构、体数据、晶形、外部程序、输入输出和程序概念；建立全功能分类 |
| <a id="vm03"></a>VM03 | [第 3 章：启动与打开](https://jp-minerals.org/vesta/en/doc/VESTAch3.html#x10-180003) | §3.3 macOS、§3.5 故障、§3.6 打开方式；目标运行与打开行为 |
| <a id="vm04"></a>VM04 | [第 4 章：主窗口](https://jp-minerals.org/vesta/en/doc/VESTAch4.html#x12-270004) | §4.1–4.5 窗口、菜单、横/纵工具栏、文本区；逐入口盘点 |
| <a id="vm05"></a>VM05 | [第 5 章：显示样式](https://jp-minerals.org/vesta/en/doc/VESTAch5.html#x17-460005) | §5.1–5.3 结构、体数据和晶形样式 |
| <a id="vm06"></a>VM06 | [第 6 章：相数据](https://jp-minerals.org/vesta/en/doc/VESTAch6.html#x18-640006) | §6.1–6.5 相、对称/晶胞、结构参数、体数据和晶形；数据模型与算法依据 |
| <a id="vm07"></a>VM07 | [第 7 章：多数据叠加](https://jp-minerals.org/vesta/en/doc/VESTAch7.html#x19-970007) | §7.1–7.3 多相定位、相对取向和样例 |
| <a id="vm08"></a>VM08 | [第 8 章：键与多面体](https://jp-minerals.org/vesta/en/doc/VESTAch8.html#x20-1010008) | §8.1–8.2 搜索规则和操作；周期成键与配位行为 |
| <a id="vm09"></a>VM09 | [第 9 章：附加对象](https://jp-minerals.org/vesta/en/doc/VESTAch9.html#x22-1130009) | §9.1–9.2 原子矢量与晶格平面 |
| <a id="vm10"></a>VM10 | [第 10 章：边界与视向](https://jp-minerals.org/vesta/en/doc/VESTAch10.html#x23-12100010) | §10.1–10.2 绘制边界和观察方向 |
| <a id="vm11"></a>VM11 | [第 11 章：交互](https://jp-minerals.org/vesta/en/doc/VESTAch11.html#x24-13000011) | §11.1–11.8 变换、选择、距离及各类角度 |
| <a id="vm12"></a>VM12 | [第 12 章：对象属性](https://jp-minerals.org/vesta/en/doc/VESTAch12.html#x25-15600012) | §12.1–12.2 属性对话框和对象侧栏 |
| <a id="vm13"></a>VM13 | [第 13 章：整体外观](https://jp-minerals.org/vesta/en/doc/VESTAch13.html#x26-19300013) | §13.1–13.4 背景、光照、投影、深度效果 |
| <a id="vm14"></a>VM14 | [第 14 章：计算工具](https://jp-minerals.org/vesta/en/doc/VESTAch14.html#x27-19800014) | §14.1–14.15 按功能逐节研究；须读公式、单位和引用来源，不仅看章节名 |
| <a id="vm15"></a>VM15 | [第 15 章：二维数据](https://jp-minerals.org/vesta/en/doc/VESTAch15.html#x34-21400015) | §15.1–15.7 窗口、菜单、交互、图像属性及二维导出 |
| <a id="vm16"></a>VM16 | [第 16 章：偏好](https://jp-minerals.org/vesta/en/doc/VESTAch16.html#x35-22800016) | §16.1–16.8 外部程序、字体、打开方式、动画、自动成键、图像输出、等值面和历史 |
| <a id="vm17"></a>VM17 | [第 17 章：CLI](https://jp-minerals.org/vesta/en/doc/VESTAch17.html#x36-23700017) | 命令清单、`-nogui` 与 GUI 差异；完成/退出、错误、路径和产物由锁定本地版本实测 |
| <a id="vm18"></a>VM18 | [第 18 章：输入输出](https://jp-minerals.org/vesta/en/doc/VESTAch18.html#x37-24200018) | §18.1–18.5.4：使用文件、用户设置、体数据和二进制格式；18.5.1输入、18.5.2数据/几何输出、18.5.3图像、18.5.4文本；原文fType/gType等疑误留SOURCE_NOTES；链出入口不当已读规范 |
| <a id="vma"></a>VMA | [附录 A：快捷键](https://jp-minerals.org/vesta/en/doc/VESTAap1.html#x50-271000A) | 键盘功能与实际焦点、上下文行为对照 |

## S02 新增的一手来源入口

| ID | 直接入口 | 读取状态与用途 |
|---|---|---|
| <a id="ext-cif1"></a>EXT-CIF1 | [IUCr CIF 1.1](https://www.iucr.org/what-we-do/digital-standards/cif/cif1) | 2026-11-08：curl403，web读1.1入口（2002批准/2003发布）及选定语法/语义段落；后续语法/字典及目标读取子集未核 |
| <a id="ext-poscar"></a>EXT-POSCAR | [VASP POSCAR](https://vasp.at/wiki/POSCAR) | 2026-11-08：重读完整格式和精度小节，oldid37632（2026-07-22）；版本/适用范围见SOURCE_NOTES，目标子集未验证 |
| <a id="ext-chgcar"></a>EXT-CHGCAR | [VASP CHGCAR](https://vasp.at/wiki/CHGCAR) | 2026-11-08：读Format/Magnetic calculations及正文，oldid37892（2026-09-14，晚于目标构建）；归一化文字/公式歧义留S05，未核目标兼容 |
| <a id="ext-xsf"></a>EXT-XSF | [XSF 官方格式](http://www.xcrysden.org/doc/XSF.html) | 旧502/超时/TLS失败保留；续作官方HTTP两次200且摘要一致，全文/示例已读（页面2025-08-04、无数字格式版号）；目标子集未核，见SOURCE_SUPPLEMENT |
| <a id="ext-ffast"></a>EXT-FFAST | [NIST FFAST](https://www.nist.gov/pml/x-ray-form-factor-attenuation-and-scattering-tables) | 2026-11-08：读SRD66入口说明；数据内容日期2005与网页日期分开；history当前2.2/May2026，未核完整表值或本地采用版本 |
| <a id="ext-examples"></a>EXT-EXAMPLES | [官方样本 ZIP](https://jp-minerals.org/vesta/archives/VESTA-Examples.zip)、[本轮清单](../output/vesta-study/20261108-113622/s05/official-examples-inventory.json) | S05 已下载及安全解包，142 成员；样本身份、独立预期与未验证分支见 SAMPLES |

## S02复核补充来源登记

本节访问/发现日期为2026-11-08。“发现”只证明链接来自已读手册，不表示目的页已访问。[获取记录](../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/primary-acquisition.json)、[补充获取](../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/supplemental-acquisition.json)、[web选读笔记](../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/web-reader-notes.json)分别保留状态，准确版本/正文范围见[SOURCE_NOTES](VESTA研究/SOURCE_NOTES.md)。

| ID | 直接入口 | 读取状态与用途 |
|---|---|---|
| <a id="ext-cif1-syntax"></a>EXT-CIF1-SYNTAX | [CIF1.1语法](https://www.iucr.org/what-we-do/digital-standards/cif/cif1/file-syntax) | web前轮¶3–24，续作补¶25–64/Appendix A；curl403保留，字典/扩展及目标符合性未核。 |
| <a id="ext-cif1-semantics"></a>EXT-CIF1-SEMANTICS | [CIF1.1语义](https://www.iucr.org/what-we-do/digital-standards/cif/cif1/common-semantic-features) | web前轮¶1–25，续作补¶26–38；语义/折叠/标记已读，字典全文及目标实现未核。 |
| <a id="ext-ffast-history"></a>EXT-FFAST-HISTORY | [NIST版本历史](https://www.nist.gov/physical-measurement-laboratory/x-ray-form-factor-attenuation-and-scattering-tables-version-history) | 正文已读；2.2/May2026、2.1/Jan2005、2.0/May2003；不能推出目标数据版本。 |
| <a id="ext-dysnomia"></a>EXT-DYSNOMIA | [Dysnomia](http://jp-minerals.org/dysnomia/en/) | 续作读程序页全文/日志/许可，1.0构建2018-12-08；手册封面2014-12-02，版本与目标集成分开。 |

VM18的45个异站外链逐条登记如下；完整小节、原链接上下文、读/未读状态及版本缺口在[外链清单](../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/vm18-external-source-inventory.json)。其中Wikipedia及手册STL误链的OBJ只是二手入口；没有被扩写成已核规范。Open Babel链接明确是2.3.1路径，ABINIT Cut3D链接是v6.4路径，X-Plor版本线索在书目[123]（3.1/1993），均不代表锁定原件使用这些版本。

| ID | 手册链出的入口 | 状态/用途 |
|---|---|---|
| <a id="ext-vm18-001"></a>EXT-VM18-001 | [database.iem.ac.ru/mincryst/](http://database.iem.ac.ru/mincryst/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-002"></a>EXT-VM18-002 | [elk.sourceforge.net/](http://elk.sourceforge.net/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-003"></a>EXT-VM18-003 | [en.wikipedia.org/wiki/Chemical_table_file](http://en.wikipedia.org/wiki/Chemical_table_file) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-004"></a>EXT-VM18-004 | [en.wikipedia.org/wiki/X-PLOR](http://en.wikipedia.org/wiki/X-PLOR) | VM18 18.5.1,18.5.2；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-005"></a>EXT-VM18-005 | [en.wikipedia.org/wiki/XYZ_file_format](http://en.wikipedia.org/wiki/XYZ_file_format) | VM18 18.5.1,18.5.2；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-006"></a>EXT-VM18-006 | [fujioizumi.verse.jp/download/download_Eng.html](http://fujioizumi.verse.jp/download/download_Eng.html) | 旧域名301至无关站后超时；替代权威恢复页/文档包见EXT-RIETAN-RESTORED。 |
| <a id="ext-vm18-007"></a>EXT-VM18-007 | [fujioizumi.verse.jp/visualization/VENUS.html](http://fujioizumi.verse.jp/visualization/VENUS.html) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-008"></a>EXT-VM18-008 | [han.ess.sunysb.edu/~USPEX/](http://han.ess.sunysb.edu/~USPEX/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-009"></a>EXT-VM18-009 | [kats-labo.jimdo.com/mxdorto-mxdtricl/](http://kats-labo.jimdo.com/mxdorto-mxdtricl/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-010"></a>EXT-VM18-010 | [openbabel.org/docs/2.3.1/FileFormats/3D_viewer_Formats.html](http://openbabel.org/docs/2.3.1/FileFormats/3D_viewer_Formats.html) | 旧2.3.1入口未补读；当前CC1页只有标题，无字段规范，见EXT-OPENBABEL-CC1。 |
| <a id="ext-vm18-011"></a>EXT-VM18-011 | [openbabel.org/docs/2.3.1/FileFormats/XYZ_cartesian_coordinates_format.html](http://openbabel.org/docs/2.3.1/FileFormats/XYZ_cartesian_coordinates_format.html) | 旧2.3.1与当前版本分开；续作当前XYZ格式正文已读，见EXT-OPENBABEL-XYZ。 |
| <a id="ext-vm18-012"></a>EXT-VM18-012 | [rruff.geo.arizona.edu/AMS/amcsd.php](http://rruff.geo.arizona.edu/AMS/amcsd.php) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-013"></a>EXT-VM18-013 | [superflip.fzu.cz/](http://superflip.fzu.cz/) | VM18 18.5.1,18.5.2；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-014"></a>EXT-VM18-014 | [www.abinit.org/](http://www.abinit.org/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-015"></a>EXT-VM18-015 | [www.abinit.org/documentation/helpfiles/for-v6.4/users/cut3d_help.html](http://www.abinit.org/documentation/helpfiles/for-v6.4/users/cut3d_help.html) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-016"></a>EXT-VM18-016 | [www.castep.org/](http://www.castep.org/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-017"></a>EXT-VM18-017 | [www.ccdc.cam.ac.uk/Solutions/CSDSystem/Pages/CSD.aspx](http://www.ccdc.cam.ac.uk/Solutions/CSDSystem/Pages/CSD.aspx) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-018"></a>EXT-VM18-018 | [www.chem.gla.ac.uk/~louis/software/wingx/](http://www.chem.gla.ac.uk/~louis/software/wingx/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-019"></a>EXT-VM18-019 | [www.chem.ous.ac.jp/~gsakane/](http://www.chem.ous.ac.jp/~gsakane/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-020"></a>EXT-VM18-020 | [www.crystalmaker.com](http://www.crystalmaker.com) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-021"></a>EXT-VM18-021 | [www.dvxa.org/](http://www.dvxa.org/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-022"></a>EXT-VM18-022 | [www.feffproject.org/](http://www.feffproject.org/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-023"></a>EXT-VM18-023 | [www.gaussian.com/](http://www.gaussian.com/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-024"></a>EXT-VM18-024 | [www.iucr.org/resources/cif/](http://www.iucr.org/resources/cif/) | VM18 18.5.1,18.5.2；关联上表来源；原入口自身未核，详见清单。 |
| <a id="ext-vm18-025"></a>EXT-VM18-025 | [www.maciejharanczyk.info/Zeopp/input.html](http://www.maciejharanczyk.info/Zeopp/input.html) | 续作找到Zeo++当前站并读正文；属使用者约定，非CSSR原始规范，见EXT-ZEOPP-CSSR。 |
| <a id="ext-vm18-026"></a>EXT-VM18-026 | [www.materialsdesign.com/medea](http://www.materialsdesign.com/medea) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-027"></a>EXT-VM18-027 | [www.msg.ameslab.gov/GAMESS/GAMESS.html](http://www.msg.ameslab.gov/GAMESS/GAMESS.html) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-028"></a>EXT-VM18-028 | [www.ncnr.nist.gov/xtal/software/gsas.html](http://www.ncnr.nist.gov/xtal/software/gsas.html) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-029"></a>EXT-VM18-029 | [www.nims.go.jp/cmsc/staff/arai/](http://www.nims.go.jp/cmsc/staff/arai/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-030"></a>EXT-VM18-030 | [www.nims.go.jp/cmsc/staff/arai/asse/](http://www.nims.go.jp/cmsc/staff/arai/asse/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-031"></a>EXT-VM18-031 | [www.nims.go.jp/cmsc/staff/arai/wien/venus.html](http://www.nims.go.jp/cmsc/staff/arai/wien/venus.html) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-032"></a>EXT-VM18-032 | [www.quantum-espresso.org/](http://www.quantum-espresso.org/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-033"></a>EXT-VM18-033 | [www.vasp.at/](http://www.vasp.at/) | VM18 18.5.1,18.5.2；关联上表来源；原入口自身未核，详见清单。 |
| <a id="ext-vm18-034"></a>EXT-VM18-034 | [www.web3d.org/x3d/specifications/vrml/](http://www.web3d.org/x3d/specifications/vrml/) | 续作读VRML97 Annex A与指定节点，未读全标准；见EXT-VRML97。 |
| <a id="ext-vm18-035"></a>EXT-VM18-035 | [www.wien2k.at/](http://www.wien2k.at/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-036"></a>EXT-VM18-036 | [www.wwpdb.org/](http://www.wwpdb.org/) | 续作读wwPDB3.3的简介/晶胞/坐标/连接章节，未读全规范；见EXT-PDB33。 |
| <a id="ext-vm18-037"></a>EXT-VM18-037 | [www.wwpdb.org/docs.html](http://www.wwpdb.org/docs.html) | 续作读wwPDB3.3指定章节；见EXT-PDB33，不证明目标完整兼容。 |
| <a id="ext-vm18-038"></a>EXT-VM18-038 | [www.xcrysden.org/](http://www.xcrysden.org/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-039"></a>EXT-VM18-039 | [www.xcrysden.org/doc/XSF.html](http://www.xcrysden.org/doc/XSF.html) | 续作官方HTTP正文/示例已读，两次摘要一致；见EXT-XSF。 |
| <a id="ext-vm18-040"></a>EXT-VM18-040 | [www2.fiz-karlsruhe.de/icsd_home.html](http://www2.fiz-karlsruhe.de/icsd_home.html) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-041"></a>EXT-VM18-041 | [www3.u-toyama.ac.jp/kihara/cc/mld/readme.html](http://www3.u-toyama.ac.jp/kihara/cc/mld/readme.html) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-042"></a>EXT-VM18-042 | [aimsclub.fhi-berlin.mpg.de/](https://aimsclub.fhi-berlin.mpg.de/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-043"></a>EXT-VM18-043 | [brettbode.github.io/wxmacmolplt/](https://brettbode.github.io/wxmacmolplt/) | VM18 18.5.1；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-044"></a>EXT-VM18-044 | [en.wikipedia.org/wiki/Wavefront_.obj_file](https://en.wikipedia.org/wiki/Wavefront_.obj_file) | VM18 18.5.2；只发现入口，目的正文/版本待核。 |
| <a id="ext-vm18-045"></a>EXT-VM18-045 | [www.scd.stfc.ac.uk/Pages/DL_POLY.aspx](https://www.scd.stfc.ac.uk/Pages/DL_POLY.aspx) | 续作读DL_POLY5.3.0 CONFIG小节，目标版本未核；见EXT-DLPOLY。 |

脚注补证给28个脚注页稳定分配 `EXT-NOTE-04` 等ID（按官方页号，不补齐不存在的编号），映射及目的URL见[脚注来源登记](../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/footnote-source-inventory.json)。28页本身已读；其目的页不因脚注读取自动算已读，后续实际状态见SOURCE_SUPPLEMENT/阅读台账。

书目所有135条以 `EXT-BIB-001`–`EXT-BIB-135` 稳定登记在[条目清单](../output/vesta-study/20261108-113622/reviews/20261108-131056-s02/bibliography-entry-inventory.json)，每条有原书目锚点、手册所印刊卷年页/版次及访问时间。它们的状态均为“书目条目已读，原出版物未独立核/正文未读”，不把书目年份当本地程序版本。SOURCE_NOTES逐组关联了散射、价键、密度转换、ORFFE/TIDY/RIETAN/MADEL等用途；其余编号同样保持该限制。

## 平台实现与验证样例（原有入口）

| ID | 直接入口 | 执行时读哪里、用来做什么 |
|---|---|---|
| <a id="p01"></a>P01 | [接入指南](接入/01-IntegrateCraft接入与复用边界.md)、[平台接入方法](/Users/agilecho/code/IntegrateCraft/docs/任务卡/阶段3-原型完善/软件接入方法-以WordCraft为基线.md) | 复用职责与本款差异；旧文末自动派发和历史保存/切视图说明不覆盖 D01/D02 与当前用户指令 |
| <a id="p02"></a>P02 | [catalog](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/catalog.mjs)、[startup](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/startup.mjs)、[client.jsx](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/client.jsx) | 软件注册、就绪与工作台页面接线；修改客户端时同步生成 bundle |
| <a id="p03"></a>P03 | [service](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/service.mjs)、[PageChannel](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/channel.mjs:3)、[SoftwareCoordinator](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/coordinator.mjs:94) | 既有工具运输、页面往返和串行协调 |
| <a id="p04"></a>P04 | [SoftwareFiles](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/software-files.mjs:25)、[SpaceFiles](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/coordinator.mjs:6)、[native-paths](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/native-paths.mjs) | 保存身份、外改基线、配额和安全路径；不另复制裸文件守卫 |
| <a id="p05"></a>P05 | [Word 页面](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/word-page.mjs:2)、[Word 工具](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/word-tools.mjs:2)、[Word 协调器](/Users/agilecho/code/IntegrateCraft/dsh/plugins/software/word-coordinator.mjs:6) | 适配职责样例。当前 `changeMode('agent')` 会保存转 MCP；VESTA 三视图按 D02 只改布局，不能逐行照抄 |
| <a id="p06"></a>P06 | [Word 构建说明](/Users/agilecho/code/IntegrateCraft/craft/wordcraft/README.md)、[锁文件](/Users/agilecho/code/IntegrateCraft/craft/wordcraft/upstream.lock.json) | 锁版本/补丁/字体/制品的组织；旧“保存即下载”等说明以 D02 为准，不照搬工具链与预算 |
| <a id="p07"></a>P07 | [页面测试](/Users/agilecho/code/IntegrateCraft/tests/demo/stage3/card03/api-browser.mjs)、[原生往返](/Users/agilecho/code/IntegrateCraft/tests/demo/stage3/card03/native-runtime.mjs)、[保存测试](/Users/agilecho/code/IntegrateCraft/tests/demo/stage3/card03/save-status-browser.mjs) | 测试入口与内容断言样例；历史 provider、制品、账号、Key、预算和通过结论不复用为本轮证据 |

## 本轮环境核实

- 当前 `/Users/agilecho/code/Craft` 尚非 Git 仓库，只有资料目录。编写计划不初始化仓库，也不在其他项目提交。
- `craft-anything-cli` 未在 PATH 中找到；S03 已用 Rust/Cargo 1.99.1 构建锁定 0.4.0 运行件，真实 commands/help、Deep triage 和原生记录往返见 [METHOD](VESTA研究/METHOD.md)。
- 默认 `rustc` 为 1.95.0；`rustc +1.99.1 --version` 和 `cargo +1.99.1 --version` 均已实际成功。Study 工具准备步骤可显式使用已安装的 1.99.1，不将默认工具链当满足 Rust 1.97 下限。
- 已核资料路径与源码符号，不代表 VESTA 或 Craft Anything 的运行、反编译、模型、重写、平台或 x86 验收已通过。

## S02.3续作来源（2026-11-08）

本节是本轮更新的索引，不改写旧获取/书目清单。准确已读页码、章节、来源类型、未读边界和更正见[SOURCE_SUPPLEMENT](VESTA研究/SOURCE_SUPPLEMENT.md)与[25项台账](../output/vesta-study/20261108-113622/reviews/20261108-133154-s02-sources/source-reading-ledger.json)；HTTP200不等于规范已核，获取失败也不等于所有通道都不可读。

| ID | 直接来源 | 版本/用途/范围 |
|---|---|---|
| <a id="ext-pdb33"></a>EXT-PDB33 | [wwPDB3.3](https://www.wwpdb.org/documentation/file-format-content/format33/sect1.html) | 简介、sect8/9/10已读；固定列/晶胞/坐标/连接，非全手册或目标通过。 |
| <a id="ext-cube"></a>EXT-CUBE | [Gaussian cubegen](https://gaussian.com/cubegen/) | G16RevC.01/2023-08-03；Output File Formats/Reading Cube Files；03/09版本待核。 |
| <a id="ext-dlpoly"></a>EXT-DLPOLY | [DL_POLY输入手册](https://ccp5.gitlab.io/dl-poly/UserManual/data_files/input_files.html) | 5.3.0，仅CONFIG/进一步说明；levcfg/imcon/单位与可选字段。 |
| <a id="ext-openbabel-xyz"></a>EXT-OPENBABEL-XYZ | [当前XYZ](https://openbabel.org/docs/FileFormats/XYZ_cartesian_coordinates_format.html) | openbabel-3-2-0格式正文；无已发布正式XYZ规范。 |
| <a id="ext-openbabel-cc1"></a>EXT-OPENBABEL-CC1 | [当前Chem3D](https://openbabel.org/docs/FileFormats/Chem3D_Cartesian_1_format.html) | 页面只有标题/导航，格式字段仍未知。 |
| <a id="ext-zeopp-cssr"></a>EXT-ZEOPP-CSSR | [Zeo++输入说明](http://www.zeoplusplus.org/input.html#cssr) | 正文/示例已读，无数字版号；使用者程序约定，不能当原始CSSR规范。 |
| <a id="ext-vrml97"></a>EXT-VRML97 | [VRML97](https://www.web3d.org/documents/specifications/14772/V2.0/index.html) | ISO/IEC14772-1:1997；AnnexA及指定节点，非全标准。 |
| <a id="ext-rietan-restored"></a>EXT-RIETAN-RESTORED | [JP-Minerals恢复页](https://jp-minerals.org/rietan/) | 页全文；维护说明2023-03-18，RIETAN-FP3.12，docs2021-12-20。 |
| <a id="ext-rietan-documents"></a>EXT-RIETAN-DOCUMENTS | [恢复文档包](https://jp-minerals.org/rietan/documents.zip) | 20成员仅选读；完整ZIP留临时目录，永久保留身份/成员元数据。 |
| <a id="ext-rietan-manual"></a>EXT-RIETAN-MANUAL | 同包RIETAN-FP_manual.pdf | 封面2021-12-16/234页；指定Ch17和AppendixD，不宣称全篇。 |
| <a id="ext-rietan-coop"></a>EXT-RIETAN-COOP | 同包RIETAN_VESTA_cooperation.pdf | 封面2021-12-16/38页；PDF1–7/20，TIDY/模板/几何联动，部分小节未完。 |
| <a id="ext-madel-manual"></a>EXT-MADEL-MANUAL | 同包MADEL.pdf | 8页全文；1991英文修订+2021附录/2.1；本地二进制版本仍未知。 |
| <a id="ext-bib-045"></a>EXT-BIB-045 | [ORFFE原报告](https://www.iucr.org/__data/iucr/software_manuals/ORFFE_ORNL-TM306.pdf) | ORNL-TM-306/1964，web读指定页；原版与RIETAN修改版整数契约不同。 |
| <a id="ext-bib-046"></a>EXT-BIB-046 | [TIDY出版页](https://journals.iucr.org/paper?buy=yes&cnor=a27733) | 1987/20:139–143，DOI及摘要核；付费正文未读。 |
| <a id="ext-bib-094"></a>EXT-BIB-094 | [Waasmaier/Kirfel出版页](https://journals.iucr.org/paper?buy=yes&cnor=sh0059) | 1995/A51:416–431，DOI及摘要核；正文未读。 |
| <a id="ext-bib-048"></a>EXT-BIB-048 | [Tsirelson DOI](https://doi.org/10.1107/S0108768102005517) | 2002/B58:632–639，题名/DOI/页码核；手册起页正确，正文未读。 |
| <a id="ext-dysnomia-manual"></a>EXT-DYSNOMIA-MANUAL | [Dysnomia手册](https://jp-minerals.org/dysnomia/archives/Dysnomia_Manual.pdf) | 2014-12-02，封面/许可及Ch4–5；比1.0程序日志旧。 |
| <a id="ext-vesta-pdf2019"></a>EXT-VESTA-PDF2019 | [VESTA下载PDF](https://jp-minerals.org/vesta/archives/VESTA_Manual.pdf) | 封面2019-01-27/177页，仅封面与定位；不同于2022HTML。 |
| <a id="ext-bv2020"></a>EXT-BV2020 | 本地Resources/bvparm2020.cif | 数据块2020-11-25/cif_core.dic2.2；前轮读头部/标签/更新历史，本轮补参数loop/Fe-O及1934行库存；参数选择逻辑仍待核。 |
| <a id="ext-mxdorto"></a>EXT-MXDORTO | [原官方入口](http://kats-labo.jimdo.com/mxdorto-mxdtricl/) | 原入口301后TLS失败保留；R133恢复[NIMS官方页](https://www.nims.go.jp/personal/mxdorto-mxdtricl/downloads.html)、手册1.0.1和两份serial生产者源码选读，见SOURCE_NOTES R133；不表示目标兼容。 |

## S02.3格式与科学来源续作（2026-11-08）

最新版本/章节边界见[SOURCE_FORMATS_SCIENCE](VESTA研究/SOURCE_FORMATS_SCIENCE.md)及[15项续作台账](../output/vesta-study/20261108-113622/reviews/20261108-135713-s02-formats-science/source-reading-ledger.json)；[47输入/23数据输出来源图](VESTA研究/FORMAT_SOURCE_COVERAGE.md)逐项列覆盖，不能当运行通过。旧VM18-003/004二手入口未作为规范，010空白文档不因取得实现代码而变成已读规范；新来源分别登记。

| ID | 直接来源 | 阅读/用途边界 |
|---|---|---|
| <a id="ext-ctfile2020"></a>EXT-CTFILE2020 | [BIOVIA原始规范](https://discover.3ds.com/sites/default/files/2020-08/biovia_ctfileformats_2020.pdf) | BIOVIA2020/版权2019；V2000/V3000/SDF指定页，非105页全文。 |
| <a id="ext-shelx"></a>EXT-SHELX | [官方索引](https://shelx.uni-goettingen.de/wikis.php) | 导航/索引；不是输入契约或目标版本证明。 |
| <a id="ext-shelxl-guide"></a>EXT-SHELXL-GUIDE | [当前指南](https://shelx.uni-goettingen.de/shelxl_user_guide.pdf) | 49页；只读Introduction/§1.1–1.9，未核准确发行版。 |
| <a id="ext-shelxl-commands"></a>EXT-SHELXL-COMMANDS | [当前指令](https://shelx.uni-goettingen.de/shelxl_html.php) | General/原子及指定指令；39页PDF只核身份。 |
| <a id="ext-shelx97"></a>EXT-SHELX97 | [97版手册](https://shelx.uni-goettingen.de/shelx97.pdf) | 明确97版/154页；§2.2–2.3、7.1和7.3选段，非全手册。 |
| <a id="ext-xplor-nih"></a>EXT-XPLOR-NIH | [NIH入口](https://nmr.cit.nih.gov/xplor-nih/) | 当前3.12及文档导航；不等于1993年3.1书版。 |
| <a id="ext-xplor-map"></a>EXT-XPLOR-MAP | [地图页](https://nmr.cit.nih.gov/xplor-nih/doc/current/xplor/node333.html) | 地图页/上级正文已读，页脚2026-07-10；CNX与历史版未核。 |
| <a id="ext-stl-burns"></a>EXT-STL-BURNS | [作者书节](https://www.fabbers.com/tech/STL_Format) | 全文及4张图表已读；非原3D Systems规范。 |
| <a id="ext-stl-loc"></a>EXT-STL-LOC | [LOC来源追溯](https://www.loc.gov/preservation/digital/formats/fdd/fdd000504.shtml) | 2025-02-25/Preliminary；策展说明选读，原规范仍缺。 |
| <a id="ext-chem3d-ob231"></a>EXT-CHEM3D-OB231 | [固定提交源码](https://github.com/openbabel/openbabel/blob/ef02552aa0d9a6d87561aad806c315fcdc3b925a/src/formats/chem3dformat.cpp) | 2.3.1标签解引用到固定提交；全文件读，未执行/复用；VESTAcc1映射待核。 |
| <a id="ext-cns"></a>EXT-CNS | [官方入口](https://www.cns-online.org/v1.3/) | 403/无可读正文；格式/版本正文未核。 |
| <a id="ext-bib-032"></a>EXT-BIB-032 | [Baur DOI](https://doi.org/10.1107/S0567740874004560) | 1974/B30:1195–1215；作者上传原论文web选读，curl403/截图超时分开。 |
| <a id="ext-bib-033"></a>EXT-BIB-033 | [Robinson等DOI](https://doi.org/10.1126/science.172.3983.567) | 1971/Science172:567–570；Crossref身份核，正文403。 |
| <a id="ext-bib-034"></a>EXT-BIB-034 | [Hoppe出版页](https://www.degruyterbrill.com/document/doi/10.1524/zkri.1979.150.14.23/html) | 1979/150:23–52；出版身份/印刷与上线日期核，正文受限。 |
| <a id="ext-bib-038"></a>EXT-BIB-038 | [Brown/Altermatt出版页](https://journals.iucr.org/paper?buy=yes&cnor=a24419) | 1985/B41:244–247；出版身份核，正文未读/Crossref429保留。 |

生产者追加8项另见[阅读台账](../output/vesta-study/20261108-113622/reviews/20261108-135713-s02-formats-science/producer-reading-ledger.json)，与上表15项合计23项，均未登记目标格式契约整体通过。

| ID | 直接来源 | 阅读/用途边界 |
|---|---|---|
| <a id="ext-fhi-aims"></a>EXT-FHI-AIMS | [生产者资料](https://fhi-aims.org/uploads/manual/Ch2/index.html) | 261002; later than target build；指定范围见台账，非全文/目标通过。 |
| <a id="ext-elk"></a>EXT-ELK | [生产者资料](https://elk.sourceforge.io/elk.pdf) | 11.2.3 explicit on web PDF cover;203pages；指定范围见台账，非全文/目标通过。 |
| <a id="ext-wien2k"></a>EXT-WIEN2K | [生产者资料](http://www.wien2k.at/reg_user/textbooks/usersguide.pdf) | 23.2 Release07/24/2024;316pages；指定范围见台账，非全文/目标通过。 |
| <a id="ext-feff"></a>EXT-FEFF | [生产者资料](https://feff.phys.washington.edu/FEFF/Docs/feff9/feff90/feff90_users_guide.pdf) | feff9.6.4 updated2013-02-02;150pages；指定范围见台账，非全文/目标通过。 |
| <a id="ext-gsas"></a>EXT-GSAS | [生产者资料](https://subversion.xray.aps.anl.gov/EXPGUI/gsas/all/GSAS%20Manual.pdf) | LAUR86-748;copyright1985-2004;page headers9/26/04;231pages；指定范围见台账，非全文/目标通过。 |
| <a id="ext-castep"></a>EXT-CASTEP | [生产者资料](https://castep-docs.github.io/castep-docs/documentation/Input_Files/cell_file/) | Project-authored current docs; numeric release not established；指定范围见台账，非全文/目标通过。 |
| <a id="ext-gamess"></a>EXT-GAMESS | [生产者资料](https://www.msg.chem.iastate.edu/GAMESS/GAMESS_Manual/input.pdf) | Input Description1July2016;523pages；指定范围见台账，非全文/目标通过。 |
| <a id="ext-macmolplt"></a>EXT-MACMOLPLT | [生产者资料](https://brettbode.github.io/wxmacmolplt/Manual_pages/MacMolPlt_Surfaces.html) | Home7.7.3 January2024;manual edition unmarked；指定范围见台账，非全文/目标通过。 |

## S02.3体数据与科学来源续作（2026-11-08）

最新实际范围见[SOURCE_VOLUME_SCIENCE](VESTA研究/SOURCE_VOLUME_SCIENCE.md)及[本轮台账](../output/vesta-study/20261108-113622/reviews/20261108-143347-s02-volume-science/source-reading-ledger.json)。共9条续记，只有6条新增正文范围，另2条定位/摘要和1条版本历史重复参照；不把全部条目计为新全文。既有EXT-WIEN2K、EXT-GSAS、EXT-BIB-048/033、EXT-FFAST继续沿用ID；历史章节范围不回写成完整规范。

| ID | 直接来源 | 阅读/用途边界 |
|---|---|---|
| <a id="ext-castep-guide2024"></a>EXT-CASTEP-GUIDE2024 | [直接入口](https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/pdfs/castep.pdf) | Materials Studio2024/248页；Electron density两段/66页视觉；非全手册/目标通过。 |
| <a id="ext-elk-writegeom"></a>EXT-ELK-WRITEGEOM | [直接入口](https://elk.sourceforge.io/doxygen/html/writegeom_8f90_source.html) | 官方Doxygen源页web选读；Elk发行版未建立，curl403；未执行/复用。 |
| <a id="ext-bib-095"></a>EXT-BIB-095 | [直接入口](https://srd.nist.gov/jpcrdreprint/1.555974.pdf) | 1995/24:71–643，DOI10.1063/1.555974；§1–2/10–11/16指定范围，574页PDF非全文阅读。 |
| <a id="ext-bib-096"></a>EXT-BIB-096 | [直接入口](https://www.nist.gov/srd/nist-standard-reference-database-journal-physical-and-chemical-reference-data-reprints) | 2000/29，DOI10.1063/1.1321055；NIST与作者书目页码差异保留，正文未读。 |

## S02.3字典与历史格式续作（2026-11-08）

最新范围见[SOURCE_DICTIONARIES_LEGACY](VESTA研究/SOURCE_DICTIONARIES_LEGACY.md)及[本轮台账](../output/vesta-study/20261108-113622/reviews/20261108-145707-s02-dictionaries-legacy/source-reading-ledger.json)：8条记录中6处正文选读、2处入口/摘要，未读全字典或登记目标通过。CIF语法、DDL与字典版本分开，静态方法阅读不等于执行。

| ID | 直接来源 | 阅读/用途边界 |
|---|---|---|
| <a id="ext-cif-dict-gateway"></a>EXT-CIF-DICT-GATEWAY | [直接入口](https://www.iucr.org/what-we-do/digital-standards/cif/dictionaries) | IUCr current dictionary gateway；范围见续读，非完整规范/目标通过。 |
| <a id="ext-cif-core245"></a>EXT-CIF-CORE245 | [直接入口](https://cif-dictionaries.iucr.org/cifdic/dic/cif_core_2.4.5.dic) | 2.4.5;2014-11-21;CIF1/DDL1；范围见续读，非完整规范/目标通过。 |
| <a id="ext-cif-core330"></a>EXT-CIF-CORE330 | [直接入口](https://cif-dictionaries.iucr.org/cifdic/dic/cif_core_3.3.0.dic) | 3.3.0;dictionary date2026-09-22;announcement2026-10-02;CIF2/DDL4.2.0；范围见续读，非完整规范/目标通过。 |
| <a id="ext-cif-mag"></a>EXT-CIF-MAG | [直接入口](https://cif-dictionaries.iucr.org/cifdic/dic/cif_mag.dic) | 0.9.8;2020-07-28;CIF2/DDL3.13.1；范围见续读，非完整规范/目标通过。 |
| <a id="ext-cif-ms"></a>EXT-CIF-MS | [直接入口](https://cif-dictionaries.iucr.org/cifdic/dic/cif_ms_1.0.1.dic) | 1.0.1;2005-06-16;DDL1;header based on core2.3(2003)；范围见续读，非完整规范/目标通过。 |
| <a id="ext-uspex-manual"></a>EXT-USPEX-MANUAL | [直接入口](https://uspex-team.org/online_utilities/uspex_manual_release/EnglishVersion/uspex_manual_english.pdf) | 9.4.4 HTML;10.5/2021-07-05/133pages;2023cover2023.0/2023-05-23 vs headerRelease2023.0.1/78pages；范围见续读，非完整规范/目标通过。 |
| <a id="ext-cmtx-producer"></a>EXT-CMTX-PRODUCER | [直接入口](https://www.crystalmaker.com/crystalmaker/release-notes/mac/11/index.html) | 11.0.0/2024-01-02 selected CMTX change;current specs,FAQ guide edition unbound；范围见续读，非完整规范/目标通过。 |
| <a id="ext-amcsd-format"></a>EXT-AMCSD-FORMAT | [直接入口](https://www.rruff.net/help/zip-download/) | Current RRUFF help/index and historical AMCSD entry;exact format edition unestablished；范围见续读，非完整规范/目标通过。 |

## S02.3历史生产者与绑定续作（2026-11-08）

最新范围见[SOURCE_LEGACY_PRODUCERS](VESTA研究/SOURCE_LEGACY_PRODUCERS.md)及[本轮台账](../output/vesta-study/20261108-113622/reviews/20261108-152746-s02-legacy-producers/source-reading-ledger.json)。7条续记包含5处正文选读、1条失效入口及1条VM18上下文复核；消费方转录不当原规范，HTTP200验证码页不当TXT。rho的转换脚本三维数据与LAPW5二维同后缀数据已分开。

| ID | 直接来源 | 阅读/用途边界 |
|---|---|---|
| <a id="ext-fdat-rpluto"></a>EXT-FDAT-RPLUTO | [直接入口](https://downloads.ccdc.cam.ac.uk/rpluto/rpluto.pdf) | RPluto68pages,cover numeric release unestablished,PDFmetadata2006-06-27;ConQuestactual2022.2/229pages despite2020filename；指定范围见续读，原规范与目标支持不隐含通过。 |
| <a id="ext-cerius2-formats"></a>EXT-CERIUS2-FORMATS | [直接入口](https://www.chem.cmu.edu/courses/09-560/docs/msi/modenv/D_Files.html) | MSI vendor page dated1999-04-08;application release not established；指定范围见续读，原规范与目标支持不隐含通过。 |
| <a id="ext-icsd-consumers"></a>EXT-ICSD-CONSUMERS | [直接入口](https://www.crystalimpact.de/download/diamond5/docs/Manual_structure-file-formats.htm) | Diamond5 documentation;1986original manual cited but not obtained;ICSDhome announcesweb5.7.0/2026-10-30；指定范围见续读，原规范与目标支持不隐含通过。 |
| <a id="ext-wingx-map"></a>EXT-WINGX-MAP | [直接入口](https://www.chem.gla.ac.uk/~louis/software/wingx/) | WinGXchanges2023.1/2023-07-23,1.70.00/2005-01-03,1.64.02/2001-05-07;MCEintegration mentionsWinGX>=1.62beta；指定范围见续读，原规范与目标支持不隐含通过。 |
| <a id="ext-wien2venus"></a>EXT-WIEN2VENUS | [直接入口](https://zeus.theochem.tuwien.ac.at/pipermail/wien/2020-January/030087.html) | Original script not obtained;Blaha reply2020-01-02；指定范围见续读，原规范与目标支持不隐含通过。 |
| <a id="ext-venus-legacy"></a>EXT-VENUS-LEGACY | [直接入口](http://fujioizumi.verse.jp/visualization/VENUS.html) | Historical source gone;guessed current path404；指定范围见续读，原规范与目标支持不隐含通过。 |

## S02.3 MOLDA、C04D与XTL续作（2026-11-08）

实际范围见[SOURCE_REMAINING_FORMATS](VESTA研究/SOURCE_REMAINING_FORMATS.md)及[7条台账](../output/vesta-study/20261108-113622/reviews/20261108-155158-s02-remaining-formats/source-reading-ledger.json)。4处新增正文选读、2处失效入口及1处VM18复核分开；CP2K是消费者代码，C04D字段不能替代SCA全布局。

| ID | 直接来源 | 阅读/用途边界 |
|---|---|---|
| <a id="ext-molda-data"></a>EXT-MOLDA-DATA | [直接来源](https://katakago.sakura.ne.jp/cc/mld/readme.html) | Readme historical notes1989-05-22/11-06,current page revision unnumbered;Yoshida/Matsuura1997；Original software format literature not obtained;units/element codes/target reader unverified。 |
| <a id="ext-dvxa-contrd"></a>EXT-DVXA-CONTRD | [直接来源](https://dvxa.org/hme/HidemaruDV/HidemaruDV.pdf) | Sakane2026-02-08,179pages;misindexed otherPDF actuallyVESTA2014-08-06/175pages；C04D selected fields only;originalReadme_contrd,F01full,SCAheaders/order,programversion,target behavior unresolved。 |
| <a id="ext-gulp-xtl"></a>EXT-GULP-XTL | [直接来源](https://gulp.curtin.edu.au/help/help_51_txt.html) | Official index maps5.1 to help_51_txt;no precise patchversion binding；Not complete XTL grammar or MSI original standard;guessed directory404;no execution。 |
| <a id="ext-cp2k-xtl"></a>EXT-CP2K-XTL | [直接来源](https://raw.githubusercontent.com/cp2k/cp2k/953180a62b30a8f3dc9eff4e1ed2aa9e1ef21684/src/topology_xtl.F) | v2025.1 peeled953180a62b30a8f3dc9eff4e1ed2aa9e1ef21684；Consumer-specific syntax,units andmatrix behavior not MSI/VESTA contract;no reuse/compile/run。 |
| <a id="ext-asse"></a>EXT-ASSE | [直接来源](https://www.nims.go.jp/cmsc/staff/arai/asse/) | Original NIMS author page404；ASSE grammar/version unknown;not target unsupported conclusion。 |
| <a id="ext-arai-eb"></a>EXT-ARAI-EB | [直接来源](https://www.nims.go.jp/cmsc/staff/arai/) | Original NIMS author root404；eb field layout,unit,offset persistence unknown;not LAPW5rho contract。 |

## S02.3 梯度修正、参数与MEM续作（2026-11-08）

见[SOURCE_SCIENCE_PARAMETERS](VESTA研究/SOURCE_SCIENCE_PARAMETERS.md)与[台账](../output/vesta-study/20261108-113622/reviews/20261108-160825-s02-science-parameters/source-reading-ledger.json)。7条记录含4处公开正文、1处本地参数深读库存及2处受限正文定位，不是7篇新全文。EXT-BV2020补参数loop库存，EXT-BIB-038/094保留正文未得；不重复分配已有ID。

| ID | 直接来源 | 实际范围和边界 |
|---|---|---|
| <a id="ext-kirzhnits1957"></a>EXT-KIRZHNITS1957 | [直接来源](https://www.jetp.ras.ru/cgi-bin/dn/e_005_01_0064.pdf) | JETP5:64-71/Aug1957,originalZhETF32:115-123/Jan1957;appendedDec15erratum1338；准确选读见续读，Integratedgradientcorrection nottargetlocaldensity/discretization;consumeerratum beforecoefficients。 |
| <a id="ext-eris-manual"></a>EXT-ERIS-MANUAL | [直接来源](https://jp-minerals.org/eris/archives/ERIS_Manual.pdf) | Application1.0.1built2026-08-04;manualcover2026-07-28,postedlabel2026-07-27,37pages；准确选读见续读，prfconditioncompatibility notpri/denbody;pgridpseudocodetypos/typeambiguities retained;no execution。 |
| <a id="ext-mincryst-producer"></a>EXT-MINCRYST-PRODUCER | [直接来源](https://mincryst.iem.ac.ru/index.php) | Olddescription editionunfixed;currentEnglishhomeupdated2026-11-05;card1created1999-08-31/edited2017-05-25；准确选读见续读，No completeMINgrammar orrawBDM obtained;SQL/HTML/BDM distinctions retained;nowholeDBdownload/Java。 |
| <a id="ext-bv-protocol"></a>EXT-BV-PROTOCOL | [直接来源](https://www.iucr.org/what-we-do/digital-standards/data/datasets/bond-valence-parameters) | IUCrBrownsigneddescription;2020datasetposted2020-12-09；准确选读见续读，No successHTMLhash orremote2020bytes obtained;not1985paper replacement orVESTAparameterselectionproof。 |

## S04/S05原件读取机制证据

本地目标新观察见[READER_PATHS](VESTA研究/READER_PATHS.md)及[本轮回执](../output/vesta-study/20261108-113622/reviews/20261108-162528-s04-reader-paths/)。这是原件/原库/临时副本的分层取证，不是新增外部规范。来源问题仍revision8；密度问题诊断resolved、MXDORTO内容/建模仍investigating，不能把阅读和诊断计为完整功能通过。

## R10原件MXDORTO失败诊断

[读取机制](VESTA研究/READER_PATHS.md)与[本轮证据](../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/)为目标观察/诊断控制，不新增文献来源ID。原样本布局和目标10字符解析不一致已解释，官方格式/生产者版本覆盖仍分别跟踪。

## R11 CLI返回机制

[CLI_COMPLETION](VESTA研究/CLI_COMPLETION.md)记录原始代码/数据与受控返回轨迹，属于目标观察，不新增外部文献ID。

## R12 MADEL目标依赖观察

[MADEL_COMPLETION](VESTA研究/MADEL_COMPLETION.md)记录锁定原包外部程序/本机运行依赖与失败，不新增领域论文来源ID或科学有效性结论。

## R13输入接受机制

[INPUT_VALIDATION](VESTA研究/INPUT_VALIDATION.md)为原始代码/既有样本的目标观察，不新增外部文献或格式规范ID。

## R14截图工具源码与界限

[WINDOW_CAPTURE](VESTA研究/WINDOW_CAPTURE.md)使用原锁定工具提交快照与真实负控制，不新增外部资料ID，不把源码接口说明或失败图像当视觉通过。

## R15数据库网络策略来源

| ID | 来源 | 本次阅读与边界 |
|---|---|---|
| <a id="apple01"></a>APPLE01 | [Apple安全连接错误定义](https://developer.apple.com/documentation/foundation/nsurlerrorapptransportsecurityrequiressecureconnection-c.enum.case) | 实取并读短Markdown正文，解释ATS错误含义；-1022数值另由本机Apple SDK头文件核。不是VESTA业务规范。 |
| <a id="apple02"></a>APPLE02 | [Apple WebView ATS键](https://developer.apple.com/documentation/bundleresources/information-property-list/nsapptransportsecurity/nsallowsarbitraryloadsinwebcontent) | 实取并读短Markdown正文，说明WKWebView例外行为；没有设置该键或弱化安全。 |
| <a id="cod01"></a>COD01 | [COD当前搜索页](https://www.crystallography.net/cod/search.html) | 读当前表单和公共入口，HTTP/HTTPS各200且无重定向；原件HTTPS实显。未提交查询，不代表历史/全数据库API或导入已核。 |

[DATABASE](VESTA研究/DATABASE.md)及其获取元数据区分JS外壳/搜索片段/Markdown正文。旧基线引用不变，不新增baseline-rev4。


## R22 SGI格式主文

| ID | 来源 | 阅读范围与边界 |
|---|---|---|
| <a id="sgi01"></a>SGI01 | [The SGI Image File Format,Version1.00,PaulHaeberli/SGI（原站归档镜像）](https://ftp.zx.net.nz/pub/archive/ftp.sgi.com/graphics/grafica/sgiimage.html) | 已读Header、非RLE数据及origin段；固定512字节头、通道平面/左下原点。用于RGB实物结构对照，非VESTA实现版本证明；[本地获取记录](../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/sgi-spec-fetch.json)。PaulBourke镜像返回crawler限制，未绕过或重试该站。 |

## R30窗口归属正控制

[WINDOW_CAPTURE](VESTA研究/WINDOW_CAPTURE.md)归档本次受控原版同PID窗口元数据、原CLI显式窗口截图/解码/视觉核对，不新增外部资料ID、不当全部功能视觉通过。

## R34独立非共振散射数据

| 资料 ID | 一手维护来源 | 实际读取范围与边界 |
|---|---|---|
| <a id="ext-dabax-waas"></a>EXT-DABAX-WAAS | [OASYS/DABAX维护库f0_WaasKirf.dat](https://raw.githubusercontent.com/oasys-kit/DabaxFiles/main/f0_WaasKirf.dat)、[项目说明](https://github.com/oasys-kit/DabaxFiles) | HTTP200/42473字节，SHA8294958f03aad421c9e65e6fdd0cc943f6f7a13d0d09d6f3f74fd894b5ead1e7；仅中性C/O各11系数、列序/五高斯式/适用范围选读。ESRF目录可读、文件web刷新timeout，不冒称两源原字节已核相等；完整表只临时阅读目录，项目留两行数值/身份。不是Waasmaier/Kirfel1995论文全文，来源问题rev8不改。 |

## R35位移参数与占位公式来源

| 资料 ID | 一手来源 | 实际选读范围 |
|---|---|---|
| <a id="ext-iucr-adp-cif"></a>EXT-IUCR-ADP-CIF | [IUCr coreCIF B_iso_or_equiv定义](https://www.iucr.org/__data/iucr/cifdic_html/3/CORE_DIC/Iatom_site.B_iso_or_equiv.html) | B/U换算B=8π²U及Å²单位；不宣称全字典重新通读。 |
| <a id="ext-iucr-intensity"></a>EXT-IUCR-INTENSITY | [Dinnebier与Scardi(2023), DOI10.1107/S1600576723002121](https://journals.iucr.org/j/issues/2023/03/00/dv5004/index.html) | §3式9/14的各向同性衰减、B/U；式12/15各向异性仅资料选读未运行；§4式17占位乘入复数和。不是整篇内容验收。 |

## R37中子相干散射长度的两份NIST页面

| 资料 ID | 一手来源 | 读取范围/边界 |
|---|---|---|
| <a id="ext-nist-neutron"></a>EXT-NIST-NEUTRON | [NIST历史表](https://www.ncnr.nist.gov/resources/n-lengths/list.html)、[另一NIST表](https://www.ncnr.nist.gov/resources/activation/scattering_table.html) | 历史页自述取NeutronNews3(3)/1992/29–37，仅读表头/单位和自然H−3.7390、C6.6460、O5.803fm。另一链接重定向表H−3.7409(11)、C6.6472(9)、O5.8037(29)，日期未建立。web选段可读、urllib raw403失败分开；无原始全文SHA/无HTTP200下载声明。数值差异保留，不能凭三元素相同断言原包全表来源或版本。 |

### R75 粉末峰形一手资料补充

| 来源ID | 原文 | 已读边界 |
| --- | --- | --- |
| <a id="ext-powder-ida1998"></a>EXT-POWDER-IDA1998 | [Takashi Ida (1998), An efficient method for calculating asymmetric diffraction peak profiles, RSI69:3837–3839](https://www.crl.nitech.ac.jp/~ida/research/reprints/efficient_convolution.pdf) | All 3 PDF pages text inspected; pages1–2 rendered locally and visually inspected. Eq1/4 direct convolution and Gaussian/Lorentz definitions used; native code not used as integration oracle. |
| <a id="ext-powder-itc2019"></a>EXT-POWDER-ITC2019 | [R.B. Von Dreele, International Tables H (2019), chapter3.3 Powder diffraction peak profiles, pp263–269](https://onlinelibrary.wiley.com/iucr/itc/Ha/ch3o3v0001/) | Publisher HTML through section3.3.2.4 inspected; Gaussian/pseudo-Voigt/TCH polynomial, asymmetry discussion and peak-vs-Bragg distinction. Later TOF sections not claimed read. |
| <a id="ext-powder-tch1987"></a>EXT-POWDER-TCH1987 | [Thompson, Cox & Hastings (1987), Rietveld refinement of Debye–Scherrer synchrotron X-ray data from Al2O3, JAC20:79–83](https://onlinelibrary.wiley.com/doi/abs/10.1107/S0021889887087090) | Publisher abstract only; full paper unavailable and not claimed read. |


## R81 磁结构一手来源

| ID | 直接入口 | 已读范围与用途 |
|---|---|---|
| <a id="ext-magcif-098"></a>EXT-MAGCIF-098 | [IUCr Magnetic CIF Dictionary 0.9.8](https://www.iucr.org/__data/iucr/cif/dictionaries/cif_mag_0.9.8.dic.pdf) | 只读第2页磁矩坐标/单位/label和第12页磁操作xyz定义；末尾±1给时间反演。HTML超时、urllib403，web PDF正文可读；未通读字典。 |
| <a id="ext-mag-transform-2023"></a>EXT-MAG-TRANSFORM-2023 | [Shinohara/Togo/Tanaka 2023](https://journals.iucr.org/a/issues/2023/05/00/ib5114/index.html) | Acta Cryst A79 390–398，DOI10.1107/S2053273323005016；只读元数据及§2.4 Eq7，独立轴矢量/时间反演基准，不声称通读算法。 |


## S02 专项来源接续 R133（2026-11-10）

实际获取/章节/摘要和限制统一见[SOURCE_NOTES](VESTA研究/SOURCE_NOTES.md)R133及其8项台账；完整外文原文仅/tmp。当前来源问题revision9，S02.3仍未完成。

| 来源ID | 直接入口 | 固定身份与实际范围 |
|---|---|---|
| EXT-MXDORTO-MANUAL101 | [官方手册](https://www.nims.go.jp/personal/mxdorto-mxdtricl/doc/mxdorto_manual_JP_1-0-1.pdf) | v1.0.1，§3.2 PDF11–14；不是完整FILE06说明。 |
| EXT-MXDORTO-SOURCE20251120 | [官方源码](https://www.nims.go.jp/personal/mxdorto-mxdtricl/src/mxdorto.zip) | serial内标2025-11-20；writer/FORMAT指定行。 |
| EXT-MXDTRICL-SOURCE20251119 | [官方源码](https://www.nims.go.jp/personal/mxdorto-mxdtricl/src/mxdtricl.zip) | serial内标2025-11-19；writer/FORMAT指定行；MPI未读。 |
| EXT-BIB-050 / EXT-BIB-030 | [Arnold](https://onlinelibrary.wiley.com/iucr/itc/Ab/ch5o1v0001/sec5o1o3/) / [原书镜像](https://ncm.unn.ru/files/2021/01/International-Tables-for-crystallography.pdf) | 2005重印前置页/2006在线章节，§5.1印78–79/85选段。 |
| EXT-BIB-010 | [Sears](https://onlinelibrary.wiley.com/iucr/itc/Cb/ch4o4v0001/sec4o4o4/) / [编表说明](https://onlinelibrary.wiley.com/iucr/itc/Cb/ch4o4v0001/sec4o4o4o6/) | ITC C在线2006，指定引言/表头/例/§4.4.4.6；非全表。 |
| EXT-ITC-EDITIONS | [IUCr版次表](https://www.iucr.org/who-we-are/commissions/commission-on-international-tables/editions-and-editors) | A2002第五版/2005重印，C2004第三版；只核版次和目录。 |
| EXT-BROWN-ITCC2021 | [作者正式后续章节](https://onlinelibrary.wiley.com/iucr/itc/Cc/wf5155/) | 2021，ch1.1；§1/§2公式1–7、§3开头；不是1985原论文已读。 |
| EXT-DVXA-BULLETIN33 | [作者原报告](https://www.center.ous.ac.jp/pdf/bulletin/bulletin_33.pdf) | 2012-03，§3/3-1；d3d26n v1.0 F01 writer及单位标志，PDF6/9/10目视。 |


## R134 公开历史资料与专业来源（2026-11-10）

相关版本、用途和访问范围见[SOURCE_NOTES](VESTA研究/SOURCE_NOTES.md)R134与[18项主台账](../output/vesta-study/20261108-113622/reviews/20261110-041200-s02-public-professional-sources/source-reading-ledger.json)、[4项补充来源](../output/vesta-study/20261108-113622/reviews/20261110-041200-s02-public-professional-sources/source-reading-addendum.json)。以下为可定位正文；未读旧论文和完整目标契约保持原状态。

| 来源ID | 准确入口与范围 |
|---|---|
| EXT-VENUS-LEGACY | [原来源](https://web.archive.org/web/20051210165942id_/http://homepage.mac.com:80/fujioizumi/visualization/VENUS.html)；Original last-modified2005-12-02; capture2005-12-10. Later author page capture2015-12-10 and PRIMA history capture2015-10-11 separately recorded. 2005 page sections2/3,8.1-8.2,11.1-11.3,12/13.1 selected; PRIMA version history full body; VICS history latest5.6.1/5.5.1 entries only. Not whole VENUS/history read. |
| EXT-VICS-VEND-MANUAL20060130 | [原来源](https://web.archive.org/web/20060219113930id_/http://quasi.nims.go.jp/powder/VENUS.tbz)；89-page original Dilanian/Izumi manual; PDF created2006-01-30, no numeric manual version; package last-modified2006-01-30, captured2006-02-19. Cover/preface identity; VICS AppendixA pp46-51; VEND AppendixA pp81-85, A.2/A.3 p86. Visual p83. Not all89 pages. |
| EXT-PRIMA-MANUAL20051021 | [原来源](https://web.archive.org/web/20060219113930id_/http://quasi.nims.go.jp/powder/VENUS.tbz)；13-page PRIMA manual; PDF created2005-10-21. No numeric manual edition. Separate PRIMA.pdf cover2004-08-10 conflicts with manual reference2005; not flattened. Cover/contents; section3.3 p4; section8 pp11-12; references identity p13. Visual pp11-12. Section7 only heading/adjacent tail, not full data-set grammar. |
| EXT-VENUS-EXAMPLES20040727 | [原来源](https://web.archive.org/web/20041106231905id_/http://quasi.nims.go.jp/powder/Examples.tbz)；Package last-modified2004-07-27, captured2004-11-06; earlier than2005 page list2005-07-22. Archive223 members. Full111-line YBa2Cu4O8.vcs read statically; xTiO2.den and KDP.den only first24 records only, without tail metadata. No whole density arrays or other examples read. |
| EXT-CONTRD-README20050202 | [原来源](https://web.archive.org/web/20060219113944id_/http://quasi.nims.go.jp/powder/contrd.tbz)；Readme signed Mizuno2002-09-30, Izumi revision2005-02-02; captured2006-02-19; Shift_JIS. Entire Readme after correct Shift_JIS decoding. |
| EXT-CONTRD-SOURCE20040202 | [原来源](https://web.archive.org/web/20060219113944id_/http://quasi.nims.go.jp/powder/contrd.tbz)；contrd.f last changelog2004-02-02; packaged with2005-02-02 Readme; branch-prefixed source requires original build preprocessing. Lines1-338 input/mesh and output writer/FORMAT; no complete PTWAV/OPENWAVF reading claim (adjacent functions displayed only). |
| EXT-ASSE05-AUTHORARCHIVE | [原来源](https://web.archive.org/web/20051110084950id_/http://www.nims.go.jp/cmsc/staff/arai/asse/download/asse-0.5.tar.gz)；Original ASSE0.5 source mtime2003-07-09, captured2005-11-10; author index last-modified2003-08-19/capture2004-12-04. Author2003 index full Japanese body correctly EUC-JP decoded; bundled doc/intro/inputfile.html full; src/iofile.cc77-217, lattice.cc308-395, LatticeWithAtom.cc61-143 selected; introductory Fermi/VTK docs only used to exclude unrelated eb. |
| EXT-WIEN2VENUS-20111124 | [原来源](https://web.archive.org/web/20150322215804id_/http://www.nims.go.jp/cmsc/staff/arai/wien/wien2venus.py)；Script original-last-modified2011-11-24; capture2015-03-22. Author page capture2015-06-25 has2010-03/2011-11 changelog. Entire Python2 script statically read plus author page full body. No execution. |
| <a id="ext-amcsd-paper2003"></a>EXT-AMCSD-PAPER2003 | [原来源](https://web.archive.org/web/20100215144400id_/http://rruff.geo.arizona.edu/xtal/group/pdf/am88_247.pdf)；Downs/Hall-Wallace, American Mineralogist88(2003)247-250; 4pages. Author archive mtime2002-12-12/capture2010-02-15. Print247 whole including Figure1; print248 first continuation line on U/b only. Visual247. Archived AMCSD format help2010 full body is only short amc/cif choice description, not field grammar. |
| EXT-MCE-SVN145-FOU | [原来源](https://svn.code.sf.net/p/mce-marching-cube-eld/svn/!svn/bc/145/IO.cpp)；Fixed SVN revision145; IO.cpp Last-Modified2026-04-09. Current release number not inferred from this revision. ReadFouFile lines438-800 complete, related project root inventory and readme identity only. Not whole4600-line IO.cpp. |
| EXT-FLEET1976-OPENPAGE | [原来源](https://static.cambridge.org/content/id/urn:cambridge.org:id:article:S0026461X00047666/resource/name/firstPage-S0026461X00047666a.jpg)；M.E.Fleet, Mineralogical Magazine40(313)(1976)531-533, DOI10.1180/minmag.1976.040.313.18; publisher public first page531 only; online2018 is not article year. Publisher firstpage531 image completely visually read. Pages532-533 and Robinson1971 body not obtained. |
| EXT-CD-ECoN2001 | [原来源](https://crm2.univ-lorraine.fr/wp-content/uploads/2022/12/CD-IT.pdf)；Nespolo,Ferraris,Ivaldi,Hoppe, ActaCrystB57(2001)652-664; author-hosted journal reprint, DOI10.1107/S0108768101009879 independently confirmed against publisher issue and NLM. Cover/title; section2 beginning PDF3(print653), PDF4(print654) full, PDF5(print655) left opening before section3. Visual PDF4. Not full13pages. |
| EXT-DABAX-WAAS | [原来源](https://ftp.esrf.fr/pub/scisoft/DabaxFiles/f0_WaasKirf.dat)；ESRF file generated2013-12-05; embedded original Waasmaier/Kirfel README April1995. Same42473-byte distribution earlier obtained through OASYS maintainer. Header and embedded original README through column labels, selected H/C/O rows. Not all211species checked. |
| EXT-BIB-096 | [原来源](https://www.ge.infn.it/geant4/temp/saracco/cor/Chantler_2000.pdf)；Original Chantler JPCRD29(4)(2000)597-1048, DOI10.1063/1.1321055; ©2001/received1999/accepted2000 retained;452PDFpages, finalfooter1048 confirms NIST rather than author-list1056. Title/contents; section2 PDF3 ending/PDF4 left(eq3-8); section8 PDF14 right and8.1-8.5 PDF15; lastfooter identity. Visually PDF4/15; other tables/results not read. |
| EXT-CHANTLER-DABAX-ERRATA1995 | [原来源](https://ftp.esrf.fr/pub/scisoft/DabaxFiles/f1f2_Chantler.dat)；FFAST1.0 July1995 converted to DABAX1996-10-24; not FFAST2.1 or2000 update. Header/embedded ERRATUM.TXT through line125; data arrays not read or reused. |
| EXT-ITC-H2019-STANDARDIZATION | [原来源](https://onlinelibrary.wiley.com/iucr/itc/Ha/ch3o7v0001/sec3o7o5o3/)；J.A.Kaduk, InternationalTablesH2019 chapter3.7 DOI10.1107/97809553602060000952; section3.7.5.3 pp316-317. Entire section3.7.5.3 narrative/references; chapter header metadata only. Not fullchapter. |
| EXT-PHASEID2005-STANDARDIZATION | [原来源](https://journals.iucr.org/a/issues/2005/06/00/me0302/)；Brown etal, Working Group on Crystal Phase Identifiers, ActaCrystA61(2005)575-580, DOI10.1107/S010876730503179X. Section6.2.4 full plus footnote1; publication header. |
| EXT-GUNKA-ECoN2019 | [原来源](https://journals.iucr.org/b/issues/2019/01/00/lo5042/)；Gůnka/Zachara, ActaCrystB75(2019)86-96, DOI10.1107/S2052520618017833. Section2 selected equations2-4 and adjacent geometricECoN vschemicalVECN/VDCN discussion; not fullpaper. |
| EXT-CNS11-MAP | [原来源](https://web.archive.org/web/20050304133933id_/http://cns.csb.yale.edu/v1.1/syntax_manual/html/cns-xray-write-map.html)；Official Yale CNS1.1 syntax-manual branch; write/read page original-last-modified1999-12-08, archived2005-03-04. CNS1.3 patch0 homepage separately recovered but not map body. Full xray write map and read map command pages; xray top-level only lattice and read/write map links. |
| EXT-CHIMERAX19-CNS-READER | [原来源](https://raw.githubusercontent.com/RBVI/ChimeraX/7048c533bbbb5abe674c625e5ad248a87f8d21b8/src/bundles/map_data/src/xplor/xplor_format.py)；Official RBVI ChimeraX v1.9 tag resolves7048c533bbbb5abe674c625e5ad248a87f8d21b8 via public GitHub tags API. Consumer implementation version, not CNS producer version. Full109-line Python reader statically read; related readarray helper not read/executed. |
| EXT-F01-INPUT2014 | [原来源](https://www.center.ous.ac.jp/pdf/bulletin/bulletin_35.pdf)；Sakane eduDV development(5), OUS InformationProcessingCenter Bulletin35(2014-03), pp1-32;70-page collected issue including cover/front matter. Contents PDF3 and author/article identity PDF5; footer2014-03 onPDF6 only identity; section6 PDF19(print15) opening and complete F01 input example, visually read. F05 example visible for context but not generalF05 grammar or atomic-energy tables. |
| EXT-MOLDA65-AUTHOR-GUIDE | [原来源](https://web.archive.org/web/20040306184124id_/http://www.molda.org/molda-e/win/guide/G30.htm)；Hiroshi Yoshida official MOLDA for Windows6.5 User Guide; parent Win page title6.5, guide/index and G30/G60 original-last-modified2001-02-02; captures2004-03/07. Download page separately6.5 revision2.0, not presumed sameguide revision. User-guide author/contents metadata and Windows version header; G30 Opening/Importing complete and G60 Saving/Exporting complete; G50 only located/previewed, no numeric-unit conclusion. |

VESTA最新S04接续（063443包）：本轮仅接续S04触发补证：新增90条原生至1740磁盘派生索引；254稳定ID/78样本，90条入口级待核。S04.2仍partial-unchecked，S04.3/.4既有接受保持；13完成/16未完成。dsh本轮独立复审partial、无硬阻断；S04整卡未完成，未进入S05或产品实现。 入口见[HANDOFF](VESTA研究/HANDOFF.md)。

## S05 格式与平台保存专项（2026-11-10）

[逐项证据矩阵](../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/S05-MATRIX.md)消费VM18§18.1–.5、VM06§6.3–.4、VM15§15.7、VM16§16.6及V02旧官方样本；D08/D02§5–6/P04已逐项形成[平台差异](../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/platform-contract-matrix.json)。没有新增专业来源身份或重开S02；PDB33/SCALE/U–B与AMCSD来源均复用原登记，本地P04按SHA另存选读快照，不当VESTA平台已接通。

## S07 CLI、外部程序和运行边界

VM17/VM03、D02§4.2/D10、VM02§2.5/VM14/VM16§16.1与V01本地LICENSE本卡复核入口见[VESTA CLI](VESTA研究/CLI.md)及[S07矩阵](../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/S07-MATRIX.md)。S02 EXT-RIETAN-DOCUMENTS/MANUAL/COOP与EXT-BIB-045/046沿用准确版本/阅读边界；没有新外部安装或许可资格通过。本地V01限制含MADEL和STRUCTURE TIDY；研究输入/程序运行事实与再分发资格分别登记。

## S08 Study收束与设计输入（2026-11-10）

[STUDY-REPORT](VESTA研究/STUDY-REPORT.md)与[HANDOFF](VESTA研究/HANDOFF.md)交付原S08四项；[实际dsh裁定](../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/review-result.json)pass、无硬阻断，[文档发现处理](../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/review-finding-resolutions.json)不冒称二次复审。D01§1/5/V03、CA05/CA08、D06–D12/P01–P07和D02§10–11按原卡引用复核；本轮本地指导资料18份与外部参考选读18份不同，准确源码/章节/导入相对路径替代见[source-reading-ledger](../output/vesta-study/20261108-113622/reviews/20261110-155840-s08-study-closure/source-reading-ledger.json)。Study研究交付完成不代表全功能行为/科学/格式/平台通过，后续P1依赖见报告DPRE-01–08，本轮停止。

## P1逐步设计引用与接续

P1.1按D05/D07/CA07：[架构设计§2](superpowers/specs/2026-11-10-vesta-p1-design.md)；P1.2按D06及FUNCTIONS/FORMATS/ALGORITHMS/CLI：[接口](superpowers/specs/2026-11-10-vesta-interfaces.md)；P1.3按CA04/CA08/METHOD：[CA接线](superpowers/specs/2026-11-10-vesta-ca-wiring.md)；P1.4按D11/D12及已形成设计：[模块计划](superpowers/plans/2026-11-10-vesta-modules.md)。实际dsh裁定partial、仅P1.1接受，不是产品通过。

本轮参考平台HEAD a62fb23b78d4bd10e46f8c44a45a97ce70758f0c/72既有状态；18选用文件与S08字节相同，R/source-baseline.json保存重核。CA方法继续锁1f8508a1，R/method-snapshot使用锁定git对象；失效字面相对链接仍通过S08 source-reading-ledger重定位，不报20字面链接已修复。新依赖一手获取与版本/许可在R/dependency-acquisition.json，文档能力与实际最小probe范围分开。R为E/reviews/20261110-p1-design。


历史R5主体设计续办：D7/I9/C6，资料rev30；31个有界操作、85格式路由/6类字段政策/28入口绑定、15项具体资格阻塞。P1.2/P1.4/整体partial，P1.3仅C4设计接受，C6增量待审；不进入P2–P5、产品实现或运行验证。证据与手动复审入口：[R5](../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/README.md)。


历史R6同卡整改：2026-11-14 R6：D8/I10按dsh R5复审整改D1–D14及σ公式，Codex自验待用户手动转dsh。P1.1继承接受、P1.3 C4设计级接受保持，C6原字节及未审增量身份不变；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料rev31；不进入P2–P5、产品实现、部署或运行验证，真实worker／Check／launcher未授权、未验证。 施工回执与手动复审入口：[R6](../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/README.md)。


当前R7勘误：2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。 [回执/复审入口](../output/vesta-study/20261108-113622/reviews/20261114-152228-p1-design-r7-errata/README.md)。
