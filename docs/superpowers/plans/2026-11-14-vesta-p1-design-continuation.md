# VESTA P1 设计续办任务卡

日期：2026-11-14。状态：已发卡，未施工。所属任务：原 P1 的同卡续办，范围为 P1.2–P1.4；P1.1 已接受成果继续保留。

## 给你看的

这张卡用于在新的施工会话中，把 VESTA 剩余设计补到能由后续实现者独立接手的程度：先说清复杂科学功能和文件格式怎样进入统一内核，再说清构建与检查工具怎样连接，最后给出覆盖全部已发现功能的具体施工安排。

已有原件研究和技术路线继续沿用。本卡交付的是修订后的设计、接口、实施计划和可核查的设计验证材料。完成后仍不会产生可使用的编辑器，不改平台、不部署，也不启动模型施工。

成功标准是后续实现者不必边施工边猜关键接口，独立复审者能分清哪些设计已有依据、哪些运行验证留待实施。如果某项设计必须依赖本卡范围外的新实验或业务裁定，执行者应完成可继续部分，明确交出阻塞问题，不能用占位协议或扩大权限换取“完成”。

Codex 直接执行设计和自验。施工会话结束后，把完整待审材料交给你，由你手动转交 dsh；独立复审和明确可接受的判断由 dsh 承担。即使设计通过，也需另行授权才能进入产品实现。

## 给执行者看的

### 1 任务理解与授权边界

本卡依据用户 2026-11-14 的指令：“把 P1 剩余工作整理成一张可独立接手的续办任务卡，我新开施工会话执行；范围仍限设计，不进入产品实现。”由用户在新会话手动转交后开工；不要在发卡或复审会话顺带执行。

按 `superpowers:executing-plans` 直接推进下文步骤；本卡已确定执行方式，不再询问执行模式。不自动派子 Agent，不交 dsh 施工。dsh 独立复审由用户手动发起；不得调用工具发送待审或整改材料、联系 dsh、创建或启动其会话。磁盘 AGENTS 若未写全这些约定，以本卡所载用户最新约定为准。

允许：读取权威文档、既有原始证据、锁定源码及必要的一手资料；修订设计与计划；在本轮新证据目录编写设计 schema、输入输出示例、独立数学预期及文档校验脚本。补读外部资料须保留具体来源、版本、正文范围和对设计的影响。

不允许：创建 VestaCraft workspace 或产品目录、初始化 Git 或 worktree、写产品实现、实现 prepare 或 launcher、安装或升级依赖、编译构建产品或试制产品、修改共享平台、部署、重建研究调查、运行 VESTA 或外算程序、启动 CA worker／Codex builder／额外模型任务，或执行原生 `rebuild_check`。需要这些操作才能回答的问题转为具名依赖并停下相应步骤；不把“设计补证”当运行授权。

本卡延续原 P1 的预算、累计费用、次数与停报状态，不因新会话重置。开工核实既有账目；本卡不新增付费调用或目标运行额度。普通本地只读核验与新目录中的文档检查可直接执行。

### 2 当前目录与真实基线

本次发卡核实的工作根为 `/Users/agilecho/code/Draft`，不是 Git 仓库。以下缩写只用于定位，不改变历史记录：

```text
ROOT = /Users/agilecho/code/Draft
E    = ROOT/output/vesta-study/20261108-113622
R0   = E/reviews/20261110-p1-design
BK   = E/reviews/20261111-095500-p1-bookkeeping
```

历史文档内的 `/Users/agilecho/code/Craft/...` 当前不可访问；对应本项目材料实际位于 `ROOT/...`。读取时先按相同相对后缀核内容、摘要和归档关系，再记录定位映射。不要创建 Craft 路径或符号链接来迁就旧脚本，也不要批量替换历史文件里的路径。T9 源码路径迁移与本次 Draft 定位是两件事。

