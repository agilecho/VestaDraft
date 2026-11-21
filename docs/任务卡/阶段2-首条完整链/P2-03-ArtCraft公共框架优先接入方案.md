# P2-03后继：ArtCraft公共框架优先接入方案

2026-11-21。用户已确认采用ArtCraft复用公共框架的做法；本文是据此修订的方案，**尚未施工、尚未获dsh独立接受**。原方案与复审原件保留。本文不新增任务卡、测试额度或依赖获取权限。

## 给你看的

以后按ArtCraft的方式组织Draft：**公共框架承担通用软件能力，Draft接入并实现晶体业务。** 先找公共框架已有入口和扩展点，确实没有等价能力时才保留自研。原方案只替换少量通用函数，不能充分落实这个方向，撤回其作为当前推荐方案的地位。

核对WordCraft实际源码后，发现它已经采用统一的`craft`门面：应用固定一个公共库版本，通过feature选择需要的能力；界面、宿主服务、配置、快捷键、控制通道等共同接入。不是复制几段算法，也不是让Draft依赖WordCraft这个应用。

按用户后继要求，又对照了PdfCraft、VectorCraft、EffectCraft、CADCraft、SolidCraft和PhotoCraft。它们共同采用共享能力＋应用业务适配，但具体接法有门面、独立公共crate和workspace共享层之别；实际部分仓库仍有多rev。Draft采用统一公共门面是本方案的选择，不能声称七个应用已经统一如此。详细事实与取舍见[七应用对照](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-artcraft-foundation-direction/CRAFT-COMPARISON.md)。

| Draft需要的成果 | 修订后的接入方向 |
|---|---|
| 软件的公共底座 | 以`craft`门面及其公共Host、主题、本地化、配置、快捷键、平台接口为默认来源，不再逐项另造同类框架 |
| 人工操作与Agent操作同一个工程 | 参考WordCraft将公共控制入口、页面API、MCP接到同一应用Engine的方式；Draft的命令语义与状态约束通过适配实现 |
| 通用界面和交互 | 把公共UI壳、组件和行为库列为目标复用项。WordCraft使用egui，而Draft现有方案是React/Three.js；完整UI迁移需要明确路线差异与晶体画布接法，不能声称已经无缝兼容 |
| 晶体内容、文件与可靠保存 | 晶体模型、科学规则、Source7及可信保存义务保留；外围尽量用公共接口，不能因为公共默认行为较宽松就降低已确认要求 |

当前可以确定的是**采用公共框架优先的方法和统一依赖方式**。UI技术栈迁移、工程格式变化等具体改变仍需方案论证；不用再次确认“要不要采用公共库”。本轮没有改产品代码，也没有因这个方向决定解除停报。

原预算保持：**138.027328167/14400秒，测试14/14，保守冷构建3/3，无新获取，P2-03 partial/未接受。** 旧四件工程迁移和完整生命周期的未完成验证继续保留。后续施工必须有真实可用的依赖、锁定与验证资源，不能用余下总秒数绕过次数上限。

## 给执行者看的

### 1. 决定、依据与后继关系

用户原话：“我的建议是artcraft复制重做软件的做法直接借鉴，即然它有公共库，也是很多其它的软件采用的如wordcraft，那我们就沿用它的做法！！！”据此登记为公共框架优先的工程方向确认，取代原方案默认“少量原语替换”的选择，不登记为整卡接受或资源增额。

权威顺序仍为用户现行决定→现行PRD/技术方案→已接受设计→任务卡。现行PRD实际入口为[DraftDuet PRD](/Users/agilecho/code/DraftDuet/docs/PRD.md) SW-4/5/7与[技术方案](/Users/agilecho/code/DraftDuet/docs/技术方案.md) C4/C16、§9.6；其固定身份沿用原盘点authority-pins。命令操作、人工续改和个人空间保存的用户结果不变。

