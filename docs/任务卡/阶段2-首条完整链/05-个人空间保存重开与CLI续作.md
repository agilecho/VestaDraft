# P2-05 · 个人空间保存重开与CLI续作

日期：2026-11-18。卡面v3，按规则v1.4补齐的文档后继；待用户手动dsh复审，未发放施工、未执行。原v2的卡面接受按原SHA保留，不移用于本后继。

## 给你看的

在隔离宿主中把实际晶体工程保存到本人空间，导出、关闭，再由CLI续改并在全新页面重开继续编辑。

本卡覆盖：首链vesta+资源/CIF、真实私有宿主保存与操作登记、close barrier和NS2生命周期交接；交第一条完整用户结果。

不改共享平台、不接真实模型、不把首链称254功能/85格式全部通过；正式平台集成由P4复用这些真实补丁。 依赖未就绪时可先做不依赖它的已授权部分，完整结果未成立则交partial。

## 给执行者看的

### 1. 接手、权威依据与实际基线

必须完整读[后续施工共同约定](/Users/agilecho/code/Draft/docs/任务卡/后续施工共同约定.md)、本卡、[阶段入口](/Users/agilecho/code/Draft/docs/任务卡/阶段2-首条完整链/README.md)及最新续办。共同约定是本卡的一部分，含绝对根、权威顺序、输入SHA、资源/停报/回执规则；先核真实状态，再动依赖步骤。

需求依据：总体计划P2.3及首链通过条件；SW-4/5/6/7、PRD新建目录D4-31；技术方案§9.6、C2/C3/C5/C15/C16；FC/HS4/NS2。

实际基线：P1资料rev67、已审技术rev66/D35/I33-r1/IP6，wire/profile I33。Draft HEAD为754d06d9d5d1dcafd30ba951105c16b73644abe9且有继承WIP；正式VestaCraft根当前不存在，产品源码/产品HEAD/锁/构建/runtime均未产生。未来接手必须读取前卡真实产物而非沿用此开工前快照。Q/input-manifest.json固定本次所读原字节，未来核漂移。

| 当前精确设计入口 | 本卡消费方式 |
|---|---|
| T66/implementation-readiness.json `/units/2`（U02） | contractIds：`FC-format-profiles, FC-full-source-audit, FC-resource-graph, FC-resource-reopen, FC-resource-content-comparison, FC-save-artifact-binding, FC-typed-source-restore, FC-save-target-policy, FC-save-settlement, HS-base, HS4, AL3-host-profile, AL3-operation-recovery, NS1, COMPOSITION2, HISTORY2, PLANNED_FILES, CURRENT_ORACLES, UNIT_RESULTS, NS2, PROFILE2, RESOURCES2, GENERATION`；按该单元contractBindings的path/SHA/version读实际文件 |
| T66/implementation-readiness.json `/units/13`（U13） | contractIds：`FC-format-profiles, FC-codec-field-policies, FC-roundtrip-assertions, FC-full-source-audit, FC-auxiliary-io, FC-data2d-text-export, FC-resource-content-comparison, AB, SG2, FS2, AB2, EX3, COMPOSITION2, PLANNED_FILES, CURRENT_ORACLES, UNIT_RESULTS`；按该单元contractBindings的path/SHA/version读实际文件 |
| T66/implementation-readiness.json `/units/14`（U14） | contractIds：`BT, CA, EX, HS4, AB, AB2, EX3, COMPOSITION2, HISTORY2, PLANNED_FILES, CURRENT_ORACLES, UNIT_RESULTS, NS2, PROFILE2, RESOURCES2, GENERATION`；按该单元contractBindings的path/SHA/version读实际文件 |
| T66/domain-consumption.json、current-oracle-bindings.json、unit-result-assertions.json | 按所列单元的实际domainBindings/publicBoundaries匹配现有group，独立预期依当前绑定；应用/交付断言按unit-result-assertions的unit匹配，不虚构域名表；predecessor只作历史 |
| T66/planned-files.json、format-implementation-bindings.json | canonical文件和85路由当前handler；两条native路由只绑native_vesta_v2.rs |

#### 原型与流程依据

