# VESTA 模块 Implementation Plan

<!-- P1-CURRENT-STATUS BEGIN -->
当前状态：rev66 已由用户带回 dsh 独审通过，F5-r／F3-r及回归闭合；用户已确认 P1.2／P1.4设计级接受，P1四项设计接受为[true,true,true,true,true]（依次为P1.1、P1.2、P1.3、P1.4、整体），P1设计阶段收口。资料基线rev67仅登记；被审技术交付仍为rev66 / D35-I33-r1-C6 / IP6，wire/profile代际I33，九摘要不变。P1.1及P1.3 C4既有接受保持，C5条件／C6未审及03rev48、04rev57/58历史范围不追认。仍停在设计阶段，不进入产品实现或真实运行。

[06接受登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/RECEIPT.md) / [设计接受记录](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/accepted-design.json) / [复审原文](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/inbound/dsh-rev66-review.md) / [过程及统计勘误](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-160449-rev66-acceptance-registration/REVIEW-PROCESS-ERRATA.md) / [已接受的rev66技术封存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/final-seal.json)。
<!-- P1-CURRENT-STATUS END -->

> **For agentic workers:** REQUIRED SUB-SKILL: 使用 `superpowers:executing-plans`，Codex直接逐任务实施；步骤使用复选框。用户已指定原生执行及dsh独立复审，不自动派子Agent、不交dsh施工。本文件本轮只编制，全部产品步骤未执行。

**Goal:** 在全功能1:1目标下固定模块依赖和可执行基础切片，同时把尚未冻结的相邻科学/codec/worker接口明确隔离为P1设计补证，不以部分计划替代全功能实施计划。

**Architecture:** Rust原创领域内核通过单一命令注册表供WASM Worker与原生CLI消费；TS/React/Three.js提供可编辑当前页面，宿主桥复用平台运输/个人空间。默认原生`.vesta`加资源，资源先写、主文件末提交，真实宿主确认对应快照。尚未冻结的高级科学payload和CA迁入接线先补设计证据。

**Tech Stack:** Rust1.99.1、wasm-bindgen0.2.130、serde1.0.229、RustFFT6.4.1；TS7.0.2、React19.3.1、Vite8.3.7、Three.js0.187.0；Playwright1.65.0。当次获取与实际可行性范围见设计§2，尚无产品锁文件/安装/构建。React DOM/types、字体/参数、Linux镜像与transitive锁由Task1/G-ASSET核实，不默认latest。

**Spec:** [架构](../specs/2026-11-10-vesta-p1-design.md)、[接口](../specs/2026-11-10-vesta-interfaces.md)、[CA接线](../specs/2026-11-10-vesta-ca-wiring.md)；执行者须同时读取。[functions.json](../../VESTA研究/functions.json)是唯一功能规格，本文任务引用不是另建覆盖矩阵。

R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

## Global Constraints

- 当前仅授权P1设计，交付后停止；本计划不授权产品编码/部署/模型builder或P2–P5。
- 全功能1:1、Deep、Codex执行和dsh独立复审保持；默认复现已证原件行为并提示科学/数据风险，修正须显式选择（DEC-P1-01）。默认`.vesta`＋资源（DEC-P1-02）。
- 计划产品workspace `/Users/agilecho/code/VestaCraft`，产品root为其`app/`，均尚不存在。资料ROOT现为Draft Git仓库，有并存WIP；旧Craft非Git为历史时点。本轮不初始化产品、不建worktree、不在共享平台施工。
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
| G4 全codec/图像/材质（消费03 rev48设计） | `codecs/src/formats/*.rs`、`render/{materials,labels,exports}.ts` | G-CODEC各原生unknown具体probe；S05 input48/output23与实际option合同逐format；PDB列/ANISOU、m3d正负量化、PRI flag/ABI、DEN/STO header、Elk空串、ASSE列、GAMESS首原子/跨度；独立读取器核字段。Raster失败不由resolved洗白。D08/D09、FORMATS、行为01/05、S05原字节 | Task4已证子集、页面接口/错误恢复 |
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


## R6继承单元合同（R7设计差异见I11继承交接规则，当前接续见R9节，未授权实施）

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

当前shape oracle角色：`local-halfspace-mesh=result-invariant`；`area-crosscheck=supporting-only`。

当前shape使用R7继承的凸边界投影：按源平面归类triangles并核覆盖/不重叠/朝向/闭合。local-halfspace-mesh比较选定凸边界几何不变量；area-crosscheck仅作事后辅助，不充当Mesh坐标、拓扑或生产精度控制。旧直接输出/面积不变量角色仅属已封存R6历史，不再作为当前单元施工判据。

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


## I11继承交接规则（原R7设计，当前文书见R9节）

消费I11§17及[R9 contracts](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163802-p1-design-r9-doc-state/contracts)。R6其余单元计划继承，以下覆盖字段身份、measure和shape；所有文件、命令、资源生命周期测试仍为未来计划，当前没有step/fixture adapter或writer实现。

- 字段模型／codec：model/host对象表生成FieldHandle，EncodeInventory只消费可信同快照表；codecs encode_plan.rs核字段owner/ID，LossReport用objectKind=field与相对Field指针。export-loss.tsx展示两个共享资源的独立对象损失，bundle-admission.ts只接受重新绑定摘要和同意。未来identity fixture须包含换序/共享资源/错误对象与跨epoch；不从value/digest推身份。
- measure：原原生距离oracle直接，原生fractional/cell σ证据仍辅助B03；当前有界Cartesian variance/σ由/cases/1及重排/交叉协方差设计例直接检验。Measurement1旧文字不再限制MeasureImagesOutput。原构造/step/consumer文件/预算保持R6当前合同。
- shape：当前Mesh字段verticesA/triangles投影到源平面/多边形，按覆盖/不重叠/边界分段/朝向/索引闭合比较；可接受不同对角线和Steiner点，不能以原polygon index数组相等验收。未来geometry shape_reference测试引用shape-projection-contract及绑定源多边形，不使用GUI面积误差作坐标阈值。原B-MATH、epoch/取消清理与候选提交边界保持；一般退化/非凸与相映射仍blocked，不以新投影宣布fullUnitReady。

R6-R5保留为proof-boundaries.json中的未证明项；合法样例能投影不证明任意半空间交集构造、数组拒绝或实际产品。P1.2/P1.4/整体partial，P1.3 C4设计接受、C6未审增量不变。

## 历史：R8原卡交接勘误（rev33时点，D9/I11语义不变）

R8／rev33历史摘要（当时待独审；现已收到dsh R8，当前入口见文首）：2026-11-14 R8：D9/I11交接勘误，修正R7-R1–R3并补跨材料一致性检查；沿用R6-R5证明边界。Codex自验待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持、C6原字节且增量未审；P1.2/P1.4/整体P1仍partial，2接受/2未接受。资料基线rev33；不进入P2–P5、产品实现或运行验证，真实worker／Check／launcher另行授权。

当前类型权威、schema请求分支、wireVersion、casebook及I11协议须一致为protocol3；measure来源适配计划与单元measurementCoverage一致，原生完整协方差B03与有界Cartesian设计控制分开。当前shape角色以上方唯一“当前shape oracle角色”行及当前contract为准。新跨材料检查拒绝旧协议/无条件sigma辅助/抬升面积角色；不增加数学或产品通过声明。

## R9原卡文书与状态勘误（不执行产品步骤）

本轮仅修R8-R1–R3与补漏检；当前消费[R9 contracts](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-163802-p1-design-r9-doc-state/contracts)，所有契约逐字节继承R8。measure双范围与shape当前角色保持既有设计；新旧协议句、章节/链接轮次和机器导航按本轮核查，不新增fixture/算法/产品/运行证明。六张剩余卡仍未施工。

## R9闭合回写与下一张设计卡（rev35）

[dsh R9](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-170522-p1-design-r9-dsh-review/review-result.json)已确认R8-R1–R3闭合，主体设计和31个fullUnitReady=false不变。当前M1–M8为非阻塞续办，详见[登记](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/非阻塞续办.md)；下一步用户在新施工会话执行[01卡](/Users/agilecho/code/Draft/docs/任务卡/阶段1-架构与施工设计/01-结构群与几何接口.md)，本轮没有执行该卡任何设计步骤。


## P1-01对G1/G2及视图/几何单元的设计交接（未实施）

2026-11-14 P1-01设计交回：追加rev36，D10/I12增量提出SG/1结构、群与几何公共设计及来源/独立演算；完整14组接口仍有6项精确缺口，本卡partial、待用户手动转dsh。P1.1继承接受，P1.3 C4设计级接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受。R9闭合仅属被审rev34，历史补档保留；02–06未施工。不进入产品实现或运行验证，真实worker／Check／launcher另行授权。

G1先消费SG/1的Id/单位/phase graph、显式3D群及参数aware covariance；G2依选定ImageKey做periodic/bonds/polyhedron/planes/shape。view/structure-render只消费完整几何和持有资源的packet，不能以MODEL/概率字段保存成功替代像素/椭球geometry。具体14组构造/生产消费/持久字段/错误/取消/undo与原source/oracle入口见[本卡设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/design.md)、[来源](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/source-field-bindings.json)和[缺口](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/remaining-issues.json)。既有单元未来文件/测试命令/预算未执行；全部fullUnitReady=false保持。原生R/t、一般群/调制、协方差、递归/选表、构造器、相机adapter缺口只停相应消费步骤；05资产不作为无关字段审计前置。06须消费独审后的具体版本，本次SG/1仅待审稿。


## P1-01补充：WordCraft职责参考（DEC-P1-04）

G1/G2与本卡render/Agent消费层按[WC适配](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-180028-p1-01-structure-geometry/wordcraft-structure-adaptation.md)和wordcraft-layer-proposal.json交接。领域model不得依赖host/DOM，geometry不反依赖codec/commands；Engine统一准入提交，Web/CLI/Agent薄适配。当前kernel/crates/cli是逻辑apps入口，root scripts承接维护gate；06汇总最终物理结构及与crates/apps/xtask的对应理由。仅设计，不执行layers/build/test或新增目录。


## P1-01 F1–F6修补对G1/G2与消费方的影响（未实施）

2026-11-14 P1-01同卡F1–F6整改：追加rev38，D11/I13/C6；另建SG/1-r2证据后继，修复数学引用、明确角色权威及四条旧表冲突、登记保留AGENTS后继、补13条MODEL/VECTS入口，并补schema范围/语义拒绝例及归档基路径。rev37旧SG/1已获dsh“主体准确、partial、轻微证据整改”结论；本次新增量仅Codex自验、待用户手动转dsh。完整六项科学缺口不变，P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial、2接受/2未接受。02–06未启动，产品和真实运行未授权未验证。

