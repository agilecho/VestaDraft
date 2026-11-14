# 样本与独立预期

当前任务对齐（2026-11-10）：S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 当前 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open，即14个未解决；来源 revision10、数据库 revision4 保持。本次状态对齐没有新增目标证据或产品代码。 既有结果与失败边界保持；从 [HANDOFF](HANDOFF.md) 接续。

稳定样本 ID、相对证据路径、SHA-256、来源、功能 ID 和独立预期保存于 [samples.json](../../output/vesta-study/20261108-113622/s05/samples.json)。ID 不复用。样本准备、原件读取和数值验收分别记状态；官方样本的“独立预期待核”不能当已验证。bvparm2020.cif 是键价参数表，不在晶体样本中。

官方来源：[VESTA 下载页](https://jp-minerals.org/vesta/en/download.html)的 Example input files，包地址为 `https://jp-minerals.org/vesta/archives/VESTA-Examples.zip`。本轮下载退出 0；ZIP SHA-256 为 `ddce7ae1e2e8dd8705e69fd81a57fc242226d35d9ee1a747e986c83a83c00895`，142 个成员、未压缩 39,282,069 字节，解包前检查无绝对路径和 `..` 越界。保留本地研究副本与[成员清单](../../output/vesta-study/20261108-113622/s05/official-examples-inventory.json)，未当作产品可再分发资产。

合成非正交输入的独立预期来自晶胞度量与 B/U 定义：体积 `12√3=20.784609690826528 Å³`；两位点直接距离 `√4.75`，周期最近距离 `√1.75`；O 占位 0.5、Biso=0.5，对应 `Uiso=0.5/(8π²)`；X 射线 F000=6+0.5×8=10。预期在运行前写入 [synthetic-independent-expected.json](../../output/vesta-study/20261108-113622/s05/synthetic-independent-expected.json)。POSCAR 样本是满占位输入，不能与半占位 CIF 当同一电子输入。

| 样本 ID | 文件 | 预期/状态 |
|---|---|---|
| VS-0001 | [temporary-C1.vesta](../../output/vesta-study/20261108-113622/s04/temporary-C1.vesta) | cell=[4, 4, 4, 90, 90, 90]; volume=64; sites=1 |
| VS-0002 | [temporary-C1-hidden.vesta](../../output/vesta-study/20261108-113622/s04/temporary-C1-hidden.vesta) | cell=[4, 4, 4, 90, 90, 90]; volume=64; sites=1 |
| VS-0003 | [temporary-C1-restored.vesta](../../output/vesta-study/20261108-113622/s04/temporary-C1-restored.vesta) | cell=[4, 4, 4, 90, 90, 90]; volume=64; sites=1 |
| VS-0004 | [temporary-C1-density.vesta](../../output/vesta-study/20261108-113622/s04/temporary-C1-density.vesta) | externalResource=temporary-C1-density.pgrid; expectedReopen=structure and volume; observed unresolved failure |
| VS-0005 | [temporary-C1-density.pgrid](../../output/vesta-study/20261108-113622/s04/temporary-C1-density.pgrid) | dimensions=[8, 8, 8]; F000=6; integralExpected=6 |
| VS-0006 | [density-absolute.vesta](../../output/vesta-study/20261108-113622/s05/density-absolute.vesta) | volume not restored in observed GUI Open |
| VS-0007 | [density-short.vesta](../../output/vesta-study/20261108-113622/s05/density-short.vesta) | volume not restored in observed GUI Open |
| VS-0008 | [C1-export.cif](../../output/vesta-study/20261108-113622/s05/C1-export.cif) | cell=[4, 4, 4, 90, 90, 90]; sites=1; site=C1/C/0,0,0/occupancy1 |
| VS-0009 | [nonorthogonal.cif](../../output/vesta-study/20261108-113622/s05/fixtures/nonorthogonal.cif) | cell=[2, 3, 4, 90, 90, 60]; volume_angstrom3=20.784609690826528; asymmetric_sites=2 |
| VS-0010 | [POSCAR](../../output/vesta-study/20261108-113622/s05/fixtures/POSCAR) | POSCAR sample intentionally lacks the CIF half occupancy and ADP; not equivalent electronic input |
| VS-0011 | [pair.xyz](../../output/vesta-study/20261108-113622/s05/fixtures/pair.xyz) | atoms=['C', 'O']; CartesianCoordinates=[[0, 0, 0], [2, 0, 0]]; distanceAngstrom=2 |
| VS-0012 | [非正交晶胞.cif](../../output/vesta-study/20261108-113622/s05/fixtures/中文%20路径/非正交晶胞.cif) | cell=[2, 3, 4, 90, 90, 60]; volume_angstrom3=20.784609690826528; asymmetric_sites=2 |
| VS-0013 | [truncated.pgrid](../../output/vesta-study/20261108-113622/s05/fixtures/truncated.pgrid) | errorExpected=incomplete 512-float payload; exact original behavior remains to observe |
| VS-0014 | [bad-cell.cif](../../output/vesta-study/20261108-113622/s05/fixtures/bad-cell.cif) | errorExpected=invalid positive cell-length constraint; exact original error/state remains to observe |
| VS-0015 | [Al2O3.cif](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/CIF/Al2O3.cif) | From input fields; symmetry expansion still to verify independently |
| VS-0016 | [KOD.vesta](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/VESTA/Volumetric_data/KOD/KOD.vesta) | project volume absent; grid alone loaded |
| VS-0017 | [KOD.pgrid](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/VESTA/Volumetric_data/KOD/KOD.pgrid) | project volume absent; grid alone loaded |
| VS-0018 | [Adularia-8twins.vesta](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/VESTA/Morphologty_data/Adularia-8twins.vesta) | phaseCountFromFile=8; shape=eight CRYSTAL blocks; numeric shape/interfacial rules remain to verify |
| VS-0019 | [C1-line-x-9.txt](../../output/vesta-study/20261108-113622/s04/C1-line-x-9.txt) | nodeValues=VS-0005 x-fastest grid at y=z=0, periodic wrap for endpoint; comparison=s04/line-profile-independent-comparison.json |
| VS-0020 | [temporary-C1-density.led](../../output/vesta-study/20261108-113622/s04/temporary-C1-density.led) | Derived output, not modification of source pgrid |
| VS-0021 | [temporary-C1-density.ted](../../output/vesta-study/20261108-113622/s04/temporary-C1-density.ted) | Derived output, not modification of source pgrid |
| VS-0022 | [Actinolite.amc](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/AMCSD/Actinolite.amc) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0023 | [Al2O3.cry](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/CRYSTIN/Al2O3.cry) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0024 | [Alizarin.cssr](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/CSSR/Alizarin.cssr) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0025 | [4-Nitroacetanilide.cc1](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/Chem3D/4-Nitroacetanilide.cc1) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0026 | [Bi2CuO4.cmt](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/CrystalMaker/Bi2CuO4.cmt) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0027 | [C57-1405.fdt](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/FDAT/C57-1405.fdt) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0028 | [feff.inp](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/FEFF/feff.inp) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0029 | [Bi4Ti3O12.ics](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/ICSD/Bi4Ti3O12.ics) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0030 | [Anthraquinone.mol](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/MDL_mol/Anthraquinone.mol) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0031 | [Acanthite.min](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/MINCRYST/Acanthite.min) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0032 | [Amitrole.mld](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/MOLDA/Amitrole.mld) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0033 | [FILE07.DAT](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/MXDORTO/FILE07.DAT) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0034 | [insulin.ent](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/PDB/Crystals/insulin.ent) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0035 | [Fapatite.ins](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/RIETAN/Fapatite/Fapatite.ins) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0036 | [Bunnoite.ins](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/SHELX/Bunnoite.ins) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0037 | [Fullerene.xyz](../../output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/XMol_XYZ/Fullerene.xyz) | prepared; normative field interpretation and independent numeric expectation pending |
| VS-0038 | [carbon.png](../../output/vesta-study/20261108-113622/reviews/20261108-204840-s05-image-export/carbon.png) | 原件PNG输出；透明/独立alpha检查，Scale2双倍宽高变体，绝对基准尺寸为当次观察；见[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0039 | [carbon.pdf](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/carbon.pdf) | 原件Vector输出；PDF/SVG独立渲染，EPS/PS结构核验，视觉等价待核；[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0040 | [carbon.svg](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/carbon.svg) | 原件Vector输出；PDF/SVG独立渲染，EPS/PS结构核验，视觉等价待核；[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0041 | [carbon.eps](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/carbon.eps) | 原件Vector输出；PDF/SVG独立渲染，EPS/PS结构核验，视觉等价待核；[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0042 | [carbon.ps](../../output/vesta-study/20261108-113622/reviews/20261108-211140-s05-vector-export/carbon.ps) | 原件Vector输出；PDF/SVG独立渲染，EPS/PS结构核验，视觉等价待核；[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0043 | [carbon.bmp](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/carbon.bmp) | 原件Raster实物；RGB严格解码失败保留、其余范围见[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0044 | [carbon-raster.eps](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/carbon-raster.eps) | 原件Raster实物；RGB严格解码失败保留、其余范围见[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0045 | [carbon.jpg](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/carbon.jpg) | 原件Raster实物；RGB严格解码失败保留、其余范围见[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0046 | [carbon.ppm](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/carbon.ppm) | 原件Raster实物；RGB严格解码失败保留、其余范围见[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0047 | [carbon.raw](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/carbon.raw) | 原件Raster实物；RGB严格解码失败保留、其余范围见[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0048 | [carbon.rgb](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/carbon.rgb) | 原件Raster实物；RGB严格解码失败保留、其余范围见[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0049 | [carbon.tga](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/carbon.tga) | 原件Raster实物；RGB严格解码失败保留、其余范围见[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0050 | [carbon.tif](../../output/vesta-study/20261108-113622/reviews/20261108-213552-s05-raster-formats/carbon.tif) | 原件Raster实物；RGB严格解码失败保留、其余范围见[IMAGE_EXPORT](IMAGE_EXPORT.md) |
| VS-0051 | [angle-working.xyz](../../output/vesta-study/20261108-113622/reviews/20261108-220228-s06-angle-torsion/angle-working.xyz) | 独立四原子：角90°、二面角45°、CF=2√2Å；镜像为变体，原件两镜像都45°，见[结构几何](行为/02-结构与几何.md) |
| VS-0052 | [periodic-working.cif](../../output/vesta-study/20261108-113622/reviews/20261108-221538-s06-periodic-pair/periodic-working.cif) | 正交P1：主像9Å、(-1,0,0)像1Å；边界x上限1→2→1，计数4/2→6/3→4/2；[结构几何](行为/02-结构与几何.md) |
| VS-0053 | [symmetry-working.cif](../../output/vesta-study/20261108-113622/reviews/20261108-224603-s06-inversion-sites/symmetry-working.cif) | 反演一般C双像/原点O单重；闭胞10图像→范围0..0.99三像；不对称显示开关，composition/F000未读回；[结构几何](行为/02-结构与几何.md) |
| VS-0054 | [octahedron-working.cif](../../output/vesta-study/20261108-113622/reviews/20261108-230337-s06-octahedron/octahedron-working.cif) | 明确Si–O规则后6配位/1八面体；体积32/3按4小数匹配，表面积仅独立预期；[结构几何](行为/02-结构与几何.md) |

## VS-0033读取路径补证

追加同字节、同inode的FILE07.DAT/file07.dat路径对照，非新增全局样本。两者识别ID26，但原件分别退出255并保存空工程、退出1且无工程；都不算读取通过。内容/建模与副本崩溃栈边界见[READER_PATHS](READER_PATHS.md)，原37样本摘要和34探针日志保持。

## R10 VS-0033诊断变体

原VS-0033摘要与37个全局样本ID保留。新增本轮私有诊断变体[派生清单](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/width-variant-derivation.json)仅补900个空格，不改900个数值token及其余字节；不编号为VS-0038。[原件保存后的300位点/元素/坐标比较](../../output/vesta-study/20261108-113622/reviews/20261108-171847-s04-mxd-build/width-output-site-comparison.json)通过选定序列化检查；原始小写样本原件SIGSEGV仍是失败，变体通过不能替换原样本结果。

## R13 VS-0013/0014接续

样本与37个稳定ID/摘要不变。[短读核对](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/grid-short-read-crosscheck.json)解释2/512成功及510标量复用，729为含周期边界内部数据；[负晶胞核对](../../output/vesta-study/20261108-113622/reviews/20261108-180957-s05-input-validation/negative-cell-crosscheck.json)保留a=-2的数值/场景/矩阵/序列化流，和旧原件产物相符。新副本探针不是新VS样本，两个坏输入均不记科学有效性通过。

R16继续使用既有C1工程，未分配新VS ID：[GUI_LIFECYCLE](GUI_LIFECYCLE.md)。gui-working及a5/a7/a6快照为同一输入的生命周期诊断产物，不当新增科学样本或规范。原37样本/源C1保持字节。

R17使用原VS-0018、不新增VS编号。独立预期补[ALGORITHMS](ALGORITHMS.md)：8相/48面、凸半空间几何、原十进制与binary32输入体积控制明确分开；原文件未改。已有37个稳定样本保持，未重跑旧34读取矩阵。

R18使用原VS0011、不增VS编号，补独立2 Å和四个几何范围预期，见[ALGORITHMS](ALGORITHMS.md)。原XYZ SHA保持；37样本身份不改、诊断仅内存中成键规则，未写回原文件。

R20首次实际图像输出代表新增VS0038，当前38个稳定样本；原37条身份/记录未改。Scale2对照不额外分VS ID，原始文件/SHA/独立解码/视觉范围见[IMAGE_EXPORT](IMAGE_EXPORT.md)，不宣称像素/光照或所有图像格式通过。

R21新增四个不同Vector格式代表VS0039—0042，当前42样本；原38条记录完全保持。PDF/SVG派生渲染图不新增样本ID，EPS/PS仍无独立渲染，详见[IMAGE_EXPORT](IMAGE_EXPORT.md)。

R22实际八个Raster产物代表新增VS0043—0050，当前50样本，旧42记录保持。JPEG2000无文件，不虚造VS号；TIFF透明对照为变体不重复编号。SGI RGB坏文件保留并标明失败，成功提示不代表格式通过。

R24新增VS0051为角度/二面角覆盖代表，旧50记录完全保持，镜像仅诊断变体；当前51样本。非周期XYZ的dummy晶胞不当PBC测试。

R26沿用VS0009非正交CIF补实际距离/像坐标/角度，身份/路径/SHA不改；其它51条记录完全保持，仍52样本，不重复分配工作副本ID。数值选定通过与本轮deadline−15/No未执行分开记录，见[结构几何](行为/02-结构与几何.md)。

R29沿VS0054新增轴向畸变诊断变体（四2Å/两2.5Å），仅其independentExpected补variant路径/SHA/独立预期和实际比较；54身份全部保持，其它53记录完全相同。distorted2为同字节重试副本，不重复分ID。[公式/实测边界](ALGORITHMS.md)。

R31新增VS0055带坐标/晶胞标准不确定度的P1 CIF代表，独立解析传播先验、实际字段、同TEXT数值轨迹与显示取整分开。旧54条记录完全保持；trace副本同SHA、不新增ID，详见[ALGORITHMS](ALGORITHMS.md)。

R32沿VS0053补密度DC/占位特殊位置/反演及CUBE单位失败对照，55身份/路径/SHA全部保持，仅该记录增加densityF000Control，其它54记录完全相同。派生工程/pgrid/ggrid/CUBE为本例产物，不新增VS号；R27旧未读回字段按历史保留，实际后续F00014另记。[ALGORITHMS](ALGORITHMS.md)。

R33仅CUBE单位原因与原版读入/显式ggrid导出诊断，55样本记录/身份全部保持。两诊断产物不新增ID；原样本单位兼容false不改成通过，问题resolved仅表示原因已说明。[FORMATS](FORMATS.md)。

R34沿VS0055补独立外部散射系数/复数结构因子与实际波长预设诊断；55身份/路径/SHA保持，仅其independentExpected追加structureFactorControl，其它54记录不改。数值txt/同TEXT诊断CIF不新编ID；原SUE预期和原失败不改。[ALGORITHMS](ALGORITHMS.md)。

R35沿VS0055仅补isotropicOccupancyControls，三CIF为诊断衍生不新编ID；55身份保持、其它54记录完全不改。B2/4与U等价、占位.5/.75的有限复数贡献见[ALGORITHMS](ALGORITHMS.md)。

R36沿VS0055追加anisotropicControl；CIF仅正交P1诊断衍生，不新编ID。18反射90字段及12导入U分量读回符合，非正交ADP/椭球/非法参数仍未验，详见[ALGORITHMS](ALGORITHMS.md)。

R37沿VS0055只加neutronControls，C/O与符号替换H/O为诊断衍生不新编ID。55身份/其它54记录保持；72字段历史值兼容及另一NIST表差异见[ALGORITHMS](ALGORITHMS.md)。

R38新增VS0056独立单斜P21螺旋消光样本；前55记录完全保持，path/SHA/操作/倒易metric/先验12点与25字段实际比较详见[ALGORITHMS](ALGORITHMS.md)。

R39新增VS0057非正交常规U张量控制，原56记录完全保持；132先验字段及两个追加自动Ueq诊断分开，见[ALGORITHMS](ALGORITHMS.md)。

R40新增VS0058通用8列单反射input，CIF载体沿VS0055。三种导入模式648体素及DC/相位独立核对；原57记录保持。见[ALGORITHMS](ALGORITHMS.md)和[体数据规格](行为/03-体数据与二维.md)。

R41复用VS0058/R40 Fc CUBE，追加独立体数据Patterson216体素/积分64取证；仍58个稳定ID，前57记录保持。见[体数据规格](行为/03-体数据与二维.md)。

R42复用VS0058 Fc网格实际运行原件体数据正向StructureFactors；六反射/24先验字段通过，仍58稳定样本，前57记录保持。见[体数据规格](行为/03-体数据与二维.md)。

R43新增VS0059四唯一反射通用8列输入，10/12/14Å载体，288体素验证三轴/相位及DC；旧58记录完全保持。见[ALGORITHMS](ALGORITHMS.md)。

R44复用VS0059三轴网格，追加原件正向28反射/112先验数值和Patterson288体素算子取证；稳定样本仍59，前58记录保持。见[体数据规格](行为/03-体数据与二维.md)。

R45复用VS0059网格，追加LineProfile十三点/二十六字段独立插值/距离验收；仍59稳定样本，前58记录保持。见[ALGORITHMS](ALGORITHMS.md)。

R46复用VS59网格，追加负周期/反向52先验数值及N0/N1→2归一化取证；仍59样本/前58记录保持。见[ALGORITHMS](ALGORITHMS.md)。

R47复用VS59网格，追加半z格vectorplane2D导出49标量/98坐标候选核对及UI标签解析修正；仍59样本/前58记录保持。见[ALGORITHMS](ALGORITHMS.md)。

R48复用VS59，追加projection全/半周期：五原候选全否定、半周期49留出预测符合新近似采样规则，精确积分未通过；仍59样本/前58记录保持。见[ALGORITHMS](ALGORITHMS.md)。

R49新增VS0060独立constantρ1/0atom/6×6×8/10×12×14Å Cube，投影非整数R4.8与整数R5对照隔离归一化偏差；旧59记录完全保持。见[ALGORITHMS](ALGORITHMS.md)。

R54新增VS0061独立正密度1+.2cosX、6×6×8/10×12×14Å/0atoms Cube；独立连续/普通中心模型不符，26邻点L计算前候选符，梯度能量事后解释保留非holdout边界；旧60记录完全保持。见[ALGORITHMS](ALGORITHMS.md)R54。

R55新增VS0062三维耦合正密度、β100和VS0063signed/zero/1e−6邻域密度，两者6×6×8/0atoms Cube。三案全部预期在本轮首GUI前登记；threshold打印精度补充在其计算前登记，189原tol失败及63预期−Infinity保留。旧61记录完全保持，见[ALGORITHMS](ALGORITHMS.md)R55。

R56新增VS0064正交7/3/5和VS0065斜4/5/5γatan2(3,4)的P1三位点CIF；超胞外坐标、占位和U独立，旧63记录原样保持。报告约化结果与当前项目状态分验，详见[ALGORITHMS](ALGORITHMS.md)R56。

R57新增VS0066常量2含两Cube原子和VS0067带正负峰三轴cos场，均4³/4Å立方；旧65记录不变。按原子中心的初始Voronoi预期不适用于菜单共用峰处理路径，未标通过；峰/整胞积分先验与事后分配比较分列。见[ALGORITHMS](ALGORITHMS.md)R57。

R58新增VS0068，6×8×10/5×7×11Å/无原子耦合signed密度；首GUI前固定342邻域边界/0严格等距、每点分配/峰积分体积预期，原件符合所选tol，旧67记录保持。见[ALGORITHMS](ALGORITHMS.md)R58。

R60新增VS0069官方COD1000000 CIF，12580bytes/公共来源。35标签元素对和146保存字段源一致性通过；源获取在首导入后，补证前解析，不称首次目标前物理Oracle或临时native响应体SHA已测。旧68记录完全保持，见[DATABASE](DATABASE.md)R60。

R62新增VS0070独立P1两位点不同occ/U；三种晶胞变换先验、111字段通过。超胞表示/标签和F000派生值与运行值区分，旧69记录保持，见[ALGORITHMS](ALGORITHMS.md)R62。

R63复用VS0070，70样本记录保持；NormalizeOff/KeepParameters与det拒绝两案34先验字段符合，见[ALGORITHMS](ALGORITHMS.md)R63。

R64复用VS0057非正交Uani，70样本记录保持。58字段保存精度通过/原1e−6有3失败的双标准明确保留，见[ALGORITHMS](ALGORITHMS.md)R64。

R65新增VS0071独立P1精确重复C与.0004Å近邻，默认/两阈值预期sitecount3/3/2符合；mergedmetadata结果后观察不作先验通过。旧70记录保持，见[CLI](CLI.md)。

R66复用VS0054原始几何，派生带显式bond规则原生项目供CLI，不新增稳定ID，71记录保持；76报告科学字段/8工程别名与列表保存一致，见[CLI](CLI.md)R66。

R67复用VS0055、71记录保持；显式calcf中性X-ray/历史neutron108先验数值字段符合，anomalous1只作变化观察，中文空格out成功/空HKL无文件故障明确，见[CLI](CLI.md)R67。

R68复用VS0055/71记录保持。两默认/显式6与12196HKL表字节对照、73212d/2θ/M字段核对；原默认名假设失败与首输出覆盖明确，不新增样本掩盖，见[CLI](CLI.md)R68。

R69新VS0072独立F居中NaCl/cell4，72记录；闭盒显示假设失败保留，8Å变体作为家族诊断记录并有运行前补充先验。VASP/SHELX/RIETAN字段与TIDY导出证据详见[CLI](CLI.md)R69。

## R79 VS-0073：人工ORFFE结果输入

新增[733bytes .ffe](../../output/vesta-study/20261108-113622/reviews/20261109-114600-s07-orffe/identity0-gui-cwd/controlled-identity0.ffe)，SHA b8b1ed61eaf451b866202ba805f1ac2d5535ca3b6affc44ec4cffa0e7737f088。旧72记录保持。7原子/无额外对称行/码0，文件9.8765±.0123Å与123.456±.0045°故意不同于源模型2Å/90°；仅签读取/详情/相对链接保存，非ORFFE计算/协方差/重开/主图高亮验收。原identity/码1失败另留R79，不删除或计为通过。

R80补VS73：相对侧文件在新实例自动读取；只删一个O6参数记录的6原子变体触发拒绝，清表和工程链接、旧详情仍留。原VS73及旧73记录完全保持，诊断变体不另占ID；重开与主图高亮证据见ALGORITHMS R80。该处R79样本记录中的pending保留原时点，当前结果由本续记更新。


## R81 磁矩反演与时间反演（2026-11-09）

新增VS0074/75：10Å立方，Fe1(.1,.2,.3)，磁矩(1,2,3)μB；两个完整操作只在反演的时间符号±1上不同。首CLI前登记 [独立期望](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-before-targets.json)。一手Shinohara2023 §2.4 Eq7给m′=εdet(W)AW A⁻¹m；本案A=10I、W=−I，故普通反演保持磁矩、带时间反演则变号；magCIF字典只选读坐标/单位和xyz时间符号，全文阅读未完成。

原版两个-nogui导入/Save正常255，完整watch；实际MAGNETIC_CRYSTAL、SYMOP时间列±1、单site分配均保留。VECTR保存Cartesian(1,2,3)，末列type0/Axial（R83修正原坐标模式解释），原先预测晶格(.1,.2,.3)失败并留原计划；没有把表示差异当科学失败。原件自行识别2.4/P−1和2.6/P−1′，这些编号是目标观测，非本轮独立群号验算。[17项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-crosscheck.json)。

两幅归属窗口原生PNG（872×558）已逐一目视：反演行−x/−y/−z、T+1显示mx/my/mz；T−1显示−mx/−my/−mz，符合独立规则。[普通反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/inversion-equivalent-ui.png) / [时间反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/anti-repeat-equivalent-ui.png)。wx表行不暴露于AX，此为原图视觉读数；没有提取实际展开原子上的数值磁矩，也未校准3D箭头方向/长度。polar、非正交、特殊位置、磁超空间与New-dialog编辑仍待，不把VF0091/VF0177或完整磁性科学标作完成。

首截图native调用尚未完成时Codex提前Close/Quit，实际screenshot_window_not_found/单AXApplication节点；失败与 [原因记录](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/native-capture-failure.json) 保留。随后两个新归属进程分别33.382/30.084秒正常0退出，成功取证；首进程75.711秒也正常0，全部<180且无残留。CIF reader0xa671c已有静态证据ID去重，不重复增加记录；新增8条至844，12问题头/旧836证据/原件257/旧73样本不变，稳定样本增至75。80个历史manifest/11112文件完整。研究仍10完成19未完成、Codex自验，dsh独立复审未开始，无产品实现。


## R83 Axial/Polar 空间反演与文件类型标志（2026-11-09）

新增VS0076：立方10Å、Fe1(.1,.2,.3)、磁群反演T+1，磁矩改为单位(1,0,0)。首CLI/GUI前分别登记 [独立空间奇偶性](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/polar-before-targets.json) 与 [固定显示Scale3](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/polar-gui-before-target.json)；用模长1避免旧3.74提交舍入，Scale3使箭头露出Fe半径，两个状态比例相同。VM09 §9.1.1给Axial=det(M)Mm、Polar=Mp，反演期望分别(+a,+a)/(+a,−a)。

原版CLI正常255；GUI原生AX先Axial后Polar，Cartesian字段始终1/0/0、Modulus1.00、Scale3。两个同PID12648/window71712的860×728原始PNG已目视，[Axial双+a](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/axial-unit-main-ui.png) / [Polar第二原子−a](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/polar-unit-main-ui.png)，第一箭头保持+a；可见a轴朝屏幕右。结果后选定箭头端点ROI最大红列宽，第二左右(7,17)→(16,7)，第一(6,17)保持，支持视觉方向，不当作事前独立像素oracle或任意3D数值校准。[12项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/polar-crosscheck.json)。

**修正R81解释**：新SaveAs文件 [仅两处diff](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/saved-project.diff) 为VECTR末列0→1、VECTS1→3，其它字节相同；末列是Axial0/Polar1类型标志，不能再称coordinate-mode0。原生save_data0xacf04/10176bytes在0xaea90读对象flags+0x8c，0xaea94提取bit2写入该列。当前权威文档/功能字段已纠正；R81冻存原始文件和报告中的storedCoordinateMode字段按历史错误解释保留，由 [解释更正记录](../../output/vesta-study/20261108-113622/reviews/20261109-125600-s06-polar-vector/interpretation-correction.json) 明确废止其语义。构造函数参数3不再单独用作Axial类型oracle，原先晶格组件保存预期失败仍保留。

GUI正常0/148.640秒<180，无残留，4次nativeUI读取均在截止前且逐一等待完成。两次CUA状态保护阻止旧动作，刷新AX后重试，不推断人为改动。新增7至859Evidence、12头/旧852/旧75样本/原件257与82历史manifest/11426文件不变，样本增至76。Polar时间反演无效的实际对照、非正交和完整矢量/磁性仍待；R82降对称丢失失败不变。254功能/32权威、10完成19未完成，Codex自验，dsh独立复审/S08未开始，无产品实现。


## R84 Polar 时间反演、视角与CLI比例（2026-11-09）

新增VS0077：由VS76单位磁矩CIF只把反演时间+1改−1，首target前登记 [独立Polar时间不变预期](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-before-targets.json)。原件CLI导入后，按R83证实的类型位/Scale格式制作新受控Polar1/Scale3副本；其与R83正时间Polar工程仅GROUP2.4→2.6及SYMOP反演时间列不同。GUI新进程回读实际Polar、Cartesian1/0/0、Modulus1.00、Scale3。

**两项原预期失败保留**：①CLI无界面回读/Save [只把VECTS3写回1](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/roundtrip-actual.diff)，其余字节与受控源相同、Polar类型保留；字节一致先验失败，不能推广为全部场景字段/一般读取丢失。首GUI准备断言在启动前停止，实际 [准备失败记录](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/roundtrip-preparation-failure.json) 保留，后续GUI明确选择未改动Scale3受控源。②首GUI原图a轴沿视线、b右c上，箭头正对视线，原注册ROI与参考差18590RGBbytes；不能由此读出方向或宣称时间反演失败。[初始原图](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-minus-main-ui.png)。

结果后另登记 [新进程显式Viewalongc](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/projection-before-target.json)，原ROI[305,80,859,550]没有移动。[沿c观察原图](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-minus-c-view-ui.png) 已目视：a右b上，两个箭头(+a,−a)，与T+1参考相同；该554×470模型ROI逐RGB字节相等，0差。独立物理预期来自VM09 §9.1.1，图像等值是按该不变量检验的原件间一致性，不是外部绝对渲染oracle。限定Polar时间不敏感有证，首失败不回写成通过。[12项核对与失败状态](../../output/vesta-study/20261108-113622/reviews/20261109-130800-s06-polar-time-reversal/time-crosscheck.json)。

GUI14360/14733正常0，75.529/37.637秒均<180无残留；一处CUA差分索引解析未找到Edit而中止，刷新完整AX后正常继续，未发额外编辑动作。所有native读取等待结束后才Cancel/Quit。新增5至864Evidence，旧859/12头/旧76样本/原件257与83历史manifest/11566文件保持，稳定样本增77。下一步可追CLI为何未保留VECTS及非正交/一般矢量；完整Study10完成19未完成，254功能/32权威，Codex自验，dsh独立复审/S08未开始，无产品实现。


## R87 恢复提示、命令行对照与持久化矢量回查（2026-11-09）

R87先登记新进程计划，实际原生AX捕获macOS“unexpectedly quit while reopening windows”提示及Reopen/Don’t Reopen按钮；对先前拥有的研究窗口选择Don’t Reopen后，新单相主窗口正常加载并退出0。随后重新登记两组短偏好目录nogui对照：单Y、X再Y均原版正常255/exited，保存Y=1，分别1/2相，VECTS仍按R85命令行契约写1。此前PID25030的idle原因为何仍未唯一确定：当时没有UI证据，新的恢复选择和偏好目录也同时改变，不能把旧失败回写成通过。

两个全新GUI分别回读原incoming输入和R86已保存双相输出。原输入为Polar/Cartesian(0,1,0)/Modulus1.00/Scale7；输出第二相实际为Polar/Cartesian(0,.1,0)/Modulus.10/Scale5。因此原先保存Y身份失败已落实为持久化后GUI显示数值差异，不再只是一种尚未解释的序列化表示；仍未定位是导入、缓存场景还是保存阶段改变，不推广为一般磁矩科学损坏。原输入与回读工作副本哈希保持。两个回查均只Cancel内层Edit a Vector，外层Vectors用OK退出，Quit时对研究副本选No，不重放已知“缓存单相→IPC追加双相→外层Cancel”崩溃。外层OK即使无数值编辑也在这两次流程触发保存提示，此观测不外推到所有对话框。

三个拥有的GUI PID29107/29811/30583正常0，64.226/76.968/108.440秒均<180，无截止终止/残留。新增6个原生来源ID至884；raw原生导出882/not_loaded2，另两条经原生按ID接口补齐的完整884为明确标注派生快照。旧878记录、12问题头、旧77样本、原件257文件及86历史manifest/12021文件保持；新增稳定VS0078为独立规定的单相PolarY单位输入，样本增78。R86的raw失败、Cancel SIG11和旧headless idle记录不改。254功能/32权威文件、计划10完成19未完成；Codex自验，dsh独立复审/S08未开始，无产品实现。

[15项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-142400-s07-recovery-vector-controls/recovery-crosscheck.json) / [封存目录](../../output/vesta-study/20261108-113622/reviews/20261109-142400-s07-recovery-vector-controls/README.md)。

## S05 样本身份与独立判据专项（2026-11-10）

原78稳定ID/path/SHA均保持；[保护基线](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/protected-before.json)逐项核过。旧官方VS22–37中10种格式补独立选字段解码，9种相符、AMCSD元素差异保留；来源/容差/源哈希/回顾性时序在[源预期](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/official-source-expectations.json)，其余6种没有虚填数值预期。旧准备状态保存在机器记录initialPreparationStatus，不把早期pending写成从未执行。

| 样本ID | 文件 | 独立预期 | 边界 |
|---|---|---|---|
| VS-0079 | [axis-sentinel.pgrid](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/axis-sentinel.pgrid) | 2×3×4，cell4/5/6，全90°；x最快；v=x+10y+100z；mean160.5、积分19260 | 原生输入248字节，SHA f3faefbb448679e6ec7843d0f742e1517682bcfeeae4be23aec391bed359dde0；[预期](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/axis-before.json)在首目标前冻结；[实测](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/axis-analysis.json)区分原始文本独立解码和m3d/pri目标解码；不接受全格式兼容 |

当前79稳定样本。新数据导出/读回是VS5/VS70/VS79实验派生，不另分稳定ID，不把原生pgrid/CIF读取器自读回当唯一独立预期。

## S05 本次继续执行结果（dsh复审通过）

稳定台账现在101条，旧79条记录逐项保持。新增VS0080–0098为19个逐格式具名代表；VS0099/0100为实际STL/VRML输出，独立原几何预期来自VS70；VS0101为独立ASCII双数据集/未知字段CIF。初始错误后缀/字段变体原文件保留，注册采用正确.asse/GSAS空格键/wien2venus尾胞布局；raw author DEN拒绝不以normalized诊断替代。MMP记录同basename GAMESS副文件SHA。所有新文件/来源/独立预期/真实比较在[samples.json](../../output/vesta-study/20261108-113622/s05/samples.json)。

[运行前定义](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/inputs-before.json)、[真实源/修正变体定义](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-variants-before.json)、[CIF定义](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/metadata-controls-before.json)、[SCAT配套定义](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/companion-controls-before.json)。本次旧五例独立源解码属回顾性复核，不声称先于历史原件读取；源码/资料自述与原件数值分别列。下面早期表的prepared/pending是初始记录，最新按[完整矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-format-matrix.json)与[字段对照](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/new-input-analysis.json)判定，不将旧原始记录重写为所有格式通过。

## S06 本卡样本与独立预期

新增VS-0102：有界资源/轴序的解析CUBE标量，16×17×18与32×33×34诊断变体；rho=10000i+100j+k−150000，含零/负值，任意无量纲标量。原件导出17×18×19与33×34×35一般网格共45084值，独立binary32解码/周期补边逐点零误差。不是容量上限或电子密度物理场。首轮错误-saveggrid无输出与分析异常保持，按旧真实-save/.ggrid命令补导出，原先验/零容差未改。

VS55增加H/O核密度和两种模型Patterson零频控制，输入沿R37诊断衍生，不重复分ID。旧101个ID/路径/SHA保持，仅VS55独立期望追加；其他100条完整记录保持。[S06_ORACLES](S06_ORACLES.md)逐案列数值/单位/容差/先后关系与失败。[新证据](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/)。

S06验收记录：本卡实际dsh独立复审pass，研究卡四项完成；已证/失败/未知的科学边界保持，完整Study和产品实现未完成。见[裁定](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/review-result.json)。本轮停止。

## S07 外部完成合同输入

新增VS-0103为R56合成P1三原子data.stin的逐字工作副本，VS-0104为NOT_A_SPACE_GROUP坏群输入；实际分配从旧lastSampleId102追加，不复用ID，旧102记录保持。事前输入SHA/6秒期限和产物预期见[登记](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/tidy-before-target.json)，4案（另含缺输入/缺数据库）分别保存真正子返回/信号/内容/清理。选定有效报告可用不等于全科学，坏群退出0明确不能作成功。
