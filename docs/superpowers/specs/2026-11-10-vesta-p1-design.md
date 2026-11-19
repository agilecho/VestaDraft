# VESTA P1 架构设计

<!-- P1-CURRENT-STATUS BEGIN -->
当前状态：rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。
<!-- P1-CURRENT-STATUS END -->

2026-11-18。设计版本P1-D35（rev66设计已独审通过；rev67仅接受登记，I33-r1/IP6与wire I33不变；C5条件/C6未审保持）。

R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

## 1. 权威、已确认决定与当前交付

需求以 [D01](../../VESTA全功能重写要求.md)、[D02](../../接入技术要求.md) 为准；功能唯一规格是 [functions.json](../../VESTA研究/functions.json)，从每个记录的 studyHandoff 取得规格/注册样本/规则/下一探针，不另建功能覆盖矩阵。Study 29/29 完成是研究交付，不是全功能或全部科学通过。设计输入必须消费 [STUDY-REPORT §4–7](../../VESTA研究/STUDY-REPORT.md#4-s083领域表示算法和依赖的确定输入)。

用户本会话追加裁定，必须沿用：

- **DEC-P1-01**：默认复现已证原件行为，明确提示科学/数据风险；修正行为须显式选择。未知机制不据名称补造。风险提示和兼容分支必须记录参考版本、功能 ID、证据及具体差异。
- **DEC-P1-02**：默认保存保持原生 `.vesta`＋资源文件；设计完整提交、冲突和恢复。没有批准把 `.vcraft` 包改成默认工程。
- 既有已确认：全功能 1:1、Deep、Web 手改/Agent 当前页面实时操作、三视图只改布局、本人空间、真实保存 ack、60 秒单文件自动保存、真正关页交接；Codex 设计/实施/自验，dsh 独立复审。

选择与用户决定分开：下文 Rust/WASM/Three.js、目录、对象 ID、sidecar 和协议是 **Codex 的设计选择**。未冻结的科学模块不得按本文外壳自行猜完。设计和精确计划必须一起阅读：本设计、[接口](2026-11-10-vesta-interfaces.md)、[CA 接线](2026-11-10-vesta-ca-wiring.md)、[模块计划](../plans/2026-11-10-vesta-modules.md)。

证据根 `R=/Users/agilecho/code/Craft/output/vesta-study/20261108-113622/reviews/20261110-p1-design`。`pre-edit-files.json`/`before/` 是实际预编辑副本；`source-baseline.json`/两类 snapshot 锁来源；`dependency-acquisition.json` 锁当次获取；`portability-probe/` 保留事前输入、源码、实际 argv/stdout/stderr/退出和清理。证据不是产品代码。

## 2. P1.1：路线比较及选择

逐步资料：D05《专题/01-领域模型与编辑算法》建模/事务/撤销、《专题/10-图形CAD三维与材质软件》三维场景；D07《方法/04-Web化与实时交互》§1–6、《专题/05-原生内核到Web与WASM》；CA07 锁定 `app.rs::default_stack/goal/checks`、`mod.rs::RunOptions`。原件无可复用开源内核，CA Web 是分析查看器，默认 egui 不满足本产品。

| 可行候选 | 如何满足完整 Web 编辑 | 成本/限制与证据 | 结论 |
|---|---|---|---|
| A 原创 Rust 领域/codec/科学内核，WASM Worker＋TS 页面，原生同内核 CLI | 人与 Agent 调同 Worker 命令；大数据和计算不占 UI 主线程；关页由 CLI 消费同一工程 | 双工具链/绑定/内存复制需控制；系统进程不能进 WASM；本机最小 wasm/Worker/WebGL2 探针通过，完整依赖集成和目标部署未测 | **选用**；避免两套科学实现漂移，并保留 Linux 无图形领域路径 |
| B 原创 TS 内核＋Node CLI，必要重计算受控服务 | 浏览器和 CLI 共享 TS；Three.js 同样可编辑/渲染 | 也能满足目标，但大量 float32 兼容、FFT/二进制 codec 和以后 native 外算仍需额外实现/边界；未有现成 TS VESTA 内核或全科学库证据 | 保留替代；若 A 的有界依赖/目标环境试验失败，再按同接口重新评估，不自动切换 |
| C 服务端 Rust 内核＋Web 编辑器/渲染客户端 | 页面发送正式编辑命令、消费结构化增量，Web 本地渲染；外算天然隔离 | 强网络依赖、离线交互/队列/资源传输成本；仍需页面模型/选取，不是视频远控；所有普通交互走服务端缺当前必要性 | 仅把确需进程的外算作为 A 的局部服务，普通领域操作不采用 |

选择不是“已全功能可运行”。拒绝以桌面 egui 或轻量查看器替代产品。科学规则由原创实现承担，Three.js 只承担图形基础，RustFFT 只承担离散变换，均不代替 VESTA 行为/数值规格。

### 2.1 选用依赖和能力边界

当次 primary registry/维护者文档于 2026-11-10 获取，精确字节及 SHA 在 R/dependency-acquisition.json；以下为设计候选的明确版本，不是已生成依赖锁或已经编译的事实。实施时锁完整 transitive graph，若版本不可得/不兼容则回到设计，不换 latest。

| 层 | 设计版本/特性 | 能力依据及许可 | 仍需验证 |
|---|---|---|---|
| Rust | 已安装 1.99.1；WASM scalar 单线程 | 本轮 `probe01-rust-wrapper.json` 实际编译；WASM 无 std I/O 注入 | Linux x86 工具链/镜像及完整依赖构建 |
| wasm-bindgen | 0.2.130，生成器必须同版；无 shared memory、无 wasm SIMD 基线 | [维护者 Worker 示例](https://rustwasm.github.io/docs/wasm-bindgen/examples/wasm-in-web-worker.html)；registry MSRV1.81，MIT OR Apache-2.0 | 本轮探针是直接 WASM，不声称绑定器或其全依赖已编译 |
| serde | 1.0.229，derive | registry MSRV1.56，MIT OR Apache-2.0；只做 schema 序列化 | 项目 wire 类型/非有限数策略和精确锁 |
| RustFFT | 6.4.1，default-features=false；先 scalar，native 优化后验 | [维护者文档](https://docs.rs/rustfft/6.4.1/rustfft/)：纯 Rust、f32/f64、输出未归一、升序频率；MIT OR Apache-2.0 | Fourier 正负指数、V/N、闭合边界、Nyquist 由本款算法负责，不能套默认归一替代 S06 |
| Web | TypeScript7.0.2、React19.3.1、Vite8.3.7、Three.js0.187.0 | 当次 npm 元数据：TS Apache-2.0，其余 MIT；Vite Node^20.19或>=22.12；本机Node22.23.2满足该声明 | React DOM/types 完整锁、构建/中文/IME/可访问性未测 |
| 图形 | Three.js WebGLRenderer；GPU 仅显示 | [维护者文档](https://threejs.org/docs/pages/WebGLRenderer.html) 明确 WebGL2，r163 后无WebGL1；透明排序不保证所有情况正确 | instancing/选取/透明/椭球/等值面、线宽和印刷导出须按原证，不能以renderer存在当全渲染通过 |
| 测试 | Playwright1.65.0，Apache-2.0，Node>=20 | 当次 npm 元数据；仅未来测试依赖 | 未安装/未运行；锁浏览器 revision/镜像摘要后才可执行计划命令 |
| 字体/参数/外算 | 本轮未选定可交付原件资产 | DPRE-06、S07外部合同、SOURCE_NOTES；许可文字/数据表可读不等于可打包 | 见§7；不自动打包原件、官方样本/表、系统字体或Fortran库 |

zip9.5.0 只被比较/获取资料，**未选为产品依赖**，因为 DEC-P1-02 选择原生多文件工程。上述版本的发行方许可元数据是技术选型依据，不是整包再分发资格结论；最终 LICENSE/NOTICE 和 transitive 许可仍有实施验收。

### 2.2 有界实验证据

P1-PROBE-01 先登记 input-plan.json：非正交 H 列(2,0,0)/(1.5,3√3/2,0)/(0,0,4)，f=(.5,.5,0)，独立平方距离=4.75，公差1e-12；像素预期[51,102,153,255]。实际 Rust1.99.1 编译为 WASM，Chrome Worker 实例化、计算4.75并转移 ArrayBuffer，源 byteLength=0；WebGL2清屏读回完全一致。Chrome实际 ANGLE Metal Renderer: Apple M5。两个 wrapper 真实退出0、未超时；浏览器1.19秒是本次探针进程时长，不是产品冷启动/p95。仅本机 Mac：不证明 wasm-bindgen/Three.js全量、Linux x86、最大网格、科学全谱或目标嵌入环境。临时本地server关闭、拥有的全新浏览器profile已清理；无原件或平台服务启动。

### 2.3 产品位置及目录职责

当次已核 Craft 非Git；`/Users/agilecho/code/VestaCraft` 不存在。明确设计选择：未来 CA 原生 workspace 根为该绝对路径，**产品仓库根为其 `app/`**，现阶段不创建、不初始化Git。资料与调查继续在 Craft，不搬迁/覆盖 vesta.cany，不在共享 IntegrateCraft 内开发产品。

| 计划路径（均尚不存在） | 职责 |
|---|---|
| `/Users/agilecho/code/VestaCraft/craft.json`、`investigation.cany/`、`capture/`、`checks/`、`inbox/` | CA生成/管理的原生记录、检查和提问；不是产品实现目录 |
| `/Users/agilecho/code/VestaCraft/reference/` | 经摘要绑定的只读需求/本P1文档/研究导航；Study调查不自行替换成另一版本 |
| `/Users/agilecho/code/VestaCraft/app/` | 未来产品Git根，所有原创实现、许可证、测试和制品脚本；当前未建 |
| `app/kernel/Cargo.toml`、`kernel/crates/{model,commands,codecs,geometry,science,bindings,cli}/` | Rust workspace；纯领域禁止依赖平台账号/DOM/进程，bindings/CLI分别实现适配 |
| `app/web/src/{kernel,render,editor,host}/` | Worker协议、Three.js显示、人工交互、薄宿主桥；无第二套科学规则 |
| `app/package.json`、`scripts/`、`Build/vestacraft-preview` | 根build/test合同、CA截图启动桥；root不放Cargo.toml，防止CA先只cargo build遗漏Web |
| `app/tests/{domain,codec,contracts,browser,external,delivery}/` | 具名fixture/oracle与真实链；不写假保存/假通过 |
| `app/assets/`、`licenses/`、`dist/`、`build-manifest.json` | 获准资产、原始许可、静态Web/CLI和实际加载指纹 |
| `/Users/agilecho/code/IntegrateCraft/craft/vestacraft/`及软件插件相关差异 | **未来P4计划位置**；本轮不创建/写共享仓库；补丁/注册须以当卡版本重核 |

## 3. P1.2：领域与运行边界

逐步资料 D06 两份、FUNCTIONS/FORMATS/ALGORITHMS/CLI，以及五域行为01–05；具体协议见接口文档。结构/Scene分离、单位/身份/提交边界已设计；一般群、多相变换、调制/准晶、ADP椭球、等值面拓扑、参数表仍有会影响公共科学payload的缺口，不能把 `unknown JSON` 当冻结接口。

依赖流：领域身份/单位/H/G → codec和命令事务 → 周期像/几何 → 科学场与资源 → 渲染与选取；人工与Agent都经过commands；保存桥处理内核产生的同一不可变快照。外部计算只能输出候选结果，在当前revision仍匹配时经正式命令提交。

- Phase包含结构/volume/shape的显式可空层；volume可有自己的cell，不强绑结构H。
- Site是非对称位点；ImageKey=(phaseId,siteId,symmetryOpId,latticeShift)，测量选择ImageKey而非标签。删site不能只隐藏；隐藏和选择分别表示。
- Scene持久相机/显示/边界等与结构分开，hover/临时选择/资源句柄不当持久身份；原件实际保存的DLATM等显示状态须按codec记录。
- H列为Å笛卡尔晶格，fractional无量纲；G=HᵀH，体积|detH|；平面normal=H⁻ᵀh，D与d_hkl分开。数学基础不依赖一般群未知。
- 持久数值采用f64并保留原词法/缺失标志；参考行为需要的binary32入参或中间平方在具名兼容算子显式转换，禁止全局降精度。每个oracle自己的公差/打印政策；R64原失败与后续5位打印政策并存。
- 当前场协议以接口I6的FieldDescriptor2为准，字段是id、kind、unit、declaredUnit、axisOrder、coreShape、closure、originA、stepBasisA、periodic、dtype、resource。旧basis/closureShape/resourceId摘要不再作为当前wire名称；电子/nuclear/Patterson单位不得混用，CUBE坐标bohr不推出scalar换算。
- `LegacyObserved`是默认行为策略；`Corrected`只有显式 opt-in 且该修正的算法/oracle已定义时允许，不预建“正确”空模式。没有证据的分支返回未冻结/未实现，不编造数值成功。

## 4. 原生工程、多资源与平台完成契约

详细形状与例子见接口§5。保留 `.vesta`主文件；资源放相邻 `<leaf>.resources/`，不可变内容寻址，主文件最后更新。小的产品manifest sidecar用于资源SHA和schema，命名由**主文件SHA**推导；对象ID只保证载入生命周期内稳定，重载换document/epoch（接口§1），不修改原件研究字节；不要求原VESTA理解sidecar。目标是否接受特定相对resource路径/未知块保留仍归codec对照，不能当已测。

平台保存可恢复是已确认D02契约，不能为了复现原件的路径拼接缺陷，故意把本产品刚保存的资源丢掉。参考导入失败和兼容导出差异保留风险报告；本产品保存/重开按manifest完整性恢复是D02明确要求的差异。崩溃分支也不能让宿主/其它稿一起崩溃，控制为带证据的 `E_REFERENCE_FAILURE`，保留当前工程；此为平台隔离边界，不把它说成原件正常返回。

单个资源失败：不写主文件、不ack；主文件失败：旧主文件仍指旧资源，新的孤立资源保留具名清理清单，不自动删除可能被别稿引用的文件。只有所有资源、metadata、主文件实际持久字节及artifact-only完整对象、owner、全数值和typed来源验证均通过，且精确操作确认，才结算对应快照Saved。60秒自动保存采用同一snapshot/ack协议，真正关闭先确认dirty快照已保存，失败阻止受控关页/交接；浏览器硬关闭不能保证最后一次异步写成，下一次只恢复已确认文件，不创建浏览器副本。

P04实际SpaceFiles Linux-only anchored I/O，write有temp+sync+rename；SoftwareFiles有SHA verify。它们未提供跨文件事务、全局CAS或多文件配额预留，不能直接称原子工程提交。设计使用不可变资源+最后主文件提交缩小范围；所有正式写入必须进已有平台队列/路径/配额，最终主写前再verify。异进程在verify与rename间的竞争保证仍需平台当卡核实（§7 G-HOST），不能声称已有防任意外改原子CAS。

## 5. Web、Agent、外算与编辑权

P02注册/启动/client，P03 PageChannel claim/send/poll/reply/release和既有serial，P04文件守卫，P05 Word样例，P06制品锁，P07测试组织均只读参考。P1 初次设计所用平台参考 HEAD `a62fb23b78d4bd10e46f8c44a45a97ce70758f0c`，当时72条既有status；18选用文件与S08及当次HEAD均相同。来源字节/原始status在R，未修改共享仓库。它不是活跃仓库当前状态，历史S08 HEAD9dc5412快照不回改；本轮只读观察与采用边界见§9。

页面状态 created→loading→ready/busy→failed/closing→closed；generation每次load新稿/重建实例递增，旧任务不能提交。三个layout mode不触发owner变化，不unmount、不release、不重新open。只有实际关闭且保存确认后释放页面claim、冻结该lease并让CLI取得同一作品的新lease；断线/页面心跳过期只能撤销旧写lease，不能把未保存内存声明已交接。CLI从最后已确认native工程读取，显式重开先刷新baseline和manifest。

不逐行复制 `WordCoordinator::changeMode` 或通用 `SoftwareCoordinator::changeMode`。拟新增 VestaCoordinator 复用既有serial与文件对象，重写layout/close/route最小差异；P4才施工。serial内部直接调用 `saveCurrentInTurn` 等内部方法，不再次调用带serial的public save；poll/reply/ack在锁外能完成，避免等待自己的队列。

页内Agent正式工具 `command_list/command_run/command_batch/doc_inspect/render_preview` 转产品注册表；真实schema未实现前不能注册假工具。外算是现有软件队列中的受控任务，带输入资源、revision、phase、取消/进程树；不另建账户、运输或计费。页面仍拥有编辑权时不让CLI另写作品；CLI/外算的结果回到页面候选提交。原件16CLI和alias/stub只作参考证据，不自动暴露成产品API。

## 6. P1.3：执行与检查

完整锁定协议和未来命令见 [CA接线](2026-11-10-vesta-ca-wiring.md)。当前没有产品workspace/app/worker/inbox，不能调用rebuild_check制造超时，也没有原生mechanics通过。Codex CLI是未来CA选用builder；本会话Codex负责P1，dsh只读复审。已核源码与help不等于已跑builder。

CA原生AppKind会要求build、适用Runs、截图和Deep mechanics；Web+Rust混合项目需要真实 `Build/vestacraft-preview --screenshot`启动桥和根npm build合同。不能只因npm build通过就略过桌面AppKind的截图/运行差异；本款还须独立Web/领域/codec/平台验收。CA截图桥和原生worker实际联调是 G-CA，未运行。

## 7. DPRE-01–08消费与冻结门

这是设计依赖表，不是新的功能规格或覆盖矩阵。每门都继承原unknown/revision，不增造原生unknown。必要补证先在新证据目录登记具体输入、独立预期、资源和失败恢复，然后走E/run_probe.py；无变化的历史不重跑。

| 门/DPRE | 已能固定 | 仍缺及受影响的冻结范围 | 最小下一动作/可继续项 |
|---|---|---|---|
| G-SCI /01 | H/G、单位、身份、ImageKey、revision、Field描述及选定oracle | science unk_522695d0039b3283；31组unknown/nextProbe，尤其一般群/磁群/调制、多相R/t、ADP/键价表、isosurface/投影/Nyquist/反常表；这些payload/算法不冻结 | 按S06每组nextProbe独立准备控制，先phase+cell-symmetry的新非正交两相/一般群模型；基础命令/身份/保存可继续 |
| G-CODEC /02 | 分格式的Result/诊断、原始块保留、默认legacy策略、资源事务 | PRI/ABI、m3d、DEN/STO、Elk/ASSE/GAMESS等字段未解；resolved CUBE/密度/Raster仍失败；不能伪写全48输入/23输出精确实现计划 | 复用S05原字节+独立解析，按UNKNOWNS各头下一探针；先原生结构子集/CIF已证字段，完整写回拒绝丢失未理解字段 |
| G-WEB /03 | Rust/WASM单worker+WebGL2路线及版本能力/许可声明；本机最小probe通过 | 完整wasm-bindgen/Three集成、静态嵌入CSP/MIME、真实预算与大grid未测 | 在未来授权下依赖小spike，先无共享内存、真实字体、VS54/59/102分段测；不改共享隔离头 |
| G-HOST /04 | layout≠owner、快照/ack、不可变资源/主文件末提交与冲突形状 | 现有服务无multi-resource quota预留/任意外改CAS；extension/naming新增差异、完整Linux安全链未验 | 明确复用SoftwareFiles、SpaceFiles及最小host批提交接口；P4卡核写入竞争承诺/共享写者范围后冻结；纯内核事务可继续 |
| G-CA /05 | worker/checks/asks/inbox、自查拒绝、源码/help与恢复协议 | 新workspace尚无；CA AppKind对混合Web的launcher与原始capture/调查迁移尚无实际接线，不能报Ready | 未来授权产品施工时先建最小真实链/launcher，锁输入与调用再启动Codex worker；本轮不调用 |
| G-ASSET /06 | 外算接口/错误不删功能；默认不打包原件 | MADEL/RIETAN/ORFFE生产者科学/运行件、TIDY一般群；表/字体/模板/样本再分发逐项待核 | 先每项登记来源、版本、许可、支持目标、独立oracle；未获资格的能力继续列计划，不伪支持、不装全局依赖 |
| G-UX /07 | IME原生DOM输入，命令草稿/预览与提交分开；按层测性能 | VS12只有路径；中文字体、IME/标签/导出、目标浏览器、RSS/OOM/GPU/冷启/p95/最大grid及生产预算 | 预登记“晶胞α、占位½”fixture；VS54/59/102与新有界grid；测试配额不是生产预算，未测不设通过值 |
| G-REF /08 | 本轮平台18份/HEAD/status快照、CA locked git读取 | 历史缺SHA/未归档图仍不可复查；新施工时活跃平台可能再变 | 每卡重核选用文件/制品，不改旧封存，不清理共享改动；CA方法继续锁1f8508a1 |

冻结边界：基础身份/提交/保存协议可审；G-SCI/G-CODEC相关科学/codec公共接口尚未全部冻结；G-HOST和G-CA有精确接缝方案但未实际联调/部分承诺待核。不能用本文件把P1.2/P1.4全功能计划自动标完成。下一会话先完成这些会改变相邻接口的设计补证，再扩精确实施卡；不要求所有行为未知先关闭才能推进独立模块。

## 8. 修订D2的自验补正

首轮dsh因STREAM_CLOSED退出1，没有最终裁定，原输入/日志保持；不能称pass。Codex自验发现并修正：有package.json时main.py不会自动launch。P1-PROBE-02链接S03既有锁定engine rlib，仅解析隔离fixture，证实main.py被忽略、唯一Build可执行候选会被选中；候选未执行。未来入口为唯一Build/vestacraft-preview，CLI置dist/cli，防mtime误选；真实浏览器/AX/worker仍需G-CA联调。

接口I2补基础模型/ResourcePort、导出format与当前工程binding分离、snapshot字节桥/lease、增量job调度和batch结果。这是Codex的修订，不是首轮dsh正式接受。

补齐选定工具/协议依赖，精确获取见R/dependency-protocol-supplement.json：wasm-bindgen-cli0.2.130（MSRV1.86、MIT OR Apache-2.0）必须与crate同版，未来安装至本产品隔离tools目录，不全局安装；serde_json1.0.145（MSRV1.61、MIT OR Apache-2.0）用于wire JSON；sha2 0.10.9（MIT OR Apache-2.0，registry未声明MSRV）用于跨native/WASM同一SHA-256，禁原生汇编特性、默认特性关闭。React DOM19.3.1已核peer React^19.3.1、MIT。它们均仅选型/获取证据，未编译集成；首次元数据TLS EOF已保留失败说明，后用固定版本endpoint成功，不伪造首轮成功。

## 历史：9 P1 设计续办 D3（R1送审时）

本轮执行[2026-11-14 续办卡](../plans/2026-11-14-vesta-p1-design-continuation.md)，用户随后明确要求在当前会话执行。角色继续为 Codex 设计／自验，用户手动转交 dsh 独立复审。证据为 [R1](../../../output/vesta-study/20261108-113622/reviews/20261114-100546-p1-design-continuation/)，实际根为 Draft；没有初始化 Git、产品 workspace、worker、builder 或原生 Check。

I5 增加 canonical 相定位、带基底 ADP、显式单步网格、格式 dispatch／loss 和宿主查询恢复边界；机器示例明确只覆盖这些类型及三个科学请求子集。它没有以宽松 options JSON 填补全域参数。C3 补出 prepare 的公开调用／独立锁定源码构建、固定 request/target/stack/kind、逐文件 Study/capture/mechanics 迁入、唯一启动桥、checks/asks 与 session 恢复的未来合同。模块计划增加 31 个领域的具体文件、测试目标、样本摘要、独立断言和进入门，并具名安排 G6 的应用／偏好／数据库／焦点和宿主差异。

P1.2 仍有 I5 §11.6 的 C-SCI-01/02/03、C-CODEC-01、C-HOST-01 等未冻结公共决定。P1.4 也因此仍缺相应高级 DTO／writer 的可开工签名和完整 oracle；列出所有 ID 或测试文件不能替它通过。C3 的设计合同可独立送审，但 prepare／worker／Check 真实运行仍未验证，原 P1.3 接受状态保持 false。全 P1 不预勾；需要 dsh 分别判断设计与后续实施验证的归属，不通过本轮自验改写历史 partial。

活跃平台本轮只读观察 HEAD=`acfa7878b1928de7cc1a79fec998be172cb8dd48`，2 条既有工作区状态，与 P1 初次快照不相同；实际 stdout/status 摘要在 R1/platform-observation.json。当前设计中的既有能力仍明确以原锁定快照为参考，没有把新 HEAD 的文件默认为相同，也没有在另一项目施工。P4 开工前须按届时版本逐入口重核，不能沿用旧72条状态或声称当前平台已接通。

本卡自验核设计 schema 子集／反例、数学独立输入、协议走查、原件研究样本／调查及旧 S08 封存身份和设计交付一致性。它不执行科学算法、真实文件保存重开、渲染、原件探针或 CA 联调。已确认的默认 legacy 风险提示和原生 .vesta＋资源保存不变；没有用未解问题删除最终功能。

## 历史：10 dsh复审后的D4整改与用户口径（R2送审时）

用户已确认DEC-P1-03：P1.3按设计级验收，真实worker/Check/launcher验证另行授权，权威记录在D01§11和总体计划P1段。R2只做设计整改，不新运行原件、产品或模型，不自动联系dsh。

I6补了可追溯RuleRegistry／FormatProfileRegistry、与schema一致的类型名、u64负例、带初始lastSavedDigest/epoch的保存演算，以及PNG显式Rendered帧绑定。C4明确主程序文件摘要、调查全部普通文件的规范清单摘要、expected counts/kind、Cargo特性草案、公开crate根证据和浏览器未绑定前拒绝启动。模块计划增加85逐格式路由及31单元的签名/原oracle与输入SHA/取消清理/失败续作/交付合同。

这些整改不解决原七类科学/格式/宿主/资产/UI门；31单元接口仍只是已证子算子提案，完整高级DTO和oracle的F13继续部分未完成，不能将完整缺口清单缩称“只有七项”。P1.2/P1.4仍partial；P1.3设计稿待用户手动转交dsh复核其条件是否满足，作者不自行勾选。新验证报告把语义断言、设计结构、证据完整性和交付结构分列，不将总数当设计覆盖率。


## 历史：11 R3的设计级接受登记及同卡整改

用户带回的dsh R2复审明确C4设计闭合，用户随后授权把P1.3设计级接受回写。接受来源及被审C4摘要在R3/P1.3-design-acceptance.json；当前C5浏览器/措辞增量待独立复核，不撤销已有设计级接受，也不说新稿已经被审。真实worker/Check/launcher是后续独立运行验证资格，不能再用未运行阻止P1.3设计接受。P1.2/P1.4及整体P1继续partial。

I7消除旧RuleRef字段漂移并给所有schema def补同名title。模块R3把oracle逐项区分direct-result/result-invariant/supporting-only/capability，绑定真正数值/公式/精度指针；相定位不再误用局部形貌面积，缺失直接结果时写missingExpected，所有fullUnitReady仍false。路径统一为项目根相对POSIX并显式pathBase，原sourcePointer内的历史定位值不改写。14图像路由补逐行/列/菜单token定位；C5补浏览器可执行路径来源及完整匹配/不匹配纸面例。没有执行产品、原件、外算或CA运行命令。


## 12 R4证据保真及C6输入合同整改

P1.3 C4设计接受仍由原dsh文字与用户授权共同支持；机器review_result只指向真实dsh R3机器结果，不能把作者转录当原始裁定。R4忠实摘要保持原文pre-write-back的P1_3_complete=false，用户授权后的true只在独立接受决定和当前状态中，二者身份分开。§9/§10/§11记录历史过程，当前以本节与文件头为准。C6定义buildRoot、外部固定摘要、获准归档表、安装receipt字段/路径/格式/失效，生产来源表当前为空；全部都是设计输入与合成反例，无安装/启动。N1/N2/N5低风险字段说明补齐，完整F1/F13仍未关闭。


## 历史：12 D7／R5主体设计（当前增补见§13）

本轮范围是现有证据支持的主体设计。接口I9§15新增带单位/基底/来源的科学对象与31个有界Input/Output；模块计划R5逐单元接到schema、原输入SHA、oracle角色、预算、取消/清理和失败交付，不再把所有高级公共边界留作自由options。它们包括纯科学子算子、结果准入和导出计划，各自边界明确；不把小算子称为整组完整接口。

格式方面，85入口继续逐项派发，6类字段/损失政策通过28个精确入口绑定；已知m3d参数、PRI flag0、网格闭合/轴序和CIF元数据损失能固定，未知位打包/方言不能凭壳推定。宿主/应用固定候选导入、外算结果准入和保存中间态；当前平台实例内串行不具备任意写者CAS。保存仍`.vesta`+资源、旧稿保护、实际读回后ack，未建立另一存储服务。

15项具体阻塞见[R5 qualification-blockers.json](../../../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/contracts/qualification-blockers.json)：B01相映射；B02群/磁群/调制及资产；B03张量/协方差；B04重复/键/边界与BV；B05平面拟合；B06视图；B07完整科学生成器；B08一般网格；B09形貌/等值面；B10渲染资产；B11投影/峰盆地/转换；B12格式；B13保存并发与原生资源路径；B14外算；B15数据库Stop/网络错误。每项最小新材料和许可种类已写清，不能拿本卡设计授权执行探针。

P1.1/P1.3 C4设计接受保持2/2；C6原字节保持R4，C5真实有条件结论和C6未审状态不变。D7/I9为Codex自验设计，等待用户手动转dsh；P1.2、P1.4与整体P1仍partial，不进入P2–P5、产品实现或真实worker/Check/launcher验证。当前资料rev30仅登记本次新增文档，不追改旧baseline/复审。


## 历史及继承：13 D8／R6整改（差异见§14）

2026-11-14 R6：D8/I10按dsh R5复审整改D1–D14及σ公式，Codex自验待用户手动转dsh。P1.1继承接受、P1.3 C4设计级接受保持，C6原字节及未审增量身份不变；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料rev31；不进入P2–P5、产品实现、部署或运行验证，真实worker／Check／launcher未授权、未验证。

I10§16以有身份的笛卡尔测量协方差明确J链和variance/σ，导出拆PlanEncode/EncodeApproved并绑定完整损失摘要；31领域允许32操作，旧内部step与旧oracle留作历史/辅助。每个结果有具名消费方，负/零张量分类与半空间输入域补完整。I8示例保留版本，新Field按schema再按语义验；112个定义均具名，17条语义例不冒称全科学实现。

85入口中57个逐字段审计仍未定义，先消费旧材料；B02不预设必须新实验，B12的8个codec问题、B13平台来源与B15应用来源具名。polyhedron关联B04/B09；指定版本NoAction能力设计不制造额外实验。具体修改/正反例/未验证见[R6](../../../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/README.md)。完整资格、20组完整来源映射及真实实现验证仍未完成，P1不推进。


## 历史：14 D9／R7交接勘误（rev32时点）

2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

字段身份通过FieldHandle进入EncodeInventory与LossField.objectKind/objectId/相对指针，旧同意因新摘要重做；shape按选定凸边界几何投影，而不是多边形faces与triangles数组改名。I11§17和[R7契约](../../../output/vesta-study/20261108-113622/reviews/20261114-152228-p1-design-r7-errata/contracts/)给当前规则、15个身份例、11个提供Mesh的几何例及8相48面源适配。一般半空间分类/粉末拒绝套件/真实产品仍未证明，完整P1不前进。rev32纠正文书，旧rev31不改。

## 历史：15 R8交接元数据勘误（rev33时点，D9技术设计不变）

R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

R7独立复审确认身份和投影设计实质整改，仍有协议权威版本、measure适配说明、shape模块角色三处残留。本轮只修这些当前交接项，并以同一检查核对schema/消息说明/来源计划/模块；不追加实验或追认C6增量。R7复审原件见[报告](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-160100-p1-design-r7-dsh-review/REVIEW.md)；R8当时材料见[R8](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-161832-p1-design-r8-consistency/README.md)。

## 16 R9文书与状态勘误（D9技术设计不变）

R8-R1–R3只涉及活跃文书与状态导航：旧R6协议/112定义句明确为历史，当前登记/链接标签跟随本轮；next_card、finding_resolutions、需求归档与当前交付同轮。原R7/R8段保持其时点，不混写新摘要；当前设计与[R9 contracts](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163802-p1-design-r9-doc-state/contracts)逐文件继承R8。该R9/rev34快照现经[dsh R9](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-170522-p1-design-r9-dsh-review/review-result.json)确认三项勘误闭合。当前rev35仅回写该结论及非阻塞续办，不把本轮新文书SHA冒称已独审，也不接受整体P1或C6。

## 17 R9闭合结果回写与P1-01接手（rev35）

2026-11-14 R9闭合回写：dsh已确认R8-R1–R3在rev34被审范围闭合，M1–M8登记为非阻塞续办；本轮追加rev35，仅回写结论及接手入口，D9/I11/C6技术版本、契约、数学和R6-R5证明边界不变。P1.1继承接受，P1.3 C4设计级接受保持、C6增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。下一卡P1-01由用户新开施工会话，当前六卡均未施工；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

已审来源及SHA见[闭合记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-171658-p1-r9-closure-rev35/R9-closure-acceptance.json)；[M1–M8非阻塞续办](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/非阻塞续办.md)不构成01开工前置。原R9设计契约保持封存，后续结构设计按[01卡](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/01-结构群与几何接口.md)新会话接手。


## 10. P1-01结构、群与几何增量（D10，partial待审）

2026-11-14 P1-01设计交回：追加rev36，D10/I12增量提出SG/1结构、群与几何公共设计及来源/独立演算；完整14组接口仍有6项精确缺口，本卡partial、待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受。R9闭合仅属被审rev34，历史补档保留；02–06未施工。不进入产品实现或运行验证，真实worker／Check／launcher另行授权。

原§3/7的缺口由本卡逐字段设计细化，完整冻结不升级。新增类型和十四组生产消费/单位/生命周期详见[SG/1设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/design.md)及[缺口](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/remaining-issues.json)。原R9的113个有界定义和protocol3继承，新增SG类型是独立设计profile；调制/超空间与一般原件相R/t没有伪造占位接口。相邻02/03/04/05消费版本、限制及资产触发范围见[共享类型影响](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/shared-type-impact.json)。


## 11. DEC-P1-04结构适配（P1-01所属范围）

新确认结构约束及原并发字节均保留。按WC01–WC03对应基础单位/model、geometry/science派生、codecs、Engine/commands、Agent绑定、Web/CLI/宿主入口；具体位置、依赖方向与差异理由见[本卡适配](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/wordcraft-structure-adaptation.md)及[设计依赖图](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/wordcraft-layer-proposal.json)。SG/1公共语义和6项缺口不变；不切换已接受栈、不在01完成06全工程汇总或运行产品。


## 12. P1-01 F1–F6同卡修补（D11，完整设计仍partial）

2026-11-14 P1-01同卡F1–F6整改：追加rev38，D11/I13/C6；另建SG/1-r2证据后继，修复数学引用、明确角色权威及四条旧表冲突、登记保留AGENTS后继、补13条MODEL/VECTS入口，并补schema范围/语义拒绝例及归档基路径。rev37旧SG/1已获dsh“主体准确、partial、轻微证据整改”结论；本次新增量仅Codex自验、待用户手动转dsh。完整六项科学缺口不变，P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial、2接受/2未接受。02–06未启动，产品和真实运行未授权未验证。

当前SG/1-r2公共设计见[本包设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/design.md)、[F1–F6修补说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/correction-notes.md)。旧113定义/39新增类型名、科学公式/原六项完整冻结缺口不变。当前payload增加negativevariance/RGBA范围、nonempty covariance及明确的schema后domain义务，不能以文档检查代替生产validator。WordCraft分层/依赖和已接受路线保持，05仅阻塞实际资产消费。旧R9/SG1/dsh/再复核/AGENTS字节保留。


## 当前续办：P1-01 F1–F6独审结论登记（rev39）

2026-11-14 P1-01同卡独审结论登记：追加资料rev39，技术版本仍D11/I13/C6；dsh已独立复审通过rev38的F1–F6证据/边界修补，最新独审指针指向该被审包及真实JSON/SHA。本次rev39仅回写结论并Codex自验，O1登记为非阻断报告导航说明，不改旧封存或AGENTS。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受，SG-01…SG-06六项科学缺口不变。02–06未启动，产品和运行验证未授权。

最新[dsh独审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/REVIEW.md) / [真实JSON](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/review-result.json) / [本次登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/RECEIPT.md) / [O1非阻断说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/nonblocking-observations.json)。

O1：旧oracle-bindings.currentVerificationEvidence定位04-full正常模式546项报告；封存只读final为570项，两者均exit0/0失败。final入口为[/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json)。本次只登记该导航边界，不改旧字段、旧报告或封存。独审被审版本是rev38，rev39仅为结论回写且尚无新的独立复审。


## P1-05资源、依赖和目标边界增量（D12）

本节补充§2.1/7，不重选路线。当前资产资格以[AB/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/contracts/asset-inputs.json)、[来源输入](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/input-manifest.json)和[消费边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/consumer-handoff.md)为准：普通/磁群、Wyckoff、BV、半径颜色、散射/反常、精确同位素、概率、字体/显示、模板/样本及其它常数共17类，逐项给已有字节或具名缺件、许可、字段/单位和生产消费方。raw-header/BNS表键是研究定位，不能填作已资格化setting/origin。Noto CJK等3face固定本地manifest/内部版本/版权与OFL，指定字符串cmap可核；原包fonts.raw不复用为OpenType。所有资产productionReady仍false，各自未验证项见合同，不由存在性关闭SG科学缺口。

[BT/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/build-targets.contract.json)与[依赖输入](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/dependency-inputs.json)固定12条已选直接依赖元数据版本/校验及特性和同版绑定器，明确完整Cargo/npm/generator锁、工具链/SDK/npm/浏览器制品和Linux镜像尚缺；不补latest或把registry许可当整包资格。app根npm混合构建、kernel独立Rustworkspace、WASM scalar单Worker和同内核CLI保持。每条未来命令有cwd/前提/输出/失败；静态CSP/MIME/字体就绪须04实际平台合同，未改共享头。

[WC适配](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/wordcraft-structure-adaptation.md)把WordCraft的基础/领域/资源→格式科学→统一命令→适配/应用方向用于本卡；工作字节与HEAD各自绑定，不照搬egui或原生Rust-only规则。原science/src/external未来必须拆纯输入/解析与native进程适配，WASM不引入进程。字体按外置不可变静态资源按需加载，16MB CJK性能未测。浏览器作品恢复仍遵D02，不能照搬桌面profile副本。

CA文档C6保持原SHA，C4接受/C5条件/C6未审逐项见[血缘](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/ca-lineage.json)。没有发现需改C6的具体相邻差异；空production浏览器来源表不因本卡获得资格。未来冷/增量120秒、IME/字体、GPU/OOM/VS102和Linux/x86、真实bootstrap/worker/Check/launcher安排见[计划](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/future-validation-plan.md)；全部未执行未授权。


## P1-05独审登记与说明后继（rev41，技术语义不变）

2026-11-14 P1-05独审登记及非阻断勘误：追加rev41，技术版本D12/I14/C6不变；dsh确认被审rev40 AB/1、EX/1、BT/1的partial设计/来源/保全边界准确、无阻断。本轮以独立后继修O1–O4，补未施工卡名单和fontTools复现环境说明，仅Codex自验、说明增量未独审；原施工包、dsh报告及rev40封存不改。05/01继续partial，SG-01…SG-06开放，C4接受/C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。02/03/04/06未施工；不启动下一卡、产品或真实运行，交回后停止。

被审原稿与SHA见[登记对象](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/review-registration.json)；当前定位/粒度补充为[AB/1-notes-r1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/contracts/asset-inputs.json)、[EX/1-notes-r1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/external-tools.contract.json)及[消费说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/consumer-handoff.md)，BT/1仍消费原被审字节。原17类资产对象、四外算科学/进程规则、build目标、protocol3/SG1-r2及科学缺件未变。新说明仅作者自验，不把本次独审扩成全部D12/I14、C6、产品或运行接受。未施工卡精确为02/03/04/06；05已部分施工并获得设计边界独审。原dsh机器组名与默认Python缺fontTools的复现差异分别在登记/复现说明中保留，不改原报告。


## P1-02场与科学职责增量（D13，FS/1 partial）

2026-11-14 P1-02设计交回：追加rev42，D13/I15/C6；提出FS/1场与科学接口设计，14组按实际来源和独立预期逐项划界，保留完整生成器、参数/许可、网格/拓扑/投影/盆地等11项具名缺口，02整卡partial，待用户手动转dsh。01/05原partial及真实独审范围不变，SG-01…SG-06开放；P1.1与P1.3 C4设计接受保持，C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。03/04/06未施工；无产品、新运行或新实验，交回后停止。

[逐组科学设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/design.md)固定逻辑场身份/物理量、core/closure/轴序/单步基底、分配前预算、候选与Engine唯一提交；原Field值描述和protocol3/113定义不改，SG的39新增定义原对象不改。本次新增25个FS设计类型，是独立profile，不是命令注册。codec解码、纯science、资源管理、渲染及宿主职责按[WC01–03适配](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/wordcraft-adaptation.md)分离，实际HEAD/文件SHA见本包来源记录；既定Rust/WASM单Worker、TS/React/Three和同核CLI路线保持。

完整生成器与有界子算子分开；A07/A08身份不是完整科学资格，A09/A10/A17缺件保留，显示概率A10不成为显式U散射的普遍前置。11项缺口与SG六项并存；poly-integral仅指定arm64 NoAction。源文件/新数值资源摘要分开，原错误、189个初始容差失败及独审范围原样保留。本卡未实施、未运行新实验，不推进P1接受。


## 当前接续：02同卡O1–O4（rev43）

2026-11-15 P1-02同卡O1–O4整改交回：追加rev43，D14/I16/C6，FS/1 evidence-r1；补35个真实投影合同有限例、澄清Patterson平方单位、固定Xcode3.9.6和11个旧WordCraft来源快照并披露2处漂移、将生命周期4个事件例移出准入证明。177定义/协议字节、11项本卡缺口及SG六缺口不改。dsh旧结论只覆盖rev42 partial设计/来源/保全边界，本轮整改增量仅Codex自验、待用户手动转dsh。01/05原partial与接受范围保持；C4接受/C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。03/04/06未施工，无产品或真实运行授权；交回后停止。

[本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/RECEIPT.md) / [整改说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/correction-notes.md) / [复现与漂移](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/reproduction-notes.md) / [下游边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/consumer-handoff.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/DSH-REVIEW-HANDOFF.md)。

本次授权为02同卡整改，沿用原会话/预算/权限/费用/次数/停报；不自动开新卡或扩大实现。投影helper只证明选定精确几何的schema/有限语义/资源域准入，未证明一般法向容差、插值/求积或完整科学能力。83例不含4个生命周期事件序列，7个output例仍是静态对象关系；租约释放、迟到原base/provenance及真实竞态均留未来04。复现使用新包reproduce.sh固定Xcode3.9.6，错误解释器exit2；WordCraft11旧Git blob锁SHA，当前两处来源漂移另记，不回滚外部仓库。原FS/1及dsh/旧复核/基线全部保全。


## 历史：P1-03 FC/1格式与资源边界（D15/rev44，partial被审范围）

2026-11-15 P1-03格式合同设计交回：追加rev44，D15/I17/C6，FC/1；85原路由身份保持，57条旧未审及28条旧政策完成有界来源字段审计，交291组源字段、逐profile候选选项/失败政策、损失同意与资源身份及85条未来真实内容验收合同。原4首链子集/81未准入保持，完整语法/模型桥、精度、布局/方言/许可和04宿主承诺未闭合，03整卡partial，仅Codex自验待用户手动转dsh。01/02/05原partial、FS11项/SG六项缺口、02rev43增量未独审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受不变。04/06未施工，无产品/新运行授权；交回后停止。

本轮精确合同见[FC/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/design.md)、[85profile](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/contracts/format-profiles.json)、[字段政策](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/contracts/codec-field-policies.json)。既有路线和分层不变，codec只解释已授权字节，Engine从同epoch/revision完整清单规划，页面/CLI薄消费。WordCraft实际HEAD52d17a91及12份参考字节/HEAD SHA见[结构取舍](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/wordcraft-adaptation.md)，不按活动上游更新Draft依赖。

原§4的`.vesta`+内容寻址资源和primary-last仍是提案，原件具体相对名读取与04宿主竞争承诺各自待资格；manifest存在不等于Saved或原件可读。完整SGGraph/非周期分子/场景到EncodeInventory的桥尚有具名缺件，遗漏未知语义层必须拒绝，不能因空字段表推断无损。PlanEncode无字节副作用；EncodeApproved只交候选owned Bundle；宿主真实读回和匹配ack才确认保存。

本卡不关闭SG六缺口、FS11项、B12原8unknown或B13两种证明。85路由有界审计不是全格式支持；全部产品未实现/未运行。04/06消费[相邻边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/consumer-handoff.md)，均需独立新卡授权，不自动开工。


## P1-03 O1–O7当前边界（D16，FC/1 evidence-r1）

2026-11-15 P1-03同卡O1–O7整改交回：追加rev45，D16/I18/C6，FC/1 evidence-r1。登记用户带回的rev44有界设计/来源/保全边界独审通过，原文6项与实列7项及411误写保留说明；当前整改增量仅Codex自验。291组明确为路由级字段清单，未建立字段到span/逐字段语义绑定；补3条读入路由的6份直接fixture及预期，I01改原生vesta；补RAW尺寸元数据来源，明确81候选shape及模板语义/精度未冻，损失枚举完整性未证，状态时间同步。原4子集/81未准入、SG六缺口/FS11项、01/02/05原partial、02rev43增量未独审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受保持。03仍partial，04/06未施工，无产品/新运行授权；交回后停止。

291是路由级字段清单数量，原rawspan字节检查不等于字段→span映射；没有291项独立语义或精度接受。候选profile只列shape，60项只有dialect；19种语义模板/1种精度模板和4条首链数字预算均不能充完整字段/参数资格。完整场景/分子/SGGraph桥、fieldloss完整枚举与资源/host承诺继续开放，不能从trustedPlan相等推出PlanEncode已穷尽源语义损失。

现消费[本轮合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/contracts/codec-field-policies.json)、[显式证明边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/proof-boundaries.json)及[缺口](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/remaining-issues.json)。原4子集、81未准入、WordCraft分层及既定产品路线不改。原rev44被审接受只针对路由级有界材料；本轮6份直接fixture是旧字节引用补强，未运行原件或writer。03仍partial、04/06未开工，消费者见[交接](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/consumer-handoff.md)。


## 03实质后继的架构约束（D17 / rev47）

完整、不可变Engine源快照先于codec约减清单。SGPhaseGraph每相id/parent/rotation/translationA/structure/fieldIds/shapeId、非周期分子/键、FieldHandle的owner和完整descriptor、SGVector的basis/kind/unit/displayScale、camera/外链/opaque各自作为源单元，不能塞进旧Structure后静默丢层。相同Resource摘要不合并两个Field。当前48个已选源属性处置和64叶有限演算见本轮model-bridge/source-model-mapping；高级DTO、非周期分子权威schema和完整capture原子性未齐，不声明桥全域冻结。

独立审计遍历原源/schema，未知扩展拒绝；对kept/dropped/rejected作完整分区，保留单元须与候选语义表示值相等，损失从原owner/id推导。该设计候选需新报告版本绑定sourceDigest/schema/profile-options/auditor/partition；不复用旧Consent声称已覆盖新字段，现有protocol3canonical不改。真实writer/宿主保存和恢复仍须各自后续资格。


## P1-03完整源与codec职责后继（D18，待独审）

[03整卡设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/design.md)统一消费当前源类型、codec、真实资源和保存结算。领域模型拥有当前状态与独立历史；纯codec只借用不可变源做计划/候选，不能做科学计算、改Engine、选择隐藏组件或写宿主；Engine掌握完整捕获/类型与引用归属、损失/Consent和typed恢复；宿主独占I/O、目标绑定、并发与持久确认；renderer提供冻结3D/2D及颜色/alpha/view定义，CLI/Agent复用同一Engine。

该分层消费实际核定的WordCraft architecture/testing/recovery/control/safe-save材料，来源及93bf14883c7a57a802f383cbcce880aae1328cc8 HEAD/所读工作区差异见570与wordcraft-resource-reference。完整源ledger和源独立预期、候选/实际产物、语义重开、精确Saved分别负责不同证明；任何活动对象缺失均不能由raw archive补账。现有平台提交模式可复用，Vesta适配和运行资格继续留后续施工。


## P1-04 宿主与应用职责后继（D19，partial待独审）

复用同服务队列、安全路径、基线和票据；明确任意外部写者check/use竞态、基本SpaceFiles无父目录sync及Sound特定协议不可套用。Vesta必须覆写基础mode换引擎、配额失败继续替换旧稿的行为，以layout-only与strict保稿合同接入。

保存先独立注册完整源预期，编码后才绑定实际产物，候选和真实持久内容分别验证。入队、Engine提交、Rendered、发布、commit-unknown、committed-unverified和快照Saved独立。目标按D4-31捕获，bindingIntent与publicationSequence防旧回执倒绑/倒退；只有全部语义及真实宿主证明才确认。首次Save、SaveAs、Export和关闭分别结算；query是拟增适配，不重编码/重写或凭SHA造Saved，主提交后不盲回滚/清理。

数据库是请求→候选→显式import，偏好12字段、Apply/Cancel、焦点/Undo/动画和HostAction回复各有AL/1合同。不造REST/provider，不把OS无能力写成功。参考WordCraft93bf1488的分层、safe-save和queued取消方法，不搬桌面恢复副本或原生RustUI限制。

完整设计、精确字段、来源与8项具名缺口见[04设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/design.md)、[HS/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/contracts/host-save.contract.json)、[AL/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/application-lifecycle.contract.json)、[源码绑定](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/host-source-bindings.json)、[remaining](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/remaining-issues.json)及[未来验收](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/verification-plan.md)。本轮仅Codex自验、待用户手动dsh；旧03/01/02/05/C6资格不升级。


## P1-04 同卡续办（D20，rev50，partial）

04同卡续办已补HS/2页面投递与恢复、AL/2真实表单及逐种系统动作设计。原8项均保留：4项缺设计输入，4项留待后续真实资格；整卡partial，待用户手动dsh。03 rev48整卡待审、01/02/05 partial、SG六/FS11、C4设计接受/C6未审、P1两接受两未接受保持。

页面消息按排队、已投递、已应用、取消请求、已证未开始取消、未知和终态分开；60秒超时/15秒claim窗口是当前PageChannel源码值，不能证明旧Engine停止。新claim不能撤销旧RAM提交；未知修改未解决时不启动同作品第二个可写Engine，旧页迟到结果只结历史。三视图切换只改布局。

保存操作必须捕获自己的目标基线；R8先发布后，旧R7不能因共享SoftwareFiles基线刷新到R8而覆盖。重启JSON/哈希链/0700均不能证明旧操作归属；缺可信原操作登记时只能明确以新生命周期恢复可核内容，不能回填旧Saved。元数据沿现有DSH设置，作品/候选仍由个人空间权限配额管辖；未新增服务或持久作品副本。

数据库参数以与R15完全同字节的四份POST表单描述绑定，偏好手册与S02原摘要相同；仅静态GET，没有提交查询/Stop实验。系统动作逐种payload及实际完成范围明确。HS-01宿主承诺、HS-02受保护凭据、AL-02原生偏好域/持久映射、AL-04原动画/历史分组输入仍缺；其余四项保留未来真实资格，不删功能或历史失败。[04续办回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/RECEIPT.md) / [续办设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/design.md) / [具体缺口](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/remaining-issues.json) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/DSH-REVIEW-HANDOFF.md)。


## P1-04 四项缺口续办（D21/rev51）

04四项缺口续办至rev51：HS/3宿主profile/私有恢复登记、AL/3偏好事务/动画历史规则已给具体设计；HS-02自身设计补齐，真实接线仍共享HS-01宿主资格。独立外部输入为HS-01、AL-02、AL-04三组，整卡partial待用户手动dsh。03 rev48待整卡独审、01/02/05 partial、SG六/FS11、C4设计接受/C6未审及P1两接受两未接受保持。

HS-01列齐所有写者、真实volume/profile与file/parent sync、每op原基线最后比较出版及具名失败；普通verify+rename不提供任意外改CAS。HS-02给出既有DSH私有path/host-only代际记录、崩溃查询、退休和配额/清理规则，不新增服务；实际隔离/卷资格仍须HS-01绑定资料，不能据作者设计宣称可信store已存在。

AL-02补既有原生load/save/Finalize及字体/历史持久字段组证据，精确UI域仍缺Ctor/OK/必要font函数。AL-04补既有PushHistory后缀丢弃、clone/append/总snapshot容量及完整tick/session/焦点/保存屏障规则；原生动画分组仍缺实际调用链。容量含current，100不等于100步Undo；历史裁剪保留current并报告损失，原件规则未证部分不假称兼容。

[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/RECEIPT.md) / [完整设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/design.md) / [最少补件](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/external-inputs.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/DSH-REVIEW-HANDOFF.md)。 原8项和四项未来运行资格全部保留；无新原件/网络/产品实验。


## P1-04 宿主来源与原生范围后继（D22/rev52）

04来源续办rev52：宿主输入已由BranchDraft源码/权威口径/历史真实Linux证据补成HS/4选定设计，纠正HS/3全外写者原子域前置；残余竞态和VESTA运行资格不消失。偏好/动画仍缺两组原生函数体；已有MouseLeftUp复用，新增方案首批6段、条件最多7段，当前未授权/未执行。整卡partial待手动dsh，未开06。

原卡只要求固定参与写者/已观察基线/剩余竞态，并不要求强制全局CAS。HS/4据现有个人space/dsh双卷、QuotaStorage实际limits、真实旧XFS/metadataguard红绿/Film隔离矩阵/Word选定服务测试，固定同协调器、每op原基线的保证。未参与外写者在check→rename与读回后的竞态明确未保护，不能宣称全局原子CAS或其间字节受保护；请求更强范围明确unsupported。

资源/主文件顺序、新目录边parent sync、HS/3私有registry/实际CLI隔离及完整03语义读回是后续实现增量，VESTA运行资格仍未验。宿主不再是用户须另交材料的独立设计缺口。AL-02/04指定原件函数体仍缺；旧nm只定位，旧LeftUp193指令复用且五virtual未知不抹平。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/RECEIPT.md) / [设计与来源](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/design.md) / [最小原件取证方案](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/DSH-REVIEW-HANDOFF.md)。


## P1-04 获授权静态证据后继（D23/rev53）

04静态续办rev53：首批6段各1次已取得并回写偏好控件/OKCancel与动画更新证据；条件7段未在窗口内排队，单次会话到时自动关闭，无重绑。宿主/恢复设计沿用，偏好完整输入域和动画历史分组仍partial；动态未授权，待用户手动dsh，未开06。

构造器绑定float/int与默认写回offset、两个spin声明上下界及步长、OK顺序/Cancel自身关闭路径已由实际静态体及已有spin体补入；原生有效文本域、字体/外层副作用仍未闭合。Timer直接修改当前Scene旋转，Idle有条件sort/repaint，虚调用未解；不能由无直接PushHistory推断原件分组。保留AL/3选定原子偏好事务和每接受样本历史规则，并明确它们是适配器设计。

取证单次arm64绑定、首批11108字节/2777指令，生命周期599.655496秒、每段<20秒，关闭后targets0、无GUI/额外模型、原件SHA不变。7段条件未尝试需新增绑定/时间额度；新暴露GetValue入口不在原13白名单，当前不读。HS/4/HS3及其它实际资格/失败继续保留。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/RECEIPT.md) / [设计与静态事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/design.md) / [最小剩余范围](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/DSH-REVIEW-HANDOFF.md)。


## P1-04 剩余静态补齐与输入后继（D24/rev54）

04补取续办rev54：剩余7段与必要GetValue596字节均各1次成功，准备后单绑定计时24.22秒即关闭，无GUI/额外模型。偏好有效值/字体/简单算式与旧值分支、intervali32到Timer及Undo/Redo入口已补；AL/4输入设计明确继任，实际转换与动画原件历史分组仍有更窄缺口，整卡partial待用户手动dsh。动态未授权，未开06。

GetDefaultParam读当前有效值而非factory reset；字体chooser仅5100分支更新草稿，外层不直接Set/写盘。GetValue先ToDouble再单次双数运算，分母abs严格大于2^-23，否则rawtype0/1/2路径取旧值候选，最后调SetValue。AL/4区分rawText、profile、候选与真实effective值，解析/转换未资格时E_PREF_DOMAIN_UNQUALIFIED，不允许client声称已通过；保持AL3全代持久/读回/安装和消费者屏障。实际间隔字段为i32，旧u64适配器线格式不是原生域证明。

Motion将pref+3a0传给循环Timer且避免重复Start；Undo/Redo各调一次库操作(int1)，成功才重建显示。raw模式UI映射、虚调用与原件历史归组未证，AL3每接受样本一事务仍明确标平台设计。GetValue尾SetValue348字节、候选模式入口OnRotate552及必要三槽24字节只列未授权下一最小范围，不追callee或启动动态。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/design.md) / [最小剩余](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/DSH-REVIEW-HANDOFF.md)。


