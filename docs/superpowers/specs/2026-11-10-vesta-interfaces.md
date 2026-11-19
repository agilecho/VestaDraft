# VESTA P1 领域、命令、页面和保存契约

<!-- P1-CURRENT-STATUS BEGIN -->
当前状态：rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。
<!-- P1-CURRENT-STATUS END -->

2026-11-18，P1-I33-r1（rev66设计已独审通过，P1.2设计级接受；rev67仅登记，wire/profileGeneration仍I33，九摘要沿用rev66；C5条件/C6未审保持）。

R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

逐步引用：D06《方法/03-命令内核与多端架构》§1–6、《模板/命令与页面接口契约》全部；唯一functions.json的对应VF/studyHandoff；FORMATS原生工程/R85–97/S05收束，ALGORITHMS与S06_ORACLES选定规则；CLI S07合同、CLI_COMPLETION和MADEL_COMPLETION；行为01的Apply/Cancel/Undo、行为02的相/图像/磁矢量、行为03的场、行为04的科学/外算、行为05的保存/CLI。宿主语义以D02 §4–6为准。

## 1. 身份、修订和值类型

| 类型 | 线协议/生命周期 | 规则 |
|---|---|---|
| `SessionId`、`DocumentId`、`Epoch`、`RequestId`、`JobId`、`SnapshotId` | 32位小写hex字符串，128bit；由适配器注入IdSource，测试可固定 | session页面实例；每次载入或恢复产生新的document/epoch；不同语义不混用 |
| `ObjectId` | 同格式128bit，type由对象集合定义 | 创建后至删除/Undo/Redo期间不变，不用label/index/坐标作为ID；Undo恢复原ID，重做分支清理；新建同label仍新ID |
| `Revision` | 正则`0|[1-9][0-9]*`十进制字符串，u64范围 | 成功持久改变单调+1，Undo/Redo也+1；只读/无变更/preview不增；溢出拒绝。JS不转Number |
| `StateDigest`/`Sha256` | 64位小写hex | 内容/资源身份，不代替revision或文档ID |
| `WorkRef` | `{path:string, primarySha256:Sha256}` | 宿主认证后个人空间规范路径＋已确认主文件SHA；无存档时null。另存只有成功ack才重绑 |
| `ResourceId` | SHA256原始字节 | 同字节可共享；文件路径是位置，不作科学单位或资源对象的身份 |
| `ImageKey` | `{phase:ObjectId,site:ObjectId,op:ObjectId,shift:[i32,i32,i32]}` | 选定像可测量9Å或1Å；不能只存同标签或擅自最短像 |

ObjectId保证**一次载入生命周期内稳定**，跨纯原生`.vesta`重载允许重新分配，必须换document/epoch；旧工具选择器一律拒绝。原生格式无稳定ID字段的事实不伪造成已存在ID存储。若未来增加跨重开ID持久化，必须先核原件扩展/sidecar兼容和冲突，不在施工时暗加。作品同一性由真实WorkRef及保存/交接证据判断；同名文件不算同稿。

基础`Cell3`输入为`{lengthA:[f64;3],anglesDeg:[f64;3]}`；H列向量Å，角度对外degree、内部radian；`Fractional3`与`CartesianA3`分别包装，不能自由数组混用。新建合法cell要求长度正有限、角(0,180)、无量纲Gram因子1+2cosαcosβcosγ−cos²α−cos²β−cos²γ正有限，H各分量/det正有限（不引入统一epsilon）；参考异常输入通过`ImportDiagnostic`保留原词法/原始块，禁止把非法cell伪装成可计算Cell3。原件接受负晶胞是兼容导入事实，不改变合法计算域。

`Site3={id,label,species,frac:[f64;3],occupancy:f64,adp:Adp,uncertainty:null}`；`species`须保留元素/离子/同位素与raw token区别，缺失不能变成空字符串默认元素。`Adp=IsoU{uA2}|CifU{six:[f64;6]}|Beta{six:[f64;6]}|RawUnsupported{format,bytesRef}`；后三种的完整编辑/PSD/协方差政策未冻结。不得默默把beta当Cartesian U。`RawUnsupported`只能保留并报告阻塞，不得被计算器当已支持值。

`ProjectState`持有有序Phase、Scene、Comment、资源引用、导入诊断；Phase有`structure:Option<Periodic3>`,`fieldLayers:FieldLayer[]`,`shape:Option<Shape>`。`Scene`与`Periodic3`分开；`phasePlacement`和superspace/magnetic-general payload见§7，尚不能冻结成通用R/t。UI暂存控件文本不直接改ProjectState。

`FieldDescriptor={kind,unit,coreShape:[u32;3],closure:[bool;3],axisOrder:'x-fastest'|'z-fastest',originA:[f64;3],basisA:[[f64;3];3],periodic:[bool;3],dtype:'f32'|'f64',resource:ResourceId}`。kind/unit为显式注册组合：electronDensity/ePerA3、nuclearDensity/fmPerA3、electronPatterson/e2PerA3、nuclearPatterson/fm2PerA3、generic/declared-string；不是统一无量纲密度。JSON非有限值只能`{special:'NaN'|'+Infinity'|'-Infinity'}`出现在诊断/参考输出中；合法可计算payload禁止非有限。原始binary字段可保留原IEEE位，计算适用域另外判。

## 2. 事务、历史及数值完成

内核唯一入口计划为 `Engine::execute(request: CommandRequest, io: &mut dyn ResourcePort) -> CommandOutcome`；schema解析→身份/expectedRevision→能力/规则适用域→临时State+候选资源→不变量→一次commit→事件。文件/网络/进程/账号不进纯model；Rust ResourcePort只接收已授权字节和资源句柄，不接受任意宿主绝对路径。

每次commit产生`ChangeSet{before,after,digest,changedObjects,invalidated:[domain|scene|field|render]}`。以不可变资源+结构增量patch保存undo，不复制全部体素。history是当前内存会话；保存本身不进undo。Undo恢复内容但revision继续增，dirty比较当前contentDigest与lastSavedDigest，故Undo回保存内容可clean；资源按当前/undo/redo引用计数释放。历史内存达到传入Budget就明确裁剪最旧记录并报告，不无提示OOM。

人工拖动/预览：`beginDraft(document,epoch,expectedRevision)`→局部draft；`updateDraft`不改持久状态；`applyDraft(draftId)`将实际差异作为一事务；`cancelDraft`只丢未Apply的draft。Boundary Apply后Cancel保留；Properties每个字段按行为01 R103–132分类，不能把本通用draft默认回滚当所有原件Cancel。原件已证残留通过具名兼容命令在关闭时提交并给warning；原件崩溃映射`E_REFERENCE_FAILURE`不提交半成品，记录D02隔离差异。

`LegacyObserved`默认只在已证输入域执行具名策略；例如Bonds binary32阈值平方再升double、26邻点密度兼容导数、粉末首点0、原PDB列差异。返回`warnings:[{code:'W_REFERENCE_DEFECT',featureId,referenceVersion:'3.90.9a-macos-arm64',evidenceRef,ruleId,impact}]`。`Corrected{ruleId}`必须显式参数并有独立oracle；尚无该实现则`E_NOT_IMPLEMENTED`，不“自动修复”。模型内部f64不意味原件所有位精度已重建；每个输出按S06_ORACLES对应显示/文件精度检查。

## 3. 命令线协议与基础注册表

外层`{schema:'vestacraft.command/1',requestId,sessionId,documentId?,epoch?,expectedRevision?,command,params}`；未知字段一律schema拒绝。新建无document/epoch；已有文档的修改必须带document/epoch/expectedRevision。读取也必须document/epoch，支持`atRevision`时不自动替成最新。requestId去重键=(sessionId,documentId/新建域,epoch,requestId)，相同canonical参数返回同结果，不再commit；同key不同参数`E_IDEMPOTENCY_CONFLICT`。缓存只在session有效，断线先`request.status`；服务重启后已过期key返回`E_REQUEST_EXPIRED`，不能猜未执行。

结果：

- `Accepted{requestId,jobId,stage:'queued'}`只表示入队。
- `Committed{requestId,documentId,epoch,revision,stateDigest,changeSet,result,warnings}`表示内核已提交。
- `Rendered{documentId,epoch,revision,frameId,resourceDigest,viewport}`表示对应资源和frame可读。
- `Saved{snapshotId,purpose,output:{path,sha256},currentWorkRef:WorkRef|null,revision,stateDigest,files:[{path,sha256,bytes}]}`必须来自宿主实际写入/读回，独立于Committed/Rendered。
- `Failed{requestId,stage,code,message,details,retryable,committedRevision?:Revision,recovery}`；不能把提交后渲染失败表示内核回滚。

以下是首链所需的**计划命令**，目前没有任何已实现产品API；原16 CLI不自动转换成它们。所有数值必须有限；无说明的参数不默认接受。

| 命令 | 精确params/结果 | 语义、VF和规格 |
|---|---|---|
| `project.new` | `{cell:Cell3,label:string}` → `{documentId,epoch,revision:'0',phaseId}` | 空结构P1；不创建文件；VF-0007/0021/0022；行为01、02 |
| `project.inspect` | `{sections:('structure'|'scene'|'resources'|'history')[],cursor?:string,limit:u32}` → `{revision,digest,data,nextCursor?}` | limit1..1000；cursor绑定epoch/revision和section，过期返回E_STALE_CURSOR，不跨版本拼页 |
| `cell.set` | `{phaseId,cell:Cell3}` → ChangeSet | VF-0022；只首链P1，不隐含换setting/超胞；非平凡群先拒绝E_RULE_UNFROZEN |
| `site.add` | `{phaseId,label:string,species:{element:string},frac:[f64;3],occupancy:f64,uIsoA2:f64}` → `{siteId}` | VF-0023/0099；首链finite occ0..1、u>=0；复杂离子/ADP编辑不借此假支持 |
| `site.setFractional` | `{phaseId,siteId,frac:[f64;3]}` → ChangeSet | 不按当前选择猜对象，不默默wrap输入；展示periodic像派生；VF-0023的坐标字段；不把Symbol/New/Link等不同控件ID混当坐标编辑 |
| `display.setHidden` | `{images:ImageKey[],hidden:boolean}` → ChangeSet | DLATM显示状态；不删除site；VF-0225；行为01隐藏恢复 |
| `boundary.set` | `{phaseId,min:[f64;3],max:[f64;3]}` → ChangeSet | 首链min≤max、范围预算；VF-0061/0181；Apply/Cancel独立 |
| `history.undo`/`history.redo` | `{}` → ChangeSet | VF-0019/0020；无历史E_NO_HISTORY，batch作为1条历史 |
| `view.setCamera` | `{rotation:[[f64;3];3],projection:'parallel'|'perspective',zoom:f64}` → ChangeSet | 正交旋转/zoom>0；VF-0030..0034/0062；完整width/legacy相机变换另冻结 |
| `project.loadBytes` | `{primary:ResourceId,format:'vesta'|'cif',resources:[{logicalPath:string,resource:ResourceId}],intent:'replace'|'appendPhase'}` → `{documentId,epoch,revision:'0',diagnostics,idMap}` | 字节先由ResourcePort注册；replace新epoch，append经事务但完整相位定位未冻结故首链拒绝append；VF-0009；旧稿失败不丢 |
| `file.snapshot` | `SnapshotOptions`（§10.2 的封闭联合）→ SaveBundle | 只导出bytes不写磁盘；VF-0012/0013仅原生保存子集、VF-0014仅CIF导出子集、VF-0015仅PNG导出子集；VF-0016矢量导出尚未冻结、不在此首链注册；PNG须对应Rendered，导出不重绑 |
| `request.status` | `{requestId:string}` → 缓存stage或E_REQUEST_EXPIRED | 只查调用者同session；不新建执行 |
| `job.cancel` | `{jobId}` → `{status:'cancelled'|'tooLate'|'alreadyFinished',committedRevision?}` | 取消结果不是自动撤销 |

`command_list{query?:string,cursor?:string,limit:1..1000}`返回注册表**已实现**命令的全schema、effects、VF/spec refs、completion、batchability/cancellation；生成于Rust注册表，TS类型/CLI帮助不得手抄成两套。`E_NOT_IMPLEMENTED`不能在已实现清单中伪装成功。

### 3.1 batch、取消、并发和错误

`command_batch{mode:'atomic'|'sequential',commands:[{command,params}],onError:'stop'|'continue'}`；非空、最多256个是本设计协议上限而非性能实测。atomic只允许同document的纯内存同步编辑，要求onError=stop，临时状态顺序执行、一次修订/undo；遇到load/save/export/网络/外算拒绝`E_NONATOMIC_EFFECT`，不留下前半。sequential逐条结果，stop/continue明确已提交范围，每项各自revision，文件命令独立Saved；不叫原子。

长算返回JobId，输入绑定epoch/revision/resourceDigest；工作在临时资源中，可分块检查取消。commit前取消释放临时资源、revision不变；commit后cancel返回tooLate，撤销走history。读文件、原生外算、渲染、宿主save分别有取消点；主文件rename以后不能承诺取消未发生，保存超时先查snapshot/真实文件，禁止直接重放旧快照覆盖新稿。

两个修改共享同一revision竞争时先提交者成功，后者E_REVISION_CONFLICT并返回currentRevision；不自动合并。无共享内存WASM内单job运行时仍要检查epoch；Web Worker崩溃从最后committed内存检查点恢复到新epoch，未提交draft丢弃并提示；无浏览器持久副本。若整个页面丢失只能恢复最后宿主确认工程。

错误集合固定：`E_SCHEMA`、`E_DOCUMENT_NOT_FOUND`、`E_STALE_EPOCH`、`E_OBJECT_NOT_FOUND`、`E_REVISION_CONFLICT`、`E_STALE_CURSOR`、`E_IDEMPOTENCY_CONFLICT`、`E_REQUEST_EXPIRED`、`E_NO_HISTORY`、`E_DOMAIN`、`E_RESOURCE_MISSING`、`E_RESOURCE_LIMIT`、`E_FORMAT_UNSUPPORTED`、`E_FORMAT_LOSS`、`E_RULE_UNFROZEN`、`E_NOT_IMPLEMENTED`、`E_NONATOMIC_EFFECT`、`E_CANCELLED`、`E_RENDER_FAILED`、`E_EXTERNAL_CHANGE`、`E_READ_ONLY`、`E_QUOTA`、`E_IO`、`E_TIMEOUT`、`E_REFERENCE_FAILURE`、`E_TARGET_TOKEN_EXPIRED`（host-only）、`E_TARGET_TOKEN_INVALID`（host-only）。details仅具体对象/字段/单位/阶段/已提交范围，不泄漏个人空间以外路径或秘密。retryable不能替代状态查询；E_IO/E_TIMEOUT未知保存状态须先核宿主，不盲重试。

## 4. Worker、页面及渲染邻接接口

计划Rust导出`KernelSession::new(id_seed,budget_json)`、`execute(request_json)->outcome_json`、`inspect(query_json)->reply_json`、`put_resource(bytes)->ResourceId`、`get_resource(id)->bytes`、`release_resource(id)`、`advance(work_units:u32)->JobEvent[]`；bindings只负责序列化，不重复业务。`Budget={maxInputBytes:u64,maxResidentBytes:u64,maxFieldValues:u64,maxMeshVertices:u64,maxOutputBytes:u64}`全部显式注入；生产数值待G-WEB/G-UX测量，不能用测试预算冒充交付预算。

Worker外层`{protocol:1,sessionId,epoch,requestId,op:'execute'|'inspect'|'put'|'get'|'release'|'advance',payload}`，大资源用transferable ArrayBuffer；所有权转移后发送方失效，不保存detached view；每个resource lease有retain/release。主线程不读WASM裸内存view跨grow；取出时复制到owned Uint8Array再transfer，render只持自己lease。取消通过消息触发分块job标志；同步长期WASM不能在主线程强停，紧急terminate需从committed checkpoint新epoch重建，不能称命令已回滚。

计划页面唯一公开对象`VestaPageV1`：

- `commands(query)->Promise<CommandPage>`，`execute(request)->Promise<Committed|Accepted|Failed>`，`inspect(query)->Promise<Inspection>`；均转Worker。
- `open(bundle,expectedPageGeneration)->Promise<Loaded>`：只有实际解析+资源验证后Loaded，不以postMessage入队作打开完成；旧稿在加载成功前保留。
- `render({documentId,epoch,revision,viewport,signal})->Promise<Rendered>`：等待该revision的场景派生、字体、纹理和GPU提交完成；请求已过期则E_REVISION_CONFLICT，不截图旧frame当目标frame。
- `snapshot(options)->Promise<SaveBundle>`，`confirmSave(HostAck)->Promise<SaveState>`、`rejectSave(HostNack)`；只有host认证适配器可发ack，业务Agent不能自行构造ack清dirty。
- `status()->{lifecycle,documentId,epoch,revision,dirty,workRef,lastRendered,activeJobs,capabilities}`；`dispose()`释放renderer/Worker/监听器，拒绝旧Promise。

