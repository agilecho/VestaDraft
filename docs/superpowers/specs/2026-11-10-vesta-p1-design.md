# VESTA P1 架构设计

2026-11-14。设计版本P1-D9。2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

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

单个资源失败：不写主文件、不ack；主文件失败：旧主文件仍指旧资源，新的孤立资源保留具名清理清单，不自动删除可能被别稿引用的文件。只有所有资源、metadata、主文件真实读回SHA匹配才返回Saved。60秒自动保存采用同一snapshot/ack协议，真正关闭先确认dirty快照已保存，失败阻止受控关页/交接；浏览器硬关闭不能保证最后一次异步写成，下一次只恢复已确认文件，不创建浏览器副本。

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


## 13 D8／R6 复审整改

2026-11-14 R6：D8/I10按dsh R5复审整改D1–D14及σ公式，Codex自验待用户手动转dsh。P1.1继承接受、P1.3 C4设计级接受保持，C6原字节及未审增量身份不变；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料rev31；不进入P2–P5、产品实现、部署或运行验证，真实worker／Check／launcher未授权、未验证。

I10§16以有身份的笛卡尔测量协方差明确J链和variance/σ，导出拆PlanEncode/EncodeApproved并绑定完整损失摘要；31领域允许32操作，旧内部step与旧oracle留作历史/辅助。每个结果有具名消费方，负/零张量分类与半空间输入域补完整。I8示例保留版本，新Field按schema再按语义验；112个定义均具名，17条语义例不冒称全科学实现。

85入口中57个逐字段审计仍未定义，先消费旧材料；B02不预设必须新实验，B12的8个codec问题、B13平台来源与B15应用来源具名。polyhedron关联B04/B09；指定版本NoAction能力设计不制造额外实验。具体修改/正反例/未验证见[R6](../../../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/README.md)。完整资格、20组完整来源映射及真实实现验证仍未完成，P1不推进。


## 14 D9／R7：交接勘误

2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

字段身份通过FieldHandle进入EncodeInventory与LossField.objectKind/objectId/相对指针，旧同意因新摘要重做；shape按选定凸边界几何投影，而不是多边形faces与triangles数组改名。I11§17和[R7契约](../../../output/vesta-study/20261108-113622/reviews/20261114-152228-p1-design-r7-errata/contracts/)给当前规则、12个身份例、10个提供Mesh的几何例及8相48面源适配。一般半空间分类/粉末拒绝套件/真实产品仍未证明，完整P1不前进。rev32纠正文书，旧rev31不改。
