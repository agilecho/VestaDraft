# P2-03 Source7 与 Graph2 有界续办提案

2026-11-21 · G26 后同卡接续 · 待批准

## 给你看的

剩余目标是让工程保存完整内容，包括分子、场、场景和各自的引用。工程关联的文件必须一并保存，包括嵌套引用和实际存在的伴生文件。验收时移开原始目录、结束旧进程，再仅凭保存产物重开、修改并重新读回。

G26 已在封存 C6 上完成真实过期拒绝、伪确认校验、四件旧工程迁移续改、OpenBundle、生命周期和 NS2 14/14，待手动独审。本轮复算 G26 的 170 项输入与 189 件封存。只读检查确认冻结格式允许 G23 的非空分子样本，且 G23 旧制品漏存两件嵌套文件。对缺少生产者上下文或资源归属证据的输入，按已接受规则保留源资料或明确拒绝。[G26 回执](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-g26-remainder/RECEIPT.md) · [本轮实核](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-source7-graph2-resume-proposal/preflight.json)

申请 **1 轮构建、2 组测试，合计最多 3000 秒**。同一构建产出最终 CLI 与测试制品，一组核完整类型、资源规则和拒绝分支，一组核实际文件、全新进程续改和受影响的保存会话回归。沿用已有 G23 行为反例，追加额度没有自动重试。

| 项目 | 当前实际 | 本提案获准后的上限 |
|---|---:|---:|
| 保守构建 | 6/6 | +1，C7 ≤1800 秒 |
| 测试 | 26/26 | +2，G27、G28 各 ≤600 秒 |
| 全卡 | 530.904372918 / 14400 秒 | 最坏累计 3530.904372918 秒，余 10869.095627082 秒 |
| 原执行包络 | 392.877044751 / 10800 秒 | 最坏累计 3392.877044751 秒，余 7407.122955249 秒 |
| 获取 / 锁解析 | 各 6/6 | 各 +0，沿用现有 pin、锁和私有缓存 |
| 网络 | 累计 97,135,307 字节 | 本轮接收 0，原上限 2 GiB |

当前仍为 partial、未接受，P2-04 未准入。批准对象是下列原生方案和具体用量。原范围、停止条件、失败账、资源上限和独审分工沿用。

## 给执行者看的

### 1. 入口与基线

- 遵循原卡、原生准入后继及 T66 rev66/D35/I33-r1/IP6 的 U01、U02、U14。已复算 T66 两份封存 244/228 件、三单元 53 项绑定和 Graph2 核心合同与伴随 schema 的封存副本。消费 FC type/audit、graph/reopen、content-comparison、artifact/settlement、GENERATION、SG/FS 及 NS2/RESOURCES2。用 FC 桥的精确 namespace 和 SHA。FS1 位于「app/contracts/source/」，纳入制品 pin。[原卡](03-晶体命令内核与原生保存基础.md) · [同卡派发](P2-03-G26后全类型与Graph2接续派发.md) · [绑定复算](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-source7-graph2-resume-proposal/preflight.json)
- 实际根「/Users/agilecho/code/VestaDraft」，分支「codex/p2-03-native」，HEAD「1fe1f0d16bc048b587a5f7fe83243821c38f6d50」。有效工作树含继承 WIP，G26 所 pin 源码匹配。Cargo.lock「ca96100d1edea152d5761b123873c32ebef1c2a5a7d9dcc003a345956b3c82ea」。正式执行前重新固定源码、锁、index 与单锁。G26 仍为 C6 的自验证据，旧 C5/C6 独审仅按原接受范围。
- 实读平台 SW-4/5/7、C4/C16、§9.6 和 §13.3。旧 BranchDraft 根缺席，DraftDuet HEAD 本轮有前进，实测 SHA 和收尾观察见预检及复核。两文档字节与原发卡观察相同，全文 SHA 与 T66 历史输入不同。继续按已核条文及 T66 办理同一引擎续改与受权文件范围。采用 WordCraft HEAD「685869239f31c890c6365f264c14b29ef48fcb7f」的引擎、宿主 I/O 与独立读回分层，原件和恢复副本各自处理。文件 SHA 和工作树字节比较见预检。沿用公共 craft 门面与现有 pin，Rust/WASM 与 React/Three.js 路线保持。[前次平台定位](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261120-preflight/platform-authority-check.json) · [本轮实核](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-source7-graph2-resume-proposal/preflight.json)
- 用原 native-attempt1/group.lock、target、私有 F6/L6 Cargo cache、Rust 1.99.1、Node 22.23.2、系统级禁网及 Cargo --offline --locked。单验证组、jobs≤2、子进程≤24、总 RSS≤8 GiB、累计新增磁盘≤16 GiB。准备、失败、超时、清场均按组和单调时钟记账。采用 G26 的历史目录一次计量，另持续计量各可变根（构建含 target）。启动前固定批准原文、argv、输入、监管器与沙箱摘要。原 P2-02 正式创建/安装、宿主 Saved/HS4、Web/GPU、P3 40 项科学计算及 P2-04 执行仍依各自边界。

