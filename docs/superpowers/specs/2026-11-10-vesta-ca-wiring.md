# VESTA P1 Craft Anything 执行与 Check 接线

2026-11-14，P1-C6。2026-11-14 R4：P1.1继承接受，P1.3 C4设计级接受保持，2接受/2未接受；P1.2、P1.4和整体P1仍partial。真实dsh R3结果记C5增量有条件通过，N3/N6及证据保真残留在本轮D6/I8/C6中整改、待用户手动转交dsh复核。真实worker／Check／launcher仍未授权、未验证。资料基线rev29；仍不进入P2–P5、产品实现、部署或模型builder。

## 1. 已核事实和角色

- CLI固定0.4.0，绝对路径`/Users/agilecho/code/Craft/output/tooling/craft-anything-1f8508a1/debug/craft-anything-cli`，SHA256 `99ad8c99cc5d7b35dfd85eb9b1a126ab7c074f3a5adbc7453b7a3d457d4a8869`。本轮真实rebuild/check help及wrapper在R/p1-ca-*-help.*，只读帮助不生成产品。
- 对`rebuild`命令，CLI顶层`--project`是调查.cany，`rebuild --project-folder`才是workspace；`--continue-in`打开有craft.json的workspace；`--resume`忽略request并续作，不能拿vesta.cany作continue-in。独立命令`rebuild-check --project`则绑定workspace，见§3；不能把两者参数语义混用。
- Codex adapter实际构造`codex exec --json --skip-git-repo-check`，builder加workspace-write，MCP按临时`-c`注入当前CA CLI和inbox参数，`--all-tools`；prompt stdin；保存/恢复Codex session。`--skip-git-repo-check`允许非Git原生workspace，不能借此初始化Craft Git。当前`codex exec --help`只证这些基础flag存在，不证模型/认证/额度或与CA全协议已联调。
- **CA Worker**负责编排Study/Build/Check、泵外部builder事件、inbox/自查、原生checks/claims/unknown/轮次与恢复。**Codex CLI builder**只在app/实现和测试，研究材料只读，维护mechanics来源。**本会话Codex**仅P1设计/有界补证/文档自验；不启动未来builder。**dsh**只读独立复审，不充当builder或worker。
- 无空闲Check/无离线Check替代；没有本产品Ready。S06 research/MECHANICS.md、mechanics.json是未来迁移输入，尚未进入app也未原生检查。

## 2. workspace及Study迁入设计

计划workspace根`/Users/agilecho/code/VestaCraft`，产品根`app/`，准确职责见架构§2.3。CA拥有craft.json/investigation.cany/capture/checks/inbox；Codex不手改craft.json伪造worker状态，不覆盖E/vesta.cany。

不能直接执行fresh `rebuild`再假称自动消费既有Study：`mod.rs::Worker` first判定会进原生study；本项目Study是工具级逐步研究，不是已经跑过原生Study worker。为了不重跑全Study，提出下列**未来bootstrap适配任务（尚未实现/验证）**：

1. 独立prepare工具调用锁定公开`rebuild::project::new_project(&Settings,target,request,Place::At(root,None),Some("React + TypeScript (Vite)"),false,Some("Codex"))`，只创建原生workspace，**不调用RebuildRun::start**。Settings来自当卡隔离目录，不读取/打印供应商秘密；正常后续builder环境另按真实启动协议管理，不滥用CA研究Key过滤。
2. 在worker未启动期间，将正式调查目录复制为workspace/investigation.cany的**派生副本**，保留ID和revision；预核symlink/目录边界，生成逐文件SHA复制清单；E正式源保持字节。不能拿不完整raw export或roundtrip.cany替代。验证2037派生索引和24unknown当前头，显式64MiB/not_loaded边界。
3. 将真实已存在的研究截图/摘要与绑定文档复制到capture/reference，保存来源SHA。`capture/window.png`只能来自有效原件图（现有E/s04/startup.png可作为启动态候选），树/样本应按实际映射核，不把无效About图/手写UI树当原件。该启动态不代表所有领域状态。
4. 将S06研究mechanics迁入app/MECHANICS.md与mechanics.json，逐条核锁定格式/source/constant绑定；保留研究原件不动。尚无app时不宣称已迁入或mechanics检查通过。
5. 自验prepare工具没有模型调用和产品实现，原生manifest仍真实初始状态；capture存在且以resume续作时，`mod.rs` first分支跳过Study的依据是源码，不是当前运行事实。未来bootstrap运行资格必须由真实新workspace最小联调验证；未通过时不得把工具/迁入/worker接通记为运行成功。P1.3设计级已接受，运行资格单列，不撤回或重新阻塞设计项；不能手改历史done字段绕过。