Renderer输入`RenderPacket{documentId,epoch,revision,resourceDigest,instances,meshes,labels,camera}`，GPU是可再生缓存；科学网格/晶胞留内核。渲染成功不是保存成功。被隐藏的页面仍持续拥有engine，rAF可能被节流；render有界等待，超时报告“领域已提交/尚未渲染”，不能伪造成功。GPU context lost后invalid frame，重建资源后重新发Rendered；Canvas中文标签等待字体ready，无字体报具体失败。目标browser/iframe/CSP实际验证见G-WEB。

## 5. 多资源SaveBundle、真实ack与恢复

计划格式：

```text
SaveBundle = {
 snapshotId, sessionId, documentId, epoch, revision, stateDigest,
 purpose: save | saveAs | export,
 targetToken, expectedPrimarySha256: Sha256 | null,
 primary: {bytesResource: ResourceId, sha256, byteLength, format:'vesta'|'cif'|'png'},
 dependencies: [{relativePath, bytesResource, sha256, byteLength}],
 manifest: {primarySha256, dependencies:[{relativePath,sha256,byteLength}]}
}
```

targetToken由现有平台命名/路径服务产生，内核没有裸个人空间路径写权限。依赖路径必须相对、无`..`/绝对/重复/大小写别名/符号链接；所有资源由host真实安全根解析。导入源/工程binding/另存target/导出copy四种角色独立。打开原生`.vesta`时先收齐所需资源再replace；缺失资源给显式诊断/可选择只读保留，不能假装完整工程Loaded。

**计划存储布局**：主文件`作品.vesta`；资源`作品.vesta.resources/blobs/<sha256>.<真实格式后缀>`；摘要manifest `作品.vesta.resources/manifests/<primarySha256>.json`。manifest只描述资源及完整性，不声称原生格式已存对象ID/修订。每次load重新建立document/epoch；后续跨加载稳定ID/provenance扩展单独设计，不是当前承诺。原件是否接受具体相对引用须G-CODEC对照后才能冻结导出；如果该writer输出不被原件读取，不报原生兼容通过。

Host算法使用既有认证、配额、路径与serial，不另建存储服务：

1. 认证targetToken、owner lease、document/epoch和purpose；纯文件计划进入当前协调器同一串行操作。取得snapshot后允许UI继续编辑。
2. 对primary旧SHA和所有实际写目标跑SoftwareFiles基线verify；资源hash路径已存在时必须读回相同SHA，否则E_EXTERNAL_CHANGE。配额检查走现有平台服务；没有多文件预留接口就逐写失败保留旧主文件，不声称预留成功。
3. 顺序写不可变资源并读回SHA；写manifest并读回；任一失败停止，不触碰primary。生成本轮孤立资源清单供后续可审清理，不自动删旧资源。
4. 主文件提交前再次verify primary基线，使用SpaceFiles安全temp+sync+rename。现有verify/write不是任意外部并发CAS，G-HOST需冻结共享写者的实际串行保证/剩余竞争边界。
5. 读回主SHA与全部依赖，并完成03独立artifact-only对象、owner、全数值和来源验证；主提交和语义凭据绑定同一操作后才返回HostAck。若主已提交但读回/连接失败，状态是`indeterminate`，下一次按快照SHA+依赖复核，而不是返回已保存或直接回滚新文件。
6. ack携带`{snapshotId,documentId,epoch,revision,stateDigest,purpose,targetToken,actualPrimarySha256,verifiedFiles}`；页面匹配原快照，保存的是R7而当前R8则lastSaved=R7、dirty=true。切到新稿epoch后旧ack只结清原快照，不能动新稿。saveAs仅匹配同稿的成功ack后重绑目标，export永不重绑/清dirty。

单文件（无外部资源）同样走主文件真实确认及60秒自动保存。多文件自动保存仍用同一协议；避免把“单文件60秒已确认”扩大成实测大资源每60秒均可完成。快照进行中周期触发合并为最新待存revision，不重复排旧快照。受控关闭：阻止新编辑→等待当前job提交/取消→snapshot最新revision→Saved→release/交接；保存失败恢复页面编辑权/dirty，不假关页。硬关闭/进程崩溃后，只能确认已写主文件/资源，不保证未保存修改恢复。

具体失败断言（未来真实测试，不是结果）：R7 snapshot后site编辑为R8，ackR7不能clear dirty；A稿ack晚于B稿load，B状态完全不变；资源2写507时primary旧SHA不变；primary409时已写资源仅为orphan；saveAs失败path不变；exportPNG成功工程path/revision/dirty不变；丢ack须查同一宿主操作及真实文件和完整语义；SHA匹配仍不足，缺操作或语义凭据保持commit-unknown/committed-unverified及dirty。

## 6. CLI/外算与四种完成时刻

计划产品原生CLI为`vestacraft`，不是原件VESTA命令包装；下列为I30历史示例，身份敏感续作以文末I31 NS/1为准：

```text
vestacraft commands --json
vestacraft run --input <authorized .vesta> --request <command.json> --output <authorized .vesta> --json
vestacraft inspect --input <authorized .vesta> --json
vestacraft serve --stdio --root <platform-mount>
```

这些命令尚不存在/未运行。run成功须处理所有请求并真实写入/核验SaveBundle，exit0；schema/domain/format失败exit2；I/O/外算失败exit3；取消exit4；内部异常exit70。stdout只有结构化协议，stderr诊断；原件255和sender0只留在兼容对照，不作为产品成功策略。stdio无头模式不依赖显示；PNG等需要真实renderer时通过独立浏览器worker，缺环境E_RENDER_FAILED，不伪造截图。`--root`不是“传了就安全”，必须服务端挂载/路径守卫实际强制；本机CLI是否真正执行这些安全约束仍未验证。

外算计划接口`ExternalJobSpec{kind:'madel'|'tidy'|'rietan'|'orffe',inputResources,options,documentId,epoch,baseRevision,budget,toolArtifactDigest}` → `ExternalResult{jobId,exit,signal,timedOut,outputs,semanticChecks,provenance}`。options各工具的科学schema未冻结前不可公开；不以自由JSON留施工猜。worker只运行已获资格的具名artifact/argv，无shell拼接；每job独立cwd、完整watch和owned process group；完成需要退出＋输出独立解析＋科学适用检查，退出0不等于结果成功。取消SIGTERM→有界grace→SIGKILL并确认树清理；输出仍是候选资源，epoch/revision不匹配不得提交。

队列accepted、内核committed、画面rendered、宿主saved分别可读。典型工具command_run若同步修改，应返回Committed及可请求render token；平台要求当前可见时继续等待Rendered再报可见，保存类等待Saved。超时必须带已达到的stage，避免重复计算/提交/写入。

## 7. 完整资格尚未冻结的相邻接口（I10有界类型见§16）

以下真实缺口仍阻止“全部领域/程序接口已冻结”的P1.2完成声明。基础层可以先设计和验证，但本阶段不自动实施。

- `PhasePlacement`：参考相图、一般非正交R/t、循环/单位/存储精度及错误后状态；S06 phase组与VS18只有选定八相形貌，不替通用模型。
- `SymmetrySet/ModulationModel`：一般/磁群表版本及许可、setting、特殊位容差、超空间维度/调制项、向量基底；不能用3×3 op宣称全部功能已覆盖。
- `ScientificRequest/Result`的全参数：ADP概率椭球、键价选表、反常/同位素、FFT网格/Nyquist、等值面退化/封口、投影、密度/峰盆地；每组准确unknown/nextProbe在S06_DOMAIN_RULES。
- 全codec字段/保留与writer策略：m3d/PRI/DEN/STO等未解字段、已证失败的原格式、图像透明/色彩/字体与全部选项；FORMATS/S05原始合同不能被本接口的vesta/cif首链子集替代。
- host不可变资源路径的原件读取兼容、超额/外改竞争承诺、已提交读回失败的真实恢复路径须按G-HOST验证；旧平台没有现成全工程事务。

I10§16已补可由既有资料确定的有界类型、语义和模块交接；下列完整原件映射/资格仍需新材料或独立判据。`RawUnsupported`、E_RULE_UNFROZEN和partial计划是诚实边界，不能用来宣称全功能已实现或把缺口删除。

## 8. I2基础邻接的精确补正

本节覆盖前文不够具体处；依旧只冻结基础切片，不把高级§7公共payload洗成完成。首轮dsh没有最终裁定，本节为Codex核实后的补正。

### 8.1 首链模型的最小完整形状

```text
Species = {element:string|null, ionicCharge:i8|null, isotopeMassNumber:u16|null, rawToken:string}
CorePeriodic3 = {cell:Cell3, symmetry:{kind:'P1'}, sites:Site3[]}
CorePhase = {id:ObjectId, label:string, structure:CorePeriodic3}
CoreCamera = {rotation:[[f64;3];3], projection:'parallel'|'perspective', zoom:f64}
CoreScene = {camera:CoreCamera, boundary:{phaseId,min:[f64;3],max:[f64;3]}, hidden:ImageKey[], style:'ball-and-stick'}
RawBlock = {source:ResourceId, byteStart:u64, byteLength:u64, kind:string}
CoreContent = {phases:CorePhase[], scene:CoreScene, comment:string, preserved:RawBlock[]}
ProjectState = {documentId,epoch,revision,content:CoreContent,contentDigest:StateDigest}
RuntimeState = {project:ProjectState,workRef:WorkRef|null,lastSavedDigest:StateDigest|null,history:HistoryState,jobs:JobState[]}
```

首链仅一个P1结构相。RawBlock只支持无损保留，不授权计算未知层；有field/shape/非P1等导入先只读保留/返回E_RULE_UNFROZEN，不套CorePeriodic3伪支持。后续必须先冻结完整Phase/Periodic3/Scene及迁移版本，Task2只消费本节。Site3.uncertainty首链只能null；已有不确定度必须保留原始块并拒绝有损重存，不能猜Option<...>科学实现。

### 8.2 资源、调度与批处理结果

ResourcePort是已授权内存资源表，不读宿主路径；`read`/`resolve` 继承自 `ResourceReader`（§10.1），自身计划方法为 `put(&mut self,bytes:Vec<u8>)->Result<ResourceId,ResourceError>`、`retain(&mut self,id:&ResourceId)->Result<(),ResourceError>`、`release(&mut self,id:&ResourceId)->Result<(),ResourceError>`、`bind_logical_path(&mut self,path:&str,id:&ResourceId)->Result<(),ResourceError>`。put以SHA-256校验/去重并先检查Budget；read借用不越过mutate/grow；临时job、提交state、history、renderer、snapshot分别retain，全部release后释放。错误Missing/Limit/DigestMismatch/InvalidLease。SHA库精确版本见R/dependency-protocol-supplement.json，尚未编译集成。

新增`Engine::advance(work_units:u32,io:&mut dyn ResourcePort)->Vec<JobEvent>`；`JobEvent=Progress{jobId,completed,total}|Done{jobId,outcome}|Failed{jobId,error}`。Accepted建立job，Worker交替advance有界work_units与yield处理消息，CLI驱动同一advance；不让一次同步WASM吞掉cancel。work_units为算法工作量上限，不是已测毫秒。commit在一个serial turn中复核epoch/revision。snapshot/encode长任务也走job，不从Accepted读不存在的bytes。

`BatchReply={mode,documentId,epoch,startRevision,endRevision,results:[{index,requestId,stage:'committed'|'failed'|'skipped',revision?,result?,error?}],rolledBack:boolean}`。atomic失败时丢所有暂存，失败项failed，其余项skipped，rolledBack=true/endRevision=startRevision；成功一次revision，所有结果引用同revision。sequential各项真实revision，stop后的项skipped、continue全部尝试，rolledBack=false；I/O不得进atomic。

### 8.3 保存桥的字节、lease与输出身份

SaveBundle只在内核/页面用ResourceId，不能直接交SpaceFiles。计划页面adapter `materializeBundle(bundle:SaveBundle):Promise<HostBundle>`：retain每个resource，取owned bytes，核SHA/长度，再转现有PageChannel/client接受的base64文件字段；base64不等于保存成功。`HostBundle={...SaveBundle,primary:{format,sha256,byteLength,bytesBase64},dependencies:[{relativePath,sha256,byteLength,bytesBase64}],manifestBytesBase64}`。宿主校验canonical base64/解码长度/SHA后走既有安全写入；不新增运输、身份或计费。既有大资源传输上限/分页能力仍属G-WEB/G-HOST；超限E_RESOURCE_LIMIT，不假称流式已成立。

snapshot retain至host匹配成功/失败/取消且无in-flight读取后释放；dirty内容不因snapshot释放而删除。页面dispose释放本实例临时resource；已传host但未确认的写入按snapshot查询真实SHA，不把端口关闭当写入失败。ResourceId session内有效，不能跨CLI进程搬裸句柄。

save/saveAs只接受format=vesta，CIF/PNG只走export；Saved.output为实际产物回执，Saved.currentWorkRef为当前工程绑定，export前后不变。HostAck保留purpose/targetToken，导出output不能冒充当前WorkRef。

同SHA blob已存在：先SoftwareFiles.read核原SHA，再observed建立该blob路径baseline并复核；相同则复用不写，不同E_EXTERNAL_CHANGE；不存在才通过write创建/读回。不能用baseline=null直接verify既存blob误报冲突。primary必须保留打开/最近确认baseline，不以observed抹外改。

manifest记录完整依赖digest；仅当primary所有引用均为指定content-addressed路径时可按primarySha命名。若某writer须保留可变原名，整个bundle先E_RULE_UNFROZEN，冻结复合manifest身份/提交协议后才能支持，不以同primarySha覆盖不同manifest。原件对hash相对路径的兼容仍未测。

## 9. 正式dsh复审后的基础补正（I3）

第二轮dsh正式裁定P1.1接受、P1.2/.3/.4未完成，整体partial；本节为Codex处理文档发现，不能据此改成整体通过。

### 9.1 科学字段及首链范围

首链Site3.uncertainty类型精确为null；任何有不确定度的输入必须保留原始段、标为未冻结编辑域，不能悄悄当零或声称已支持通用协方差。完整Uncertainty公共payload仍属G-SCI。Species.isotopeMassNumber为**质量数A，无量纲正整数u16**，null表示没有显式指定同位素；它不是原子质量u，后者计算必须使用有资格的版本化参数表，不从A推定精确质量。

file.snapshot首链仅vesta保存/另存、CIF数据导出、PNG栅格导出，分别关联VF-0012/0013、VF-0014、VF-0015的选定子集；不表示这些VF的所有格式/选项已规划完毕。VF-0016矢量导出及其它格式留G4，尚无可公开注册的实现。

### 9.2 targetToken签发、绑定与SaveState

计划在现有认证的/api/software运输和同一VestaCoordinator串行顺序内增加prepareSave操作，P4才实现；**不是现有平台已支持的接口**。内部签名：
`prepareSave(input:PrepareSave & {documentId,epoch}):Promise<TargetGrant>`，其中 `PrepareSave` 为 §10.2 的封闭联合，不再平铺 purpose/format/targetIntent。
它用现有本人空间/path/newPath/newExportPath/只读/配额入口解析真实目标，不信任页面自报用户/空间。TargetGrant线协议为`{targetToken:string,documentId,epoch,purpose,format,expiresAtUtc:string}`；opaque token由宿主crypto.randomUUID生成，不把裸路径签发权交内核。

服务端内存grant记录绑定当前认证实例/编辑lease、document/epoch、purpose/format、已授权canonical targetPath、expectedPrimarySha256及命名reservation；初始不绑定revision。闲置grant有效期精确600000ms（设计选择，非性能预算/实测）；首次接受HostBundle时原子绑定snapshotId/revision/stateDigest与全部字节摘要，此后同token同bundle只查询/返回同operation结果，不重复写；不同bundle E_IDEMPOTENCY_CONFLICT。首次写前过期拒绝E_TARGET_TOKEN_EXPIRED；已进入writing的同一操作不因TTL中途失效而假取消，写成/失败/indeterminate均保留真实状态。撤销编辑lease的检查在主提交前执行；真正关页必须等该保存结算后handoff。

其他document/epoch/purpose/format、目标基线不同、未知token均拒绝E_TARGET_TOKEN_INVALID，不按调用者提供路径降级。宿主重启失去grant表时旧token失效，保存不能盲重放；通过重新认证的既有作品路径及其实际主文件/资源SHA恢复确认或明确indeterminate，再签新grant。已执行任务的查询与真实文件核验不是再次写入。Token不落浏览器持久存储、不含秘密，Agent不能自行签发HostAck。

```text
SaveState = {
 documentId,epoch,currentRevision:Revision,currentDigest:StateDigest,
 currentWorkRef:WorkRef|null,lastSavedRevision:Revision|null,lastSavedDigest:StateDigest|null,
 dirty:boolean,
 pending:[{snapshotId,purpose,revision,stateDigest,targetToken,
           stage:'materializing'|'writing'|'indeterminate'}],
 lastError:{code:string,stage:string,snapshotId?:SnapshotId}|null
}
```

