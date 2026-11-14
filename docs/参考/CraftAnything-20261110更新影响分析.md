# Craft Anything 2026-11-10 更新影响分析

核验时间：2026-11-10 08:45（Asia/Shanghai）。对象为当前 Craft 文档与 VESTA Study；Codex 直接分析。本轮仅做更新影响分析、身份核验与文档维护，未升级工具、未重跑目标或启动产品实现。

**结论：现有锁定运行链未被本次源码更新替换，未发现需要回滚或整体重做 Study 的依据。当前 S04 可继续沿用既有运行件；若以后升级 Craft Anything，需要另建运行件并核验 Mach-O 解析差异和 rebuild 工作区兼容性。** 这不构成新版已编译、全部兼容或科学结论全部正确的声明。

## 对比基线

| 对象 | 本轮核到的事实 |
|---|---|
| VESTA 方法基线 | `1f8508a104134c436cbba5ea2f675619ef47702f`；保持不变 |
| 当前上游目录 | `/Volumes/T9/Development/Sources/Artcraft/craft-anything` |
| 当前本地 HEAD | `67b95106c2fbb79c57802857f7813d96cfb88c8d`；`main`，核验前后工作树干净 |
| 相对方法基线 | 204 个提交，239 个文件变化，17,229 行增加、1,411 行删除；行数含测试与文档 |
| 最近一次本地更新 | reflog 显示 11 月 10 日 08:08 从 `02e51b0` 更新至上述 HEAD；这一段为 166 个提交，不能与相对方法基线的 204 个混用 |
| 登记使用的研究 CLI | `output/tooling/craft-anything-1f8508a1/debug/craft-anything-cli`，仍为独立运行件；历史实际调用链另有回执，本轮只运行 `--version` |
| CLI 身份 | SHA-256 `99ad8c99cc5d7b35dfd85eb9b1a126ab7c074f3a5adbc7453b7a3d457d4a8869`，匹配 S03 登记；实际 `--version` 返回 0、版本 `0.4.0` |
| VESTA 原件 | S01 登记的主程序、libVESTA、Info.plist、LICENSE 四项摘要均匹配；不是整包重新验收 |
| 正式调查 | `vesta.cany` 的 1,845 个文件在本轮核验前后摘要一致；文件数不等于 evidence 数 |

可复核的[原始核验结果](../../output/research/craft-anything-update-20261110-67b9510/inspection.json)、[完整差异](../../output/research/craft-anything-update-20261110-67b9510/full-diff.patch)、[提交清单](../../output/research/craft-anything-update-20261110-67b9510/commits-since-method.txt)与[reflog](../../output/research/craft-anything-update-20261110-67b9510/reflog.txt)均已保存。锁定 CLI 的[实际版本回执](../../output/research/craft-anything-update-20261110-67b9510/locked-cli-version.json)不能单独区分新旧源码；本轮以摘要与提交共同识别。

“登记使用”有[历史原生探针 argv/退出回执](../../output/research/craft-anything-update-20261110-67b9510/registered-cli-callchain.json)支持；不表示本轮再次运行了该探针。迁移版本和旧版写回限制另有 Codex 实际 `git show` 取得的[源码选段与进程返回码](../../output/research/craft-anything-update-20261110-67b9510/compatibility-source-evidence.json)。它们属于源码核对，未做真实工作区迁移测试。

## 对当前研究的影响

| 领域 | 源码变化及依据 | 对当前项目的判断 |
|---|---|---|
| 原生观察与工具入口 | CLI `main.rs`、`apple_runtime.rs`、`native_ui_helper.swift`、`apple_tools.rs` 与所核 native 工具文件均与锁定提交相同 | 原有截图、AX、walkthrough 及函数工具的入口无需因这次更新改参数；既有工具限制仍按 METHOD 保留 |
| 证据与反汇编 | `crates/evidence`、`crates/disasm`、`crates/mcp`、调查 `project.rs`、`session_tools.rs` 未变 | 未发现 `.cany` 存储/证据身份或该反汇编器的直接迁移要求；依赖的格式解析器有变化，不能把这一项扩大为全部分析输出不变 |
| 方法主线 | `brief.rs` 中 `RULES` 与 `steps` 原文均未变 | 用户要求优先、证据/unknown、Study→Build→Check→修正的方法不需要重定；全功能/Web/平台接入要求不缩减 |
| 目标行为与科学结果 | 当前运行件、四项原件身份与正式调查在核验前后均保持 | 上游更新本身不推翻已经记录的原件行为；也不替代现有 S04 未核项或科学契约的补证 |
| Web 交付 | `apps/craft-anything-web/src/main.rs` 未变 | 本次更新没有改变既有轻量 Web 查看器边界，仍须按接入要求实现 VESTA 产品与平台链 |

