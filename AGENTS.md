# 全局协作约定

## 角色与交付方式

- Codex 默认直接执行编码、验证、施工验收、维护文档并提交执行证据；只有用户明确要求时才交由 deepseek-harness（简称 dsh）执行。
- 独立复审统一由 dsh 承担；后续不再使用 Hermes 复审口径（2026-11-08 用户确认）。
- 用户已确认的裁定要回写权威文档并沿用，不重复请示。先完成已授权的工作，不擅自扩大为实现。

## 开发前阅读与参考

- 涉及开发或编写代码前，先读取当前任务卡、最新续办及相关权威文档，并参考 `/Volumes/T9/Development/Sources/Artcraft/wordcraft`（用户于2026-11-14确认）。
- 按任务选择 wordcraft 的 `AGENTS.md`、`README.md`、`docs/architecture.md`、`docs/testing.md`、`docs/recovery.md`、`docs/control-protocol.md` 等相关文档及对应源码；记录实际参考文件、HEAD及必要SHA，区分工作区改动与提交版本。
- 参考结论按 Draft 已确认的路线、接口与宿主约束取舍；具体编码仍由对应独立施工卡授权。

## 源码与证据位置（2026-11-19用户确认）

- 用户明确指定代码直接放在 `/Users/agilecho/code/Draft`，与文档使用本项目现有 Git 仓库。日常应用源码为本仓库 `app/`，引导工具在 `tools/ca-bootstrap/crates/p1-ca-bootstrap/`；后续源码在本项目维护。
- `output/` 继续保存原始证据；已封存的源码树、归档和账目保留。现行源码不再只存于被忽略的 `output/`。目录裁定与精确入口见 [源码项目目录调整](docs/任务卡/源码项目目录调整.md)。
- 原 `/Users/agilecho/code/VestaCraft` 正式 CA 工作区仍未创建；新源码项目存在不替代 P2-02 的正式 API 创建、已审源码安装和验收，不重置预算或解除原停报。

## P2-01本轮Git施工决定（2026-11-19）

- 用户明确要求先提交现有代码，再直接在main办理本卡整改，当前不使用开发分支。基线067ee95、整改0818939；精确授权及原预算边界见 `docs/任务卡/阶段2-首条完整链/P2-01-独审整改续办.md`。此项只登记本轮具体要求，不变更其它任务的默认分支规则或dsh独立接受权限。

## 仓库名称与远端（2026-11-19用户确认）

- GitHub公开仓库名为VestaDraft，地址：https://github.com/agilecho/VestaDraft；origin已指向该仓库，main已推送到0818939。旧远端保留为agilecho2022-origin。
- 用户表示整理后也会将当前项目改名为VestaDraft；当前本地目录仍为/Users/agilecho/code/Draft，本地目录和历史证据路径尚未改名。施工保持用户要求的暂停状态。