[P2-03原卡](/Users/agilecho/code/VestaDraft/docs/任务卡/阶段2-首条完整链/03-晶体命令内核与原生保存基础.md) SHA仍为`13437984e00cef259ebfb4e7d852d54f9b8251adaf6b72bd41f7749b476f0f57`；U01/U02/U14、T66、NS2、FC、HS4不缩窄。[P1设计](/Users/agilecho/code/VestaDraft/docs/superpowers/specs/2026-11-10-vesta-p1-design.md)中Rust/WASM单Worker＋TS/React/Three.js是当前路线；将其改为公共egui壳是明确的候选路线变更，本文列出影响，未偷偷改写已接受设计。

[原公共库方案](/Users/agilecho/code/VestaDraft/docs/任务卡/阶段2-首条完整链/P2-03-公共基础库接入整改方案.md) SHA `ec30b24289d064ec328d910189398fe0eb833d4fb2714a7ef5741634cabd9ee4`保留原字节；[dsh方案复审](/Users/agilecho/code/VestaDraft/output/vesta-dsh-reviews/20261121-105114-p2-03-craft-libs-plan-review/REVIEW.md)为“有条件通过、需修订后复审”，并非已接受。其F1–F5按§5修订；[Codex只读复核](/Users/agilecho/code/VestaDraft/output/vesta-execution/20261121-p2-03-craft-libs-plan-review-recheck/RECHECK.md)不替代dsh。

### 2. 实际消费者基线：先选对公共框架入口

| 输入 | 本轮实核身份及用途 |
|---|---|
| WordCraft | HEAD `685869239f31c890c6365f264c14b29ef48fcb7f`；读取AGENTS、架构/测试/恢复/控制文档及实际Cargo、UI、web、MCP、保存源码。选中16份工作区文件内容均等于该提交，不能把模式变化解释为内容修改 |
| WordCraft实际依赖的craft-libs | `243d68e12d05e89122db6d154924bf1992e10d38`；WordCraft已提交的根Cargo明确pin此rev，并声明`craft`，native/web/UI消费者选择features。直接的craft-codepage/keymap/mime/mcp-server、ui-test-kit亦使用同一rev |
| 旧盘点基线 | 本地craft-libs HEAD和本地origin/main为`c6c90e991b776a7d2a4231e14220b91c804d650f`。旧199个workspace member、crates下206个递归manifest、122份输入属于这个提交；不冒充新框架基线 |
| 新基线范围 | `243d68e1…`的crates下有229个顶层member manifest、236个递归manifest；7个嵌套manifest不是额外顶层member。两版本有增加也有移除，不把差额简单等同于新增库数；新名单包含工具/演示crate，不都是生产基础库 |
| 取证方式 | 新提交已在本地Git对象库，使用`git show`/`ls-tree`读固定blob；没有fetch、checkout、vendor或Cargo解析。固定53份参考blob及229份member声明身份，不是229库逐行审计/平台验收 |

[本轮证据目录](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-artcraft-foundation-direction/)保存`reference-input-pins.json`、`consumer-baseline-inventory.json`与`reference-blobs/`。它们是只读取证副本，不是可供产品构建的vendor输入。新版本的`crates/behavior/README.md`不存在，已列为缺少的请求路径；其实际src/lib.rs已固定，不称已读该README。

可直接核实：[craft门面说明](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-artcraft-foundation-direction/reference-blobs/craft-libs/crates/craft/README.md)、[门面manifest](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-artcraft-foundation-direction/reference-blobs/craft-libs/crates/craft/Cargo.toml)、[WordCraft根依赖](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-artcraft-foundation-direction/reference-blobs/wordcraft/Cargo.toml)、[实际UI features](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-artcraft-foundation-direction/reference-blobs/wordcraft/crates/ui-egui/Cargo.toml)、[同Engine MCP适配](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-artcraft-foundation-direction/reference-blobs/wordcraft/crates/mcp/src/server.rs)。

### 3. 目标组织与接入方式

补充消费者证据：扫描38个本地独立Git仓库的根manifest（21份明确声明craft-libs、2份声明craft门面），并为新增六个重点应用固定239份说明/manifest/选中源码。根扫描不是完整依赖图或“不使用公共库”的反证，架构文档的旧数量不作为当前验收。PdfCraft直接再导出公共PDF对象层、VectorCraft公共容器＋领域Document适配、EffectCraft公共docking/layout事务＋业务面板适配、CADCraft公共DXF标签＋Drawing映射，均支持优先接入完整共享子系统。SolidCraft自身shared视口/UI层仅作边界参考；PhotoCraft也使用craft门面但pin为另一rev，不混入本方案243d68e1… API基线。