### 2. 实施顺序、文件与输入判定

#### A. 按冻结定义生成持久类型和角色

- [ ] 在 model/src/source7/ 定型 Source7 根、全部可达族、变体和角色。model 拥有实际持久对象，codec 提取不可变快照。复用 source-derive/src/lib.rs 的 source_model! 生成 serde、audit、rebind、descriptor。候选 app/tools/generate-source7.py 使用标准库生成声明并校验完整覆盖，输入限指定 pin。在 C7 内运行 --check，任一根、属性、容器、变体漂移均校验失败。仅对 schema 结构无法表达的语义用 FC/SG/FS 的明文规则分类，禁止按字段名、字符串相等或测试 fixture 推断。独立基线有 22 根、89 个不同 (源文档, ref)、86 个目标、501 个属性 schema 位置、79 个组合分支位置。逐位置比较而非只比总数。还须消费桥及其契约另行注册的开放 AuditNode 变体，未注册/未资格采用允许的 opaque/source-only 或拒绝。
- [ ] 在 model/src/source_audit.rs 与生成器处理定义、引用、别名、owner、字节用途、跨度和字面值。Owner、SourceEntryRef 和 SceneNode 用 discriminator 定目标 kind，Bond 端点额外约束同帧。SG wrapper /id 与 /value/id 为同一个对象的双槽，只分配一次。RawSpan.sourceResourceId 与 byteOffset、byteLength、范围 SHA 对实际内容核验，重叠合法范围保留。原始序号、名称及与旧 ID 同字面的 label 不做重绑。既有 AUDIT1/StoredNode 已规定 TaggedVariant，现 Node::from_wire 和 restore_node 拒绝该形状，补判别、payload 递归角色和嵌套引用重绑，保留精确 bits 和顺序。非有限活动数值和可保留原始 IEEE 位各按合同。
- [ ] 取消 codecs/src/formats/source7_types.rs 的 15 个空占位及 current_source7.rs 的手拼外层。model/src/project.rs 的 ProjectContent/Capture、commands/src/{engine_v2,history,transaction,source_capture}.rs 捕获完整内核状态。内核事务提交经过验证的当前持久对象、不可变来源和独立 source-only 内容。codec 用该状态生成完整 SourceExpectation、辅助 ledger 和语义投影，读取独立还原全部成员与角色后才安装。active/source 共享一次身份分配与重绑，验证别名、资源、旧句柄和不同生命期。合法 phaseGraph:null 的仅分子/仅场工程有真实对象，不能造默认 P1；已有编辑命令更新当前对象，尚未资格的领域命令继续按现有 unsupported。新产物保留旧原始资料，重开不会把原资料再次应用到当前模型。
- [ ] **Field data use 缺口。** FS1 Field.resource 只有 digest/bytes，可同时命中两个逻辑资源。BoundDescriptorContext1 注册覆盖 descriptor use，其资格不能由二元推回当前 Field 数据资源。方案提出内部 FieldStorageUse1，通过 Engine 持有的 (Field ID, 当前 owner, 当前 typed use, Resource ID) 真正绑定，既有 SourceMetadata.tree 的闭合 TaggedVariant 及 typed BytesRef 随保存、参与规则摘要和资源闭包。FS1 Resource/Source7 根/NS2 shape 沿用。两个 Field 即使字节相同，实际 use 仍独立。导入缺可信归属或编码鉴定时走 source-only / E_REOPEN_COVERAGE。字段以 FS1 已资格的 little-endian、dtype、shape/closure/order/axis frame 等实际数据恢复，普通原件归档不足以称 Field 恢复。
- [ ] 角色依据仍缺时先记录精确 schema pointer、FC 规则及缺失声明。允许继续已明确根/类型、合法状态和拒绝分支。生成与完整恢复对未识别角色保持拒绝，保留其原 T66 义务。

#### B. 从受权读取创建 Graph2