G1使用SG/1-r2的schema+requiredSemanticValidation两层义务；G2继续原计算/几何边界，不接入未资格化native链。03审原生字段时使用249条source span中新增inlineMODEL/VECTS入口，不能误判为全codec已审；04的measurement/camera/label消费须执行非负variance、PSD/方阵、near/far和RGBA规则。05资产选择及06完整物理结构不新增执行动作。具体[当前修补](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/correction-notes.md) / [24个设计例](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/domain-cases.json) / [剩余科学缺口](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/remaining-issues.json)；全部fullUnitReady=false保持。


## 当前续办：P1-01 F1–F6独审结论登记（rev39）

2026-11-14 P1-01同卡独审结论登记：追加资料rev39，技术版本仍D11/I13/C6；dsh已独立复审通过rev38的F1–F6证据/边界修补，最新独审指针指向该被审包及真实JSON/SHA。本次rev39仅回写结论并Codex自验，O1登记为非阻断报告导航说明，不改旧封存或AGENTS。P1.1及P1.3 C4接受保持，C5条件复审/C6增量未审；P1.2/P1.4/整体仍partial，2接受/2未接受，SG-01…SG-06六项科学缺口不变。02–06未启动，产品和运行验证未授权。

最新[dsh独审](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/REVIEW.md) / [真实JSON](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-194200-p1-01-f1-f6-dsh-review/review-result.json) / [本次登记回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/RECEIPT.md) / [O1非阻断说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-200614-p1-01-f1-f6-dsh-registration/nonblocking-observations.json)。

O1：旧oracle-bindings.currentVerificationEvidence定位04-full正常模式546项报告；封存只读final为570项，两者均exit0/0失败。final入口为[/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/runs/final/stdout.json)。本次只登记该导航边界，不改旧字段、旧报告或封存。独审被审版本是rev38，rev39仅为结论回写且尚无新的独立复审。


## P1-05构建、外算与未来验证后继（rev40）

本节只给05范围可交接的设计合同；Task1/6/8和G5不因本卡进入产品实现。消费[BT/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/build-targets.contract.json)逐条完成完整来源/锁/工具链/npm/SDK/许可前置，按native→WASM→同版bindgen→TypeScript/Vite→资产指纹与main manifest汇合，根npm run build必须失败即非零；app根不放Cargo.toml，Build仅唯一预览桥、CLI在dist/cli。crate/脚本名是未来合同，06须核正式实施卡，不当作已存在文件。

G5外算消费[EX/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/external-tools.contract.json)：TIDY/Madel纯输入/解析留科学/几何，进程适配放native CLI/host条件模块，WASM不得链接进程依赖；Engine统一候选准入，当前revision/lease匹配才提交。RIETAN/ORFFE及绘图查看器按各自真实链保留，不能只实现Madel/Tidy而丢功能。没有精确runtime/grammar/oracle/license就停该profile，不创建返回成功的占位。

AB/1、EX/1、BT/1当前均partial待手动dsh；[下游消费表](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/consumer-handoff.md)给01/02/03/04/06字段和限制，[未来验证](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-205326-p1-05-assets-build-external/future-validation-plan.md)预登记冷增量、120秒CA、中文IME/字体/导出内容往返、GPU/OOM/VS102、Linux/x86和真实接线。B-MATH/GRID/WEB是测试防失控提案，不是生产容量；B-EXTERNAL未分配，缺资源/运行授权不可执行。

参考WC01–03/xtask资产与层次检查，先审依赖方向和每个资产来源/许可/消费，不把路径出现在ATTRIBUTION当完整许可或科学资格。05没有关闭完整P1.2/P1.4/20组映射/57路由字段及六项SG缺口；06仍要按实际相邻接受范围汇总。


## P1-05独审登记与说明后继（rev41，技术语义不变）

2026-11-14 P1-05独审登记及非阻断勘误：追加rev41，技术版本D12/I14/C6不变；dsh确认被审rev40 AB/1、EX/1、BT/1的partial设计/来源/保全边界准确、无阻断。本轮以独立后继修O1–O4，补未施工卡名单和fontTools复现环境说明，仅Codex自验、说明增量未独审；原施工包、dsh报告及rev40封存不改。05/01继续partial，SG-01…SG-06开放，C4接受/C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。02/03/04/06未施工；不启动下一卡、产品或真实运行，交回后停止。

被审原稿与SHA见[登记对象](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/review-registration.json)；当前定位/粒度补充为[AB/1-notes-r1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/contracts/asset-inputs.json)、[EX/1-notes-r1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/external-tools.contract.json)及[消费说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261114-214811-p1-05-dsh-registration-notes/consumer-handoff.md)，BT/1仍消费原被审字节。原17类资产对象、四外算科学/进程规则、build目标、protocol3/SG1-r2及科学缺件未变。新说明仅作者自验，不把本次独审扩成全部D12/I14、C6、产品或运行接受。未施工卡精确为02/03/04/06；05已部分施工并获得设计边界独审。原dsh机器组名与默认Python缺fontTools的复现差异分别在登记/复现说明中保留，不改原报告。


## P1-02对G3及科学单元的设计交接（rev42，未实施）

2026-11-14 P1-02设计交回：追加rev42，D13/I15/C6；提出FS/1场与科学接口设计，14组按实际来源和独立预期逐项划界，保留完整生成器、参数/许可、网格/拓扑/投影/盆地等11项具名缺口，02整卡partial，待用户手动转dsh。01/05原partial及真实独审范围不变，SG-01…SG-06开放；P1.1与P1.3 C4设计接受保持，C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。03/04/06未施工；无产品、新运行或新实验，交回后停止。

实施时先消费[FS/1消费说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/consumer-handoff.md)和[14组边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/group-designs.json)；原R6公开Input/Output和future命令仍是历史有界计划，新增FS profile须按[类型影响](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/shared-type-impact.json)同步内核/TS/codec消费者，不能静默复用旧封闭请求。原G3未来文件职责继续，所有命令只属未来，当前不得运行。

| 本卡单元 | 已设计可独立消费的结果 | 完整冻结仍缺 |
|---|---|---|
| powder | 显式卷积/数组拒绝；峰位只辅助 | FS-P01 |
| sf | 显式image复振幅求和/完整结果列 | FS-S01 |
| fourier | caller有限谱与Hermitian/DC/Nyquist | FS-F01 |
| electron | DC场；全谱不能由积分替代 | FS-G01 |
| nuclear | signed DC场；不截负值 | FS-G01 |
| model-patterson | 系数模方及平方单位 | FS-G01 |
| volume-patterson | 独立core直接卷积/谱归一 | FS-F02 |
| volume-data | 实际字节重排/closure/预算 | FS-V01 |
| volume-render | 边交点；无完整三角拓扑承诺 | FS-I01 |
| 2d | 原始网格切片/显式权重求积 | FS-D01 |
| line | 两端/物理metric/全值列 | FS-L01 |
| peak-voronoi | selected峰集合及有界盆地字段 | FS-K01 |
| poly-integral | 锁定arm64菜单185 NoAction | 其它版本未知；无数值积分功能承诺 |
| density-conversion | 原26邻点/单位解释/非有限诊断 | FS-C01 |

先做真实字节fixture适配和来源/单位断言，再做有界实现与资源/取消/迟到/已提交tooLate测试；相关完整生成器须先取得remaining-issues指定输入与预期，不能以既有小算子通过放行。未来文件/命令及精度原合同保留，本包verification-plan细化真实输出全数组、独立reader/写入重开判据；本次只执行设计自验。完整计划由06汇总，不在02启动06或产品。


## 当前接续：02同卡O1–O4（rev43）

2026-11-15 P1-02同卡O1–O4整改交回：追加rev43，D14/I16/C6，FS/1 evidence-r1；补35个真实投影合同有限例、澄清Patterson平方单位、固定Xcode3.9.6和11个旧WordCraft来源快照并披露2处漂移、将生命周期4个事件例移出准入证明。177定义/协议字节、11项本卡缺口及SG六缺口不改。dsh旧结论只覆盖rev42 partial设计/来源/保全边界，本轮整改增量仅Codex自验、待用户手动转dsh。01/05原partial与接受范围保持；C4接受/C5条件/C6未审，P1.2/P1.4/整体partial、2接受/2未接受。03/04/06未施工，无产品或真实运行授权；交回后停止。

[本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/RECEIPT.md) / [整改说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/correction-notes.md) / [复现与漂移](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/reproduction-notes.md) / [下游边界](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/consumer-handoff.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/DSH-REVIEW-HANDOFF.md)。

本次授权为02同卡整改，沿用原会话/预算/权限/费用/次数/停报；不自动开新卡或扩大实现。投影helper只证明选定精确几何的schema/有限语义/资源域准入，未证明一般法向容差、插值/求积或完整科学能力。83例不含4个生命周期事件序列，7个output例仍是静态对象关系；租约释放、迟到原base/provenance及真实竞态均留未来04。复现使用新包reproduce.sh固定Xcode3.9.6，错误解释器exit2；WordCraft11旧Git blob锁SHA，当前两处来源漂移另记，不回滚外部仓库。原FS/1及dsh/旧复核/基线全部保全。


## 历史：P1-03对G4 codec/图像与exports单元的交接（FC/1 rev44，未实施）

2026-11-15 P1-03格式合同设计交回：追加rev44，D15/I17/C6，FC/1；85原路由身份保持，57条旧未审及28条旧政策完成有界来源字段审计，交291组源字段、逐profile候选选项/失败政策、损失同意与资源身份及85条未来真实内容验收合同。原4首链子集/81未准入保持，完整语法/模型桥、精度、布局/方言/许可和04宿主承诺未闭合，03整卡partial，仅Codex自验待用户手动转dsh。01/02/05原partial、FS11项/SG六项缺口、02rev43增量未独审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受不变。04/06未施工，无产品/新运行授权；交回后停止。

G4表及原exports单元的未来文件/命令仍是计划。本轮输入改消费[85精确路由/参数](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/contracts/format-profiles.json)、[291组字段](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/contracts/codec-field-policies.json)和[85个未来真实断言](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/contracts/roundtrip-assertions.json)，以[剩余缺件](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-065047-p1-03-format-contracts/remaining-issues.json)替代旧57条完全未审的接手假设；不得把candidateSchema当admitted请求，或把现有4首链子集当全原生保存资格。

未来实现先核原行/原字节/独立预期及所消费模型资格；构造可信同稿完整EncodeInventory，未知块保留/拒绝，完成逐对象LossReport和新同意，再写真正writer。验证实际文件内容、量化/单位、资源往返，新实例重开，不调用产品codec自造expected。未定writer误差、方言/布局/许可和非周期/完整场景模型桥按路由停报，禁止按后缀或family空壳批量开支持。