dirty=(lastSavedDigest为null或currentDigest不等于lastSavedDigest)。导出不更新lastSaved*和currentWorkRef；saveAs只在同document/epoch的真实匹配ack后重绑；R7ack到达R8更新已保存快照但dirty仍true；新稿epoch的状态完全不受旧ack影响。过期/拒绝/空间满等失败不能清dirty。错误集合新增E_TARGET_TOKEN_EXPIRED/E_TARGET_TOKEN_INVALID。

### 9.3 解码资源名字与异步资源所有权

`ResourceReader::resolve(logical_path:&str)->Result<ResourceId,ResourceError>`和`read(id:&ResourceId)->Result<&[u8],ResourceError>`由已授权资源表提供；project.loadBytes传明确logicalPath→ResourceId映射，不能只给无名字ID数组让codec猜IMPORT_DENSITY。logicalPath是原生文件中的相对逻辑名，不是宿主绝对路径；拒绝绝对、..、NUL、重复及符号链接。宿主先用既有安全守卫读字节并构映射，内核不自行打开任意路径。

Engine::advance与execute每次都显式接收同一ResourcePort；KernelSession/CLI适配器拥有资源表，job只retain稳定ID，不保存一次调用传入的短期借用。Worker在每个advance返回后yield，取消和新epoch才可进入；不跨mutate/grow保留read切片。

### 9.4 CA目标类型补充

锁定KindRegistry::detect及WebsiteKind::detect另核于R/method-kind-supplement.json：本地VESTA目标加site栈不自动变website，request中的website/网站/网页措辞可能触发WebsiteKind。未来prepare必须锁真实request/target/stack及选中kind，确认相应检查集合；不能靠任意文案让AppKind/WebsiteKind互换。此项继续属于G-CA，未运行worker。

## 10. dsh差异复核后补正（I4；2026-11-11 dsh复审）

同一dsh session差异复核接受§9四项主修正，仍指出两处类型邻接残余。本节修订它们；原dsh审查字节是I3。I4随后于2026-11-11由dsh独立复审（R/dsh-review-I4-final.md）：R3-1/R3-2实质解决；P1.2与整体仍未完成。

### 10.1 bindings、Worker与资源trait闭合

KernelSession新增正式导出`advance(work_units:u32)->JobEvent[]`，Worker envelope新增op=advance、payload=`{workUnits:u32}`；Worker内部scheduler每轮调用它并yield。外部Agent不需手动advance，调用由同Worker适配器承担；原生CLI用Engine.advance。KernelSession拥有MemResources，对Engine::execute和Engine::advance每次传同一ResourcePort。

ResourceReader具有resolve/read；ResourcePort明确为其supertrait：`trait ResourcePort:ResourceReader`，自身方法为put/retain/release及`bind_logical_path(&mut self,path:&str,id:&ResourceId)->Result<(),ResourceError>`。read/resolve继承，不再是两个无关接口。先校验loadBytes的逻辑名映射并绑定，再decode；每个doc/epoch/job持不可变NameMap，不能被下一稿替换。MemResources及其具名view实现两个trait；codec签名统一为`decode<R:ResourceReader+?Sized>(input:&[u8],format:InputFormat,resources:&R)->Result<DecodedProject,CodecError>`，由泛型接收ResourcePort的reader能力，不要求施工猜trait转换。ResourcePort读取切片仍不得跨mutate/grow。

### 10.2 保存参数的封闭联合与错误归属

以下为唯一首链schema，替代前文purpose/format/targetIntent松散并列的示意；注册表生成JSON Schema oneOf，每支const固定判别字段并additionalProperties=false：

```text
PrepareSave =
 {purpose:'save',format:'vesta',targetIntent:'current'}
 | {purpose:'saveAs',format:'vesta',targetIntent:'saveAs',requestedName?:string}
 | {purpose:'export',format:'cif'|'png',targetIntent:'export',requestedName?:string}

SnapshotOptions =
 {purpose:'save',format:'vesta',targetToken:string}
 | {purpose:'saveAs',format:'vesta',targetToken:string}
 | {purpose:'export',format:'cif',targetToken:string}
 | {purpose:'export',format:'png',targetToken:string,renderedFrame:RenderedFrameRef1}
```

prepareSave与file.snapshot分别只接这两个类型；grant内部按该分支绑定intent，不能传save+png、saveAs+cif、export+vesta或save+export intent。无效组合在产生快照/写文件前E_SCHEMA。HostBundle和HostAck延续同一判别字段，宿主再次验证，不靠客户端类型检查代替服务端约束。

E_TARGET_TOKEN_EXPIRED/E_TARGET_TOKEN_INVALID现在列入§3.1稳定协议错误集合，均host-only，只由prepare/commit/status的HostNack产生；Rust领域Engine不自行签发/消费token，领域错误仍属CommandError，统一API错误为CommandError或HostNack的判别联合。Token错误、晚ack和export均不能清dirty。

## 11 P1 续办的领域与格式边界 I5

2026-11-14。本节是 Codex 的设计选择和对既有规格的消费，不是新增原件观测。证据目录为 [R1](../../../output/vesta-study/20261108-113622/reviews/20261114-100546-p1-design-continuation/)。`R1/contracts/*.schema.json` 和 `examples.json` 是 design-only 协议示例，只覆盖明确列出的类型／命令子集；`profile-scope.json` 明示 fullDomainFrozen=false。它们不能注册为已实现能力，也不替代 §7 的全域冻结门。

### 11.1 相定位和数值的生产消费约定

所有矩阵按列向量数学约定表达，JSON 用三行数组；不得由数组存放顺序猜乘法方向。`H` 将相内 fractional 坐标映射为相内 Cartesian Å。新增计划类型 `PhasePlacement1` 的两个封闭分支是 `{reference:'scene',rotation:Mat3,translationA:Vec3}` 和 `{reference:'phase',phaseId:ObjectId,rotation:Mat3,translationA:Vec3}`；第二支必须引用已存在且非自身的相。相参考图先检查无环，再拓扑求值：`world=Rparent*(Rlocal*H*f+tlocal)+tparent`；scene 根取单位变换。旋转需有限、正交、det=+1；这是本产品合法表示域，不声称原件所有控制路径均这样校验。

`PhasePlacement1` 由 model 保存、geometry 解析，scene／renderer 只消费解析后的 worldFromPhase；codec 必须显式把参考文件中的坐标／旋转／平移约定转换至本类型。原件 GUI 追加相 Y 缩放及一般 R/t 未证，不能把该 canonical 类型当已完成的原件字段映射。placement.set 是未来计划命令，只在该 profile 的映射和测试前置通过后公开；一般 reference 输入未明时不暴露一个猜测默认值。

`Cell3`、`ImageKey`、`Revision` 沿用 I4。Revision 在 JSON 保持 u64 十进制字符串，除正则外仍须检查 <=18446744073709551615。I5 的调度消息明确 workUnits 为 1..u32::MAX，零值在消息 schema 层拒绝；这是本轮防空转的协议选择，尚无运行实现。闭合 schema 只验证形状，Cell 的 Gram 因子／矩阵秩、旋转正交、参考图无环、对象存在和预算均由具名语义检查承担。未知字段、NaN、Infinity 不能进入合法计算值；原字节可另以诊断资源保留。

### 11.2 张量 群表和科学算子

`AdpTensor1` 为 isoU/uA2、cifU/sixA2、beta/six、cartesianU/sixA2 四个有 tag 的分支，六分量次序固定 11,22,33,12,13,23。CIF 的非正交 U、无量纲 beta 和 Cartesian 协方差禁止混用。model 保存基底标签；geometry 的转换输出新的带标签张量；render 只能消费 Cartesian 协方差和已核的概率 profile，不能直接将六项当缩放系数。PSD／零／负特征值的原件适用域、完整协方差与科学椭球几何仍属 C-SCI-02，不能用 schema 数字合法推定能绘制。

群表／参数表采用计划 `ParameterTableRef{resource:Sha256,sourceRef:string,tableVersion:string,licenseRef:string}`，同时绑定实际原始字节、来源、版本及许可。它只规定资产身份；空版本、运行时 latest、仅有群名称、用同位素质量数 A 代替精确质量都拒绝。一般／磁群／调制的具体表及标准化、Wyckoff 容差、超空间算子仍没有可接受的完整模型，不输出宽松 `SymmetrySet:any`。P1／已证反演与磁性对照只是相应测试 profile，不能代表全部群。

本轮只给已有明确输入边界的计划科学请求增加闭合机器示例：

| 计划类型 | 精确输入 | 输出与消费方 |
|---|---|---|
| `LineRequest1` | operation=line.sample，fieldId、startFractional／endFractional 各三有限数、count 为 u32 且 >=2、RuleRef | `LineResult1{unit,points:[{distanceA,value}],fieldDigest,sourceRevision}`；points 数必须等于 count，来自同 field/revision；UI／文本导出消费 |
| `PattersonRequest1` | operation=field.patterson，fieldId、RuleRef | `FieldResult1{descriptor:FieldDescriptor2,resource,sourceRevision,warnings}`；单位由电子／核／通用声明场分别推导，不能统一无量纲 |
| `ConversionRequest1` | operation=field.convert，fieldId、quantity∈lnAbs/laplacian/g/v/he、RuleRef | 同 FieldResult1；每种量的单位、阈值、兼容 26 邻点规则独立；不把连续导数 oracle 当兼容离散结果 |

`RuleRef={ruleId,ruleVersion,featureId,evidenceRef,evidenceSha256,referenceVersion:'3.90.9a-macos-arm64'}`（当前封闭联合以§12.1为准） 不授予执行能力。注册表必须核 ruleId／feature／输入域与实际实现匹配，不能由调用方任填一条证据就宣告算法有效。上述结果是字段设计，不是已执行结果。全 ScientificRequest 的反常散射、全谱/Nyquist、powder、多相、等值面退化、投影、峰盆地及外算 options 尚不能统一导出；它们必须在各自冻结后增加有 tag 的独立分支。

### 11.3 场网格和资源顺序

为消除原 I4 的 `basisA` 究竟表示整胞还是单步的歧义，本轮提出 `FieldDescriptor2`，以 `stepBasisA` 的三个列向量直接表示索引每增加一格的 Cartesian Å 增量；`originA+stepBasisA*[i,j,k]` 为采样点。原 `FieldDescriptor` 在未来完整绑定前不得不带版本混用；旧格式转换需显式记录原 convention。此处只是无现存产品的协议修订，没有迁移实际工程。

FieldDescriptor2 保留 kind／unit、coreShape、closure、periodic、axisOrder、dtype、resource 和 originA；generic 的 declaredUnit 必须为非空字符串，其余 kind 的 declaredUnit=null，并分别匹配 ePerA3、fmPerA3、e2PerA3、fm2PerA3。closure 为 true 的轴必须 periodic，实际存放维数为 coreShape+closure，资源长度精确等于三维存放点数乘 dtype 字节数；乘法须做 u64 溢出和 Budget 检查。周期补边取 core 索引模数，不把重复末点计入 FFT 的 N；非周期轴没有隐式补边。通用非周期／单点场的运算适用域单独检查，不能把没有周期的字段悄悄套入周期 Patterson。

x-fastest 的线性索引 `i+nx*(j+ny*k)`；z-fastest 为 `k+nz*(j+ny*i)`。从文件得到 descriptor 的是 codec，资源表拥有原始数组，science 按显式顺序读取并输出新 resource，renderer 得到 retain 的 owned copy；不在 UI 再计算一次科学结果。CUBE 的坐标 bohr→Å 与 scalar 的物理量换算是两个独立转换；保留原件 scalar 缺陷的已证兼容 profile 和显式风险，不能只换坐标后宣告物理密度正确。

### 11.4 全格式调度与信息保留

FormatId 不从文件后缀独自推断。现有 S05 的 VM18-I01–I47 加 OBS-I-MCIF 为 48 个输入身份，OBS-O01–O23 为 23 个数据输出身份，枚举来自原矩阵；图像／2D／文本是另外的 profile，不挤入“23”。本轮 `format-identifiers.json` 是派生枚举，不另建覆盖状态。`FormatRequest1{formatRef,direction,purpose,primaryResource,dependencies:[{logicalPath,resource}],profileId}` 的 decode 仅接受输入枚举与purpose=import；encode 的 save/saveAs仅接受OBS-O01原生工程，其他数据输出走export，不暗改I4的工程／导出身份。profileId 还必须选择一个已经冻结的格式参数类型，不能接自由 options JSON；这个调度外壳不证明各格式内部参数已冻结。

统一未来接口：`decode<R:ResourceReader+?Sized>(bytes,format,resources)->Result<DecodedProject,CodecError>` 保持 I4；增加计划 `assess_loss(project:&ProjectState,profile:&ExportProfile)->LossReport`，其中 `LossReport{droppedFields:[{objectId,fieldPath,reason}],unsupportedBlocks:[RawBlock],requiredResources:[ResourceId]}`；`encode(project,profile,lossConsent)->EncodedBundle` 只消费冻结 profile。lossConsent 绑定 sourceDigest、profileId、完整 LossReport 摘要和一次显式用户选择，旧同意不得套新稿。默认为拒绝静默丢字段；原件已有的有损行为须具名风险 profile，不能称完整无损保存。

字段共同模型为 cell／symmetry／sites／species／occupancy／带基底 ADP／vectors／field／scene／comment，加原始词法和 source byte spans；每个 codec 必须逐字段声明 decoded、preserved-opaque 或 unsupported-loss，不把 preserved-opaque 宣称可编辑。具体格式安排见模块计划 G4.1；原始矩阵中的证据、产物 SHA 和 nextProbe 继续为依据。

特别边界：PDB 的 U/B 与 ANISOU 列分别核；Elk 空字符串／物种丢失、ASSE 列、GAMESS 首原子／跨度和 AMCSD 大小写保留原失败；m3d 量化、PRI flag/ABI、DEN／STO 方言没有完整 writer 资格，不能拿统一结构模型凑出已冻结格式选项。JP2 无产物、SGI 无效、TIFF alpha 丢失与 STL 非有限法向也不能被“resolved 原因”洗成成功。

### 11.5 宿主竞争和查询恢复

I4 的 grant／HostBundle／HostAck 保持。`commitNativeBundle` 是拟新增宿主函数，不是现有 SoftwareFiles 能力：它在既有协调器 serial 内串行处理快照；write 前及 primary 提交前复核打开／最近确认的 baseline；资源和 manifest 写入且读回成功后才写 primary，最后实际读回确认全部文件。只读／配额／路径仍由平台强制，不能由客户端 grant 类型替代。

新增计划查询 `querySnapshot({documentId,epoch,snapshotId,targetToken})->Writing|Verified{actualFiles}|Rejected{code}|Indeterminate{expectedFiles}`，它不能重发写入。宿主重启 token 失效后，用重新认证的作品路径和原快照预期 SHA 做只读核验；不得信任页面自报目录。全部 SHA 符合才可为原快照恢复 Saved；部分不符保留 dirty 和 indeterminate，提供显式重新加载／另存，不先回滚可能已经提交的新文件。

快照协议不等于任意外部写者 CAS。既有 coordinator serial 只保证该实例自己的操作；不同实例／进程和外部文件编辑器不由它共同加锁。本轮不新增一项“已经有全局锁”的假事实。P4 若要提供所有受控写者的串行承诺，必须核真实所有写入口及锁范围；不能覆盖的外部写入窗需要明确契约和用户裁定，见 C-HOST-01。在此之前，完整 G-HOST 仍未冻结。

### 11.6 本轮仍未冻结的具体设计问题

这些是 P1 设计阻塞标识，不是新增原生 unknown，也不改变原 24 个问题头。

| 标识 | 尚缺的接口决定与原依据 | 本卡能继续的设计／不得冒充的结论 |
|---|---|---|
| C-SCI-01 | 一般参考相 R/t 的原件字段映射、全部群表／setting／磁群／调制及超空间算子；S06 phase/cell-symmetry/vectors 的 nextProbe | canonical placement／资产身份可定义；通用导入和相操作不冻结 |
| C-SCI-02 | 非正定 U、概率椭球实际几何、不确定度／协方差和键价选表；sites-adp/bonds/polyhedron | 带基底张量与单独 rule 引用可定义；不得猜表版本、阈值和坏输入后状态 |
| C-SCI-03 | 反射集合／Nyquist、全谱、等值面退化／封口、投影离散权重、峰盆地；对应各组 | 已证算子子集和资源边界可定义；全 ScientificRequest/Result 不冻结 |
| C-CODEC-01 | m3d 全量化、PRI 各 flag/ABI、DEN/STO 方言及缺陷 profile 的完整字段／选项 | FormatId、loss 与资源策略可定义；全格式 writer 精确参数不冻结 |
| C-HOST-01 | 多写者竞争／CAS 的承诺与原件对 hash 相对路径／manifest 的读取兼容 | grant／查询恢复可精确设计；完整跨资源保存安全和原生兼容不宣称通过 |
| C-ASSET-01 | 群／参数／字体许可、MADEL/RIETAN/ORFFE 实际运行件与 Linux/x86 环境 | 精确资产身份／外算候选结果协议可继续；运行与性能资格不追认 |
| C-UI-01 | 数据库查询／结果字段与网络 Stop、若干偏好范围和 OS 宿主能力映射；VF-0011 及 G6 原入口／unknown | 应用／焦点／保存生命周期能设计；未证网络和宿主行为不伪装为已冻结 API |

