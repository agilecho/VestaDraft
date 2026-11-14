# VESTA 模块 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: 使用 `superpowers:executing-plans`，Codex直接逐任务实施；步骤使用复选框。用户已指定原生执行及dsh独立复审，不自动派子Agent、不交dsh施工。本文件本轮只编制，全部产品步骤未执行。

**Goal:** 在全功能1:1目标下固定模块依赖和可执行基础切片，同时把尚未冻结的相邻科学/codec/worker接口明确隔离为P1设计补证，不以部分计划替代全功能实施计划。

**Architecture:** Rust原创领域内核通过单一命令注册表供WASM Worker与原生CLI消费；TS/React/Three.js提供可编辑当前页面，宿主桥复用平台运输/个人空间。默认原生`.vesta`加资源，资源先写、主文件末提交，真实宿主确认对应快照。尚未冻结的高级科学payload和CA迁入接线先补设计证据。

**Tech Stack:** Rust1.99.1、wasm-bindgen0.2.130、serde1.0.229、RustFFT6.4.1；TS7.0.2、React19.3.1、Vite8.3.7、Three.js0.187.0；Playwright1.65.0。当次获取与实际可行性范围见设计§2，尚无产品锁文件/安装/构建。React DOM/types、字体/参数、Linux镜像与transitive锁由Task1/G-ASSET核实，不默认latest。

**Spec:** [架构](../specs/2026-11-10-vesta-p1-design.md)、[接口](../specs/2026-11-10-vesta-interfaces.md)、[CA接线](../specs/2026-11-10-vesta-ca-wiring.md)；执行者须同时读取。[functions.json](../../VESTA研究/functions.json)是唯一功能规格，本文任务引用不是另建覆盖矩阵。

2026-11-14 R7：D9/I11完成R6-R1–R4设计勘误，R6-R5未证明边界明确；Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev32；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

## Global Constraints

- 当前仅授权P1设计，交付后停止；本计划不授权产品编码/部署/模型builder或P2–P5。
- 全功能1:1、Deep、Codex执行和dsh独立复审保持；默认复现已证原件行为并提示科学/数据风险，修正须显式选择（DEC-P1-01）。默认`.vesta`＋资源（DEC-P1-02）。
- 计划产品workspace `/Users/agilecho/code/VestaCraft`，产品root为其`app/`，均尚不存在。Craft非Git，不初始化、不建worktree、不在共享平台施工。
- 后续取得实施授权且相应设计门通过后，才能执行对应Task。Task1–8描述可独立基础子集，不意味着P1.2/P1.4全部功能设计已完成。
- 文中所有代码/测试路径和命令均为**计划创建/未来执行**，不是现存文件或实际通过；需要先有工具链/锁文件/浏览器/资产/真实host实现。失败日志、每轮新目录、实际argv/cwd/退出/清理必须保留。
- 人和Agent共享当前engine；三视图只改变布局；同一作品同一时刻一个写lease。保存真实写入对应快照才ack，失败和晚ack不得清新编辑；无浏览器作品持久副本、不自动下载。
- 原件/共享CA/平台只读；CA方法锁1f8508a1，选用平台每卡重核；不读取秘密、不自动全局安装/补原包、不继承旧模型预算。
- 所选库能力是有限文档/最小probe依据；科学/格式/渲染/目标环境实测分别记，不把Mac24GiB或旧1GiB作产品预算。

## Review Focus

1. R7快照后R8编辑、切稿后旧ack、另存失败：不得清dirty/串绑定；由Task5/7测试。
2. 非正交cell和同标签不同周期像：不能逐轴round或替换所选像；由Task2/3断言VS9/52。
3. 资源2写失败、主写冲突、ack丢失：旧主文件/可继续编辑保持，indeterminate先查询；由Task5/7测试。
4. reference缺陷、非有限字段和未理解原生块：提示/保留/拒绝适用计算，不能伪修复或静默丢字段；由Task4及G-CODEC补证。
5. Worker取消/失效epoch、GPU丢失/字体迟到、关页与CLI竞争：提交/渲染/保存分开，旧结果不覆盖；由Task6/7/8及G-CA验证。

## 目录与命令合同

未来执行共同cwd `A=/Users/agilecho/code/VestaCraft/app`。所有相对代码路径均相对此计划root，链接不代表存在。root package.json不放Cargo.toml，kernel/Cargo.toml为Rust workspace；root脚本负责完整混合构建，避免CA只编译Rust。

计划package名：`vesta-model`、`vesta-commands`、`vesta-codecs`、`vesta-geometry`、`vesta-science`、`vesta-bindings`、`vestacraft-cli`。目录分别`kernel/crates/{model,commands,codecs,geometry,science,bindings,cli}/src/`。Task1创建这些名称前，以下cargo命令不可运行。

未来根scripts（必须真实实现，不允许空脚本返回0）：

| 命令（cwd A） | 计划实际职责/通过定义 |
|---|---|
| `npm ci --ignore-scripts` | 锁定依赖安装至本产品隔离node_modules；禁任意postinstall，所需步骤由显式脚本承担；当前未执行 |
| `npm run build` | `node scripts/build.mjs`：核工具版本→cargo build --locked（kernel manifest，native）→WASM release→同版wasm-bindgen web绑定→tsc→vite build；产物/许可/hash清单缺项非零 |
| `npm test -- --scope core` | `node scripts/test.mjs --scope core`：Task2–5 Rust/host合同与直接关联TS tests；任一失败非零，不把未实现skip记pass |
| `npm run test:browser -- --grep first-chain` | Playwright真实当前页面核心链，fixtures只用于输入，不能注入最终状态/自行生成软件产物 |
| `npm run test:delivery -- --target linux-x86_64` | 真实锁定Linux x86镜像内CLI无display与静态Web加载；缺镜像/浏览器/权限必须失败或明确未执行，不用Mac追认 |
| `Build/vestacraft-preview --screenshot <本轮png绝对路径>` | 真dist启动桥，等Loaded/Rendered，截图/清理；CA launch选取仍有G-CA前置，不是假定可用 |

工具安装/镜像获取尚无本轮执行授权。冷构建单独记录资源，CA120秒增量build约束不能靠假缓存/放宽结果规避。执行测试用显式小Budget（例如maxInputBytes1024、maxFieldValues512）只验证拒绝逻辑；生产预算由G-WEB/G-UX测量后裁定，不在本计划冒充已测上限。

## Task 1：锁输入并建立混合构建合同（未来实施；依赖G-WEB）

**Files — Create:** `package.json`、`package-lock.json`、`tsconfig.json`、`vite.config.ts`、`index.html`、`rust-toolchain.toml`、`kernel/Cargo.toml`、`kernel/Cargo.lock`、`scripts/build.mjs`、`scripts/test.mjs`、`licenses/dependencies.json`、`BUILDING.md`；测试`tests/delivery/build-manifest.test.mjs`。

**Interfaces:** consumes设计选定版本/许可与来源SHA；produces `build-manifest.json{schema:1,sourceDigest,toolVersions,dependencyLockDigests,artifacts:[{path,sha256,bytes}],assets:[{source,license,sha256}],target}`。dist/Web与CLI不得空缺或使用旧摘要。函数`verifyManifest(root:string, manifest:BuildManifest):Promise<void>`在scripts/build.mjs，任一文件不符抛错。

**VF/规格/资料:** VF-0001/0007、全部模块交付基础；D05/D07/CA07、D11§5/D12构建模板、设计§2；不代表任一业务已实现。首链fixture沿VS-0001及VS-0009，manifest独立SHA校验不调用被测构建函数造期望。

- [ ] 先补G-WEB依赖小spike：同版绑定器＋真实Three.js最小球/场/资源，记录浏览器/CPU/GPU/首帧，核选定版本能共同构建；失败返回路线设计，不安装全局依赖。
- [ ] 写manifest测试：改一个生成WASM字节后verify必须拒绝；缺CLI/字体license字段非零；依赖锁SHA固定。
- [ ] 实现目录和脚本，使root build覆盖native+WASM+Web。锁React DOM/types的真实匹配版本、完整transitive依赖；注册表版本冲突不得静默升级。
- [ ] 运行build和manifest反例，再保存真实工具/制品/耗时；通过后Task2–6可按各依赖继续。build失败不阻止独立数学oracle准备。

## Task 2：身份、Cell3、Phase骨架及修订（未来实施；独立基础）

**Files — Create:** `kernel/crates/model/{Cargo.toml,src/lib.rs,src/ids.rs,src/cell.rs,src/project.rs,src/resources.rs}`；测试`kernel/crates/model/tests/cell_identity.rs`。

**Interfaces:** `Cell3::from_parameters(lengths_a:[f64;3],angles_deg:[f64;3])->Result<Cell3,DomainError>`；`fractional_to_cartesian(&self,f:[f64;3])->[f64;3]`；`volume_a3(&self)->f64`；`ProjectState::new(ids:&mut dyn IdSource,cell:Cell3,label:String)->ProjectState`；types依接口§1及§8.1首链精确形状，ObjectId不取标签/位置，reload必换epoch。

**VF/规格/样本/资料:** VF-0007/0021/0022/0145；行为02、S06 cell-symmetry已证基础，VS-0001、VS-0009、S05 synthetic-independent-expected.json；D05/D06/VM06。高级群/多相/超空间不由此骨架宣称支持。

- [ ] 写失败断言：4Å立方V64；VS9的Hf=(1.75,3√3/4,0)，V12√3，数值按1e-12基础oracle；NaN/零det/负length拒绝且旧state不变。
- [ ] 运行`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-model --test cell_identity`，实现前须真实失败，保存结果。
- [ ] 实现上述API/typed单位/不可变对象ID；重复label对象ID不同；Undo恢复ID，reload新document/epoch后旧请求拒绝由Task3接。
- [ ] 重跑同命令与关联断言；全过才向Task3发布types。一般群接口仍留G-SCI，不写成P1.2已全冻结。

## Task 3：命令事务、历史、批量与周期选择（未来实施；依赖Task2）

**Files — Create:** `kernel/crates/commands/{Cargo.toml,src/lib.rs,src/schema.rs,src/registry.rs,src/transaction.rs,src/history.rs,src/jobs.rs}`；`kernel/crates/geometry/{Cargo.toml,src/lib.rs,src/images.rs,src/measure.rs}`；测试`commands/tests/transaction_contract.rs`、`geometry/tests/selected_image.rs`。

**Interfaces:** `Engine::execute(CommandRequest,&mut dyn ResourcePort)->CommandOutcome`，`Engine::inspect(InspectionQuery)->Result<Inspection,CommandError>` and `Engine::advance(work_units:u32,io:&mut dyn ResourcePort)->Vec<JobEvent>`，`selected_distance(cell:&Cell3,a:ImagePosition,b:ImagePosition)->Result<f64,DomainError>`。实现接口§3基础registry，不使用通用自由JSON执行函数；batch/error/idempotency形状同一authority生成。

**VF/规格/样本/资料:** VF-0019/0020/0022/0061/0208/0224/0225，D06、行为01 Boundary Apply/Cancel、行为02 R25/R26、S06_ORACLES；VS9/52对应主像√19/2、近像√7/2及9/1Å。

- [ ] 写失败断言：atomic第二命令非法时全部回滚/修订不变；sequential stop/continue逐项真实修订；重复request参数相同只commit1次、不同参数幂等冲突；旧revision/epoch拒绝；Undo/Redo单调增revision且内容digest正确。
- [ ] 写所选image断言：VS52主像9Å、shift(-1,0,0)1Å；VS9值独立根式，不调用生产nearest-image生成预期；不在此任务实现未知一般三斜最短像。
- [ ] 运行上述两个cargo测试目标为红，随后实现事务/增量patch/history/resource引用；draft Apply提交、Cancel仅丢未提交，不能泛化所有Properties。
- [ ] 重跑测试为绿，核原子batch一次undo与取消commit前/后两边界；定义生成`web/src/kernel/generated.ts`及CLI schema来源。失败时Task4可做独立codec读取研究，但不得消费未接受命令API。

## Task 4：首链原生结构/CIF codec及资源诊断（未来实施；依赖Task2，完整codec依赖G-CODEC）

**Files — Create:** `kernel/crates/codecs/{Cargo.toml,src/lib.rs,src/vesta.rs,src/cif.rs,src/diagnostics.rs,src/resource_manifest.rs}`；测试`codecs/tests/native_structure.rs`和`tests/codec/oracles/first_chain.py`。

**Interfaces:** `decode<R:ResourceReader+?Sized>(input:&[u8],format:InputFormat,resources:&R)->Result<DecodedProject,CodecError>`；`encode_native(project:&ProjectState,options:&NativeWriteOptions)->Result<EncodedBundle,CodecError>`；`DecodedProject{state,diagnostics,rawBlocks,editableCapabilities}`；encode不能静默舍弃raw未知块，无法保持时E_FORMAT_LOSS。

**VF/规格/样本/资料:** VF-0009/0012/0013/0014；FORMATS/SAMPLES、行为05、S05原始结构子集；VS0001/2/3/8/9/12/13/14；D06/D08、VM18/EXT-CIF1。不因两个解码器同意就自证。

- [ ] 写失败断言：VS1 cell4³和C1、VS2隐藏但STRUC未删、VS9两位点占位/Biso和V12√3、VS12中文路径等价；VS13截断拒绝可计算payload但保留原字节诊断；VS14负cell不伪装合法。
- [ ] 独立Python只读字段解析和手算值生成oracle；不得直接调用Rust生产codec生成期望。运行`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-codecs --test native_structure`。
- [ ] 实现所列明确子集和raw block位置保留，元数据/科学层未理解时readOnly/拒绝破坏性写回；产物由软件真实encode产生。
- [ ] 原生保存后新实例读回，并用独立Python解析关键内容；需要原件对照时按E/run_probe.py新cwd/VESTA_PREF/工作副本/watch/PID/超时余量执行，不重跑已答历史。完整48/23格式归G-CODEC后续卡。

## Task 5：快照、ack和多资源提交状态机（未来实施；内核部分可独立，host依赖G-HOST）

**Files — Create:** `kernel/crates/commands/src/snapshot.rs`、`web/src/host/save-state.ts`、`tests/contracts/save-state.test.mjs`、`tests/contracts/bundle-plan.test.mjs`。P4计划宿主差异路径`IntegrateCraft/dsh/plugins/software/vesta-files.mjs`，本轮/产品首任务不写共享仓库。

**Interfaces:** `snapshot(state:&ProjectState,options:SnapshotOptions)->Result<SaveBundle,CommandError>`；`SaveTracker.confirm(ack:HostAck):SaveState`、`reject(nack:HostNack):SaveState`；host未来`commitNativeBundle(space:SoftwareFiles,bundle:SaveBundle,lease:WriteLease):Promise<HostAck|HostNack>`。字段、步骤和完成时刻严格接口§5；具体平台写者竞争承诺须G-HOST先冻结，mock不能证明真配额/路径/原子性。

**VF/规格/资料:** VF-0012/0013/0014/0018、D02§5–6、D06、P04/P05及设计§4；VS4/5的外部资源失败是反例，另合成2资源bundle用于合同测试，不能登记成已运行稳定样本。

- [ ] 先写五个失败断言：R7ack后R8仍dirty；A稿旧ack不动B；saveAs失败path不变；export不清dirty/重绑；重复ack不重复结算。
- [ ] 写顺序fault injection：资源2的507与主409保持旧主SHA；主提交后ack丢失返回indeterminate→按实际SHA复核；错误SHA ack必须拒绝。此处替身身份明确，仅核状态机。
- [ ] 实现immutable SaveBundle/主文件末提交计划、host ack不可由Agent直接调用；真实文件/权限在G-HOST和Task7，不以替身通过称保存接通。
- [ ] 运行`node --test tests/contracts/save-state.test.mjs tests/contracts/bundle-plan.test.mjs`并保存反例结果；G-HOST未闭合时Task7不能接真host，但Task6页面可消费内存save契约。

## Task 6：单Worker、Three.js首链页面及人工编辑（未来实施；依赖Task1–4，G-WEB/G-UX）

**Files — Create:** `kernel/crates/bindings/{Cargo.toml,src/lib.rs}`、`web/src/kernel/{worker.ts,client.ts,generated.ts}`、`web/src/render/{renderer.ts,packets.ts}`、`web/src/editor/{App.tsx,cell-editor.tsx,site-editor.tsx}`、`web/src/host/page-api.ts`；测试`tests/browser/first-chain.spec.ts`、`tests/contracts/worker-epoch.test.mjs`。

**Interfaces:** KernelSession导出、Worker envelope、VestaPageV1、RenderPacket/Rendered同接口§4；UI全部发正式registry命令，renderer不生成科学状态。

**VF/规格/样本/资料:** VF-0007/0022/0023/0030/0031/0037/0040/0061/0208/0225；行为01/02、D07、D09、VS1/9/52；G-UX新“晶胞α、占位½”fixture先独立登记。首链球棍只已证结构，不替全材质/体数据。

- [ ] 写真实页面断言：命令建4Å/C1→页面inspect是同document/revision→Rendered对应修订；人工改a=5后Agent读体积80 Å³；之后Agent改a=6画面/状态体积96 Å³，无刷新/重开。
- [ ] 写Worker错误/资源所有权/取消/旧epoch测试；主线程在grow后不复用WASM旧view；GPU lost使frame无效；字体未ready不报中文字已渲染。
- [ ] 实现正式page API和DOM输入/IME组成中不提交（compositionend才按确认动作提交）；真实Three.js和资源释放，无localStorage/IndexedDB/OPFS工程路径。
- [ ] 运行`npm run test:browser -- --grep first-chain`；截图与领域断言并列；显式收集解析/传输/首帧与交互用时，不用sleep替代render token。预算未测不能接受G-UX性能。

## Task 7：平台三视图/文件桥/关闭续作（未来P4；依赖Task5–6、G-HOST）

**Files — Plan Modify/Create in共享平台，需届时授权:** `dsh/plugins/software/{catalog.mjs,startup.mjs,service.mjs,client.jsx,client.js,software-files.mjs,vesta-page.mjs,vesta-tools.mjs,vesta-coordinator.mjs,vesta-files.mjs}`，`craft/vestacraft/{README.md,upstream.lock.json,patches/}`；测试`tests/demo/vestacraft/{page.mjs,save.mjs,handoff.mjs}`。当前不创建。

**Interfaces:** VestaCoordinator复用existing serial，`changeLayout(mode)`只layout，`runInTurn`转page或CLI当前lease，`saveCurrentInTurn()`不重入serial，`closePage()`保存确认后handoff；`commitNativeBundle`同Task5。现有PageChannel不重造。NEW_FILES/ext白名单增加本款经核类型，不偷放任意后缀。

**VF/规格/资料:** VF-0009/0012–0018和当前命令；D02§2/4–6、P01–P07、Task5具体异常；VS1/9/12和真实保存产物。没有稳定样本的路径/配额反例使用具名测试fixture，不伪造Study实测关联。

- [ ] 先重核HEAD/status/所改符号/制品，锁受影响共享差异；读G-HOST的真实外改竞争和配额语义，未确定不写commitNativeBundle空实现。
- [ ] 写真实零模型工具→当前页面→手改→Agent读回测试，三layout切换document/epoch/owner/activeJob完全不变。
- [ ] 写真host409/507/只读/越根/资源symlink、迟到ack/网络中断/另存/导出测试；Linux anchored路径守卫必须真实运行，Mac ENOTSUP不算通过。
- [ ] 关页保存确认→CLI读取同WorkRef继续→新page显式打开；验证实际文件内容与新内核状态，驱动不得自行写最终文件。
- [ ] 同步client生成bundle并核实际加载摘要；运行本款新增/直接关联测试。完整平台回归/真实模型仍按P5及未来明确授权，不在本卡触发。

## Task 8：原生CLI及CA真实worker/Check（未来；依赖首链和G-CA）

**Files — Create:** `kernel/crates/cli/{Cargo.toml,src/main.rs,src/stdio.rs}` and `kernel/crates/preview/{Cargo.toml,src/main.rs}` and `scripts/preview.mjs`、`Build/vestacraft-preview`、`scripts/ca-prepare/`、`tests/contracts/cli-completion.test.mjs`、`tests/delivery/ca-wiring.mjs`、`MECHANICS.md`、`mechanics.json`。

**Interfaces:** CLI首链argv/退出/stdio协议见接口§6；CA prepare必须调用锁定public API、生成真实workspace后派生复制既有Study，不手造done；打包启动桥真实浏览器启动/截图和正常模式不能预制图。prepare工具精确构建制品和迁入/launch闭环仍待G-CA，因此本Task不能称完全可直接执行。

**VF/规格/资料:** 逐条使用文末16行“相关功能导航”的原command JSON pointer、functions.json VF条目和CLI.md R节，不假称official-command-contracts自带VF字段；首链及扩展对应VF-0009/0012/0013/0014/0015/0018/0032/0033/0064/0069/0106/0191–0196/0228/0238/0239的相关子路径，不自动等同GUI全功能。不可达set/stub不加入产品。D06/CA04/CA08/METHOD、CLI_COMPLETION及S06 mechanics仍为指定资料。

- [ ] 写CLI首链成功/失败退出及内容oracle，字节来自本产品保存；displayless CLI不调用GUI库，PNG单独renderer前置，缺环境明确失败。
- [ ] 完成G-CA prepare/launch设计补证，真实Codex后端/模型/次数/预算获后续授权后才启动worker；精确请求、inbox和checks/asks配对。
- [ ] 在运行worker中调用rebuild_check round0；故意破坏本轮产物使Check失败，再由Codex修复/检查；同进程API builder的FeatureMissing按结束turn路径验证，不绕guard。
- [ ] 保存真实build/runs/screenshot/mechanics/claims、停止/续作和session边界；dsh独立复审后才判该任务。Ready不替全功能/目标平台接受。

## 未冻结模块：先完成P1补证，再写其精确实施卡

下表是施工依赖安排，**不是声称已形成可执行全功能实现任务**。具体代码路径是职责预定，不保证高级schema冻结；不以`options:unknown`或stub填空。每行以functions.json与S06组直接取对应VF/样本/unknown，下面附现有VF列表便于准确接手。全功能范围始终保留。

