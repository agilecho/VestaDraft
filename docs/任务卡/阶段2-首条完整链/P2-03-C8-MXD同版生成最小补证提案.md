# P2-03 C8 MXD 同版生成最小补证提案

**提案待批。本轮仅静态核对。** 目标是用冻结版 NIMS MXDORTO 串行源码产出一份新的真实 FILE06，包含 COORDN 完整统计段和可追溯的 XD 作业上下文。先拟一支 ortho，tricl 的真实厂版正例保持原资格与后续条件。 [原 C8 提案](P2-03-C8全类型与资源闭合最小续办提案.md) · [NIMS 官方例件](https://www.nims.go.jp/personal/mxdorto-mxdtricl/examples.html)

## 给你看的

归档没有历史 FILE10，现有材料可以配出一套**新的工程 XD 作业**。官方 Ni 示例的 500 个初始坐标逐项等于参考坐标，正好覆盖 5×5×5 个面心立方常规晶胞。取完整的 2×2×2 区域，得到 7.048 Å 的 32 粒子盒。选用显式 P1 参考，每个粒子有自己的编号，原编号与新编号可核对。已在内存中复算并逐字段读回拟用 FILE10，未生成作业文件或报表。 [32 点与固定列核验](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-c8-producer-evidence-plans/file10-feasibility.json)

拟 **1 次离线编译 ≤300 秒、1 次同版生产运行 ≤180 秒，合计 ≤480 秒**。使用已有且验证过可离线工作的 Fortran 环境。当前 macOS PATH 未找到适用工具链，编译环境未齐。已合资格的同版串行制品若可取得，本方案可省去编译，只保留 1 次生产运行。

## 给执行者看的

1. **固定来源与环境。** 从 [封存 L-J 官方包](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-c8-archived-producers/nims-archives/L-J.zip) 取启动用 `L-J/file05.dat` 与 `L-J/file07.dat`。锁定 [NIMS ortho 20251120 串行源码](/Users/agilecho/code/VestaDraft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/inputs/mxdorto-serial.f90) 及原发布包 SHA。在预装且可禁网的兼容 Fortran 主机或 Linux x86_64 客体，预先固定 `ifx`、链接器、运行库、版本和摘要。厂方串行脚本是 `ifx -O2 mxdorto.f90 -o ./bin/mxdorto`。在空的私有目录等价执行一次，留模块、编译输出、制品 SHA 和退出码。工具链配置、安装或联网获取另有前置授权，未满足时此步不启动。 [厂方源包](https://www.nims.go.jp/personal/mxdorto-mxdtricl/src/mxdorto.zip)
2. **配对新作业。** 新根下以归档起始 FILE07 的固定列 `7702` 区分位置、参考位置、速度和缺陷字。选取三个原分数坐标均小于 0.4 的完整晶胞块，维持原粒子顺序，坐标与参考坐标乘 2.5 并使用新盒长 7.048 Å。保留已选起始速度词与元素，独立核对 32 个粒子按新 1…32 重编号。把 FILE07 对应的粒子数、Ni 物种计数、范围和盒参数一起更新，检查所有余下起始计数及记录宽度。原词与涉及变更均做前后字节审计。FILE05 用 `XD.......I`、同一个新标题、`START`、Ni 数量 32 与归档 L-J 参数。使用 `NOACCUM` 和固定盒、关闭压力与温度调节的短工程作业。拟 `IRECRD(1..5)=[10,10,5,5,5]`，源码的记录及取模条件指向第 10 步。以输入和实际控制台核首个统计间隔。 [作业模式与 FILE10 阅读源码](/Users/agilecho/code/VestaDraft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/inputs/mxdorto-serial.f90:1275)
3. **FILE10 独立校核。** 采用显式 P1、`NBOX=[1,1,1]`、`NPT=NPTP=MATM=32`、`NSYM=1`、空 HEX。前两行依次为盒长及角余弦、复制数等计数。设 A4 标记 `N001…N032`，明确都关联 Ni。每个 `NIU=1`，依次写入 32 条 `JON/P0C`，按原固定列写单位 `RS` 与逐粒子 `ISYM=1`。逐粒子把 `JON` 对照新 FILE07 行和旧原件序号，将参考列精确回读到同一坐标。每个统计组一条贡献，P1 映射无按显示名称合并。全部参考分数位于 [0,1)，避免代码的另一组跨胞归并。源码 `LAA=172`，相同的一点一标记方法无法直接容纳原 500 点。内存预览已验证这一 FILE10 的长度、精确字段及 SHA。获准后才将配对的三个文件固化进同一个新作业。历史原件仍按普通 MD 原字节保留。 [独立固定列和行映射](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-c8-producer-evidence-plans/file10-feasibility.json)
4. **一次真实生产与验收。** 编译、输入核对齐备才从新根启动串行制品一次。禁网，终端输入关闭，禁止共享或写回原例件。保存 PID、真实 argv、版本、编译／生产耗时、退出、标准输出／错误、前后文件列表以及所有产物。必须有完整、未重写的 FILE06。独立从原字节定位厂版头、完整 COORDN 段、8 组 × 4 标签 × 三轴、每轴均值／括号统计量／参考值，以及参考列和作业内所有 32 个身份的对应。记录有序行跨度与 SHA。核有限数值、字段宽度、缺行、重复、溢出星号与错误元素，并确认当次盒长、复制数、操作和真实统计样本计数的 provenance。括号量按冻结合同保留无量纲分数统计，原文字面 `A^2` 保留。可随实际生产者产物核对精确读取和后续保存重开资格。 [冻结统计合同](/Users/agilecho/code/VestaDraft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/final-seal/core/mxd-statistic-consumption.contract.json)
5. **停止条件。** 缺编译环境、确定的粒子对照、同次作业证据或已批准单列额度，相关目标不启动。实际运行有超时、提前退出、产出错误或报表缺项时保留原件和失败快照，清场并停止。拟重试数为 0。只计一次编译和一次生产，没有追加获取、无 Cargo 或产品组。可合资格的结果交用户手动转 dsh。运行后另一分支、科学历史及原卡联合覆盖继续按同一资格矩阵裁定。

**历史 FILE10 结论。** 对六个封存示例包共核 16 份 FILE05，均为 MD 且缺 FILE10。原 FILE07 给出盒和粒子位置及参考位置，未留旧 XD 的具体复制、参考单元、操作与粒子绑定。以上是重新定义的新 XD 作业与来自原例件的完整数值映射。它的科学历史身份为工程正例，真实 FILE06 的生产者身份须用后续原版编译和运行建立。 [归档负证据](/Users/agilecho/code/VestaDraft/output/vesta-p2-03/20261121-c8-archived-producers/archive-check.json)