每项最小后续输入来自原规则 nextProbe／UNKNOWNS 的具名材料，不在本轮运行原件补证。已经冻结且不依赖这些问题的基础模块可以形成具体未来任务；P1.2 全域完成与受影响的 P1.4 精确施工安排仍不能由本节单方宣告。待审材料将分别报告已推进内容和这些残留。

## 12 dsh 复审后的接口整改 I6

本节对应 F2–F8；[R2](../../../output/vesta-study/20261108-113622/reviews/20261114-105518-p1-design-corrections/) 保存新的 contracts 和反例，R1 原 schema／检查器／报告保持字节，不把本节修订回填成 R1 已有内容。§11.6 的七个门继续存在，且 F13 的高级类型/真实oracle和逐格式合同不能由注册表自动解决；剩余问题不再声称只有七项。

### 12.1 RuleRegistry 和 FormatProfileRegistry

`R2/contracts/rule-registry.json` 是本轮**设计命名空间**，不是声称 S06 原来存在同名 ruleId。每条记录绑定 ruleId、ruleVersion、operation、featureIds、domainGroup、inputDomain、referenceVersion、evidenceRef／evidenceSha256、S06 规则 JSON pointer／整文件 SHA 和 observed-limited-design-binding 状态；productImplemented=false。当前只定义 line.sample、field.patterson、field.convert 的三个已证子域，没有登记一般群／全谱／未证算法。

`RuleRef` 现为注册表生成的封闭联合：`{ruleId,ruleVersion,featureId,evidenceRef,evidenceSha256,referenceVersion}`，字段必须共同匹配同一记录。LineRequest1 等各自只允许对应 operation 的 binding。未来内核 `resolve_rule(reference:&RuleRef,operation:OperationKind)->Result<&RuleBinding,CommandError>` 先检查存在、操作／功能／版本／证据一致，再检查该记录的输入域和实际实现；未知或未冻结规则返回 E_RULE_UNFROZEN，不接受任填非空字符串或只伪装来源。本轮反例包含 unknown rule、错 feature、错版本和错证据摘要。

`R2/contracts/format-profile-registry.json` 明列48输入、23数据输出、14图像路由，共85项。每项有独立 profileId、FormatId/方向/用途、精确 handler 文件、源矩阵行或图像入口及 SHA、字段保留政策、参数类型／schema和设计状态；两个 EPS 路由不同。它是格式分派合同，不是功能覆盖率或接受台账。

其中仅四个 CoreStructureSubsetProfile1（VESTA/CIF读入、原生保存、CIF导出）具有本轮封闭的空参数 schema，沿用 I4 的单P1结构字段，未知科学层只保留或拒绝有损写回，不能当可编辑全格式。其它条目明确 unfrozen-parameters、parameterType=null，不进入当前设计示例的封闭请求联合。未来 `resolve_profile(id:&ProfileId,format:FormatId,direction:CodecDirection,purpose:FilePurpose)->Result<&FormatProfile,CommandError>`：未知 id／键不匹配拒绝 E_SCHEMA；已知但参数未冻结拒绝 E_RULE_UNFROZEN；实现后的正式命令发现还必须过滤 productImplemented=false。unknown profile和跨方向复用的反例已具名。

### 12.2 正文类型名和机器定义

I6 把 defs 直接命名为正文名称并加 title，完整对应在 R2/contracts/type-map.json：PhasePlacement1、AdpTensor1、FieldDescriptor2、LineRequest1、PattersonRequest1、ConversionRequest1、FormatRequest1 对应同名 `#/$defs/...`，不再用 Placement/Tensor/Field 等别名。原先未接线的 ParameterTable def 已移除；ParameterTableRef仍是§11.2的资产身份设计，不宣称它已成为可接收 wire type。

Revision继续是canonical十进制字符串。未来唯一语义函数 `validate_revision(text:&str)->Result<u64,CommandError>` 同时检查 ASCII canonical正则和 u64 checked parse；不得转 JS Number 或截断。R2列0、u64::MAX为正例，18446744073709551616、前导零、负号、空串和超长数为反例。schema只检查形状，语义上限由具名校验承担，不再把正则通过说成整个Revision合法。

### 12.3 保存案例的初始条件和 PNG 帧

所有书面保存案例显式给出初始 lastSavedDigest。失败／导出／跨稿ack保持这个值，只有同稿同epoch且成功的save/saveAs才能改成快照digest；dirty永远从当前digest与最终lastSavedDigest推导。R2同时有“原本clean的SaveAs失败仍clean”“dirty工程导出仍dirty”和“原本clean的导出仍clean”，不把所有失败一律写dirty=true。这是演算修正，不声称产品实现已经验证。

I6 的 PNG SnapshotOptions 分支增加必需 `renderedFrame:RenderedFrameRef1`，CIF分支不得携带它：

```text
RenderedFrameRef1 = {documentId,epoch,revision,frameId:Id,resourceDigest:Sha256,
                    viewport:{widthPx:u32>0,heightPx:u32>0,dpr:finite>0}}
```

SaveBundle／materializeBundle／HostBundle在PNG时携带相同 renderedFrame，grant首次接受快照时将该值连同snapshotId／revision／stateDigest／outputSHA绑定。页面的具名 `assert_rendered_for_snapshot(frame,lastRendered,currentDocument,currentEpoch,currentRevision)` 在取不可变frame lease和编码前检查所有字段；通过前不生成PNG字节，不靠UI自报“渲染成功”。帧的资源保留到编码完成后释放；GPU lost或资源失效用E_RENDER_FAILED，身份/修订/frame/resource/viewport任一失配返回E_REVISION_CONFLICT，不截图更新后别的帧。

宿主在同一编辑lease和认证PageChannel的snapshot应答中核该绑定，回执不得来自Agent自行提交的伪造HostAck；它不能直接观测GPU，所以实际页面frame lease及服务端核对留P2/P4真实验证。PNG缺frame、错误修订／epoch／frameId／resourceDigest都有具名设计反例。此处不新增一个已经运行的渲染能力。


## 13 R3的当前机器定义与引用导航

当前contracts入口为 [R3](../../../output/vesta-study/20261108-113622/reviews/20261114-120337-p1-design-r3/contracts/)；R2中的schema和回执保持历史字节。§11.2的RuleRef现与§12.1六字段统一；9根schema的全部21个def（包括RenderedFrameRef1）都有同名title，正文/defs双射不变。没有新增一个可执行科学功能。

domain-handoff-contracts.json的pathConvention明确oracle.path、inputs.path、inputs.sourceRecord均为项目根相对POSIX；每条input的pathBase=project-root，sourceRecord存在时sourceRecordBase同值。sourcePointer仍定位原冻结记录里的字面路径，通过相同后缀与SHA解释历史Craft定位，不能把该字面值当当前可执行路径。

oracle.comparisonBindings逐项给输出字段、role、单位、expectedPointers、tolerancePointers、比较条件与精度政策。只有direct-result/result-invariant是对应输出的直接/局部判据；supporting-only不能作为该函数结果的通过依据。missingExpected和unfrozen/analytic-reference-no-runtime-tolerance明确缺口，不能自动补统一epsilon或把不存在的null当预期。N1/N2的导航修正不关闭F1或完整F13；P1.2/P1.4仍partial。


## 14 R4的来源字段精确定义

当前contracts位于[R4](../../../output/vesta-study/20261108-113622/reviews/20261114-124229-p1-design-r4/contracts/)，不修改R3已封存字节。oracle.expectedPointers是按binding顺序去重的非supporting-only expectedPointers并集；支持证据另在supportingPointers，不能只读顶层字段就当全部oracle。tolerancePointerKinds区分numeric-bound/structured-field-bounds/prose-policy-or-reason；sf的comparisonPolicy和tolerance_reason等文字不是数值epsilon，内部精度缺口继续列missingExpected，不据此声称可机器数值验收。

sourceRecordBase=project-root只定位sourceRecord JSON文件；sourcePointer指向的值是原历史定位文本，sourcePointerValueBase分别标study-root或historical-project-absolute，再按同后缀和输入SHA解释。planes的contract和两条源值记录统一指本轮同字节副本。图像sourceLocator：lineNumber为1基；columnIndex/tokenIndex为0基，列按原始行split("|")（含首尾空项），取列去两端空白后按“、”分token；rowSha256是去掉LF/CRLF行终止符的原始UTF-8行SHA，不trim其它空白、不正规化。

新增browser-install-receipt.schema.json的12个基础设施类型与既有21个产品wire def分属不同schema，不改原wire双射；receipt格式/失效与来源信任以C6及browser-input-contract.json为准，标准schema全规范仍未验证。


## 历史：15 R5主体设计（当前修正见§16）

本节是当前I9新增设计，入口为[R5 contracts](../../../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/contracts/)。I8以前首链合同保持；本节补足可由现有资料确定的公开数据边界，不以31个小操作代替31组完整功能。P1.2/P1.4仍partial；产品没有注册或实现这些命令。其余字段/布局按本文先前已冻结合同继承，冲突时本节仅覆盖明确列出的新增有界操作。

`scientific-operations.contract.json`内99个有名类型定义；全部对象拒绝额外字段，所有ref局部可解析。`Request`按operation辨别，每项有完整Input/Output形状，Context绑定document/epoch/baseRevision和预算。类型数是结构计数，不是科学覆盖率。`scientific-semantics.json`规定schema之外的有限数、引用闭合、矩阵/单位/数组长度和资源不变量，不能只验JSON形状后进入计算。通用执行仍走Engine/advance/ResourcePort，结果只为候选，不能越过修订校验直接commit。

新增模型包括：有原token的Species、显式基底Adp、带参数单位的Covariance、带有理平移/时间反转的已提供三维SymmetryOp、Structure、phase Placement、Field、Reflection/PattersonCoefficient、Mesh、Camera、ImagePoint、Measurement/Line/Slice和资源Bundle。3D算子不代表superspace/调制结构；没有资质的原始块继续保留并拒绝未知科学编辑。Field的stepBasisA列为网格步向量，coreShape/closure决定采样数，坐标r=originA+sum(i_j*stepBasisA_j)；这与晶胞H不能混用。体积算子使用周期cell列`N_j*stepBasisA_j`，闭合副本不参加积分或FFT。

ADP保留输入U/beta和负特征值分类，不暗中夹零。B=8π²U；CIF U经H和倒易长度矩阵转Cartesian U，beta经H beta H^T/(2π²)。Ueq=trace/3。概率表99个标量因子逐条绑定既有只读快照；此举不证明完整椭球几何。协方差矩阵与Parameter列表双射，元素单位为unit_i*unit_j；缺失为null，不造零误差。完整ORFFE解析/非正定椭球仍B03。

| 领域 | 有界公开操作 | 输入→结果与限制 |
|---|---|---|
| phase | `ResolvePlacements` | `ResolvePlacementsInput → ResolvePlacementsOutput`；World transform is parent composition, translations in Cartesian A; graph must exist and be acyclic. Product convention is explicit; native refer/R/t mapping remains B01. |
| cell-symmetry | `ExpandImages` | `ExpandImagesInput → ExpandImagesOutput`；r=H(Mf+t+n); explicit supplied 3D operations only. No group-number lookup, Wyckoff tolerance or superspace inference. |
| transform | `ChangeBasis` | `ChangeBasisInput → ChangeBasisOutput`；Hnew=H P; fnew=P^-1(f-p+n). Reject singular P; explicit selected n; no guessing group/setting. Tensor transport mathematical, native covariance qualification B03. |
| sites-adp | `TransportAdp` | `TransportAdpInput → TransportAdpOutput`；Six order 11,22,33,12,13,23; Ucart=H D* Ucif D* H^T; beta converts via H beta H^T/(2pi²); Ueq=trace/3. Preserve nonpositive tensor; return classification, do not invent ellipsoid geometry. |
| duplicate | `FindDuplicates` | `FindDuplicatesInput → FindDuplicatesOutput`；Returns connected components of explicit image positions, distance<=threshold; ordered by first input. Group merge/averaging policy not inferred; B04. |
| bonds | `SelectBondPairs` | `SelectBondPairsInput → SelectBondPairsOutput`；Distinct explicit images, unordered index i<j; inclusive squared thresholds. Do not replace selected image with nearest one or imply recursive boundary/bond-valence support. |
| polyhedron | `MeasurePolyhedron` | `MeasurePolyhedronInput → MeasurePolyhedronOutput`；Closed consistently oriented triangular surface required; V=abs(sum(v0 dot(v1 cross v2))/6), area=sum triangle areas. Require mesh validity; do not infer general VESTA neighbor selection/angle/BV policies. |
| vectors | `TransformVector` | `TransformVectorInput → TransformVectorOutput`；Polar Mv; axial timeReversal*det(M)*Mv. M orthogonal only; fractional magnetic basis conversion requires B02. |
| planes | `MillerPlane` | `MillerPlaneInput → MillerPlaneOutput`；g=H^-T h, n=g/|g|, d=1/|g|, offset=order*d; zero h rejected. Fit/SVD and native sign convention B05. |
| boundary | `ClipExplicitImages` | `ClipExplicitImagesInput → ClipExplicitImagesOutput`；Use H^-1 r, inclusive per-axis boundaries, no generation of extra images. Full legacy duplicate/special-position behavior B04. |
| view | `ComposeCamera` | `ComposeCameraInput → ComposeCameraOutput`；Proposed composition Rz Ry Rx Rcurrent, degrees and right-handed axes. CLI selected x/y controls do not prove all legacy camera conventions B06. |
| measure | `MeasureImages` | `MeasureImagesInput → MeasureImagesOutput`；2/3/4 points respectively; nonzero lengths/normals required; unsigned dihedral 0..180; variance=J C J^T; sigma=sqrt(variance) only when supplied covariance parameters exactly identify points. Native ORFFE/general covariance B03. |
| tidy | `ProposeTidyResult` | `ProposeTidyResultInput → ProposeTidyResultOutput`；Validate candidate first, never auto-replace active structure. This is typed external-result admission, not a general TIDY/Niggli implementation; B14. |
| madel | `PrepareMadelInput` | `PrepareMadelInputInput → PrepareMadelInputOutput`；Input file generation plan only; charge-site bijection and explicit radius. Exact MADEL grammar/parameter qualification B14; no runtime or numeric-result claim. |
| powder | `ConvolvePowderLines` | `ConvolvePowderLinesInput → ConvolvePowderLinesOutput`；Explicit pseudo-Voigt unit-area G/L mixture, no background or intensity generator; equal line arrays and monotone sample grid. Compatibility index first point zero only selected evidence; full native width/weights B07. |
| sf | `SumStructureFactors` | `SumStructureFactorsInput → SumStructureFactorsOutput`；Provided per-site per-hkl amplitude already includes declared scattering/ADP factors; sum occ*a*exp(+2pi i h.f). Explicit symmetry-expanded sites; no hidden table lookup or isotope default B02/B07. |
| fourier | `SynthesizeFourier` | `SynthesizeFourierInput → SynthesizeFourierOutput`；rho=1/V sum F exp(-2pi i h.f); explicit signed hkl, DC replacement and conjugate policy. Reject aliases including Nyquist duplicate conjugates; modes/normalization beyond supplied coefficients B08. |
| electron | `ElectronDc` | `ElectronDcInput → ElectronDcOutput`；Constant field count/V, not full atom-centered density or sampling-size algorithm. Full generator B07. |
| nuclear | `NuclearDc` | `NuclearDcInput → NuclearDcOutput`；Constant field signed sum/V fm/A3, negative allowed. Do not claim whole-map shape from DC control B07. |
| model-patterson | `ModelPattersonCoefficients` | `ModelPattersonCoefficientsInput → ModelPattersonCoefficientsOutput`；Per explicit hkl modulus squared; output imaginary 0 and squared-unit metadata described in operation units. Full model grid/table policy B07/B08. |
| volume-patterson | `VolumePatterson` | `VolumePattersonInput → VolumePattersonOutput`；Periodic core samples only, F=V/N sum rho exp(+2pi i h.f), inverse |F|²/V, closure regenerated. Selected orthogonal complete-grid oracles exist; generic Nyquist/nonorthogonal B08. |
| volume-data | `NormalizeGridLayout` | `NormalizeGridLayoutInput → NormalizeGridLayoutOutput`；Reindex and periodic closure only; no scalar rescale or inferred physical-unit conversion; strip/regenerate closure with periodicity check. |
| shape | `IntersectHalfspaces` | `IntersectHalfspacesInput → IntersectHalfspacesOutput`；Halfspaces n.r<=d, normalize normals with offsets consistently; reject unbounded/empty/lower-dimensional result before returning mesh. Native UI face/area settings B09. |
| volume-render | `SelectIsoEdges` | `SelectIsoEdgesInput → SelectIsoEdgesOutput`；Returns interpolated edge-crossing positions only, equality policy lower-inclusive upper-exclusive; no triangles/caps/NaN-normal compatibility claim B09. |
| structure-render | `BuildAtomInstances` | `BuildAtomInstancesInput → BuildAtomInstancesOutput`；Explicit uniform ball instances only; no unlicensed radius lookup, ADP ellipsoid or hidden style defaults. Full style/radii/font B10. |
| 2d | `SliceTrilinear` | `SliceTrilinearInput → SliceTrilinearOutput`；Sample origin+i*u+j*v with explicit step vectors; outside nonperiodic extent rejects; periodic eight-corner trilinear. Projection/contours/log B11. |
| line | `SampleLine` | `SampleLineInput → SampleLineOutput`；Include both endpoints at i/(N-1); distance from H(end-start), preserve direction, periodic trilinear. Selected oracle full path/value bindings retained. |
| peak-voronoi | `FindLocalExtrema` | `FindLocalExtremaInput → FindLocalExtremaOutput`；Periodic core 26 neighbors; strict comparison to all; equal plateau omitted with diagnostic; sort maxima desc/minima asc then x/y/z lexicographic. This explicit new policy is not general legacy basin qualification B11. |
| poly-integral | `QueryPolyIntegralCapability` | `QueryPolyIntegralCapabilityInput → QueryPolyIntegralCapabilityOutput`；Exact pinned reference menu185 no-action only. No numerical integration operation, no new architecture/version extrapolation. |
| density-conversion | `ConvertDensity` | `ConvertDensityInput → ConvertDensityOutput`；Use locked S06 formulas and raw-rho<=1e-6 energy zero; lnAbs zero produces diagnostic special -Infinity, never ordinary finite Field. Full units/nonorthogonal qualification B08/B11. |
| exports | `PlanEncode` | `PlanEncodeInput → PlanEncodeOutput`；Profile must resolve exactly in 85-route registry; source model/field audited before bytes creation; acknowledged per-field losses required. This is a plan envelope, each writer/reader grammar qualified independently B12/B13. |