prepare 工具仍未实施；本轮 §6 补出其未来构建、输入和制品合同，实际复制和原生联调仍未进行。下面命令是**未来启动协议**，不是当前可执行动作。不得为补这条证据在本 P1 启动产品 builder。

## 3. 启动、队列及未来调用

只有用户以后授权产品施工/原生builder和相关资源后，且§2 prepare通过、后端名字真实存在/启用、预算/模型/权限确定，才可使用。`--backend Codex`必须匹配真实配置名，不能用本机存在codex推断Settings已配置。当前E/config路径传入不等于Settings落盘。

未来示例（**未执行；cwd为workspace根；需真实配置、prepare与实施授权**）：

```sh
CRAFT_ANYTHING_CONFIG_DIR=<当卡已核隔离配置绝对路径> /Users/agilecho/code/Craft/output/tooling/craft-anything-1f8508a1/debug/craft-anything-cli --deep rebuild /Users/agilecho/Documents/VESTA/VESTA.app "续作已授权模块；全部范围和D02以reference文档为准" --continue-in /Users/agilecho/code/VestaCraft --resume --backend Codex --rounds 0
```

这里`rounds 0`是建议首次接线不自动追加修正轮的**未来操作参数**，不是用户已定最终资源预算；仍会有首次Build/Check/模型工作，绝不能因0称零模型。实际执行卡应明确次数、超时、费用与人工停点。正常后续修改去掉resume并用真实follow-up request；stack只允许新项目选择，continue不能附--build-with。

worker持有真实RebuildRun，`converse_turn`约120ms泵builder事件/relay_inbox/answer_self_checks；inbox目录承载用户问题/审批/notes。Check的请求目录单独是`<workspace>/checks/asks/`，不是把一个文件扔入普通inbox就算检查。

外部Codex CLI构建期间，需要即时检查时，通过其注入的MCP或锁定CLI调用真实schema：

```sh
/Users/agilecho/code/Craft/output/tooling/craft-anything-1f8508a1/debug/craft-anything-cli rebuild-check --project /Users/agilecho/code/VestaCraft --wait-minutes 20
```

前置：worker仍在构建/泵请求，app已有真实build和适用入口，当前项目/轮次身份明确。`CheckParams.project`可显式或从工具server的InboxDir父目录推导；wait_minutes1..60/default20。工具写Question请求到checks/asks，worker用同一组checks执行round0并answer，工具再返回report；round0不计正式修正轮。工具超时可能是没worker或检查过久，不是内容失败/通过；先查worker/running、request ID、asks实际回答和checks产物。

**同进程API builder**：BuildingHere检测engine锁，返回FeatureMissing“不能在构建中检查自己”；正确路径是结束当前turn让原生worker随后Check，不能在同锁中起另一个等待、不删guard/伪造answer。用户选Codex CLI外进程可避免该特定锁问题，但不等于已接线成功。

## 4. 混合Web/Rust构建与AppKind检查

CA07默认AppKind goal是desktop/core、默认egui，本项目由明确request/reference覆盖成全功能Web；但锁定kind/check代码仍有具体约束：

- `AppKind::checks`＝BuildCheck、适用RunsCheck、ScreenshotCheck、DeepOnly(MechanicsCheck)。Runs是否适用来自真实capture/runs，不造空runs以跳过截图。
- `build_check`先看app/Cargo.toml，再package.json。设计root仅package.json，Rust workspace在kernel/；root `npm run build`必须真实调用Rust native/WASM、同版wasm-bindgen、TypeScript和Vite，并产出app/dist。不能让cargo成功遗漏Web。CA普通app构建默认2分钟期限，首次冷构建可能超出：必须真实保留失败，不通过改源码/假日志绕过；未来执行卡先允许独立真实冷构建再Check，仍另报冷/增量耗时。
- Node工具链无launch recipe。`engine/toolchains.toml`给Python `main.py`、`app.py`、`__main__.py`独立入口；`outputs::launch`只对found_in选中的toolchain调launch；有package.json就识别Node，不回落Python。P1-PROBE-02链接锁定engine rlib核实：main.py被忽略，唯一Build无扩展可执行候选被选中（候选未执行）。因此不采用main.py自动入口，计划打包唯一`app/Build/vestacraft-preview`；CLI放`dist/cli/vestacraft`，Build内无第二候选，避免packaged_program按mtime选错。
- `Build/vestacraft-preview --screenshot <绝对png>`计划启动本次dist本地静态server＋受控浏览器，等当前真实页面Loaded/Rendered后捕获，输出PNG并清理拥有进程；不用旧截图/预制图、不得绕开正式页面状态。正常无参数启动也应显示同一Web应用；AppKind的原生窗口/AX对照与DOM/Canvas不同，必须如实保留无法映射和缺失claims，不能伪造UI树。`ScreenshotCheck`90秒、非空图/可解码只是原生局部检查，不替代Web语义或平台整链。
- Mechanics检查的字面常量/来源完整性不证明科学真值；仍用S06_ORACLES和独立输出解析；原生Check没有覆盖的科学/平台/格式结果另归实际测试证据。