| 设计补证任务 | 计划模块与依赖 | 必须先回答的具体问题、独立预期及资料 | 失败后能继续 |
|---|---|---|---|
| G1 相/群/变胞/ADP | `geometry/src/{symmetry,phase,transform,adp}.rs`；依Task2 | S06 phase/cell-symmetry/transform/sites-adp/vectors；新非正交两相以H和正交方向事前解R/t、循环拒绝；一般群/setting/近特殊位置枚举；新U张量特征值/换基与打印精度分开。VF/S06 nextProbe逐条不猜。D05、VM06/07/09、行为02、VS18/53/57/74–78仅既有范围 | Task2/3基础ID/事务、首链P1/CIF |
| G2 周期图/几何/形貌 | `geometry/src/{neighbors,polyhedra,planes,shape,reduction}.rs` | S06 duplicate/bonds/polyhedron/planes/boundary/measure/tidy/shape；三斜最近像收敛整数界、新四面体/近共面det与面积、平面法向/符号、Niggli独立不变量；键价表按资产门逐项绑定，不能默认B=.37。D05、VM08/10/11/14、VS9/52/54/55/64/65；原精度和失败保持 | 已证所选像测量、纯模型/保存 |
| G3 场/FFT/等值面/2D | `science/src/{field,fft,structure_factors,fourier,density,isosurface,slices,projection,line,peaks}.rs` | S06 volume-data/volume-render/sf/fourier/electron/nuclear/model-patterson/volume-patterson/2d/line/peak-voronoi/density-conversion；FFT正负相位/V/N独立DFT、Nyquist奇偶/非正交三轴，解析平面等值面交点/法向/面积与退化；投影精确积分和legacy偏差分别预期。D07、VM14/15、VS59/62/63/79/102和S06_ORACLES；不是把成熟FFT包等同科学规则 | 基础Worker/渲染框架，不冻结场算法 |
| G4 全codec/图像/材质 | `codecs/src/formats/*.rs`、`render/{materials,labels,exports}.ts` | G-CODEC各原生unknown具体probe；S05 input48/output23与实际option合同逐format；PDB列/ANISOU、m3d正负量化、PRI flag/ABI、DEN/STO header、Elk空串、ASSE列、GAMESS首原子/跨度；独立读取器核字段。Raster失败不由resolved洗白。D08/D09、FORMATS、行为01/05、S05原字节 | Task4已证子集、页面接口/错误恢复 |
| G5 外算/参数/字体/环境 | `science/src/external/*.rs`、`assets/`、`tests/external/`、Linux build镜像 | MADEL中性NaCl与独立Ewald（先具备获准运行件，不重跑缺库崩溃）；TIDY通用群；RIETAN/ORFFE producer真实计算、参数/字体许可与字段/原始版本；中文IME/导出和VS102规模分层RSS/GPU/p95/冷启测量。D11、DPRE05–07、CLI/MADEL/S07合同、SOURCE_NOTES指定原文，不复制原件资产 | 普通领域、无网络命令、现有首链 |

G1–G6新probe不能以本表作空白授权：执行前在新目录登记实际输入/SHA、独立预期/单位/公差、工具/argv/cwd/watch、秒/内存/磁盘限和cleanup，再由E/run_probe.py记录。预算/外部环境尚未确定的动作保持未执行。任何关键公共payload改变，先修接口/设计、复审，再写精确到函数/测试的对应模块卡；不直接进P3施工。

## 当前计划自审与交接

Task1–8覆盖基础链/宿主/CLI的具体邻接；G1–G6将全部高级功能保持在依赖安排中。尚无全高级schema和精确测试命令，故不能声称P1.4全功能精确计划已通过。当前正式停止点是P1交付，不选择执行方式、不启动实施。

独立复审由dsh只读判断P1原四项及通过条件，不要求所有unknown先关闭，也不能把“已列门”当“相邻接口已冻结”。复审后的实际裁定/剩余项以R/review-result.json及HANDOFF当前头为准。

## 修订2的接手范围

接口§8补足Task2–8消费的首链模型、ResourcePort、增量job调度、batch结果、跨桥真实字节/lease与导出回执。launcher选择由P1-PROBE-02修正为唯一Build可执行启动桥（候选未执行）。首轮dsh流中断没有最终裁定，修订稿另审。

G6补应用/偏好/数据库/交互生命周期设计：计划路径 `web/src/editor/{preferences,lifecycle,database,interaction,panels}.tsx` 和 `render/environment.ts`。读取functions.json中全部非S06领域48项，沿其studyHandoff/行为01、05及VM04/11/13/16、DATABASE/GUI_LIFECYCLE定位。OS Hide/Show/Quit/New Window、Stop/网络、Output/Comment/Summary、AA/光照/深度/投影、动画/工具/焦点/面板的Web宿主映射不能由菜单名猜测；有能力差异须具名设计及验证，不能默默删入口。尚未冻结者留P1设计，不假写实现API。基础内核/codec/保存可继续不依赖这些行为的设计。

下面仅列任务作用域ID，自动取自唯一functions.json/S06分组，便于每任务带VF；没有新增功能规格/覆盖状态，也不把254当潜在功能穷尽。样本和独立断言仍从每条studyHandoff/SAMPLES/S06_ORACLES取得，候选不算实测。

G1: VF-0021, VF-0022, VF-0023, VF-0027, VF-0063, VF-0084, VF-0085, VF-0086, VF-0087, VF-0088, VF-0089, VF-0090, VF-0091, VF-0092, VF-0093, VF-0094, VF-0095, VF-0096, VF-0097, VF-0098, VF-0099, VF-0100, VF-0101, VF-0102, VF-0103, VF-0104, VF-0105, VF-0145, VF-0146, VF-0147, VF-0148, VF-0149, VF-0176, VF-0177, VF-0178, VF-0230

G2: VF-0025, VF-0026, VF-0028, VF-0042, VF-0051, VF-0052, VF-0053, VF-0054, VF-0058, VF-0061, VF-0064, VF-0065, VF-0066, VF-0106, VF-0111, VF-0112, VF-0113, VF-0114, VF-0115, VF-0116, VF-0137, VF-0151, VF-0159, VF-0174, VF-0175, VF-0179, VF-0180, VF-0181, VF-0182, VF-0208, VF-0209, VF-0210, VF-0211, VF-0221, VF-0224, VF-0231

G3: VF-0024, VF-0045, VF-0046, VF-0047, VF-0048, VF-0049, VF-0050, VF-0059, VF-0060, VF-0068, VF-0069, VF-0070, VF-0071, VF-0072, VF-0073, VF-0074, VF-0075, VF-0076, VF-0077, VF-0078, VF-0079, VF-0080, VF-0081, VF-0107, VF-0108, VF-0109, VF-0110, VF-0138, VF-0139, VF-0140, VF-0141, VF-0142, VF-0150, VF-0160, VF-0161, VF-0162, VF-0163, VF-0164, VF-0172, VF-0234, VF-0235, VF-0236, VF-0237, VF-0238, VF-0239, VF-0240, VF-0241, VF-0242, VF-0243, VF-0244, VF-0245, VF-0246, VF-0247, VF-0248, VF-0249, VF-0250, VF-0251, VF-0252

G4: VF-0014, VF-0015, VF-0016, VF-0030, VF-0031, VF-0032, VF-0033, VF-0034, VF-0037, VF-0038, VF-0039, VF-0040, VF-0041, VF-0043, VF-0044, VF-0055, VF-0056, VF-0057, VF-0062, VF-0117, VF-0118, VF-0119, VF-0120, VF-0121, VF-0122, VF-0123, VF-0124, VF-0125, VF-0126, VF-0127, VF-0128, VF-0129, VF-0130, VF-0131, VF-0132, VF-0133, VF-0134, VF-0135, VF-0136, VF-0143, VF-0144, VF-0152, VF-0153, VF-0154, VF-0155, VF-0156, VF-0157, VF-0158, VF-0169, VF-0171, VF-0183, VF-0184, VF-0185, VF-0186, VF-0187, VF-0188, VF-0189, VF-0190, VF-0191, VF-0192, VF-0193, VF-0194, VF-0195, VF-0196, VF-0197, VF-0198, VF-0199, VF-0200, VF-0201, VF-0202, VF-0203, VF-0253, VF-0254

G5: VF-0067, VF-0232, VF-0233

G6: VF-0001, VF-0002, VF-0003, VF-0004, VF-0005, VF-0006, VF-0007, VF-0008, VF-0009, VF-0010, VF-0011, VF-0012, VF-0013, VF-0017, VF-0018, VF-0019, VF-0020, VF-0029, VF-0035, VF-0036, VF-0082, VF-0083, VF-0165, VF-0166, VF-0167, VF-0168, VF-0170, VF-0173, VF-0204, VF-0205, VF-0206, VF-0207, VF-0212, VF-0213, VF-0214, VF-0215, VF-0216, VF-0217, VF-0218, VF-0219, VF-0220, VF-0222, VF-0223, VF-0225, VF-0226, VF-0227, VF-0228, VF-0229

工具合同补正：Task1未来显式使用`cargo +1.99.1 install wasm-bindgen-cli --version 0.2.130 --locked --root tools --target-dir build-cache/tools`（仅后续授权、隔离目录、未执行），随后调用`tools/bin/wasm-bindgen`，不靠PATH中未锁定版本。kernel将serde_json锁1.0.145、sha2锁0.10.9（default-features=false），wasm-bindgen锁0.2.130；具体元数据见R/dependency-protocol-supplement.json。Build/vestacraft-preview由kernel/crates/preview真实编译后复制，Build只保留一个候选；scripts/preview.mjs驱动实际浏览器，源码/制品/浏览器须在manifest绑定，候选被路径选中不等于它已运行。

## 复审发现处理（修订3）

正式dsh接受P1.1、整体partial，P1.2/.3/.4保持未完成。接口I3补明uncertainty首链null、同位素质量数、首链格式/VF子集、targetToken签发/有效期/绑定、SaveState、带ResourcePort的advance和逻辑资源名映射。Task2–8消费这些具体补正；不因此解除高级schema或G-CA门。

G1–G6下面的ID清单现定义为**唯一主责**，其它章节的重复提及仅为依赖/共同测试。TIDY VF-0065/0066主责G2；内置powder VF-0068/0234–0237主责G3；G5为这些任务提供外部运行件/资产资格，不能再算一份主责。G5主责仅Madelung相应ID。此划分是施工安排，不是第二套功能规格/科学覆盖矩阵。

Task8引用更正：official-command-contracts.json没有VF字段，不能“从该表读取对应VF”。下表是Codex依据functions.json、CLI.md R65–R92建立的**相关功能导航**；合同JSON仍保留原16条及原字段，不改历史。相关不表示GUI/CLI参数或完成语义等价，更不表示产品已支持这些原命令。标明无独立VF的协议分支不造新ID。

| Original CLI | Source pointer | Related VF navigation | Scope |
|---|---|---|---|
| `-i` | `official-command-contracts.json#/commands/0` | VF-0009 | CLI.md R65/R85–92；读取/追加合同，非通用全场景替换 |
| `-o` | `official-command-contracts.json#/commands/1` | VF-0012、VF-0013 | CLI.md R66；原保存别名，不自造编辑API |
| `-calcf` | `official-command-contracts.json#/commands/2` | VF-0069、VF-0238、VF-0239 | CLI.md R67–68；原显式/默认因子参数 |
| `-list` | `official-command-contracts.json#/commands/3` | VF-0064 | CLI.md R66；原几何列表，读/计算与显示选择分别判断 |
| `-merge_split_site` | `official-command-contracts.json#/commands/4` | VF-0106 | ALGORITHMS/CLI.md R65；原阈值及已证合并域 |
| `-rotate_x` | `official-command-contracts.json#/commands/5` | VF-0191、VF-0192 | CLI.md R65/R70；x视图旋转关联，CLI角度不等同GUI步长 |
| `-rotate_y` | `official-command-contracts.json#/commands/6` | VF-0193、VF-0194 | CLI.md R65；y旋转关联，GUI相同链未由此通过 |
| `-rotate_z` | `official-command-contracts.json#/commands/7` | VF-0195、VF-0196 | CLI.md R65；z旋转关联，GUI相同链未由此通过 |
| `-close` | `official-command-contracts.json#/commands/8` | VF-0018 | CLI.md R70；指定路径关闭合同 |
| `-export_img` | `official-command-contracts.json#/commands/9` | VF-0015 | CLI.md R70；原raster分派，不能关联成VF-0016矢量成功 |
| `-flush` | `official-command-contracts.json#/commands/10` | 无独立VF；协议相邻于VF-0015的完成链 | CLI.md R70/R92；不是已证render屏障，不新增稳定ID |
| `-open` | `official-command-contracts.json#/commands/11` | VF-0009 | CLI.md R70；GUI IPC读入与真实完成分开 |
| `-reopen` | `official-command-contracts.json#/commands/12` | VF-0009相关的重载子路径 | CLI.md R70；原合同单列，不冒称File/Open全语义等价 |
| `-save` | `official-command-contracts.json#/commands/13` | VF-0012、VF-0013、VF-0014 | CLI.md R69/R71；按原format/option区分工程与数据输出 |
| `-scale` | `official-command-contracts.json#/commands/14` | VF-0032、VF-0033相关视图缩放 | ALGORITHMS/CLI.md R65；乘法控制，不推任意GUI缩放全部等价 |
| `-scale_width_to` | `official-command-contracts.json#/commands/15` | VF-0228 | CLI.md R70；物理graphics width单位Å |

## P1 续办的模块细化 2026年11月14日
本节是未来实施安排，未写产品代码。Task1–8 和原 G1–G6 的唯一主责清单保持；下面的领域引用是共同测试／设计依赖，不另算主责。完整公开科学／codec schema 仍受接口 I5 §11.6 约束，所以本节增加了具体路径、fixture 和断言，但**没有把所有高级实现单元判为可开工，也不单方宣告 P1.4 完成**。
共同 cwd 为未来 `/Users/agilecho/code/VestaCraft/app`。Rust 测试使用 `cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p <package> --test <target>`；web 测试使用 `npm run test:browser -- --grep <test-name>`。这些文件／命令均为计划，当前不存在，必须先完成 Task1 工具／锁合同及每单元冻结门。
预算设计：B-MATH 为未来单目标测试 timeout30s、RSS256MiB、fixture/输出16MiB；B-GRID 为已测 VS102 规模的检查 timeout60s、RSS512MiB、输出64MiB；B-WEB 为未来单浏览器用例 timeout60s、一个受控页面，RSS/首帧/p95只测不设生产通过值；B-EXTERNAL 不分配运行额度，先具备获准运行件和独立原始输入才发具体预算。数值是 Codex 提出的测试防失控上限，未实测、不继承为生产预算或当前执行权限；超出即停止该测试并保留部分产物。CA 自带 build120s/screenshot90s另按原限制，不能取较宽预算绕过。
基础可定义接口继续用 I4 的 execute／advance／ResourceReader、I5 的 PhasePlacement1／AdpTensor1／FieldDescriptor2／LineRequest1 和 typed RuleRef。尚无冻结 DTO 的高级操作不假造 `Options:any` 或占位函数签名；下文 blocked 的单元只接受证据／独立预期准备，**产品入口必须在对应公共类型完成后另行写入本计划才能执行**。

### G1 领域 phase
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/0` 和其中 VF-0021, VF-0084, VF-0085, VF-0086, VF-0087, VF-0088, VF-0089, VF-0090, VF-0146, VF-0147 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/phase.rs`；测试 `kernel/crates/geometry/tests/phase_reference.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0018 `s05/official-examples/VESTA-Examples/VESTA/Morphologty_data/Adularia-8twins.vesta` SHA256 `0635fe7c321415204948c389b899c5b6a767ccfee2bbd828d32532c9543d4343`；VS-0009 `s05/fixtures/nonorthogonal.cif` SHA256 `e661819c3ce9bfb87bcdfa2a28373910407803e4fa806283a9a9728298713970`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：world=Rparent(Rlocal Hf+tlocal)+tparent；非正交父子、重复label、不存在引用和循环拒绝；原件R/t映射未证。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test phase_reference`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：两不同非正交相，事前以H和两个正交方向解R/t；分别测refer=0/前相、非法循环及失败后保存；避开已知两相Vectors Cancel崩溃。

#### 历史：R3接口与交接 phase

当前设计接口：`PhaseResolveJob::new(phase_id:ObjectId)->Result<PhaseResolveJob,DomainError>`；`fn step(job:&mut PhaseResolveJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ResolvedPlacement1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261108-200510-s04-multiphase-shapes/shape-geometry-crosscheck.json`（pathBase=project-root），SHA256 `9f9dcff928928583663ec6c8d839c4845cb3e93ada6881e834d1bd847dd27835`；性质post-observation-independent-recompute。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- shape-support，role=supporting-only；输出不作为该函数输出判据；单位A / A2。expected：`/phases`, `/comparisons`；tolerance：无源数值容差；政策unfrozen。Local shape geometry only. No assertion on ResolvedPlacement1.rotation/translationA.

未冻结：ResolvedPlacement1.rotation/translationA and refer graph original oracle; shape area is not a substitute。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/VESTA/Morphologty_data/Adularia-8twins.vesta`（pathBase=project-root），SHA256 `0635fe7c321415204948c389b899c5b6a767ccfee2bbd828d32532c9543d4343`；来源指针 /source，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/phase/input-manifest.json`, `tests/delivery/phase/actual.json`, `tests/delivery/phase/independent-comparison.json`, `tests/delivery/phase/resource-lifecycle.json`。当前未执行。

### G1 领域 cell-symmetry
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/1` 和其中 VF-0022, VF-0063, VF-0091, VF-0093, VF-0095, VF-0145, VF-0230 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/symmetry.rs`；测试 `kernel/crates/geometry/tests/symmetry_images.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0053 `reviews/20261108-224603-s06-inversion-sites/symmetry-working.cif` SHA256 `f040c441371611018284b31e73a0692c930a5de307fe4e27279a1c725b5f97fc`；VS-0074 `reviews/20261109-122500-s06-magnetic-symmetry/inversion-cwd/controlled.mcif` SHA256 `3d5c1445c0164a1d278297ee8461be0f74832ff2647a7e6c58c5b2a11402e878`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：VS53一般C两像／特殊O一像；磁性空间／时间反演分别核；群表和近特殊位置容差不能沿用P1假定。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test symmetry_images`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新群与近特殊位置双侧epsilon；冻结周期唯一集合及磁矩，区分site与display image；先补群表/基底再实现。 VF-0145：冻结非正交a/b/c与alpha/beta/gamma的不同s.u.，独立一阶导数和协方差候选；逐字段读回、失焦提交、异常/保存后状态另核。

#### 历史：R3接口与交接 cell-symmetry

当前设计接口：`ImageExpansionJob::new(phase_id:ObjectId,ops:Vec<AffineOp3>,bounds:Bounds1)->Result<ImageExpansionJob,DomainError>`；`fn step(job:&mut ImageExpansionJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Vec<ImagePosition1>>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/independent-inversion-expectation.json`（pathBase=project-root），SHA256 `76696f580e24a1e12ba819e18e0bcbc59414d072304e99d790d1a99bf03ae026`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- inversion-count，role=result-invariant；输出images.length；单位count。expected：`/halfOpenUnitCellAtomImages`, `/closedUnitCellVisibleImagesConditionalNoExtraSearch`；tolerance：无源数值容差；政策exact-discrete。P-1 fixture only: half-open count3; closed visible count10 only with stated no-extra-search condition; full ImageKeys/positions still missing.

未冻结：Complete ImageKey and fractional/cartesian coordinates for each supported operator/setting。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/symmetry-working.cif`（pathBase=project-root），SHA256 `f040c441371611018284b31e73a0692c930a5de307fe4e27279a1c725b5f97fc`；来源指针 /records/52/path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/cell-symmetry/input-manifest.json`, `tests/delivery/cell-symmetry/actual.json`, `tests/delivery/cell-symmetry/independent-comparison.json`, `tests/delivery/cell-symmetry/resource-lifecycle.json`。当前未执行。

### G1 领域 transform
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/2` 和其中 VF-0092, VF-0094 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/transform.rs`；测试 `kernel/crates/geometry/tests/cell_transform.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0070 `s05/independent-transform-p1.cif` SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：Hprime=HP、fprime=inv(P)(f-p+n)、beta inverse congruence；det0旧状态不变；输出打印政策与模型精度分列。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test cell_transform`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新非平凡群+det1/2减胞，预枚举陪集与操作共轭；以保存精度/模型精度两套固定判据核，不从输出放宽。

#### 历史：R3接口与交接 transform

当前设计接口：`CellTransformJob::new(phase_id:ObjectId,p:Mat3,origin_fractional:Vec3)->Result<CellTransformJob,DomainError>`；`fn step(job:&mut CellTransformJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<TransformedStructure1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-073141-s06-unit-cell-transform/independent-transform-before-gui.json`（pathBase=project-root），SHA256 `29c69b46d38a531e21021cb393b3038a6eeb7ba72c1541892b25b8971cffd5f0`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- p1-transform，role=direct-result；输出cell, sites；单位A / degree / A3 / fractional / occupancy / A2。expected：`/cases/new-symmetry/expectedCell`, `/cases/new-symmetry/expectedVolume`, `/cases/new-symmetry/expectedStoredSites`, `/cases/new-symmetry/expectedExpandedPhysicalSitesModulo1`, `/cases/new-sites/expectedCell`, `/cases/new-sites/expectedVolume`, `/cases/new-sites/expectedStoredSites`, `/cases/new-sites/expectedExpandedPhysicalSitesModulo1`, `/cases/shear-origin/expectedCell`, `/cases/shear-origin/expectedVolume`, `/cases/shear-origin/expectedStoredSites`, `/cases/shear-origin/expectedExpandedPhysicalSitesModulo1`；tolerance：`/tol`；政策source-declared。Match each case matrixP/origin/mode first; compare metric and sites with per-field tol, fractional modulo1 and duplicate-aware multiplicities. No general group/ADP claim.