| 项目 | 发卡时已核实状态 |
|---|---|
| 当前资料基线 | `E/s01/baseline-rev25.json`；HANDOFF 等少数摘要仍写 rev24，以 rev25 和 BK 的归档链解释差异 |
| 已接受研究 | S01–S08、Study 29/29、P0 研究交付完成；不是科学、格式或产品全通过 |
| P1 | P1.1 接受；P1.2、P1.3、P1.4 和整体 P1 仍为 partial／未完成 |
| 研究资产 | 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条逐文件核验的磁盘派生原生记录，非全量 raw export |
| 原生问题头 | 24 个：10 resolved、3 investigating、11 open；14 个未解决，不能由阶段通过改为 resolved |
| 最新 I4 处理 | 2026-11-11 dsh 判 R3-1／R3-2 实质解决；其后 BK 清理文书残留并绑定 rev25，没有推进剩余设计 |
| 未来产品位置 | `/Users/agilecho/code/VestaCraft`，产品根 `app/`；发卡时不存在，本卡不得创建 |
| 锁定 CA 方法 | `1f8508a104134c436cbba5ea2f675619ef47702f`；活跃源码仅为定位，不自动升级方法 |
| 锁定 CLI | `ROOT/output/tooling/craft-anything-1f8508a1/debug/craft-anything-cli`，SHA256 `99ad8c99cc5d7b35dfd85eb9b1a126ab7c074f3a5adbc7453b7a3d457d4a8869`；本卡不启动 worker 或 Check |

以下 SHA256 是发卡核对值。变化先查原因和对应新交付，不能自动覆盖当前工作或退回旧版：

| 文件，相对 ROOT | SHA256 |
|---|---|
| `docs/superpowers/specs/2026-11-10-vesta-p1-design.md` | `8042f24bf806bd46975532016e39c0f9877ee39fbef646c7fef162da49b8ceeb` |
| `docs/superpowers/specs/2026-11-10-vesta-interfaces.md` | `7e71e4e6a9fc1951f8c971dad589d8bfebd6df2b77a49bcb274bb9d6034cb259` |
| `docs/superpowers/specs/2026-11-10-vesta-ca-wiring.md` | `e85b868ba2c8e06307dd95bb08d4e5eb5a0b82cec00da3ee2f34251c5ff558b3` |
| `docs/superpowers/plans/2026-11-10-vesta-modules.md` | `9a1d92d0a49e71891471f7ceb311cb3fa1f211c3cf528a3453a751188e3a4028` |
| `output/vesta-study/20261108-113622/s01/baseline-rev25.json` | `0983bfd08600dd8282fdf0c7e5df8b8a7e7f3a0844931fea7b0629a8d01619c0` |

### 3 开工读取顺序

以下均相对 ROOT。先读当前文档与最新续办事实，再沿功能规格定位需要的原始材料；不要从历史末尾的“下一步”或旧 README 重新排阶段。

1. `AGENTS.md`、本卡全文、`docs/VESTA全功能重写要求.md`、`docs/接入技术要求.md`、`docs/VESTA研究/HANDOFF.md` 当前头与最新接续段、`docs/VESTA研究/BASELINE.md` 的 rev25 登记。
2. `E/s01/baseline-rev25.json`、`E/reviews/current-task-status.json`；`R0/review-result-effective.json`、`R0/dsh-review-I4-final.md` 和 `.json`；`BK/README.md`、`BK/result.json`、`BK/verification-rev25.json` 与接口文书修正 diff。BK README 的 3438 为旧摘要，最终验证 JSON 是 3440 项、passed=true；不能混成一次新独立复审。
3. `docs/superpowers/plans/2026-11-08-vesta-full-rewrite.md` 的 P1 原四项和通过条件；现有四份 P1 文档：架构 D2、接口 I4、CA 接线 C2、模块计划。以现行文件和 BK 为当前内容，不把 `R0/README.md` 的“I4 未复审”旧表述当最新事实。
4. `docs/VESTA研究/STUDY-REPORT.md` 的设计输入、依赖与 DPRE-01–08；`docs/VESTA研究/functions.json` 是唯一功能规格；`S06_DOMAIN_RULES.json`、`S06_ORACLES.md`、`UNKNOWNS.md`、`FORMATS.md`、`ALGORITHMS.md`、`CLI.md` 和五篇行为文档按受影响功能阅读。31 组领域规则与其余应用／交互入口都要进入设计安排。
5. `docs/VESTA资料索引.md`：P1.2 读 D06 及领域资料；P1.3 读 CA04／CA08、METHOD、锁定相邻源码；P1.4 读 D11／D12。原始输入输出从每条 `studyHandoff`、SAMPLES 和下列目录取，不凭名称补造参数。

关键原始证据入口：