这里尚有真实launcher进程/浏览器/AX、capture/调查派生迁入、Web启动与原生AX差异、cold build期限等接线待证。它们不是泛泛“后续完善”，而是G-CA的明确接受项，G-CA真实运行资格不能由帮助文字认定；P1.3设计级接受依据用户带回的dsh结论及DEC-P1-03。

## 5. 失败、恢复和验收证据

- builder启动/认证/额度/程序不存在：agent Error→不冒称进行了build/check；保留实际stderr、退出、耗时和非秘密session绑定。修复前置后续作，不改为默认其他后端/dsh。
- builder退出但`awaiting_check=true`：原生resume应只继续未完成检查；读manifest、当前进程和实际制品，不能按日志“准备检查”推断已做。
- stop/超时：先原生stop/拥有进程组清理，保留manifest/feed/checks/asks和未结任务；检查实际写入/committed状态再重试，不盲重发副作用操作。
- 丢session：使用manifest记录的当前builder session/agent_session_by及per-agent session；lost-session分支按锁定实现重新起turn并带brief，记录它是新turn。不得回退调查revision或把旧round改成新结果。
- Check失败：消费具体checks/claims/unknown/artifacts，Codex修差异后再Build/Check；未通过的源码/日志留每轮新目录；到迭代上限Ready也不是本项目全部功能完成。
- 普通数学/codec/页面/保存测试在无worker时可以按产品测试合同运行，但名称是这些测试，不能称rebuild_check。dsh复审也不等于CA原生Check。

后续G-CA运行验证接受需有：prepare源码/制品和逐文件迁入核对、真实Codex worker/PID/backend身份、不泄密的argv/cwd/env政策、inbox与checks/asks请求/回答绑定、实际AppKind checks/round0与轮次、截图来自当前真实Web、mechanics真实结果、故意失败反例、可恢复停止/续作。当前只核源码/help和设计；以上未运行。本阶段不以空Check、产品builder或额外模型冒烟制造“验证”。

## 6 C3 的精确桥接设计合同

本节只设计，证据目录 [R1](../../../output/vesta-study/20261108-113622/reviews/20261114-100546-p1-design-continuation/)。原始相邻源码在 R0/method-snapshot；本轮新增读取锁定对象中的 Cargo.toml／Cargo.lock、engine/ai 的 Cargo.toml、engine 的 settings.rs／lib.rs，保存于 R1/source-supplement，摘要见 source-supplement.json。没有编译、安装、创建原生项目或调用模型。

### 6.1 prepare 的未来位置和构建输入

未来工具设计名为 `p1-ca-bootstrap`。源码放在**当次另获实施授权的新证据目录**所持有的 `ca-method-src/crates/p1-ca-bootstrap/{Cargo.toml,src/main.rs,src/{plan,copy,verify}.rs}`；制品复制至该轮 `tools/p1-ca-bootstrap`。它属于引导工具，不是未来 VestaCraft/app 的产品功能，也不放入共享 CA 活跃源码。

构建时先从指定 CA repo 按锁定提交 `git archive` 导出完整、独立的源码树至 ca-method-src，逐文件核 git 对象；不得将当前活动工作树直接复制作为锁定源。锁定根 Cargo.toml 已有 `members=["crates/*","apps/*","xtask"]`，新增 crates/p1-ca-bootstrap 自动成为 member，**不需要改原根 Cargo.toml**。工具包使用 version.workspace=true（0.4.0）、edition.workspace=true（2024）、publish=false；dependencies 仅继承 workspace 的 `craft-anything-ai`、`craft-anything-engine`、serde／serde_json／sha2，不新选 registry 版本。保留原 Cargo.lock 的逐包 name/version/source/checksum；工具引入的唯一新包是 p1-ca-bootstrap，不允许旧节点或依赖边静默变化。

首次锁派生图时运行 `cargo +1.99.1 metadata --offline --format-version 1 --manifest-path <staged>/Cargo.toml`，保留前后锁 diff，再由独立脚本核上述约束；缺缓存即失败并报告，不能本卡安装或改为 latest。形成已核派生锁后，未来构建合同为 `cargo +1.99.1 build --offline --locked --manifest-path <staged>/Cargo.toml -p p1-ca-bootstrap --target-dir <round>/build-cache`。记录 rustc/cargo、所有锁摘要、源码 tree 摘要、制品 SHA 和平台；只读原工具和源 repo 均不改字节。本轮只是给出这套合同，没有生成 Cargo.lock 或运行命令。