采用[原型与验收依据](/Users/agilecho/code/Draft/docs/任务卡/原型与验收依据.md)的 **FILE/EDIT/HOST**（版本、页面/流程、原始证据及SHA入口见该文）。本卡采用：本人空间Save/Save As/Export、dirty、关闭三选项、全新页面重开；保留原件桌面行为与平台保存差异。 对应PRD/技术方案仍按本节既有引用执行。原v2/v3接受仅属原卡面，当前状态见阶段README与HISTORY；不得用旧回执替代原型或需求。

### 2. 唯一结果、真实依赖与步骤

唯一交付结果为本卡首段所述完整用户/工程结果；不因脚本成功或部分源码完成自动登记整单元。

| 依赖产物/条件 | 何时需要、缺失如何处理 |
|---|---|
| P2-03真实模型/codec/NS2与P2-04Web/Linux制品 | 核当前源码、制品及runtime一致；不复用旧构建 |
| 真实隔离宿主principal/SpaceFiles/配额/只读/操作登记输入 | 运行前落成绝对根和真实鉴权；合成Ack不能验Saved |
| FC/HS4/Source7完整内容比较合同 | 逐字段/资源消费；现行native handler为native_vesta_v2 |

承接IP6步骤及原始依赖（源码、资格、完整结果分别验；不是按卡号串行）：

| 步骤 | dependsOn原值 |
|---|---|
| `U02.host-source` | `U02.save-state`、`U14.cli` |
| `U02.host-qualification` | `U00.build`、`U02.host-source`、`U14.linux-qualification` |
| `U13.first-codec` | `U00.build` |
| `U02.web-close-reopen` | `U01.current-page`、`U02.host-qualification`、`U13.first-codec`、`U14.linux-qualification` |
| `U02.close-reopen` | `U02.web-close-reopen`、`U14.native-stateful-binding` |

### 3. 文件、实施顺序与交付物


P2只在本卡RUN持有的隔离宿主副本实现`vesta-{files,coordinator,page,tools}.mjs`，副本按BranchDraft实际HEAD/WIP及逐文件SHA导出；不得改活跃BranchDraft插件或挂真实用户资料。复用实际SpaceFiles/鉴权/路径/存储层配额，不另建绕过权限的裸文件服务；测试principal与文件系统须是真实本卡隔离实例，能力不足停对应资格，不用mock确认代替。

本卡APP内计划文件及归属：`web/src/host/{save-state,page-api}.ts`和`kernel/crates/commands/src/{snapshot,source_capture}.rs`消费/补齐P2-03已有保存结果；`kernel/crates/codecs/src/formats/{native_vesta_v2,cif}.rs`负责首链真实保存/导出所需增量。命令中的`tests/contracts/save-state.test.mjs`、`tests/contracts/bundle-plan.test.mjs`、`tests/codec/profiles.mjs`、`tests/codec/artifact_roundtrip.mjs`、`tests/browser/native-lifecycle.spec.ts`均由本卡在APP新建或接手已有真实测试并补齐，非RUN占位driver；codec用例本卡覆盖首链路由，P3-11扩为85路由完整回归。`test:delivery:ns2`继续调用P2-01/03已有实际三份合同测试。隔离宿主副本的源码/来源清单与运行日志留RUN，不迁入共享平台。

精确场景见IP6引用的first-chain-casebook.json（R00/R01）。它是I31时期首链场景材料，当前运输一律NS2/I33，不能照其历史NS1类型降级。按new 64→a5 80→a6 96→undo80→redo96→noop→save中a7/a8、CIF、关闭、CLI、新页面的真实动作/内容断言验收；完整revision/保存时序从原casebook逐行消费，不手造expected进程ID。

本卡必须覆盖资源落盘/缺失/错误内容反例，即使简单晶胞本身没有体数据，也须沿合法fixture/资源生产入口得到真实依赖；不能在驱动里补写最终产物。多资源完整内容读回与确认采用FC/HS合同，任意外部写者CAS保证不得超出已证适配边界。

实施顺序：核授权/输入/真实依赖→固定独立expected并留必要RED→实现本卡业务及贯通适配→运行下表必要检查和失败反例→真实内容/生命周期验证→保存源码/锁/制品/runtime及原始日志→更新状态并交手动dsh材料。只有卡面所列消费者输入缺失才停依赖部分；不擅自改需求、黄金值或老封存。

### 4. 关键验收断言与原始证据

