# S02.3 体数据与科学来源续读（2026-11-08）

**当前任务状态（2026-11-10）：** S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 来源unknown revision10/high/investigating保留目标绑定边界；下文旧 revision/未勾选续记为历史，来源阅读不等于能力通过。接续见 [HANDOFF](HANDOFF.md)，阅读边界见 [SOURCE_NOTES](SOURCE_NOTES.md)。

承接[SOURCE_FORMATS_SCIENCE](SOURCE_FORMATS_SCIENCE.md)。本轮按旧覆盖表缺口继续选读，不重复登记前轮已经读过的章节；S02.3仍为[ ]。本轮[证据目录](../../output/vesta-study/20261108-113622/reviews/20261108-143347-s02-volume-science/)保存获取元数据、实际范围、原生问题更新和验收记录；完整原文及渲染页仍只在临时阅读目录。

本页为前轮范围；最新字典/历史格式补证见[SOURCE_DICTIONARIES_LEGACY](SOURCE_DICTIONARIES_LEGACY.md)，来源问题接续revision5，旧未读范围按新台账收窄，未回改成全文通过。

来源绑定修订见[SOURCE_LEGACY_PRODUCERS](SOURCE_LEGACY_PRODUCERS.md)：VM18所指为wien2venus.py三维输出；LAPW5二维case.rho仅为同后缀参照，未建立二者等价。下表LAPW5章节阅读保留，不能据此填成VESTA该条的生产者契约。

## 结构输出及体数据