工具自身的未来函数：`read_plan(path:&Path)->Result<BootstrapPlan,BootstrapError>`；`preflight(plan:&BootstrapPlan)->Result<VerifiedInputs,BootstrapError>`；`create(plan:&BootstrapPlan,inputs:&VerifiedInputs)->Result<Workspace,BootstrapError>`；`copy_study(ws:&Workspace,inputs:&VerifiedInputs)->Result<CopyReceipt,BootstrapError>`；`verify_prepared(ws:&Workspace,receipt:&CopyReceipt)->Result<PreparedReceipt,BootstrapError>`。错误只含逻辑路径／摘要／阶段，不回显配置原文或凭据。

### 6.2 闭合输入和公开 API

`BootstrapPlan` 在未来 `bootstrap-plan.json` 中固定：`schema:'vesta-bootstrap/1'`、workspaceRoot、configFile、targetPath、targetSha256、methodCommit、stack、builder、requestFile、requestSha256、studyRoot、studyManifestSha256、captureEntries、referenceEntries、mechanicsEntries。所有路径为该轮显式绝对路径，copy entry 为 `{source,relativeDestination,sha256,bytes}`；无 shell 或可执行表达式。configFile 的内容不写进 plan 或证据，只记录配置版本、Deep、后端名字／启用状态和凭据是否具备，不记录秘密摘要。

固定未来 workspaceRoot=`/Users/agilecho/code/VestaCraft`，stack=`React + TypeScript (Vite)`，builder=`Codex`，targetPath=`/Users/agilecho/Documents/VESTA/VESTA.app`；目标主二进制另核需求里的 SHA。根必须尚不存在，任何已有 craft.json、文件、symlink 或活跃 worker 均拒绝自动复用。恢复不能误走 create。

未来 requestFile 的初始精确正文为：

```text
Rebuild VESTA with all recorded capabilities; follow reference/requirements.md and reference/p1-design.md. Use the pinned architecture and preserve all recorded limits.
```

这里省去会触发 WebsiteKind 的 website／网页／网站措辞只是固定 kind 的技术输入；完整 Web 产品要求在 reference 文档中，不能删掉。`KindRegistry::default().detect(request,&target_ref,stack)` 必须实际返回 id=app，否则 prepare 拒绝。此源码级推演不是已经运行 registry 的结果；未来测试需同一请求正例和追加 `website` 后应改 kind 的反例。

配置读取使用公开 `craft_anything_engine::settings::read_settings(&std::fs::read_to_string(configFile)?) -> Result<Settings,String>`，显式检查解析、settings version和deep_mode；generate_assets=false是后续new_project参数及manifest核验，见§7.3，不是Settings字段。具名 builder 用 `AiConfig::from_table(&settings.ai)->Result<AiConfig,ToolError>` 再 `config.get("Codex")` 检查 config.enabled 和 AgentCli 类型；缺失／禁用／错误类型即拒绝，不能使用会吞解析错误的 AiConfig::from_settings。不用 `Settings::load_from` 的缺失／无效配置回退默认行为掩盖错误，不创建 AiHub，不 detect/add backend，不发模型请求；解析失败仅报无秘密错误码，不回显含配置正文的 parser 原文。该公开配置类型来自本轮追加锁定读取的 ai/config.rs，已保存 SHA。

唯一创建调用仍是锁定公开 `rebuild::project::new_project(&settings,target,request,Place::At(workspace_root,None),Some(stack),false,Some("Codex"))`。Place::At 会打开已存在 manifest，因此前述“目录必须不存在”要在调用前单独强制。不得调用 RebuildRun::start 或手工修改 craft.json 的 step/status/done；Workspace 初始清单由 API 自行生成，再只读核 manifest stack、builder、target、app_in_root=false 和未启动状态。

### 6.3 调查和 capture 的迁入顺序

prepare 由 plan→verified-inputs→workspace-created→study-copied→assets-copied→prepared 递进；每步在本轮引导证据写 receipt，不手造原生 worker 完成状态。copy 前全部源 SHA 必须与进入条件一致；运行中的正式调查不得复制，需先确认没有正在追加的写者，并在复制前后核完整清单。