## P1-04 数值存储与静态槽后继（D25/rev55）

04续办rev55：SetValue348/OnRotate552及必要三槽各8字节均1次成功，准备后单绑定18.78秒即关闭，无GUI/额外模型/其它函数跟进。已补实际数值转换、候选/显示/存储分离和Canvas聚焦/刷新/交换缓冲静态目标；OnRotate已证是固定轴按钮，不能代替持续动画模式。完整偏好UI资格及原件持续动画分组仍partial，待用户手动dsh，动态未授权、未开06。

SetValue的i32 FCVTZS向零、f32 FCVT、f64 STR storepoint先于格式化；Printf仍用原double。GetValue返回/显示文本不能代替实际typed有效值，AL4已补serverowned转换profile及持久后精确读回。实际FPCR/非有限/溢出/完整字段域、两格式常量bytes和回调效果未证，不宣称全域或猜格式。AL3原子偏好事务/消费者屏障沿用。

三处class槽与同SHA旧nm精确对应SetFocus/Refresh/SwapBuffers，无callee体/livevptr/全调用链证明；OnRotate199..204是固定轴旋转按钮，持续模式入口仍缺。保留AL3选定历史策略而不冒称原件归组。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/design.md) / [最小剩余输入](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/DSH-REVIEW-HANDOFF.md)。