- `E/reviews/20261110-104021-s05-completion/`：逐格式 input／output 矩阵、字段解析、真实编辑与新进程读取、独立 PRI／mesh 分析。
- `E/reviews/20261110-135709-s06-domain-study/`：领域矩阵、独立规则及 `research/MECHANICS.md`、`research/mechanics.json`；后两者只是未来迁移输入。
- `E/reviews/20261110-150652-s07-contract-audit/`：`official-command-contracts.json`、argv 索引、外部程序与完成边界。16 条原件 CLI 的“相关 VF 导航”不等于产品命令等价。
- `E/reviews/20261110-155840-s08-study-closure/`：unknown 裁定、失败边界、平台参考和 601 文件封存；保持旧封存字节。
- `R0/method-snapshot/`、`method-source-manifest.json`、`method-kind-supplement.json`、`launch-resolution-probe/`、`platform-snapshot/`、`source-baseline.json`：优先读锁定快照。CA 活跃源码定位为 `/Volumes/T9/Development/Sources/Artcraft/craft-anything`；新增源码只按锁定 git 对象读取。

平台参考快照是当次事实。若设计需引用现有 `/Users/agilecho/code/IntegrateCraft` 的能力，先核实际路径、HEAD、受影响文件摘要及现有改动；不切换到另一个项目，不清理外部改动，不把 BranchDraft 界面讨论混入本卡。

### 4 已定决定与待续办内容

保持全功能 1:1 和 Deep 方法。P1.1 的 Rust 原创领域内核、WASM 单 Worker、TS／React／Three.js WebGL2、同内核原生 CLI 路线不重选；历史版本选型继续作为有来源的设计输入，不自动升级为 latest，也不声称依赖已经安装或集成。

两项用户裁定必须延续：默认复现已证原件行为并提示科学／数据风险，修正须显式选择；默认原生 `.vesta` 加资源保存，完整提交、冲突与恢复。保留平台对正确保存重开和故障隔离的要求，不能为“兼容”主动丢失刚保存的资源或让宿主崩溃。

人和 Agent 使用当前同一内核；三视图只改布局；真正关页且保存确认后才交接。队列接受、内核提交、对应修订渲染、宿主实际保存分别定义。保留本人空间、60 秒单文件自动保存、失败保持 dirty、迟到 ack 隔离和无浏览器作品持久副本的约束。

I4 已解决的资源 trait／advance 与保存封闭联合继续使用，不再作为未整改项重做。剩余核心缺口是科学／codec 公共参数、CA 桥接的精确设计和 G1–G6 的完整实施安排。`RawUnsupported`、`E_RULE_UNFROZEN` 可以如实表达未支持，不能替代这些设计完成条件。

### 5 实施顺序与交付物

以下是同一张卡的顺序步骤，不另拆新卡，也不允许执行模块计划内的未来产品步骤。

#### 步骤一 核对入口并冻结本轮输入

- [ ] 在新会话输出任务理解：已有接受项、剩余三项、允许写入目录、禁止操作和实际依赖。核当前状态与本卡差异，保留已有成果。
- [ ] 在 `E/reviews/` 下以实际开始时间新建 `<YYYYMMDD-HHMMSS>-p1-design-continuation/`，下称 `R1`；同卡恢复沿用它。记录 `preflight.json`、`before/`、`input-manifest.json`、`progress.md`，包含本轮选用文档／快照摘要、路径映射及既有资源账目；不复制整个调查或修改旧包。
- [ ] 新脚本显式接收当前 ROOT 和 R1，不直接执行硬编码 Craft 路径、会写回旧包或自动启动 dsh 的旧脚本。入口局部失效可据证重定位；锁定身份或核心语义无法确认时停下依赖部分。

#### 步骤二 固定剩余领域与程序接口 P1.2