1. 正式源为当前 Draft 的 `E/vesta.cany`。按实际目录逐普通文件复制至 staging，拒绝 symlink／special file／越根；核每个相对路径、字节数、SHA 后再安装为新 workspace/investigation.cany。锁定 create_at 只保证写 manifest，不会预建调查，目的通常尚不存在；若发现已存在目录，先清点，只允许替换本轮自己刚创建且仍为空的目录。有额外记录立即停止，不能覆盖用户资料。
2. 2037 条 evidence ID 和 24 个 unknown 当前头／revision 都从源逐文件解析核对，副本必须相同。64 MiB/not_loaded 只影响原生读取预算，不能用不完整 raw export 建新源，也不能删记录规避。原始路径字段保持历史字节；另生成 `reference/locator-map.json`，以源相对路径／SHA绑定当前定位和派生副本，不向原生记录灌新路径。需读取历史外部产物时按该表核实际字节，缺产物照实缺失。
3. `E/s04/startup.png`→capture/window.png；`E/s04/startup-ui.json`→capture/ui-tree.json。前者是有效 1720×1456 原件启动图，后者是实际 344 节点树；分别核 SHA 和 JSON nodes/role/title 形状，不生成手写 AX 树。无效 About 图不迁入作为有效参考。caption／reference 记录此图只证明启动态。
4. reference/requirements.md、p1-design.md、interfaces.md、ca-wiring.md、module-plan.md、study-report.md 分别复制当前已接受或明确待审的文档版本并标状态／SHA。内容决定产品需求，不把 plan 的简短 request 当全部范围。
5. S06 的 research/MECHANICS.md 和 mechanics.json 派生复制到 app/ 对应文件；原文件保持，全部 source／常量引用按锁定 schema 检查。它是准备输入，不宣告 MechanicsCheck 成功，也不能把 literal 常量出现当科学证明。
6. prepared receipt 核 workspace 的真实路径、manifest SHA、所有 copy 摘要、kind 检查结果、无新模型调用；将 capture 和 app 输入全部就绪后才允许未来人工明确启动 worker。源或复制输入发生变化必须使 receipt 失效，不能仅见 capture 目录存在就当迁入正确。

中断后只允许 `verify_prepared` 检查本轮 workspace 和 receipt，缺哪一步就重做**本轮自己的未提交复制步骤**；目录含未知写入／worker 状态时停止。不要重复 new_project，也不要自动删除可能含产品的 app/。具体 rename／目录替换与异常清理须在未来工具测试中证实，本轮不实施。

### 6.4 launcher 的进程与页面合同

未来唯一 Build/vestacraft-preview 是 Rust `kernel/crates/preview` 编译的启动桥，消费同一 build-manifest.json 和 dist；它调用产品 `scripts/preview.mjs`，由后者控制锁定浏览器。正常模式启动 loopback 静态服务和一个可见浏览器窗口，持续到用户退出；headless 截图模式只在明确 --screenshot 下使用，不能让 AppKind 的 normal capture 拍到终端或空桌面。

`--screenshot <absolute.png>` 模式：验证目标是本轮获准位置、产物 SHA／浏览器 revision→启动 loopback 无目录列举服务器→打开实际 dist→等 Loaded 和 Rendered 同 document/epoch/revision、字体 ready→截图到临时 PNG→核解码／尺寸和状态指纹→rename 到请求路径→释放自己的页／server／browser／子进程组→退出。不能用预制图、sleep 或固定约 120 帧代替就绪；CA 的 90 秒限制仍原样，超时清理并失败。

正常退出码 0；输入／路径 2；产物／浏览器／就绪／截图失败 3；取消 4。stdout 输出一条 JSON `{mode,buildDigest,pageIdentity,screenshot?,sha256?,exitStage}`，stderr 为无秘密诊断。只有自己的 PID／process group 可清理。截图来自当前 Web 并不能保证 native AX label 与原件对应；AppKind `running_note` 读 capture/ui-tree.json 比较的缺失项必须保留，不能伪造浏览器 AX 或屏蔽原生检查。

### 6.5 worker 请求 身份和恢复

prepared receipt 和后续模型／资源授权都具备后，未来才可运行 §3 的启动命令；当前 CLI 的绝对定位改用 Draft 下已核同 SHA 制品，不能照抄失效 Craft 路径。每次启动绑定 workspaceRoot／manifest SHA／backend 名／session／当前 round；--resume 使 message=None、continuing=true，且 capture 存在时源码 first=false。它只是跳过重复 Study 的程序分支，不证明原 Study worker 已跑过。