图像/mesh保留JP2无产物、SGI不合规、TIFFalpha丢失、STL非有限法向、EPS/PS未独立渲染等反例；2D/Text按辅助合同核采样/单位/Output与Comment。04尚未施工，接收loss/resource/取消/迟到义务及待协商布局，不假设host承诺已闭合；06汇总完整实施顺序及实际独审身份。本卡没有创建产品codec文件、运行上述cargo/node/browser/外算命令或实现保存，fullUnitReady均不提升。


## P1-03 O1–O7对G4/exports的当前消费（未实施）

2026-11-15 P1-03同卡O1–O7整改交回：追加rev45，D16/I18/C6，FC/1 evidence-r1。登记用户带回的rev44有界设计/来源/保全边界独审通过，原文6项与实列7项及411误写保留说明；当前整改增量仅Codex自验。291组明确为路由级字段清单，未建立字段到span/逐字段语义绑定；补3条读入路由的6份直接fixture及预期，I01改原生vesta；补RAW尺寸元数据来源，明确81候选shape及模板语义/精度未冻，损失枚举完整性未证，状态时间同步。原4子集/81未准入、SG六缺口/FS11项、01/02/05原partial、02rev43增量未独审、C4接受/C5条件/C6未审、P1.2/P1.4/整体partial及2接受/2未接受保持。03仍partial，04/06未施工，无产品/新运行授权；交回后停止。

消费[路由级字段清单](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/contracts/codec-field-policies.json)、[直接fixture](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/contracts/roundtrip-assertions.json)及[scope统计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-074631-p1-03-o1-o7-corrections/scope-statistics.json)。291库存数/19规则模板/1精度模板不代表291条可直接编码的独立合同；完整81profile、逐字段语义/精度、字段span和loss枚举仍是实施前对应缺件。06不得据本次文字/引用修补将fullUnitReady置true。04只接收原候选资源身份/取消/迟到义务，未获得新host/fencing/ack承诺。没有运行未来cargo/node/browser/原件/外算命令或创建产品文件。


## G4后续施工进入条件细化（rev47，仅设计）

消费本轮field-evidence/parameter-qualification/numeric-contracts/source-model-mapping及B12最小材料，不以85键/291库存/typed候选数量决定实现资格。后续codec切片必须先明确源版本、可表达字段、真实选项及writer预算；已有绑定只可用于相同有限样本/组件。m3d须独立整数位流与误差证明，PRI扩展ABI须原producer规范，八项缺件只阻相应profile。

完整PlanEncode前，冻结完整Engine源捕获和独立枚举审计版本；迁移报告/同意摘要并验证新增源字段、第二个共享bytes场、未知块全部报告或拒绝。当前64叶演算是测试设计输入，不是产品实现。资源writer切片还须03原件相对布局判据与04写者/fencing/配额/ack/恢复；真实写出→独立读回→新实例重开须另授权卡，当前04/06未开工。


## G4完整字段/参数/资源设计消费（rev48，未实施）

消费[03整卡设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/design.md)和[04/06交接](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/consumer-handoff.md)，按实际输入/源类型/参数与字段规则施工。原57未审、m3d位流、PRI各选定flag、DEN/STO、旧数据库、原生完整源及资源/损失已有精确后继，不能继续仅按旧missing文本索取全部新样本。全历史producer、01/02/05资格与未来真实输出仍是各自门槛。

G4先实现实际类型完整visitor与原始/当前分存，继而各格式纯decode/plan/encode和独立验证器；04接真实保存/取消/关页/新实例，06只在其独立卡汇总实施顺序。每个完整用户结果均要检查全部实际内容、owner/ref、预算及原失败回归；不能按固定文件行数拆支持，不把测试绿或source-only当产品通过。当前fullUnitReady及其他卡接受状态不提升。


## P1-04 对Task5/7/8及G6的设计接续（rev49，partial）

本节仅安排04相关接缝，未执行产品任务，不代06总体收口。

| 单元 | 顺序与完成判据 | 缺口与失败处理 |
|---|---|---|
| Task5 snapshot/save-state | 完整Source7/ledger与预期先于encode；候选实际artifact绑定、03三种完成凭据、bindingIntent/publicationSequence及content dirty；先有限反例再实现 | HS-01/02只阻真实host冻结；未知commit不重试、不清可能可达资源；mock不是实际写入 |
| Task7 vesta-coordinator/page/files/tools | 重核共享锁定源；复用serial/path/baseline/ticket，最小Vesta注册/扩展名/源资源/分块/查询适配；mode只layout、strict保稿；资源先行、primary-last、父sync及完整独立读回 | 无全局CAS；基础SpaceFiles缺父sync；Sound目录交换/caps不套用；HS-01～03仅阻相应资格，失败恢复编辑 |
| Task8 CLI交接，CA范围不扩大 | page最新revision及所有maypublish操作结算后释放，CLI官方接口读同真实WorkRef；CLI实际保存/独立读回后新page新epoch继续编辑 | C6仍另行授权worker/Check/launcher；不以旧Engine/CWD/cache补缺，不创建产品workspace |
| G6 lifecycle/database/preferences/interaction/host | AL/1请求/候选/显式import、12偏好字段及version/Apply/Cancel、焦点/IME/历史/动画屏障、HostAction完成凭据；UI/Agent同命令，偏好不存作品字节 | AL-01～05仅停具体服务字段/范围/OS资格；保留R103选择性Cancel/default和原Stop未证，不将unsupported删成已完成 |

未来交tests/delivery/host-save/run.mjs及独立verify_persisted.py，按verification-plan明确前提、独立预期、故障点和原始证据。实际写入→独立完整读→销毁旧实例→新实例重开→继续编辑才是完整保存用户结果。HTTP/文件存在/计数不足。本卡只运行Python设计辅助验证，06与产品/构建/部署/运行均未启动。

完整设计、精确字段、来源与8项具名缺口见[04设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/design.md)、[HS/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/contracts/host-save.contract.json)、[AL/1](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/application-lifecycle.contract.json)、[源码绑定](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/host-source-bindings.json)、[remaining](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/remaining-issues.json)及[未来验收](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/verification-plan.md)。本轮仅Codex自验、待用户手动dsh；旧03/01/02/05/C6资格不升级。


## P1-04 对Task5/7/8与G6的续办输入（rev50）

Task5/7/8新增按每次操作捕获基线、poll后未知不重放、旧claim历史结算、原操作凭据与新生命周期恢复分离的设计约束；G6消费四份来源绑定的COD表单、手册参数单位和逐种HostAction参数/回复。未开始任何产品步骤。

依赖须按具体步骤表达：HS-01/02挡宿主保证及原操作恢复的最终冻结；AL-02挡完整偏好域/持久规则，AL-04挡未证原生分组一致性。HS-03/AL-01/03/05的真实运行资格仍是后续实现/原件验收要求，不能只因本卡未运行产品而把其它设计一概退回。未来必须实际写入、独立读取完整对象/资源、销毁旧实例、新实例重开并继续编辑；有限演算和静态网页字节均不能替代。

04同卡续办已补HS/2页面投递与恢复、AL/2真实表单及逐种系统动作设计。原8项均保留：4项缺设计输入，4项留待后续真实资格；整卡partial，待用户手动dsh。03 rev48整卡待审、01/02/05 partial、SG六/FS11、C4设计接受/C6未审、P1两接受两未接受保持。 [04续办回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/RECEIPT.md) / [续办设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/design.md) / [具体缺口](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/remaining-issues.json) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261117-232931-design-completion/DSH-REVIEW-HANDOFF.md)。


## P1-04 四缺口对Task5/7/8与G6的消费（rev51）

Task5/7/8消费host profile入场/每op基线/依赖目录sync/私有代际/query/退休合同，G6消费整笔偏好事务、数值词法后继、history包括current及tick/drain/focus规则。具体合同与源事实见[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/RECEIPT.md) / [完整设计](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/design.md) / [最少补件](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/external-inputs.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-004924-four-design-gaps/DSH-REVIEW-HANDOFF.md)。

最终共享宿主保证仍依HS-01真实profile单；完整原生偏好域仅缺AL-02指定静态函数/必要callback，原动画grouping仅缺AL-04指定调用链/序列。HS-02自身设计齐备但共享HS-01实际能力未合格，不能据此宣布恢复已实现。四项其它真实资格保留未来完整写/读/销毁旧实例/新实例重开继续编辑及原失败。原件/产品运行与06均未开启。


## P1-04 来源补齐对Task5/7/8和G6的消费（rev52）

Task5/7/8可消费HS/4选定宿主设计与HS/3恢复设计，不等待额外平台承诺单；实际VESTA接线、目录sync、private registry、并发和完整重开仍是后续施工/资格，不能据别的软件历史通过移植为已验。未参与writer竞态明确保留，不以全域新CAS作默认设计前置。

G6原生偏好/动画域冻结只缺AL-02/04指定函数体。最小静态首批6段（4偏好+Timer/Idle），条件最多7段；已有LeftUp/load/save/PushHistory不重取。原件取证只列方案、没有获得或执行新动作；06及产品未开工。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/RECEIPT.md) / [设计与来源](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/design.md) / [最小原件取证方案](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-011705-source-history-resolution/DSH-REVIEW-HANDOFF.md)。


## P1-04 静态证据对G6及Task5/7/8消费（rev53）

HS/4与HS/3继续供后续施工消费。G6可消费AL/3 evidence-r3已确认控件/内层处理与Timer/Idle观察；AL-02完整域和AL-04原件分组仍缺，不能标全部完成或实际运行通过。仅未来明确实施卡接线，不在本轮实现产品。

原授权首批6段成功，条件7段未执行且旧窗口关闭；不重取已取得体、spin和LeftUp，不扩大为GUI或额外模型。下一最小集合7段/3144字节和条件GetValue596字节分开登记，许可边界如实累计。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/RECEIPT.md) / [设计与静态事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/design.md) / [最小剩余范围](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-020831-authorized-static/DSH-REVIEW-HANDOFF.md)。


## P1-04 授权补取结果供G6/Task5/7/8消费（rev54）

04补取续办rev54：剩余7段与必要GetValue596字节均各1次成功，准备后单绑定计时24.22秒即关闭，无GUI/额外模型。偏好有效值/字体/简单算式与旧值分支、intervali32到Timer及Undo/Redo入口已补；AL/4输入设计明确继任，实际转换与动画原件历史分组仍有更窄缺口，整卡partial待用户手动dsh。动态未授权，未开06。