## P1-04 格式/模式与选择历史作用（D26/rev56）

> 历史rev56正文：作用范围、profile字段、聚合与handler口径由文末rev57整改后继覆盖；旧包保持封存，不以本历史段作为当前合同。

04续办rev56：两格式常量和Mode0及三个必要条件体均各1次成功，准备后单绑定50.68秒关闭，无GUI/额外模型/其它函数。%g/%.*f、模式/轴/计时器启停和条件清选择后逐ownerDoc PushHistory2已补；AL4历史策略明确覆盖纯UI不记历史的泛化，逐Doc回执不冒称跨Doc原子性。完整偏好输入资格/实际动画UndoRedo分组仍partial，待用户手动dsh，动态未授权、未开06。

两原始格式已证，旧“未读字面量”只保留历史；Printf仍使用原候选double，typed存储/实际读回/effectiveVersion各自明确。Mode0/1/2直接分支给rawmode/axis/startstop，namedDrag/Click/Push/Random是旧AX/CUA/端点与静态关联推断，不冒充live索引、nativeenabled或tick计数。

ChangeManipulation raw1或>=4遍历ownerDoc集合，有效Doc reset_all_selected_objects返回>=1才PushHistory(2)。AL4给明确例外、集合scope/perDoc身份epochrevision权限/选择profile预检、逐Doc提交与partialunknown回执；不把泛化纯UI规则盖掉正证据，不捏造flags2mask/跨Doc原子性/实际动画分组。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/design.md) / [最小剩余证据](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/DSH-REVIEW-HANDOFF.md)。