未来每次 `rebuild-check --project <workspace> --wait-minutes 20` 绑定唯一 checks/asks request id；worker 泵并在 checks/round-0 返回同 request 的 Answer。普通 inbox note 不是自查请求。检查序列保留实际适用条件和失败即停止后续的规则：BuildCheck→适用 RunsCheck→无 captured runs 时 ScreenshotCheck→Deep MechanicsCheck；被跳过没有结果，不能报告“全四项通过”。同进程 API builder 触发 FeatureMissing 时结束该 turn，等待 worker 的后续检查；不可删除 BuildingHere guard。

| 故障 | 未来动作和禁止项 | 必须保留的证据 |
|---|---|---|
| 未配置／禁用 Codex、认证或额度失败 | 停在 backend 前置，不换后端，不声称 Build 已开始 | 无秘密错误、backend 名、进程退出、累计费用／次数 |
| builder 正常结束但 awaiting_check=true | 读取真实 manifest 与存活状态，--resume 只续检查；不先重发编辑 | session／round／manifest 前后 SHA、checks 产物 |
| check 请求超时 | 查同 request 的 asks/answer 和 worker；无答案记未完成，不标内容失败或重投副作用 | request id、到期时间、worker PID、实际 answer 文件 |
| session 丢失 | 只用锁定实现的恢复／新 turn 分支，标新 turn；不改旧 session 或调查 revision | 原失败、恢复分支、实际新 session 和 prompt 来源 |
| stop／崩溃 | 请求 stop、清理拥有进程，核最后提交制品及 manifest；恢复前查询未结写入 | PID／signal／耗时、未结 asks、保存 SHA 与 orphan 清单 |
| prepare 副本或制品被改 | receipt 失效，停启动；不覆盖源调查或伪补 done | 具体文件的 expected／actual SHA 和原 receipt |

未来联调必须让当前产品产生一份可解码非空截图、真实 build/mechanics 结果，并故意破坏本轮制品使相应 Check 失败，再恢复并跑真实检查；每轮新证据保留。不把 Ready 或原生 advisory similarity 当全功能接受。

### 6.6 本卡可核与尚待验证

本轮能核：公开函数和锁定来源、构建派生锁约束、精确 request/target/stack/kind 设计、迁入文件与身份、launcher 进程／就绪合同、checks/asks 和恢复序列。输入、拷贝、kind、锁和页面合同都有明确的未来失败反例，不再只写“后续完善”。

本轮不能核：未来工具是否编译、new_project／复制／kind 实际结果、worker/backend/round0、当前 Web 截图和 AX、冷构建／目标环境及故意失败恢复。上述不是已完成事实。P1.3的C4设计已由用户带回的dsh复审判为可独立接手，用户授权登记设计级接受；C5增量待复核，实际联调仍须另行授权。

## 7 dsh F9–F12 的摘要和构建合同整改 C4

本节按用户DEC-P1-03固定P1.3的设计级验收口径。P1.3要求下一实施者能依据真实锁定系统、明确输入与失败恢复独立接手；本卡无需也不得为了勾选而启动new_project/copy/worker/Check/launcher。真实运行、故意失败和恢复仍是后续须单独授权的有界实施验证，不计为本轮已通过。本次已有dsh C4设计级接受由用户带回并明确授权回写，P1.3复选框按设计项更新；C5新增设计/文书修正另送增量复核，不把当前增量说成已审。

### 7.1 targetSha256 的唯一对象

`targetPath`是 `/Users/agilecho/Documents/VESTA/VESTA.app`，是目录。`targetExecutableRelativePath`固定为 `Contents/MacOS/VESTA`。`targetSha256`仅指这一个普通文件的完整原始字节SHA256，固定期望为 `b4835068478ec1852f732c93ab25768f2b8af0166a92fc68dfd85704fe71c9e9`；不是目录摘要、不是libVESTA摘要、也不是mtime或代码段摘要。读取前拒绝路径越根和符号链接；另按原基线核 `Contents/MacOS/libVESTA.dylib` 的 `c2c677facf6c67783cc7bbe64e92b75beb1a567fc373eacabee58b11fb81a77f`，不能仅主程序不变就把换过库的目标当同一科学基线。

明确来源为D01《VESTA全功能重写要求》§2的主程序/库SHA、STUDY-REPORT §1和原S01 baseline.json identities。D01当前§2实际载有这两个摘要；复审中“D01没有SHA”的来源描述与当前正文不符，摘要对象未定义的问题本身成立，本节予以解决。这里不声称已对整.app或所有外算/资源建立树身份；那部分仍按原研究范围和未来资产资格处理。

R2/contracts/bootstrap-identities.json记录期望与本轮仅文件读取所得主程序SHA；它没有启动VESTA。未来prepare的目标前置必须比较指定文件实际SHA，不能对.app目录调用普通文件hasher，不能把lib摘要填进targetSha256。