- [ ] cli/src/paths.rs 提供受限根下的只读捕获，沿 cli/src/main.rs 的 run --preserve-source PATH 分派已资格 source profile。现有原始资料附件用途继续保留。所选 FEFF10 输入需要显式 producer job base/映射；建议在同一原生入口允许 --source-profile feff10-pinned-input --producer-job-root <受权根内路径>，作为读取请求。Paths 验证受权根、实际读取不可变字节并核 pinned profile/版本及适用证据，发进程私有捕获令牌并记录真实读取/探测。profile/扩展名或调用者的 job-root 声明不能单独认证 producer 和权限。NS2 OpenBundle 的字段沿用。
- [ ] codecs/src/formats/resource_graph.rs 实现 discover_graph(request: &CaptureRequest, reader: &mut impl CaptureReader, limits: CaptureLimits) -> Result<CapturedGraph> 与 verify_artifact_graph(graph: &CapturedGraph, blobs: &impl CapturedBytes, limits: CaptureLimits) -> Result<()>。model/src/source7/resource_graph.rs 拥有图、绑定和强类型输入，Paths 实现真实读取。结构导入按每个声明文件的目录，记录 exact path span。原始路径字面值与安全便携 key 独立，亲路径仅在实际授权根允许且解析成功时捕获。FEFF include/load 用受权 producer-job base，保留调用次数、顺序和共享解析状态；测试有子目录同名诱饵。GAMESS 同 stem producer-pair 是有限互相关联，科学 join 仍需独立证据。选定 RIETAN .ins/.lst 同 stem .ffe 确实探测。明确 NotFound 才记 absent，拒读、错类型、越界或坏件不能变成 absent，显式 IMPORT_ORFFE 必须存在且可读。reference-free 由冻结规则和实际字节的完整解析分区证明。未知 reference-bearing 路由须诊断。
- [ ] 依递归调用栈判定 E_GRAPH_CYCLE，非递归关联回边保留。累加 ParseLimits 的字节、record、node、grid、reference 与额外 depth/work，无子任务 reset，按真实输入和解码分配扣账。来源限制按原生 profile 取更严值并固定。逻辑边、owner、Resource ID 与物理 SHA 各自保留；同 blob 多次使用不合并语义。捕获和校验合格后单事务替换，拒绝分支保留原 draft、文件与资源。
- [ ] **可信捕获和制品承载缺口。** 现有 OpenBundle 缺受信采集上下文，闭合 SavedSourceDocument 无 graph 成员。方案用已有 context-resource 与 SourceMetadata.tree 的内部闭合 NativeGraphCapture2 注册，typed BytesRef 绑定 canonical Graph2、捕获策略/探测事实及虚拟名称映射，逐文件和用途以 typed Resource ID 重绑。传出时只留虚拟逻辑名称和已捕获事实，原目录读取权不随制品转移。composite resource digest 绑定 native Graph2 和 typed use 并集。artifact-only 从已绑定字节独立发现并核对每个条目、目标及保存期的 optional outcome；postpublish 用实际新字节与预登记 SourceExpectation 比较。上传或新进程读取的 hostSnapshotId 只是保存事实的一部分，不能当作新受信探测、producer 上下文或宿主权。若确实无法取得可信 producer mapping 或 lossless carrier，停其完整准入，报告精确缺件，继续已资格路径。沿用 D2-13 的文件系统 check/use 边界。FieldStorageUse1 与 NativeGraphCapture2 是本提案拟定的内部绑定，需按上述角色/规则/digest 明确登记，避免冒充冻结已实现合同。

#### C. 唯一 writer、兼容和封存

- [ ] codecs/src/formats/{current_source7,native_vesta_v2,schema_validation}.rs 消费完整 typed capture，保留既有外壳和 stamp 顺序。新 source/type/rule digest 参与比较；旧 Core1 走已有已钉 reader，C5/C6 Core2 按精确旧 type/rule digest 进入专用读迁移，保留旧完整内容和旧 SourceDocument，陌生 digest 拒绝。当前唯一 writer 写新产物。cli/src/session_v2.rs 的装载和 snapshot 串联相同 codec，save.confirm 契约和授权边界沿用。
- [ ] writer 写私有候选后独立验证，再发布及读回。保存的 expected 从真实编辑前/后业务输入与捕获字节得到，writer 返回的 manifest、生产 reader 输出及 driver 拼装的新工程均不能自任预期。最终 CLI 经真实操作写主文件和侧车，独立 reader 校验每个字节的 SHA/长度、完整内容/资源集合/逻辑身份及工程投影。按相同二进制封存，独立与最终各自的原始日志不能混计。

### 3. 独立 expected 与受影响回归