### 15.1 格式与外部边界

`codec-field-contracts.json`保留85个精确FormatId路由与原矩阵行/源SHA；6类字段政策通过28条明确FormatId绑定补充原注册表。原注册表仍只有四个首链可接收profile，新增政策均为设计提案，未把其它81项整体提升。m3d已知direct-f32/rescale31/28/24/20的参数可固定，但独立量化编码仍B12；PRI仅flag0/little-endian子域，非零flag先拒绝；轴序/闭合数按7个既有独立输出记录核对，CUBE物理标量失败、X-PLOR尾部不合规仍保留。字段不在已核集合时不默认支持：保留原始源；编辑破坏未知块则拒绝或明确逐字段lossConsent，导出不改变dirty。

`host-application.contract.json`新增7类邻接类型：WriterCapability、SavePlan/SaveObservation、DatabaseNavigation/ImportCandidate、ExternalCandidate、AssetSource。宿主实例内serial与安全rename不构成外部CAS；共享受控写者需平台真实fence/写者集合，客户端不能自报权限。`primary-committed-unverified`先查询实际SHA/依赖，不盲重写。B13未解除，不新建存储服务。

数据库按已观察COD网页导航/候选下载设计，不造查询REST API或provider列表；URL重定向、大小和媒体类型须宿主核验，解析/确认成功才替换旧工程。Stop令navigationId失效并尝试abort，真实慢请求取消仍B15。外算输出即使exit0仍pending，须独立语义检查再候选提交；各工具完整输入语法/artifact/许可仍B14。AssetSource必须有字节/版本/许可/再分发决定，缺项不默选系统字体/原件表。

### 15.2 证据可证明的范围

每单元`R5OracleProjectionLimit`把旧oracle与新增输出逐项对照：相面积不能证明R/t，缩放不能证明相机旋转，峰位不能证明卷积样本，模型场积分不能证明Patterson系数。原32条binding/262条源值保持；直接结果、局部不变量、辅助及能力四类不互换。未来fixture adapter、结果投影与真实取消/资源释放测试均列为交付物，当前没有运行或伪造这些实现。

完整资格阻塞收敛到`qualification-blockers.json`的B01–B15，每项给准确问题、旧证据为何不足、最小新材料/探针和授权类型；不是要求先把24个unknown全部关掉才可设计。现有证据可固定的边界已写入本节；剩余原件行为、资产/许可、平台并发或真实环境验证另取具体授权。本轮未新增实验许可，未开始任何运行验证。

I9准入补充：有界DTO不自动注册产品命令，仍由Engine的已实现命令注册表和已注册RuleRef/输入域控制准入。默认保持DEC-P1-01的已证原件行为；与已知原件不同的数学政策仅能在显式Corrected选择和独立oracle具备后公开，不能靠新protocol绕过旧rule/profile门。新资源字节数/预算在线协议限定JS精确安全整数，原生仍checked u64；Revision继续u64十进制字符串。

R5 source-adapter-plans.json补11个可从旧证据逐字段固定的输入映射（line/sf/fourier/ADP/transform/bonds/Patterson/volume/2d/peak/conversion），其余20组完整映射仍未建立；它是fixture设计，不是已实现解析器。转换算子显式lengthUnit=angstrom/bohr，避免两个旧holdout单位被默认合并。


## 历史及继承：16 I10／R6有界契约（差异以§17为准）

R6时点历史（仅追溯，不作当前准入）：该轮机器契约位于[R6 contracts](../../../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/contracts/)。当时新增科学请求使用`protocol=vestacraft.scientific.bounded/2`，该轮有112个有名定义、31领域/32操作。此处保留旧时点，现行消息准入和当前契约以§18及文首R9入口为准；旧protocol2消息不因保留历史文字而获准。

### 16.1 测量身份、协方差及标准不确定度

`MeasureImagesInput={points:MeasurementPoint[],kind,covariance:CartesianPointCovariance|null}`，每个点为`{pointId,cartesianA}`；协方差参数为`{pointId,axis:x|y|z,unit:A}`，矩阵单位A²。必须对所有点的x/y/z恰一次绑定，按parameters顺序建立J；重复/缺失/额外参数、错误尺寸、非对称/非PSD矩阵均拒绝。数值相同的点不能据坐标推断同一对象。一般fractional/cell协方差需显式链式适配，仍B03，不能偷套本有界输入。

输出`{value,unit,variance,sigma}`。**variance=J C Jᵀ，σ=√variance**；variance单位是输出单位的平方，σ与value同单位；角度J包含180/π。缺协方差则二者均null；零方差是0而不是缺失。两个端点沿x相距5Å、方差.01与.03Å²时，J=(-1,0,0,+1,0,0)，variance=.04Å²、σ=.2Å；.04不能作为σ。零长度/零法向或不可微情形先拒绝，不夹负方差成0。精确PSD数学域与未来稳定分解/源精度资格分开，不临时放宽通用epsilon。

`TransportAdpOutput`增加inertia(positive,zero,negative)，总和3；六个互斥分类为正定、非零半正定、负定、非零半负定、正负混合不定、全零。保留负张量用于诊断，不能直接据此承诺绘制原件异常椭球；B03保持。

### 16.2 公开操作、旧内部step和证据不得混同

31单元现在用`publicBoundaries`与32个当前操作对应；active constructor/step/resultType使用同一Input/Output。R5旧内部签名移至historicalInternalPrimitive，不作当前接口。powder为ConvolvePowderLinesJob/Curve，旧Bragg峰位仅辅助；model-patterson为系数Job/平方单位系数输出，旧场积分仅辅助；boundary的最近像只辅助，新增裁剪集合数学控制；bonds按具名两像对直接比较距离，不扩成完整邻接证明；exports的PRI解码只辅助。独立卷积、系数、裁剪与测量控制见operation-control-cases.json，不是原件或产品运行结果。

每单元consumers具名计划文件、用途、操作及结果schema，另有资源retain/release规则；commitOwner仍仅Engine。结果消费不是一条泛称Renderer/UI/codec的话。模块R6逐项列当前step、消费方和旧oracle角色；完整31单元fullUnitReady仍false。指定版本poly-integral仅NoAction能力查询，其有界设计已闭合，fullUnitReady=false表示产品/运行未验证，不为它新增科学实验或虚构积分。

### 16.3 导出先审计、后获准编码

`PlanEncodeInput={profileId,purpose,source:EncodeSource}`，source带document/epoch/revision/stateDigest及完整可审inventory（structures、fields、placements、scene、comment、opaqueBlocks）。Engine从同一不可变快照核验/生成清单，客户端自报清单不构成权威，缺失未知语义层必须拒绝或明列损失。PlanEncode只返回EncodePlan，无字节/宿主副作用。

`EncodeApprovedInput={source,plan,lossConsent:LossConsent1|null}`；LossConsent1为`{sourceDigest,profileId,lossReportSha256,explicit:true}`。必须重新核实际快照、profile及完整LossReport摘要；有丢弃字段/未知块时三项绑定加显式true缺一不可，不接受string[]，旧同意不能用于新稿。LossReport只含字符串/bool/安全整数，排序/UTF8 canonical JSON及SHA规则在encode-loss-contract.json；Bundle带sourceDigest/profileId/lossReportSha256。实际writer内容、独立读回及原生往返仍未执行；原4首链profile/81未冻结状态保持。

`ExternalCandidate`必带baseDocument/baseEpoch/baseRevision，与所属job快照及当前准入身份核对；失配保留当前稿、拒绝提交并释放候选资源。exit0或semanticAdmission自报都不能替代独立检查。

### 16.4 数据域、版本例子和真实剩余设计

IntersectHalfspaces允许有限零/负offset，非零normal与offset数组须等长且至少4项；n、d同时除以|n|。不要求原点在内部；空/无界/降维结果拒绝E_DOMAIN，不能因d≤0一律拒绝合法平移盒。

I8的6个Field示例保持原值并标版本；protocol/2新增8个Field例子先过新schema，再检单位/闭合/资源长度与实际观测字节，记录具体失败层。Field不是FieldDescriptor2的无条件别名：旧id移到document/epoch内FieldHandle，resource摘要须经真实资源表补bytes；stepBasisA及其它数值语义不变。缺资源/长度不明禁止猜。全部112定义加同名title，机器语义索引按实际17条逐项列正反例；旧图的无环/自环/双环/缺父例保留，新phaseRoot联动根与引用身份，不能说旧图反例不存在。例子通过仍不证明完整数值实现。

85路由只有28条绑定6类选定字段政策，另57条显式field-audit-not-defined；这些首先是旧资料字段审计设计工作，不自动需要新实验。B12逐项列8个codec原生未知头，B13绑定真实平台合同/源码快照，B15绑定DATABASE/GUI_LIFECYCLE而不造S06领域组。B02先审既有非P1/磁性/调制证据和资产身份，只有仍无法回答的问题另成有界探针；没有新增实验权限。polyhedron按几何与BV分别B09/B04；poly-integral不虚构B项。

本轮D1–D14为作者整改、自验待用户转dsh，不是独立接受；P1.1/P1.3 C4设计接受保持，C6原字节未审状态不变，不进入P2–P5、产品实现或运行验证。

I10导出用途补充：PlanEncode/EncodePlan/LossReport均带purpose，resolver同时核注册profile的encode方向及allowed purposes；purpose进入完整报告摘要。不能拿decode profile或未冻结profile借LossConsent绕过准入，旧SnapshotOptions和宿主SaveBundle的save/saveAs/export规则保持。


## 17 I11字段身份与投影（继承R7设计，当前接续见§18）

本节覆盖§16中本轮修正的部分；其余I10有界合同与首链协议继续继承，未因此全域冻结。当前契约见[R9](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163802-p1-design-r9-doc-state/contracts)，113定义、31领域/32操作，所有新操作仍未实现。Field值形状不变；新增FieldHandle及loss objectKind使封闭消息采用`vestacraft.scientific.bounded/3`，不得无提示接受旧protocol2 inventory或旧损失同意。

### 17.1 身份贯通迁移、清单与字段损失

`FieldHandle={fieldId:ObjectId,phaseId:ObjectId,field:Field}`已在$defs定义，EncodeInventory.fields直接引用它。Field是值描述；fieldId是同document/epoch内的逻辑对象身份，Resource.digest只是字节身份。两个场可共享完全相同的描述/字节，仍是两个对象，不能按摘要去重字段对象。fieldId在表中唯一、owner phase真实存在，不能与本快照phase/site/operator主对象ID冲突；清单行顺序不承担场身份或Scene层序。

I8迁移须同时具备old.id、可信owner phase和实际资源摘要/长度观测，形成FieldHandle；缺一拒绝，不由数组下标、label、cell或资源摘要猜身份。新document/epoch通过IdSource重新分配并显式返回映射，旧选择/旧同意不能跨epoch使用。field-type-migration.json和field-identity-contract.json定义精确关系，纸面观测不冒充真实ResourcePort读取。

`LossField`新增必需objectKind(project/phase/site/scene/field)。字段损失必须objectKind=field、objectId=本源快照FieldHandle.fieldId；fieldPath为相对handle.field的RFC6901指针，空指针代表整个字段对象。未知/重复/冲突ID、错误owner/kind、不存在的指针都在编码前拒绝。objectKind进入完整LossReport canonical SHA，所以本次迁移必须重新审计并重新取得LossConsent1，旧摘要/旧同意不沿用。其它对象的逐字段语义仍受对应codec审计，不以新字段规则宣称全部闭合。

field-identity-examples.json的两个场共用资源但各有损失绑定；正例验证ID选择而非数组位置，反例覆盖错ID、错owner/kind、资源冒充ID、重复/空ID、无指针及旧行缺fieldId。它证明身份/摘要合同，不证明实际字段bytes或CIF writer内容。

### 17.2 测量的两种证据作用域

原原生Measurement1的oracle只直接支持选定中央距离；旧原生对角SUE与完整fractional/cell协方差链仍是辅助/B03，不能借它宣称原件全协方差通过。当前MeasureImagesOutput明确含variance和σ，其有界Cartesian合同由operation-control-cases.json `/cases/1` 的.04Å²/.2Å直接设计控制，以及参数重排/交叉协方差例检验。两种说法分别标记在measurementCoverage中，不再写无条件“sigma remains supporting”。

### 17.3 source polygon到Mesh的几何投影

源oracle的faces是hkl/距离/面积的多边形平面记录，当前Mesh只有verticesA/triangles/normals，不能直接把faces改名后逐数组比。shape-projection-contract.json的`convex-local-boundary/1`把已封存cell/H⁻ᵀh、源vertices及平面投影成有序多边形；shape-reference-polygons.json绑定原JSON及只抽取的旧纯函数SHA，8相48面面积逐项与旧源值核对。未执行旧脚本的UI读取/写回顶层，也没有新原件观测。

对候选Mesh：核有限坐标、全半空间可行及边界顶点、每个非退化triangle恰落一个源平面且朝外；按平面投到2D后检查三角形内部不重叠、面积覆盖、边界分段覆盖，最后检查全Mesh每条索引边二次反向和源极点表示。允许不同顶点/triangle顺序、不同对角线及合法边界Steiner点；不要求原件的三角剖分次序。三角面积总和相同不足以通过，重叠加缺口的反例会失败。

坐标/平面阈值1e-7Å只继承旧独立重算脚本的去重/落面阈值，不是原件精度。面积阈值为本设计选定比较预算`1e-9+4*perimeterA*coordinateToleranceA`，不借用GUI显示epsilon，不扩为生产通用误差。shape主绑定是选定凸边界几何不变量，旧48项GUI显示面积仅辅助；完整原件拓扑、相R/t、一般非凸/近退化情形仍B09等未证明。

outputFields现在明示为展示标签，不假装每个散文不变量都是JSON路径；真正shape比较以projectionSpec的Mesh字段选择器及算法/源域为准。其它单元缺少结构化投影时不能据标签计成机器内容验收。

### 17.4 R6-R5与资料继任

proof-boundaries.json明确：一般半空间空/无界/降维分类，以及powder数组配对/单调/空采样拒绝套件仍not-machine-verified；已有合法输入的数值/边界投影例不能替代它们。字段损失身份新增设计例已覆盖，真实读取/编码/往返仍未验证。没有为补台账制造实验。

