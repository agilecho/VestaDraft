# Craft Anything 应用打包入口

本目录的 `package.json` 安装到 Craft Anything 工作区的 `app/package.json`。同版产品源树放在 `app/source/`，包含根 Cargo workspace 和产品自己的 `package.json`。正式创建和已审源码安装按 P2-02 办理，引导工具见 [bootstrap](bootstrap/README.md)；新布局的双端构建和制品验收归 P2-04，worker 与 Check 按各自卡片执行。

锁定的 Craft Anything 提交为 `1f8508a104134c436cbba5ea2f675619ef47702f`。其 `crates/ai/src/rebuild/check.rs` 中的 `build_check` 在普通应用上按以下入口选择：

| 检查位置 | 命中的命令 | 当前范围 |
|---|---|---|
| 开发仓库根同时有 Cargo.toml 和 package.json | `cargo build --quiet` | Cargo 原生构建路径 |
| 适配副本的 `app/` 有 package.json，Cargo.toml 位于 `source/` | `npm run build` | 委托到同版源树的完整双端构建脚本 |
| 当前适配副本，源树仍只有合同工具 | `npm run build` 返回失败 | 待 P2-04 实现源树的 `build` |

从一个确定的布局提交准备本机适配副本：

```sh
# APP_COPY 是本机测试所有的空目录，例如临时目录中的 app/。
mkdir "$APP_COPY/source"
git show "$PRODUCT_REF:packaging/ca/package.json" > "$APP_COPY/package.json"
git archive "$PRODUCT_REF" .cargo Cargo.toml Cargo.lock rust-toolchain.toml \
  package.json package-lock.json tsconfig.contracts.json toolchain.inputs.json toolchain.sources.json \
  apps assets contracts crates scripts tests xtask | tar -x -C "$APP_COPY/source"
```

CA 在 `app/` 执行 `npm run build`，适配入口从 `source/` 执行 `npm run build -- --output-root ..`。P2-04 的源树入口应从该版本的 Cargo.lock 构建原生 CLI 与共享内核 WASM，使用匹配的 wasm-bindgen，再运行 TypeScript、Vite 及交付清单生成。命令以 `source/` 为 cwd，`--output-root ..` 把制品写至外层 `app/`。新增的真实构建输入按 P2-04 的最终消费图加入安装清单。

完整链验收读取实际 `app/dist/index.html`、其引用的前端、Worker 和 WASM、`app/dist/cli/vestacraft`、构建清单及唯一 `app/Build/vestacraft-preview`，核同一提交、两份锁、工具、资源和制品字节。锁定 CA 的 `crates/engine/src/outputs.rs` 优先找有 index.html 的 `dist/`。目前通过适配副本只可验证 npm 选路和缺完整构建的失败行为。普通 CA 构建窗口为两分钟，真实冷构建及后续 Check 计时按原卡记账。