目标依赖方向为：公共`craft`框架→Draft平台/宿主/协议适配→Draft应用Engine→晶体模型与codec。UI、页面API、MCP和原生CLI最终操作同一Engine；公共框架不成为第二套晶体状态持有者，Draft不依赖wordcraft-engine等应用业务crate。

| 能力层 | 直接采用的公共边界 | Draft保留的业务适配 |
|---|---|---|
| 公共服务 | `craft::host`、`theme`、`i18n`，公共settings/keymap/platform接口；配置不再单独设计另一套通用框架 | Host的Files/Jobs/SectionStore显式注入。默认NoPickFiles仍有普通文件读写，native默认ThreadJobs、WASM默认InlineJobs，不能直接获得Draft目录权限或突破单Worker/调度限制。Host Commands提供标签/提示，不等于Draft Engine执行器 |
| 通用交互与UI | `craft::behavior`纯行为层；公共ui/window/menu/dialogs/quit及主题壳为完整UI目标来源 | 选择、数值输入和快捷键连接Draft命令；晶体3D画布和空间坐标/相机仍由业务适配承接。公共EditHistory不能直接代替Draft事务、undo/redo和资源拥有权 |
| 软件控制 | 公共automation/control、web-api以及MCP server的生命周期/传输设施优先复用 | 通过适配进入NS2/Engine，保留seq、epoch、取消、配额、严格JSON及权限；公共通道ID不取代documentId，MCP生命周期不冒充NS2生命周期 |
| 页面宿主 | 公共web-host的消息与桥接接口 | 可信保存单独接HS4。公共saved(id)或无id的“已发出”不清Draft dirty；消息传输成功与个人空间保存完成分开核验 |
| 编码与原生I/O | 门面jsonrpc/mime/sniff/atomic等能力，开发侧testkit与相应测试工具 | 保持物理帧边界、schema错误、原生多文件发布次序和更严格持久化策略；不是逐函数拷贝源码 |
| 晶体领域 | 当前没有证明等价的公共实现 | 模型/科学算法、生成角色访问器、Source7/fresh身份、资源lease、T66/HS4规则继续在Draft；后续若找到等价公共能力，再按真实语义接入 |

**拟定统一依赖方式**：遵循WordCraft，在应用workspace根声明`craft`的Git URL、完整rev、版本和features；各消费者用workspace依赖选择能力。选`243d68e1…`作为本方案候选基线，不用移动分支、同名`0.1.0`或T9工作区HEAD代替pin。超出门面的开发工具/确有必要的独立crate使用同rev声明，保留来源与许可。旧方案“只vendor五库并改写manifest”不再是默认接法；确有离线交付要求时，未来可导出完整可复现依赖，须另留锁与来源证明。

`default-features=false`不等于无基础依赖：门面仍强制behavior/host/i18n/theme与toml等；行为层虽不依赖egui，却依赖emath/kurbo/accesskit等。`ui`还启用egui并向已启用的相应库转发ui。WordCraft的egui fork patch需要消费者workspace根承担，库自身patch不会替应用生效。不能复制其全部PDF、字体、桌面patch；实际选中图、feature合并、锁和平台影响须受控解析后再报告。

### 4. 如何处理与当前路线的差异

推荐以公共框架为目标，不再以“当前没有消费者”为理由预先排除公共UI/平台层；同时按实际依赖办理：