G6消费AL/4 raw输入/profile/候选/effective分离及12字段资格，禁止从“函数读取成功”生成全偏好准入。AL3历史选定规则继续，原件归组仍缺证；Task5/7/8宿主/恢复设计不变。两轮14个授权体均完成，各一次，24.22秒新窗口已关；下次更窄静态白名单或动态均未执行，不能顺带进入产品或06。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/design.md) / [最小剩余](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-030321-static-completion/DSH-REVIEW-HANDOFF.md)。


## P1-04 转换与槽目标供G6/Task5/7/8消费（rev55）

04续办rev55：SetValue348/OnRotate552及必要三槽各8字节均1次成功，准备后单绑定18.78秒即关闭，无GUI/额外模型/其它函数跟进。已补实际数值转换、候选/显示/存储分离和Canvas聚焦/刷新/交换缓冲静态目标；OnRotate已证是固定轴按钮，不能代替持续动画模式。完整偏好UI资格及原件持续动画分组仍partial，待用户手动dsh，动态未授权、未开06。

G6新增AL4转换profile、实际值读回要求及AL3历史证据r5；宿主/恢复设计沿用。禁止据文本/返回数相同确认偏好保存，禁止把固定轴按钮或Refresh符号当持续动画归组证明。当前16函数体/3槽是三轮已知授权合计，不是完整成本总账。未追其他函数，未来输入/静态可选范围只列不执行，动态仍另行授权。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/design.md) / [最小剩余输入](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-032858-static-conversion-rotation/DSH-REVIEW-HANDOFF.md)。


## P1-04 格式/模式/集合历史消费（rev56）

04续办rev56：两格式常量和Mode0及三个必要条件体均各1次成功，准备后单绑定50.68秒关闭，无GUI/额外模型/其它函数。%g/%.*f、模式/轴/计时器启停和条件清选择后逐ownerDoc PushHistory2已补；AL4历史策略明确覆盖纯UI不记历史的泛化，逐Doc回执不冒称跨Doc原子性。完整偏好输入资格/实际动画UndoRedo分组仍partial，待用户手动dsh，动态未授权、未开06。

G6消费AL4偏好格式来源和明确清选择历史事件/集合权限/逐Doc结算，保留native分组未证；Task5/7/8宿主/恢复不变。无真实跨Doc协调能力不能承诺all-or-none；已提交Doc不能因后续失败盲回滚/重放。20体3槽2常量不重复，剩余改列实际资格/计数轨迹，不继续泛化追函数或开06/产品。[04本轮回执](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/RECEIPT.md) / [设计与事实](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/design.md) / [最小剩余证据](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/collection-plan.md) / [手动dsh材料](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261117-224424-p1-04-host-save-application/attempts/20261118-035409-static-format-modes/DSH-REVIEW-HANDOFF.md)。

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

## P1-06 全功能实施安排（rev60 / IP1，全部产品步骤未执行）

> IP1保留为既有15结果及合同边界。当前步骤进入以文末IP3为准（IP2保留历史），IP1整单元依赖不得作为全部源码的总前置。
本节是当前实施安排，覆盖旧Task1–8/G1–G6及31领域；旧计划保留历史与有界签名，本节对包名、真实依赖及实施顺序有明确后继。计划根 `/Users/agilecho/code/VestaCraft/app` 当前不存在。任何步骤开始执行须另获产品/运行授权，当前06只完成设计和自验。P1.2/P1.4/整体仍partial；有界可接手不等于全功能可施工。
精确合同/版本/SHA与被审范围：[合同寄存](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/contract-register.json)；逐单元机器安排：[实施就绪](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/implementation-readiness.json)；31组原签名、输入/独立预期/单位/公差/消费者：[领域消费](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/domain-consumption.json)；85格式及额外IO：[格式消费](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/format-implementation-bindings.json)。这些为本卡证据，不另立长期功能规格/覆盖台账。
[需求依据](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/requirements-basis.md)按现行PRD/技术方案/DEC裁定；[接口相容](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/interface-compatibility.md)区分内部设计与产品准入；[未来验证](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/verification-plan.md)固定真实写读/销毁重开/续改及异常。独审由用户手动转dsh。
### 结构参考与工程职责
WordCraft实际HEAD `93bf14883c7a57a802f383cbcce880aae1328cc8`，有并存WIP；逐文件工作区SHA/HEAD blob差异和实际参考入口在wordcraft-reference.json。其L0 units→L1 model/fonts→L2 layout/formats→L3 render→L4 Session→L5 MCP→L6 UI/app映射为本产品model/geometry/science/codecs→Engine统一事务→bindings/CLI/host→TS/React/Three；xtask资产/层次/制品质量职责由本项目scripts承担。pure层禁止DOM/账号/网络/进程，codecs只候选，Engine唯一commit，host唯一I/O。
保留app根package.json、kernel根Cargo.toml及计划crates工程。采用统一命令、失败恢复快照、固定独立fixture和完成身份；不照搬egui/Rust-only界面、本地profile、浏览器作品副本、下载或TCP控制密钥策略。字体外置受许可且Web/CLI分别验；16,437,364B CJK不因WordCraft策略被裁掉。
### 依赖与预算边界
U00工程输入/锁和U01基础数学/类型可独立准备；实现顺序只按真实生产消费：U00→U01→U02捕获/持久桥；U03/U04/U05/U07/U11/U12各按字段进入；U06/U08/U09/U13消费已具备类型/资源；U10真实外算及U14目标/CA需专项资格。它不是多Agent/并行执行授权，也不要求前五张卡整卡通过才开始所有工作。
B-MATH 30s/256MiB/16MiB、B-GRID 60s/512MiB/64MiB是原05测试提案；B-WEB 60s/一页，其余资源未来卡先补数值；B-EXTERNAL、冷构建总额/次数、真实模型均未分配。CA120s构建/90s截图不放宽。生产Budget需目标测量，不能拿测试额度或Mac24GiB追认。全部run自有PID/组/目录，取消/timeout只清自己的资源；累计历史费用/次数/停报不重置。

### U00 可重建的同内核Web与CLI交付底座
状态：`bounded-design-ready`；本卡未实现/未运行。原领域：应用或工程横切结果。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：无其它实施单元前置；合同：BT, AB, CA，实际字节/SHA在contractBindings。阻断完整冻结：P05-08, P05-09；未来资格：P05-08, P05-09。
计划创建/修改（非已存在）：`package.json`、`package-lock.json`、`rust-toolchain.toml`、`kernel/{Cargo.toml,Cargo.lock}`、`scripts/{build,test}.mjs`、`licenses/{dependencies,assets}.json`、`BUILDING.md`、`tests/delivery/build-manifest.test.mjs`。
接口与生产消费：继承Task1 verifyManifest(root:string,manifest:BuildManifest):Promise<void>；BuildManifest字段按Task1。crate names本轮定为vesta-{model,commands,codecs,geometry,science,bindings}、vestacraft-cli、vestacraft-preview；CLI bin vestacraft，preview bin vestacraft-preview。BT/1中vesta-cli/vesta-preview为待06确认旧名，使用本后继且不改BT原件。
- [ ] 冻结源和许可；按BT现有候选版本生成真实完整Cargo/npm图、Node/npm/SDK/镜像身份；缺缓存停获取步骤，不能换latest。纯类型/架构检查可先做。
- [ ] 创建app根package.json、kernel workspace；统一native→WASM→同版bindgen→TS→Vite→许可/制品manifest。crate不引用DOM/进程至model/science。
- [ ] 对manifest刻意改一WASM字节、删CLI/字体许可应失败；没有实现的脚本不能空返回0。
- [ ] 真实冷构建和增量分开；目标Linux与嵌入浏览器能力由U14资格步骤产生，Mac不追认。