## 04复核整改后继（rev57 / D27-I29-C6）

> 历史交付时状态；随后R57已独审通过F1–F6技术增量，登记及当前R1–R3修补以文末rev58为准。历史“待dsh”不表示本次技术增量仍未审；R3正文已按后继补清。

04按dsh复核完成F1–F6设计整改，rev57/D27-I29-C6；rev56静态证据独审通过，整改增量仅作者自验、待用户手动dsh。当前notebook全部已开文档的原生作用范围、逐Doc配置摘要、汇总优先级和事件处理器副作用已澄清；完整偏好输入资格与实际动画撤销重做分组仍缺，04整卡partial、未接受。

F1：原生身份链为canvas+0x568→WinManager*、manager+8→vxNotebook*、notebook+0x3c0/+0x3c8→本notebook全部已开文档向量；不推及其它notebook/窗口或账号。raw manipulation为1或>=4时，有效Doc当前Scene清选择返回>=1才逐Doc请求PushHistory(2)。适配器单Doc/明确授权集合是对原生范围的有意收窄。rev56“ownerDoc/ownerDocument”表述已由本段修订，冻结旧包和复审原件不改。

F2：`selectionProfileDigest`必须放在每个`capturedDocuments`条目，允许不同文档使用不同profile；请求级同名字段禁止。逐Doc权限/资格由服务器预检，不接收客户端permission/qualified声明。documentEnvelope、schema、闭合值、真实构造请求与有限例一致。[后继合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/contracts/history-evidence.contract.json) / [schema](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/contracts/manipulation-history.schema.json)。