| 具体问题 | 后继处理 |
|---|---|
| P2-03原生部分可以使用门面非UI能力 | 可设计先接公共框架依赖及I/O/协议适配；不要求先迁移整套UI，也不能仅引入一个未使用的门面就称框架接入完成 |
| 全面照WordCraft接公共UI会改变React/Three.js路线 | 公共egui UI壳作为优先评估的目标方案，提交“现有前端保留并桥接公共服务”与“egui公共壳＋晶体画布适配”的具体差异；比较现有投入、3D渲染、单Worker、当前页面控制和个人空间保存。尚无编译/运行证据，不声称Three.js可直接嵌入egui或wgpu可直接替代。相关路线取舍在该比较后裁定 |
| 公共format/document-bundle是ZIP | 不自动改变已确认`.vesta＋资源目录`。容器迁移属于独立需求/兼容性改变，未包含在本次公共框架方法决定内 |
| 公共默认宿主/保存比Draft约束宽松 | 优先找公开注入/扩展点实现Draft规则；无法等价时保留相应适配，不修改共享dsh核心，不以改黄金值使测试变绿 |

上述比较属于方案工作；UI/Web/MCP等按对应真实工程结果进入，不把它们全部并入P2-03来扩大本卡范围，也不新发卡或挪预算。

### 5. dsh F1–F5修订与必须锁定的判据