| 来源 / 直接入口 | 版本与本轮实际范围 | 可用结论及尚未核定部分 |
|---|---|---|
| EXT-WIEN2K，[官方指南](http://www.wien2k.at/reg_user/textbooks/usersguide.pdf) | 沿用前轮23.2/2024-07-24、316页原字节，不重下载。新增§8.14开头至§8.14.3中指定字段：PDF215–218，印刷195–198；PDF216实际查看。未读218页之后的STM其余设置。 | `LAPW5 case.rho`是指定平面的二维网格，头为npx/npy/xlength/ylength；随后5e16.8，j内层、i外层。npy=1另写rho_onedim。密度/势及ANG/ATU选项不同，原点、平面、正交选项在in5，不由rho头自足恢复；3D另用3ddens。目标读取子集及长度单位关联仍需实测。 |
| EXT-CASTEP-GUIDE2024，[BIOVIA指南，Cambridge托管](https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/pdfs/castep.pdf) | 封面Materials Studio2024、248页；PDF元数据2023不是另一应用版。读封面及Electron density选段PDF66/224（印刷58/216），实际查看PDF66。 | `.charg_frm`为规则晶格索引与Charge，可另含Spin；示例a索引先变。文件值按每胞电子数归一，界面场按e/Å³，不能混用。摘录没有完整网格尺寸/晶胞头，也未证明任意`.den_fmt`改后缀即兼容；目标支持与其它年代仍待核。 |
| EXT-ELK-WRITEGEOM，[官方Doxygen源码页](https://elk.sourceforge.io/doxygen/html/writegeom_8f90_source.html) | web读源文件显示行1–73及文件说明；curl403，无源码原字节摘要。页尾1.8.14是Doxygen版，不是Elk版；不能绑定前轮11.2.3手册。调用图SVG在web不受支持，curl403。 | writer写scale/scale1–3为1、avec、可选molecule、按物种组织的位置/场。molecule分支乘晶格矩阵；注释声称的区间映射不能当通用取模保证。与GEOMETRY.OUT的具体调用、该文档对应发行版仍未核；未执行或复用源码。 |
| EXT-GSAS，[分发手册](https://subversion.xray.aps.anl.gov/EXPGUI/gsas/all/GSAS%20Manual.pdf) | 沿用2004、231页原字节。新增Files章EXPNAM.fmp段，PDF222–223（印刷215–216）完整该段；不重读前轮EXP记录。 | fmp是非格式化直接访问文件，两条头记录后按切片存密度，并有方向标志/大切片分记录规则；未据此推断字节序和编译器记录长度。它不是通用grd的证据；覆盖表OBS-O18原来的“GSAS待补”须收窄为两者对应关系未建立。 |

## 科学原文及访问边界

| 来源 / 直接入口 | 实际读取 | 边界与接续 |
|---|---|---|
| EXT-BIB-095，[Chantler1995，NIST原论文](https://srd.nist.gov/jpcrdreprint/1.555974.pdf) | DOI10.1063/1.555974，JPCRD24:71–643。574页PDF含新增出版封页及573页论文；读封页/目录/§1–2（PDF1–3）、§10和§11所在PDF10–11、§16的说明/表头（PDF14），另看PDF15表头，不宣称数表全核。实际查看PDF3/14。 | f₁不是直接的f′，有Z与相对论修正区别，f″对应f₂；表头另列核散射修正。能量keV、波长nm、散射因子e/atom与衰减单位分开。插值建议和孤立原子/吸收边附近限制已读，不登记目标插值、常数或整张参数表通过。web因38MB超限失败，curl200完整取得，分开留证。 |
| EXT-BIB-048，[Tsirelson2002作者上传稿](https://www.researchgate.net/profile/Vladimir-Tsirelson/publication/11232328_The_mapping_of_electronic_energy_distributions_using_experimental_electron_density/links/0fcfd507e406bb7222000000/The-mapping-of-electronic-energy-distributions-using-experimental-electron-density.pdf) | 作者记录核来源；web可读8页原论文。新增印刷632–634及635前段的文字、公式提取与表头选读，不宣称整篇；curl记录页/PDF均403，web截图633页超时，公式视觉核验未完成。初始相对下载链接404后改用页面作者/profile路径。 | 二阶梯度展开与mean-path系数不同，文中比较后采用前者；近核区存在不适用范围，不能由该近似保证处处正值。离散导数、边界、零/负密度、单位与目标实现仍属S06；其余正文/图像未核，不把本文当所有ELF/LOL公式来源。 |
| EXT-BIB-033，[Robinson1971的PubMed条目](https://pubmed.ncbi.nlm.nih.gov/17802221/)、[USGS条目](https://pubs.usgs.gov/publication/70010317) | 搜索通道可见原刊引文和摘要；PubMed直接web仅一行，curl203挑战页，USGS curl超时/web403。未得到4页原文，前轮出版社受限下载不重复。 | 摘要不是公式正文。USGS索引的作者展开姓名与本条身份有疑点，不据此改原作者K. Robinson/G.V. Gibbs/P.H. Ribbe；不能把引用本文的其它论文变成本文已读。 |
| EXT-FFAST / EXT-FFAST-HISTORY，[NIST版本历史](https://www.nist.gov/pml/x-ray-form-factor-attenuation-and-scattering-tables/x-ray-form-factor-attenuation-and-scattering) | 本轮重复访问已读历史页，只作1995论文与当前数据库的版本参照，不计新增正文成果。前轮2.2/May2026等裁定沿用。 | 数据库当前版本不等于1995论文或本地参数版本；未核完整表值。 |
| EXT-BIB-096，[NIST重印目录](https://www.nist.gov/srd/nist-standard-reference-database-journal-physical-and-chemical-reference-data-reprints)、[作者书目](https://www.ph.unimelb.edu.au/~chantler/opticshome/papers.html) | 只核2000条目：NIST列DOI10.1063/1.1321055、29:597–1048，且仅给DOI；作者页列597–1056、题名/能量范围也有差别，未据此改原书目。按DOI猜测的NIST PDF路径404，作者页curl超时但web可选读书目。正文仍未读。 | 不把1995论文封页上的“相关论文”当成2000正文。作者书目另列1996勘误25:527–532；NIST对应项题名空白/DOI n/a，勘误原文未核。因此1995选读也不能代表修正后全部数表。 |

## 后续工作

上述补证使LAPW5二维rho参照的头/轴序、CASTEP charg_frm列/归一化、Elk writer布局和两份科学原文的选段不再是完全未读。它们仍没有组成目标版本的完整契约；格式覆盖表保留47输入/23数据输出的既有清单与未完成状态。

剩余工作按覆盖表继续：历史数据库/专有格式、CIF字典/扩展、Elk输出调用与版本、各格式余下头部/边界；Robinson/Hoppe/Brown/Waasmaier/ITC原文与科学公式也保留真实访问状态。研究不触发原件或外部科学计算，不新增功能/样本通过，也不扩大为产品实现。