历史基线：R7追加rev32、R8追加rev33，各自封存保留；当时独审指针及待审状态只属对应轮次。R9/rev34施工时review_result指dsh R8、当时待独审；现已由dsh R9确认R8-R1–R3闭合，rev35回写真实dsh R9 JSON及其SHA。P1.2/P1.4/整体partial，P1.3 C4设计接受保持，C6增量未审，不进入产品或runtime。

I11身份准入补充：Engine必须按可信不可变对象表重新派生/核对完整inventory；不能只核调用方自报的stateDigest。即使phase存在，字段owner或描述不符合该fieldId的源对象也拒绝。trustedSnapshot只用于纸面观察夹具，不是客户端自签权限。

### 17.5 继承R8的交接一致性规则（I11协议及类型不变）

当前契约与复跑入口见[R9](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163802-p1-design-r9-doc-state/README.md)。该规则来自R8：当时同步currentTypeAuthority为protocol3，并使measure来源适配计划与measurementCoverage及原生/有界投影限制一致。R9逐字节继承这些契约，完整来源映射仍未建立。

当前shape oracle角色：`local-halfspace-mesh=result-invariant`；`area-crosscheck=supporting-only`。

前者按既有convex-local-boundary/1比较选定凸边界，后者只作显示面积辅助。该行与模块单元、JSON角色需交叉核验。schema、字段身份、shape投影例与R6-R5证明边界保留R7字节；实际资源读、writer、codec及往返/运行仍未验证。

## 18 R9当前文书准入与状态导航（I11语义不变）

当前机器契约位于[R9 contracts](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163802-p1-design-r9-doc-state/contracts)。新增科学请求使用`protocol=vestacraft.scientific.bounded/3`，对应113个有名定义、31领域/32操作；R6时点的protocol2/112定义仅供追溯。字段身份、损失同意、shape比较及其正反例均逐字节继承R8，不作新数值或运行通过声明。rev34/R9被审时点的next_card、整改结果和归档原件保留；当前rev35将next_card转为P1-01未施工，真实独审与非阻塞登记见文首收口入口，rev32/33/34不回写。

### 18.1 R9独审闭合回写（rev35，接口语义不变）

R8-R1–R3已由[dsh R9](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-170522-p1-design-r9-dsh-review/review-result.json)在列明范围闭合；当前契约仍为原R9/protocol3，113定义、32操作、31领域，完整资格未冻结。M1短式协议覆盖、M8历史正文机器化等八条只登记[非阻塞续办](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/非阻塞续办.md)，不改变当前类型或使01等待新实验。


## 19. P1-01公共类型提案（I12，SG/1 partial）

消费[新契约](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/contracts/structure-geometry.contract.json)、[字段绑定](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/source-field-bindings.json)、[原生字段span](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/native-block-bindings.json)及[完整语义](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/design.md)；旧R9/contracts保持原字节。SGCovariance是参数aware提议，不允许塞进原CartesianPointCovariance输入；缺失covariance非0，variance=J C Jᵀ、sigma=√variance。SGPeriodic3只代表明确3D周期结构，未给调制/超空间冒造类型；完整域仍partial。SGCamera canonical与SGNativeCameraFields分开，原shift/FOV适配未知。消息协议仍vestacraft.scientific.bounded/3；SG/1不注册可执行命令。源缺口与对02–05影响见本卡remaining-issues/shared-type-impact。


## 20. SG/1-r2当前修补边界（I13）

当前[契约](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/contracts/structure-geometry.contract.json)与[语义义务](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/contracts/structure-geometry.semantics.json)替代SG/1前稿中对应的variance/covariance/RGBA范围；type名不变，wire仍vestacraft.scientific.bounded/3，未注册可执行能力。negativevariance/空supplied covariance/RGBA越界在schema层拒绝；方阵/PSD/对角来源/far>near/variance-sigma必须在语义层拒绝。原生fractional/cell链仍SG-03，不因有设计谓词声称已实现。旧角色明确继承R9 domain-handoff的comparisonBindings，冲突flattened表不作角色准入；16条原角色不变。新旧入口和精确source定位见[修补说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/correction-notes.md)，对02–05影响见[类型影响](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/shared-type-impact.json)。


## 当前续办：P1-01 F1–F6独审结论登记（rev39）

2026-11-14 P1-01同卡独审结论登记：追加资料rev39，技术版本仍D11/I13/C6；dsh已独立复审通过rev38的F1–F6证据/边界修补，最新独审指针指向该被审包及真实JSON/SHA。本次rev39仅回写结论并Codex自验，O1登记为非阻断报告导航说明，不改旧封存或AGENTS。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受，SG-01…SG-06六项科学缺口不变。02–06未启动，产品和运行验证未授权。

最新[dsh独审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/REVIEW.md) / [真实JSON](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/review-result.json) / [本次登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/RECEIPT.md) / [O1非阻断说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/nonblocking-observations.json)。

O1：旧oracle-bindings.currentVerificationEvidence定位04-full正常模式546项报告；封存只读final为570项，两者均exit0/0失败。final入口为[/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json)。本次只登记该导航边界，不改旧字段、旧报告或封存。独审被审版本是rev38，rev39仅为结论回写且尚无新的独立复审。


## P1-05资产与外算交接增量（I14；protocol3及SG/1-r2不变）

[AB/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/contracts/asset-inputs.json)消费旧SGTableIdentity七字段，明确resource整源SHA、tableContentDigest所选raw span/face SHA、tableKey源内定位、sourceVersion/edition及licenseResource。未标版本与未知setting/origin如实保留，不能靠字符串schema通过升为可用科学表。A05 Ro/B成对、同离子多候选/氧化9/unchecked不隐式选首行；A09缺精确同位素质量，不以massNumber代替u。A11–13分别可提供font-face身份，cmap非零不证明排版、字体就绪、IME或导出。

[EX/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/external-tools.contract.json)补§6和ExternalCandidate：jobId、kind、baseDocument/baseEpoch/baseRevision、phaseId、输入资源、tool/asset/manifest摘要、profileId、授权及预算全部绑定可信队列快照。启动→子退出→完整解析→独立科学检查→候选→当前Engine正式提交→保存ack分开；迟到/取消/超时/旧身份/缺产物拒绝，原稿保留，不直写.vesta。未知完成查询receipt再决定，不盲重跑；TERM/grace/KILL仅拥有进程树，完整归属/退出/清理留证。

TIDY给选定P1 grammar及0/29/0/29旧子返回，坏群exit0无ATOM仍失败；一般群/setting未冻结。MADEL旧pre-main缺库-6不算零能量；完整PME/输出/能量归一与Ewald预期仍缺。RIETAN18argv顺序固定为脚本证据，实际方向/必需产物须按已资格化版本和模式，内置粉末或ins导出不能冒称外算完成。ORFFE是已证FFE读取/链接路径，未发现producer进程调用；不制造通用ORFFE argv。其它绘图/OS协作入口分别保留。

每工具publicOptionsSchema仍null且productionEligible=false，表示该科学profile未公开冻结，不能注册自由options JSON能力。新合同是P1-05设计补充，未修改原SG1-r2或R9schema、未实现validator/外算/producer。完整关键缺件见[P05-01至10](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/remaining-issues.json)。


## P1-05独审登记与说明后继（rev41，技术语义不变）

2026-11-14 P1-05独审登记及非阻断勘误：追加rev41，技术版本D12/I14/C6不变；dsh确认被审rev40 AB/1、EX/1、BT/1的partial设计/来源/保全边界准确、无阻断。本轮以独立后继修O1–O4，补未施工卡名单和fontTools复现环境说明，仅Codex自验、说明增量未独审；原施工包、dsh报告及rev40封存不改。05/01继续partial，SG-01…SG-06开放，C4接受/C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。02/03/04/06未施工；不启动下一卡、产品或真实运行，交回后停止。

被审原稿与SHA见[登记对象](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/review-registration.json)；当前定位/粒度补充为[AB/1-notes-r1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/contracts/asset-inputs.json)、[EX/1-notes-r1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/external-tools.contract.json)及[消费说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/consumer-handoff.md)，BT/1仍消费原被审字节。原17类资产对象、四外算科学/进程规则、build目标、protocol3/SG1-r2及科学缺件未变。新说明仅作者自验，不把本次独审扩成全部D12/I14、C6、产品或运行接受。未施工卡精确为02/03/04/06；05已部分施工并获得设计边界独审。原dsh机器组名与默认Python缺fontTools的复现差异分别在登记/复现说明中保留，不改原报告。


## P1-02 Field与科学设计profile（I16，FS/1 evidence-r1 partial）

[FS/1合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json)继承原152定义并新增25定义；[强制语义](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/semantics.json)与[完整设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/design.md)必须同读。FSFieldBinding由原FieldHandle加可信snapshot/descriptorDigest/IEEE编码组成；FSProvenance保留job原base；输出候选表与输入表分开，正式提交后才由Engine重绑输出revision，取消不能抹掉已提交结果。

FSSpectrumInput/Result明确有限系数、独立DC、Hermitian/偶奇Nyquist/别名和V/N归一；FSSumInput/ReflectionResult只消费显式image/复振幅，不代替表/全反射生成；FSProjection为显式长度权重求积，FSLine/Slice结果保留完整几何/全数组；FSPeakResult区分detected与参与盆地selected集合；FSConversionResult分finite-field与reference-nonfinite诊断；FSNoActionResult不制造数值积分。非周期布局未算积分时成对null，已声明生成器的积分验收不得省略。

[01/03/04/05/06迁移影响](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/shared-type-impact.json)说明新增FS结构不得作为额外字段静默通过protocol3封闭Request；未来公开须显式schema/版本/注册和损失同意审查。[来源字段](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/source-field-bindings.json)保留8组选定映射/6组未建立完整映射；[预期角色](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/oracle-bindings.json)沿R9权威。原参数/许可/完整网格拓扑等缺口仍partial，当前不是全ScientificRequest冻结。


## 当前接续：02同卡O1–O4（rev43）

2026-11-15 P1-02同卡O1–O4整改交回：追加rev43，D14/I16/C6，FS/1 evidence-r1；补35个真实投影合同有限例、澄清Patterson平方单位、固定Xcode3.9.6和11个旧WordCraft来源快照并披露2处漂移、将生命周期4个事件例移出准入证明。177定义/协议字节、11项本卡缺口及SG六缺口不改。dsh旧结论只覆盖rev42 partial设计/来源/保全边界，本轮整改增量仅Codex自验、待用户手动转dsh。01/05原partial与接受范围保持；C4接受/C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。03/04/06未施工，无产品或真实运行授权；交回后停止。

[本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/RECEIPT.md) / [整改说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/correction-notes.md) / [复现与漂移](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/reproduction-notes.md) / [下游边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/consumer-handoff.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/DSH-REVIEW-HANDOFF.md)。

本次授权为02同卡整改，沿用原会话/预算/权限/费用/次数/停报；不自动开新卡或扩大实现。投影helper只证明选定精确几何的schema/有限语义/资源域准入，未证明一般法向容差、插值/求积或完整科学能力。83例不含4个生命周期事件序列，7个output例仍是静态对象关系；租约释放、迟到原base/provenance及真实竞态均留未来04。复现使用新包reproduce.sh固定Xcode3.9.6，错误解释器exit2；WordCraft11旧Git blob锁SHA，当前两处来源漂移另记，不回滚外部仓库。原FS/1及dsh/旧复核/基线全部保全。


## 历史：21 P1-03 FC/1格式字段、选项与读写合同（I17/rev44，partial被审范围）

当前入口：[format-profiles](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/contracts/format-profiles.json)、[codec-field-policies](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/contracts/codec-field-policies.json)、[encode-resource-contract](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/contracts/encode-resource-contract.json)、[roundtrip-assertions](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/contracts/roundtrip-assertions.json)、[remaining](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/remaining-issues.json)。上述覆盖本轮格式设计的有界审计；旧§12/15–17的57未审/28选定是历史快照，原注册身份、四个首链空schema和81未准入边界仍保持。新的291源字段组不是291项已实现能力。

85路由分别按原键+方向+purpose+profile判定，不按Chem3D名称子串走m3d。81候选Schema提供显式dialect/参数类型和来源/消费方/最小缺件，但不进入可接收联合；未知ID/方向/用途E_SCHEMA，已知未冻E_RULE_UNFROZEN。source-audit逐路由锁原行SHA/Pointer、原始wire及独立内容预期；保留raw不得称可编辑或可计算。m3d量化/PRI非零flag、DEN/STO、Elk/ASSE/GAMESS等仍各停对应冻结。

PlanEncode→LossReport→Consent→EncodeApproved沿既有schema：Engine同稿全文核身份与资源，Plan无编码/文件副作用；field loss绑objectKind=field、可信owner和fieldId及相对Field的RFC6901指针。共享bytes的两个场分列，不靠Resource.digest或行序推ID。报告完整objectKind/purpose参与原canonical UTF8 SHA；旧epoch/revision/report/protocol2 consent拒绝。完整非field字段审计和SGGraph→旧EncodeInventory桥未齐时拒绝完整规划，不把控制性guard当真实Engine/ResourcePort证明。

sourcefile字节SHA、解码数组SHA、FieldHandle、descriptorDigest、logicalPath分别处理；缺资源保持原文+诊断/可选只读，不制造空场。目录沿§5提案，原件相对布局与04host承诺尚未接受；actualsafeFS/lease/配额/竞态由04。取消只释放本job所有权，宿主已发送后结果可能indeterminate；迟到旧快照不能清新稿dirty，export不重绑，编码不等于Saved。

85条future断言规定真实软件写出→独立读取实际内容/精度/损失→新实例重开同作品；导入only不凭空新增writer，image/mesh/外部输入用各自合格消费者。2D/Text另列辅助合同，不新增原85路由。未实施产品，无真实往返。SG/FS/AB及C4/C5/C6、P1partial和原预算不因本节改变。


## 22 当前P1-03证据范围与直接输入绑定（I18，evidence-r1）

当前[format-profiles](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/contracts/format-profiles.json)、[字段清单](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/contracts/codec-field-policies.json)、[读入/往返提案](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/contracts/roundtrip-assertions.json)为原85身份的证据后继，schema/wire/177公共类型不变。291组统一标route-level-inventory，individualSemanticValidation=not-established；原先“字段位置取具体span”不作已完成声明，完整字段定位/语义/精度仍缺。

I01直接绑VS0001/0002原生.vesta与其注册预期，I06绑VS0009/0070 CIF，MCIF绑VS0074/0075并保留其“完整展开磁矩未证”范围。I01数字预算不再用CIF作为原生源；全部write/reopen仍是未来。RAWwidth/height仅为设计所需导出缓冲元数据，非原件UI选项，须与实际payload相符；81candidate不进入准入联合。

有限admit_encode仅证明已列loss的身份/指针、source/owner、freshconsent和与控制trustedPlan的一致性，不证明PlanEncode完整枚举droppedFields或unsupportedBlocks。完整性见证明确：请求漏一场而trustedPlan完整则E_PLAN；两者控制性地同漏且重绑摘要时仍接受，不能以此宣称完整审计或真实产品漏洞。该未证项在[proof-boundaries](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/proof-boundaries.json)和remaining-issues开放；helper字节未改，未补产品codec。

用户转交的rev44复审原文保留6项标签/实列7项差异，411计数不采纳；真实被审范围与当前整改未审分列。资源目录、04host、SG/FS/AB、C4/C5/C6及P1partial原边界保持，见[相邻消费](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/consumer-handoff.md)。


## FC/1 evidence-r2具体消费（I19 / rev47）

本轮field-evidence以SHA+byteOffset/Length+原词法/位及具体字段locator绑定84组的91处组件；19个独立数值例不外推完整291组。typed选项来源/互斥/默认/消费方/失败见parameter-qualification，16路由有选定域细化，全部81候选仍不得进入已准入联合。m3d旧direct-f32候选错误，真实UI为direct32bit integer；改名direct-i32不补位打包资格。PRI只flag0/little32的现有三个例；不支持非零flag或别的ABI。PDB坐标8.3、B6.2、U/B显式选择、二进制32和MMP n-1各有独立规则，历史最大误差不作通用容差。

Plan/LossReport完整性候选须独立从全源枚举，不能将trustedPlan共同漏项当证明。当前64叶人工完整源能拒绝漏第二个共享资源场、新增源场旧报告、未知key及伪造保留值；它没有实现Engine/codec或修改当前LossReport/Consent schema。真实完整源/报告版本迁移未冻结时E_RULE_UNFROZEN。

IMPORT_DENSITY真实相对名、scale及实际pgrid SHA已静态绑定，同时原registry的恢复未解决失败保留。默认.vesta加资源裁定不变；拟定hash嵌套路径仍未获原件兼容证明。ResourceReader只消费明确logicalPath映射；缺件/摘要错/别名拒绝，安全根/真实lease/配额/竞争/ack/恢复仍由04证明。


### 03续办工作稿：资源捕获与语义重开（未封存、未准入）