**F1：同步语义。** Rust 1.99.1在Apple的`File::sync_all`已经调用`fcntl(F_FULLFSYNC)`，公共atomic不是新增这个能力。固定新版atomic仍在ENOTSUP(45)/ENOTTY(25)后回退普通fsync；成功是以较弱的同步接口换取网络卷可用性，失败传播回退错误，其他错误保持传播。WordCraft自己的safe_save还有EINVAL分支，和公共atomic也非完全相同，不能一概照搬。依据为[Rust固定版本实现](https://raw.githubusercontent.com/rust-lang/rust/1.99.1/library/std/src/sys/fs/unix.rs)及本轮固定atomic/WordCraft保存源码。

默认保留Draft现有同步强度；公共框架优先不等于默认接受该回退。若选用回退，须把文件和父目录策略、较弱成功的结果标识及对外宣称范围写入具体施工依据。未来注入用例至少锁定fullsync成功、45/25回退成功/失败、其他错误、文件与父目录两处失败；父目录失败发生在rename后时不能声称旧primary未变。公共整套writer的symlink/best-effort目录同步也不能直接替换现有路径与发布策略。

**F2：stdio行为。** `FrameReadError::Io`必须映射现有stdin读取错误：静默结束读流、丢弃未完成帧、不生成N_FRAME，已入队请求仍可能处理；干净结束exit 0，有未结算快照等则E_IO/exit 3。不能改成N_FRAME从而变为exit 2。TooLong保持原消息`frame exceeds 1048576 bytes`、N_FRAME及E_SCHEMA/exit 2（输出正常且该错误到达处理器时），超限后终止读流；若stdout失败则仍可能E_IO/3。共享reader按不含LF计限，而Draft按包含LF的物理帧计限；适配必须保留LF、CRLF、EOF无LF、部分帧后I/O错误及精确1MiB/+1边界。空行/非法UTF-8/重复键/深度和节点预算继续交原严格解析，不能换成普通JSON-RPC解析器。

**F3：Base64收益与证据。** 删除旧实现的理由是公共标准化与维护性，未证明现有实现有缺陷。dsh的175,570输入、0差异是两份Python转写的有限差分，非真实Rust实现的全输入等价性证明；不沿用“190k+”措辞。未来删除旧生产实现前，将其固定为一次性测试oracle并记录SHA，使用实际Rust两实现做有界差分及资源上限/无效padding/尾bits验证，完成后不留下双生产实现。mime无default feature，写false是no-op；其codepage/encoding_rs/chrono等成本必须如实进入依赖图。本轮已补codepage Cargo及src/lib.rs的固定blob证据，未生成vendor。

**F4：追溯。** 本文附录逐项覆盖原43候选，并将“接法”与“进入时点”分列。原199/206与新229/236分开登记，122旧pin不回填或冒充新版本；新参考证据包含codepage。未来依赖准入时需完整实际图及许可/锁证据，53份选中说明/源码pin不是传递依赖完整vendor证明。

**F5：范围和工具链。** “属于本卡主题”不等于依赖/新锁/构建/测试授权。testkit拟用于CLI以及model/commands的dev测试，只能作为开发依赖，不能说只影响CLI。新版声明edition 2024、rust-version 1.95；本机1.99.1在声明层面满足，但并未据此证明全部传递依赖、egui fork、WASM、native或其他平台可用。静态读取不是产品测试组，也不能把产品验证改名为静态读取绕过14/14。

### 6. 后续施工顺序及真实验收（本轮均未执行）

| 顺序 | 可审交付 | 进入条件与停止条件 |
|---|---|---|
| 1 恢复可验基线 | 修正原两条驱动的dirty判据和domain.prepare params.resultMode；真实到达旧四件迁移及完整生命周期，保留旧失败 | 先有明确可用验证额度；失败继续partial，不把换框架代替原验收 |
| 2 公共框架依赖准入 | 同rev门面与必要独立crate、选中features、真实Cargo.lock差异、来源/许可、平台边界；参考WordCraft根patch规则 | 先有依赖获取/锁变更和必要验证资源。超出现行上限即停，不启动fetch、全库构建或冷构建来试探 |
| 3 原生消费者适配 | 实际使用门面jsonrpc等、明确Host服务消费者及I/O适配；删除对应重复生产逻辑，保留协议/持久化义务 | stdio注入、原错误码/退出、边界、Base64真实差分和保存次序符合§5；若业务状态发生非预期变化，停止该接入点 |
| 4 对应UI/Web/控制结果 | 公共配置、宿主、UI/行为、页面/MCP按确定路线接到同Engine；说明哪些公共模块真实运行 | 路线差异与实际依赖已解决，且对应任务/资源可用；不把此行当作本卡额外施工授权 |
| 5 同源验收交回 | 源码/锁/制品同源，独立读真实工程与资源，当前Core与旧迁移、NS2关口及生命周期真实证明 | Codex自验后整理材料，用户手动发dsh；没有自动转审、合并、发布或下一卡 |

最低验证须证明：非默认hidden/boundary/camera与fresh资源身份、两个逻辑资源共享一份字节、进程销毁后实际读取续改；旧四件工程迁移另存并重开；旧documentId、合法unsupported、分页/status、release/reject/cancel/close/shutdown/EOF与目录额外制品的真实处理。冻结NS2关口零skip/cancel；Source7生成访问器与冻结profile不得由公共框架名义替代。真实文件存在、绿色单测或框架函数被调用均不能单独证明完成。

本轮不分配新测试组，不发可直接越过停点执行的命令。未来具体依赖图尚未解析，不能编造获取字节数、构建时长或承诺用现有零余次数完成。旧制品、资源账和seal继续保留；回退恢复候选源码/依赖，不删除用户工程或失败证据。

### 附录：原43项候选逐项去向

“接法”说明公共API与业务适配关系；“进入时点”说明对应消费者/资格，两轴均不是额度或施工批次。原旧分类仍在旧inventory中，以下为新方向的后继处置，不重算旧5/17/21/156。

| 候选 | 接法/处置 | 进入时点与原因 |
|---|---|---|
| craft-jsonrpc | 门面直接调用reader＋协议适配 | 原生续办；保持§5物理帧/Io/TooLong，NS2不外包 |
| craft-keymap | 公共命令/快捷键设施＋业务绑定 | 控制/UI；不自建另一套通用快捷键框架 |
| craft-mime | 门面标准编码＋限额包装 | 原生续办；维护性收益，先保留一次性差分oracle |
| craft-sniff | 公共识别＋Draft路由适配 | 格式消费者；不替代冻结科学路由资格 |
| craft-testkit | 同rev独立dev依赖 | CLI/model/commands开发验证，未进生产 |
| craft-atomic | 公共同步能力条件适配 | 原生续办；先锁定较弱回退政策，整套writer不直接替换 |
| craft-control | 公共控制设施＋NS2入口适配 | 控制消费者；保留生命周期与配额 |
| craft-control-protocol | 可复用公共传输类型，应用协议映射 | 控制消费者；不替换NS2信封/重放规则 |
| craft-docpath | 公共文档定位＋领域映射 | UI/控制；不是原生授权路径安全检查器 |
| craft-document-bundle | 容器/依赖能力按需适配 | 格式消费者；当前不改为ZIP原生工程 |
| craft-format | 公共格式设施按需接入 | 格式消费者；Source7及原生codec规则保留 |
| craft-gpu | 公共渲染接口候选适配 | UI/3D路线验证；不直接认定科学渲染等价 |
| craft-gpu-null | 开发侧公共空后端候选 | 相关渲染测试；不是实际GPU证据 |
| craft-i18n | 门面基础能力直接采用 | 公共框架基础；晶体术语目录属应用 |
| craft-ipc | 公共IPC设施＋权限适配 | 原生控制；不新增未经批准端口/进程 |
| craft-mcp | 公共MCP能力按需采用 | MCP消费者；不启动第二套Engine |
| craft-mcp-server | 同WordCraft的server＋Tools适配 | MCP消费者；不拿MCP状态机替代NS2 |
| craft-platform | 公共平台接口＋宿主实现 | 平台消费者；时钟/剪贴板/文件与单Worker约束逐项核实 |
| craft-render-parity | 同rev公共开发工具候选 | 渲染验证；不凭结构一致宣称像素/科学一致 |
| craft-settings | 公共配置存储＋Draft配置类型 | 公共宿主/UI；工程内容不塞入偏好存储 |
| craft-web-api | 公共页面命令入口＋Engine适配 | Web消费者；当前页面操作、超时/取消语义接现行合同 |
| craft-web-host | 公共宿主桥＋HS4适配 | Web/保存；普通saved不等于可信Saved |
| craft-ai | 不与现行平台重复建立AI编排 | 暂不启用；沿用平台/公开扩展边界 |
| craft-ai-core | 按真实接口需求再选 | 暂不启用；CA引导工具不是产品已接入证明 |
| craft-cloud | 公共云能力按宿主需求适配 | 暂不启用；不能绕开个人空间和权限 |
| craft-cloud-local | 公共本地实现候选 | 暂不启用；不建立第二套工程持久化入口 |
| craft-dialogs | 公共对话框/文件服务＋授权适配 | 原生UI；替换自建通用壳的目标项 |
| craft-doc | 不替换晶体领域模型 | 暂不启用；通用文稿类型不等价晶体Source7 |
| craft-file-formats | 公共格式能力目录＋科学路由映射 | 格式消费者；不冒充全85路由通过 |
| craft-file-import | 按实际格式选择公共实现 | 格式消费者；不启用整个大聚合默认图 |
| craft-geom | 按真实几何需求复用 | 3D路线；不未经比对替换晶胞科学算法 |
| craft-gpu-providers | 公共后端提供者候选 | GPU路线验证后；不是当前必选依赖 |
| craft-gpu-wgpu | 公共GPU实现候选 | GPU/UI路线验证后；不暗改Three.js路线 |
| craft-math | 标准数学能力按需复用 | 科学/几何消费者；数值规则与容差需领域验证 |
| craft-menu | 公共菜单＋Draft命令绑定 | 公共UI；无需重新自建同类组件体系 |
| craft-plate-test | 公共开发工具候选 | 对应渲染测试；不作为当前必跑全库套件 |
| craft-quit | 公共退出交互＋未保存状态适配 | 公共UI；不能绕过close/shutdown/HS4 |
| craft-suite-account | 平台账号适配候选 | 暂不启用；不重复账号体系或改变身份授权 |
| craft-time | 按媒体时间需求选择 | 当前不启用；不是单调时钟替代品 |
| craft-ui | 公共界面壳/组件的目标底座 | UI路线差异明确后；不再因旧方案默认排除 |
| craft-update | 公共更新能力按分发需求选择 | 分发消费者；本卡不联网更新/安装 |
| craft-window | 公共窗口/恢复设施＋宿主适配 | 原生UI；浏览器页面与桌面边界分别处理 |
| craft-xtask | 借鉴检查组织，按实际需要选工具 | 开发工具；不复制全库任务或运行全库构建 |

新基线额外的craft门面、craft-host、craft-behavior、craft-theme及craft-automation等不属于原43项，已在§2–3说明；不把它们塞回旧数字。完整差异名单见本轮inventory。