独立预期与用户结果：
- Web JS/WASM/CLI共享sourceDigest及实际lock/artifact SHA；Build只有唯一preview候选，CLI在dist/cli。
- U00只交可审构建结果，不冒称业务链或全科学通过。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
npm ci --offline --ignore-scripts && npm run build && node --test tests/delivery/build-manifest.test.mjs
```
预算/归属：BT冷构建须未来卡先分配数值wall/RSS/disk/次数；CA增量120s不能放宽。失败/取消恢复：缺lock、cache、许可证、镜像时停止该构建；保留源/数学输入；只清自己build-cache，不动别卡/用户空间。
真实内容往返：构建正式source/locks → 构建脚本生成dist和manifest，不制造用户作品 → 独立sha工具逐制品核对；静态Web/CLI实际加载同一指纹 → 移除本轮拥有进程及缓存实例，不删除源 → 干净隔离runner按同锁重建和加载 → 一处具名源码修改后重建，指纹及受影响测试变化；业务编辑留U01/U02。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U01 在当前页创建晶体并连续手改、工具编辑和撤销
状态：`bounded-design-ready`；本卡未实现/未运行。原领域：应用或工程横切结果。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U00；合同：BOUNDED3, SG, FS，实际字节/SHA在contractBindings。阻断完整冻结：无新增设计输入；仍有审查/产品授权及真实实现资格；未来资格：相关目标/资源验证。
计划创建/修改（非已存在）：`kernel/crates/model/src/{ids,cell,project,resources}.rs`、`kernel/crates/commands/src/{registry,transaction,history,jobs}.rs`、`kernel/crates/bindings/src/lib.rs`、`web/src/kernel/{worker,client,generated}.ts`、`web/src/editor/{App,cell-editor,site-editor}.tsx`、`web/src/host/page-api.ts`、`tests/browser/first-chain.spec.ts`。
接口与生产消费：Cell3::from_parameters([f64;3],[f64;3])->Result<Cell3,DomainError>；Engine::execute(CommandRequest,&mut dyn ResourcePort)->CommandOutcome；Engine::advance(u32,&mut dyn ResourcePort)->Vec<JobEvent>；KernelSession/VestaPageV1按I30§4/10.1，project.new/cell.set/site.add/site.setFractional/history.undo/redo使用§3封闭schema。首链P1，不把全部SG/FS profile注册为产品API。
- [ ] 先固定ID/时钟和4Å立方C1、VS9、VS52输入及独立oracle；创建合法Cell和资源计数。
- [ ] 同一Rust注册表生成TS、CLI和工具schema。实现request去重/expectedRevision/atomic与sequential，旧epoch拒绝。
- [ ] 单Worker驱动advance并yield；ArrayBuffer转移/retain-release/grow后旧view无效；UI draft/IME确认后才正式命令。
- [ ] 真实页Agent改a=5、手改a=6、Agent读到V96；Undo/Redo内容及monotonic revision；最终Saved链只在U02接通后验。

独立预期与用户结果：
- 4Å体积64Å³；VS9 Hf=(1.75,3√3/4,0)，V12√3；基础解析1e-12只限该基础fixture。
- 4→5→6，V64→80→96Å³；atomic第二项非法不提交；相同request只一次；Worker/Rendered同document/epoch/revision，字体未就绪不算可见。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
cargo +1.99.1 test --offline --locked --manifest-path kernel/Cargo.toml -p vesta-model --test cell_identity && npm run test:browser -- --grep first-chain
```
预算/归属：B-MATH；B-WEB每例60s/一页，未来卡补RSS/GPU/disk/出口。失败/取消恢复：commit前取消无修订且释放候选；commit后tooLate，撤销走history。Worker崩溃从committed RAM checkpoint新epoch；整页丢失只恢复U02最后确认文件。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U02 工程及多资源真实保存、另存、关闭交接和新实例续作
状态：`source-bound-design-ready-runtime-unqualified`；本卡未实现/未运行。原领域：应用或工程横切结果。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U00, U01；合同：FC-format-profiles, FC-full-source-audit, FC-resource-graph, FC-resource-reopen, FC-resource-content-comparison, FC-save-artifact-binding, FC-typed-source-restore, FC-save-target-policy, FC-save-settlement, HS-base, HS4, AL3-host-profile, AL3-operation-recovery，实际字节/SHA在contractBindings。阻断完整冻结：无新增设计输入；仍有审查/产品授权及真实实现资格；未来资格：HS-01, HS-02, HS-03。
计划创建/修改（非已存在）：`kernel/crates/commands/src/{snapshot,source_capture}.rs`、`kernel/crates/codecs/src/formats/native_vesta.rs`、`web/src/host/{save-state,page-api}.ts`、`/Users/agilecho/code/BranchDraft/dsh/plugins/software/{vesta-files,vesta-coordinator,vesta-page,vesta-tools}.mjs`、`/Users/agilecho/code/BranchDraft/craft/vestacraft/{upstream.lock.json,patches/}`、`tests/contracts/{save-state,bundle-plan}.test.mjs`、`tests/browser/native-lifecycle.spec.ts`。
接口与生产消费：snapshot(&ProjectState,SnapshotOptions)->Result<SaveBundle,CommandError>；SaveTracker.confirm(HostAck)->SaveState/reject(HostNack)->SaveState为原Task5计划API。持久确认必须按FC artifact-only comparison和HS4/HS3 operation registry；单独调用confirm无授权。HS4/Source7内部设计须显式迁移producer/schema/server/receiver/client，不能借protocol3宽松JSON。
- [ ] 先实现Engine独立完整source visitor/owner ledger/数值和原历史捕获；未知活动字段拒绝完整保存，不以source-only藏丢失。
- [ ] codec只私有候选，独立candidate artifact-only恢复全对象/全部grid/mask/frame及typed引用；source expectation在编码前冻结，不从输出造。
- [ ] 未来共享施工先重核平台HEAD/源码/权限，复用serial/SpaceFiles/naming；每op冻结baseline，资源/manifest先写读回并sync新增边，主文件最后publish+parent sync。
- [ ] HS3私有不可变journal generations/完整操作及语义凭据；查询绑定principal及原op，失联先resolve；同服务受控写者保证，任意Shell check-use窗残余明确。
- [ ] 已有Save原位置；新工程明确合法路径优先，否则绑定workspace，无位置needs-location；首次Save/SaveAs成功才安装WorkRef，导出不重绑；60秒自动保存合并最新pending。
- [ ] 受控关闭停止编辑/tick，处理IME与在途发布，最新稳定Saved后release给CLI；新page新epoch用同实际文件恢复并继续改。

独立预期与用户结果：
- C1+两同bytes但不同owner场、当前热参数/截面/相机/独立历史均实际持久恢复；全512或实际全N值比较，不能只摘要。
- R7 ack不清R8；A旧ack不改B；507资源2失败旧primary不变；409拒绝；提交后断线/缺registry/语义失败为commit-unknown/committed-unverified/expired-unresolved并保持dirty。
- 已实际写入路径在文件列表可找，独立读取器新上下文不使用旧Engine/cache/CWD补缺；HS03原件资源重开失败保留。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
node --test tests/contracts/save-state.test.mjs tests/contracts/bundle-plan.test.mjs && npm run test:browser -- --grep native-lifecycle
```
预算/归属：B-GRID + B-WEB；HS4实际Linux volume/registry/配额资格由未来实施证据产生，不要求04再补无意义全局CAS。失败/取消恢复：未发布候选且已证无读者才可清本轮孤儿；已提交/unknown可达资源不删。Cancel恢复编辑；Discard只丢RAM，不撤销已发布；outside-writer无globalCAS，强保证请求E_HOST_SCOPE_UNSUPPORTED。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U03 可编辑多相、群、变胞、位点和张量的晶体工程
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：phase, cell-symmetry, transform, sites-adp, vectors。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U01, U02；合同：BOUNDED3, SG, AB, FC-codec-field-policies，实际字节/SHA在contractBindings。阻断完整冻结：SG-01, SG-02, SG-03, P05-01, P05-04, P05-05；未来资格：相关目标/资源验证。
计划创建/修改（非已存在）：`kernel/crates/geometry/src/{phase,symmetry,transform,adp,vectors}.rs`、`kernel/crates/model/src/{phase,site,adp}.rs`、`web/src/editor/{phases,sites,symmetry,adp}.tsx`。
接口与生产消费：逐组constructor/step/input/output/consumer精确继承domain-consumption.json；SGPhaseGraph/SGGroupSetting3/SGPeriodic3/SGAdp/SGCovariance/SGVector按SG schema+semantics。SG-02调制/超空间无公共DTO，禁止伪造通用函数。
- [ ] 先实现canonical根图存在性/无环、显式3D算子、P1位点及已定张量换基；冻结source→current字段映射及独立不变量。
- [ ] 非正交两相R/t依SG01；完整setting/Wyckoff/磁/调制依SG02/05表资格；具体原生编辑等待最小合同，不从shape面积反推。
- [ ] 完整协方差/约束导数、PSD/零/负椭球/概率几何依SG03及P05-05；已有Cartesian J C Jᵀ不被阻断。
- [ ] 具备相应合同后实现实际编辑与source/typed恢复，复用U02整链；保留R64原失败与5位打印政策两套。

独立预期与用户结果：
- 原P1变胞三个source case逐cell/site/occupancy/ADP字段比较；G′=PᵀGP，H′=HP、f′=P⁻¹(f−p+n)。
- 每参数含单位；variance .04Å²与sigma .2Å不可混；R64打印5.000001e-6不外推内部误差；群像完整ImageKey，不能只数量。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
cargo +1.99.1 test --offline --locked --manifest-path kernel/Cargo.toml -p vesta-geometry --test phase_reference
```
预算/归属：B-MATH。失败/取消恢复：缺setting/源字段只拒绝该原生投影/算子；保留source bytes/旧稿，其它明确3D/Cell/事务和纯数学可继续。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U04 选定周期像的测量、边界、成键和去重结果
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：duplicate, bonds, boundary, measure。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U01, U02；合同：BOUNDED3, SG, AB, EX，实际字节/SHA在contractBindings。阻断完整冻结：SG-03, SG-04, P05-02, P05-03；未来资格：相关目标/资源验证。
计划创建/修改（非已存在）：`kernel/crates/geometry/src/{images,measure,neighbors,bonds,duplicates,boundary}.rs`、`web/src/editor/{measure,bonds,boundary}.tsx`。
接口与生产消费：SelectBondPairs/ClipExplicitImages/MeasureImages/FindDuplicates原有Input/Output和step见domain-consumption；SGSelectedImage/SGBondRule/SGBoundary/SGMeasurement。selected_distance必须消费ImageKey；merge不是detect-only结果别名。
- [ ] 固定VS9/52具体像，先实现选中像距离/角/Cartesian covariance与显式clipping；禁止逐轴round替代三斜最近像。
- [ ] 有界threshold先binary32平方规则；阈值计数和原生合并metadata分别验。
- [ ] 完整recursive frontier/special alias/near-chain跨元素合并/BV表选择依SG04/P05-02/03；未冻时检测可交，不能提交猜测的平均site。
- [ ] 实际图/测量报告→持久对象→独立文本/CIF/原生读回→新实例再次测同对象/像。

独立预期与用户结果：
- VS52 9Å和shift−1 1Å；VS9 √19/2与√7/2；显示5e-6Å，角5e-5°只限原显示半末位。
- 解析distance方差单位Å²；零长度/零法向拒绝；no-op不增revision，Boundary Apply后Cancel保留，坏输入不丢原稿。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
cargo +1.99.1 test --offline --locked --manifest-path kernel/Cargo.toml -p vesta-geometry --test selected_image
```
预算/归属：B-MATH。失败/取消恢复：未知merge只阻提交该merge；已证显式几何、保存可继续。资源预算先计候选像/边，取消释放临时列表不撤销已提交。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U05 生成并编辑晶形、平面与配位多面体几何
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：polyhedron, planes, shape。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U01, U02；合同：BOUNDED3, SG, AB, FC-codec-field-policies，实际字节/SHA在contractBindings。阻断完整冻结：SG-05；未来资格：相关目标/资源验证。
计划创建/修改（非已存在）：`kernel/crates/geometry/src/{polyhedra,planes,shape}.rs`、`web/src/render/shape.ts`、`web/src/editor/{planes,polyhedra,shape}.tsx`。
接口与生产消费：MeasurePolyhedron/MillerPlane/IntersectHalfspaces原constructor/step以及SGPlaneSpec/SGHalfspace/SGShape/SGHalfspaceCertificate；源polygon→Mesh使用convex-local-boundary/1，不能改名faces当triangles。
- [ ] 从H⁻ᵀh计算单位法向，区分plane offset与d_hkl，允许合法平移盒中零/负offset。
- [ ] 先实现supplied hull测量与已定精确空/无界/降维certificate；8相48面面积是事后几何/显示证据，不接受一般相定位或构造器。
- [ ] 一般least-squares符号/退化、非凸邻居/hull、近平行鲁棒/误差与完整constructor依SG05，不选任意epsilon。
- [ ] 原生几何/mesh输出经U13，独立mesh拓扑/面积/体积后新实例恢复并修改一个Miller面。

独立预期与用户结果：
- SiO6 V32/3→40/3Å³、D0→4/39、λ1→1.023356477765、ECoN6→4.3148170791；各显示4/5小数半末位按S06。
- projection检查面集合与面积，不把支持性source面数当Mesh逐三角oracle；空/无界拒绝无假mesh。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
cargo +1.99.1 test --offline --locked --manifest-path kernel/Cargo.toml -p vesta-geometry --test shape_projection
```
预算/归属：B-MATH。失败/取消恢复：constructor未冻保留supplied mesh/解析平面及存档；无界/退化返回具名错误，旧形状和持久资源保留。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U06 当前页可见且可继续编辑的结构视图和三维交互
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：view, structure-render。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U00, U01；合同：BOUNDED3, SG, AB, AL-base, AL4-history-evidence.contract.json, AL4-manipulation-history.schema.json，实际字节/SHA在contractBindings。阻断完整冻结：SG-06, P05-03, P05-06, AL-04；未来资格：AL-03, AL-04, SG-06, P05-06。
计划创建/修改（非已存在）：`web/src/render/{renderer,packets,materials,labels,environment,camera}.ts`、`web/src/editor/{interaction,panels,properties}.tsx`、`kernel/crates/commands/src/scene.rs`、`tests/browser/scene-editing.spec.ts`。
接口与生产消费：ComposeCamera/BuildAtomInstances及SGCamera/SGNativeCameraFields/SGRenderPacket；VestaPageV1.render和RenderPacket按I30。AL4集合清选择每capturedDocuments条带selectionProfileDigest；服务端判权限/资格；未知优先unresolved，不宣称跨Doc原子。
- [ ] 先sphere/explicit transform/所选像pick，DOM文本/IME/列表/模态快捷键分路；同document的手动和工具走Engine。
- [ ] 透视/平行、SCENE原始参数保存和canonical相机分开；SG06未冻panWorldA/FOV映射不得猜。
- [ ] 实体/线宽/材质/光照/alpha/深度与AA requested/effective按真实frame与独立几何/像素；SG06/P05-03/06字体半径资格逐项。
- [ ] AL4仅当前notebook已开向量范围；适配器单Doc/授权集合有意收窄。每Doc outcome及集合优先级按合同；纯focus/noop无history，条件性清选择PushHistory请求不等于实际Undo内容。
- [ ] Properties字段Cancel/Apply/SaveDefault按03每条mask，不全量rollback；真实保存、重开再pick/改相机通过U02/U13。