| 验收项 | 必须证明的结果 |
|---|---|
| 真实保存 | 主文件和全部资源真实写成，独立artifact-only reader逐字段/数值/资源读回才确认Saved；不以HTTP/哈希单独证明 |
| 并发编辑 | 保存旧snapshot期间新编辑保持dirty；迟到真实响应不清新revision、不绑新doc；SaveAs失败保留原身份 |
| 导出独立 | 当前a=8导CIF，主文件仍a=7，导出不清dirty/不改WorkRef/不下载；独立解析CIF得到128 Å³ |
| 真实关闭 | 结清所有可能发布操作才关闭，dispose旧page/Worker/Engine并清内存资源；新实例只读真实WorkRef及依赖 |
| 续作身份 | 关页后一次NS2 serve载入并使用新ID，保存实际续改；再次新页面读回并继续编辑，旧请求/Consent拒绝 |
| 失败恢复 | quota507、外改409、publish unknown、只读/越根/symlink、坏资源均保留旧工程/dirty；只能查询同op恢复，不盲重放 |

每项同时保留真实输入/操作/输出与独立预期来源、完整内容比较、原始失败及版本身份；公共写入链按共同约定核实际写/读/销毁/重开/续改。只验类型、计数、文件存在、HTTP成功或自读自写不能完成本卡。

### 5. 必要命令、前提与失败处理

#### 受影响测试选择

save-state/bundle-plan、首链native与CIF的真实读写/失败、native-lifecycle与CLI交接；不默认跑其它83条路由，保存共享逻辑改变扩到真正受影响格式。

先按共同约定§5.1落`<RUN>/inputs/affected-tests.json`。下列为首次新增结果的候选命令；卡内小修使用实际测试定义的用例过滤、明确文件/目标和非零选中清单，记录预期与未受影响项不跑的理由。正例须证明本卡真实结果，反例须拒绝且不污染当前工程；失败/超时/0选中均不算通过。纯文档只做相关静态核对。本卡不是阶段收尾时，其单卡界面测试不替代阶段整体UI E2E。



下面全部是**未来待实现/待运行**命令，当前未执行。npm/cargo命令cwd=APP；RUN driver从卡面实际受管根调用，其源/输入/工具版本须锁定。先有真实源码/脚本/锁/包/目标资格再运行，不创建占位成功出口。命令内的占位必须在开工输入落成实际绝对路径。

```text
node --test tests/contracts/save-state.test.mjs tests/contracts/bundle-plan.test.mjs
node --test --test-name-pattern '^(VM18-I01|OBS-O01|VM18-I06|OBS-O06)[.]' tests/codec/profiles.mjs tests/codec/artifact_roundtrip.mjs
npm run test:browser -- --grep native-lifecycle
```

仅在共同约定§5.1列出的共享合同/生成器/transport/交付绑定实际受影响时追加完整`npm run test:delivery:ns2`，不作为每次业务修正的默认命令。

codec用例由本卡新建/复用时按上述真实路由ID前缀命名，首链仅native/CIF；若共享保存/恢复变更确实波及其它格式，把它们加入实际选择清单，不静默漏测。

测试名/grep须绑定真实测试定义且非零选中；没有选中不能算通过。顶层失败状态原样传播。编译/单元测试只证明对应层；真实用户链必须另留当前runtime、画面/状态、持久内容和新实例证据。若需新增源/依赖/环境，仅按本卡未来授权及共同约定拟定上限取得；不静默换版本或联网补件。

### 6. 授权、停报与回执

本轮仅修订卡面和管理文档。未来明确执行本卡后，允许范围限本卡文件/结果/必要检查；资源上限和每次计量执行共同约定§6，旧同卡累计/更严限额优先。新增原件/外算/provider/CA/模型/目标付费资源必须有具体授权输入；缺者停对应分支，保留未执行，不把其它项目权限或旧卡余额继承成新额度。

输入/类型关键冲突、原生根冲突、未知写者、环境或剩余额度缺失、超时/越额时停相关步骤并交最小复现；保留已完成及可继续项。不得自动启动下一卡或清别人的WIP。回执按“给你看的／给执行者看的”交真实结果、版本、完整证据/失败/限制/未验/设计偏差及DSH-REVIEW-HANDOFF；只由用户手动转交dsh，Codex不联系或启动复审会话。

不从发卡推导stage/commit/merge/push/发布/部署权；已存在的具体授权按其范围执行，不重复询问。独审通过仅接受本卡已证明结果，阶段/整产品接受须满足其余真实判据。