未冻结：General symmetry/ADP transforms and sourceToCandidate object-ID mapping。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/independent-transform-p1.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/transform/input-manifest.json`, `tests/delivery/transform/actual.json`, `tests/delivery/transform/independent-comparison.json`, `tests/delivery/transform/resource-lifecycle.json`。当前未执行。

### G1 领域 sites-adp
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/3` 和其中 VF-0023, VF-0096, VF-0097, VF-0098, VF-0099, VF-0100, VF-0101, VF-0102, VF-0103, VF-0104, VF-0105, VF-0148, VF-0149 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/adp.rs`；测试 `kernel/crates/geometry/tests/adp_basis.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0057 `s05/independent-nonorthogonal-adp.cif` SHA256 `dc52d2f42554ceb909a6d9ea129a3bfa54efa1ca708d4bb1c605b34f3d892840`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：CIF U/beta/Cartesian带基底，Ueq与B=8pi²U独立核；PSD/零/负U及实际椭球轴必须另有输入判据。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test adp_basis`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-02)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：冻结PSD/零/负特征值U，区分U/B/beta与Cartesian协方差；保存/重开和实际椭球轴分别判，不以字段编码代替几何。

#### 历史：R3接口与交接 sites-adp

当前设计接口：`AdpBasisJob::new(cell:Cell3,tensor:AdpTensor1)->Result<AdpBasisJob,DomainError>`；`fn step(job:&mut AdpBasisJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<CartesianTensor1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-021600-s06-nonorthogonal-adp/independent-nonorthogonal-adp.json`（pathBase=project-root），SHA256 `4bb0cb36a1316f5f1a6c71af05b3ceb4c97d6470261134f7c10916af10d6557d`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- scattering-support，role=supporting-only；输出不作为该函数输出判据；单位reflection/scattering units declared in source。expected：`/fullHKL`；tolerance：`/comparisonPolicy`；政策source-declared。Conventional CIF Uij scattering alternative is source evidence, not six Cartesian tensor components. Do not use it as CartesianTensor1 direct oracle.

未冻结：CartesianTensor1.sixA2 independent basis-transformed values and geometry tolerance。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/independent-nonorthogonal-adp.cif`（pathBase=project-root），SHA256 `dc52d2f42554ceb909a6d9ea129a3bfa54efa1ca708d4bb1c605b34f3d892840`；来源指针 /samplePath，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/sites-adp/input-manifest.json`, `tests/delivery/sites-adp/actual.json`, `tests/delivery/sites-adp/independent-comparison.json`, `tests/delivery/sites-adp/resource-lifecycle.json`。当前未执行。

### G2 领域 duplicate
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/4` 和其中 VF-0106 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/reduction.rs`；测试 `kernel/crates/geometry/tests/merge_sites.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0052 `reviews/20261108-221538-s06-periodic-pair/periodic-working.cif` SHA256 `18892a023f638f83ec263c8e47e5fda247dd57fabb437e383779532ba9505998`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：跨周期且占位不等的三点；距离阈值与元素/占位合并规则分别冻结；GUI合并未知不能由平均坐标拟合。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test merge_sites`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-02)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新分裂点不等占位/跨边界，事先分列几何和metadata多种候选，不复用旧均值拟合输入。

#### 历史：R3接口与交接 duplicate

当前设计接口：`CandidateClustersJob::new(phase_id:ObjectId,threshold_a:f64)->Result<CandidateClustersJob,DomainError>`；`fn step(job:&mut CandidateClustersJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Vec<Vec<ObjectId>>>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/independent-cli-before-runs.json`（pathBase=project-root），SHA256 `a1f7e6a2a71c6a94b818ccedc6c90b7e369ffefcf1a51f569c17001d3f2852f9`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- cluster-count-support，role=result-invariant；输出cluster-induced site count；单位count / A。expected：`/cases/merge-default/expected/storedSites`, `/cases/merge-default/expected/independentDistancesA`, `/cases/merge-small/expected/storedSites`, `/cases/merge-small/expected/independentDistancesA`, `/cases/merge-large/expected/storedSites`, `/cases/merge-large/expected/independentDistancesA`；tolerance：`/tol/fractional`；政策source-declared。Threshold-specific CLI merge counts are supporting compatibility checks; fractional tolerance is for source coordinate comparisons only, not a new distance tolerance. Exact ObjectId cluster membership still missing.

未冻结：CandidateClustersJob exact identity membership and explicit threshold-distance comparison tolerance。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/independent-transform-p1.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /source，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/s05/independent-cli-duplicate-sites.cif`（pathBase=project-root），SHA256 `06053f8f618ed61f2d750c4bda634d7ed0ca10e25d97c290191480fc183a17c4`；来源指针 /newSamplePath，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/baseline-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/baseline/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/rotate-x-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/rotate-x/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/rotate-y-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/rotate-y/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/rotate-z-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/rotate-z/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/rotate-xy-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/rotate-xy/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/rotate-yx-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/rotate-yx/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/scale-two-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/scale-two/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/scale-order-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/scale-order/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/width-nogui-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/width-nogui/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/merge-default-working.cif`（pathBase=project-root），SHA256 `06053f8f618ed61f2d750c4bda634d7ed0ca10e25d97c290191480fc183a17c4`；来源指针 /cases/merge-default/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/merge-small-working.cif`（pathBase=project-root），SHA256 `06053f8f618ed61f2d750c4bda634d7ed0ca10e25d97c290191480fc183a17c4`；来源指针 /cases/merge-small/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/merge-large-working.cif`（pathBase=project-root），SHA256 `06053f8f618ed61f2d750c4bda634d7ed0ca10e25d97c290191480fc183a17c4`；来源指针 /cases/merge-large/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/list-cell-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/list-cell/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/list-site-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/list-site/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/list-bond-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/list-bond/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/list-angle-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/list-angle/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-082000-s07-cli-geometry/list-poly-working.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /cases/list-poly/workingInput，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/duplicate/input-manifest.json`, `tests/delivery/duplicate/actual.json`, `tests/delivery/duplicate/independent-comparison.json`, `tests/delivery/duplicate/resource-lifecycle.json`。当前未执行。

### G2 领域 bonds
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/5` 和其中 VF-0026, VF-0174, VF-0175 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/neighbors.rs`；测试 `kernel/crates/geometry/tests/bond_graph.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0052 `reviews/20261108-221538-s06-periodic-pair/periodic-working.cif` SHA256 `18892a023f638f83ec263c8e47e5fda247dd57fabb437e383779532ba9505998`；VS-0054 `reviews/20261108-230337-s06-octahedron/octahedron-working.cif` SHA256 `b08a9e646594d7c8419c559071fd9dbe1080934a5a079c3daff119f1d459393b`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：显式所选9Å与近像1Å分开；binary32界限平方；三搜索/三边界与价键参数必须具名。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test bond_graph`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-02)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：有限分子和周期三斜网络分列各模式，独立枚举有界邻接图；键价另核表中多参数/氧化态9含义，不默认B=.37。 VF-0175：相同SiO6输入对Show polyhedra开/关、按label/species及三search mode分别冻结图边/中心/预期poly数；逐选项回读和独立几何/状态核，不只看按钮值。

#### 历史：R3接口与交接 bonds

当前设计接口：`BondIntervalJob::new(images:Vec<ImagePosition1>,min_a:f64,max_a:f64)->Result<BondIntervalJob,DomainError>`；`fn step(job:&mut BondIntervalJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Vec<Edge1>>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/independent-periodic-expectation.json`（pathBase=project-root），SHA256 `a674830e2d48f0a00b35f7fbd66624f9e75abc0ea67979cbca065288812390cc`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- periodic-distances，role=result-invariant；输出edges[].distanceA；单位A。expected：`/primaryCartesianDistanceA`, `/minimumImageDistanceA`, `/latticeEnumeration`；tolerance：无源数值容差；政策analytic-reference-no-runtime-tolerance。Selected C/O images give9A and nearest1A; all enumerated shifts have independent distances. Match selected ImageKeys and explicit min/max first; this does not select chemical adjacency.

未冻结：Numeric execution tolerance and complete chemical/search-mode adjacency。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/periodic-working.cif`（pathBase=project-root），SHA256 `18892a023f638f83ec263c8e47e5fda247dd57fabb437e383779532ba9505998`；来源指针 /records/51/path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/bonds/input-manifest.json`, `tests/delivery/bonds/actual.json`, `tests/delivery/bonds/independent-comparison.json`, `tests/delivery/bonds/resource-lifecycle.json`。当前未执行。

### G2 领域 polyhedron
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/6` 和其中 VF-0042, VF-0058, VF-0137, VF-0159, VF-0221 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/polyhedra.rs`；测试 `kernel/crates/geometry/tests/polyhedron_geometry.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0054 `reviews/20261108-230337-s06-octahedron/octahedron-working.cif` SHA256 `b08a9e646594d7c8419c559071fd9dbe1080934a5a079c3daff119f1d459393b`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：SiO6体积32/3；表面积16sqrt(3)只是独立预期，未冒称已原件读回；新四面体/近共面/非凸分别判。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test polyhedron_geometry`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-02)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新四面体与近共面构型，以独立行列式/凸包推体积和理想参考；另核BV资源/调用再固定价键oracle。

#### 历史：R3接口与交接 polyhedron

当前设计接口：`PolyhedronMeasureJob::new(vertices_a:Vec<Vec3>,faces:Vec<Vec<u32>>)->Result<PolyhedronMeasureJob,DomainError>`；`fn step(job:&mut PolyhedronMeasureJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<PolyhedronMeasure1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/independent-octahedron-expectation.json`（pathBase=project-root），SHA256 `110274accebf52b3584e43fe4874d36760d42b1c3179ba13b5bf6a3951a9721d`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- regular-octahedron，role=direct-result；输出volumeA3, surfaceAreaA2；单位A3 / A2。expected：`/volumeA3Exact`, `/volumeA3`, `/surfaceAreaA2Exact`, `/surfaceAreaA2`；tolerance：无源数值容差；政策analytic-reference-no-runtime-tolerance。The six +/-Cartesian-axis ligands form the bounded convex octahedron; exact references32/3 and16sqrt3. Decimal values are independent approximations; runtime numeric tolerance remains unfrozen.

未冻结：Runtime numeric tolerance, nonconvex/degenerate cases, chemistry/BV scope。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/octahedron-working.cif`（pathBase=project-root），SHA256 `b08a9e646594d7c8419c559071fd9dbe1080934a5a079c3daff119f1d459393b`；来源指针 /records/53/path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/polyhedron/input-manifest.json`, `tests/delivery/polyhedron/actual.json`, `tests/delivery/polyhedron/independent-comparison.json`, `tests/delivery/polyhedron/resource-lifecycle.json`。当前未执行。

### G1 领域 vectors
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/7` 和其中 VF-0027, VF-0176, VF-0177, VF-0178 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/vectors.rs`；测试 `kernel/crates/geometry/tests/vector_parity.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0074 `reviews/20261109-122500-s06-magnetic-symmetry/inversion-cwd/controlled.mcif` SHA256 `3d5c1445c0164a1d278297ee8461be0f74832ff2647a7e6c58c5b2a11402e878`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：axial/polar与时间反演分开、ImageKey不按label查；不把sqrt(14)文本舍入回写冒充原向量。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test vector_parity`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新非正交单位向量，冻结H与规范化转换，单相GUI直接编辑和安全独立重开分别测；不重放已知Cancel崩溃。

#### 历史：R3接口与交接 vectors

当前设计接口：`CartesianVectorJob::new(components:Vec3,rotation:Mat3,axial:bool,time_reversal:i8)->Result<CartesianVectorJob,DomainError>`；`fn step(job:&mut CartesianVectorJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Vec3>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/axial-before-target.json`（pathBase=project-root），SHA256 `abb3e8eb2172324b665402d12b77a93058a4030d07a14889cf27dbc8581a55f9`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- axial-cartesian，role=direct-result；输出Vec3；单位Cartesian magnetic moments, source convention。expected：`/independentExpected/CartesianMoments`；tolerance：`/independentExpected/printedTolerance`；政策source-declared。Match the two fixture positions/operators and axial parity; printedTolerance applies only to original printed values, not general internal precision.

未冻结：General nonorthogonal/time-reversal domains and internal precision。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/anti-inversion-cwd/imported.vesta`（pathBase=project-root），SHA256 `805e067ba42b2b879f878ef149dc7e400dc590068d8565e21315853bd478204d`；来源指针 /source，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/vectors/input-manifest.json`, `tests/delivery/vectors/actual.json`, `tests/delivery/vectors/independent-comparison.json`, `tests/delivery/vectors/resource-lifecycle.json`。当前未执行。

### G2 领域 planes
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/8` 和其中 VF-0028, VF-0179, VF-0180, VF-0182, VF-0211 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/planes.rs`；测试 `kernel/crates/geometry/tests/plane_geometry.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0009 `s05/fixtures/nonorthogonal.cif` SHA256 `e661819c3ce9bfb87bcdfa2a28373910407803e4fa806283a9a9728298713970`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：g=inv(H).T h、d=1/norm(g)；90°/反面180°/退化输入；best-fit和实际夹角范围不能从静态acos直接通过。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test plane_geometry`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：冻结正交(100)/(010)=90°、非正交两面g夹角及反面180°候选；新GUI选中两个面验符号/范围与错误后状态。

#### 历史：R3接口与交接 planes

当前设计接口：`PlaneNormalJob::new(cell:Cell3,hkl:[i32;3],offset_a:f64)->Result<PlaneNormalJob,DomainError>`；`fn step(job:&mut PlaneNormalJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Plane1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261114-124229-p1-design-r4/contracts/plane-independent-input.json`（pathBase=project-root），SHA256 `6db2d5c10451966ddc3bfa7598360012b35feceacefe1be9021cbbb4d813257f`；性质new-independent-math-only。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- plane-angle-support，role=supporting-only；输出不作为该函数输出判据；单位degree。expected：`/expectedDegrees`；tolerance：`/toleranceDegrees`；政策source-declared。New independent pair-angle math90/180 only; neither reference execution nor direct Plane1.normal/offset expected values.

未冻结：Reference behavior and direct Plane1.unitNormal/offsetA expected values。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261114-124229-p1-design-r4/contracts/plane-independent-input.json`（pathBase=project-root），SHA256 `6db2d5c10451966ddc3bfa7598360012b35feceacefe1be9021cbbb4d813257f`；来源指针 ，identityFromOriginalRecord=false。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/planes/input-manifest.json`, `tests/delivery/planes/actual.json`, `tests/delivery/planes/independent-comparison.json`, `tests/delivery/planes/resource-lifecycle.json`。当前未执行。

### G2 领域 boundary
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/9` 和其中 VF-0061, VF-0181 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/boundary.rs`；测试 `kernel/crates/geometry/tests/boundary_images.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0052 `reviews/20261108-221538-s06-periodic-pair/periodic-working.cif` SHA256 `18892a023f638f83ec263c8e47e5fda247dd57fabb437e383779532ba9505998`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：Apply后Cancel保留；绘制边界与化学邻接分开；极斜晶胞需整数域收敛界，拒绝逐轴round最近像。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test boundary_images`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：极斜晶胞用足够整数域/收敛界枚举最近像；绘制边界与化学邻接分别核；失败前后完整保存比较。

#### 历史：R3接口与交接 boundary

当前设计接口：`BoundaryImagesJob::new(phase_id:ObjectId,ops:Vec<AffineOp3>,bounds:Bounds1)->Result<BoundaryImagesJob,DomainError>`；`fn step(job:&mut BoundaryImagesJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Vec<ImagePosition1>>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/independent-periodic-expectation.json`（pathBase=project-root），SHA256 `a674830e2d48f0a00b35f7fbd66624f9e75abc0ea67979cbca065288812390cc`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- selected-image-geometry，role=result-invariant；输出fractional, ImageKey.shift；单位fractional / integer shift / A。expected：`/closestOImageFractional`, `/closestOImageTranslation`, `/latticeEnumeration`；tolerance：无源数值容差；政策analytic-reference-no-runtime-tolerance。Use the exact selected O image(-1,0,0), not only the same atom label. Enumeration verifies this bounded orthogonal example; complete display-boundary image set is still missing.

未冻结：Complete bounded image set for general cell/bounds/operators and execution tolerance。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/periodic-working.cif`（pathBase=project-root），SHA256 `18892a023f638f83ec263c8e47e5fda247dd57fabb437e383779532ba9505998`；来源指针 /records/51/path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/boundary/input-manifest.json`, `tests/delivery/boundary/actual.json`, `tests/delivery/boundary/independent-comparison.json`, `tests/delivery/boundary/resource-lifecycle.json`。当前未执行。

### G4 领域 view
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/10` 和其中 VF-0030, VF-0031, VF-0032, VF-0033, VF-0034, VF-0062, VF-0183, VF-0184, VF-0185, VF-0186, VF-0187, VF-0188, VF-0189, VF-0190, VF-0191, VF-0192, VF-0193, VF-0194, VF-0195, VF-0196, VF-0197, VF-0198, VF-0199, VF-0200, VF-0201, VF-0202, VF-0203 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `web/src/render/camera.ts`；测试 `tests/browser/view_transform.spec.ts`。消费渲染 packet，不在 TS 重算科学状态。
fixture：VS-0009 `s05/fixtures/nonorthogonal.cif` SHA256 `e661819c3ce9bfb87bcdfa2a28373910407803e4fa806283a9a9728298713970`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：CLI RyRx与GUI Rx45不同测试；非正交a与a*不同方向、width/透视/奇异向上向量另核。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`npm run test:browser -- --grep view_transform`。预算 B-WEB；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：冻结非正交a与a*不同的基底，分别核方向、向上正交化、保存矩阵及实际投影；不以相同site当view成功。

#### 历史：R3接口与交接 view

当前设计接口：`export function buildCameraPacket(input:{rotation:Mat3;projection:Projection1;zoom:number},identity:PageIdentity1):Camera1`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Renderer/page adapter consumes Worker-committed packets and same PageIdentity1; UI changes go through formal page commands, no TS science reimplementation and no direct host write.

来源：`output/vesta-study/20261108-113622/reviews/20261109-163500-s04-view-camera/camera-before-targets.json`（pathBase=project-root），SHA256 `c59f36ea2ca0cd0de5c792a469b0d566ee2ad3d4bfa525d80682ae7d79fce2a3`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- camera-zoom，role=result-invariant；输出zoom；单位dimensionless zoom。expected：`/predictions/zoom/baseline/zoom`, `/predictions/zoom/in/zoom`, `/predictions/zoom/out/zoom`, `/predictions/zoom/fit/zoom`；tolerance：无源数值容差；政策analytic-reference-no-runtime-tolerance。1.25/1.375/1.25/1 apply to this saved camera sequence. Source PROJT/DISPF flags support projection compatibility, not the full Camera1 rotation.

未冻结：Camera1 rotation/projection enum mapping and numerical tolerance。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261109-154800-s04-output-text/model-control-cwd/clear-model-only.vesta`（pathBase=project-root），SHA256 `d4c508851f6fe92c09e2931e31079dff6d9468fe748d950250887dab16bc9f48`；来源指针 /source，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-163500-s04-view-camera/zoom-cwd/zoom-source.vesta`（pathBase=project-root），SHA256 `79a5d6f0992df11aac090e796d73abeb420cf89b2533051a791c441a2591a82c`；来源指针 /cases/zoom/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-163500-s04-view-camera/projection-cwd/projection-source.vesta`（pathBase=project-root），SHA256 `d4c508851f6fe92c09e2931e31079dff6d9468fe748d950250887dab16bc9f48`；来源指针 /cases/projection/workingInput，identityFromOriginalRecord=true。