### 7.2 studyManifestSha256 的规范字节

正式调查没有一个可直接当清单的原生manifest。新字段 `studyTreeManifestPath` 指向prepare输入中的外置冻结清单；清单不放进vesta.cany，不修改project.toml或原生记录。其对象是vesta.cany下**全部普通文件**，包括project.toml、evidence、unknowns、journal、annotations，排除目录本身，不排除任何普通文件；symlink/special file/重复相对名/不合法UTF8/NUL/反斜杠/`.`/`..`路径一律拒绝。路径采用相对POSIX文本，不大小写转换、不Unicode正规化，按原UTF8字节序排序。

规范JSON固定为 `{files:[{bytes:<u64>,path:<string>,sha256:<64lowerhex>},...],schema:'vesta-study-tree/1'}`：对象键按ASCII字典序，UTF8编码，字符串按JSON最小必要转义，ensure_ascii=false，无空格、BOM或结尾换行。`studyManifestSha256 = SHA256(这些精确字节)`，不是project.toml摘要或目录名摘要。未来Rust实现可用按该键顺序声明的结构体/显式有序map序列化；必须与冻结字节逐一比较，不能只比较解析后的无序对象。

本轮实际清单在R2/contracts/study-tree.canonical.json，摘要为 `0b261acf9848b87db34e404ed7ebef48ff8b8857515987634e86fe0da48463ba`。BootstrapPlan同时固定 `expectedFiles:2068`、`expectedEvidence:2037`、`expectedUnknowns:24`、`expectedKind:'app'`。清单中每个原记录文件的SHA不变，计数只是补充校验，不能用数量替代内容。

preflight先核冻结清单自身SHA，再读取当前源生成候选规范字节与之比较，解析并核ID/unknown头和revision。copy后再对副本做同样核验；源变化即使条数相同也拒绝，不能把新生成的清单同时充当未独立绑定的“预期”。未来必须获得另行授权才能复制；本轮只生成只读清单和反例。

### 7.3 精确Cargo草案和公开性证据

R2/contracts/bootstrap-Cargo.toml是设计草案，没有建crate。package version/edition/license继承锁定workspace，publish=false；五个依赖逐行使用 `workspace=true, default-features=true, features=[]`。这会继承根workspace已有serde derive/rc等特性，`features=[]`不清除它们；不主动增加wasm-plugins或其它可选特性。不改原根Cargo.toml，不新选registry版本，仍按§6.1的派生锁只新增一个本地包规则核验。

本轮追加锁定Git对象 `crates/ai/src/lib.rs` 到R2/source-supplement，明确 `pub mod rebuild`、`pub use config::AiConfig`；与R1已绑的rebuild/mod.rs公开project以及project.rs公开new_project构成可复查公开路径。没有链接或编译库，源码公开性不冒充可运行制品通过。

`generate_assets=false`是new_project参数及原生manifest/person字段，不是Settings字段：未来应调用 `new_project(...,false,Some("Codex"))`，随后只读核manifest.generate_assets=false。Settings阶段只核配置版本/deep_mode/后端；不为核配置创建AiHub或执行模型。

### 7.4 浏览器输入、buildRoot与安装receipt（C6）

本节修正C5新增的receipt/buildRoot定义缺口；C4设计接受保持，C5为真实dsh R3有条件通过，当前C6整改待复核。精确字段及封闭schema见R4/contracts/browser-input-contract.json、browser-install-receipt.schema.json；本节不是已实现的prepare/launcher。

**信任输入。** 未来另获授权的任务在其证据目录`inputs/browser-launch-plan.json`保存BrowserLaunchPlan1，并由buildRoot之外的不可变任务input manifest固定其规范字节SHA。当前活跃任务提供authorizationId和该SHA，不能从页面/候选manifest/receipt自报值取得。schema固定为vesta-browser-launch-plan/1，字段仅authorizationId、buildRoot、sourceRegistry、installReceipt及schema；sourceRegistry/installReceipt各为{relativePath,sha256}。不存在活跃授权或固定摘要即失败，本卡未提供运行授权。

**buildRoot身份与阶段。** BuildRootRef1={canonicalAbsolutePath,buildId,mainBuildManifestSha256,browserManifestRelativePath,browserManifestSha256}；buildId为32小写hex，SHA为64小写hex。buildRoot是该授权任务创建并拥有的一次构建输出目录；构建/安装阶段可写，完成并生成外部输入清单后，在启动核验阶段只读。不能“在只读root内安装”。复用未知根、符号链接/特殊节点或canonical路径不等于固定绝对路径均拒绝。真实主`build-manifest.json`的全部原始字节SHA必须等于外部pin，buildId必须相同，并包含`browserBindingManifest:{relativePath,sha256}`与固定BrowserBindingManifest1引用一致。