F3：集合结果按顺序计算：任一unknown→unresolved；无unknown且failed+committed→partial；有failed但无committed（含failed+unchanged）→failed；全unchanged含空集合→unchanged；其余至少有committed、仅committed/unchanged→complete。逐Doc失败/unknown保留已提交结果，不宣称跨Doc原子性或自动整批回滚/重放。

F4：Mode0主选择0写raw0、隐藏次选择、停运行timer并Enable(1)；非0显示次选择并写raw=secondary+1，raw2停运行timer，其它已资格raw仅未运行时start(pref+3a0,false)，此分支不调用Enable。Modes1同样写raw=secondary+1，沿同timer规则且Enable(raw!=3)，raw1/raw2启用、raw3禁用；Modes2写axis selector并重置向量(1,0,0)。事件处理器副作用与适配器停止/屏障/不自动恢复策略分开；named映射仍为跨源推断，未测实际节拍或历史分组。

F5：[数值子集schema](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/contracts/preferences-input.schema.json)只覆盖animationStepDeg、animationIntervalMs、historyLimit、rasterTessellationFactor、isosurfaceN五项。其余textFont、三artifact、openTarget、startupBondSearch、hardwareAntialias七项各自沿AL3/05字体资产、宿主和能力资格，去向见[偏好合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/contracts/preferences-input.contract.json)；12字段矩阵不等于全字段wire准入或全原生资格。