独立预期与用户结果：
- 旧MODEL失败保留；frame tuple、可见对象/位置和字体ready同时成立；GPU lost失效旧frame，恢复后新Rendered。
- 所有owner/hidden/site区分；逐Doc不同profile合法，请求级profile拒绝；任一unknown先unresolved，其余failed/partial/unchanged/complete按AL4。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
npm run test:browser -- --grep scene-editing
```
预算/归属：B-WEB每例60s/一页；RSS/GPU/disk/出口须先有实际host数值。失败/取消恢复：GPU/字体失败只影响呈现；Engine Committed不回滚。AL04只挡原生动画历史等价；固定手动旋转/正式场景编辑与界面布局可继续。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U07 体数据实际字节导入、运算和完整工程恢复
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：volume-data。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U00, U01, U02；合同：FS, FC-format-profiles, FC-codec-field-policies, FC-resource-graph, FC-typed-source-restore，实际字节/SHA在contractBindings。阻断完整冻结：FS-V01, P05-09；未来资格：相关目标/资源验证。
计划创建/修改（非已存在）：`kernel/crates/science/src/{field,field_ops,resample}.rs`、`kernel/crates/codecs/src/formats/{cube,xsf,pri,m3d,den,sto}.rs`、`web/src/editor/field-layers.tsx`、`kernel/crates/science/tests/field_layout.rs`。
接口与生产消费：NormalizeGridLayout和FSFieldBinding/FSGridOperationInput/FSBudgetPlan/FSFieldOutput；FieldHandle为逻辑对象，不按Resource digest合并两场；closure/order/unit/bytes绑定真实ResourcePort。
- [ ] 按03每route sourceContract解码真实原字节，明确x/z-fastest、core和closed planes；typed byte view不跨grow。
- [ ] 先有界重排/加权/实际全值，double owner同资源独立；generic声明单位与electron/nuclear/Patterson严格区分。
- [ ] 一般重采样/单位和生产memory capacity依FSV01，VS102只旧大网格观察；测量 scratch/transfer/GPU/undo峰值再定生产准入。
- [ ] 通过U02保存materialized全部数值和当前编辑，销毁原Engine/NameMap后独立文件重开；不能由旧源bytes冒充当前场。

独立预期与用户结果：
- 选定shape/order/core/closure和每个值与独立解包数组比较；资源长度必须dtype×grid一致，整数溢出/非有限/坏closure拒绝。
- CUBE bohr坐标不推出scalar换算；所有N值、owner、相别、units重开一致；源文件与当前值不同也必须保存当前值。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
cargo +1.99.1 test --offline --locked --manifest-path kernel/Cargo.toml -p vesta-science --test field_layout
```
预算/归属：B-GRID 60s/512MiB RSS/64MiB输出仅测试；生产maxResident/field/GPU单独实测。失败/取消恢复：取消/资源不足不安装一半field，释放临时和重复lease；原场/已存工程可继续。新capacity未测只停超出已资格profile的准入。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U08 从结构得到反射、粉末谱和完整模型密度
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：powder, sf, fourier, electron, nuclear, model-patterson。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U01, U02, U07；合同：BOUNDED3, FS, SG, AB, EX, FC-auxiliary-io，实际字节/SHA在contractBindings。阻断完整冻结：SG-02, SG-03, FS-P01, FS-S01, FS-F01, FS-G01, P05-04, P05-07；未来资格：相关目标/资源验证。
计划创建/修改（非已存在）：`kernel/crates/science/src/{structure_factors,powder,fourier,density}.rs`、`web/src/editor/{reflections,powder,density}.tsx`、`kernel/crates/science/tests/spectrum_controls.rs`。
接口与生产消费：ConvolvePowderLines/SumStructureFactors/SynthesizeFourier/ElectronDc/NuclearDc/ModelPattersonCoefficients及FS FSSumInput/FSReflectionResult/FSSpectrumInput/FSSpectrumResult；DC或系数模方不能包装成全体素generator；未冻全生成器不新增函数。
- [ ] 先explicit lines卷积、显式image复求和、caller有限谱和独立DFT，分别标bounded；SG/FS profile准入迁移同步各消费者。
- [ ] 全species/radiation/magnetic/modulation、反常边、精确同位素质量/散射依SG02/03及FSS01/P05-04，不能用质量数A代u。
- [ ] 完整grid/cutoff/phase/normalization与全voxel预期依FSF01/FSG01；完整粉末多相强度/背景/宽度CTHM等依FSP01；RIETAN需要U10资格。
- [ ] 输出反射全列、curve全点、field全体素，经U02/U13真实存档、重开再改一个物理输入，应重算而非固定fixture结果。

独立预期与用户结果：
- VS59四谐波288点ρ=(8+2Σ(Re cos+Im sin))/1680，逐点1e-7 e/Å³、积分1e-4e；DC integral通过不接受高频。
- VS55 F100实虚按5e-7显示半末位；核2.064fm、核P4.260096fm²/电子P81e²各用已冻积分预算，仅积分角色。
- 粉末首点原0与独立3.070702272失败保留；Bragg79.83242239646°与修正79.80735918261°/采样79.81°分列。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
cargo +1.99.1 test --offline --locked --manifest-path kernel/Cargo.toml -p vesta-science --test spectrum_controls
```
预算/归属：B-GRID；coeff和FFT scratch先估算与实际预算，native external额度不继承。失败/取消恢复：缺系数/域/输入schema不注册全generator；显式caller谱/数学控制可继续。job临时结果绑定baseRevision，迟到保原base provenance且不提交。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U09 体数据的等值面、切片、线剖面与局域分析
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：volume-render, 2d, line, peak-voronoi, poly-integral, density-conversion, volume-patterson。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U01, U02, U07；合同：BOUNDED3, FS, SG, FC-auxiliary-io, FC-data2d-text-export，实际字节/SHA在contractBindings。阻断完整冻结：FS-I01, FS-D01, FS-L01, FS-K01, FS-C01, FS-F02, SG-01, SG-06；未来资格：相关目标/资源验证。
计划创建/修改（非已存在）：`kernel/crates/science/src/{isosurface,slices,projection,line,peaks,voronoi,density_conversion,patterson}.rs`、`web/src/render/{field,contours}.ts`、`web/src/editor/{slice,line,peaks}.tsx`、`kernel/crates/science/tests/field_analysis.rs`。
接口与生产消费：SelectIsoEdges/SliceTrilinear/SampleLine/FindLocalExtrema/QueryPolyIntegralCapability/ConvertDensity/VolumePatterson；FSProjectionInput/FSProjectionResult/FSLineResult/FSPeakResult/FSBasinAllocation/FSNonfiniteDiagnostic。SelectIsoEdges不是全mesh；NoAction仅锁定菜单185，无积分值。
- [ ] 先边交点、原始网格周期三线性13点线、显式权重投影及finite core直接卷积、strict局部峰和26邻点兼容转换。
- [ ] 完整拓扑/caps/seams/saddle法向依FSI01；投影sampling/世界相机依FSD01/SG01/06；line异常计数FS L01；plateau/排序/三斜basin FSK01；全密度物理域FSC01；一般Patterson Nyquist FSF02。
- [ ] 预先冻结完整数组/mesh independent expected及per-field误差；不能拿原27邻体积或DC积分替代实际三角/盆地。
- [ ] 每种结果都走对应表/图/mesh/field真实export与U02恢复，销毁原field实例后重开并改切面或线端点继续。

独立预期与用户结果：
- 线端点全13行，总长√185Å，距离1e-4Å、ρ1e-6；对端反向/周期坐标逐字段比较。
- 峰正(.25,.5,.75)=6、负(.75,0,.25)=−6只在负搜索启用；plateau不借这两峰通过。
- logabs0为诊断−Inf，不转合法finite；26邻点离散兼容与连续物理模型并存；poly-integral NoAction必须revision/history/resources不变。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
cargo +1.99.1 test --offline --locked --manifest-path kernel/Cargo.toml -p vesta-science --test field_analysis
```
预算/归属：B-GRID + B-WEB。失败/取消恢复：edge-only不能冒完整网格；未冻的拓扑/盆地不注册。取消释放临时mesh/field，GPU失败不清dirty/改变已有场；bounded线/NoAction可继续。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U10 有真实数值与可恢复取消的外算工作流
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：tidy, madel。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U01, U02；合同：BOUNDED3, EX, AB, BT，实际字节/SHA在contractBindings。阻断完整冻结：SG-02, SG-04, P05-07, P05-04；未来资格：相关目标/资源验证。
计划创建/修改（非已存在）：`kernel/crates/science/src/external/{madel,tidy,rietan,orffe}.rs`、`kernel/crates/cli/src/external/{runner,profiles}.rs`、`tests/external/qualified-tools.mjs`。
接口与生产消费：ProposeTidyResult/PrepareMadelInput只原有有界数学/候选接口；ExternalJobSpec/ExternalResult见I30§6和EX每tool闭合schema，options未冻不公开为自由JSON；进程只native side，WASM无std::process。
- [ ] 锁每artifact/runtime/ABI/data/template/许可/实际argv、完整输出语法和独立数值oracle；四producer与viewer分开。
- [ ] 先纯输入/解析与候选验证；真实外算须P05-07及相应SG/FS输入合同和数值资源额度，不重复已知MADEL pre-main缺库崩溃。
- [ ] 未来每job自有cwd/process group，直接argv不用shell；退出+新输出+独立parse+科学检查四层都通过再候选commit。
- [ ] TIDY输出原子/setting/标准化；MADEL中性NaCl独立收敛Ewald；RIETAN mode完整产物；ORFFE真covariance的JΣJᵀ含交叉项。tool viewer读synthetic FFE不能接受producer。
- [ ] 成功后同page可见→U02保存→独立读回→关页CLI继续→新page重开；cancel/timeout/stale/hash变化候选不覆盖新稿。