以上“未变”均指 `1f8508a` 与本轮 `67b9510` 的直接比较，不是只看最近几个提交标题。

## 需要留意的变化

### 1. Mach-O 与签名解析修正：有潜在研究影响

这一组最接近当前 VESTA 的输入类型：

- `37a9285`：`LC_MAIN` 入口地址只映射到实际包含该文件偏移的 segment，避免零填充段误占入口。
- `fbdc304`：`LC_UNIXTHREAD` 按匹配 CPU 的完整 flavor/count/state 记录读取入口。
- `efddbde`：间接导入槽限额从各节分别计算改为跨节总预算；截断时保留 warning。
- `f7b985c`：拒绝短于声明布局的 Objective-C method 记录；`b6a5081` 排除 zero-fill section 的文件字节读取。
- 共享 `binary.rs` 也收紧文件偏移/虚拟地址映射范围，并按导入名称、库身份及槽地址的歧义处理导入槽关联；即使反汇编器本身未变，其消费的地址与导入元数据仍可能变化。
- `ae4b5cc`：修正 CodeDirectory 签名标志的位值与名称；例如 `library-validation` 应对应 `0x2000`。`runtime=0x10000` 位值未变。

**判断：这些变化可能影响重新解析得到的入口、导入、Objective-C 元数据及签名标签，不能宣称旧版所有静态输出都准确。** 本轮没有执行新解析器，尚无同一 VESTA 样本的新旧差分结果，也没有证据表明上述边界错误已在本项目命中。

已做限定溯源：正式 `vesta.cany` JSON 中未发现 `flag_names`、`force-hard`、`force-kill` 字面标记；该扫描不覆盖全部历史日志，也不证明所有签名相关语义均不受影响。已有 MADEL hardened-runtime 判断另有系统 `/usr/bin/codesign` 原始回执，见[当时的签名记录](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-codesign.json)，不依赖本次修正的错误标签。

继续使用旧运行件时，涉及这些字段的新结论应交叉核对系统工具或实际原件行为；若升级，则先在副本上比较目标主程序、libVESTA 与 MADEL 的相关输出，仅追踪实际发生变化的下游结论，不全量重做研究。

### 2. rebuild 工作区 v3→v4：升级时需要兼容验证

`crates/engine/src/workspace.rs` 的 `MANIFEST_VERSION` 从 3 升至 4，新增保存 Agent 会话的模式 `agent_session_deep`。旧会话缺模式时按 Deep 迁移，Clean room 不复用 Deep 会话；相关改动包括 `0e7e43f`、`0326b4d`、`7a38723`、`cf2a8ad`。

这是 rebuild 的 **`craft.json`** 版本变化，不能与当前调查 **`.cany/project.toml`** 混同。本项目尚未启动原生 rebuild；目前没有这条会话迁移链需要执行。

将来启用新版时应复制 rebuild 工作区验证迁移。锁定旧源码对超出支持版本的 manifest 采用只读或拒绝读取路径，因此新版保存后的工作区不能假定仍可由旧版写回。保留旧运行件与旧工作区是升级/回退验证的前提。

### 3. Deep 续跑与 Check 编排：主要影响后续 Build/Check

- `a4f1ee8`：后续 Deep build 重新打开研究目标，避免新 builder 的原生工具遇到“尚未打开目标”。这里是分析目标打开，不等于启动 VESTA GUI。
- `1b4352c`：跨请求保留已反编译状态，减少重复反编译提示；不能以该状态位证明科学算法完整或准确。
- `5870bce`：检查所依赖内容不变且此前检查成功时复用检查结果；复用不代表又执行了一次原件运行。
- `28cccdb`、`eb03911`、`c2487bb`：补齐 Check 的期限、请求处理中标记与停止后的答复。
- `43f4a06`：续跑 API builder 使用其循环可执行的检查路径；`c813f6b` 按完成流事件结束等待外部 builder。