F6：仅解码既存rev55 SetValue指令bytes：ADRP@0x100016e00+ADD@0x100016e04→0x1004d4f28（%.*f），ADRP@0x100016e28+ADD@0x100016e2c→0x1004d4f1c（%g），连同BL到Printf重载核对。候选double、typed存储、显示与实际生效读回保持分离；locale/FPCR/完整词法溢出和原件实际格式结果未验证。

本轮新增原件读取/绑定/GUI/动态/网络/额外模型均0，未运行会读原件的旧复审检查器。原复审含当前原件离线otool/nm与旧字节比较，不能笼统记为“未新读原件静态”；未target绑定/GUI/运行是另一边界。只追加补注，不改R或裁定权限违规，不重置累计预算/费用/次数/停报。原8问题ID、AL-02/AL-04、03rev48待整卡独审及旧47子项、01/02/05、SG六/FS11、C4接受/C6未审、P1两接受两未接受、06未启动均保持。未实现产品、未迁移177定义/productprotocol3、未自动转审。

[04整改回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/RECEIPT.md) / [整改设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/design.md) / [最小剩余输入](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/collection-plan.md) / [手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/DSH-REVIEW-HANDOFF.md) / [复审边界补注](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-045848-rev56-design-corrections/boundary-errata.md)。