本段消费本轮真实失败：8³原网格直接加载/原件保存阳性，实际工程和资源SHA及逻辑相对引用可核，但新原件进程未恢复体数据。当前§5/§8.2的字节、资源和精确快照确认仍必要；完整保存还必须独立证明预期对象确实被重建。只有主SHA/资源SHA/文件存在或exit0不足以返回Saved。

具体设计见[资源保存/重开合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/resource-reopen.md)、[闭合合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/resource-reopen.contract.json)和[未准入schema](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/resource-reopen.schema.json)。它们是当前03草案，不扩展已接受protocol3或悄悄向原闭合消息添加字段；完整实施须显式版本化准入。

拟议`decodeCaptured(primary,profile,CapturedInputContext{reader,validatedBindings})`先验证主文件SHA/真实路径范围或隐式seed及规则，再使用安全canonicalKey调用既有ResourceReader。旧源`../../inputs/source.pgrid`是可在实际本人授权根内捕获的原路径事实，不是核心可直接resolve的安全逻辑键；新便携输出仍不允许`..`。源主记录、请求/SourceDescriptor上下文和Engine资源都要独立枚举并绑定，不能因路径不在文件文本里而漏掉。未知块依赖闭合未证时只读保留/拒绝破坏性写回，不构造全零场或覆盖当前稿。

PlanEncode将由完整源独立生成的预期对象/owner/数据/单位坐标/保留源账本与比较规则固定到原Engine-issued planId及既有LossReport/Consent；修改集合或规则必须新计划和新同意。编码候选与主文件落盘读回分别进行artifact-only的一次性语义解码/核验，禁止从当前Engine、旧ResourceId、原路径、CWD或缓存补缺。新实例runtime ID可变化，但来源到输出对象必须有双射，两个owner不能因为共享bytes合并。缺预期Field即E_REOPEN_COVERAGE，不能以全SHA相同覆盖该错误。

Saved须同时满足实际宿主持久提交、相同字节集合上的内部可信语义核验及精确捕获快照确认。主文件可能发布后的错误保留committed-unverified/unknown、稿/dirty、旧资源、新文件和诊断；查询必须重核语义，不能盲重放/回滚/重绑/假关闭。受控关闭只在最新稿相符Saved后释放；export确认副本，不清工程dirty或重绑WorkRef。

48个输入路由不得虚构各自encoder：先证明导入内容，再仅在独立OBS-O01有能力保存完整结果时走原生持久化分支；不满足则保留明确阻塞。1个原生保存、22个数据导出、14个图像输出按各自内容和用途验证，导出不是完整工程保存。85条未来断言已按这些路径分开，均未被标为产品往返完成。

一次性解码上下文不要求每次保存启动原VESTA/新OS进程；真实独立读者/新实例资格仍属后续具名profile实现验收。原件实际失败保持，04/06未启动，真实Engine/宿主未实现；03整卡仍待其余卡面工作完成后一并提审。

03同卡资源后继草案进一步区分：编码前的SourceExpectation只固定完整源/对象/资源角色/布局/预算；编码后CandidateArtifactBinding才固定真实候选SHA和长度，不能反过来改源预期。精确内容继续用原exact分支；非精确内容须由[逐叶内容比较](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/resource-content-comparison.contract.json)完整绑定owner、pointer、单位、数量、结构分区与先验PrecisionBudget，原protocol3不改。

[递归/可选资源图](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/resource-graph.contract.json)按每个引用主文件独立解析，区分递归解码与源配对，RIETAN同名FFE的存在/缺失须进入捕获快照；不从“RIETAN写.vesta”推断VESTA普通加载扫描.ins。[完整源保全草案](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-source-envelope.contract.json)延续.vesta及<leaf>.resources/blobs和manifests/<实际主SHA>.json布局，显式提出sourceSHA头标记、SavedSourceDocument和manifest2，以独立保存完整源而非仅hash清单。它不是已接受manifest1的暗加字段，也不是公开跨重开稳定ObjectId：重载换document/epoch/runtime资源，只按验证后的本地saved-key双射还原。新标记/源树的原件互操作、实际typed恢复、产品实现及宿主资格均未通过；不足时拒绝完整保存/加载而保留原稿。

对应请求显式升为未准入FC-request-proposal-3，原生decode/save的preservationProfile必须与实际头和完整保全政策相符，原proposal2工作快照另存；原4旧空子集/81未准入不变。当前仍03整卡施工，全部新增值与此前已审范围分开，未启动04/06。
# 03当前恢复与确认后继（未封存）

03当前资源后继还消费completion工作目录的typed-source-restore.contract.json与save-settlement.contract.json：完整类型角色、全新生命周期映射和实际语义对象覆盖，与候选语义、宿主操作、持久语义的相同源/产物/目标/规则绑定分别必须成立。R7完成不清R8，旧document/epoch只结旧ticket；export不重绑、清dirty或关工程；关闭需先静止编辑并等当前精确修订。未知结果只能查询既有操作及实际字节/语义，不能盲重写或承诺回滚。已批准不可变快照任务的后到完成仅结捕获修订，不能替陈旧新提交豁免重计划。source-envelope标记移位或重复不得降级legacy丢弃保全。上述仍为03未准入proposal3请求/manifest2设计与条件有限见证，原native资源重开仍失败，实际Engine/宿主/认证未实现，04/06未启动。

03未封存源模型后继现为CodecSourceDocument-proposal-4：必须包含sections数组，每项有独立section身份、实际phase/source-entity所有者、完整native-section-source记录、被消费场引用、资格/描述符和源元数据。Owner增加section，SceneNode增加section-instance；不将截面等同全局SCENE、相或共享bytes场。完整源/资源/新生命周期遍历及未来语义重开须比较全部截面集合，缺活动截面不能由完整archive摘要抵消。旧proposal3在completion/pre-section-source-model按SHA保留，不静默接纳为完整当前快照。resource-reopen和source-artifact内部提案2显式纳入section对象种类；请求提案仍3，现行产品protocol3、旧177定义及SG/FS partial不变。选定布局/作者例不证明全模式几何、非空原件PLN2D保存或实际Engine恢复；04/06未启动。

03当前未封存SourceDescriptor后继显式为proposal2，定义资源仍经请求proposal3的实际资源/SHA绑定；descriptor1前态保持，旧1须有冻结迁移或source-only/拒绝。封闭单位表补至35，source-unit-domains.contract.json区分U/B/beta、张量基底与四种FS密度/Patterson量，不凭同量纲授科学资格。SourceDTO4的Field数据消费既有FS绑定的IEEE754 little-endian，storedShape=N+closure、closure⇒periodic、实际全部bytes/有限性及每个面/棱/角一致性都须在规划前和重开后核验；裸legacy Field无编码证明不能猜本机字节序。核心512与闭合存储729不可混为同一计数，原始wire资源独立保存。产品protocol3及02/SG/C6/P1原partial不变，04/06未启动。

03当前向量后继消费completion/native-vector-format-contract.json：逐向量的所有者、类型、分量、全部位点/运算关联及半径/RGB/样式都是持久内容，VECTR末列不是坐标模式。源元数据与科学投影分开；未知物理基底不能造SGVector。源/损失/重开覆盖集须保留多个同值向量及其不同身份，不能用坐标展开通过替代缺失磁矩。临时对象Polar置位未清和样式高位覆盖类型仅有静态机制证据，原P1缺第二磁矩的真实失败保持。

03资源运算保存政策消费completion/density-resource-effect.contract.json：默认捕获当前已完成Field的完整不可变值与描述符，作为独立物化资源；原始输入、顺序、运算、分量规则和依赖另行完整保留。规划与普通重开不隐式重算表达式、不依赖活源路径、不将原数据顶替变换结果，也不重复施加系数/幂。经显式资格的源重放是另一条路径；完整Saved还需同一源/产物/目标身份下的全值和元数据读回。02科学执行、powf跨平台资格、原件元数据互操作及实际Engine/04实现没有因此建立，04/06未启动。

03当前未准入解析结果补failed-before-apply，区分预算/能力/规则不足与已核实invalid-source。completion/parse-budget.contract.json规定48条decode现有五项限额跨递归资源、源树和所有语义场累计，包含stored闭合/分量，checked计数先于分配；子读取器不可重置或自提额度。未完整读取时不得称完整source-only保全，候选/持久验证须用独立注册并绑定同源/产物的限额。该设计不暗加产品protocol3结果或宿主quota承诺；04/06仍未施工。

03未封存源模型现显式后继为CodecSourceDocument-proposal-5，补bondRules/boundaries/planes三个独立根，不能由phaseGraph或SGDisplay替代；sg-persistent-source.contract.json沿现行SGReviewBundle八类持久类型逐项消费，仍不称ReviewBundle穷尽真实Engine状态。源对象id与内含SG值id在已知包装类型中是一个定义与类型化别名，须相等并只重映射一次，SG身份仍遵守现有32位小写hex域。每条边界保留所属相及全部规则引用，平面保留源几何和local/world值；源元数据/原生搜索规则与派生键/渲染mesh分开。旧4/2的8份真实前态另存；旧4不得靠补空根静默升级，实际原源缺项须有独立迁移依据。资源重开/产物内部提案3纳入全部已知对象种类；请求3/描述符2/manifest2/现行产品protocol3与SG/FS partial分别保持。原件实际资源重开仍失败，无04/06施工。

03当前源后继继续显式为CodecSourceDocument-proposal-6，仍22必需根；bondRules现在是SG pair包装或NativeBondSource的闭合联合。原件mode1只指定A1、mode2不指定任一端点，现行SG site/element pair不能冒充这些模式。NativeBondSource按NBOND1完整保留原生15字段、独立32hex id/真实owner和任意已资格SG投影的类型化引用；SG包装保留内含id别名，原生记录没有伪造value.id。界面改写XX/清标志不能覆盖原输入源值；算法/递归/边界等价必须独立证明。旧5/3的8前态另保留且与253输入一致；不改资源/产物3、请求3、descriptor2、manifest2、产品protocol3或上游科学资格。没有新增原件运行、04/06施工或整卡接受。

03当前未封存合同又补出原生路径组装的条件静态反例：目标字符串与末输入别名，辅助函数只追加而未清空，不能得到要求的“操作＋空格＋路径”。纠正设计以不可变操作/路径和不同的空目标或类型化句柄交接；新鲜语义读回的resolvedReferenceGraphDigest须由实际资源读取和完成的对象关联生成，不能回显计划图。见本轮native-resource-path-assembly.contract.json，真实原件失败及缺少实际打开路径/唯一根因追踪保持。原生六种键样式和SBOND半径/线宽与BONDP反序另有精确源绑定；不以此授予渲染等价。VM18-I29补两版FILE06报告的三行/三观察量设计和显式源块选择，不能与FILE07重启文件或热参数合并；请求3的当前226属性仍为未封存提案。旧接受边界、Source6/资源3/manifest2/产品protocol3、04/06未施工均保持。

03历史源记录前态以NVIS1/NativeObjectFlags-proposal-1封闭五类phase持久标签/禁用源记录，置于既有SourceMetadata.tree，不增加SourceDTO6根。原生零基生成序号不是SG持久id；原子映射既有Image及SGDisplay.hiddenImages，键、多面体、形貌面各绑定自己的源定义/目标和枚举依据。LBLSP指生成形貌面，SPLAN是不同的切片源列表。类型化ID须作EntityRef并核完整源定义/真实所有者；schema版本或自称qualified不替代可信枚举上下文/映射。新实例须比较全部源状态和实际适用标签/可见性，并从同一不可变主文件捕获及应用，不能另开环境路径或回显计划图。完整原生生成、SG科学及Engine恢复仍未因此接受；不启动04/06。


03当前未封存源合同以CodecSourceDocument-proposal-7为准，前文proposal3–6均是保留的历史演进。22个必需根不变；SectionSource.nativeRecord显式联合未改的现代18行/80值与独立选定旧15行/69值记录。旧作者工程名义3.0头不足判定布局；旧记录四套颜色与现代六套不同，不补行、补颜色或按行号静默转换。16份实际Source6前态独立保留；迁移须证明实际旧内容/身份/当前状态完整，不能靠补空数组恢复遗漏内容。来源字节和typed记录完整不自动证明活动截面已重建；真实新实例必须覆盖完整预期section集合、owner/field/mode/basis/matrix/colors及资格。未执行实际Engine或新原件核验，仍不启动04/06。

当前持久标签/禁用状态采用NVIS2/NativeFlagState-proposal-2，NVIS1只保留原始源记录前态。不可变来源观察与五套完整当前集合分别捕获；用户编辑/删除后不能从原始列表推回当前状态。原生序号必须绑定可信有序源population，不能只验证摘要或另一个真实对象存在；当前稳定目标须按实际输出population重映射。周期目标用实际Image/规则/半空间，非周期目标用已有Molecule/MoleculeFrame/Atom/Bond身份；latent状态须有独立适用性依据，不能自报潜伏掩盖恢复缺件。完整保存/重开分别核原始来源保全和当前适用效果，裸raw列表、全局STYLE、archive摘要都不能替代。资源/产物内部3、请求3、描述符2、manifest2与现行产品protocol3分别保持；旧接受和SG/FS/C6/P1 partial边界不变。

03当前未封存EB数值消费后继（2026-11-17）见completion目录的`native-numerical-consumption.contract.json`。原生47/46共用单标量读者；其等值面输入消费存储f32数值，不读取能量单位、偏移或费米能级。物理信息缺失不能笼统阻断原版按存储数值显示，但仍不能猜eV/Ry、偏移0或授予物理费米面资格。导入到现有模型的晶格选择、当前有效仿射变换与等值面数值都须保存，原始文件不能代替这些当前状态。

Source7的开放SourceEntity/AuditNode域新增闭合`NativeFractionalGridCurrent1`和`NativeStoredScalarIsoview1`设计记录：前者保存实际未掩码f32全缓冲、N点/N-1分段与实际3×4 binary64分数坐标到显示坐标矩阵；后者以实际EntityRef绑定同一当前对象，保存存储f32等值数和正/负/双面模式。三个闭合schema分别限制当前记录、无引用frame及view，具体owner/dataRef/SceneNode角色及Descriptor2实际定义、owner/use上下文沿原完整源合同消费；不能以普通字符串、同SHA资源或旧别名替代。数值坐标按原生先求1/(N-1)再乘索引，再用实际仿射及平移，不能以代数等价就声称IEEE逐位等价。

不可变`NativeScalarGridSource1`原输入历史、已有可带mask/f64的`NativeScalarGridCurrent1`仿射步长对象和新增原生分数网格对象是不同持久类型，不互相静默迁移，更不接管实际FS/倒易Field。全部当前样本、坐标参数、实际view和原历史须独立捕获并在新生命周期重建/比较；原输入回放、遗漏平移/负面、猜费米0不能通过完整保存审计。原生完整周期显示扩展、cutoff/其他场景字段继续完整审计；本有限存储域的输入合同不冒充完整mesh/renderer或物理Brillouin域证明。后续实际Engine/renderer及其原件对照是独立施工验收，不作为03尚未编码的阻塞理由。Source7/Descriptor2/request3/Graph2/resource3/manifest2/loss2及产品protocol3版本、原4子集/81未准入及SG/FS/C6原partial均保持。
## 2026-11-17 · 03当前补充：旧数据库实际消费、自旋图表与首次保存

以下是03同卡施工后继，精确当前合同位于[03工作目录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion)。未改变产品protocol3、旧封存47或其他卡接受状态，也不表示产品已实现。

`historical-native-record-consumption.contract.json`补齐锁定读者的ICSD TF分支：六个系数按独立热参数序号写到当前槽，随后跳过一个非空非注释物理行；原标签、被跳过行与当前位点身份分别保存，不能猜成另一个张量或已定SU。CRYSTIN完整数值分派只消费十个记录码，原生忽略SMAT、Bij和条目分隔记录，不能把它们包装为已计算群或张量。新增请求方言明确限于所选原生legacy消费，原有选定修正方言及其资格边界独立保留；历史1986/1991手册未取得不再等于这些原生布局仍未知。

`gsas-original-spin-chart.contract.json`依据原作者APS r1253 POWPREF真实字节，区分十二个SPNFLP生成元/中心输入与完整操作集。可信、来源版本/相/setting/origin绑定的图表给每个实际操作计算tau；SG的timeSign取tau，轴向变换另施det(R)，不能重复乘行列式。闭合源图表定义可无引用保存；当前`GSASSpinProjection1`必须绑定真实相、全部实际操作EntityRef和本次拥有的chart BytesRef。新生命周期重建逐操作精确比旋转、平移和timeSign；实际编辑状态不得被旧导入标志覆盖。真实图表factory、完整SG科学资格和产品实现仍由后续对应施工验证承担。

`save-target-policy.contract.json`消费现行PRD §10.10/D4-31：已有工程Save回原WorkRef；新工程有合法指定路径时优先，否则用当前会话工作区，缺位置/名称先进入选择状态，尚不编码。切换软件页签和后续工作区变动不得重定向已捕获操作；导出独立选择目的地且不自动下载。