**有类型的浏览器输入。** browserManifestRelativePath固定`browser/browser-input-manifest.json`，内容BrowserBindingManifest1={schema:'vesta-browser-input-manifest/1',buildId,browserArtifact}。browserArtifact字段精确为browserName/version/revision/platform/sourceRef/archiveRelativePath/archiveSha256/artifactRootRelativePath/executableRelativePath/executableSha256。所有相对路径为规范POSIX，无绝对/空段/点段/反斜杠/NUL。实际可执行路径为canonical(buildRoot/artifactRootRelativePath/executableRelativePath)，其物理节点必须普通且不越拥有root；不得从PATH/系统浏览器猜测。

**获准来源。** `browser/approved-archive-sources.json`的ApprovedArchiveSources1={schema:'vesta-approved-browser-archives/1',scope,revision,approvals}，规范字节SHA固定在外部launchPlan。每个唯一sourceRef条目精确含browserName/version/revision/platform/sourceUrl/archiveFormat/archiveSha256及sourceRef；按sourceRef和完整身份tuple共同匹配，archiveFormat仅zip/tar.xz/tar.gz。生产scope=production，sourceUrl须为后续用户授权固定的官方发布者/分发者HTTPS具体制品来源，不能latest/通配/只看文件名。该表当前approvals=[]，没有选择或批准真实浏览器；未知sourceRef无法生产启动。fixture表仅用于合成比较，scope=design-fixture、fixture://不是生产来源。内容SHA绑定获准字节，不声称证明网络传输路径；真实获取/资格仍另授权。

**归档和receipt。** 本地归档路径固定`browser/archives/<archiveSha256>.<approved archiveFormat>`，启动核验必须读回普通文件SHA。另获授权的安装器核归档后安全解出全部普通文件，拒绝链接/特殊节点/越根，枚举artifactRoot下完整文件集。安装receipt位于`browser/install-receipts/<buildId>.json`，BrowserInstallReceipt1字段精确为schema:'vesta-browser-install-receipt/1'、buildId、buildRootCanonicalAbsolutePath、mainBuildManifestSha256、browserManifestSha256、sourceRegistrySha256、artifact（完整BrowserArtifact1）、installedFiles、extractionPolicy:'ordinary-files-no-links/1'。installedFiles为全部普通文件的{path,bytes,sha256}，path相对artifactRoot，按UTF-8字节序排序且唯一，bytes为u64；不只记主程序。

全部这些独立JSON文件采用UTF-8规范字节：键排序、ensure_ascii=false、无空格/BOM/行尾换行，拒绝重复键/非有限数，整数不经JS Number截断。receipt在实际安装核验后生成；最后由独立可信准备步骤把receipt原始规范字节SHA放入外部launchPlan，任务input manifest再pin launchPlan。主/浏览器manifest不包含receipt SHA；receipt包含二者SHA；外部plan最后包含receipt SHA，三者均不保存自己的可信摘要，避免循环或候选自证。

**启动前顺序与失效。** 先核活跃授权和外部plan pin→拥有root及主manifest SHA/引用→浏览器manifest SHA/buildId/路径→来源表pin与唯一批准tuple→归档实际SHA→固定receipt路径/格式/外部pin→receipt所有身份字段对应→完整安装文件集合/长度/SHA/普通节点→派生可执行路径与获准归档的version/revision/platform元数据。receipt的成功布尔值或自报SHA一律不能代替实际读回。授权退休、root/buildId/manifest/registry/来源/归档/receipt或任一安装成员变化/缺失/多出即失效，停止，不自动重装/下载/切系统浏览器；重新准备也须原任务权限仍有效或另行授权。

所有缺失/伪造/不符在server/browser启动前统一E_RENDER_FAILED，reason='browser-unbound-or-mismatch'，退出3。正常模式和截图共用前置，真实启动/截图/恢复仍未验证。30个合成例覆盖外部pin与授权失配、完整匹配、receipt缺失/伪造/旧manifest、来源未批准、root/主摘要不符、新路径字段缺失、绝对/越界/特殊文件、template、安装成员变化等；relative-path-escape的observed路径与候选路径保持自洽。productionEligible=false由检查器断言，fixture不授予真实资格。

R4/browser_design_checks.py只比较合成JSON observations，不读取真实浏览器/归档、不校验实际OS权限、不解压、不实现installer或launcher。上述物理读回和授权来源是未来实现须履行的合同，不是本轮已做事实。P1.3 C4设计接受保持；P1.2/P1.4/整体partial，不进入P2–P5、产品或运行验证。