## 04独审登记与R1–R3修补（rev58 / D28-I30-C6）

04本轮登记rev57 F1–F6技术整改独审通过（53/53），并完成R1–R3设计修补；当前rev58/D28-I30-C6，本轮增量仅作者自验、待用户手动dsh。04整卡仍partial、不接受，AL-02/AL-04未闭合，C6未审，未实现产品或启动运行/06。

登记范围仅为[rev57 dsh报告](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-053126-p1-04-rev57-dsh-review/REVIEW.md)的F1–F6技术整改增量：53/53，182例零失配、85种聚合组合一致，作者及复审411/0、七负控检出；原文correctionVerdict仍为pass-with-minor-doc-residual。未扩大为04整卡、整个接口、C6、其它P1或产品/运行接受。本轮R1–R3尚待用户手动dsh。原8问题ID、AL-02/AL-04、03rev48另案待整卡独审/旧47子项、01/02/05 partial、SG6/FS11、C4接受/C6未审、P1两接受两未接受、06未启动保持。

R1：当前摘要/导航统一到rev58/D28-I30-C6；原生范围为当前notebook全部已开文档，单Doc/明确授权集合是适配器有意收窄。STATE的P1.scope_clarification、P1.items.P1.2/P1.4及P1.bookkeeping_round已具名修正；历史rev56/57记录保留并标明由本节覆盖。新校验只放行这些精确字段及04当前指针/登记，不豁免其它P1字段、其它卡或user_decisions。

R2：新检查器按sorted(others)输出七项非数值排除检查，另核其它输出循环的顺序。相同冻结输入、final阶段及同一解释器用两个不同PYTHONHASHSEED实跑，逐字节stdout SHA/检查顺序/内容对照见[确定性记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-054254-rev57-review-registration/determinism.json)。不修改旧checker/stdout/seal。

F4：Mode0主选择0写raw0、隐藏次选择、停运行timer并Enable(1)；非0显示次选择并写raw=secondary+1，raw2停运行timer，其它已资格raw仅未运行时start(pref+3a0,false)，此分支不调用Enable。Modes1同样写raw=secondary+1，沿同timer规则且Enable(raw!=3)，raw1/raw2启用、raw3禁用；Modes2写axis selector并重置向量(1,0,0)。事件处理器副作用与适配器停止/屏障/不自动恢复策略分开；named映射仍为跨源推断，未测实际节拍或历史分组。

R3补注：rev57模型native_mode0已写secondary='show'，但rev57合同primaryNonzero并无显式secondary/show字段，R57称“合同与模型均覆盖”不够精确。本后继合同显式增加secondary='show'，正文同步Mode0非0显示及Modes1 secondary+1，并核模型/合同一致；只消费已保存证据，不新增原生行为或取证。R57原报告不改。

rev57 sealed189产物、18作者run及复审另加的runs/20261118-053226-180186-final一并保全；该复审run仍非原作者seal成员。rev56及两份dsh目录/旧失败/基线原字节保留；本轮新增原件读取、target绑定、GUI、动态、网络、额外模型均0，不运行会访问原件的旧R56/E56检查器，不重置预算/费用/累计次数/停报。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-054254-rev57-review-registration/RECEIPT.md) / [登记与R1–R3设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-054254-rev57-review-registration/design.md) / [手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-054254-rev57-review-registration/DSH-REVIEW-HANDOFF.md)。

## P1-06 当前实施依赖与结构接续（D29）

P1-06已完成已有输入下的实施计划设计并partial交回，待用户手动转dsh；rev60/D29-I30-C6。15单元具名依赖与真实内容验收已安排，SG六/FS11/资产外算/AL02和AL04仍限制完整冻结。03rev48整卡独审待发起；04rev57 F1–F6已审、rev58 R1–R3未审且整卡partial。P1.1/C4接受保持，C5条件/C6未审，P1.2/P1.4/整体partial；未启动产品或运行。

[模块计划](/Users/agilecho/code/Draft/docs/superpowers/plans/2026-11-10-vesta-modules.md)06节将原Task/G与31领域汇为15个用户/工程结果；[接口相容](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/interface-compatibility.md)锁实际生产消费、Source7/HS4/AL4与protocol3迁移边界。基础model/统一命令/codec候选→Engine commit→host实际I/O；TS Web/CLI共享Rust，WordCraft层次只参考职责与失败恢复，不迁移egui/本地profile/作品副本。SG/FS/P05/AL缺口按实际字段阻断，03格式设计完成不补上游科学资格；04HS01/02设计已source-bound，未来Linux/registry真实资格不包装成缺全局CAS。详细结果/文件/命令/预算/资源/用户链归现有模块计划；不新增公共类型或改I30/C6。

## P1-06步骤准入与同生命周期组合（D30）

P1-06同卡步骤准入设计交回，rev61/D30-I30-C6，待用户手动dsh。已区分源码、构建、运行资格与完整结果，首个Web保存导出重开续改方案明确；CLI同生命周期身份绑定缺口单独停受影响路径。SG六/FS11/AL02与AL04等原缺口保持，P1两接受两未接受、整体partial；未实现产品或执行运行。

[模块IP2](/Users/agilecho/code/Draft/docs/superpowers/plans/2026-11-10-vesta-modules.md)区分源码→实际构建→目标资格→完整用户结果，整单元关联不作全部源码总前置。首链R00 Web写读/销毁重开/续改可独立设计；R01 CLI需要[同context绑定](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-063853-step-admission/composition-gap.json)，不能复用另一one-shot process载入的IDs。当前I30/已有类型/protocol3不改，未定messages不以宽JSON或假函数填充；原完整功能与CLI要求不缩减。


## D31 · CLI同生命周期组合

原生进程只在真实关闭并释放页面owner后启动。单进程单Engine，实际open返回身份，typed inspect取对象，execute与snapshot沿用同一context；真实发布/独立全语义读回/私有journal仍由现有host承担。snapshot-ready不能Saved；EOF/restart拒旧frame并由HS3查原op。具体采用[I31 NS/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/native-session-design.md)，不新增守护进程、存储或认证服务，完整领域资格不变。