这些改动可改善未来自动构建的恢复与等待行为。本项目现在仍以手工调用真实 Study 工具为主，不能把新版修复当成本轮已经启动 builder/Check 或已通过全功能验收。今后升级应分开记录“新执行的检查”和“复用检查”。

`brief.rs` 的非 `RULES/steps` 部分还增加按 builder 是否能看图分流的提示：不能看图的后端转用布局测量/UI 树等文字材料。`study.rs` 除上述 Deep 重新打开目标外，也调整了窗口截图说明中的读图措辞。方法骨架相同并不代表完整 prompt 或 Study 实现相同；本轮只核源码差异，未实际运行这些模型视觉路径。

### 4. Agent、模型与其他格式：当前链路无直接替换

其余更新包括本地模型发现/连接、Agent 身份与残留会话清理、SSH 工具服务器路径，以及 SQLite、压缩包、游戏和其他二进制格式边界修正。它们属于后续使用相应功能时的升级收益；本轮运行链未切换到这些后端。dsh 独立复审职责仍按本项目 AGENTS 执行，不由上游 Agent 支持清单改变。

`Cargo.lock` 也发生变化：`craft-anything-formats` 的依赖列表新增 `semver`，对应其 `Cargo.toml` 新增 `semver = "1"`，见上述源码证据。后续新版构建须使用对应提交及其锁文件，并验证依赖解析、构建与运行件摘要；不能把旧 CLI 的构建成功沿用为新版证明。本轮没有更新本项目的锁定依赖或重新构建。

## 当前处置与后续入口

1. **保持现有 Study 的方法提交与 CLI 摘要。** 后续源码解释继续用 `git show 1f8508a104134c436cbba5ea2f675619ef47702f:<path>` 或已登记快照；活动目录只作为定位入口。当前目录的行号/源码内容不能冒充锁定证据。
2. **现有 Study 状态沿用。** 本次分析不调整功能数、完成项、unknown 或 S04 裁定，也不进入下一卡或产品实现。
3. **升级作为后续单独工作。** 如要升级，在独立位置构建新版并保留旧二进制；对副本验证命令/help、Deep triage、Mach-O/签名关键字段、evidence/unknown 往返。原生 rebuild 首次启用前还要验证 v4 工作区、模式续接及 Check 停止/复用语义。本轮未执行这些升级工作。
4. **旧参考报告继续标注历史基线。** [项目分析](CraftAnything-项目分析.md)与[依据登记](依据与差异登记.md)仍以 `078cf0c4842c` 为历史分析；其中工具/代码规模和旧缺口不能按当前 HEAD 解读。本报告是增量影响分析，没有把那些统计静默改成“最新已验收”。

## 核验与限制

[核验脚本](../../output/research/craft-anything-update-20261110-67b9510/inspect_update.py)实际退出 0。已核 Git 差异、关键源文件摘要、构建运行件身份、四项目标身份与正式调查前后完整文件摘要；没有运行新版 Cargo 构建/测试、新解析器差分、目标 GUI、原生 rebuild 或迁移。不能据静态修复记录宣称新版在本机可运行或已与 VESTA 全面兼容。

独立复审：dsh 首轮达 240 秒期限，未返回最终裁定；原请求、stdout/stderr 与[进程回执](../../output/research/craft-anything-update-20261110-67b9510/dsh-review-run.json)保留，不能因停止后 exit 0 称为复审通过。随后基于随附报告与 `inspection.json` 的限定材料复核返回 **pass（限定范围）、无硬阻断**，见[复核原文](../../output/research/craft-anything-update-20261110-67b9510/dsh-final-review.stdout)与[正常完成回执](../../output/research/craft-anything-update-20261110-67b9510/dsh-final-review-run.json)。该次复核未调用工具，不等于独立运行源码或目标测试。其四项修订要求已补充模型视觉提示、来源层次、锁文件变化与“无强制重做依据”的限制，见[处置记录](../../output/research/craft-anything-update-20261110-67b9510/review-followup.json)；修订后文档由 Codex 自验，未再作一轮完整独立源码复审。