- [ ] 修订 `docs/superpowers/specs/2026-11-10-vesta-interfaces.md`，同步架构文档。保留基础切片并写出完整公共类型、单位、坐标／张量基底、缺失与非法值、原始字段保留、精度、能力适用域、事务／撤销、错误及四种完成时刻。类型的生产者、消费者与资源生命周期必须闭合。
- [ ] 从接口现有 §7 和原领域规则逐项处理：`PhasePlacement` 的参考图／非正交 R/t／循环；`SymmetrySet`／`ModulationModel` 的 setting、群表、磁性、超空间／调制与向量基底；ADP／不确定度／键价选表；散射／反常／同位素、FFT／Nyquist、等值面、投影、密度与峰盆地。未知规则保持原 unknown 归属；不凭字段名称制造科学结论。
- [ ] 按 S05 的 48 类输入、23 类数据输出及其它输出，明确共享导入／导出模型和逐格式差异：字段映射、单位、方言、未知块、资源引用、读写／编辑能力、失败与拒绝有损写回规则。m3d、PRI、DEN／STO、Elk、ASSE、GAMESS 以及已解释但仍失败的格式必须有具体消费路径。
- [ ] 闭合 G-HOST 对公共接口的影响：资源名、manifest 身份、字节桥、token／lease、配额失败、外改竞争保证和 indeterminate 的查询恢复。说明现有平台能保证什么、拟增什么、后续实际验证什么；不把 verify＋rename 写成任意外改原子 CAS。
- [ ] 在 `R1/contracts/` 保存对应的机器可读设计 schema、有效／无效消息和资源交接示例，文件内明确 `design-only`。不得出现未定义类型、无约束 `options:unknown`、自由 JSON 或只供首链的类型却声称全部领域已冻结。

不要求先关闭全部行为未知。能在不猜内部算法的前提下固定公共边界者，写出依据、适用域和后续独立判据；未知会改变相邻公共类型、保存承诺或测试预期者，列出精确问题与所需证据，P1.2 保持相应未完成，不假填占位值。

#### 步骤三 固定 CA 桥接设计 P1.3

- [ ] 修订 `docs/superpowers/specs/2026-11-10-vesta-ca-wiring.md`，按锁定源码给出 prepare 的未来源码／构建位置、精确依赖和公开 API 签名、配置读取、target／request／stack／backend／kind 的绑定及可运行制品合同。只写设计和未来步骤，不创建工具或 workspace。
- [ ] 逐步设计正式调查与 capture／reference／mechanics 的派生迁入：源与目标、SHA 清单、ID／revision 保持、symlink／边界拒绝、64 MiB／not_loaded 的核对办法、初始原生 manifest 和可恢复失败；不得手填 done 或用 roundtrip 调查替代正式源。
- [ ] 固定混合构建、唯一 `Build/vestacraft-preview` 启动桥和当前真实 Web 截图的未来合同，绑定 Node 标识、Rust／WASM／Web 产物、浏览器与页面 Loaded／Rendered、正常启动、截图模式及拥有进程清理。保留 main.py 不会因存在就被 Node 工程自动启动的已证结论。
- [ ] 把 worker／builder／inbox／checks/asks、请求／回答／轮次身份、round0、同进程自查拒绝、超时／丢 session／检查未完／停止续作写成完整协议序列。明确每条未来命令的 cwd、前提、配置来源、预期产物、成功／失败判据和禁止盲重试的副作用。
- [ ] 将“本卡可完成的设计核验”与“未来实施才运行的 worker／Check 联调”分开列清。未来联调至少含真实 build／screenshot／mechanics、一次故意失败反例、停止与恢复及请求回答绑定，但本卡一项也不运行。

原 dsh 裁定中保留了真实联调未完成的措辞；最新用户授权仍仅设计。本卡不能自行宣告这些实测已通过，也不能据该措辞启动产品。提交时请 dsh 分别判断设计是否可接受、剩余实施验证是否归属清楚；若其认为 P1 通过仍必须先运行，交用户裁定范围冲突，保持原 partial，不暗改原通过条件。

#### 步骤四 形成全部模块的精确施工安排 P1.4