## 历史同卡CLI契约续办（rev63 / IP3；当前见rev64）

P1-06同卡CLI续作身份契约设计交回，rev63/D32-I31-C6。NS/1补齐一次serve载入后的实际身份、消息/资源/重试、候选保存与宿主确认、关闭恢复；仅首链有界设计作者完成，待用户手动dsh。15单元/56步骤保留，SG六/FS11/AL02与AL04/资产及运行资格保持；P1两接受两未接受、整体partial。未进入产品实现或运行。

[06本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/RECEIPT.md) / [CLI契约设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/native-session-design.md) / [手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/DSH-REVIEW-HANDOFF.md)。

设计与验收依据沿原06需求材料；[NS/1机器合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/contracts/native-session.contract.json)、[封闭schema](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/contracts/native-session.schema.json)、[生产消费和权限](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/producer-consumer.json)、[设计身份链](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/identity-trace.json)、[当前步骤图](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/step-plan.json)。P106-CLI-BINDING仅在NS/1首链范围作者设计补齐、未获独审或运行接受。I31明确取代I30跨进程inspect→run传runtime ID的错误续作配方，commands/inspect可保留诊断身份；完整254/31领域/85格式需求与SG/FS/AL/资产缺口不缩减。

历史rev60/61与原封存保持；文中旧“CLI合同未冻结”是当时缺口，不是当前NS/1状态。未来实现须迁移native/core/save/host严格profile生成及现有私有宿主driver，再单独获产品/运行授权、分配具体根/环境/预算/次数；本轮没有操作VESTA/产品/worker/Check/launcher/模型，没有自动发起dsh或新卡。累计费用/次数/停报继承原06，完整账本未核不记零总额。


### rev63上位输入对齐

封存前只读核验发现平台PRD/技术方案/阶段4决策新增D4-33；已核三份全文差异均仅插入新条目。平台软件后续目标由旧三模式改两布局，布局切换保留同一页面、工程、任务和草稿；本轮不重做UI，CLI仍仅真实受控关页后启动。旧三模式表述作为历史背景，不作为现行平台布局要求。新条目不改变本NS/1身份/宿主保存合同或开放产品/运行。首轮3项SHA失败与中间baseline62及其字节快照保留，当前追加rev63/D32-I31-C6对齐输入，NS/1 schema及核心类型未改。

[实际差异与输入SHA](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/input-drift.json)。

## 历史 P1-06 全功能设计交付（rev64 / D33-I32-C6 / IP4，存在复审漏项）

P1-06 全功能作者设计已完成，rev64 / D33-I32-C6 / IP4，整体待用户手动 dsh 独立复审。科学六域、FS十一项、偏好与动画历史、资产及四个外算流程已有具体类型、来源规则与完整输出先验；同生命周期 CLI 由保留的 NS/1 明确扩展为 NS/2。15 实施单元、56 依赖步骤覆盖原 254 功能和 85 格式路由。当前没有待补且会改变公共契约的来源/类型字段；未来产品与原件运行资格另列。P1 接受仍为两项接受、两项未接受，整体未接受；没有产品实现或运行。

[06 最终回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/RECEIPT.md) / [统一接口与 CLI](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/composition-design.md) / [完整实施计划](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/implementation-readiness.json) / [一次性手动 dsh 待审包](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/DSH-REVIEW-HANDOFF.md)。本节为当前同卡后继；下文/前文各 rev60–63 及更早的“当前、partial、尚缺”描述均保留为当时记录，原字节证据与旧 baseline 不改。新合同是明确的设计后继，不追认旧卡、旧 177 定义、bounded protocol3、NS/1 或 C6 已支持新字段。

用户 DEC-P1-05 已确认“先不管许可的事，先完成功能”：许可不再阻塞本轮功能设计；保留资产来源与条款身份，不推定已取得授权。当前只做设计，继续原卡累计预算、权限、费用、次数及停报；完整旧账本未核，不记零累计。原件执行、产品运行、worker、Check、launcher、模型调用均未发生。dsh 由用户手动发起，本轮不代发或建立复审会话。

D33职责：SG/2当前科学状态与不可变原始来源分离；FS/2完整生成/读入/采样/分析输出；唯一surface操作消费前置网格、实际相机、LUT、边界绘制，输出最终packet；EX/3只由有权限的宿主owned runner执行；偏好由现有认证owner持久化。Engine只准备具名typed结果及candidate、可信投影与history effect，不能通过业务参数获得shell/文件系统权力。七个消费方（Rust、Worker、EditorRender、CLI、HostAgentMCP、HostSave、CodecRestore）同步迁移schema/catalog/profile generation。正常相机动画默认保留已存历史cursor，真实live内容与saved digest独立；修改记录、Undo/Redo、资源和所有发布者关闭屏障分别明确。源缺陷保留具名失败或明确无操作；高维FFT布局、退化法向、不安全原生调色分支等没有伪称已运行。[完整职责和生命周期设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/composition-design.md)。

## 历史 P1-06 同卡复审整改（rev65 / D34-I33-C6 / IP5，仍有两项残留）

P1-06 rev64复审问题已同轮完成作者设计整改，当前 rev65 / D34-I33-C6 / IP5，待用户一次性手动dsh复审。NS2计划、九摘要派生、typed资源、31域预期、15单元证据与canonical文件已统一；rev64原封存保持。P1接受仍为两项接受、两项未接受、整体未接受。只做设计，真实运行/目标资格是后续施工产物，不作为本轮设计接受前置。

[06整改回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-124023-rev64-review-corrections/RECEIPT.md) / [I33/IP5整改设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-124023-rev64-review-corrections/DESIGN.md) / [当前实施计划](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-124023-rev64-review-corrections/implementation-readiness.json) / [一次性手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-124023-rev64-review-corrections/DSH-REVIEW-HANDOFF.md)。旧rev64“全部作者设计完成”的表述是当时交付结论；收到复审后已确认计划与接口接线漏项，本节替代其当前判断，保留原证据和原文作为历史。

当前精确输入见本轮 `contracts/profile-derivation.contract.json`（9个输入选择/CPJ1/SHA及固定向量）、`contracts/resource-bindings.contract.json`（6类资源及11完成类型）、`current-oracle-bindings.json`、`unit-result-assertions.json`、`planned-files.json`。NS2仍17外层操作/14顺序＋3带外、11内核命令，当前40科学/表面操作保持；新代际必须同步Rust/Worker/editor/CLI/HostAgentMCP/HostSave/CodecRestore。旧NS1仅明确的前代参考，不允许自动降级。

IP5保留15结果单元和56按产物依赖步骤，U00生成器及delivery CLI分支必须执行NS2/资源/摘要契约测试；U02完整终端不再因“缺NS1设计”受阻。31域的旧 `missingExpected/blockedBy` 仅在显式predecessor中，当前预期按完整字段/数组/资源绑定；shape-only示例不代替科学数值预期。15单元入口均指本轮文件并锁SHA，当前原生codec唯一计划文件为 `kernel/crates/codecs/src/formats/native_vesta_v2.rs`，高维模型在 `kernel/crates/model/src/native_dimension.rs`。所有产品文件/命令均待创建/待执行。

接受沿DEC-P1-03设计级口径：现有P1.1/C4接受保持，C5条件/C6未审及03/04旧整卡边界不追认。实际锁/构建/OS/ABI/GPU/IME/宿主持久化/外算/数值误差资格留给获授权的实施步骤；不能改为本轮设计验收必须先运行。DEC-P1-05许可后置保持，既有预算/累计费用/次数/停报不重置。只由用户转交dsh，本轮无产品/原件/worker/Check/launcher/模型、安装、部署、stage/commit/push。

## 历史：P1-06 两项残留同卡整改（rev66 / D35-I33-r1-C6 / IP6）

P1-06 rev65两项残留已同轮作者整改，当前 rev66 / D35-I33-r1-C6 / IP6，线协议摘要代际保持I33。两条原生路由handler统一v2，payloadEnvelope及代际来源已入唯一合同，生成/校验实际读取声明并新增反例；九摘要全部重算，仅保存profile摘要变化，另八项字节不变。rev64/rev65原封存保持。仅设计，接受仍[true,false,true,false,false]，待一次性手动dsh。

[06本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/RECEIPT.md) / [两项整改设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/DESIGN.md) / [摘要变更对照](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/profile-digest-delta.json) / [一次性手动dsh待审材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/DSH-REVIEW-HANDOFF.md)。

当前原生输入VM18-I01与输出OBS-O01的 `handlerPlanned` 均为 `kernel/crates/codecs/src/formats/native_vesta_v2.rs`；85路由绑定表是本轮后继，底层codec资料仍按SHA引用，不声称整套源文件复制入包。全85路由当前handler、两处部分漏改、其它路由误指旧名均有回归检查。

派生合同明确三字段信封、四字段输入记录、选择/投影/逻辑根替换顺序和唯一 `interfaceGeneration` 来源。文档修订I33-r1与wire/profile代际I33分开；所有9项按合同重算。单补信封不改变任何原向量；本轮两条handler改变仅saveBundleProfileDigest。若以后真正改代际，全部九项需重算/协商，不能沿用本轮“只变一项”结论。缺声明/代际冲突/记录缺项或多项/错序拒绝，禁止从脚本常量或样例反推。

本次只关闭F5-r/F3-r及对应回归检查，原科学模型/源/NS2消息/NS1/C6不改；15单元/56步骤/31域/40操作/254主责/85路由保持。原03/04及C5/C6被审范围不追认。DEC-P1-03仍按设计级复审，不新增真实运行前置；许可后置和原预算/累计费用次数/停报继承，不归零。产品/原件/worker/Check/launcher/外算/模型未运行，未安装部署或提交推送；仅由用户手动转dsh。

## rev67：rev66独审通过与P1设计阶段收口登记

rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。

这是接受状态后继，不改rev64／rev65／rev66封存。上文带历史版本号的partial、待审和未完成段落保留其当时范围；现行接受状态以文首登记和本节为准。未来实际构建、宿主及数值资格仍按获授权步骤验证，不反向变成本次设计接受前置。原累计预算／费用／次数和停报规则继承，不归零。