独立预期与用户结果：
- TIDY exit0+Stop/job或无ATOM必须失败；数据库29非科学成功。
- MADEL缺库没有0能量；退出0不保证数值；原文argv/output SHA只provenance不是oracle。
- 旧epoch/baseRevision、缺文件、非有限/坏单位、错误工具hash、ABI、权限/配额均明确拒绝，保留旧模型。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
node --test tests/external/qualified-tools.mjs
```
预算/归属：B-EXTERNAL未分配；未给每tool数值不启动。失败/取消恢复：runner TERM→有界grace→KILL→wait/reap仅自有组；具体wall/RSS/disk/子进程/egress/次数未来卡先逐工具给数值。exact scientific run argv尚缺，测试入口可计划但禁止占位执行。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U11 偏好可确认生效、持久重载且不损坏正在编辑的工程
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：应用或工程横切结果。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U01；合同：AL3-preferences, AL4-preferences-input.contract.json, AL4-preferences-input.schema.json, AL4-numeric-conversion-profile.json, AL3-history-animation, AB, EX，实际字节/SHA在contractBindings。阻断完整冻结：AL-02, P05-06, P05-07；未来资格：AL-02。
计划创建/修改（非已存在）：`web/src/editor/preferences.tsx`、`web/src/host/preferences.ts`、`kernel/crates/commands/src/preference_binding.rs`、`tests/contracts/preferences.test.mjs`、`tests/browser/preferences-effective.spec.ts`。
接口与生产消费：AL4 input字段rawText/basePreferenceVersion/domainProfileDigest/parseProfileDigest由server资格绑定；outcome parsed-scalar/parsed-binary/kept-previous/unqualified/safety-rejected。数字schema只五字段；其余textFont、三artifact、openTarget、startupBondSearch、hardwareAntialias七项走独立资格，不能宣传12字段全部wire准入。
- [ ] 先defaults观察/类型/版本与draft Begin/Set/Apply/Cancel；Apply是adapter操作非原件已证按钮；UI字体/外算profile按05授权资产。
- [ ] 资格通过的输入先parser candidate再lower typed store，再整代持久写/读回/install；显示格式与实际effective读回分离。
- [ ] AL02缺locale/FPCR/词法/溢出/field资格只停改值准入；未知E_PREF_DOMAIN_UNQUALIFIED不发生部分持久/Canvas副作用，保留draft及当前有效版。
- [ ] Apply排空旧tick/IME/捕获作业；history裁剪先报告具体资源与显式确认；未来job固定新偏好version，旧job不被改参。
- [ ] 偏好允许prefs-only浏览器持久，但不得存作品；新profile/page重开读实际偏好，工程通过U02仍同内容，继续手改。

独立预期与用户结果：
- 数值存储i32/f32/f64按server conversion profile，formatter %g/%.*f不证明安装值。
- Apply成功后Cancel仅丢新draft，已Apply版保留；冲突/写回失败有效版不变；font/artifact坏摘要拒绝且不启动进程。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
node --test tests/contracts/preferences.test.mjs && npm run test:browser -- --grep preferences-effective
```
预算/归属：B-WEB + prefs持久层实际quota/runtime前置。失败/取消恢复：未资格值不安装；已资格defaults展示/草稿/版本冲突可先做。偏好改坏不丢工程，失败保effective和未Apply文本。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U12 数据库导入、面板文本与系统动作的完整应用结果
状态：`source-bound-design-ready-runtime-unqualified`；本卡未实现/未运行。原领域：应用或工程横切结果。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U01, U02；合同：AL-base, HS4, FC-auxiliary-io, FC-save-target-policy，实际字节/SHA在contractBindings。阻断完整冻结：无新增设计输入；仍有审查/产品授权及真实实现资格；未来资格：AL-01, AL-05。
计划创建/修改（非已存在）：`web/src/editor/{database,lifecycle,panels,interaction}.tsx`、`web/src/host/actions.ts`、`tests/browser/application-lifecycle.spec.ts`。
接口与生产消费：AL database navigation/document/epoch/baseRevision/request及私有candidate/import事务；HostAction只内部认证capability与实际完成票据，不新增任意REST或OS函数；Summary是revision派生、Comment是作品、Output是UI。
- [ ] 四个COD已锁表单请求和已证CIF/404先实现映射，平台HTTPS适配提示；没有原生API/provider滤项就不造。
- [ ] 请求先candidate，明确import一次Engine事务；新稿/导航/Stop/timeout/SSL失败丢弃迟到影响但保provenance；真实传输终止需未来owned endpoint资格。
- [ ] Comment UTF8与尾LF持久；Output清除不改工程；Summary同当前revision；Output文本实际导出独立读取内容。
- [ ] About从实际manifest、Manual/Updates用批准来源不自动安装；HideSelf/ShowAll限工作台，HideOthers不越域隐藏别软件，Quit/NewWindow/Recent走U02保存与关闭屏障。
- [ ] 当前页三layout切换只布局，document/epoch/owner/jobs保持；新窗口/显式重开读取个人空间真实文件继续编辑。