此前保存说明中仅列SaveAs重绑的措辞在此补齐：未绑定新工程的首次Save，在同document/epoch且真实提交及完整语义读回通过后，安装该回执实际WorkRef。若当前已到R8，R7保存仍可登记其实际路径，但R8保持dirty且不能关闭；旧epoch/其他稿、失败或未知结果不能绑定。已有工程的普通Save保留原目标，export不重绑。该角色绑定进既有内部targetDigest，不新增客户端确认权限或静默修改Receipt/protocol3。实际写入路径须可从本人空间文件列表找到并以完整内容重开，路径字符串或SHA相同不足Saved。
### 当前多模型定位与原始来源的独立保存（03施工后继）

`native-layer-source.schema.json`原有Source1形状继续表示原始来源；其中kind/id字符串是历史别名，不能当作跨新实例仍有效的活动引用。新增`native-layer-current.contract.json`与闭合Current1形状，逐项捕获实际完整、有序的当前原生3D实例：真实phase/frame/source-entity定义、位置与取向两套独立EntityRef、全部18个锚/方向f64值、完整32位当前flag、实际16个矩阵f64值及3个平移f32值。分子包含多帧时必须由真实实例映射确定frame，不能用moleculeId或默认frame1替代。

当前记录由文档拥有并按`NativeLayerCurrent1`写入Source7既有元数据类型；EntityRef及IEEE位保留各自角色，原始选择器/历史标签保持字面。新实例分别重绑两套引用和全部owner，原生输出序号从本次真实输出顺序重建。当前矩阵、SG位姿与SceneNode的真实显示状态也分别捕获，不能在Save中从旧来源重放，不能将一套矩阵重复应用。原生四个打印flag或一套SG parent均不能替代完整当前状态。该补充不授物理flag、高维科学、实际Engine捕获或原件互操作资格。


## P1-03整卡读写设计后继（I20，待独审）

本节和[整卡设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/design.md)是03本次精确消费入口，取代旧rev44–47关于本卡自身“全部未知/参数仅shape”的笼统施工状态；原封存和科学/资产/运行资格不改。三个继任合同分别为[profile](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/contracts/format-profiles.json)、[字段](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/contracts/codec-field-policies.json)、[未来完整内容断言](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/contracts/roundtrip-assertions.json)。每条selectedParameterDesign/selectedFieldDesign有精确SHA/record绑定，变化必须重新冻结；历史4空/81准入不充当新协议接受。

Source7及AuditNode注册实际当前NativeLegacyDisplayCurrent1/NativeLegacyViewCurrent1、NativeThermalState1、NativeLayerCurrent1、完整数值网格/显式存储等值视图、截面及既有类型；MXDReportBlockObservations1是独立原始观察。原始tokens/aliases和当前typed引用不同角色，不从源重放当前编辑；未表示的实际Engine字段/角色拒绝完整捕获。源存档不证明活动Field/科学投影成立。

PlanEncode在同一不可变完整源上独立枚举全部叶/对象/资源，先冻结units/frame、精度、固定codec/view/color/alpha/background政策，再列LossReport和新Consent。精确数值消费采用[有界decimal](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/decimal-consumption.contract.json)，完整ASCII token和可信job累计额度在扩张前检查；m3d预算和外层每叶预算同时满足。比较不是只看摘要、头/尾值或误差均值。

编码私有候选后，artifact-only全新上下文先恢复完整独立预期；宿主依赖先行/主文件最后发布，实际持久字节和语义读回都满足才发精确Saved。提交后未知/失败为committed-unverified，保留dirty；D4-31现有路径/显式新位置/会话workspace及首次Save绑定消费[目标政策](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/save-target-policy.contract.json)。导出不重绑工程/清dirty/自动下载。04适配和真实Engine/codec/新实例验证属于后续实施，原资源失败是必须保留的回归。

参数差异详见选定合同：STIN GUI19/native20的mode0不能抄P1的选项，恰一相且真实old-name/control；WIEN原生读停后不消费Precise尾段；RIETAN实际临时表示开关不能泛化8pi²；旧材质、TF及ignored records与实际当前类型分存。完整31关键词/16分派/十码域和source-only/错误不冒充额外producer或科学能力。


## P1-04 保存和应用内部合同后继（I21，partial待独审）

HS/1及其封闭值定义目标/能力摘要、source→actual-artifact分层、operation/query/close身份和生命周期；AL/1及其封闭值定义数据库请求/候选、Preferences2和HostAction字段。所有字段、单位、生产消费方、失败/取消及未证能力见合同，不用自由JSON或客户端passed授权。

03 proposal按准确URI/SHA消费；旧protocol3及177公共定义未增加字段。未来必须显式选择transport版本并协商HS/AL+03 schema摘要，生成类型/validators，更新server action allowlist、client bundle及消费者，拒绝旧版本/额外字段。本卡不准入产品消息。WorkRef旧形状不变，新source/target/intent元数据在内部registry。当前service没有Vesta/querySnapshot动作，不能称为现有API。

§5及历史§11.5的hash-only查询描述以本节和03settlement完整证明要求为准。不同document/epoch只结旧历史；R7回执不清不同内容R8；较旧bindingIntent不得重绑，较旧publicationSequence不得倒退lastSaved。新工程首次Save/SaveAs完整同生命周期确认才绑实际路径；Export不重绑/清dirty/自动下载。close先处理控件/IME及全部maypublish操作，稳定最新revision后释放。

完整设计、精确字段、来源与8项具名缺口见[04设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/design.md)、[HS/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/contracts/host-save.contract.json)、[AL/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/application-lifecycle.contract.json)、[源码绑定](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/host-source-bindings.json)、[remaining](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/remaining-issues.json)及[未来验收](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/verification-plan.md)。本轮仅Codex自验、待用户手动dsh；旧03/01/02/05/C6资格不升级。


## P1-04 同卡续办接口（I22，HS/2、AL/2，partial）

旧HS/1、AL/1文件及177公共定义不改。新[恢复合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/contracts/host-recovery.contract.json)、[页面投递合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/contracts/page-delivery.contract.json)、[应用合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/contracts/application.contract.json)和[封闭值](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/contracts/continuation-values.schema.json)是后继设计，消费时须同时读取父合同，明确冲突以本后继局部字段为准。所有新类型仅用于设计，不接收为现有protocol3消息；未来显式协商版本、服务器准入、客户端bundle和receiver一起迁移。

FormSubmission分四个表单，以descriptorDigest/formIndex和精确可编辑字段绑定；隐藏字段、endpoint和submit由来源描述符生产，客户端不能注入。文本保持原表单词法值，checkbox为布尔，未选项不发送；size不是输入上限。HostActionRequest按11种kind给封闭payload，回复按6种status约束effect/receipt；内部登记须匹配request/kind/payloadDigest和真实完成范围，字符串receipt不构成授权。RecoveryReply将原操作可信结算与新生命周期可恢复产物分开，未知不Saved。PageCommandKey含身份/代际/摘要；重复未知修改不能重放。全部字段和失败语义见上述合同及[演算](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/wire-examples.json)。

04同卡续办已补HS/2页面投递与恢复、AL/2真实表单及逐种系统动作设计。原8项均保留：4项缺设计输入，4项留待后续真实资格；整卡partial，待用户手动dsh。03 rev48整卡待审、01/02/05 partial、SG六/FS11、C4设计接受/C6未审、P1两接受两未接受保持。 [04续办回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/RECEIPT.md) / [续办设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/design.md) / [具体缺口](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/remaining-issues.json) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/DSH-REVIEW-HANDOFF.md)。


## P1-04 缺口设计后继（I23 / HS/3、AL/3）

[Host profile](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/contracts/host-profile.contract.json)绑定server principal/build/volume/writer-domain/metadata isolation/durability/实际limits与verifier；实际宿主产生和准入，客户端passed或hash不能授权。[恢复](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/contracts/operation-recovery.contract.json)固定host-issued capture代际与request绑定、不可变操作记录、只读query/principal-target守卫、失去凭据的新生命周期恢复、未知可达保留和终态退休。过期旧capture不重放，查询不自动重绑当前稿。

[偏好](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/contracts/preferences.contract.json)以changedFields+stagedValues+version/profileDigest整笔提交/独立读回；AL/3将数值草稿改为词法字符串，替代旧内部u64示例对未知native域的过早约束，**不是**允许任意负数/小数。native类型/精度/词法域须有合格证据；宿主安全上限独立。新profile/query/preference值在[封闭值](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/contracts/values.schema.json)，mask/权限/真正持久化仍有语义校核，有限schema不能替代它们。[历史/动画](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/contracts/history-animation.contract.json)补snapshot容量及单tick/session/pref/epoch/revision、资源leases和屏障。

以上仅设计后继，旧HS/1、HS/2、AL/1、AL/2及177定义原字节保持；必须同时消费父合同，局部差异按本后继明确段落覆盖。未来生产schema/服务器准入/receiver/client bundle须显式协商迁移，productprotocol3未准入。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/RECEIPT.md) / [完整设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/design.md) / [最少补件](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/external-inputs.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/DSH-REVIEW-HANDOFF.md)。


## P1-04 宿主范围后继（I24 / HS/4）

[HS/4](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/contracts/host-binding.contract.json)显式局部覆盖HS/3“所有外写者均受强制原子域才允许保存”的作者前置；按原卡选定same-service-controlled-writers+observed-baseline范围，不降低权限/配额/父sync/完整语义/真实registry资格。额外globalCAS请求E_HOST_SCOPE_UNSUPPORTED；source-bound不生成runtime-qualified。

新[封闭profile/采集值](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/contracts/values.schema.json)仅内部设计，旧HS/1～3及AL/1～3、177定义原字节不改，productprotocol3未准入。AL/3仅[evidence-r2](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/application-source-status.json)：定位Prefs内部OK/Cancel/Get/Set，复用已有MouseLeftUp条件timerStop，不填造native域/分组。未来schema/server admission/receiver/client须显式版本迁移。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/RECEIPT.md) / [设计与来源](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/design.md) / [最小原件取证方案](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/DSH-REVIEW-HANDOFF.md)。


## P1-04 静态来源后继（I25 / AL/3 evidence-r3）

[偏好证据合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/contracts/preferences-evidence.contract.json)和[历史证据合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/contracts/history-evidence.contract.json)继承AL/3并补逐地址观察，未改变旧Apply/Cancel、历史/revision或productprotocol3准入。ID8/14声明1～10000/1～1000000且step1，经已有spin构造/事件证据关联；完整字段语义、失焦/钳位/词法仍未证，不自动开放未知字段。

OnOK本体GetValue→Set→InitCanvas→5100，Cancel本体5101→关闭；外层与虚调用副作用未验证。Timer/Idle已读不等于原件历史链闭合；精确缺口和下一范围见本轮证据。动态仍未授权，不以静态推执行。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/RECEIPT.md) / [设计与静态事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/design.md) / [最小剩余范围](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/DSH-REVIEW-HANDOFF.md)。


## P1-04 偏好输入后继与历史新证据（I26）

[AL/4-preferences-input](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/contracts/preferences-input.contract.json)明确继任AL/3数字输入设计；原177定义/productprotocol3/旧AL3原字节不改，未来显式协商并一起迁移producer/schema/server/receiver/client。rawText+baseVersion+profile输入，服务端拥有资格，结果区分parsed-scalar/parsed-binary/kept-previous/unqualified/safety-rejected，候选不等于安装值。对12字段保留单位/默认/原生与安全资格；interval绑定i32，不按u64宣传完整原生域。

[AL3 history evidence-r4](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/contracts/history-evidence.contract.json)补timer入口及undo1/redo1成功门控，不证明int1是一快照或原件动画归组。保留焦点/IME、tick、save/close/Apply/Undo屏障及资源lease规则；未解虚表/库语义明确列出。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/design.md) / [最小剩余](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/DSH-REVIEW-HANDOFF.md)。


## P1-04 转换profile与历史证据后继（I27）

[AL/4输入evidence-r2](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/contracts/preferences-input.contract.json)形状继承；新增[数值转换profile](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/contracts/numeric-conversion-profile.json)，field/tag/profile由server绑定。已资格finite/inrange i32截断、f32明确舍入资格、f64复制；candidateConvertedValue仍待真实整代持久/读回/安装，不能把Printf输入或返回数宣布effective。旧177定义/productprotocol3不变，未来显式迁移。

[历史evidence-r5](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/contracts/history-evidence.contract.json)解析Canvas230/310/6f8静态目标为聚焦/刷新/交换缓冲，OnRotate固定轴按钮职责已定；没有追取其它函数、callee或运行轨迹，持续动画分组仍未证。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/design.md) / [最小剩余输入](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/DSH-REVIEW-HANDOFF.md)。


## P1-04 AL4历史范围与格式来源（I28）

> 历史rev56正文：作用范围、profile字段、聚合与handler口径由文末rev57整改后继覆盖；旧包保持封存，不以本历史段作为当前合同。

[偏好evidence-r3](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/contracts/preferences-input.contract.json)绑定%g/%.*f字节并保持候选/存储/显示/生效确认分离；全locale/FPCR/词法资格不自动提升。

[AL4-history-manipulation](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/contracts/history-evidence.contract.json)显式覆盖AL3泛化中“纯UI不记历史”对真实清选择提交的误用，保留纯focus/noop规则。新[闭合值](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/contracts/manipulation-history.schema.json)区分owner集合请求、逐Doc committed/unchanged/failed/unknown和集合complete/partial/unresolved/failed；server绑定route/profile/scope，client不得声明已资格或将单Doc命令扩集合。旧177定义/productprotocol3未改，未来需显式同步迁移，不是已实现/准入API。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/design.md) / [最小剩余证据](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/DSH-REVIEW-HANDOFF.md)。

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


## I31 · NS/1同生命周期CLI身份与保存衔接

本节是用户明确授权06补齐CLI契约后的有界后继：[规范设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/native-session-design.md)、[机器合同](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/contracts/native-session.contract.json)、[封闭Request/Response及具名资源schema](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/contracts/native-session.schema.json)、[producer/consumer与host权限边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/producer-consumer.json)。这些是本接口规范的精确组成部分，需一并读取，不是已实现API。

身份敏感续作采用一次 `vestacraft serve --stdio --root <host-derived mount>`：hello严格匹配native/core/save/host profile摘要；stage已授权真实字节→open一次返回实际doc/epoch→同Engine inspect取phase/siteId→携DocumentAt执行现有8个core命令→snapshot候选→既有host实际发布及完整独立读回→私有HostAck确认→settled close/shutdown。所有u64十进制字符串；顺序seq生成core requestId，不接受另外一个调用者requestId。

逐字节同seq只重放原Accepted/终态，淘汰后expired绝不重执行；contextVersion不符是准入后失败并消耗seq。status/cancel为OOB；EOF不隐式保存，新进程freshsid，旧工具不得换seq/新身份盲重放。资源、分页身份、负向/退出、关闭证明及捕获版本的精确规则以具名合同为准。

本节明确取代§6独立inspect进程的身份传给另一个run进程的错误编辑配方。one-shot commands/inspect可保留诊断，返回身份随进程失效；历史run示例不提供跨进程编辑入口，不按label/index/SHA重新猜对象。没有删除原件16CLI要求，没有改变现有command/1核心语义或bounded/3/177科学定义。

native无私有registry/host凭据或主文件直写权；只通过现有认证host的私有pipe/allowlist接控制帧。HostAck元数据不自行构成权威，必须源自HS3/4同op可信查询与新鲜全语义比较；export/旧revision确认不能清当前dirty。close必须结清全部可能发布的操作，dirty=false、exit0、文件存在或SHA相同均不够。新增NS schema生成器/driver/严格profile consumer升级和真实R01往返是未来独立施工/运行范围，当前未实施或获接受。


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

I32 是显式后继：[封闭科学请求/结果与状态编辑](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/contracts/scientific-composition.schema.json)共40个具名操作，SG11＋FS28＋唯一surface操作；没有free options或caller qualified标志。[NS/2](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/contracts/native-session-v2.schema.json)继承NS/1实际session/document/epoch/context/seq、资源传输、重试高水位、发布ack、关闭与新生命周期规则，新增 domain.prepare/domain.state.prepare/domain.apply，所有11条命令完成均带可信历史摘要。握手强制9项schema/catalog/资产/历史/偏好/外算等摘要与候选数量/字节/保留预算；旧generation执行前拒绝。候选只来自完整具名结果，绑定实际owner/source/revision/profile/digest，query不改变当前工程，apply只原子提交真实差异。[历史线协议](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/contracts/history-effects-wire.schema.json)以规范十进制u64字符串投影内部逻辑整数，禁止JS Number丢精度；source/current/candidate/hostack权限互不冒充。全部输入输出及生产消费精确引用在 [接口组合](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/interface-composition.json)。旧177定义、bounded protocol3、NS/1、C6原字节保持；迁移后重新做真实共享端往返，不能声称旧客户端已支持。

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