- [ ] 在现有 `docs/superpowers/plans/2026-11-10-vesta-modules.md` 内扩全 Task1–8 与 G1–G6；内部单元按完整可验结果组织，不按功能个数机械拆卡。保留原件 CLI 合同的“相关导航”边界。
- [ ] 每个单元写清计划文件、具体接口签名及相邻消费方、前置设计和运行条件、真实样本／输入 SHA、VF 与规格、独立预期／单位／容差、必要命令及 cwd、资源／取消／清理、失败可继续项、交付及证据。未创建的文件、命令、测试和产物明确标“未来”，本卡不执行。
- [ ] 覆盖结构／群／变胞／ADP、周期几何／形貌、场与科学算子、全部 codec／图像／材质、外算／参数／字体／环境，以及应用／偏好／数据库／交互生命周期。G6 的 OS 行为在 Web／宿主中的对应、网络 Stop、焦点／Undo、动画和面板都要具名设计；不能以首链代替。
- [ ] 对唯一 `functions.json` 的当前 254 个 ID 做派生只读检查：恰有一个主责单元、无遗漏／重复主责／不存在 ID；交叉依赖可以多处引用。检查结果仅是本轮自验输出，不另建功能规格、覆盖矩阵、颜色门、mutations.tsv 或长期映射台账。
- [ ] 对当前 31 组领域规则和其余应用入口核计划消费，检查每个关键 unknown 的设计影响与后续验证归属；字段有归属不等于规则已证。性能、外算、资产、Linux／x86、真实宿主与 CA 联调不能用 Mac 最小探针追认。
- [ ] 为未来验证写出具名预算及依据，区分协议上限、测试预算和未测生产预算。需要后续资源裁定或实验的单元写明进入条件和停点，不把“待定”包装成可直接执行；若因此缺乏可接手的关键安排，P1.4 如实保持未完成。

#### 步骤五 设计自验与交付待审材料

- [ ] 在 R1 编写 `verify_design_packet.py`、`verification-plan.md` 和验证输入。脚本显式收 `--root`、`--evidence-dir`，只读设计／旧证据，仅向 R1 写报告；任何失败非零。脚本不得调用产品、原件、模型、dsh 或原生 Check。
- [ ] 做语义走查和反例核验，至少覆盖下一节断言；机器检查不能只搜索关键词。若已有标准 schema 校验器可直接使用，记录版本；否则如实区分结构检查、手工协议走查和未完成的标准校验，不声称 Python 标准库已验证完整 JSON Schema 语义。
- [ ] 保存 `after/`、`diffs/`、本轮源文件与证据的 SHA 清单、原始 stdout／stderr／退出码、失败及纠正过程、自验报告和 `RECEIPT.md`。保留所有旧裁定、旧失败、旧 baseline／归档和调查字节；不要覆盖历史证据来让旧验证变绿。
- [ ] 更新当前 HANDOFF、必要入口、总体计划及 `E/reviews/current-task-status.json`，记实际设计变化、待审状态和下一动作；P1.1 接受保持，P1.2–P1.4 在 dsh 结果由用户带回前保持未接受。需求正文／基线绑定若受影响，按实际最新版本追加 successor 和正文归档，不覆盖 rev25 或重写旧验收；不得只为发卡新增科学结论。
- [ ] 写 `R1/DSH-REVIEW-HANDOFF.md`，使用“给你看的／给执行者看的”，供用户原文转交；列审查范围、文件摘要、前后 diff、设计断言、验证命令及证明边界，要求 dsh 分别回答 P1.2／P1.3／P1.4 和整体设计是否可接受。交付后停止，等用户带回复审结果；同卡整改可沿用本施工会话。

### 6 必要验收断言与证明边界

| 断言 | 本卡必须交出的设计证据 | 留待实施的真实证明 |
|---|---|---|
| 全部领域可由统一接口表达 | 公共类型及所有引用闭合；复杂相／群／张量／场／格式有明确单位、输入域、失败路径和消费方；关键未决项如实阻止完成声明 | 内核算法、codec 和渲染真实结果 |
| 已接受基础协议没有退化 | advance 贯通 Engine／KernelSession／Worker／CLI；ResourcePort 继承读取能力；保存封闭联合拒绝 save＋png 等非法组合 | 真实绑定、调度与服务端约束 |
| 并发和保存不串稿 | R7 快照后 R8 编辑、A 稿旧 ack 到 B 稿、另存失败、导出、资源 507／主 409／丢 ack 的逐步状态和反例预期；覆盖主提交前后取消 | 真宿主安全路径、文件读回、配额、竞争及新实例重开 |
| 数学和格式预期独立 | 非正交基底、所选周期像、ADP 基变换、FFT 相位／V／N、轴哨兵、字段往返等具名输入与独立预期；绑定来源和精度，不用未来生产实现生成期望 | 原件对照、完整科学／格式数值验收 |
| CA 接线设计可接手 | prepare／迁入／kind／构建／launcher／checks-asks／恢复的生产者消费方、签名、顺序、身份、失败及未来命令完整；源码／help 与运行事实分列 | 实际 worker、build／check、故意失败、截图／mechanics 和恢复 |
| 全部功能有实施归属 | 从唯一规格派生的 254 ID 主责唯一性和现有领域规则消费检查；每单元有可审样本、断言、前提、预算和命令 | 功能支持率、性能、目标环境和平台整链 |
| 设计成果可复查且范围守住 | 当前来源 SHA、旧封存抽取范围／完整性核对、前后文件、失败原始输出、路径映射和所有未运行项 | dsh 独立接受判断；本卡自验不能代替 |

