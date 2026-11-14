# VESTA P1 领域、命令、页面和保存契约

2026-11-14，P1-I11。2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

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
5. 读回主SHA与全部依赖；仅全相符返回HostAck。若主已提交但读回/连接失败，状态是`indeterminate`，下一次按快照SHA+依赖复核，而不是返回已保存或直接回滚新文件。
6. ack携带`{snapshotId,documentId,epoch,revision,stateDigest,purpose,targetToken,actualPrimarySha256,verifiedFiles}`；页面匹配原快照，保存的是R7而当前R8则lastSaved=R7、dirty=true。切到新稿epoch后旧ack只结清原快照，不能动新稿。saveAs仅匹配同稿的成功ack后重绑目标，export永不重绑/清dirty。

单文件（无外部资源）同样走主文件真实确认及60秒自动保存。多文件自动保存仍用同一协议；避免把“单文件60秒已确认”扩大成实测大资源每60秒均可完成。快照进行中周期触发合并为最新待存revision，不重复排旧快照。受控关闭：阻止新编辑→等待当前job提交/取消→snapshot最新revision→Saved→release/交接；保存失败恢复页面编辑权/dirty，不假关页。硬关闭/进程崩溃后，只能确认已写主文件/资源，不保证未保存修改恢复。

具体失败断言（未来真实测试，不是结果）：R7 snapshot后site编辑为R8，ackR7不能clear dirty；A稿ack晚于B稿load，B状态完全不变；资源2写507时primary旧SHA不变；primary409时已写资源仅为orphan；saveAs失败path不变；exportPNG成功工程path/revision/dirty不变；丢ack但文件全SHA匹配通过status恢复Saved，若不匹配保持indeterminate和dirty。

## 6. CLI/外算与四种完成时刻

计划产品原生CLI为`vestacraft`，不是原件VESTA命令包装；首链接口：

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


## 16 I10／R6 当前有界契约与D1–D14修正

当前机器契约位于[R6 contracts](../../../output/vesta-study/20261108-113622/reviews/20261114-141647-p1-design-r6-corrections/contracts/)。§15及模块R3/R5旧签名已标历史；I8首链不变，新增科学请求必须`protocol=vestacraft.scientific.bounded/2`，不是把旧协议改名后接收。当前112个有名定义、31领域/32操作；领域数不是操作数上限。全部新操作仍productImplemented=false、fullLegacyQualification=false；完整P1继续partial。

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


## 17 I11／R7 当前字段身份、投影与基线勘误

本节覆盖§16中本轮修正的部分；其余I10有界合同与首链协议继续继承，未因此全域冻结。当前契约见[R7](../../../output/vesta-study/20261108-113622/reviews/20261114-152228-p1-design-r7-errata/contracts/)，113定义、31领域/32操作，所有新操作仍未实现。Field值形状不变；新增FieldHandle及loss objectKind使封闭消息采用`vestacraft.scientific.bounded/3`，不得无提示接受旧protocol2 inventory或旧损失同意。

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

追加rev32，status/status_correction与结构化current_status及当前P1状态统一，rev31按原SHA封存保留，旧顶层文字只作为明确historical记录。最新真实review_result指dsh R6原JSON；R7为作者勘误待手动独立复审，P1.2/P1.4/整体partial，P1.3 C4设计接受保持，C6增量未审。不进入产品或runtime。