取消：Check AbortSignal and document/epoch/revision before frame/bytes delivery. Abort only this consumer request; do not undo a committed domain mutation. After host primary commit, query snapshot instead of claiming cancel prevented write. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/view/input-manifest.json`, `tests/delivery/view/actual.json`, `tests/delivery/view/independent-comparison.json`, `tests/delivery/view/resource-lifecycle.json`。当前未执行。

### G2 领域 measure
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/11` 和其中 VF-0064, VF-0208, VF-0209, VF-0210, VF-0224, VF-0231 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/measure.rs`；测试 `kernel/crates/geometry/tests/measurement.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0052 `reviews/20261108-221538-s06-periodic-pair/periodic-working.cif` SHA256 `18892a023f638f83ec263c8e47e5fda247dd57fabb437e383779532ba9505998`；VS-0055 `reviews/20261108-235339-s06-distance-uncertainty/sue-working.cif` SHA256 `4ab83a950e032dadc77ce5f1ea80ca62046cb3383337fa2ae99dd0b84bb85c4f`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：距离9/1Å、对角SUE方差.000504；无符号二面角和显示formatter分判，非对角协方差不能沿用对角例。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test measurement`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-02)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：冻结退化与跨周期所选image、协方差非对角反例；ORFFE producer缺失不伪造计算输出，保留读取与计算之别。 VF-0224：冻结新非正交一般/特殊位置以及重复label图像，分别点选已知image并核f、Hf、affine symmetry、integer translation及报告刷新；失败/空选状态保留。

#### 历史：R3接口与交接 measure

当前设计接口：`SelectedMeasureJob::new(points:Vec<ImagePosition1>,kind:MeasureKind1)->Result<SelectedMeasureJob,DomainError>`；`fn step(job:&mut SelectedMeasureJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Measurement1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/independent-distance-sue.json`（pathBase=project-root），SHA256 `bf34e1c22316030d3bb727b7645a60725496fe577c5888a847dacd87db40d495`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- selected-distance，role=direct-result；输出value, unit；单位A。expected：`/distanceA`；tolerance：无源数值容差；政策analytic-reference-no-runtime-tolerance。Central selected P1 distance is2A. Standard uncertainty is deliberately not in Measurement1 result; no hidden covariance field.
- sue-support，role=supporting-only；输出不作为该函数输出判据；单位A2 / A。expected：`/varianceA2`, `/standardUncertaintyA`, `/diagnosticCoordinateOnlySigmaA`；tolerance：无源数值容差；政策unfrozen。Independent diagonal-SUE support only, not a public Measurement1 covariance contract.

未冻结：Execution tolerance and angle/dihedral/degenerate/image/SUE full contract。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-working.cif`（pathBase=project-root），SHA256 `4ab83a950e032dadc77ce5f1ea80ca62046cb3383337fa2ae99dd0b84bb85c4f`；来源指针 /records/54/path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/measure/input-manifest.json`, `tests/delivery/measure/actual.json`, `tests/delivery/measure/independent-comparison.json`, `tests/delivery/measure/resource-lifecycle.json`。当前未执行。

### G2 领域 tidy
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/12` 和其中 VF-0065, VF-0066 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/reduction.rs`；测试 `kernel/crates/geometry/tests/tidy_result.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0064 `s05/independent-niggli-orthogonal.cif` SHA256 `5133da551618f7663e7284e54e66582c8e7a3e2e17a059ec27a43ce443b707e9`；VS-0103 `reviews/20261110-150652-s07-contract-audit/tidy-work/valid/input.stin` SHA256 `5e5df67102ada2941b41c75978fabc644d71cf5ee6fabdac976ecd7b151cd537`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：结构更新与文本报告分开，保体积/metric不变量；错误群exit0仍失败；完整Niggli/setting需独立来源。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test tidy_result`。预算 B-EXTERNAL；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-ASSET-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新等价三斜胞双输入，在目标前冻结metric不变量、轴变换/原点和独立约化来源；外部执行生命周期归S07。

#### 历史：R3接口与交接 tidy

当前设计接口：`TidyParseJob::new(bytes:ResourceId,format_profile:ProfileId)->Result<TidyParseJob,DomainError>`；`fn step(job:&mut TidyParseJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ProposedStructure1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-055000-s06-tidy-niggli/independent-tidy-niggli.json`（pathBase=project-root），SHA256 `f03d700173e4296c4f675b2fc95bc939a1b50625ccd7f864152110d3ebbdfaef`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- niggli-metric，role=result-invariant；输出cell, sites；单位A / degree / A3 / fractional。expected：`/inputs/orthogonal/expectedNiggliCell`, `/inputs/orthogonal/expectedVolumeA3`, `/inputs/orthogonal/niggliCandidateRelativeCoordinates`, `/inputs/orthogonal/periodicPairDistanceExpectations`, `/inputs/skew/expectedNiggliCell`, `/inputs/skew/expectedVolumeA3`, `/inputs/skew/niggliCandidateRelativeCoordinates`, `/inputs/skew/periodicPairDistanceExpectations`；tolerance：`/cellAbsoluteTolerance`, `/angleAbsoluteToleranceDegrees`, `/distanceAbsoluteToleranceA`, `/fractionalModuloTolerance`, `/occupancyTolerance`, `/UisoTolerance`；政策source-declared。Compare metric, origin-free relative candidates modulo1 and pair distances by matching input case; these Niggli invariants are not proof that an external Tidy tool ran.

未冻结：External-tool successful result/normalization and full source-resource proposal validation。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/independent-niggli-orthogonal.cif`（pathBase=project-root），SHA256 `5133da551618f7663e7284e54e66582c8e7a3e2e17a059ec27a43ce443b707e9`；来源指针 /inputs/orthogonal/source，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/s05/independent-niggli-skew.cif`（pathBase=project-root），SHA256 `a407ac2fed5a2b1c37860ddb6a0c337960839fd29c43321bb96929ff90dacfef`；来源指针 /inputs/skew/source，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. External process phase, when separately authorized, uses owned process group TERM/grace/KILL and actual exit/output query; never replay on unknown completion. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/tidy/input-manifest.json`, `tests/delivery/tidy/actual.json`, `tests/delivery/tidy/independent-comparison.json`, `tests/delivery/tidy/resource-lifecycle.json`。当前未执行。

### G5 领域 madel
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/13` 和其中 VF-0067, VF-0232, VF-0233 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/external/madel.rs`；测试 `kernel/crates/science/tests/madel_external.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0052 `reviews/20261108-221538-s06-periodic-pair/periodic-working.cif` SHA256 `18892a023f638f83ec263c8e47e5fda247dd57fabb437e383779532ba9505998`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：既有包main前缺库崩溃只验证错误；未来中性NaCl/Ewald收敛对照必须先有获准运行件，不能拿零输出当成功。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test madel_external`。预算 B-EXTERNAL；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-ASSET-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：在具备已授权依赖的后续环境，用中性NaCl/独立Ewald和RADIUS/REGION收敛曲线；本卡不安装。

#### 历史：R3接口与交接 madel

当前设计接口：`MadelInputJob::new(phase_id:ObjectId,charges:Vec<SiteCharge1>,controls:MadelControls1)->Result<MadelInputJob,DomainError>`；`fn step(job:&mut MadelInputJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ExternalInputBundle1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-original-observation.json`（pathBase=project-root），SHA256 `54f3ca8f09b7c85c5800ca0f9566b219847b144dbcff3aaa1e16059f4d5b4750`；性质observed-environment-failure-no-numeric-oracle。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。


未冻结：Original environment failure has no success numeric oracle or ExternalInputBundle1 expected bytes; argv/input SHA are provenance only。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-original-observation.json`（pathBase=project-root），SHA256 `54f3ca8f09b7c85c5800ca0f9566b219847b144dbcff3aaa1e16059f4d5b4750`；来源指针 ，identityFromOriginalRecord=false。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. External process phase, when separately authorized, uses owned process group TERM/grace/KILL and actual exit/output query; never replay on unknown completion. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/madel/input-manifest.json`, `tests/delivery/madel/actual.json`, `tests/delivery/madel/independent-comparison.json`, `tests/delivery/madel/resource-lifecycle.json`。当前未执行。

### G3 领域 powder
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/14` 和其中 VF-0068, VF-0234, VF-0235, VF-0236, VF-0237 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/powder.rs`；测试 `kernel/crates/science/tests/powder_pattern.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0070 `s05/independent-transform-p1.cif` SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：d6/lambda1的2theta=9.560383694°；首点目标0与独立正积分失败并存；多反射/绝对强度单独冻结。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test powder_pattern`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：冻结新多反射/多相与绝对强度独立表，保持首点失败；不要扩大归一内部峰形通过。外部联动后续S07核，当前不装RIETAN。

#### 历史：R3接口与交接 powder

当前设计接口：`BraggPositionsJob::new(d_spacings_a:Vec<f64>,wavelength_a:f64)->Result<BraggPositionsJob,DomainError>`；`fn step(job:&mut BraggPositionsJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Vec<f64>>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-101000-s06-powder-positive/powder-before-gui.json`（pathBase=project-root），SHA256 `59bf51dcf9d3865ca8e23809c1d6df09480dba95868da333fe92a696b2bdacec`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- bragg-positions，role=direct-result；输出Vec<f64> twoTheta；单位degree / A。expected：`/independentExpected/allAllowedSignedHKL`, `/independentExpected/dA`, `/independentExpected/twoThetaDeg`；tolerance：`/independentExpected/reflectionAbsoluteToleranceDeg`, `/independentExpected/reflectionAbsoluteToleranceDA`, `/independentExpected/sampledPeakNearestGridToleranceDeg`；政策source-declared。lambda1A, window9..10degree, signed(0,0,+/-1), d6A and2theta9.560383694398318. Distinguish Bragg-position tolerance from sampled-profile nearest-grid tolerance; no intensity accuracy claim.

未冻结：Full intensity/profile/multiphase and remaining reflected domains。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/independent-transform-p1.cif`（pathBase=project-root），SHA256 `a706350a49871ab3379aa7a986ff3827270b60ce3ba89e1f9a2ec07c74927afa`；来源指针 /sourcePath，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/powder/input-manifest.json`, `tests/delivery/powder/actual.json`, `tests/delivery/powder/independent-comparison.json`, `tests/delivery/powder/resource-lifecycle.json`。当前未执行。

### G3 领域 sf
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/15` 和其中 VF-0069, VF-0238, VF-0239 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/structure_factors.rs`；测试 `kernel/crates/science/tests/structure_factors.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0053 `reviews/20261108-224603-s06-inversion-sites/symmetry-working.cif` SHA256 `f040c441371611018284b31e73a0692c930a5de307fe4e27279a1c725b5f97fc`；VS-0055 `reviews/20261108-235339-s06-distance-uncertainty/sue-working.cif` SHA256 `4ab83a950e032dadc77ce5f1ea80ca62046cb3383337fa2ae99dd0b84bb85c4f`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：正相位、occ/ADP/散射表版本，P21禁戒反射；反常边双侧/负bc/多群不以DC替代完整谱。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test structure_factors`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新元素/反常边双侧与非正交U；先绑定实际输入lambda/参数版本，不能拿AX显示舍入值代替模型double。

#### 历史：R3接口与交接 sf

当前设计接口：`ScatteringSumJob::new(terms:Vec<ScatteringTerm1>,unit:ScatteringUnit1)->Result<ScatteringSumJob,DomainError>`；`fn step(job:&mut ScatteringSumJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Vec<Reflection1>>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-005135-s06-structure-factors/independent-structure-factors.json`（pathBase=project-root），SHA256 `de835b4319f583f13c3482f5e5058c381be65d818dfae2949e5159333548b832`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- six-reflections，role=direct-result；输出hkl, complexF；单位source electron scattering amplitude / phase degree。expected：`/fullReflectionsBeforeFriedelReduction`；tolerance：`/comparisonPolicy`；政策source-declared。Match each signed HKL; real/imag primary decimal dataset is independent. Printed values use half-last-unit, phase modulo360; binary32 alternative stays a separate control, not fitted expected output.

未冻结：General scattering/profile/ADP/dispersion domain and internal tolerance。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261108-235339-s06-distance-uncertainty/sue-working.cif`（pathBase=project-root），SHA256 `4ab83a950e032dadc77ce5f1ea80ca62046cb3383337fa2ae99dd0b84bb85c4f`；来源指针 /records/54/path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/sf/input-manifest.json`, `tests/delivery/sf/actual.json`, `tests/delivery/sf/independent-comparison.json`, `tests/delivery/sf/resource-lifecycle.json`。当前未执行。

### G3 领域 fourier
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/16` 和其中 VF-0070, VF-0240, VF-0241, VF-0242 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/fourier.rs`；测试 `kernel/crates/science/tests/fourier_field.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0059 `s05/independent-three-axis-fourier.txt` SHA256 `885a50deb6de83f38819484a1a969670d0ff3d3d1770e51b16726ba305b24feb`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：负相位、1/V逆变换、F000独立、共轭补全；完整288点原期望可复用，Fo-Fo/Nyquist另冻结。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test fourier_field`。预算 B-GRID；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新非平凡群禁戒反射和相位缺失，运行前冻结补全集合/DC/谱与所有体素；保留拒绝和脏状态。

#### 历史：R3接口与交接 fourier

当前设计接口：`FourierFieldJob::new(cell:Cell3,grid:GridSpec1,terms:Vec<Reflection1>)->Result<FourierFieldJob,DomainError>`；`fn step(job:&mut FourierFieldJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<FieldResult1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-031600-s06-fourier-three-axes/independent-three-axis-fourier.json`（pathBase=project-root），SHA256 `e6fef1ea9de5959a7812e02db90c5e330fb18cb916558cfc33bd52339ce1e377`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- analytic-field，role=result-invariant；输出field values, integral；单位electron/A3 / electron。expected：`/densityFormula`, `/reflections`, `/independentExpectedIntegral`；tolerance：`/pointwiseAbsoluteTolerance`, `/integralAbsoluteTolerance`；政策source-declared。Evaluate the explicit independent finite-spectrum formula at observed periodic-core indices; integral8. GridDimsPreregistered=false is a missing dimension policy, never an expected null or false dimension.

未冻结：General spectrum/Nyquist and fixed grid-selection policy。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/independent-three-axis-fourier.txt`（pathBase=project-root），SHA256 `885a50deb6de83f38819484a1a969670d0ff3d3d1770e51b16726ba305b24feb`；来源指针 /samplePath，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/fourier/input-manifest.json`, `tests/delivery/fourier/actual.json`, `tests/delivery/fourier/independent-comparison.json`, `tests/delivery/fourier/resource-lifecycle.json`。当前未执行。

### G3 领域 electron
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/17` 和其中 VF-0071, VF-0243 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/density.rs`；测试 `kernel/crates/science/tests/electron_density.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0053 `reviews/20261108-224603-s06-inversion-sites/symmetry-working.cif` SHA256 `f040c441371611018284b31e73a0692c930a5de307fe4e27279a1c725b5f97fc`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：F00014与积分14只是DC；gridNo对称/resolution必须单独输入，不用axis/res猜一般格数。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test electron_density`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新对称与粗/细resolution小矩阵冻结格数候选及F000，资源渐进有界记录RSS/时间；不从一次6³推一般格数政策。

#### 历史：R3接口与交接 electron

当前设计接口：`ElectronFieldJob::new(cell:Cell3,grid:GridSpec1,terms:Vec<Reflection1>)->Result<ElectronFieldJob,DomainError>`；`fn step(job:&mut ElectronFieldJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<FieldResult1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-001810-s06-density-f000/independent-density-f000.json`（pathBase=project-root），SHA256 `9605ff954f579d27150117dd0cfb098321fe2b521cec8fabafd0de5a6105bf06`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- f000-integral，role=result-invariant；输出field integral, mean；单位electron / electron/A3。expected：`/neutralPhysicalF000Electrons`, `/meanElectronDensityEA3`, `/checks/integralExpectedElectrons`, `/checks/inversionSymmetry`；tolerance：`/checks/integralAbsoluteToleranceElectrons`, `/checks/inversionAbsoluteToleranceEA3`；政策source-declared。Fixture F00014, mean0.014; unique periodic core and source nominal volume1000A3. Inversion check is a source predicate. generatedGridCountExpected=null is explicitly not an expected grid size.

未冻结：Full voxel spectrum and grid dimension policy。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/symmetry-working.cif`（pathBase=project-root），SHA256 `f040c441371611018284b31e73a0692c930a5de307fe4e27279a1c725b5f97fc`；来源指针 /records/52/path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/electron/input-manifest.json`, `tests/delivery/electron/actual.json`, `tests/delivery/electron/independent-comparison.json`, `tests/delivery/electron/resource-lifecycle.json`。当前未执行。

### G3 领域 nuclear
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/18` 和其中 VF-0072 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/density.rs`；测试 `kernel/crates/science/tests/nuclear_density.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0055 `reviews/20261108-235339-s06-distance-uncertainty/sue-working.cif` SHA256 `4ab83a950e032dadc77ce5f1ea80ca62046cb3383337fa2ae99dd0b84bb85c4f`。实际H/O衍生输入见 `reviews/20261110-135709-s06-domain-study/gui/nuclear/input.cif`（与两种Patterson输入同SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`），事前判据在该轮 density-oracle-before-target.json；不把VS55原SUE输入和衍生H/O混同。
- [ ] 未来先写独立断言：原H/O积分2.064fm为指定样本，不复用VS55原SUE输入当成H/O输入；精确H/O输入从原规则证据定位，负bc全谱另核。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test nuclear_density`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新有限B、非正交/负bc样本先冻结完整谱与Nyquist约定，再核逐点；保留历史NIST与新版表不同。

#### 历史：R3接口与交接 nuclear

当前设计接口：`NuclearFieldJob::new(cell:Cell3,grid:GridSpec1,terms:Vec<Reflection1>)->Result<NuclearFieldJob,DomainError>`；`fn step(job:&mut NuclearFieldJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<FieldResult1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/density-oracle-before-target.json`（pathBase=project-root），SHA256 `b2aa1209126157f1a149cbfa5d6bc3498dd8775a32cbcb12763988b6584caffb`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- nuclear-dc，role=result-invariant；输出field integral；单位fm。expected：`/cases/0/dc_integral`, `/cases/0/unit`；tolerance：`/cases/0/integral_absolute_tolerance`, `/cases/0/tolerance_reason`；政策source-declared。Only case=nuclear gives2.064fm; do not mix the squared Patterson cases or claim all voxel values.

未冻结：Full nuclear voxel/spectrum/isotope grid policy。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261109-014900-s06-neutron/neutron-ho-working.cif`（pathBase=project-root），SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`；来源指针 /cases/2/source，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/gui/nuclear/input.cif`（pathBase=project-root），SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`；来源指针 /cases/0/input，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/gui/patterson-nuclear/input.cif`（pathBase=project-root），SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`；来源指针 /cases/1/input，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/gui/patterson-electron/input.cif`（pathBase=project-root），SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`；来源指针 /cases/2/input，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/nuclear/input-manifest.json`, `tests/delivery/nuclear/actual.json`, `tests/delivery/nuclear/independent-comparison.json`, `tests/delivery/nuclear/resource-lifecycle.json`。当前未执行。

### G3 领域 model-patterson
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/19` 和其中 VF-0073, VF-0074 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/patterson.rs`；测试 `kernel/crates/science/tests/model_patterson.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0055 `reviews/20261108-235339-s06-distance-uncertainty/sue-working.cif` SHA256 `4ab83a950e032dadc77ce5f1ea80ca62046cb3383337fa2ae99dd0b84bb85c4f`。实际H/O衍生输入见 `reviews/20261110-135709-s06-domain-study/gui/nuclear/input.cif`（与两种Patterson输入同SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`），事前判据在该轮 density-oracle-before-target.json；不把VS55原SUE输入和衍生H/O混同。
- [ ] 未来先写独立断言：4.260096fm²/81e²为原H/O控制预期，VS55原SUE输入只供来源定位；原H/O实际输入从原证据取，偶对称/DC不代全体素。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test model_patterson`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：独立有限分辨率和式的全体素新输入留出，先查实际反射集合/采样，再冻结oracle，不用两条原件往返互证。

#### 历史：R3接口与交接 model-patterson

当前设计接口：`ModelPattersonJob::new(cell:Cell3,grid:GridSpec1,terms:Vec<Reflection1>,unit:ScatteringUnit1)->Result<ModelPattersonJob,DomainError>`；`fn step(job:&mut ModelPattersonJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<FieldResult1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/density-oracle-before-target.json`（pathBase=project-root），SHA256 `b2aa1209126157f1a149cbfa5d6bc3498dd8775a32cbcb12763988b6584caffb`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- patterson-dc，role=result-invariant；输出field integral, evenness, origin maximum；单位fm2 / electron2。expected：`/cases/1/dc_integral`, `/cases/1/unit`, `/cases/1/additional_predicates`, `/cases/2/dc_integral`, `/cases/2/unit`, `/cases/2/additional_predicates`；tolerance：`/cases/1/integral_absolute_tolerance`, `/cases/1/tolerance_reason`, `/cases/2/integral_absolute_tolerance`, `/cases/2/tolerance_reason`；政策source-declared。Case1=4.260096fm2 and case2=81electron2. Source additional_predicates carry1e-6 evenness/origin-max limits; preserve each case unit and nominal volume.

未冻结：Full voxel/spectrum/Nyquist and general model domain。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261109-014900-s06-neutron/neutron-ho-working.cif`（pathBase=project-root），SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`；来源指针 /cases/2/source，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/gui/nuclear/input.cif`（pathBase=project-root），SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`；来源指针 /cases/0/input，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/gui/patterson-nuclear/input.cif`（pathBase=project-root），SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`；来源指针 /cases/1/input，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/gui/patterson-electron/input.cif`（pathBase=project-root），SHA256 `2448e20c788c1bb9736d95748c69bbf666d6d04f9df44e771562cbc698d6808d`；来源指针 /cases/2/input，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/model-patterson/input-manifest.json`, `tests/delivery/model-patterson/actual.json`, `tests/delivery/model-patterson/independent-comparison.json`, `tests/delivery/model-patterson/resource-lifecycle.json`。当前未执行。

### G3 领域 volume-patterson
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/20` 和其中 VF-0075 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/patterson.rs`；测试 `kernel/crates/science/tests/volume_patterson.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0059 `s05/independent-three-axis-fourier.txt` SHA256 `885a50deb6de83f38819484a1a969670d0ff3d3d1770e51b16726ba305b24feb`；VS-0079 `reviews/20261110-085833-s05-formats-audit/axis-sentinel.pgrid` SHA256 `f3faefbb448679e6ec7843d0f742e1517682bcfeeae4be23aec391bed359dde0`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：V/N正向、模方、1/V逆向；新小网格独立O(N²)卷积/积分；signed相关不一律截零。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test volume_patterson`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新非正交有理小网格独立O(N²)卷积+积分，逐点比较并区分core与闭合边界。

#### 历史：R3接口与交接 volume-patterson

当前设计接口：`VolumePattersonJob::new(request:PattersonRequest1,input:FieldDescriptor2)->Result<VolumePattersonJob,DomainError>`；`fn step(job:&mut VolumePattersonJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<FieldResult1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-025400-s06-patterson-volume/independent-patterson-volume.json`（pathBase=project-root），SHA256 `a1f2a660683a143b717a93aee0fdd09c3567326acbfe070365d261db0bbf31bc`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- discrete-convolution，role=result-invariant；输出field values, integral；单位source Patterson unit / integral。expected：`/expectedDx`, `/expectedIntegral`, `/analyticPatterson`；tolerance：`/primaryPointwiseAbsTolerance`, `/integralAbsTolerance`；政策source-declared。Use independentDiscreteConvolutionOfActualInput at each dx for actual rounded bytes. idealAnalytic and inputTextRoundingDifference are distinct diagnostics; integral64, core6^3.

未冻结：General grid/Nyquist/nonorthogonal and full-domain validation。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261109-022700-s06-analytic-fourier/fc-density.cube`（pathBase=project-root），SHA256 `fd1dbe891b852bd86df8791a6c1c67fe0d2c41912c56d92c291587d45d7cd34e`；来源指针 /sourceArtifact，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/volume-patterson/input-manifest.json`, `tests/delivery/volume-patterson/actual.json`, `tests/delivery/volume-patterson/independent-comparison.json`, `tests/delivery/volume-patterson/resource-lifecycle.json`。当前未执行。

### G3 领域 volume-data
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/21` 和其中 VF-0024, VF-0107, VF-0108, VF-0109, VF-0110, VF-0150 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/field.rs`；测试 `kernel/crates/science/tests/field_layout.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0079 `reviews/20261110-085833-s05-formats-audit/axis-sentinel.pgrid` SHA256 `f3faefbb448679e6ec7843d0f742e1517682bcfeeae4be23aec391bed359dde0`；VS-0102 `reviews/20261110-135709-s06-domain-study/resources/medium/input.cube` SHA256 `1e942ad6048c02ff23fde5b82d58646c1995a68ed65cb39a051dc9fd6d576140`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：x+10y+100z轴哨兵；core/closure分列；45084闭合值只为已测规模，重采样/双场运算顺序另核。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test field_layout`。预算 B-GRID；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：两个3轴不同尺寸解析标量，冻结A+B/A-B/乘系数/平移/重采样各候选与周期；记录失败后原layer/保存引用。

#### 历史：R3接口与交接 volume-data

当前设计接口：`FieldLayoutJob::new(input:FieldDescriptor2,target_axis:AxisOrder1)->Result<FieldLayoutJob,DomainError>`；`fn step(job:&mut FieldLayoutJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<FieldResult1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/resource-oracle-before-target.json`（pathBase=project-root），SHA256 `fa5e12107678c4b607f07bb9eadc5f9dcc3c34471f858e21cfdfccb415cf382c`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- axis-sentinel，role=result-invariant；输出descriptor shape/order, field values；单位dimensionless scalar / index。expected：`/cases/0/formula`, `/cases/0/expectedOutputDims`, `/cases/0/expectedOutputOrder`, `/cases/1/formula`, `/cases/1/expectedOutputDims`, `/cases/1/expectedOutputOrder`；tolerance：`/cases/0/scalarTolerance`, `/cases/1/scalarTolerance`；政策source-declared。Evaluate10000i+100j+k-150000 and periodic padding by modulo for the two frozen input sizes. scalarTolerance=0 is source-declared exact representable integer equality.

未冻结：General scalar kinds/interpolation and production resource budgets。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/resources/medium/input.cube`（pathBase=project-root），SHA256 `1e942ad6048c02ff23fde5b82d58646c1995a68ed65cb39a051dc9fd6d576140`；来源指针 /cases/0/input，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/resources/larger/input.cube`（pathBase=project-root），SHA256 `4bd3c3ece1a80fb98c5e6cc584767c3768144ca6daea27a4d1bb94a265e92763`；来源指针 /cases/1/input，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/volume-data/input-manifest.json`, `tests/delivery/volume-data/actual.json`, `tests/delivery/volume-data/independent-comparison.json`, `tests/delivery/volume-data/resource-lifecycle.json`。当前未执行。

### G2 领域 shape
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/22` 和其中 VF-0025, VF-0111, VF-0112, VF-0113, VF-0114, VF-0115, VF-0116, VF-0151, VF-0051, VF-0052, VF-0053, VF-0054 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/geometry/src/shape.rs`；测试 `kernel/crates/geometry/tests/halfspace_shape.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0018 `s05/official-examples/VESTA-Examples/VESTA/Morphologty_data/Adularia-8twins.vesta` SHA256 `0635fe7c321415204948c389b899c5b6a767ccfee2bbd828d32532c9543d4343`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：n dot x<=D而非D=d_hkl；八相48面所选面积精度；不闭合prism/重复hkl/零面必须另立预期。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test halfspace_shape`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新立方八面体与非闭合prism、重复hkl，事前算有界性/面积/面距离，独立核每个相的R/t。

#### 历史：R3接口与交接 shape

当前设计接口：`HalfspaceShapeJob::new(planes:Vec<Plane1>)->Result<HalfspaceShapeJob,DomainError>`；`fn step(job:&mut HalfspaceShapeJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ShapeMesh1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261108-200510-s04-multiphase-shapes/shape-geometry-crosscheck.json`（pathBase=project-root），SHA256 `9f9dcff928928583663ec6c8d839c4845cb3e93ada6881e834d1bd847dd27835`；性质post-observation-independent-recompute。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- local-halfspace-mesh，role=direct-result；输出verticesA, faces；单位A / polygon topology。expected：`/phases`；tolerance：无源数值容差；政策unfrozen。Source independent convex half-space vertices and face hkl describe local mesh. Vertex permutation/face orientation/mesh tolerance is still unfrozen; do not use UI displayed area as direct mesh equality.
- area-crosscheck，role=result-invariant；输出derived local face area；单位A2。expected：`/comparisons/0/expectedArea`, `/comparisons/1/expectedArea`, `/comparisons/2/expectedArea`, `/comparisons/3/expectedArea`, `/comparisons/4/expectedArea`, `/comparisons/5/expectedArea`, `/comparisons/6/expectedArea`, `/comparisons/7/expectedArea`, `/comparisons/8/expectedArea`, `/comparisons/9/expectedArea`, `/comparisons/10/expectedArea`, `/comparisons/11/expectedArea`, `/comparisons/12/expectedArea`, `/comparisons/13/expectedArea`, `/comparisons/14/expectedArea`, `/comparisons/15/expectedArea`, `/comparisons/16/expectedArea`, `/comparisons/17/expectedArea`, `/comparisons/18/expectedArea`, `/comparisons/19/expectedArea`, `/comparisons/20/expectedArea`, `/comparisons/21/expectedArea`, `/comparisons/22/expectedArea`, `/comparisons/23/expectedArea`, `/comparisons/24/expectedArea`, `/comparisons/25/expectedArea`, `/comparisons/26/expectedArea`, `/comparisons/27/expectedArea`, `/comparisons/28/expectedArea`, `/comparisons/29/expectedArea`, `/comparisons/30/expectedArea`, `/comparisons/31/expectedArea`, `/comparisons/32/expectedArea`, `/comparisons/33/expectedArea`, `/comparisons/34/expectedArea`, `/comparisons/35/expectedArea`, `/comparisons/36/expectedArea`, `/comparisons/37/expectedArea`, `/comparisons/38/expectedArea`, `/comparisons/39/expectedArea`, `/comparisons/40/expectedArea`, `/comparisons/41/expectedArea`, `/comparisons/42/expectedArea`, `/comparisons/43/expectedArea`, `/comparisons/44/expectedArea`, `/comparisons/45/expectedArea`, `/comparisons/46/expectedArea`, `/comparisons/47/expectedArea`；tolerance：`/comparisons/0/comparisonTolerance`, `/comparisons/1/comparisonTolerance`, `/comparisons/2/comparisonTolerance`, `/comparisons/3/comparisonTolerance`, `/comparisons/4/comparisonTolerance`, `/comparisons/5/comparisonTolerance`, `/comparisons/6/comparisonTolerance`, `/comparisons/7/comparisonTolerance`, `/comparisons/8/comparisonTolerance`, `/comparisons/9/comparisonTolerance`, `/comparisons/10/comparisonTolerance`, `/comparisons/11/comparisonTolerance`, `/comparisons/12/comparisonTolerance`, `/comparisons/13/comparisonTolerance`, `/comparisons/14/comparisonTolerance`, `/comparisons/15/comparisonTolerance`, `/comparisons/16/comparisonTolerance`, `/comparisons/17/comparisonTolerance`, `/comparisons/18/comparisonTolerance`, `/comparisons/19/comparisonTolerance`, `/comparisons/20/comparisonTolerance`, `/comparisons/21/comparisonTolerance`, `/comparisons/22/comparisonTolerance`, `/comparisons/23/comparisonTolerance`, `/comparisons/24/comparisonTolerance`, `/comparisons/25/comparisonTolerance`, `/comparisons/26/comparisonTolerance`, `/comparisons/27/comparisonTolerance`, `/comparisons/28/comparisonTolerance`, `/comparisons/29/comparisonTolerance`, `/comparisons/30/comparisonTolerance`, `/comparisons/31/comparisonTolerance`, `/comparisons/32/comparisonTolerance`, `/comparisons/33/comparisonTolerance`, `/comparisons/34/comparisonTolerance`, `/comparisons/35/comparisonTolerance`, `/comparisons/36/comparisonTolerance`, `/comparisons/37/comparisonTolerance`, `/comparisons/38/comparisonTolerance`, `/comparisons/39/comparisonTolerance`, `/comparisons/40/comparisonTolerance`, `/comparisons/41/comparisonTolerance`, `/comparisons/42/comparisonTolerance`, `/comparisons/43/comparisonTolerance`, `/comparisons/44/comparisonTolerance`, `/comparisons/45/comparisonTolerance`, `/comparisons/46/comparisonTolerance`, `/comparisons/47/comparisonTolerance`；政策source-declared。48 local faces only; original displayed-area tolerance is not a mesh coordinate tolerance and does not prove general phase R/t.

未冻结：General topology/tolerance, nonconvex/unbounded shape and phase placement。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/VESTA/Morphologty_data/Adularia-8twins.vesta`（pathBase=project-root），SHA256 `0635fe7c321415204948c389b899c5b6a767ccfee2bbd828d32532c9543d4343`；来源指针 /source，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/shape/input-manifest.json`, `tests/delivery/shape/actual.json`, `tests/delivery/shape/independent-comparison.json`, `tests/delivery/shape/resource-lifecycle.json`。当前未执行。

### G3 领域 volume-render
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/23` 和其中 VF-0045, VF-0046, VF-0047, VF-0048, VF-0049, VF-0050, VF-0059, VF-0060, VF-0138, VF-0139, VF-0140, VF-0141, VF-0142, VF-0160, VF-0161, VF-0162, VF-0163, VF-0164, VF-0172 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/isosurface.rs`；测试 `kernel/crates/science/tests/iso_geometry.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0062 `s05/independent-nonorthogonal-positive-density.cube` SHA256 `b3c5a5bb6c4c72cc1df9b536a19ce6ac36777ef0a60bf8176aa231ba83412f29`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：合成rho=x+2y-3z的2³/3³体素、交点/法向/封口预期；STL非有限法向为失败，不用旧文件当正确网格。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test iso_geometry`。预算 B-GRID；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：小2³/3³解析rho=x+2y-3z与零/负等值，预交点/包围盒/边界cap，分别导出mesh与颜色；另用2D实际坐标核等高线。

#### 历史：R3接口与交接 volume-render

当前设计接口：`IsoEdgeCrossingsJob::new(input:FieldDescriptor2,level:f64)->Result<IsoEdgeCrossingsJob,DomainError>`；`fn step(job:&mut IsoEdgeCrossingsJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Vec<Vec3>>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-210600-s04-isosurface-properties/isosurface-cube-before-functional-targets.json`（pathBase=project-root），SHA256 `4a8377874f332d4774d5283b64b8f03e4e303e9d0f21a68570e9534263c20919`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- serialization-support，role=supporting-only；输出不作为该函数输出判据；单位DISPF / TEX3P saved flags。expected：`/oracles`；tolerance：无源数值容差；政策exact-discrete。Observed visibility/inverse-color/Cancel saved-state contract only. nativeInitialHypothesis polygon count is a hypothesis and not an independent IsoEdgeCrossingsJob numeric oracle.

未冻结：Vec<Vec3> edge crossings, degeneracy/caps and scientific geometry tolerance。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261109-172600-s04-volume-display/visibility-cwd/visibility-volume.cube`（pathBase=project-root），SHA256 `04a77cbbe05ff93df31612dfc25d5a38c8f69bb4d1fd853277e63aa098b13f68`；来源指针 /cases/inverse_cube/source，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-210600-s04-isosurface-properties/front_cube-cwd/visibility-volume.cube`（pathBase=project-root），SHA256 `04a77cbbe05ff93df31612dfc25d5a38c8f69bb4d1fd853277e63aa098b13f68`；来源指针 /cases/front_cube/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/s05/independent-signed-three-axis-peaks.cube`（pathBase=project-root），SHA256 `04a77cbbe05ff93df31612dfc25d5a38c8f69bb4d1fd853277e63aa098b13f68`；来源指针 /records/66/path，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-210600-s04-isosurface-properties/inverse_cube-cwd/visibility-volume.cube`（pathBase=project-root），SHA256 `04a77cbbe05ff93df31612dfc25d5a38c8f69bb4d1fd853277e63aa098b13f68`；来源指针 /cases/inverse_cube/workingInput，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/volume-render/input-manifest.json`, `tests/delivery/volume-render/actual.json`, `tests/delivery/volume-render/independent-comparison.json`, `tests/delivery/volume-render/resource-lifecycle.json`。当前未执行。

### G4 领域 structure-render
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/24` 和其中 VF-0037, VF-0038, VF-0039, VF-0040, VF-0041, VF-0043, VF-0044, VF-0055, VF-0056, VF-0057, VF-0117, VF-0118, VF-0119, VF-0120, VF-0121, VF-0122, VF-0123, VF-0124, VF-0125, VF-0126, VF-0127, VF-0128, VF-0129, VF-0130, VF-0131, VF-0132, VF-0133, VF-0134, VF-0135, VF-0136, VF-0143, VF-0144, VF-0152, VF-0153, VF-0154, VF-0155, VF-0156, VF-0157, VF-0158, VF-0169, VF-0171 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `web/src/render/materials.ts`；测试 `tests/browser/structure_style.spec.ts`。消费渲染 packet，不在 TS 重算科学状态。
fixture：VS-0054 `reviews/20261108-230337-s06-octahedron/octahedron-working.cif` SHA256 `b08a9e646594d7c8419c559071fd9dbe1080934a5a079c3daff119f1d459393b`；VS-0057 `s05/independent-nonorthogonal-adp.cif` SHA256 `dc52d2f42554ceb909a6d9ea129a3bfa54efa1ca708d4bb1c605b34f3d892840`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：所选半径/材质字段及Properties Cancel残留逐字段；椭球和像素与原子字段编码分判。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`npm run test:browser -- --grep structure_style`。预算 B-WEB；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-02)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：选择解析球/已知Cartesian协方差，冻结半径、特征轴与概率系数，再核mesh/像素；字段字面通过不升级科学通过。

#### 历史：R3接口与交接 structure-render

当前设计接口：`export function buildAtomPacket(images:ImagePosition1[],styles:AtomStyle1[],identity:PageIdentity1):AtomPacket1`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Renderer/page adapter consumes Worker-committed packets and same PageIdentity1; UI changes go through formal page commands, no TS science reimplementation and no direct host write.

来源：`output/vesta-study/20261108-113622/reviews/20261109-170000-s04-model-display/display-before-targets.json`（pathBase=project-root），SHA256 `81ff8f567293b4249504b162cfbbf3e12f04de446710f4bda9592c4a7e267b3f`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- display-support，role=supporting-only；输出不作为该函数输出判据；单位saved display flags。expected：`/cases`, `/predictions`；tolerance：无源数值容差；政策exact-discrete。Visibility/style serialization/Cancel synchronization only, not direct AtomPacket1 images/radius/rgba arrays or rendered geometry.

未冻结：AtomPacket1 exact arrays/styles and renderer geometry/material contract。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261109-154800-s04-output-text/model-control-cwd/clear-model-only.vesta`（pathBase=project-root），SHA256 `d4c508851f6fe92c09e2931e31079dff6d9468fe748d950250887dab16bc9f48`；来源指针 /source，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-170000-s04-model-display/visibility-cwd/visibility-source.vesta`（pathBase=project-root），SHA256 `d4c508851f6fe92c09e2931e31079dff6d9468fe748d950250887dab16bc9f48`；来源指针 /cases/visibility/workingInput，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261109-170000-s04-model-display/styles-cwd/styles-source.vesta`（pathBase=project-root），SHA256 `d4c508851f6fe92c09e2931e31079dff6d9468fe748d950250887dab16bc9f48`；来源指针 /cases/styles/workingInput，identityFromOriginalRecord=true。

取消：Check AbortSignal and document/epoch/revision before frame/bytes delivery. Abort only this consumer request; do not undo a committed domain mutation. After host primary commit, query snapshot instead of claiming cancel prevented write. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/structure-render/input-manifest.json`, `tests/delivery/structure-render/actual.json`, `tests/delivery/structure-render/independent-comparison.json`, `tests/delivery/structure-render/resource-lifecycle.json`。当前未执行。

### G3 领域 2d
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/25` 和其中 VF-0076, VF-0244, VF-0245, VF-0246, VF-0247, VF-0248 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/slices.rs`；测试 `kernel/crates/science/tests/slice_projection.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0062 `s05/independent-nonorthogonal-positive-density.cube` SHA256 `b3c5a5bb6c4c72cc1df9b536a19ce6ac36777ef0a60bf8176aa231ba83412f29`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：三线性取样两端/原点；连续积分与兼容非整数投影偏差分别判；bbox导出禁用是已证条件。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test slice_projection`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新倾斜解析场、半/非整数周期投影留出，分别冻结连续积分与兼容离散权重；两套判据分别报告。

#### 历史：R3接口与交接 2d

当前设计接口：`SliceGridJob::new(input:FieldDescriptor2,grid:SliceGrid1)->Result<SliceGridJob,DomainError>`；`fn step(job:&mut SliceGridJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<SliceResult1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-035000-s06-vector-slice/independent-vector-slice.json`（pathBase=project-root），SHA256 `9c1d34ff1bb8ecf35e4b67bb6755354cd04eae06e03c7e9f38be94d5d55c4856`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- slice-interpolation，role=result-invariant；输出grid, values；单位fractional / electron/A3 / A。expected：`/expectedPlaneZFractional`, `/expectedSpanFractionalXY`, `/manualNxNyFromVectors`, `/closedEndpointCandidateExpected`, `/independentInterpolationFormula`；tolerance：`/densityAbsoluteTolerance`, `/geometryToleranceA`；政策source-declared。z0.3125 vector-defined slice; endpoint table is conditional on actual closed-endpoint convention. Source samplingCountScope must hold before comparing, not fit dimensions to values.

未冻结：General slice/projection weights and sampling policy。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261109-031600-s06-fourier-three-axes/three-axis-density.cube`（pathBase=project-root），SHA256 `9e35b3874f3405239a5c39f053e8e63c866ccf0788600549253d3dcdf475dbc2`；来源指针 /sourceArtifact，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/2d/input-manifest.json`, `tests/delivery/2d/actual.json`, `tests/delivery/2d/independent-comparison.json`, `tests/delivery/2d/resource-lifecycle.json`。当前未执行。

### G3 领域 line
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/26` 和其中 VF-0077, VF-0249 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/line.rs`；测试 `kernel/crates/science/tests/line_sample.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0059 `s05/independent-three-axis-fourier.txt` SHA256 `885a50deb6de83f38819484a1a969670d0ff3d3d1770e51b16726ba305b24feb`。原13点剖面实际起终点/预期见R45 independent-line-profile.json；本例不使用VS9代替。
- [ ] 未来先写独立断言：N点含两端t=i/(N-1)，H距离和周期8角值独立预期；原sqrt185/13点输入从原证据取，不与VS9混同。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test line_sample`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新非正交跨多周期分段，冻结8角插值、count和总长；无效N/输出无权限的失败状态另核。

#### 历史：R3接口与交接 line

当前设计接口：`LineSampleJob::new(request:LineRequest1,input:FieldDescriptor2)->Result<LineSampleJob,DomainError>`；`fn step(job:&mut LineSampleJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<LineResult1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-033200-s06-line-profile/independent-line-profile.json`（pathBase=project-root），SHA256 `c0f1055325702066d0870d72bff1820def58c41a5240c5005f7baa6b3aff8591`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- thirteen-point-line，role=direct-result；输出points.distanceA, points.value；单位A / source field unit。expected：`/expectedRows`, `/totalLengthA`, `/pointCount`, `/includeBothEndpointsExpected`；tolerance：`/distanceAbsToleranceA`, `/densityAbsTolerance`；政策source-declared。Compare all13 source rows including both endpoints, source periodic trilinear rule and length sqrt185A; no universal interpolation/resource scope extrapolation.

未冻结：General line size/nonfinite/output failure domains。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261109-031600-s06-fourier-three-axes/three-axis-density.cube`（pathBase=project-root），SHA256 `9e35b3874f3405239a5c39f053e8e63c866ccf0788600549253d3dcdf475dbc2`；来源指针 /sourceArtifact，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/line/input-manifest.json`, `tests/delivery/line/actual.json`, `tests/delivery/line/independent-comparison.json`, `tests/delivery/line/resource-lifecycle.json`。当前未执行。

### G3 领域 peak-voronoi
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/27` 和其中 VF-0078, VF-0079, VF-0250 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/peaks.rs`；测试 `kernel/crates/science/tests/peak_basins.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0062 `s05/independent-nonorthogonal-positive-density.cube` SHA256 `b3c5a5bb6c4c72cc1df9b536a19ce6ac36777ef0a60bf8176aa231ba83412f29`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：正/负峰、max0/all/max1与26邻点积分；新台地/并列/非正交体积界；峰数不等于原子数。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test peak_basins`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新非对称多峰和台地，用独立几何/体积上界冻结积分，区分峰计数与化学原子计数。

#### 历史：R3接口与交接 peak-voronoi

当前设计接口：`StrictExtremaJob::new(input:FieldDescriptor2,negative:bool,limit:u32)->Result<StrictExtremaJob,DomainError>`；`fn step(job:&mut StrictExtremaJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Vec<Peak1>>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-063000-s06-voronoi-peaks/independent-voronoi-peaks.json`（pathBase=project-root），SHA256 `9dafa8c0f4a5f17546fdb3bd4f3cc71aada9cf66f906a42bb95da90b33d34cab`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- strict-peak，role=direct-result；输出fractional, value；单位fractional / scalar。expected：`/peakOracle/uniquePositiveMaximum`, `/peakOracle/uniqueNegativeMinimum`, `/peakOracle/positivePeakCountExpected`, `/peakOracle/negativePeakCountExpectedIfNegativeSearchEnabled`；tolerance：`/peakOracle/valueAbsoluteTolerance`, `/peakOracle/coordinateAbsoluteTolerance`；政策source-declared。Positive(0.25,0.5,0.75)=6; negative(0.75,0,0.25)=-6 only when negative search enabled. Voronoi integrals are different operations and not used as Peak1 expectations.

未冻结：Plateau/basin integration and general Voronoi policies。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/independent-two-site-constant.cube`（pathBase=project-root），SHA256 `c60027eaa4327015a6e35172c91931bea84bad0a25c98bb7556d67f3402b6a96`；来源指针 /samples/VS-0066/path，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/s05/independent-signed-three-axis-peaks.cube`（pathBase=project-root），SHA256 `04a77cbbe05ff93df31612dfc25d5a38c8f69bb4d1fd853277e63aa098b13f68`；来源指针 /samples/VS-0067/path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/peak-voronoi/input-manifest.json`, `tests/delivery/peak-voronoi/actual.json`, `tests/delivery/peak-voronoi/independent-comparison.json`, `tests/delivery/peak-voronoi/resource-lifecycle.json`。当前未执行。

### G3 领域 poly-integral
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/28` 和其中 VF-0080 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/capabilities.rs`；测试 `kernel/crates/science/tests/poly_integral_capability.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：无新增稳定样本；复用该组原 handler/原生证据，只验证已证无动作，不伪造科学输入。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：VF-0080选定arm64 handler为ret，无新增数值积分；入口保留明确无动作能力，不能自行发明算法。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test poly_integral_capability`。预算 B-MATH；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：此无动作能力可独立定义；不扩成其它版本支持。

#### 历史：R3接口与交接 poly-integral

当前设计接口：`PolyIntegralCapabilityJob::new(reference_version:String)->Result<PolyIntegralCapabilityJob,DomainError>`；`fn step(job:&mut PolyIntegralCapabilityJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<Capability1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-070500-s05-polyhedron-integration-stub/poly-stub-before-gui.json`（pathBase=project-root），SHA256 `25a5c0c845f4e286c8556e01ca338da94fed3291590389b38a6c4496e9675ccb`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- return-handler，role=capability；输出NoAction；单位no mutation/call。expected：`/expectedClickResult`；tolerance：无源数值容差；政策exact-discrete。Original one-instruction return handler yields no calculation or state mutation. No invented integral value or other-version capability.

未冻结：仅此版本无动作能力，不能扩大到其它版本。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/independent-two-site-constant.cube`（pathBase=project-root），SHA256 `c60027eaa4327015a6e35172c91931bea84bad0a25c98bb7556d67f3402b6a96`；来源指针 /source，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/poly-integral/input-manifest.json`, `tests/delivery/poly-integral/actual.json`, `tests/delivery/poly-integral/independent-comparison.json`, `tests/delivery/poly-integral/resource-lifecycle.json`。当前未执行。

### G3 领域 density-conversion
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/29` 和其中 VF-0081, VF-0251, VF-0252 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `kernel/crates/science/src/density_conversion.rs`；测试 `kernel/crates/science/tests/density_conversion.rs`。消费统一 ProjectState／资源表，输出候选领域结果，由 Engine 在相同 epoch/revision 下提交。
fixture：VS-0062 `s05/independent-nonorthogonal-positive-density.cube` SHA256 `b3c5a5bb6c4c72cc1df9b536a19ce6ac36777ef0a60bf8176aa231ba83412f29`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：lnAbs/laplacian/g/v/he各单位与26邻点兼容；零负阈值/连续导数失败分列，保留lned原崩溃。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test density_conversion`。预算 B-GRID；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-SCI-03)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：新解析正密度做网格加密收敛，连续物理与26邻点兼容分列；保留zero/negative阈值适用域，不将lnabs宣称物理电子密度。

#### 历史：R3接口与交接 density-conversion

当前设计接口：`DensityConversionJob::new(request:ConversionRequest1,input:FieldDescriptor2)->Result<DensityConversionJob,DomainError>`；`fn step(job:&mut DensityConversionJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<FieldResult1>,DomainError>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Engine::advance drives bounded steps; only Engine validates same document/epoch/revision and commits candidate. Renderer/UI/codec consume typed result as stated; no side effect inside step.

来源：`output/vesta-study/20261108-113622/reviews/20261109-053500-s06-3d-threshold-conversion/independent-3d-threshold-holdout.json`（pathBase=project-root），SHA256 `840133d2f6c9703307cbc8a74438e15d91c1118201ec57c7ffd991771a2d42cd`；性质original-frozen-input-or-prior。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- observed-discrete-kernel，role=result-invariant；输出field values；单位per-case Angstrom/atomic and converted scalar units。expected：`/cases/nonortho-angstrom/compatibilityBeforeGUI`, `/cases/nonortho-atomic/compatibilityBeforeGUI`, `/cases/threshold-angstrom/compatibilityBeforeGUI`；tolerance：`/scalarTolerance`；政策source-declared。Each fixture matches its own26-neighbor raw-threshold compatibility table. independentContinuousPhysics is diagnostic only; do not replace observed discrete expectation with continuum truth.

未冻结：General physical/scientific domain and full threshold/nonfinite policy。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/s05/independent-nonorthogonal-positive-density.cube`（pathBase=project-root），SHA256 `b3c5a5bb6c4c72cc1df9b536a19ce6ac36777ef0a60bf8176aa231ba83412f29`；来源指针 /samples/VS-0062/path，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/s05/independent-density-thresholds.cube`（pathBase=project-root），SHA256 `76d8dba063be43ec96469566d7311a87556669f011a2f955f9d84314d4395e2b`；来源指针 /samples/VS-0063/path，identityFromOriginalRecord=true。

取消：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/density-conversion/input-manifest.json`, `tests/delivery/density-conversion/actual.json`, `tests/delivery/density-conversion/independent-comparison.json`, `tests/delivery/density-conversion/resource-lifecycle.json`。当前未执行。

### G4 领域 exports
读取 `docs/VESTA研究/S06_DOMAIN_RULES.json#/groups/30` 和其中 VF-0014, VF-0015, VF-0016, VF-0253, VF-0254 的 studyHandoff／specRefs／nextProbe；本节不改变原 fullScientificAcceptance。
计划修改／新增 `web/src/render/exports.ts`；测试 `tests/browser/export_content.spec.ts`。消费渲染 packet，不在 TS 重算科学状态。
fixture：VS-0079 `reviews/20261110-085833-s05-formats-audit/axis-sentinel.pgrid` SHA256 `f3faefbb448679e6ec7843d0f742e1517682bcfeeae4be23aec391bed359dde0`；VS-0102 `reviews/20261110-135709-s06-domain-study/resources/medium/input.cube` SHA256 `1e942ad6048c02ff23fde5b82d58646c1995a68ed65cb39a051dc9fd6d576140`。上述样本只是具名控制；各具体 oracle 的实际输入从该组原始 locator 取，不能将控制样本改称未登记的另一个样本。
- [ ] 未来先写独立断言：23数据输出与14图像入口分别验实际内容；PDB/CUBE/STL/m3d/PRI和JP2/SGI/TIFF失败保持。数值公差按原 S06_ORACLES 的对应子例，合成纯数学采用单独1e-12或精确整数，不能观察输出后放宽。
- [ ] 未来验证命令：`npm run test:browser -- --grep export_content`。预算 B-WEB；失败日志、原始输入、模型状态／产物分别留证。
- [ ] 进入条件：blocked(C-CODEC-01)。精确公共输入／结果及适用域未全部冻结，不允许编造参数开工。最小设计补证：后续按FORMAT矩阵具体未知补生产者与外部decoder，再用同输入精确领域值；科学域的导出使用其各自oracle。

#### 历史：R3接口与交接 exports

当前设计接口：`export async function encodeRegisteredProfile(page:VestaPageV1,request:FormatRequest1,lossConsent:LossConsent1|null,signal:AbortSignal):Promise<EncodedBundle1>`。仅已证子域/内部算子，fullUnitReady=false。相邻消费：Renderer/page adapter consumes Worker-committed packets and same PageIdentity1; UI changes go through formal page commands, no TS science reimplementation and no direct host write.

来源：`output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/independent-pri-analysis.json`（pathBase=project-root），SHA256 `3046675f05f7cc7fdc9c46b9583c11138fa2b71c2ee60f51e31a6744468f8f77`；性质post-observation-independent-recompute。以下是显式选择的输出比较绑定，不从关键词自动挑oracle。

- pri-decoded-invariants，role=result-invariant；输出decoded PRI structure；单位PRI flag/count/grid/scalar。expected：`/0/flagObserved`, `/0/asymmetricRecords`, `/0/cell`, `/0/dimensions`, `/0/NSYM`, `/0/NCEN`, `/0/NSUB`, `/0/symmetry`, `/0/centering`, `/0/maxErrorVsRawSource`, `/1/flagObserved`, `/1/asymmetricRecords`, `/1/cell`, `/1/dimensions`, `/1/NSYM`, `/1/NCEN`, `/1/NSUB`, `/1/symmetry`, `/1/centering`, `/1/maxErrorVsRawSource`, `/2/flagObserved`, `/2/asymmetricRecords`, `/2/cell`, `/2/dimensions`, `/2/NSYM`, `/2/NCEN`, `/2/NSUB`, `/2/symmetry`, `/2/centering`, `/2/maxErrorVsRawSource`；tolerance：无源数值容差；政策exact-discrete。Only these observed flag0 PRI artifacts, maxErrorVsRawSource=0. Actual primary/dependency ResourceIds and loss report hashes are implementation outputs, not reusable oracle values. Post-observation analysis, not a new preregistered writer proof.

未冻结：EncodedBundle byte/loss/resource expectation and full format/dialect contracts。supporting-only或未定容差不能生成通过记录。

输入：`output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/axis/export-pri/actual.pri`（pathBase=project-root），SHA256 `9b73599d25aa764683f9459977f3cf368031254b6fb564aa31fd4867f10c63f2`；来源指针 /0/path，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/exports/volume-pri/actual.pri`（pathBase=project-root），SHA256 `a1c1ac7f5445f0ec5d1bf43f109bea5d0c1bc947f30c1e2fd262d068d63271aa`；来源指针 /1/path，identityFromOriginalRecord=true。

输入：`output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/gui/batch-f/gui-pri.pri`（pathBase=project-root），SHA256 `eed0764bf6e9276833e6f867df71e0be0dbcb5912cfa893df3c03e76907d0fc5`；来源指针 /2/path，identityFromOriginalRecord=true。

取消：Check AbortSignal and document/epoch/revision before frame/bytes delivery. Abort only this consumer request; do not undo a committed domain mutation. After host primary commit, query snapshot instead of claiming cancel prevented write. 清理：On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow.

失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 未来交付：`tests/delivery/exports/input-manifest.json`, `tests/delivery/exports/actual.json`, `tests/delivery/exports/independent-comparison.json`, `tests/delivery/exports/resource-lifecycle.json`。当前未执行。

### G4.1 逐格式分派设计与待冻结字段

当前逐项分派以 [R4的format-profile-registry.json](../../../output/vesta-study/20261108-113622/reviews/20261114-124229-p1-design-r4/contracts/format-profile-registry.json) 为设计合同：48输入、23数据输出、14图像入口各一项，字段包含formatRef/profileId/direction/purposes、精确handler、来源行/入口与SHA、字段政策和冻结门。每个结构/场格式已分到具名文件，不再用structure_text.rs吞所有格式；Raster EPS与Vector EPS保持独立handler。

其中只有I4单P1结构/CIF四个设计profile有闭合参数类型，其余81项明确unfrozen-parameters且不进入可接收的请求联合。字段政策的unfrozen-field-audit不是decoded/preserved已证声明；必须在原S05字段证据和对应门完成后改成有依据的三态。这个分派合同解决F14的逐项归属，**不声称全codec字段/选项已冻结**。

未来测试仍是 `kernel/crates/codecs/tests/all_format_dispatch.rs`、`tests/codec/independent_fields.py` 和各具名handler自己的格式测试，输入/实际字节/独立解析必须来自同一FormatId源行；模板profile、未知ID、跨方向、未冻结参数都要失败。数据输出与图像/2D/文本的真实内容验证分开；不能只凭文件非空或自读回验收。

### G6 应用 偏好与交互的 Web 宿主设计
计划文件 `web/src/editor/{lifecycle,preferences,database,interaction,panels}.tsx`、`web/src/render/environment.ts`、`web/src/host/capabilities.ts`；测试 `tests/browser/application-lifecycle.spec.ts`、`preferences.spec.ts`、`database.spec.ts`、`interaction-focus.spec.ts`、`render-environment.spec.ts`。当前 G6 的48个唯一主责 ID原清单保持，以下是它们的具体职责。
| 入口组 | 计划接口与语义 | 未来断言与前置 |
|---|---|---|
| About／Manual／Updates，VF0001/0082/0083 | `requestHostAction({kind:about/openManual/checkUpdate}):Promise<HostActionReply>`；只读版本来自实际build-manifest；更新检查不能自动下载安装 | 断网/无host能力返回unsupported或明确错误；不伪造原件联网成功；手册版本/来源可追溯 |
| Hide／Show／Quit／New Window，VF0003–0006/0008 | HostAction kind=hideSelf/hideOthers/showAll/newWindow/quit，reply=done/unsupported/failed；页面无权限操作其它OS app时保持unsupported | Quit/close须先save确认，失败保留页面和dirty；新窗口独立document/epoch，不能跨页共用写lease；宿主映射未实测仍是设计差异 |
| New/Open/Recent/Save/Close/Undo，VF0007/0009/0010/0012/0013/0018–0020 | 消费Task2–7正式命令及I4 grant/ack，Recent只保存获准定位／偏好，不存工程字节 | 切视图不unmount/换owner；真正关页才交接；输入框原生Undo和工程Undo按焦点分开，IME合成不提交 |
| Preferences，VF0002/0165–0168/0170/0173 | `Preferences1{openTarget:currentTab/newTab,animationStepDeg,animationIntervalMs,historyLimit,textFontRef,antialiasMode}`；外算路径只为宿主已授权artifact ID，不让Web录入任意shell路径 | 浏览器只持久偏好，绝无作品恢复副本；字体资产先有资格；history裁剪报告资源回收；具体原件范围/默认仍沿source/GUI_LIFECYCLE，不自创数字 |
| 数据库/Stop，VF0011 | database.ts只通过现有网络边界，结果先为候选导入；查询参数/服务版本未全部冻结，不注册自由JSON接口 | 查原unk_3ee7a6c51afa307a的网络/SSL/Stop/超时输入；失败不清当前稿，不绕CSP/平台外网授权；没有新联网探针 |
| 文本和面板，VF0017/0036/0218–0220/0222/0223 | Output只读结果、Comment持久工程正文、Summary当前修订派生值；panel可见性是UI状态；清Output不删Comment | 真UTF8/中文、空输出与注释区别；新load后不留旧Summary；alpha浮窗/焦点按原条件，不把Slider字段当材质像素 |
| 工具/动画/选择，VF0204–0207/0212–0217/0225 | InteractionState1={tool:rotate/select/translate/magnify,rotationMode:drag/free/x/y/z,selectedImages:ImageKey[],animation:stopped/running}；不持久资源句柄 | 列表/Canvas/文本焦点各自验证，失效epoch丢旧选中；动画暂停/失焦不改工程原子；隐藏与删除site分开 |
| 外观/渲染环境，VF0029/0035/0226–0229 | environment.ts消费已冻结scene/material packet；background RGB、lighting、projection/physicalWidth、depthCue各自有tag和单位，原件SaveDefault/Cancel有选择性保留 | Preview/Apply/Cancel逐字段参考原行为；GPU context lost令frame失效，字体未ready不返回Rendered；完整UI范围和像素仍未验证 |
未来命令 `npm run test:browser -- --grep application-lifecycle`、`--grep preferences`、`--grep database`、`--grep interaction-focus`、`--grep render-environment`，B-WEB；数据库真实联网和外算/字体环境须独立具备授权。应用映射不等于OS功能已实现；Web宿主不能承担的语义如实登记差异，最终全功能目标不删除该缺口。

### 本次计划的接受边界
R1 verifier从唯一functions.json与原G1–G6主责清单派生检查254 ID唯一性，并核31个group均有本节路径/fixture/断言/命令/冻结门；这是计划可追溯性，不是全高级参数已固定。C-SCI-01/02/03、C-CODEC-01、C-HOST-01和资产/数据库细节继续妨碍相应完整实现签名与测试预期，P1.4保持未接受。已经不依赖这些门的Task1–8基础合同继续保留，但本卡仍然不允许开始产品施工。

## R2逐单元合同的类型与接受边界

[domain-handoff-contracts.json](../../../output/vesta-study/20261108-113622/reviews/20261114-105518-p1-design-corrections/contracts/domain-handoff-contracts.json)逐项给构造/step签名、完整共同DTO说明、原先验SHA及实际输入SHA、精度定位、取消清理、失败续作和交付物；纯Job不直接commit或写宿主。它只冻结已证子算子的交接提案，不拿内部小算子冒充整组全部功能API。F13已补交接要素，但全域DTO和未证科学/格式oracle仍随F1阻塞，所以F13整体仍为部分整改、P1.4保持partial。新增plane输入仅独立数学，没有原件执行；madel输入仅原缺库失败，不冒称存在能量oracle。


R3来源粒度：14图像route的sourceLocator给IMAGE_EXPORT当前SHA、表格行号/列/菜单token索引与行SHA；sourcePointerKind=markdown-table-cell，不伪装成JSON pointer。48/23 JSON来源仍按原矩阵逐项定位。所有oracle/input/sourceRecord路径统一以项目根为基准，见R3/contracts/domain-handoff-contracts.json的pathConvention。


R4口径补正：oracle顶层expectedPointers仅为非supporting-only的顺序去重并集，支持证据另列；文字精度/理由不是机器数值容差。sourceRecordBase只定位JSON文件，所指原路径值按sourcePointerValueBase另解析；planes及源值表统一本轮同字节副本。图像行SHA与split基数以I8§14和R4 registry的sourceLocatorConvention为准。完整31单元fullUnitReady仍false。


## 历史：R5主体设计的逐单元续办（当前见R6段）

I9§15及[R5 domain-handoff-contracts.json](../../../output/vesta-study/20261108-113622/reviews/20261114-131653-p1-main-design-r5/contracts/domain-handoff-contracts.json)是当前新增有界操作合同。下表和逐项说明继承原G1–G6唯一主责254个ID及原各段的计划文件、命令、预算；不另建功能覆盖目录。start只生成CandidateJob，Engine依epoch/revision提交。所有fullUnitReady仍false；真实fixture adapter和输出投影尚未实现，不将源文件存在当内容往返验收。

### 历史：R5公开边界 phase

`fn start(input: ResolvePlacementsInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ResolvePlacementsOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ResolvePlacementsInput`、`contracts/scientific-operations.contract.json#/$defs/ResolvePlacementsOutput`（相对R5包）。World transform is parent composition, translations in Cartesian A; graph must exist and be acyclic. Product convention is explicit; native refer/R/t mapping remains B01.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：supporting only, no R/t result。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/phase/input-manifest.json`, `tests/delivery/phase/actual.json`, `tests/delivery/phase/independent-comparison.json`, `tests/delivery/phase/resource-lifecycle.json`。完整资格阻塞：B01；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 cell-symmetry

`fn start(input: ExpandImagesInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ExpandImagesOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ExpandImagesInput`、`contracts/scientific-operations.contract.json#/$defs/ExpandImagesOutput`（相对R5包）。r=H(Mf+t+n); explicit supplied 3D operations only. No group-number lookup, Wyckoff tolerance or superspace inference.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：counts invariant only, explicit op/image adapter needed。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/cell-symmetry/input-manifest.json`, `tests/delivery/cell-symmetry/actual.json`, `tests/delivery/cell-symmetry/independent-comparison.json`, `tests/delivery/cell-symmetry/resource-lifecycle.json`。完整资格阻塞：B02；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 transform

`fn start(input: ChangeBasisInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ChangeBasisOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ChangeBasisInput`、`contracts/scientific-operations.contract.json#/$defs/ChangeBasisOutput`（相对R5包）。Hnew=H P; fnew=P^-1(f-p+n). Reject singular P; explicit selected n; no guessing group/setting. Tensor transport mathematical, native covariance qualification B03.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：cell/sites subset after enumerated-supercell adapter; no full group remap。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/transform/input-manifest.json`, `tests/delivery/transform/actual.json`, `tests/delivery/transform/independent-comparison.json`, `tests/delivery/transform/resource-lifecycle.json`。完整资格阻塞：B03；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 sites-adp

`fn start(input: TransportAdpInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<TransportAdpOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/TransportAdpInput`、`contracts/scientific-operations.contract.json#/$defs/TransportAdpOutput`（相对R5包）。Six order 11,22,33,12,13,23; Ucart=H D* Ucif D* H^T; beta converts via H beta H^T/(2pi²); Ueq=trace/3. Preserve nonpositive tensor; return classification, do not invent ellipsoid geometry.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：supporting HKL record; use tensor controls separately, not direct full result。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/sites-adp/input-manifest.json`, `tests/delivery/sites-adp/actual.json`, `tests/delivery/sites-adp/independent-comparison.json`, `tests/delivery/sites-adp/resource-lifecycle.json`。完整资格阻塞：B03；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 duplicate

`fn start(input: FindDuplicatesInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<FindDuplicatesOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/FindDuplicatesInput`、`contracts/scientific-operations.contract.json#/$defs/FindDuplicatesOutput`（相对R5包）。Returns connected components of explicit image positions, distance<=threshold; ordered by first input. Group merge/averaging policy not inferred; B04.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：cluster count is not original merged coordinates; compare count only。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/duplicate/input-manifest.json`, `tests/delivery/duplicate/actual.json`, `tests/delivery/duplicate/independent-comparison.json`, `tests/delivery/duplicate/resource-lifecycle.json`。完整资格阻塞：B04；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 bonds

`fn start(input: SelectBondPairsInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<SelectBondPairsOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/SelectBondPairsInput`、`contracts/scientific-operations.contract.json#/$defs/SelectBondPairsOutput`（相对R5包）。Distinct explicit images, unordered index i<j; inclusive squared thresholds. Do not replace selected image with nearest one or imply recursive boundary/bond-valence support.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：explicit selected pair distances; no recursive group acceptance。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/bonds/input-manifest.json`, `tests/delivery/bonds/actual.json`, `tests/delivery/bonds/independent-comparison.json`, `tests/delivery/bonds/resource-lifecycle.json`。完整资格阻塞：B04；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 polyhedron

`fn start(input: MeasurePolyhedronInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<MeasurePolyhedronOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/MeasurePolyhedronInput`、`contracts/scientific-operations.contract.json#/$defs/MeasurePolyhedronOutput`（相对R5包）。Closed consistently oriented triangular surface required; V=abs(sum(v0 dot(v1 cross v2))/6), area=sum triangle areas. Require mesh validity; do not infer general VESTA neighbor selection/angle/BV policies.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：volume/area projection from Metric list; input oriented octahedral mesh must be independent。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/polyhedron/input-manifest.json`, `tests/delivery/polyhedron/actual.json`, `tests/delivery/polyhedron/independent-comparison.json`, `tests/delivery/polyhedron/resource-lifecycle.json`。完整资格阻塞：全功能来源和适配尚未确立；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 vectors

`fn start(input: TransformVectorInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<TransformVectorOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/TransformVectorInput`、`contracts/scientific-operations.contract.json#/$defs/TransformVectorOutput`（相对R5包）。Polar Mv; axial timeReversal*det(M)*Mv. M orthogonal only; fractional magnetic basis conversion requires B02.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：direct Cartesian moment for supplied orthogonal op only。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/vectors/input-manifest.json`, `tests/delivery/vectors/actual.json`, `tests/delivery/vectors/independent-comparison.json`, `tests/delivery/vectors/resource-lifecycle.json`。完整资格阻塞：B02；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 planes

`fn start(input: MillerPlaneInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<MillerPlaneOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/MillerPlaneInput`、`contracts/scientific-operations.contract.json#/$defs/MillerPlaneOutput`（相对R5包）。g=H^-T h, n=g/|g|, d=1/|g|, offset=order*d; zero h rejected. Fit/SVD and native sign convention B05.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：angle support does not validate Plane output。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/planes/input-manifest.json`, `tests/delivery/planes/actual.json`, `tests/delivery/planes/independent-comparison.json`, `tests/delivery/planes/resource-lifecycle.json`。完整资格阻塞：B05；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 boundary

`fn start(input: ClipExplicitImagesInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ClipExplicitImagesOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ClipExplicitImagesInput`、`contracts/scientific-operations.contract.json#/$defs/ClipExplicitImagesOutput`（相对R5包）。Use H^-1 r, inclusive per-axis boundaries, no generation of extra images. Full legacy duplicate/special-position behavior B04.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：nearest image shift invariant only, no complete boundary count proof。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/boundary/input-manifest.json`, `tests/delivery/boundary/actual.json`, `tests/delivery/boundary/independent-comparison.json`, `tests/delivery/boundary/resource-lifecycle.json`。完整资格阻塞：B04；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 view

`fn start(input: ComposeCameraInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ComposeCameraOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ComposeCameraInput`、`contracts/scientific-operations.contract.json#/$defs/ComposeCameraOutput`（相对R5包）。Proposed composition Rz Ry Rx Rcurrent, degrees and right-handed axes. CLI selected x/y controls do not prove all legacy camera conventions B06.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：zoom invariant does not validate new rotation composition。缺失数值精度不得临时设通用epsilon。

取消/清理：Check AbortSignal and document/epoch/revision before frame/bytes delivery. Abort only this consumer request; do not undo a committed domain mutation. After host primary commit, query snapshot instead of claiming cancel prevented write. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/view/input-manifest.json`, `tests/delivery/view/actual.json`, `tests/delivery/view/independent-comparison.json`, `tests/delivery/view/resource-lifecycle.json`。完整资格阻塞：B06；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 measure

`fn start(input: MeasureImagesInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<MeasureImagesOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/MeasureImagesInput`、`contracts/scientific-operations.contract.json#/$defs/MeasureImagesOutput`（相对R5包）。2/3/4 points respectively; nonzero lengths/normals required; unsigned dihedral 0..180; sigma J C J^T only when supplied covariance parameters exactly identify points. Native ORFFE/general covariance B03.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：原原生oracle只直接支持distance，原生fractional/cell σ仍辅助B03；当前有界Cartesian variance/σ由/cases/1直接设计控制检验。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/measure/input-manifest.json`, `tests/delivery/measure/actual.json`, `tests/delivery/measure/independent-comparison.json`, `tests/delivery/measure/resource-lifecycle.json`。完整资格阻塞：B03；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 tidy

`fn start(input: ProposeTidyResultInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ProposeTidyResultOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ProposeTidyResultInput`、`contracts/scientific-operations.contract.json#/$defs/ProposeTidyResultOutput`（相对R5包）。Validate candidate first, never auto-replace active structure. This is typed external-result admission, not a general TIDY/Niggli implementation; B14.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：candidate-result admission can check invariants; does not implement external generator。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. External process phase, when separately authorized, uses owned process group TERM/grace/KILL and actual exit/output query; never replay on unknown completion. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/tidy/input-manifest.json`, `tests/delivery/tidy/actual.json`, `tests/delivery/tidy/independent-comparison.json`, `tests/delivery/tidy/resource-lifecycle.json`。完整资格阻塞：B14；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 madel

`fn start(input: PrepareMadelInputInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<PrepareMadelInputOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/PrepareMadelInputInput`、`contracts/scientific-operations.contract.json#/$defs/PrepareMadelInputOutput`（相对R5包）。Input file generation plan only; charge-site bijection and explicit radius. Exact MADEL grammar/parameter qualification B14; no runtime or numeric-result claim.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：failure observation has no successful input/result oracle。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. External process phase, when separately authorized, uses owned process group TERM/grace/KILL and actual exit/output query; never replay on unknown completion. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/madel/input-manifest.json`, `tests/delivery/madel/actual.json`, `tests/delivery/madel/independent-comparison.json`, `tests/delivery/madel/resource-lifecycle.json`。完整资格阻塞：B14；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 powder

`fn start(input: ConvolvePowderLinesInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ConvolvePowderLinesOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ConvolvePowderLinesInput`、`contracts/scientific-operations.contract.json#/$defs/ConvolvePowderLinesOutput`（相对R5包）。Explicit pseudo-Voigt unit-area G/L mixture, no background or intensity generator; equal line arrays and monotone sample grid. Compatibility index first point zero only selected evidence; full native width/weights B07.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：reflection positions do not validate convolution samples; existing source needs separate convolution binding。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/powder/input-manifest.json`, `tests/delivery/powder/actual.json`, `tests/delivery/powder/independent-comparison.json`, `tests/delivery/powder/resource-lifecycle.json`。完整资格阻塞：B07；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 sf

`fn start(input: SumStructureFactorsInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<SumStructureFactorsOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/SumStructureFactorsInput`、`contracts/scientific-operations.contract.json#/$defs/SumStructureFactorsOutput`（相对R5包）。Provided per-site per-hkl amplitude already includes declared scattering/ADP factors; sum occ*a*exp(+2pi i h.f). Explicit symmetry-expanded sites; no hidden table lookup or isotope default B02/B07.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：explicit coefficient lists may compare after site-amplitude adapter; no table lookup proof。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/sf/input-manifest.json`, `tests/delivery/sf/actual.json`, `tests/delivery/sf/independent-comparison.json`, `tests/delivery/sf/resource-lifecycle.json`。完整资格阻塞：B02, B07；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 fourier

`fn start(input: SynthesizeFourierInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<SynthesizeFourierOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/SynthesizeFourierInput`、`contracts/scientific-operations.contract.json#/$defs/SynthesizeFourierOutput`（相对R5包）。rho=1/V sum F exp(-2pi i h.f); explicit signed hkl, DC replacement and conjugate policy. Reject aliases including Nyquist duplicate conjugates; modes/normalization beyond supplied coefficients B08.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：integral/formula invariant; generate independent full array from frozen formula。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/fourier/input-manifest.json`, `tests/delivery/fourier/actual.json`, `tests/delivery/fourier/independent-comparison.json`, `tests/delivery/fourier/resource-lifecycle.json`。完整资格阻塞：B08；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 electron

`fn start(input: ElectronDcInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ElectronDcOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ElectronDcInput`、`contracts/scientific-operations.contract.json#/$defs/ElectronDcOutput`（相对R5包）。Constant field count/V, not full atom-centered density or sampling-size algorithm. Full generator B07.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：DC mean/integral only; no full generator。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/electron/input-manifest.json`, `tests/delivery/electron/actual.json`, `tests/delivery/electron/independent-comparison.json`, `tests/delivery/electron/resource-lifecycle.json`。完整资格阻塞：B07；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 nuclear

`fn start(input: NuclearDcInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<NuclearDcOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/NuclearDcInput`、`contracts/scientific-operations.contract.json#/$defs/NuclearDcOutput`（相对R5包）。Constant field signed sum/V fm/A3, negative allowed. Do not claim whole-map shape from DC control B07.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：DC integral only; no full generator。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/nuclear/input-manifest.json`, `tests/delivery/nuclear/actual.json`, `tests/delivery/nuclear/independent-comparison.json`, `tests/delivery/nuclear/resource-lifecycle.json`。完整资格阻塞：B07；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 model-patterson

`fn start(input: ModelPattersonCoefficientsInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ModelPattersonCoefficientsOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ModelPattersonCoefficientsInput`、`contracts/scientific-operations.contract.json#/$defs/ModelPattersonCoefficientsOutput`（相对R5包）。Per explicit hkl modulus squared; output imaginary 0 and squared-unit metadata described in operation units. Full model grid/table policy B07/B08.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：field integral/evenness cannot directly validate coefficient output; needs independent coefficient projection。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/model-patterson/input-manifest.json`, `tests/delivery/model-patterson/actual.json`, `tests/delivery/model-patterson/independent-comparison.json`, `tests/delivery/model-patterson/resource-lifecycle.json`。完整资格阻塞：B07；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 volume-patterson

`fn start(input: VolumePattersonInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<VolumePattersonOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/VolumePattersonInput`、`contracts/scientific-operations.contract.json#/$defs/VolumePattersonOutput`（相对R5包）。Periodic core samples only, F=V/N sum rho exp(+2pi i h.f), inverse |F|²/V, closure regenerated. Selected orthogonal complete-grid oracles exist; generic Nyquist/nonorthogonal B08.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：full-grid formula source usable with exact core/closure adapter。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/volume-patterson/input-manifest.json`, `tests/delivery/volume-patterson/actual.json`, `tests/delivery/volume-patterson/independent-comparison.json`, `tests/delivery/volume-patterson/resource-lifecycle.json`。完整资格阻塞：B08；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 volume-data

`fn start(input: NormalizeGridLayoutInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<NormalizeGridLayoutOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/NormalizeGridLayoutInput`、`contracts/scientific-operations.contract.json#/$defs/NormalizeGridLayoutOutput`（相对R5包）。Reindex and periodic closure only; no scalar rescale or inferred physical-unit conversion; strip/regenerate closure with periodicity check.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：dimension/order/formula invariant for layout operator only。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/volume-data/input-manifest.json`, `tests/delivery/volume-data/actual.json`, `tests/delivery/volume-data/independent-comparison.json`, `tests/delivery/volume-data/resource-lifecycle.json`。完整资格阻塞：B08；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 shape

`fn start(input: IntersectHalfspacesInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<IntersectHalfspacesOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/IntersectHalfspacesInput`、`contracts/scientific-operations.contract.json#/$defs/IntersectHalfspacesOutput`（相对R5包）。Halfspaces n.r<=d, normalize normals with offsets consistently; reject unbounded/empty/lower-dimensional result before returning mesh. Native UI face/area settings B09.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：当前shape使用R7凸边界投影：按源平面归类triangles并核覆盖/不重叠/朝向/闭合；原显示面积仍为事后辅助，不能充当Mesh坐标精度。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/shape/input-manifest.json`, `tests/delivery/shape/actual.json`, `tests/delivery/shape/independent-comparison.json`, `tests/delivery/shape/resource-lifecycle.json`。完整资格阻塞：B09；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 volume-render

`fn start(input: SelectIsoEdgesInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<SelectIsoEdgesOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/SelectIsoEdgesInput`、`contracts/scientific-operations.contract.json#/$defs/SelectIsoEdgesOutput`（相对R5包）。Returns interpolated edge-crossing positions only, equality policy lower-inclusive upper-exclusive; no triangles/caps/NaN-normal compatibility claim B09.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：supporting UI only, no edge/topology oracle。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/volume-render/input-manifest.json`, `tests/delivery/volume-render/actual.json`, `tests/delivery/volume-render/independent-comparison.json`, `tests/delivery/volume-render/resource-lifecycle.json`。完整资格阻塞：B09；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 structure-render

`fn start(input: BuildAtomInstancesInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<BuildAtomInstancesOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/BuildAtomInstancesInput`、`contracts/scientific-operations.contract.json#/$defs/BuildAtomInstancesOutput`（相对R5包）。Explicit uniform ball instances only; no unlicensed radius lookup, ADP ellipsoid or hidden style defaults. Full style/radii/font B10.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：supporting UI only, no AtomInstance/radius result oracle。缺失数值精度不得临时设通用epsilon。

取消/清理：Check AbortSignal and document/epoch/revision before frame/bytes delivery. Abort only this consumer request; do not undo a committed domain mutation. After host primary commit, query snapshot instead of claiming cancel prevented write. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/structure-render/input-manifest.json`, `tests/delivery/structure-render/actual.json`, `tests/delivery/structure-render/independent-comparison.json`, `tests/delivery/structure-render/resource-lifecycle.json`。完整资格阻塞：B03, B10；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 2d

`fn start(input: SliceTrilinearInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<SliceTrilinearOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/SliceTrilinearInput`、`contracts/scientific-operations.contract.json#/$defs/SliceTrilinearOutput`（相对R5包）。Sample origin+i*u+j*v with explicit step vectors; outside nonperiodic extent rejects; periodic eight-corner trilinear. Projection/contours/log B11.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：slice samples/invariants after vector-to-step adapter。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/2d/input-manifest.json`, `tests/delivery/2d/actual.json`, `tests/delivery/2d/independent-comparison.json`, `tests/delivery/2d/resource-lifecycle.json`。完整资格阻塞：B11；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 line

`fn start(input: SampleLineInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<SampleLineOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/SampleLineInput`、`contracts/scientific-operations.contract.json#/$defs/SampleLineOutput`（相对R5包）。Include both endpoints at i/(N-1); distance from H(end-start), preserve direction, periodic trilinear. Selected oracle full path/value bindings retained.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：paired distance/value direct via expectedRows projection。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/line/input-manifest.json`, `tests/delivery/line/actual.json`, `tests/delivery/line/independent-comparison.json`, `tests/delivery/line/resource-lifecycle.json`。完整资格阻塞：B11；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 peak-voronoi

`fn start(input: FindLocalExtremaInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<FindLocalExtremaOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/FindLocalExtremaInput`、`contracts/scientific-operations.contract.json#/$defs/FindLocalExtremaOutput`（相对R5包）。Periodic core 26 neighbors; strict comparison to all; equal plateau omitted with diagnostic; sort maxima desc/minima asc then x/y/z lexicographic. This explicit new policy is not general legacy basin qualification B11.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：selected extrema direct; strict plateau proposal unqualified。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/peak-voronoi/input-manifest.json`, `tests/delivery/peak-voronoi/actual.json`, `tests/delivery/peak-voronoi/independent-comparison.json`, `tests/delivery/peak-voronoi/resource-lifecycle.json`。完整资格阻塞：B11；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 poly-integral

`fn start(input: QueryPolyIntegralCapabilityInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<QueryPolyIntegralCapabilityOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/QueryPolyIntegralCapabilityInput`、`contracts/scientific-operations.contract.json#/$defs/QueryPolyIntegralCapabilityOutput`（相对R5包）。Exact pinned reference menu185 no-action only. No numerical integration operation, no new architecture/version extrapolation.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：capability exact pinned NoAction, not numerical feature。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/poly-integral/input-manifest.json`, `tests/delivery/poly-integral/actual.json`, `tests/delivery/poly-integral/independent-comparison.json`, `tests/delivery/poly-integral/resource-lifecycle.json`。完整资格阻塞：全功能来源和适配尚未确立；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 density-conversion

`fn start(input: ConvertDensityInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<ConvertDensityOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/ConvertDensityInput`、`contracts/scientific-operations.contract.json#/$defs/ConvertDensityOutput`（相对R5包）。Use locked S06 formulas and raw-rho<=1e-6 energy zero; lnAbs zero produces diagnostic special -Infinity, never ordinary finite Field. Full units/nonorthogonal qualification B08/B11.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：selected full values only with exact stencil/unit adapter。缺失数值精度不得临时设通用epsilon。

取消/清理：Before each work_units chunk and before candidate delivery; cancel frees only this job temporary ResourceId retains. After Engine commit return tooLate; undo is separate. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/density-conversion/input-manifest.json`, `tests/delivery/density-conversion/actual.json`, `tests/delivery/density-conversion/independent-comparison.json`, `tests/delivery/density-conversion/resource-lifecycle.json`。完整资格阻塞：B08, B11；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### 历史：R5公开边界 exports

`fn start(input: PlanEncodeInput, snapshot: &ImmutableSnapshot, resources: &mut dyn ResourcePort, budget: Budget) -> Result<CandidateJob<PlanEncodeOutput>, DomainError>`；输入/结果分别为`contracts/scientific-operations.contract.json#/$defs/PlanEncodeInput`、`contracts/scientific-operations.contract.json#/$defs/PlanEncodeOutput`（相对R5包）。Profile must resolve exactly in 85-route registry; source model/field audited before bytes creation; acknowledged per-field losses required. This is a plan envelope, each writer/reader grammar qualified independently B12/B13.

复用原本领域段的具名计划文件/测试命令/预算，进入前按inputs逐份核SHA，独立fixture adapter仅从这些源解构，不用控制样本替代。旧oracle可比范围：PRI parsed invariants do not validate generic EncodeBundle bytes。缺失数值精度不得临时设通用epsilon。

取消/清理：Check AbortSignal and document/epoch/revision before frame/bytes delivery. Abort only this consumer request; do not undo a committed domain mutation. After host primary commit, query snapshot instead of claiming cancel prevented write. On error/cancel release this job retained inputs and uncommitted outputs; never delete source/other jobs/shared blobs; no host path or process in pure primitive. Renderer consumer also releases its own GPU/frame/font resources and rejects stale Rendered; no borrowed WASM view across grow. 失败续作：Keep original ProjectState/revision; unrelated independent jobs may continue. This unit resumes only after input/rule/budget or source-field issue is corrected; no blind rerun of external effects. 交付物：`tests/delivery/exports/input-manifest.json`, `tests/delivery/exports/actual.json`, `tests/delivery/exports/independent-comparison.json`, `tests/delivery/exports/resource-lifecycle.json`。完整资格阻塞：B12, B13；各项最小补证及授权见qualification-blockers.json。本轮仅设计，不执行上述计划命令。

### R5跨模块先后与验收

先由G1固定公共输入/资源/不变量，再由各纯Job消费；G3必须使用显式系数/网格单位，G4按精确FormatId字段审计生成Bundle，G6仅取候选结果和完成事件，G-HOST独占实际文件写入/读回确认。并行设计互不改另一组原始证据，实施前仍需独立复审及另行授权。验收同时要求实际输入适配、对应输出的独立数值判据、取消/失败/清理及完整读写往返；类型检查或本轮文档检查通过不能替代这些结果。


## R6 当前逐单元有界交接（设计，未授权实施）

当前以I10§16及R6 contracts为准；R3/R5内部签名仅历史。31领域可含多个操作，exports明确两阶段；Engine独占修订校验/提交。下面所有文件/命令/测试均为未来计划，不执行。

### R6公开边界 phase

`ResolvePlacementsJob::new(input:ResolvePlacementsInput)->Result<ResolvePlacementsJob,DomainError>`；`fn step(job:&mut ResolvePlacementsJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ResolvePlacementsOutput>,DomainError>`。公开操作：ResolvePlacements。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/phase-placement.ts`：world transform scene instances，消费`ResolvePlacementsOutput`；`kernel/crates/codecs/src/formats/native_vesta.rs`：mapped phase placement after B01 qualification，消费`ResolvePlacementsOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：shape-support=supporting-only。supporting only, no R/t result 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/phase.rs`, `kernel/crates/geometry/tests/phase_reference.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test phase_reference`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B01。

### R6公开边界 cell-symmetry

`ExpandImagesJob::new(input:ExpandImagesInput)->Result<ExpandImagesJob,DomainError>`；`fn step(job:&mut ExpandImagesJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ExpandImagesOutput>,DomainError>`。公开操作：ExpandImages。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/image-instances.ts`：explicit periodic image instances，消费`ExpandImagesOutput`；`web/src/editor/selection.ts`：Image identity/selected image positions，消费`ExpandImagesOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：inversion-count=result-invariant。counts invariant only, explicit op/image adapter needed 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/symmetry.rs`, `kernel/crates/geometry/tests/symmetry_images.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test symmetry_images`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B02。

### R6公开边界 transform

`ChangeBasisJob::new(input:ChangeBasisInput)->Result<ChangeBasisJob,DomainError>`；`fn step(job:&mut ChangeBasisJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ChangeBasisOutput>,DomainError>`。公开操作：ChangeBasis。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/cell-transform.tsx`：candidate structure and old-to-new selection preview，消费`ChangeBasisOutput`；`kernel/crates/codecs/src/formats/native_vesta.rs`：candidate cell/site persistence，消费`ChangeBasisOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：p1-transform=direct-result。cell/sites subset after enumerated-supercell adapter; no full group remap 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/transform.rs`, `kernel/crates/geometry/tests/cell_transform.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test cell_transform`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B03。

### R6公开边界 sites-adp

`TransportAdpJob::new(input:TransportAdpInput)->Result<TransportAdpJob,DomainError>`；`fn step(job:&mut TransportAdpJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<TransportAdpOutput>,DomainError>`。公开操作：TransportAdp。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/adp.tsx`：tensor/Ueq/inertia diagnostic，消费`TransportAdpOutput`；`web/src/render/ellipsoid-admission.ts`：reject unqualified nonpositive rendering, no clamping，消费`TransportAdpOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：scattering-support=supporting-only。supporting HKL record; use tensor controls separately, not direct full result 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/adp.rs`, `kernel/crates/geometry/tests/adp_basis.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test adp_basis`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B03。

### R6公开边界 duplicate

`FindDuplicatesJob::new(input:FindDuplicatesInput)->Result<FindDuplicatesJob,DomainError>`；`fn step(job:&mut FindDuplicatesJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<FindDuplicatesOutput>,DomainError>`。公开操作：FindDuplicates。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/duplicates.tsx`：cluster preview; no implicit merge，消费`FindDuplicatesOutput`；`web/src/output/duplicate-clusters.ts`：typed cluster identity listing，消费`FindDuplicatesOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：cluster-count-support=result-invariant。cluster count is not original merged coordinates; compare count only 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/reduction.rs`, `kernel/crates/geometry/tests/merge_sites.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test merge_sites`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B04。

### R6公开边界 bonds

`SelectBondPairsJob::new(input:SelectBondPairsInput)->Result<SelectBondPairsJob,DomainError>`；`fn step(job:&mut SelectBondPairsJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<SelectBondPairsOutput>,DomainError>`。公开操作：SelectBondPairs。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/bond-instances.ts`：selected image endpoints and distance，消费`SelectBondPairsOutput`；`web/src/output/bond-table.ts`：per-pair distance table，消费`SelectBondPairsOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：periodic-distances=direct-result。explicit selected pair distances; no recursive group acceptance 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/neighbors.rs`, `kernel/crates/geometry/tests/bond_graph.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test bond_graph`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B04。

### R6公开边界 polyhedron

`MeasurePolyhedronJob::new(input:MeasurePolyhedronInput)->Result<MeasurePolyhedronJob,DomainError>`；`fn step(job:&mut MeasurePolyhedronJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<MeasurePolyhedronOutput>,DomainError>`。公开操作：MeasurePolyhedron。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/polyhedron-metrics.tsx`：volume/area statistics，消费`MeasurePolyhedronOutput`；`web/src/output/polyhedron-metrics.ts`：named metric text export，消费`MeasurePolyhedronOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：regular-octahedron=direct-result。volume/area projection from Metric list; input oriented octahedral mesh must be independent 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/polyhedra.rs`, `kernel/crates/geometry/tests/polyhedron_geometry.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test polyhedron_geometry`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B04, B09。

### R6公开边界 vectors

`TransformVectorJob::new(input:TransformVectorInput)->Result<TransformVectorJob,DomainError>`；`fn step(job:&mut TransformVectorJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<TransformVectorOutput>,DomainError>`。公开操作：TransformVector。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/vector-instances.ts`：Cartesian arrow vector，消费`TransformVectorOutput`；`web/src/editor/vectors.tsx`：transformed vector numeric preview，消费`TransformVectorOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：axial-cartesian=direct-result。direct Cartesian moment for supplied orthogonal op only 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/vectors.rs`, `kernel/crates/geometry/tests/vector_parity.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test vector_parity`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B02。

### R6公开边界 planes

`MillerPlaneJob::new(input:MillerPlaneInput)->Result<MillerPlaneJob,DomainError>`；`fn step(job:&mut MillerPlaneJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<MillerPlaneOutput>,DomainError>`。公开操作：MillerPlane。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/plane-instances.ts`：normal/offset/spacing plane，消费`MillerPlaneOutput`；`web/src/output/plane-table.ts`：Miller plane data listing，消费`MillerPlaneOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：plane-angle-support=supporting-only。angle support does not validate Plane output 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/planes.rs`, `kernel/crates/geometry/tests/plane_geometry.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test plane_geometry`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B05。

### R6公开边界 boundary

`ClipExplicitImagesJob::new(input:ClipExplicitImagesInput)->Result<ClipExplicitImagesJob,DomainError>`；`fn step(job:&mut ClipExplicitImagesJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ClipExplicitImagesOutput>,DomainError>`。公开操作：ClipExplicitImages。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/visible-images.ts`：clipped explicit image set，消费`ClipExplicitImagesOutput`；`web/src/editor/selection.ts`：remove clipped-away selected images，消费`ClipExplicitImagesOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：selected-image-geometry=supporting-only。Historical original observations are supporting-only. Independent typed analytic output comparison is in designControlBindings; it does not establish whole native feature qualification. 独立控制：/cases/2/expected。

未来文件：`kernel/crates/geometry/src/boundary.rs`, `kernel/crates/geometry/tests/boundary_images.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test boundary_images`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B04。

### R6公开边界 view

`ComposeCameraJob::new(input:ComposeCameraInput)->Result<ComposeCameraJob,DomainError>`；`fn step(job:&mut ComposeCameraJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ComposeCameraOutput>,DomainError>`。公开操作：ComposeCamera。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/camera.ts`：camera rotation/projection/zoom，消费`ComposeCameraOutput`；`web/src/editor/view.tsx`：camera controls matching candidate，消费`ComposeCameraOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：camera-zoom=result-invariant。zoom invariant does not validate new rotation composition 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`web/src/render/camera.ts`, `tests/browser/view_transform.spec.ts`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`npm run test:browser -- --grep view_transform`；预算B-WEB。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B06。

### R6公开边界 measure

`MeasureImagesJob::new(input:MeasureImagesInput)->Result<MeasureImagesJob,DomainError>`；`fn step(job:&mut MeasureImagesJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<MeasureImagesOutput>,DomainError>`。公开操作：MeasureImages。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/measurement-labels.ts`：value/unit/sigma label，消费`MeasureImagesOutput`；`web/src/output/measurement-table.ts`：variance and sigma with squared/unit labels，消费`MeasureImagesOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：selected-distance=direct-result, sue-support=supporting-only。原原生oracle只直接支持distance，原生fractional/cell σ仍辅助B03；当前有界Cartesian variance/σ由/cases/1直接设计控制检验 独立控制：/cases/1/expected。

未来文件：`kernel/crates/geometry/src/measure.rs`, `kernel/crates/geometry/tests/measurement.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test measurement`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B03。

### R6公开边界 tidy

`ProposeTidyResultJob::new(input:ProposeTidyResultInput)->Result<ProposeTidyResultJob,DomainError>`；`fn step(job:&mut ProposeTidyResultJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ProposeTidyResultOutput>,DomainError>`。公开操作：ProposeTidyResult。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/tidy-preview.tsx`：show independently validated proposed structure，消费`ProposeTidyResultOutput`；`kernel/crates/commands/src/apply_candidate.rs`：explicit revision-checked apply; no automatic replace，消费`ProposeTidyResultOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：niggli-metric=result-invariant。candidate-result admission can check invariants; does not implement external generator 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/reduction.rs`, `kernel/crates/geometry/tests/tidy_result.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test tidy_result`；预算B-EXTERNAL。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B14。

### R6公开边界 madel

`PrepareMadelInputJob::new(input:PrepareMadelInputInput)->Result<PrepareMadelInputJob,DomainError>`；`fn step(job:&mut PrepareMadelInputJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<PrepareMadelInputOutput>,DomainError>`。公开操作：PrepareMadelInput。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/host/external-inputs.ts`：materialize admitted candidate files only，消费`PrepareMadelInputOutput`；`web/src/editor/madel.tsx`：input manifest preview; run remains unauthorized，消费`PrepareMadelInputOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：。failure observation has no successful input/result oracle 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/external/madel.rs`, `kernel/crates/science/tests/madel_external.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test madel_external`；预算B-EXTERNAL。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B14。

### R6公开边界 powder

`ConvolvePowderLinesJob::new(input:ConvolvePowderLinesInput)->Result<ConvolvePowderLinesJob,DomainError>`；`fn step(job:&mut ConvolvePowderLinesJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ConvolvePowderLinesOutput>,DomainError>`。公开操作：ConvolvePowderLines。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/powder-chart.ts`：sampled intensity curve，消费`ConvolvePowderLinesOutput`；`web/src/output/powder-curve.ts`：x/y text export; not Bragg line table，消费`ConvolvePowderLinesOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：bragg-positions=supporting-only。Historical original observations are supporting-only. Independent typed analytic output comparison is in designControlBindings; it does not establish whole native feature qualification. 独立控制：/cases/0/expected。

未来文件：`kernel/crates/science/src/powder.rs`, `kernel/crates/science/tests/powder_pattern.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test powder_pattern`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B07。

### R6公开边界 sf

`SumStructureFactorsJob::new(input:SumStructureFactorsInput)->Result<SumStructureFactorsJob,DomainError>`；`fn step(job:&mut SumStructureFactorsJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<SumStructureFactorsOutput>,DomainError>`。公开操作：SumStructureFactors。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/reflections.tsx`：complex reflection table，消费`SumStructureFactorsOutput`；`web/src/output/structure-factors.ts`：hkl/real/imaginary unit-aware export，消费`SumStructureFactorsOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：six-reflections=direct-result。explicit coefficient lists may compare after site-amplitude adapter; no table lookup proof 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/structure_factors.rs`, `kernel/crates/science/tests/structure_factors.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test structure_factors`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B02, B07。

### R6公开边界 fourier

`SynthesizeFourierJob::new(input:SynthesizeFourierInput)->Result<SynthesizeFourierJob,DomainError>`；`fn step(job:&mut SynthesizeFourierJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<SynthesizeFourierOutput>,DomainError>`。公开操作：SynthesizeFourier。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/field-layers.ts`：attach candidate field layer，消费`SynthesizeFourierOutput`；`kernel/crates/codecs/src/formats/general_grid.rs`：candidate scalar grid export，消费`SynthesizeFourierOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：analytic-field=result-invariant。integral/formula invariant; generate independent full array from frozen formula 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/fourier.rs`, `kernel/crates/science/tests/fourier_field.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test fourier_field`；预算B-GRID。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B08。

### R6公开边界 electron

`ElectronDcJob::new(input:ElectronDcInput)->Result<ElectronDcJob,DomainError>`；`fn step(job:&mut ElectronDcJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ElectronDcOutput>,DomainError>`。公开操作：ElectronDc。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/density-summary.tsx`：DC count/volume mean，消费`ElectronDcOutput`；`web/src/render/field-layer.ts`：constant field display; not full generator，消费`ElectronDcOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：f000-integral=result-invariant。DC mean/integral only; no full generator 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/density.rs`, `kernel/crates/science/tests/electron_density.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test electron_density`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B07。

### R6公开边界 nuclear

`NuclearDcJob::new(input:NuclearDcInput)->Result<NuclearDcJob,DomainError>`；`fn step(job:&mut NuclearDcJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<NuclearDcOutput>,DomainError>`。公开操作：NuclearDc。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/density-summary.tsx`：signed DC sum/mean，消费`NuclearDcOutput`；`web/src/render/field-layer.ts`：signed constant field layer，消费`NuclearDcOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：nuclear-dc=result-invariant。DC integral only; no full generator 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/density.rs`, `kernel/crates/science/tests/nuclear_density.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test nuclear_density`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B07。

### R6公开边界 model-patterson

`ModelPattersonCoefficientsJob::new(input:ModelPattersonCoefficientsInput)->Result<ModelPattersonCoefficientsJob,DomainError>`；`fn step(job:&mut ModelPattersonCoefficientsJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ModelPattersonCoefficientsOutput>,DomainError>`。公开操作：ModelPattersonCoefficients。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/patterson-coefficients.tsx`：squared-unit coefficient table，消费`ModelPattersonCoefficientsOutput`；`web/src/output/patterson-coefficients.ts`：coefficient TSV; never label a field result，消费`ModelPattersonCoefficientsOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：patterson-dc=supporting-only。Historical original observations are supporting-only. Independent typed analytic output comparison is in designControlBindings; it does not establish whole native feature qualification. 独立控制：/cases/3/expected。

未来文件：`kernel/crates/science/src/patterson.rs`, `kernel/crates/science/tests/model_patterson.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test model_patterson`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B07。

### R6公开边界 volume-patterson

`VolumePattersonJob::new(input:VolumePattersonInput)->Result<VolumePattersonJob,DomainError>`；`fn step(job:&mut VolumePattersonJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<VolumePattersonOutput>,DomainError>`。公开操作：VolumePatterson。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/field-layers.ts`：Patterson map layer，消费`VolumePattersonOutput`；`kernel/crates/codecs/src/formats/general_grid.rs`：Patterson unit-aware map export，消费`VolumePattersonOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：discrete-convolution=result-invariant。full-grid formula source usable with exact core/closure adapter 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/patterson.rs`, `kernel/crates/science/tests/volume_patterson.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test volume_patterson`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B08。

### R6公开边界 volume-data

`NormalizeGridLayoutJob::new(input:NormalizeGridLayoutInput)->Result<NormalizeGridLayoutJob,DomainError>`；`fn step(job:&mut NormalizeGridLayoutJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<NormalizeGridLayoutOutput>,DomainError>`。公开操作：NormalizeGridLayout。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/field-resources.ts`：replace layout descriptor with new owned bytes，消费`NormalizeGridLayoutOutput`；`kernel/crates/codecs/src/formats/general_grid.rs`：axis/closure-aware serialization，消费`NormalizeGridLayoutOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：axis-sentinel=result-invariant。dimension/order/formula invariant for layout operator only 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/field.rs`, `kernel/crates/science/tests/field_layout.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test field_layout`；预算B-GRID。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B08。

### R6公开边界 shape

`IntersectHalfspacesJob::new(input:IntersectHalfspacesInput)->Result<IntersectHalfspacesJob,DomainError>`；`fn step(job:&mut IntersectHalfspacesJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<IntersectHalfspacesOutput>,DomainError>`。公开操作：IntersectHalfspaces。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/facet-mesh.ts`：bounded halfspace mesh，消费`IntersectHalfspacesOutput`；`kernel/crates/codecs/src/formats/mesh.rs`：mesh export with manifold/normal checks，消费`IntersectHalfspacesOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：local-halfspace-mesh=direct-result, area-crosscheck=result-invariant。当前shape使用R7凸边界投影：按源平面归类triangles并核覆盖/不重叠/朝向/闭合；原显示面积仍为事后辅助，不能充当Mesh坐标精度 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/geometry/src/shape.rs`, `kernel/crates/geometry/tests/halfspace_shape.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-geometry --test halfspace_shape`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B09。

### R6公开边界 volume-render

`SelectIsoEdgesJob::new(input:SelectIsoEdgesInput)->Result<SelectIsoEdgesJob,DomainError>`；`fn step(job:&mut SelectIsoEdgesJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<SelectIsoEdgesOutput>,DomainError>`。公开操作：SelectIsoEdges。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/iso-edge-debug.ts`：edge crossing diagnostic points only，消费`SelectIsoEdgesOutput`；`web/src/output/iso-crossings.ts`：crossing-coordinate listing; no triangle/cap claim，消费`SelectIsoEdgesOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：serialization-support=supporting-only。supporting UI only, no edge/topology oracle 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/isosurface.rs`, `kernel/crates/science/tests/iso_geometry.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test iso_geometry`；预算B-GRID。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B09。

### R6公开边界 structure-render

`BuildAtomInstancesJob::new(input:BuildAtomInstancesInput)->Result<BuildAtomInstancesJob,DomainError>`；`fn step(job:&mut BuildAtomInstancesJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<BuildAtomInstancesOutput>,DomainError>`。公开操作：BuildAtomInstances。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/atom-instances.ts`：uniform atom sphere instances，消费`BuildAtomInstancesOutput`；`web/src/editor/picking.ts`：Image identity picking from instance index，消费`BuildAtomInstancesOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：display-support=supporting-only。supporting UI only, no AtomInstance/radius result oracle 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`web/src/render/materials.ts`, `tests/browser/structure_style.spec.ts`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`npm run test:browser -- --grep structure_style`；预算B-WEB。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B03, B10。

### R6公开边界 2d

`SliceTrilinearJob::new(input:SliceTrilinearInput)->Result<SliceTrilinearJob,DomainError>`；`fn step(job:&mut SliceTrilinearJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<SliceTrilinearOutput>,DomainError>`。公开操作：SliceTrilinear。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/slice-canvas.ts`：slice value grid，消费`SliceTrilinearOutput`；`web/src/output/slice-grid.ts`：grid geometry and scalar text export，消费`SliceTrilinearOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：slice-interpolation=result-invariant。slice samples/invariants after vector-to-step adapter 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/slices.rs`, `kernel/crates/science/tests/slice_projection.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test slice_projection`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B11。

### R6公开边界 line

`SampleLineJob::new(input:SampleLineInput)->Result<SampleLineJob,DomainError>`；`fn step(job:&mut SampleLineJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<SampleLineOutput>,DomainError>`。公开操作：SampleLine。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/render/line-chart.ts`：distance/value chart，消费`SampleLineOutput`；`web/src/output/line-profile.ts`：paired line profile text，消费`SampleLineOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：thirteen-point-line=direct-result。paired distance/value direct via expectedRows projection 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/line.rs`, `kernel/crates/science/tests/line_sample.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test line_sample`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B11。

### R6公开边界 peak-voronoi

`FindLocalExtremaJob::new(input:FindLocalExtremaInput)->Result<FindLocalExtremaJob,DomainError>`；`fn step(job:&mut FindLocalExtremaJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<FindLocalExtremaOutput>,DomainError>`。公开操作：FindLocalExtrema。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/peak-table.tsx`：selected extrema table，消费`FindLocalExtremaOutput`；`web/src/render/peak-markers.ts`：fractional extrema markers; no basin integration，消费`FindLocalExtremaOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：strict-peak=direct-result。selected extrema direct; strict plateau proposal unqualified 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/peaks.rs`, `kernel/crates/science/tests/peak_basins.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test peak_basins`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B11。

### R6公开边界 poly-integral

`QueryPolyIntegralCapabilityJob::new(input:QueryPolyIntegralCapabilityInput)->Result<QueryPolyIntegralCapabilityJob,DomainError>`；`fn step(job:&mut QueryPolyIntegralCapabilityJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<QueryPolyIntegralCapabilityOutput>,DomainError>`。公开操作：QueryPolyIntegralCapability。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/menu-capabilities.ts`：pinned no-action menu state，消费`QueryPolyIntegralCapabilityOutput`；`web/src/commands/capabilities.ts`：version-scoped capability query; no invented integral，消费`QueryPolyIntegralCapabilityOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：return-handler=capability。capability exact pinned NoAction, not numerical feature 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/capabilities.rs`, `kernel/crates/science/tests/poly_integral_capability.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test poly_integral_capability`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：仅指定版本NoAction能力设计闭合，实施/运行未验证，不新增实验。

### R6公开边界 density-conversion

`ConvertDensityJob::new(input:ConvertDensityInput)->Result<ConvertDensityJob,DomainError>`；`fn step(job:&mut ConvertDensityJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<ConvertDensityOutput>,DomainError>`。公开操作：ConvertDensity。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/field-layers.ts`：derived field unit-tagged layer，消费`ConvertDensityOutput`；`kernel/crates/codecs/src/formats/general_grid.rs`：derived scalar export, not original unit by default，消费`ConvertDensityOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：observed-discrete-kernel=result-invariant。selected full values only with exact stencil/unit adapter 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/science/src/density_conversion.rs`, `kernel/crates/science/tests/density_conversion.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-science --test density_conversion`；预算B-GRID。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B08, B11。

### R6公开边界 exports

`PlanEncodeJob::new(input:PlanEncodeInput)->Result<PlanEncodeJob,DomainError>`；`fn step(job:&mut PlanEncodeJob,cx:&mut PureJobContext,work_units:u32)->Result<JobStep<PlanEncodeOutput>,DomainError>`。公开操作：PlanEncode, EncodeApproved。输入/输出schema见同名Input/Output，旧step在historicalInternalPrimitive。

消费：`web/src/editor/export-loss.tsx`：display exact loss report/consent binding，消费`PlanEncodeOutput`；`web/src/host/bundle-admission.ts`：candidate encoded Bundle then existing host save protocol，消费`EncodeApprovedOutput`。消费者只retain所需候选资源，错误/取消/销毁释放自身retain；提交状态/history另有Engine引用。

旧oracle角色：pri-decoded-invariants=supporting-only。Historical original observations are supporting-only. PlanEncode checks loss-plan metadata; EncodeApproved byte content/roundtrip remain missingExpected, not proven by PRI decode. 独立控制：保留原直接/不变量及显式missingExpected，未补造新运行结果。

未来文件：`kernel/crates/codecs/src/encode_plan.rs`, `kernel/crates/codecs/tests/encode_loss_contract.rs`；cwd=`/Users/agilecho/code/VestaCraft/app`；命令=`cargo +1.99.1 test --manifest-path kernel/Cargo.toml --locked -p vesta-codecs --test encode_loss_contract`；预算B-MATH。进入前按inputs中真实SHA和source-adapter-plans核对，失败不得拿其它控制样本替代。取消/清理/失败续作/交付沿本单元同名字段；当前fullUnitReady=false。阻塞：B12, B13。


## R7当前交接增补（设计，不执行产品步骤）

消费I11§17及[R7 contracts](../../../output/vesta-study/20261108-113622/reviews/20261114-152228-p1-design-r7-errata/contracts/)。R6其余单元计划继承，以下覆盖字段身份、measure和shape；所有文件、命令、资源生命周期测试仍为未来计划，当前没有step/fixture adapter或writer实现。

- 字段模型／codec：model/host对象表生成FieldHandle，EncodeInventory只消费可信同快照表；codecs encode_plan.rs核字段owner/ID，LossReport用objectKind=field与相对Field指针。export-loss.tsx展示两个共享资源的独立对象损失，bundle-admission.ts只接受重新绑定摘要和同意。未来identity fixture须包含换序/共享资源/错误对象与跨epoch；不从value/digest推身份。
- measure：原原生距离oracle直接，原生fractional/cell σ证据仍辅助B03；当前有界Cartesian variance/σ由/cases/1及重排/交叉协方差设计例直接检验。Measurement1旧文字不再限制MeasureImagesOutput。原构造/step/consumer文件/预算保持R6当前合同。
- shape：当前Mesh字段verticesA/triangles投影到源平面/多边形，按覆盖/不重叠/边界分段/朝向/索引闭合比较；可接受不同对角线和Steiner点，不能以原polygon index数组相等验收。未来geometry shape_reference测试引用shape-projection-contract及绑定源多边形，不使用GUI面积误差作坐标阈值。原B-MATH、epoch/取消清理与候选提交边界保持；一般退化/非凸与相映射仍blocked，不以新投影宣布fullUnitReady。

R6-R5保留为proof-boundaries.json中的未证明项；合法样例能投影不证明任意半空间交集构造、数组拒绝或实际产品。P1.2/P1.4/整体partial，P1.3 C4设计接受、C6未审增量不变。
