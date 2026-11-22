# Craft Anything 引导工具

这里维护 VestaDraft 的 `p1-ca-bootstrap` 源码和 P2-02 派生的 `Cargo.lock`。引导工具使用独立 Cargo workspace。Craft Anything 上游从提交 `1f8508a104134c436cbba5ea2f675619ef47702f` 取用。

用本机已有该 Git 对象的 Craft Anything 仓库，从干净的 VestaDraft checkout 准备独立源树。目标位置由调用方提供，父目录须存在。

```sh
bash packaging/ca/bootstrap/prepare-source.sh "$CA_REPO" "$NEW_SOURCE_DIR"
cargo +1.99.1 metadata --offline --locked --no-deps --format-version 1 \
  --manifest-path "$NEW_SOURCE_DIR/Cargo.toml"
```

脚本从 Git 对象导出锁定上游，再放入本目录的工具 crate 和派生锁。只有目标目录首次创建才写入。取得同一输入的另一入口是从原 P2-02 封存包解出 `ca-method-src/`。归档 `P2-02.prepare-source.tar.gz` 的 SHA-256 是 `c76627884a2807673c3ff9fd9d3bda0caa6c85ed805f98e0c9884347034920f6`。两条路径都保留上游文档、schema、测试和许可材料。 [原封存说明](../../../docs/任务卡/阶段2-首条完整链/P2-02-原时长续办.md)

后续离线构建须使用 P2-02 原卡认可的工具链、私有缓存与相应执行授权。正式工作区创建及已审源码安装继续由 P2-02 执行。 [现行开发与构建](../../../docs/开发与构建.md)