本卡禁止靠 schema 通过、文件存在、HTTP 成功、退出 0、关键词数量、功能 ID 并集或一张截图宣告功能可用。验收针对设计能否独立交接；尚不存在的真实写入、读取与往返必须在未来实施合同里明确证明内容，不能由设计示例冒充已完成。

### 7 核验命令与预期结果

开工可执行的只读核对，cwd 为 ROOT：

```sh
cd /Users/agilecho/code/Draft
python3 - <<'PY'
from pathlib import Path
import hashlib, json
root = Path.cwd()
e = root / 'output/vesta-study/20261108-113622'
baseline = json.loads((e / 's01/baseline-rev25.json').read_text())
assert baseline['revision'] == 25
assert baseline['p1_design_status']['P1_1_complete'] is True
assert baseline['p1_design_status']['P1_complete'] is False
records = json.loads((root / 'docs/VESTA研究/functions.json').read_text())['records']
ids = [record['featureId'] for record in records]
assert len(ids) == len(set(ids)) == 254
cli = root / 'output/tooling/craft-anything-1f8508a1/debug/craft-anything-cli'
assert hashlib.sha256(cli.read_bytes()).hexdigest() == '99ad8c99cc5d7b35dfd85eb9b1a126ab7c074f3a5adbc7453b7a3d457d4a8869'
print('baseline=25; P1=partial; unique_features=254; locked_cli=match')
PY
```

该输出只核入口与身份。若后继 baseline 或功能目录确有已接受更新，记录新材料并据此接续；不要为满足断言回滚文件。新脚本与示例尚未创建，下列命令仅在步骤五完成后运行，不是现成产品测试：

```sh
# 在当前施工会话中，将变量设为步骤一实际创建的 R1 绝对路径。
python3 "$P1_EVIDENCE_DIR/verify_design_packet.py" \
  --root /Users/agilecho/code/Draft --evidence-dir "$P1_EVIDENCE_DIR"
```

预期：生成 R1 内的结构化报告，所有必检项通过才 exit 0；报告必须列失败、跳过、手工走查和未验证项。路径／SHA／主责归属／协议反例失败必须非零。此命令不证明产品或原生 Check 通过。

不自动重跑 R0／BK 中的旧 verifier；它们的绝对路径、封存输入和当前状态断言属于原轮。确需复用逻辑时，在 R1 明确修正定位、原判据和实际输入，保留旧失败及范围，不删断言或改旧包消除漂移。

### 8 停报与最终回执

遇以下情形，停止依赖该条件的步骤，继续能独立完成的设计，并及时报告：核心来源或继承裁定无法核实；公共接口必须靠未获授权的新运行才能决定；需要改变已确认行为／保存方案／技术路线；需要运行产品、prepare、worker、模型或修改平台；既有预算／费用／次数触顶；必须依赖未知许可、运行件或目标环境才可作承诺。

停报必须给出具体问题、证据、受影响接口／任务、已完成与可继续项、最小待裁定事项。不要只报“还有 unknown”，也不要在同一阻塞上反复试跑、自动补依赖或扩大施工。

最终回执固定含“给你看的／给执行者看的”。前者用业务语言说明设计实际做成什么、对以后实现的帮助、剩余缺口和是否待裁定；后者列修改文件与 SHA、实际命令／退出／报告、反例和原始证据入口、未运行项、设计偏差、继承账目、本轮消耗及可直接转交的 dsh 待审文件。状态写“Codex 自验完成，待用户转交 dsh 独立复审”或实际 partial；不得写“P1 已通过／可合并”。

本卡完成后停止。不得执行模块计划中的产品步骤，不自动创建后续任务或施工会话，不自动进入 P2–P5；用户带回复审结果后才据实际裁定更新接受状态。