| 用例与入口 | 真实判据 |
|---|---|
| G27 model/tests/source7_roles.rs、现有 source_audit、定向编译门禁 | 将冻结 property/variant/alias 精确位置集与生成 output 对照，建立实际有值的合法族及变体夹具。FC 独立 molecule fixture 预期一个 C1、一个 ghost basis center、formal +1、partial +0.25、同位素只在 Species，分子无 Cell。另测同帧 Bond/跨帧拒绝、SG wrapper alias、跨 kind/悬空、合法 null/空、原始 span 越界与合法重叠、普通文本等于旧 ID。以 C7 rustc 连接同源产物，逐例验证漏访、未知变体、错角色及 skip/skip_if/flatten/rename/定制序列化/cfg/untagged 的政策诊断。 |
| G27 codecs/tests/source7_graph2.rs、native_roundtrip、source_contracts | G23 已有独立 shape 合法样本，所注入旧 ledger 仍需拒绝。新正例从实际 Engine 装载的受信 typed 输入调用生产 writer，实际非空族按 active/source-only 资格进入存读。原 8×8×8 pgrid 512 项小端 IEEE 为原始数据预期；合格 active Field 的语义和映射另证。合成两 Field 同物理 blob，coreShape=[2,2,2]，closure=[true,false,false]，x-fastest f32 值 [-0,1,-0, 2,3,2, 4,5,4, 6,7,6]。逐位核 signed zero 和闭合样本、两个不同逻辑 owner/use 与几何。调换同 SHA 绑定、缺 Field/section、错 closure 必须拒绝；临界预算和越额分别验。 |
| G27 commands/tests/engine.rs、CLI 单元与 cli/tests/session.rs | 真实晶胞、原子 batch、undo/redo、no-op、SaveTracker 四条确认拒绝同源复验。分别验 owner、authority、整数到期点、无私有权及原捕获状态。CLI 验真实注册/框架/帧。Paths 临时根验存在、明确缺席和读取错误，使用确定的拒读或非文件对照，保留实际证据。 |
| G28 driver.py + readback.py，固定 C7 | 实际 G23 三节点，outer decoy 排除，两个 span offset=75、length=15/10；散列见预检。FEFF 预期 job/main.inp→job/sub/atoms.inc→job/shared.inc，后一个可重复调用，子目录同名 decoy 排除。GAMESS 非递归有限回边、RIETAN optional 有/无/坏件、显式必需引用丢失、Graph2 递归环、错路径/owner/hash、额外/少节点与越限逐项。隔离原输入/缓存并销毁旧进程后，仅凭实际产物启新进程。编辑晶胞 6×4×4→7×4×4 Å 和位点 [0.625,0.25,0.125]，独立读回全非空内容和图，定义/引用/资源 fresh 且丢弃旧 ID 的操作被拒。 |
| G28 C7 最终兼容与确认 | C5 Core2 和旧 Core1 四件迁移/续改/重开及 raw 来源、OpenBundle 继续编辑、完整生命周期真实进程。读与写调用变化，故用 C7 复跑伪 HostAck（同上传 payload 独立 schema 校验）、错误目标、真实单调 ≥60 秒过期及状态/文件前后比较。完整 NS2 三文件 14/14、0 失败/跳过/取消。G26 的 C6 结果保留，C7 只报告自己的实测项。 |

G27 若用真实内核及 writer 提供已验证的含非空族合成输入给 G28，先独立核对并封存该**输入**。G28 的输出一律由 C7 CLI 实际编辑和保存产生。按 FC 合同给每项生成、覆盖、定义、引用、活跃/档案和产物检查填精确实际用例名与 expected。完整结果要求已明确的合法族和原范围都成立。

### 4. 真实命令与停点

候选绝对 argv、cwd 和每组 expected 见[命令矩阵](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-source7-graph2-resume-proposal/affected-tests.json)。未实现的目标为计划落点。受控执行前钉住实际命令和文件；C7 后用 Cargo JSON 的 executable/filenames 固定 G27/G28 制品、动态链接依赖与 SHA。

| 关口 | 顺序与结果 |
|---|---|
| C7 ≤1800 秒 | Python 生成器 --check、Cargo --offline --locked 定向 test --tests --no-run、同版本 build -p vestacraft-cli。封存生成表、实际源码/锁/编译/测试制品。 |
| G27 ≤600 秒 | 直接跑 C7 model、commands、codecs 和 CLI 的定向测试文件；对 C7 必要 rlib/宏制品用同一 rustc 跑编译门禁，不唤起第二次 Cargo 构建。保留非零选择、确切拒绝和所有原始输出。 |
| G28 ≤600 秒 | 实际 C7 CLI 驱动、独立 readback、同源 authority/legacy/lifecycle 回归和 Node 完整 NS2。留 trace、前后状态、产物及各自 SHA/退出码。 |

当前只有计划，先固定实际授权再运行。若候选源码、锁、制品变化，或者失败/超时需要新增组次，保留原始记录与仍可独立验收的结果，按原停点报告。产品最终完工回执与待审材料由用户手动转 dsh，整卡独审结论按原要求登记。

### 5. 可直接批准的范围

> 批准按《P2-03 Source7 与 Graph2 有界续办提案》继续同卡施工，追加保守构建 1 轮（6→7，C7 ≤1800 秒）及测试 2 组（26→28，G27、G28 各 ≤600 秒）。新增执行最多 3000 秒，同时计入原 14400 秒全卡总额与 10800 秒包络。新增获取、锁解析各 0 次，禁网，沿现有锁、私有 cache、target、单验证锁及其他上限。保留 G26、既有独审范围、原失败和封存。
