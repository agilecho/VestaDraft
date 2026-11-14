# S06 独立预期与同输入对照

当前任务对齐（2026-11-10）：S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 当前 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open，即14个未解决；来源 revision10、数据库 revision4 保持。本次状态对齐没有新增目标证据或产品代码。 既有结果与失败边界保持；从 [HANDOFF](HANDOFF.md) 接续。

本页索引具体输入、值、来源和公差；所有原始先验/失败文件保持。[原始预期清单](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/oracle-source-inventory.json)只定位文件，不将名字含independent/before的文件一律算事前先验。每轮使用原记录时间/源SHA识别 primary、事后诊断、兼容规则留出；详见 [ALGORITHMS](ALGORITHMS.md)对应R节。

| 样本/输入 | 具体独立预期、单位 | 公差与理由 | 来源及实际边界 |
|---|---|---|---|
| VS9，2/3/4 Å、γ60°，C0/O(.5,.5,0) | 主像√19/2=2.1794494718 Å，近像√7/2=1.3228756555 Å，角acos(5/√133)=64.3066190955° | 原件显示距离5小数/角4小数：半末位5e−6 Å/5e−5°；不推广为任意内部精度 | R26 independent-nonorthogonal-expectation.json / independent-nonorthogonal-angle.json；对称/阈值的完整域未知 |
| VS52，10 Å P1 Cx.05/Ox.95 | 主像9 Å、O平移−1后1 Å，所选image不得被最短像替换 | 5e−6 Å显示半末位；边界图像数模型有事后条件，不冒充预注册通则 | R25 independent-periodic-expectation.json及原测距；显示与化学重数分开 |
| VS53，P−1一般C占位.5/特殊O占位1 | 唯一C2/O1，组成C1O1，F00014 e、均值.014 e/Å³，反演相等 | 积分1e−4 e、反演1e−5 e/Å³，针对216 float/text点预定，不作通用FFT界 | R27有理位置枚举；R32 independent-density-f000.json后真F000/网格积分；CUBE外部原子单位不通过保持 |
| VS55、σx=.001/.002，a10、σa.01 | d2 Å、Var=.000504 Å²、σ=.02244994432064 Å | 解析对角传播；显示按目标formatter核2.00(3)，不拿括号3当直接σ | R31 independent-distance-sue.json先验；.0224500163812是同TEXT副本与binary32次级诊断，不改解析oracle |
| VS55衍生，B2/4、等价U或occ.5/.75 | F100(B)=2.242452518+10.704928974i；F100(occ)=.518421472+7.251833301i | 实虚6小数半末位5e−7，模按实际有效位的半末位；不是统一4小数 | R35 independent-isotropic-occupancy.json、独立DABAX C/O；B与U导出相等仅辅助，独立复和式才判数值 |
| VS57非正交U与换基 | Ueq C .020772643606/O .050558914515 Å²；beta′=Q beta Qᵀ，回U′ | R64原U1e−6有3失败；第二案前另固定5位打印半末位5.000001e−6，双标准保持；Ueq1e−6 | independent-tensor-before-gui.json、printed-tensor-policy-before-shear.json；不把保存5位验证称内部更高精度通过 |
| VS54 SiO6规则/四2 Å两2.5 Å | V32/3→40/3 Å³，D0→4/39，λ1→1.023356477765，ECoN6→4.3148170791 | V/λ/ECoN4小数半末位5e−5，D5小数5e−6；显示格式单独核 | R28/29 independent-octahedron-expectation.json / independent-distorted-expectation.json；迭代ECoN候选4.34238被否定 |
| VS18八相Adularia，各面取其H及D | 48面面积；首相333.1748950653/81.8699800881/240.6778843599 Å²，D/d1.51034010125 | 六有效数字逐字舍入；体积binary32解释与高精度十进制分开 | R17为**事后独立几何重算/显示舍入政策校对**：GUI 20:03:45–20:09:05，数学产物20:10；不称预注册先验。independent_shape_geometry.py和shape-display-rounding-check.json；这是多相可计算场景，不声称一般phase定位编辑已验 |
| VS59四谐波P1，10/12/14 Å | F1003+4i/F010−2+i/F0011−3i/F1112+2i；ρ=[8+2Σ(Re cos+Im sin)]/1680，积分8 e | 逐点1e−7 e/Å³、积分1e−4 e，原预定float FFT/六有效位文本；本卡重算288点max4.8437e−8 | R43 independent-three-axis-fourier.json先验/原CUBE；本卡existing-independent-reanalysis.json是新离线复核，没有重跑目标 |
| VS59线剖面，(.0,.125,.25)→(1,.625,.75)，N13 | t=i/12，8角周期三线性；总长√185=13.6014705087 Å，26字段 | d1e−4 Å、ρ1e−6；事前float坐标/导出预算；连续解析场不能代替粗网格插值 | R45 independent-line-profile.json；R46负周期/反向/N归一另有先验 |
| VS62三维非正交正ρ、VS63零负阈值 | 26邻点兼容导数与独立连续metric导数分列；rawρ>1e−6能量、logabs0=−Inf | R55原先验按场/量纲分别固定；6615字段中非有限值单列，不以全finite假通过 | independent-3d-threshold-holdout.json；4410非正交两单位字段符合兼容模型，连续精确导数不通过 |
| VS70粉末λ7.7、Z−.02/Ds−.06/Ts.08 | Bragg79.83242239646°；兼容修正反射79.80735918261°、采样峰79.81° | 内部index1..99 .01/100、反射显示半末位；首点原0 vs独立3.070702272，明确失败 | R78 shift-holdout-before-target.json及CSV；R77误读360原失败保留，原完整谱不接受 |
| VS55 H/O诊断衍生，新核/模型Patterson | −3.739+5.803=2.064 fm；核P积分4.260096 fm²；电子Z和9→P积分81 e²；偶对称、P0最大 | 分别1e−4/2e−4/.002积分绝对公差，float/text预算≤5e−5相对；P偶差≤1e−6，事前冻结 | 本卡density-oracle-before-target.json早于三目标启动；实积分2.0640034722/4.2600898148/81.000199537。没有完整谱/逐体素通过结论 |

## 未核定预期与P1交接

[S06_DOMAIN_RULES.json](S06_DOMAIN_RULES.json)每组的unknown/nextProbe列出未核定领域，包括多相R/t、一般群/三斜最短像、键价选参、椭球轴、等值面拓扑/映射、精确投影、所有Nyquist/网格政策、反常表和资源上限。没有科学oracle的条目保持unknown，不用截图/目标自读/源码常量替它造预期。MADEL数值因真实加载阻塞未验，本卡不安装或修补；逐多面体菜单的预期为已证空处理，不编造积分。

P1消费这些已锁输入及字段时，先保存oracle/SHA/单位/容差版本，再对参考和新实现运行相同输入、选项、所选image和提交顺序。比较模型/科学值、显示舍入、资源保存与重开、可见状态分别给通过/失败/未知；不以其中一项替代另一项。结构/体数据/相机必须分层核；Mac arm64证据不能替代Web、容器或Linux x86链。数值误差按每个字段/输出格式，不引入统一相似度或改变本项目完整验收要求。

原件缺陷如何兼容属于后续明确设计裁定：当前保留原失败，既不能自动洗成通过，也不在S06擅自修复/删功能。拟合解释必须用新输入留出；R74/R77反例、R64双精度政策、R55连续/兼容模型并存。本页仅交接同输入方法，没有启动P1或产品实现。