独立预期与用户结果：
- COD下载/解析/导入阶段独立，失败不替旧稿；测试不能自写CIF最终作品。
- 权限/配额/登录退出/URL批准由现有服务执行；不从原件HTTP/ATS缺陷降低HTTPS；无浏览器作品恢复副本。
- Comment保存重开等字节/码点，Output清空不进入history；Summary不是旧cache。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
npm run test:browser -- --grep application-lifecycle
```
预算/归属：B-WEB；网络真实owned endpoint未来须单独wall/bytes/egress数值。失败/取消恢复：AL01真实网络cancel、AL05宿主动作和runtime未验只阻相应资格；可继续本地candidate/文本/严格capability拒绝。UI不支持的OS域报具体unavailable，不暗删原功能。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U13 全部原格式与数据、图片、mesh导出的真实内容交付
状态：`source-bound-design-ready-runtime-unqualified`；本卡未实现/未运行。原领域：exports。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U00, U01, U02；合同：FC-format-profiles, FC-codec-field-policies, FC-roundtrip-assertions, FC-full-source-audit, FC-auxiliary-io, FC-data2d-text-export, FC-resource-content-comparison, AB，实际字节/SHA在contractBindings。阻断完整冻结：无新增设计输入；仍有审查/产品授权及真实实现资格；未来资格：HS-03, P05-10。
计划创建/修改（非已存在）：`kernel/crates/codecs/src/{registry,source_capture,loss,diagnostics}.rs`、`kernel/crates/codecs/src/formats/*.rs`、`web/src/render/exports.ts`、`tests/codec/{profiles,artifact_roundtrip}.mjs`。
接口与生产消费：decode<R:ResourceReader+?Sized>(&[u8],InputFormat,&R)->Result<DecodedProject,CodecError>；PlanEncode/EncodeApproved依原签名，Source7/selected profile typed请求必须显式迁移；无自由params。每profile handler/234参数/291组选定字段规则从FC合同当前SHA取，不重新编格式白名单。
- [ ] 按真实可独立验收的输入族和输出用途组织子步骤：原生完整工程；文本晶体/分子与多模型源；binary/text field；reflection/2D/report；raster；vector/mesh。不是每扩展名一张卡。
- [ ] 逐profile实现sourceContract→selectedParameterDesign→selectedFieldDesign；85键外已有aux IO同样安排。输入只读格式必须导入当前工程后可保存重开，不能伪造该格式writer。
- [ ] 完整活动source捕获后PlanEncode→LossReport逐对象/owner/叶→绑定当前snapshot/profile/purpose/report的显式Consent→EncodeApproved；规则变动Consent失效。
- [ ] 真实候选和持久artifact-only恢复比较完整对象/数值/资源；科学资格缺失只阻相关active projection，不阻合法source存档与已经冻结的原生域。
- [ ] 14图像政策及mesh维度/单位/面/透明/背景逐一独立reader；原JP2无产物明确reference failure，不能伪造成功。所有导出不重绑工程/清dirty/下载。
- [ ] 销毁所有旧实例，新page/CLI从U02保存的工程重开，导出副本用独立外部reader看实际内容，再正式修改和二次导出。

独立预期与用户结果：
- FC85条route-specific roundtrip断言必须逐项绑定实际输入和独立reader全内容，不能只HTTP/存在/扩展名。
- m3d是原UI32位整数；PRI明确flag/ABI；CIF/PDB/CUBE/XSF显式source selection，不能默认首模型/A构象/首轨道。
- 原SGI/JP2/TIFF/STL等失败保留；corrected STL拒绝退化，删除须显式编辑；JPEG实际DQT/4:2:0不是反推quality。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
node --test tests/codec/profiles.mjs tests/codec/artifact_roundtrip.mjs
```
预算/归属：B-MATH/B-GRID/B-WEB按具体route与实际输出；234参数组合有界选择，不能声称全笛卡尔积已测。失败/取消恢复：缺格式对应科学/模板资格只停该profile步骤；保留source和原稿，E_FORMAT_LOSS需新Consent，未知实际类型拒绝保存而非source-only洗白。
真实内容往返：正式Engine/页面命令，不直接注入状态 → U02真实Save/SaveAs，U13实际export；内存工程阶段明确未Saved → 独立reader逐对象/字段/数值，U02绑定候选及实际持久内容 → dispose旧page/Worker/Engine；清内存资源和缓存，保留受管真实个人空间文件 → 新document/epoch，从实际WorkRef及依赖恢复；重开不要求旧ObjectId相等 → 用新ID选择器再正式编辑并独立读回；旧请求/旧Consent拒绝。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### U14 目标环境、正式CLI/平台运输和CA交付资格
状态：`partial-with-bounded-work`；本卡未实现/未运行。原领域：应用或工程横切结果。主责VF见本卡派生快照；其它重复提及仅依赖。
进入：U00, U01, U02；合同：BT, CA, EX, HS4, AB，实际字节/SHA在contractBindings。阻断完整冻结：P05-08, P05-09, P05-06, C6-review, runtime-authorization；未来资格：HS-01, HS-02, P05-08, P05-09, C6-review, runtime-authorization。
计划创建/修改（非已存在）：`kernel/crates/cli/src/{main,stdio}.rs`、`kernel/crates/preview/src/main.rs`、`scripts/preview.mjs`、`Build/vestacraft-preview`、`scripts/ca-prepare/`、`tests/{delivery,contracts}/`、`MECHANICS.md`、`mechanics.json`、`/Users/agilecho/code/BranchDraft/dsh/plugins/software/{catalog,startup,service}.mjs`、`/Users/agilecho/code/BranchDraft/dsh/plugins/software/client.jsx`、`/Users/agilecho/code/BranchDraft/dsh/plugins/software/client.js`。
接口与生产消费：I30§6 commands/run/inspect/serve --stdio --root为产品CLI；原16命令仅兼容来源；prepare工具公开API与C6 browser receipt/buildRoot合同按CA§6/7，不手造done。最终shared plugin与client.jsx/client.js同步由未来P4授权。
- [ ] 先实现displayless CLI同Engine/codec；roots由服务端挂载/anchored守卫强制，stdout协议/stderr诊断和0/2/3/4/70退出；页面开着则仅page engine，不能起第二写者。
- [ ] 嵌入Web实际CSP/MIME/Worker/font/WebGL2与Linux x86镜像glibc/CPU/工具锁逐目标资格；中文IME和静态16,437,364B CJK真加载、导出分别验；GPU/OOM/VS102分层测量定生产预算。
- [ ] 锁CA1f8508a1只读导出源码，再另授权的bootstrap工具new_project/Study2068文件迁入、目标kind=app、完整study manifest/actual browser archive安装receipt；生产批准表为空时不启动。
- [ ] 真实Codex worker/模型/次数/费用另预算，inbox/checks/asks id配对，Build→适用Runs→无capture runs时Screenshot→Deep Mechanics保留实际skip；故意坏产物Check失败再修。
- [ ] 源码/patch/generated bundle/dist/runtime同一摘要；真实零模型工具链、获授权真实模型任务与x86各自结果，Ready/CA相似度不接受全功能。

独立预期与用户结果：
- Linux x86无display实际CLI读写完整工程、越根/symlink/只读/507拒绝保稿；Mac/ARM证据不外推。
- Build唯一preview从本dist实际Loaded/Rendered截图；冷总耗时独立，CA120s、截图90s不修改；启动/取消只清自有PID组。
- 未执行/不适用/失败分开；C4设计接受不因未运行撤销，C6未审阻其新增生产准入。

未来验证命令（cwd `/Users/agilecho/code/VestaCraft/app`；创建真实锁/脚本/测试目标并具备本单元授权与输入后才可运行）：

```sh
npm run test:delivery -- --target linux-x86_64
```
预算/归属：BT/CA分阶段；B-EXTERNAL和真实模型预算未分配、历史次数/停报不重置。失败/取消恢复：模型/认证/额度/缺浏览器receipt停对应启动，不改后端；awaiting_check只续检查，丢session标新turn；unknown先查原op；不可重复new_project覆盖已有workspace。
真实内容往返：正式CLI/工具/真实worker按各自授权 → 同kernel真实SaveBundle→HS4实际个人空间 → 独立artifact-only全字段读回+runtime制品hash → 释放页/CLI/worker拥有的engine、进程和资源；保留原操作记录 → 同WorkRef，新epoch/newpage/重启CLI，查询未结op后再写 → 授权当前页正式手改/Agent再改；新model和x86分别证明，不用mock替代。对图像/只读输入不伪造原格式编辑器，仍从真实工程重开续改。
交付：原创实现/必要schema迁移、独立expected、真实输入/产物/运行指纹、原始日志/退出/超时、写入读回重开内容断言、资源归属和未验说明；非本轮已实现。

### 完整冻结仍缺及P1接受边界
SG六项/FS十一项/05十项及04八项原ID全部保留；精确最小输入、触发单元和可继续项在dependency-disposition.json（引用原卡原字节）。HS01/02已source-bound不再列缺全局CAS的设计总前置；HS03/AL01/03/05主要是未来真实资格。AL02完整偏好输入资格及AL04实际动画Undo分组仍为04设计输入，未用适配器政策冒充原件证据。
P1.1路线与目录接受保留；P1.3 C4设计接受、C5条件、C6未审分开；P1.2完整公共科学/应用接口仍缺，P1.4关键单元因此只能partial，本轮15单元/254主责/31领域/85路由的静态通过不等于全功能可执行。03rev48整卡自验完成待整卡独审，04rev57 F1–F6已审与rev58 R1–R3未审分开，01/02/05原partial保持。详见acceptance-arguments.md。06交回后停止，未进入P2–P5。

## P1-06 步骤准入与首链接续（rev61 / IP2）

P1-06同卡步骤准入设计交回，rev61/D30-I30-C6，待用户手动dsh。已区分源码、构建、运行资格与完整结果，首个Web保存导出重开续改方案明确；CLI同生命周期身份绑定缺口单独停受影响路径。SG六/FS11/AL02与AL04等原缺口保持，P1两接受两未接受、整体partial；未实现产品或执行运行。

本节继任IP1的依赖解释，不新发任务卡、不执行产品步骤、不扩大公共wire。原15用户/工程结果和254主责、31领域/85格式及额外IO留原真实合同；每个结果内部按源码、输入资格、构建制品、运行资格、完整用户结果进入。当前全部执行权限仍false。

[精确步骤图](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-063853-step-admission/step-plan.json)列实际消费合同/版本/SHA、文件/接口/输入、独立预期、cwd/未来命令、预算和取消/恢复；[逐步说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-063853-step-admission/step-admission.md)及[实施就绪](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-063853-step-admission/implementation-readiness.json)为当前接手入口。

| 接手结果 | 本次真实顺序与理由 |
|---|---|
| 基础内核与当前页 | U00.scaffold→U01.model→U01.engine→U06.basic-scene/U01.page；先源码后完整锁/真实build，源码不等browser/host/CA资格 |
| 首链混合构建 | U00.lock-inputs + page/save-state/CLI/preview具体源码→U00.build；制品必须来自已实现首链，不能空脚本或不完整包报成功 |
| 目标环境与交付 | U00.build→U14.web-qualification和linux-qualification→U00.delivery；不再以U14整单元完成作为U00源码/构建前置 |
| 保存与恢复 | codec/source visitor/save-state source→host-source→真实Linux/registry/路径配额资格；替身状态机不报Saved；只有完整当前持久类型捕获与独立artifact读回通过才发布确认 |
| 首个独立Web用户结果 | U01.current-page + U02.host-qualification + U13.first-codec→U02.web-close-reopen（R00）；actualSave/export/destroy/reopen/edit，值与dirty/WorkRef见casebook |
| CLI延伸 | U14.cli source可继续；同进程load/inspect/execute/save绑定未冻时U14.native-stateful-binding与U02.close-reopen（R01）停止。不能把独立inspect的ID塞给下一run进程 |
| 高级域/其它格式/偏好/应用 | 各bounded-source只消费实际model/Engine/field/codec源码；实际算法/codec/UI验证需freshbuild与独立预期；完整结果仍挡在原具名SG/FS/P05/AL及CLI组合门，不拿有限结果升级整卡 |
| CA真实资格 | CAprepare/preview可按既定来源写未来source；worker/Check/launcher/C6及模型权限为终端，不反向阻断普通首链源码/页面资格 |

[首链接手说明](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-063853-step-admission/first-chain-handoff.md)不是已发放施工卡。R00在同Web做4→5→6、Undo/Redo/noop，旧snapshot6的真实宿主ack不能清a7的dirty；保存a7后改a8导出CIF时main仍a7，导出不重绑/清dirty；最后真实Save a8/128、关页销毁、新Web读a8再手改a9/144并实际存读。模型、文件精度及epoch/ID分开断言，driver不得代写最终作品或伪造ack。

[P106-CLI-BINDING](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-063853-step-admission/composition-gap.json)是新发现的本卡接口组合缺口：I30规定每次载入换document/epoch且ObjectId只在生命周期内稳定，§6的one-shot inspect/run没有提供同Engine消息/资源/选择器衔接。停受影响步骤，完整CLI要求保留，未新造publictype或修改I30；R01理想a9/144→Weba10/160链只保留为待合同后的验收。首个R00 Web用户结果不是全CLI/全功能接受。

本轮WordCraft实际参考Session::run、显式snapshot coverage和async_saves身份/完成分离（真实路径ui-egui），HEAD/工作区SHA见wordcraft-reference；不迁移其浏览器下载、localprofile或文稿命令。D30只补依赖说明，I30/C6原字节、C4接受/C5条件/C6未审及所有原卡边界不变。


## P1-06 CLI续作契约与步骤后继（rev63 / IP3）

沿用15结果、56步骤；[step-plan后继](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/step-plan.json)只把U14.native-stateful-binding从缺设计输入变为有精确NS/1输入的未来source步骤，并更新U14.cli与U02.close-reopen消费。完整结果仍须源码实现、准确输入构建、受限环境资格和真实内容链，当前无执行授权。

未来源目标沿原crate布局：kernel/crates/cli/src/{stdio,args}.rs、bindings/native_session.rs及tests/contracts；host桥接沿既有BranchDraft/dsh/plugins/software/service/catalog/startup与client.jsx/client.js，具体分阶段授权，不能在app另造认证/持久journal服务。当前第一独立Web结果R00保持；CLI后继R01按[身份绑定链](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/identity-trace.json)在一个进程内load/inspect/9Å续改，实际host保存后销毁，再Web新load 9Å→10Å。无代码/编译/运行证据。


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

IP4保留 U00–U14 的完整用户/工程结果和56步骤ID，逐项替换旧“有界数学原语即全部功能”的交接：U03实际D3..6/OD/群与张量，U08完整反射/Fourier/模型场/粉末，U09完整网格/斜切片/厚度/等高线/线/局域分析，U06最终表面颜色/排序/截面消费，U10四个EX3工作流，U11十二偏好及可信历史策略。[15单元详细安排](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/implementation-readiness.json)和[按结果依赖的56步骤](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/step-plan.json)给出实际输入版本、计划路径/命令、预期、失败恢复与未授权边界。各运行步骤必须核实际编译版本、目标与数值预算；不能用首链成功认证后续源改动。[完整结果验收](/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/verification-plan.md)要求真实写入、独立读取、销毁重开及继续编辑，完整数组/网格/字段，而非计数、HTTP成功、文件存在或自读。

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
