# S02.3 字典、扩展与历史格式续读（2026-11-08）

**当前任务状态（2026-11-10）：** S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 来源unknown revision10/high/investigating保留目标绑定边界；下文旧 revision/未勾选续记为历史，来源阅读不等于能力通过。接续见 [HANDOFF](HANDOFF.md)，阅读边界见 [SOURCE_NOTES](SOURCE_NOTES.md)。

承接[SOURCE_VOLUME_SCIENCE](SOURCE_VOLUME_SCIENCE.md)。本轮沿用CIF1语法/语义的既有阅读，补字典定义及未补的生产者资料；S02.3仍为[ ]。完整外部原文、字典和渲染页只在临时阅读目录，永久[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-145707-s02-dictionaries-legacy/)保留身份、章节、短笔记、原生状态与校验回执。

本页记录前轮范围；后续历史格式和rho生产者关联修订见[SOURCE_LEGACY_PRODUCERS](SOURCE_LEGACY_PRODUCERS.md)，来源问题现已接续revision6；不把本页选读扩大为完整规范。

## CIF字典与扩展

| 来源 / 直接入口 | 准确版本及本轮阅读范围 | 可用于后续验证的区别与边界 |
|---|---|---|
| EXT-CIF-DICT-GATEWAY，[IUCr字典入口](https://www.iucr.org/what-we-do/digital-standards/cif/dictionaries) | web读正文的字典角色/版本/导航，curl403；正文入口不作为字典条目全文。新字典服务的curl200与web403分别保留。 | 入口并列DDL1 2.4.5及DDLm 3.3.0；字典版本、CIF语法版本和DDL版本分别登记。入口的使用比例描述不作为目标兼容证据。 |
| EXT-CIF-CORE245，[固定原字典](https://cif-dictionaries.iucr.org/cifdic/dic/cif_core_2.4.5.dic)、[展开视图](https://cif-dictionaries.iucr.org/cifdic/dichtm.php?dic=cif_core_2.4.5) | 2.4.5、2014-11-21，CIF1/DDL1。读身份头及cell_length/angle、atom_site_fract、occupancy/type_symbol、U_iso/B_iso、aniso_U组、symmetry_equiv_pos_as_xyz、audit_conform_dict_version完整条目；未读整字典/历史。 | 长度Å、角度度，U/B均有Å²单位但相差8π²；各向异性U是含倒易晶格度量的结构因子分量，不能直接当笛卡尔矩阵。占位定义含3u容差与dummy例外，不可无条件截到[0,1]。类型码有父项链接，不能一律从标签截取元素。字典声明也不证明目标执行了校验。 |
| EXT-CIF-CORE330，[固定原字典](https://cif-dictionaries.iucr.org/cifdic/dic/cif_core_3.3.0.dic)、[展开视图](https://cif-dictionaries.iucr.org/cifdic/dichtm.php?dic=cif_core)、[发布公告](https://www.iucr.org/__data/iucr/lists/cif-developers/msg00530.html) | 原字典3.3.0、2026-09-22、DDL conformance4.2.0，带CIF2标记；公告2026-10-02。读身份/导入声明及展开视图的cell.length_a、atom_site.fract_xyz/occupancy/type_symbol、audit_conform.dict_version五项；未读整字典或执行dREL方法。DOI重定向后TLS失败，随后固定版本直链200；固定/通用文件字节相等。 | 版本晚于目标构建日期；不能从当前“latest”倒推目标采用版。新名字有旧名别名，向量、派生方法和外部导入也有独立语义。展开视图的长度范围声明按来源原样保留，未提升为一般物理最小晶胞判据；模板导入图未整体锁定/验证。 |
| EXT-CIF-MAG，[magCIF原字典](https://cif-dictionaries.iucr.org/cifdic/dic/cif_mag.dic)、[展开视图](https://cif-dictionaries.iucr.org/cifdic/dichtm.php?dic=cif_mag) | 0.9.8、2020-07-28，CIF2标记/DDL3.13.1；不是从文件名猜版本。读身份/未定版导入声明及moment.crystalaxis、crystalaxis_x、Cartn、symmform、magn_operation.xyz、magn_centering.xyz指定完整条目，未读全磁性/调制定义。 | 磁矩单位μB；crystalaxis沿晶轴，Cartn为x∥a、z∥c*的右手正交系。操作末尾±1给时间反演，磁矩约束没有位置的平移项；centering/anti-centering另列。其导入文件名未定版，不能把四份字典视为已锁定的完整依赖包；目标磁性导入/显示仍待核。 |
| EXT-CIF-MS，[调制字典原文](https://cif-dictionaries.iucr.org/cifdic/dic/cif_ms_1.0.1.dic)、[展开视图](https://cif-dictionaries.iucr.org/cifdic/dichtm.php?dic=cif_ms) | 1.0.1、2005-06-16；DDL1原字典说明基于core2.3(2003)。读身份头、modulation_dimension、cell_wave_vector序号/分量、atom_site_Fourier_wave_vector分量、displace_Fourier_param的cos/sin、ssg_name及ssg_operation_algebraic完整条目；未读整字典。 | 独立q向量与Fourier谐波向量不同；分量所用倒易基、复合结构子系统及W矩阵须区分。位移系数可为晶胞分数或特殊轴下Å，不能统一套单位。超空间操作按调制维数使用x1…xn；其余调制、占位/ADP与目标约定仍待核。 |

原字典以原始UTF-8字节为准；通用HTML提取器用于导航/展开视图，不能用于判断原字典尖括号、反斜线或完整语法。字典方法只作静态阅读，没有执行、导入到产品或生成新科学计算通过记录。

## 历史/生产者格式

| 来源 / 直接入口 | 本轮实际阅读 | 边界 |
|---|---|---|
| EXT-USPEX-MANUAL，[9.4.4输出页](https://uspex-team.org/online_utilities/uspex_manual/sect0016.html)、[10.5手册](https://uspex-team.org/online_utilities/uspex_manual_release/EnglishVersion/uspex_manual_english.pdf)、[2023手册](https://uspex-team.org/static/file/uspex_manual_english_v2023.0.pdf) | 9.4.4页读§3.2引言和gathered/BEST/unrelaxed/order等选段；10.5封面2021-07-05、133页，只读封面及§3.2的PDF30–31选段，两页实际查看；2023文件78页，封面2023.0/2023-05-23，页眉Release2023.0.1，读封面及§3.2（PDF19）。 | gatheredPOSCARS是弛豫结构，BEST按每代最佳组织，unrelaxed含义不同。10.5例连续写EA1/EA2两个VASP5 POSCAR块，旧order文件有额外原子字段；不能只认一个块或丢弃余列而声称完整支持。2023版本字段按封面/页眉分记，未核其余章节、所有输出变体或目标读取。 |
| EXT-CMTX-PRODUCER，[官方版本记录](https://www.crystalmaker.com/crystalmaker/release-notes/mac/11/index.html)、[手册位置](https://crystalmaker.com/support/advice/pages/gn-wheres-the-manual.html)、[规格](https://www.crystalmaker.com/crystalmaker/specs/index.html) | 读11.0.0/2024-01-02中Extended CMTX完整条目、手册位置说明全文及规格的文件格式表选段，未读完整版本历史/规格。 | 官方记录新增DATE/VERS/FORM/DENS并放开旧“每位点3占据者”限制；这是版本变更说明，尚非完整CMT/CMTX语法。手册随应用Help提供，未获取完整手册；没有把其它程序的读写列表当格式规范。 |
| EXT-AMCSD-FORMAT，[RRUFF下载帮助](https://www.rruff.net/help/zip-download/)、[旧AMCSD入口](https://rruff.geo.arizona.edu/AMS/amcsd.php) | 仅搜索通道可见生产者帮助的AMCSD文件类型/文件名说明；帮助及新入口直读403，旧入口超时，未取得原始amc样本或字段规范。未下载完整数据库ZIP。 | 帮助将amc描述为标准化晶体数据，不能据摘要恢复历史记录布局、空间群设置或元素标签规则；这项只收窄来源入口，仍不登记格式正文已读。 |

[覆盖表](FORMAT_SOURCE_COVERAGE.md)继续保留47类旧手册输入、23个已观察数据输出，新增字典/生产者出处与各自边界。现有样本结果仍以FORMATS/SAMPLES为准，字典数据项和版本记录都不替代原件的实际输入/往返/计算验证。

后续仍需补未读字典条目/依赖版本、历史数据库与专有格式完整布局、核心科学原文及勘误；目标版本绑定与科学离散约定由S05–S07继续取证。S02.3不因选读和新增来源数而提前勾选。
