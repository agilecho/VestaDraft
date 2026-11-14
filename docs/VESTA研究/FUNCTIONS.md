# VESTA 功能目录

当前任务对齐（2026-11-10）：S01–S08按原研究判据完成，Study29/29与P0研究交付保持。P1本轮经dsh正式独立复审为partial：P1.1路线/技术栈/目录接受，P1.2–P1.4关键接口/接线/全模块精确计划未完成，整体P1未通过。用户默认复现已证原件行为并提示风险、修正显式选择，默认.vesta加资源保存的裁定已回写。已停在P1，不进入P2–P5/产品施工/部署/模型builder；研究完成不代表全部科学/格式/平台通过。当前入口为HANDOFF及P1设计。 当前 254 个稳定功能 ID、104 个稳定样本、84 条入口级待核；2037 条原生记录为逐文件哈希核对的磁盘派生索引，不是全量 raw 导出。24 个原生问题头为 10 resolved/3 investigating/11 open，即14个未解决；来源 revision10、数据库 revision4 保持。本次状态对齐没有新增目标证据或产品代码。 既有结果与失败边界保持；从 [HANDOFF](HANDOFF.md) 接续。

本目录先登记已观察入口；`entry-observed` 只证明入口存在，不代表行为或算法已通过。稳定 ID 不回收；详细字段保存于 [functions.json](functions.json)，逐域行为规格继续补证。手册发现条目另见 S02，尚不等于实测功能数量。

| 功能 ID | 实际入口（同功能别名合并） | 资料 | 当前状态 |
|---|---|---|---|
| VF-0001 | VESTA / About VESTA... | VM04, VM16 | partial behavior observed; see behavior/01 |
| VF-0002 | VESTA / Preferences... | VM04, VM16 | entry-observed / behavior-pending |
| VF-0003 | VESTA / Hide Vesta | VM04, VM16 | entry-observed / behavior-pending |
| VF-0004 | VESTA / Hide Others | VM04, VM16 | entry-observed / behavior-pending |
| VF-0005 | VESTA / Show All | VM04, VM16 | entry-observed / behavior-pending |
| VF-0006 | VESTA / Quit Vesta | VM04, VM16 | selected-clean-and-General-dirty-No-Quit-verified / cancel-multidocument-and-R101-write-cause-pending |
| VF-0007 | File / New Structure... | VM04, VM18 | partial behavior observed; see behavior/01 |
| VF-0008 | File / New Window | VM04, VM18 | entry-observed / behavior-pending |
| VF-0009 | File / Open... | VM04, VM18 | selected-name-dispatch-and-source-field-reuse / full-per-format-read-display-edit-retention-pending |
| VF-0010 | File / Open Recent | VM04, VM18 | entry-observed / behavior-pending |
| VF-0011 | File / Search Database... | VM04, VM18, APPLE01, APPLE02, COD01 | selected-HTTPS-COD-import-save-navigation-404-dirty-contracts-verified / full-network-cancel-provider-matrix-pending |
| VF-0012 | File / Save | VM04, VM18 | selected-project-and-comment-save-observed / exact-tail-and-general-branches-pending |
| VF-0013 | File / Save As... | VM04, VM18 | selected-behavior-observed / remaining-branches-pending |
| VF-0014 | File / Export Data... | VM04, VM18, VM14 | selected-default-data-exports-and-independent-semantics / PDB-B-failure-and-full-options-pending |
| VF-0015 | File / Export Raster Image... | VM04, VM18, SGI01 | all-format-entries-executed / selected-three-failure-causes-diagnosed / full-options-pending |
| VF-0016 | File / Export Vector Image... | VM04, VM18 | four-formats-generated / PDF-SVG-selected-parse-render-verified / EPS-PS-structure-only |
| VF-0017 | File / Save Output Text... | VM04, VM18 | selected-Output-text-export-and-source-observed / Summary-cancel-overwrite-write-errors-pending |
| VF-0018 | File / Close | VM04, VM18 | selected-close-choices-verified / failure-multidocument-pending |
| VF-0019 | Edit / Undo<br>Comment / focused text / Cmd-Z | VM04, VM06 | selected-boundary-parameter-undo-redo-observed / general-history-pending |
| VF-0020 | Edit / Redo<br>Comment / focused text / Cmd-Shift-Z | VM04, VM06 | selected-boundary-parameter-undo-redo-observed / general-history-pending |
| VF-0021 | Edit / Edit Data / Phase... | VM04, VM06 | selected-context-observed / remaining-branches-pending |
| VF-0022 | Edit / Edit Data / Unit Cell... | VM04, VM06 | selected-behavior-observed / remaining-branches-pending |
| VF-0023 | Edit / Edit Data / Structure Parameters... | VM04, VM06 | entry-observed / behavior-pending |
| VF-0024 | Edit / Edit Data / Volumetric Data... | VM04, VM06 | entry-observed / behavior-pending |
| VF-0025 | Edit / Edit Data / Crystal Shape... | VM04, VM06 | selected-context-observed / remaining-branches-pending |
| VF-0026 | Edit / Bonds... | VM04, VM06 | selected-bond-range-observed / other-modes-pending |
| VF-0027 | Edit / Vectors... | VM04, VM06 | dialog/control observed; behavior pending |
| VF-0028 | Edit / Lattice Planes... | VM04, VM06 | dialog/control observed; behavior pending |
| VF-0029 | View / Antialiasing | VM04, VM13 | entry-observed / behavior-pending |
| VF-0030 | View / Parallel | VM04, VM13 | selected-camera-state-and-byte-roundtrip-verified / rendering-custom-steps-and-general-branches-pending |
| VF-0031 | View / Perspective | VM04, VM13 | selected-camera-state-and-byte-roundtrip-verified / rendering-custom-steps-and-general-branches-pending |
| VF-0032 | View / Zoom In<br>Horizontal toolbar / Zoom in | VM04, VM13 | selected-camera-state-and-byte-roundtrip-verified / rendering-custom-steps-and-general-branches-pending |
| VF-0033 | View / Zoom Out<br>Horizontal toolbar / Zoom out | VM04, VM13 | selected-camera-state-and-byte-roundtrip-verified / rendering-custom-steps-and-general-branches-pending |
| VF-0034 | View / Fit to Screen<br>Horizontal toolbar / Fit to screen | VM04, VM13 | selected-camera-state-and-byte-roundtrip-verified / rendering-custom-steps-and-general-branches-pending |
| VF-0035 | View / Overall Appearance... | VM04, VM13 | partial behavior observed; see behavior/01 and format notes |
| VF-0036 | View / Clear Text Area | VM04, VM13 | selected-Output-clear-and-model-byte-preservation-verified / other-text-tabs-and-undo-pending |
| VF-0037 | Objects / Structural Models / Show Models<br>Side panel / Style / Structural Models / Show models | VM04, VM12 | selected-toggle-or-style-state-save-fresh-read-verified / conflicts-renderer-and-general-branches-pending |
| VF-0038 | Objects / Structural Models / Show Only Asymmetric Unit<br>Side panel / Style / Structural Models / Show only asymmetric unit | VM04, VM12 | selected-inversion-filter-and-toggle-persistence-verified / full-visibility-and-renderer-pending |
| VF-0039 | Objects / Structural Models / Show Dot Surface<br>Side panel / Style / Structural Models / Show dot surface | VM04, VM12 | selected-toggle-or-style-state-save-fresh-read-verified / conflicts-renderer-and-general-branches-pending |
| VF-0040 | Objects / Structural Models / Ball-and-Stick<br>Side panel / Style / Structural Models / Ball-and-stick | VM04, VM12 | selected-style-AX-and-serialized-state-verified / full-rendering-and-general-cases-pending |
| VF-0041 | Objects / Structural Models / Space-Filling<br>Side panel / Style / Structural Models / Space-filling | VM04, VM12 | selected-style-AX-and-serialized-state-verified / full-rendering-and-general-cases-pending |
| VF-0042 | Objects / Structural Models / Polyhedral<br>Side panel / Style / Structural Models / Polyhedral | VM04, VM12, VM11 | selected-polyhedra-formulas-and-style-state-verified / full-polyhedra-rendering-pending |
| VF-0043 | Objects / Structural Models / Wireframe<br>Side panel / Style / Structural Models / Wireframe | VM04, VM12 | selected-toggle-or-style-state-save-fresh-read-verified / conflicts-renderer-and-general-branches-pending |
| VF-0044 | Objects / Structural Models / Stick<br>Side panel / Style / Structural Models / Stick | VM04, VM12 | selected-style-AX-and-serialized-state-verified / full-rendering-and-general-cases-pending |
| VF-0045 | Objects / Volumetric Data / Show Sections<br>Side panel / Style / Volumetric Data / Show sections | VM04, VM12 | selected-volume-toggle-AX-and-file-roundtrip-verified / renderer-resources-and-general-branches-pending |
| VF-0046 | Objects / Volumetric Data / Show Isosurfaces<br>Side panel / Style / Volumetric Data / Show isosurfaces | VM04, VM12 | selected-volume-toggle-AX-and-file-roundtrip-verified / renderer-resources-and-general-branches-pending |
| VF-0047 | Objects / Volumetric Data / Surface Coloring<br>Side panel / Style / Volumetric Data / Surface coloring | VM04, VM12 | selected-secondary-grid-enables-Surface-coloring-observed / coloring-numerics-pending |
| VF-0048 | Objects / Volumetric Data / Smooth Shading<br>Side panel / Style / Volumetric Data / Smooth shading | VM04, VM12 | selected-volume-style-AX-and-serialized-state-verified / renderer-and-general-branches-pending |
| VF-0049 | Objects / Volumetric Data / Wireframe<br>Side panel / Style / Volumetric Data / Wireframe | VM04, VM12 | selected-volume-style-AX-and-serialized-state-verified / renderer-and-general-branches-pending |
| VF-0050 | Objects / Volumetric Data / Dot Surface<br>Side panel / Style / Volumetric Data / Dot surface | VM04, VM12 | selected-volume-style-AX-and-serialized-state-verified / renderer-and-general-branches-pending |
| VF-0051 | Objects / Crystal Shapes / Show Shapes<br>Side panel / Style / Crystal Shapes / Show shapes | VM04, VM12 | partial / first-menu-no-effect failure retained; current-format and sidebar-cycle holdouts support sync explanation, cause pending |
| VF-0052 | Objects / Crystal Shapes / Unicolor<br>Side panel / Style / Crystal Shapes / Unicolor | VM04, VM12 | selected-shape-style-AX-and-file-roundtrip-verified / geometry-colors-renderer-and-general-cases-pending |
| VF-0053 | Objects / Crystal Shapes / Custom Color<br>Side panel / Style / Crystal Shapes / Custom color | VM04, VM12 | selected-shape-style-AX-and-file-roundtrip-verified / geometry-colors-renderer-and-general-cases-pending |
| VF-0054 | Objects / Crystal Shapes / Wireframe<br>Side panel / Style / Crystal Shapes / Wireframe | VM04, VM12 | selected-shape-style-AX-and-file-roundtrip-verified / geometry-colors-renderer-and-general-cases-pending |
| VF-0055 | Objects / Properties / General...<br>Side panel / Style / Properties... | VM04, VM12 | partial / selected General Preview-off isolation and Preview-on COMPS application verified |
| VF-0056 | Objects / Properties / Atoms... | VM04, VM12 | selected-Atoms-mode-flag-and-Cancel-contracts-verified / remaining Atoms controls and renderer pending |
| VF-0057 | Objects / Properties / Bonds... | VM04, VM12 | selected Bond conflict readback and manual normalization verified / rendering and broader persistence pending |
| VF-0058 | Objects / Properties / Polyhedra... | VM04, VM12 | selected Polyhedra edge and linewidth persistence/Cancel verified / material/rendering broader behavior pending |
| VF-0059 | Objects / Properties / Isosurfaces... | VM04, VM12 | selected Isosurfaces front/inverse persistence and Cancel verified / resource reopen and rendering pending |
| VF-0060 | Objects / Properties / Sections... | VM04, VM12 | selected Sections flags, absolute range coupling and Cancel verified / rendering and contour numerics pending |
| VF-0061 | Objects / Boundary...<br>Side panel / Boundary... | VM04, VM12 | selected-periodic-range-roundtrip-verified / full-boundary-pending |
| VF-0062 | Objects / Orientation...<br>Keyboard / macOS Cmd+Shift+O (observed from Comment focus)<br>Side panel / Orientation... | VM04, VM12 | selected-behavior-observed / remaining-branches-pending |
| VF-0063 | Utilities / Equivalent Positions... | VM04, VM14 | partial behavior observed; see behavior/01 and format notes |
| VF-0064 | Utilities / Geometrical Parameters...<br>CLI / -nogui -i PROJECT -list bond,angle,poly | VM04, VM14 | partial: R80 fresh relative-link reopen/two-atom visual highlight and bad-count link-clear SaveAs validated; stale detail/timing miss retained; full external covariance/general formats pending |
| VF-0065 | Utilities / Standardization of Crystal Data | VM04, VM14 | selected-P1-standardization-geometry-and-metadata-verified / other-groups-and-settings-pending |
| VF-0066 | Utilities / Niggli-Reduced Cell | VM04, VM14 | selected-orthogonal-skew-reduced-report-and-active-state-verified / general-reduction-pending |
| VF-0067 | Utilities / Site Potentials and Madelung Energy... | VM04, VM14 | partial behavior observed; see behavior/01 and format notes |
| VF-0068 | Utilities / Powder Diffraction Pattern...<br>Powder window / File / Import Data<br>Powder window / File / Export Graph<br>Powder window / File / Export Data (profile data; distinct from main Export Data...) | VM04, VM14, EXT-POWDER-IDA1998, EXT-POWDER-ITC2019, EXT-POWDER-TCH1987 | partial: R78 prospective native half-angle shift/nondefault LP/corrected-width normalized interior passes; R77 failure retained; first-index science failure/absolute F/full powder pending |
| VF-0069 | Utilities / Structure Factors...<br>CLI / -nogui -i FILE -calcf source=N lambda=A resolution=d anomalous=N out=FILE | VM04, VM14, EXT-DABAX-WAAS, EXT-IUCR-ADP-CIF, EXT-IUCR-INTENSITY, EXT-NIST-NEUTRON, VM06 | selected-model-volume-and-CLI-neutral-Xray-neutron-scattering-verified / anomalous-full-scattering-pending |
| VF-0070 | Utilities / Fourier Synthesis... | VM04, VM14 | selected-analytic-single-and-three-axis-inverse-Fourier-verified / full-Fourier-pending |
| VF-0071 | Utilities / Model Electron Densities... | VM04, VM14, VM18 | selected-occupancy-symmetry-density-DC-verified / full-density-pending |
| VF-0072 | Utilities / Model Nuclear Densities... | VM04, VM14, VM14 14.10-14.11 | selected-zero-mode-and-Patterson-invariants-verified / full-voxel-and-general-domain-pending |
| VF-0073 | Utilities / Patterson Densities / From Model Electron Densities... | VM04, VM14, VM14 14.10-14.11 | selected-zero-mode-and-Patterson-invariants-verified / full-voxel-and-general-domain-pending |
| VF-0074 | Utilities / Patterson Densities / From Model Nuclear Densities... | VM04, VM14, VM14 14.10-14.11 | selected-zero-mode-and-Patterson-invariants-verified / full-voxel-and-general-domain-pending |
| VF-0075 | Utilities / Patterson Densities / From Volumetric Data | VM04, VM14 | selected-single-and-three-axis-volumetric-Patterson-verified / full-Patterson-pending |
| VF-0076 | Utilities / 2D Data Display... | VM04, VM14, VM15 | selected-vector-slice-and-projection-contracts-observed / full-2D-pending |
| VF-0077 | Utilities / Line Profile... | VM04, VM14 | selected-line-periodic-negative-reverse-count-normalization-verified / remaining-line-errors-pending |
| VF-0078 | Utilities / Peak Search... | VM04, VM14, VM18 | selected-signed-coupled-grid-peaks-and-integrals-verified / general-search-pending |
| VF-0079 | Utilities / Voronoi Integration of Densities... | VM04, VM14 | selected-menu-alias-and-26-neighbor-boundary-subvoxel-holdout-verified / general-Voronoi-pending |
| VF-0080 | Utilities / Integrate Voxel Data in Each Polyhedron | VM04, VM14 | current-baseline-arm64-menu-dispatch-to-empty-handler-verified / other-architectures-and-versions-pending |
| VF-0081 | Utilities / Conversion of Electron Densities... | VM04, VM14 | selected-constant-and-3D-nonorthogonal-unit-and-threshold-conversion-contracts-verified / full-conversion-pending |
| VF-0082 | Help / Manual | VM04, VM04 | entry-observed / behavior-pending |
| VF-0083 | Help / Check for Updates... | VM04, VM04 | entry-observed / behavior-pending |
| VF-0084 | File / New Structure... / Phase / New<br>Edit / Edit Data / Phase / New | VM06 | entry-observed / behavior-pending |
| VF-0085 | File / New Structure... / Phase / Delete<br>Edit / Edit Data / Phase / Delete | VM06 | entry-observed / behavior-pending |
| VF-0086 | File / New Structure... / Phase / Clear<br>Edit / Edit Data / Phase / Clear | VM06 | entry-observed / behavior-pending |
| VF-0087 | File / New Structure... / Phase / ↑<br>Edit / Edit Data / Phase / ↑ | VM06 | entry-observed / behavior-pending |
| VF-0088 | File / New Structure... / Phase / ↓<br>Edit / Edit Data / Phase / ↓ | VM06 | entry-observed / behavior-pending |
| VF-0089 | File / New Structure... / Phase / Copy<br>Edit / Edit Data / Phase / Copy | VM06 | selected-context-observed / remaining-branches-pending |
| VF-0090 | File / New Structure... / Phase / Import...<br>Edit / Edit Data / Phase / Import... | VM06 | entry-observed / behavior-pending |
| VF-0091 | File / New Structure... / Unit cell / Magnetic structure | VM06, EXT-MAGCIF-098, EXT-MAG-TRANSFORM-2023, VM09 | partial / selected magnetic symmetry reduction and persistent vector loss observed; general and parity cases pending |
| VF-0092 | File / New Structure... / Unit cell / Transform... | VM06 | selected-P1-transform-modes-and-anisotropic-basis-changes-verified-at-saved-precision / general-transform-pending |
| VF-0093 | File / New Structure... / Unit cell / Customize... | VM06 | entry-observed / behavior-pending |
| VF-0094 | File / New Structure... / Unit cell / Update structure parameters to keep 3D geometry | VM06 | selected-P1-transform-modes-and-anisotropic-basis-changes-verified-at-saved-precision / general-transform-pending |
| VF-0095 | File / New Structure... / Unit cell / Remove symmetry | VM06, VM09, EXT-MAG-TRANSFORM-2023 | partial / selected magnetic symmetry reduction and persistent vector loss observed; general and parity cases pending |
| VF-0096 | File / New Structure... / Structure parameters / Symbol... | VM06 | entry-observed / behavior-pending |
| VF-0097 | File / New Structure... / Structure parameters / None | VM06 | entry-observed / behavior-pending |
| VF-0098 | File / New Structure... / Structure parameters / U | VM06 | entry-observed / behavior-pending |
| VF-0099 | File / New Structure... / Structure parameters / New | VM06 | entry-observed / behavior-pending |
| VF-0100 | File / New Structure... / Structure parameters / Delete | VM06 | entry-observed / behavior-pending |
| VF-0101 | File / New Structure... / Structure parameters / Clear | VM06 | entry-observed / behavior-pending |
| VF-0102 | File / New Structure... / Structure parameters / ↑ | VM06 | entry-observed / behavior-pending |
| VF-0103 | File / New Structure... / Structure parameters / ↓ | VM06 | entry-observed / behavior-pending |
| VF-0104 | File / New Structure... / Structure parameters / Import... | VM06 | entry-observed / behavior-pending |
| VF-0105 | File / New Structure... / Structure parameters / Link | VM06 | entry-observed / behavior-pending |
| VF-0106 | File / New Structure... / Structure parameters / Remove duplicate atoms<br>CLI / -nogui -i FILE -merge_split_site [dmin=f] -save FILE | VM06 | selected-CLI-exact-and-nearby-duplicate-threshold-counts-verified / GUI-metadata-general-removal-pending |
| VF-0107 | File / New Structure... / Volumetric data / Import... | VM06 | entry-observed / behavior-pending |
| VF-0108 | File / New Structure... / Volumetric data / Delete | VM06 | entry-observed / behavior-pending |
| VF-0109 | File / New Structure... / Volumetric data / Import... (second control) | VM06 | entry-observed / behavior-pending |
| VF-0110 | File / New Structure... / Volumetric data / Delete (second control) | VM06 | entry-observed / behavior-pending |
| VF-0111 | File / New Structure... / Crystal shape / Apply symmetry operations<br>Edit / Edit Data / Crystal shape / Apply symmetry operations | VM06 | entry-observed / behavior-pending |
| VF-0112 | File / New Structure... / Crystal shape / New<br>Edit / Edit Data / Crystal shape / New | VM06 | entry-observed / behavior-pending |
| VF-0113 | File / New Structure... / Crystal shape / Delete<br>Edit / Edit Data / Crystal shape / Delete | VM06 | entry-observed / behavior-pending |
| VF-0114 | File / New Structure... / Crystal shape / Clear<br>Edit / Edit Data / Crystal shape / Clear | VM06 | entry-observed / behavior-pending |
| VF-0115 | File / New Structure... / Crystal shape / ↑<br>Edit / Edit Data / Crystal shape / ↑ | VM06 | entry-observed / behavior-pending |
| VF-0116 | File / New Structure... / Crystal shape / ↓<br>Edit / Edit Data / Crystal shape / ↓ | VM06 | entry-observed / behavior-pending |
| VF-0117 | Objects / Properties / General / Do not show | VM12 | selected-General-field-serialization-and-terminal-action-verified / close-input-write-and-general-branches-pending |
| VF-0118 | Objects / Properties / General / Single unit cell | VM12 | selected-General-field-serialization-and-terminal-action-verified / close-input-write-and-general-branches-pending |
| VF-0119 | Objects / Properties / General / All unit cells | VM12 | selected-General-field-serialization-and-terminal-action-verified / close-input-write-and-general-branches-pending |
| VF-0120 | Objects / Properties / General / Solid lines | VM12 | selected-General-field-serialization-and-terminal-action-verified / close-input-write-and-general-branches-pending |
| VF-0121 | Objects / Properties / General / Dotted lines | VM12 | selected-General-field-serialization-and-terminal-action-verified / close-input-write-and-general-branches-pending |
| VF-0122 | Objects / Properties / General / Dashed lines | VM12 | selected-General-field-serialization-and-terminal-action-verified / close-input-write-and-general-branches-pending |
| VF-0123 | Objects / Properties / General / Show compass | VM12 | partial / COMPS encoding and OK commit verified; unexpected working-input exit write retained, cause pending |
| VF-0124 | Objects / Properties / General / Show axis labels | VM12 | partial / COMPS encoding and OK commit verified; unexpected working-input exit write retained, cause pending |
| VF-0125 | Objects / Properties / General / Show edges | VM12 | partial / edge serialization verified; selected Cancel leaves edge change, full rendering pending |
| VF-0126 | Objects / Properties / Atoms / Show as balls | VM12 | selected-Atoms-field-and-Cancel-serialization-verified / renderer-and-positive-visual-contexts-pending |
| VF-0127 | Objects / Properties / Atoms / Show as displacement ellipsoids | VM12 | selected-Atoms-field-and-Cancel-serialization-verified / renderer-and-positive-visual-contexts-pending |
| VF-0128 | Objects / Properties / Atoms / Show principal ellipses | VM12 | selected-Atoms-field-and-Cancel-serialization-verified / renderer-and-positive-visual-contexts-pending |
| VF-0129 | Objects / Properties / Atoms / Hide nonbonding atoms | VM12 | selected-Atoms-field-and-Cancel-serialization-verified / renderer-and-positive-visual-contexts-pending |
| VF-0130 | Objects / Properties / Atoms / Mark enantiomorphic sites | VM12 | partial / mark encoding verified; Cancel retains mark, chiral-pair rendering pending |
| VF-0131 | Objects / Properties / Bonds / Unicolor cylinder | VM12 | partial / coupled Unicolor persistence and unedited conflict Cancel verified; rendering pending |
| VF-0132 | Objects / Properties / Bonds / Bicolor cylinder | VM12 | partial / Bicolor global selection, sameclick preservation and manual rule synchronization verified |
| VF-0133 | Objects / Properties / Bonds / Color line | VM12 | partial / coupled BONDS-SBOND style encoding verified; Cancel retains rule style, rendering/reopen pending |
| VF-0134 | Objects / Properties / Bonds / Gradient line | VM12 | partial / Gradient conflict readback and manual synchronization verified; rendered precedence pending |
| VF-0135 | Objects / Properties / Bonds / Dotted line | VM12 | partial / coupled BONDS-SBOND style encoding verified; Cancel retains rule style, rendering/reopen pending |
| VF-0136 | Objects / Properties / Bonds / Dashed line | VM12 | partial / coupled BONDS-SBOND style encoding verified; Cancel retains rule style, rendering/reopen pending |
| VF-0137 | Objects / Properties / Polyhedra / Show polyhedral edges | VM12 | partial / Polyhedra edge checkbox, dependent controls, encoding and changed-state Cancel verified |
| VF-0138 | Objects / Properties / Isosurfaces / Render from front to back | VM12 | partial / front-back bit13 encoding, native selection and changed-state Cancel verified |
| VF-0139 | Objects / Properties / Isosurfaces / Inverse color | VM12 | partial / inverse signed TEX3P encoding, native selection and changed-state Cancel verified |
| VF-0140 | Objects / Properties / Sections / Absolute values | VM12 | partial / Absolute values bit0 and coupled min0 encoding/UI/Cancel verified |
| VF-0141 | Objects / Properties / Sections / Assign colors recursively | VM12 | partial / recursive color bit1 encoding/UI and changed-state Cancel verified |
| VF-0142 | Objects / Properties / Sections / Draw contour lines | VM12 | partial / contour line bit6 encoding/UI and changed-state Cancel verified |
| VF-0143 | Objects / Properties / Preview, Cancel | VM12 | partial / selected Preview-off isolation, Preview-on application and Cancel verified; prior retained-field failures remain |
| VF-0144 | Objects / Properties / Save as Default | VM12 | partial / selected default copy-exit-persist-reload verified; FORMP excluded from selected copy |
| VF-0145 | Edit Data / Unit cell / Lattice parameters | VM06, VM11 | selected-CIF-uncertainty-readback-verified / full-fields-pending |
| VF-0146 | Edit Data / Phase / Positioning | VM07 | entry-observed / behavior-pending |
| VF-0147 | Edit Data / Phase / Orientation | VM07 | selected-context-observed / remaining-branches-pending |
| VF-0148 | Edit Data / Structure parameters / site fields | VM06, VM11, EXT-DABAX-WAAS, EXT-IUCR-ADP-CIF, EXT-IUCR-INTENSITY | selected-CIF-uncertainty-readback-verified / full-fields-pending |
| VF-0149 | Edit Data / Structure parameters / ADP fields | VM06, EXT-DABAX-WAAS, EXT-IUCR-ADP-CIF, EXT-IUCR-INTENSITY | selected-imported-iso-aniso-nonorthogonal-U-and-Ueq-verified / editing-all-transforms-pending |
| VF-0150 | Edit Data / Volumetric data / Interpolation | VM06 | entry-observed / behavior-pending |
| VF-0151 | Edit Data / Crystal shape / face fields<br>Objects / Shape / selected form distance cell / F2<br>Objects / Shape / selected form color cell | VM06 | selected-Shape-F2-distance-and-color-entry-observed / full-shape-fields-pending |
| VF-0152 | Properties / General / line fields | VM12 | partial / selected unit cell line width and RGB encoding/UI/Cancel verified; range and rendering pending |
| VF-0153 | Properties / Atoms and Bonds / Material, Resolution | VM12 | partial / selected Atoms, Bonds and Polyhedra material and tessellation controls verified; broader inputs and rendering pending |
| VF-0154 | Properties / Atoms / Radius and color<br>Objects / Atoms / selected site radius cell / F2<br>Objects / Atoms / site color cell | VM12 | partial / selected Si atomic radius and RGB encoding/UI/Cancel verified; range, other elements and rendering pending |
| VF-0155 | Properties / Atoms / Radii type | VM12 | partial / selected SiO6 Atomic to Ionic and van der Waals UI/encoding/Cancel/reopen verified; broader elements and rendering pending |
| VF-0156 | Properties / Atoms / Probability, Outline, Scale isotropic atoms | VM12 | partial / selected ADP controls and numeric out-of-range text commits verified; other parsing and scientific rendering pending |
| VF-0157 | Properties / Atoms / Label | VM12 | partial / selected label type and z offset UI/serialization/Cancel verified; rendering and broader inputs pending |
| VF-0158 | Properties / Bonds / Radius and color | VM12 | partial / selected Bond radius, line width and RGB coupling and retained-rule Cancel verified; rendering and broader semantics pending |
| VF-0159 | Properties / Polyhedra / Planes and Edges | VM12 | partial / selected Polyhedra edge and face RGBA/opacity encoding verified; face Cancel retains ATOMT; rendering and broader inputs pending |
| VF-0160 | Properties / Isosurfaces / levels | VM12 | entry-observed / behavior-pending |
| VF-0161 | Properties / Isosurfaces / Surface coloring | VM12 | entry-observed / behavior-pending |
| VF-0162 | Properties / Sections / Sections and slices | VM12 | entry-observed / behavior-pending |
| VF-0163 | Properties / Sections / Cutoff levels | VM12 | entry-observed / behavior-pending |
| VF-0164 | Properties / Sections / Contours | VM12 | entry-observed / behavior-pending |
| VF-0165 | Preferences / Settings for RIETAN | VM16 | entry-observed / behavior-pending |
| VF-0166 | Preferences / Font for Text Area | VM16 | entry-observed / behavior-pending |
| VF-0167 | Preferences / Open a new file in | VM16 | entry-observed / behavior-pending |
| VF-0168 | Preferences / Animation | VM16 | entry-observed / behavior-pending |
| VF-0169 | Preferences / Start-up search for bonds | VM16 | entry-observed / behavior-pending |
| VF-0170 | Preferences / Enable hardware anti aliasing | VM16 | entry-observed / behavior-pending |
| VF-0171 | Preferences / Increasing factor for stacks/slices | VM16 | entry-observed / behavior-pending |
| VF-0172 | Preferences / Default isosurface level | VM16 | entry-observed / behavior-pending |
| VF-0173 | Preferences / Max. number of history | VM16 | entry-observed / behavior-pending |
| VF-0174 | Edit / Bonds / Search mode, Boundary mode | VM08 | selected-recursive-to-inbox-controls-observed / full-search-modes-pending |
| VF-0175 | Edit / Bonds / A1, A2, Search by label, length, Show polyhedra | VM08, VM11 | selected-range-verified / other-policies-pending |
| VF-0176 | Edit / Vectors / Crystallographic Sites, Individual Atom, Vectors | VM09 | entry-observed / behavior-pending |
| VF-0177 | Edit / Vectors / Create a New Vector / Type, components | VM09, EXT-MAGCIF-098, EXT-MAG-TRANSFORM-2023, VM06, VM17 | partial / persisted GUI Y1 to Y0.1 discrepancy verified; headless preserves Y1; exact GUI stage and general vector contract pending |
| VF-0178 | Edit / Vectors / Create a New Vector / appearance | VM09 | entry-observed / behavior-pending |
| VF-0179 | Edit / Lattice Planes / New, Delete, Clear, order, best plane | VM09 | selected-default-plane-creates-Slices-context-observed / order-delete-bestfit-pending |
| VF-0180 | Edit / Lattice Planes / fields<br>Objects / Slices / selected distance cell / F2<br>Objects / Slices / selected color cell / Space<br>Objects / Slices / color right half / alpha popup | VM09 | selected-Slices-F2-distance-and-RGB-chooser-observed / opacity-mouse-and-full-fields-pending |
| VF-0181 | Objects / Boundary / ranges, Apply | VM10 | selected-boundary-Apply-Cancel-observed / full-boundary-pending |
| VF-0182 | Objects / Boundary / Cutoff planes | VM10 | entry-observed / behavior-pending |
| VF-0183 | Objects / Orientation / modes, vectors, matrix | VM10 | selected-matrix-read-and-accelerator-observed / full-orientation-edit-pending |
| VF-0184 | Horizontal toolbar / a | VM04, VMA | entry-observed / behavior-pending |
| VF-0185 | Horizontal toolbar / b | VM04, VMA | entry-observed / behavior-pending |
| VF-0186 | Horizontal toolbar / c | VM04, VMA | entry-observed / behavior-pending |
| VF-0187 | Horizontal toolbar / a* | VM04, VMA | entry-observed / behavior-pending |
| VF-0188 | Horizontal toolbar / b* | VM04, VMA | entry-observed / behavior-pending |
| VF-0189 | Horizontal toolbar / c* | VM04, VMA | entry-observed / behavior-pending |
| VF-0190 | Horizontal toolbar / Standard orientation | VM04, VMA | entry-observed / behavior-pending |
| VF-0191 | Horizontal toolbar / Rotate -x | VM04, VMA | entry-observed / behavior-pending |
| VF-0192 | Horizontal toolbar / Rotate +x | VM04, VMA | selected-plus-x-45-degree-orientation-observed / other-rotations-pending |
| VF-0193 | Horizontal toolbar / Rotate -y | VM04, VMA | entry-observed / behavior-pending |
| VF-0194 | Horizontal toolbar / Rotate +y | VM04, VMA | entry-observed / behavior-pending |
| VF-0195 | Horizontal toolbar / Rotate -z | VM04, VMA | entry-observed / behavior-pending |
| VF-0196 | Horizontal toolbar / Rotate +z | VM04, VMA | entry-observed / behavior-pending |
| VF-0197 | Horizontal toolbar / Translate upward | VM04, VMA | entry-observed / behavior-pending |
| VF-0198 | Horizontal toolbar / Translate downward | VM04, VMA | entry-observed / behavior-pending |
| VF-0199 | Horizontal toolbar / Translate leftward | VM04, VMA | entry-observed / behavior-pending |
| VF-0200 | Horizontal toolbar / Translate rightward | VM04, VMA | entry-observed / behavior-pending |
| VF-0201 | Horizontal toolbar / Rotation step (degrees) | VM04, VMA | entry-observed / behavior-pending |
| VF-0202 | Horizontal toolbar / Translation step (pixels) | VM04, VMA | entry-observed / behavior-pending |
| VF-0203 | Horizontal toolbar / Zoom step (percent) | VM04, VMA | entry-observed / behavior-pending |
| VF-0204 | Vertical toolbar / Rotate ; graphics key r | VM11, VMA | selected-arrow-modifier-and-letter-input-observed / physical-key-platform-context-pending |
| VF-0205 | Vertical toolbar / Select ; graphics key s | VM11, VMA | selected-Graphics-letter-tool-switch-observed / full-input-context-pending |
| VF-0206 | Vertical toolbar / Translate ; graphics key t | VM11, VMA | selected-Graphics-letter-tool-switch-observed / full-input-context-pending |
| VF-0207 | Vertical toolbar / Magnify ; graphics key m | VM11, VMA | selected-Graphics-letter-tool-switch-observed / full-input-context-pending |
| VF-0208 | Vertical toolbar / Distance ; graphics key d | VM11, VMA | selected-distance-diagonal-SUE-and-display-verified / full-geometry-pending |
| VF-0209 | Vertical toolbar / Angle ; graphics key a | VM11, VMA | selected-nonperiodic-and-nonorthogonal-image-angles-verified / full-geometry-pending |
| VF-0210 | Vertical toolbar / Dihedral angle | VM11, VMA | selected-independent-angle-controls-verified / full-geometry-pending |
| VF-0211 | Vertical toolbar / Interfacial angle | VM11, VMA | entry-observed / behavior-pending |
| VF-0212 | Tools / Rotate / Drag | VM11 | entry-observed / behavior-pending |
| VF-0213 | Tools / Rotate / Animation | VM11 | selected-animation-Click-Push-Random-continuity-stop-observed / rates-and-general-inputs-pending |
| VF-0214 | Tools / Rotate / Free rotation | VM11 | entry-observed / behavior-pending |
| VF-0215 | Tools / Rotate / Around X axis | VM11 | entry-observed / behavior-pending |
| VF-0216 | Tools / Rotate / Around Y axis | VM11 | entry-observed / behavior-pending |
| VF-0217 | Tools / Rotate / Around Z axis | VM11 | entry-observed / behavior-pending |
| VF-0218 | Main window / Tools tab | VM04, VM12 | entry-observed / behavior-pending |
| VF-0219 | Main window / Style tab | VM04, VM12 | entry-observed / behavior-pending |
| VF-0220 | Main window / Objects tab<br>Objects / Toggle data / <, >, phase number<br>Objects / Slices / color right half / alpha popup | VM04, VM12 | selected-all-five-Objects-types-phase-and-F2-contexts-observed / opacity-pointer-trigger-pending |
| VF-0221 | Main window / Output tab | VM04, VM12, VM11, VM14, EXT-DABAX-WAAS | selected-polyhedron-report-verified / full-output-pending |
| VF-0222 | Main window / Summary tab | VM04, VM12 | selected-known-cell-title-BNS-display-verified / grids-multiphase-general-summary-pending |
| VF-0223 | Main window / Comment tab<br>Comment / selected text / Control-click | VM04, VM12 | partial / Unicode-comment-save-reopen observed with original exact-tail failure; input/commit cause unisolated |
| VF-0224 | Graphics / Select / click atom | VM11 | selected-Shift-add-replace-marquee-Rotate-doubleclick-observed / occlusion-and-other-objects-pending |
| VF-0225 | Graphics / Delete ; Escape<br>Objects / selected atom S cell / BackSpace or Delete | VM11 | selected-Graphics-text-numeric-and-list-Delete-focus-observed / IME-generalcontext-pending |
| VF-0226 | Overall Appearance / Background | VM13 | dialog/control observed; branch behavior pending |
| VF-0227 | Overall Appearance / Lighting | VM13 | dialog/control observed; branch behavior pending |
| VF-0228 | Overall Appearance / Projection | VM13 | dialog/control observed; branch behavior pending |
| VF-0229 | Overall Appearance / Depth cueing, Preview, Save as Default | VM13 | dialog/control observed; branch behavior pending |
| VF-0230 | Equivalent Positions / Site, Equivalent positions | VM14 | selected-inversion-affine-operation-text-verified / full-matrix-workflow-pending |
| VF-0231 | Geometrical Parameters / fields | VM14 | partial: R80 fresh relative-link reopen/two-atom visual highlight and bad-count link-clear SaveAs validated; stale detail/timing miss retained; full external covariance/general formats pending |
| VF-0232 | Site Potentials and Madelung Energy / Radius | VM14 | dialog/control observed; branch behavior pending |
| VF-0233 | Site Potentials and Madelung Energy / Calculation region | VM14 | dialog/control observed; branch behavior pending |
| VF-0234 | Powder Diffraction / Conditions / Radiation | VM14, EXT-POWDER-IDA1998, EXT-POWDER-ITC2019, EXT-POWDER-TCH1987 | partial: R78 prospective native half-angle shift/nondefault LP/corrected-width normalized interior passes; R77 failure retained; first-index science failure/absolute F/full powder pending |
| VF-0235 | Powder Diffraction / Conditions / Range, corrections | VM14, EXT-POWDER-IDA1998, EXT-POWDER-ITC2019, EXT-POWDER-TCH1987 | partial: R78 prospective native half-angle shift/nondefault LP/corrected-width normalized interior passes; R77 failure retained; first-index science failure/absolute F/full powder pending |
| VF-0236 | Powder Diffraction / Calculate, Close | VM14, EXT-POWDER-IDA1998, EXT-POWDER-ITC2019, EXT-POWDER-TCH1987 | partial: R78 prospective native half-angle shift/nondefault LP/corrected-width normalized interior passes; R77 failure retained; first-index science failure/absolute F/full powder pending |
| VF-0237 | Powder Diffraction / Reflections / Export<br>Powder window / File / Export Reflection Table | VM14, EXT-POWDER-IDA1998, EXT-POWDER-ITC2019, EXT-POWDER-TCH1987 | partial: R78 prospective native half-angle shift/nondefault LP/corrected-width normalized interior passes; R77 failure retained; first-index science failure/absolute F/full powder pending |
| VF-0238 | Structure Factors / Structural model, Radiation, d(min) | VM14, EXT-DABAX-WAAS, EXT-NIST-NEUTRON | selected-model-and-volume-dmin-controls-observed / full-controls-pending |
| VF-0239 | Structure Factors / Calculate, Export, Close | VM14, EXT-DABAX-WAAS, EXT-IUCR-ADP-CIF, EXT-IUCR-INTENSITY, EXT-NIST-NEUTRON, VM06 | selected-model-and-volume-calculation-export-verified / full-calculation-pending |
| VF-0240 | Fourier Synthesis / F(000), Resolution, reflections | VM14 | selected-F000-resolution-and-single-reflection-observed / full-controls-pending |
| VF-0241 | Fourier Synthesis / New, Delete, Clear, order, Import, Export | VM14 | selected-generic-eight-column-three-import-modes-verified / other-branches-pending |
| VF-0242 | Fourier Synthesis / Calculate | VM14 | selected-analytic-three-axis-phase-DC-and-Cube-layout-verified / full-calculate-pending |
| VF-0243 | Model Electron Densities / Resolution | VM14, VM18 | selected-resolution-field-and-grid-observed / full-grid-policy-pending |
| VF-0244 | 2D Data Display / General | VM15 | selected-grid-fill-bird-controls-and-inprocess-restoration-observed / color-range-general-manipulation-pending |
| VF-0245 | 2D Data Display / Contours | VM15 | selected-linear-log-controls-render-and-static-level-rules-observed / runtime-levels-and-style-equivalence-pending |
| VF-0246 | 2D Data Display / Slice / (hkl) plane in the bounding box | VM15 | selected-001-bbox-range-and-export-disabled-state-transition-observed / general-bbox-pending |
| VF-0247 | 2D Data Display / Slice / (hkl) plane defined by two vectors | VM15 | selected-001-vector-plane-interpolation-closed-grid-export-verified / general-vector-plane-pending |
| VF-0248 | 2D Data Display / Slice / Project along [hkl] axis | VM15 | selected-projection-raw-count-weight-mismatch-isolated / exact-integral-failure-and-general-projection-pending |
| VF-0249 | Line Profile / Fractional coordinates, Number of data points | VM14 | dialog/control observed; branch behavior pending |
| VF-0250 | Peak Search / Max. number of peaks, Search negative peaks | VM14, VM18 | selected-negative-max0-max1-reset-and-boundary-integration-contracts-verified / full-limits-pending |
| VF-0251 | Conversion of Electron Densities / led, ked, ped, ted, lned | VM14 | selected-five-field-3D-holdout-and-zero-negative-threshold-contracts-verified / full-domain-pending |
| VF-0252 | Conversion of Electron Densities / Angstrom, Atomic unit | VM14 | selected-orthogonal-nonorthogonal-Angstrom-atomic-conversion-factors-verified / other-cells-and-unit-variants-pending |
| VF-0253 | Export Data / File type (structure) | VM18 | selected-six-default-structure-exports-and-five-fresh-readbacks / full-GUI-options-and-retention-pending |
| VF-0254 | Export Data / File type (volume) | VM18 | selected-five-volume-default-exports-and-fresh-readbacks / m3d-loss-PRI-byte-contract-and-full-options-pending |

证据路径相对于 `output/vesta-study/20261108-113622/`。条目数量不等于穷尽或通过数量；所有未执行分支保留待核。

## 读取机制接续

VF-0009补名字识别/VS-0033失败与机制证据，VF-0013补生成密度工程的读取端诊断；仅在各自范围更新，仍partial。没有增加ID、将Save菜单未测状态提升为通过或宣称所有格式兼容。见[READER_PATHS](READER_PATHS.md)。

R10仍保留254个ID。VF-0009补入MXDORTO原样本信号退出、字段/索引机制和原件受控变体的300位点/900坐标证据，保持部分覆盖，不把单个诊断变体当完整格式矩阵验收。

R11仍254个ID。VF-0009补入nogui返回机制证据；全CLI命令全集/组合契约尚未盘完，不能用此条替代S07功能枚举。

R12保留254个ID。VF-0232/0233补入MADEL外部加载失败与GUI结果缺失分支证据，参数入口观察保留，领域数值未验收。

R13保留254个ID；VF-0009补入选定短读/负晶胞的接受机制和证据，不将原件未报错提升为有效输入或完整验证覆盖。

R14保留254个ID；截图接口负控制不能当视觉验收。独立进程启动触发单实例IPC busy，与New Window菜单不同，生命周期/参数交接需在S04/S07继续盘点，未扩大为产品实现。

R15保留254个ID，VF-0011所选数据库行为已补：[DATABASE](DATABASE.md)。原件HTTPS表单与副本HTTP错误轨迹支持空白原因；提供者、查询、下载/导入仍pending，不当全功能通过。

R16保留254个ID，VF-0012/0013/0018/0022/0062补选定GUI行为：[GUI_LIFECYCLE](GUI_LIFECYCLE.md)。GUI与nogui取向行为分开；Cancel来自第二次明确观察，第一次未观察转移不算通过；外改覆盖只是单次序列。

R17复用254个ID，补Phase/Crystal shape控制的Edit Data入口及触发条件，VF0021/0025/0089/0147有选定行为：[行为02](行为/02-结构与几何.md)。0原子配位多面体不等于没有晶形。Copy只核暂存/取消，New/Delete/排序/Import与Apply提交仍待测。

R18保持254个ID；VF0208/VF0026/VF0175补2 Å距离/四组有限区间与字段提交边界，见[ALGORITHMS](ALGORITHMS.md)。本轮No点击未执行/期限退出，关闭分支仍未通过，不以进程清理替代交互验收。

R19保持254个ID，VF0018补普通有名工程No/Yes正常退出与文件变化，VF0012补Quit/Yes保存路径；与R16 Cancel合用，仍不当全部关闭/写失败/多文档通过。见[GUI_LIFECYCLE](GUI_LIFECYCLE.md)。

R20保持254个功能ID，VF0015补PNG Scale/透明组合及实际文件，VF0016补Vector四格式菜单但无产物；Scale参数归Raster导出，VF0171 Preferences tessellation仍独立待核。[IMAGE_EXPORT](IMAGE_EXPORT.md)。样本新增图像代表VS0038，原37保持。

R21 VF0016补四种Vector实物、PDF/SVG独立解析渲染、EPS/PS结构范围与PNG外观差异；254功能ID保持。[IMAGE_EXPORT](IMAGE_EXPORT.md)。

R22 VF0015补全部Raster入口实跑、六种可自动解码/RAW无头/EPS位图及JP2无产物、RGB无效、TIFF透明无效；全选项与现场机制仍待核。[IMAGE_EXPORT](IMAGE_EXPORT.md)。254功能ID不变。

## R23 Raster三异常现场归因（不改计划勾选）

同TEXT/原库研究副本实测：JP2临时Save返回1→空转换返回0被忽略→删~→仍成功；RGB index7转自定义type4，坏文件与原件完全相同；TIFF生成实际alpha0..255却index9转RGB-only type6，实物仍与原件RGB TIFF相同。0回调错误/目标正常0，watch本轮未截断、原件不改。问题unk_ff152c8e4008160b rev3 resolved仅指原因查清，三项功能仍失败。208证据、10问题=7resolved3investigating，旧9头不变；254功能/50样本完全保持，副本产物不重复编样本ID。10完成19未完成、完整Study未完，Codex自验、dsh独立复审尚未开始。 [图像权威说明](IMAGE_EXPORT.md)。

## R24角与镜像二面角（不改勾选）

独立四原子XYZ预期键角90°、无符号二面角45°、CF=2√2Å；原件90.0000(0)、两镜像均45.0000(0)、2.82843(0)Å按显示精度符合。静态通用核心是归一化法向点积夹紧后acos，与无符号镜像结果一致；退化/PBC/SUE仍未核。Quit两文档依次保存提示/No，正常0/300.248秒，两源未改，dirty触发点未分离。220证据/10头不变，254功能/51样本，旧50保持、新VS0051，镜像变体不加ID。10完成19未完成，Study未完，Codex自验、dsh独立复审尚未开始。

## R25周期图像与边界（不改勾选）

正交P1双site主像距离9Å，选O(-1,0,0)像后1Å；测量使用所选图像，非自动全局最短像。默认显示4图像原子/2键，x上限1→2→1提交并读回，计数4/2→6/3→4/2。条件边端点枚举一致，但不宣称通用边界/三斜算法或化学成键。原件36492正常0/267.323秒、QuitNo/CIF未改、CUA已summary。227证据/10头不变，254功能/52样本，旧51保持、新VS0052；10完成19未完成，完整Study未完，Codex自验、dsh独立复审尚未开始。

## R26 VS0009非正交补测（不改勾选）

主像2.17945Å、(-1,0,0)像1.32288Å、两O像围C64.3066°与独立metric/点积显示精度相符。真实选中C/O规则是递归extraatoms；122/116→禁止越界10/4→小范围3/1，修改后规则/边界再次原生读回未完成。早期错误中心角79.1066保留，正确新组原子坐标另核。生命周期原37980期限-15，No未成功；AX重启空白38111已菜单关闭，无残留，源未改，不签正常关闭。232证据/10头不变，254功能/52样本身份摘要保持，仅VS9期望补证，其它51记录保持。10完成19未完成，完整Study未完，Codex自验、dsh独立复审未开始。

## R27反演一般/特殊位置（不改勾选）

独立P-1有理数去重C双像/O原点单重；原件C2i/O1a，闭胞10图像，xyz上限.99后3像，反演C坐标/操作实际读回符合。仅不对称显示checkbox1/0，CUA对象3→2→3，Output旧计数不刷新，不凭此宣称内部数组不变。组成CO/F00014仅独立条件预期，Summary未提供读回；Wyckoff函数定位不等完整算法。38361正常0/354.149秒、No/源未改，softstop360前关闭，无重启。241证据/10头不变，254功能/53样本，旧52保持、新VS0053；10完成19未完成，Study未完，Codex自验、dsh复审未开始。

## R28规则配位八面体（不改勾选）

初始Bonds规则空，明确Si/O1.9..2.1Å+Showpolyhedra1后7/6/1；全局Polyhedral/选择中心面，报告六Si-O2Å/平均2/体积10.6667Å³，与独立32/3按显示精度相符。distortion0/elongation1/ECN6仅规则基准，完整公式/非规则待；surfacearea未读回，Utilities/ORFFE未打开。体积/生成静态函数定位，浮点舍入指令存在不当完整算法/现场路径证明。原件38984正常0/309.52秒、softstop前No关闭、源未改。248证据/10头不变，254功能/54样本，旧53保持、新VS0054；10完成19未完成，Study未完、Codex自验、dsh复审未开始。

R30补原生窗口截图方法正控制，详见[WINDOW_CAPTURE](WINDOW_CAPTURE.md)。254功能条目与functions.json未改，截图归属问题解决不自动升级任何目标功能的行为状态。

## R80 ORFFE重开/高亮与坏读（不改勾选）

新实例相对IMPORT_ORFFE侧文件自动恢复，无手动Read；原生详情9.877(13)/Si1/O1一致。指定拥有PID91883/主窗口70518两张860×728原图目视确认选择行在O1/Si1产生黄圈，模型区黄色像素0→298，非全渲染验收。只删O6条目7→6后表空、无错误提示，顶部旧详情仍留；失败后原生UI读被180截止阻止，没有回执。坏读SaveAs清IMPORT_ORFFE，逐字节回到原R66无链接5144bytes；原输入/有效侧文件保持。GUI正常0/227.666<soft240，但超180目标47.666且清理晚，allTimingTargetsMet=false。6新/836Evidence、12头/73样本保持，254功能/32权威/10完成19未完成；详见ALGORITHMS R80，Codex自验，dsh复审未开始，完整ORFFE/Study待。


## R81 磁矩反演与时间反演（2026-11-09）

新增VS0074/75：10Å立方，Fe1(.1,.2,.3)，磁矩(1,2,3)μB；两个完整操作只在反演的时间符号±1上不同。首CLI前登记 [独立期望](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-before-targets.json)。一手Shinohara2023 §2.4 Eq7给m′=εdet(W)AW A⁻¹m；本案A=10I、W=−I，故普通反演保持磁矩、带时间反演则变号；magCIF字典只选读坐标/单位和xyz时间符号，全文阅读未完成。

原版两个-nogui导入/Save正常255，完整watch；实际MAGNETIC_CRYSTAL、SYMOP时间列±1、单site分配均保留。VECTR保存Cartesian(1,2,3)，末列type0/Axial（R83修正原坐标模式解释），原先预测晶格(.1,.2,.3)失败并留原计划；没有把表示差异当科学失败。原件自行识别2.4/P−1和2.6/P−1′，这些编号是目标观测，非本轮独立群号验算。[17项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/magnetic-crosscheck.json)。

两幅归属窗口原生PNG（872×558）已逐一目视：反演行−x/−y/−z、T+1显示mx/my/mz；T−1显示−mx/−my/−mz，符合独立规则。[普通反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/inversion-equivalent-ui.png) / [时间反演](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/anti-repeat-equivalent-ui.png)。wx表行不暴露于AX，此为原图视觉读数；没有提取实际展开原子上的数值磁矩，也未校准3D箭头方向/长度。polar、非正交、特殊位置、磁超空间与New-dialog编辑仍待，不把VF0091/VF0177或完整磁性科学标作完成。

首截图native调用尚未完成时Codex提前Close/Quit，实际screenshot_window_not_found/单AXApplication节点；失败与 [原因记录](../../output/vesta-study/20261108-113622/reviews/20261109-122500-s06-magnetic-symmetry/native-capture-failure.json) 保留。随后两个新归属进程分别33.382/30.084秒正常0退出，成功取证；首进程75.711秒也正常0，全部<180且无残留。CIF reader0xa671c已有静态证据ID去重，不重复增加记录；新增8条至844，12问题头/旧836证据/原件257/旧73样本不变，稳定样本增至75。80个历史manifest/11112文件完整。研究仍10完成19未完成、Codex自验，dsh独立复审未开始，无产品实现。


## R82 降对称后的第二位点矢量缺失（2026-11-09）

复用VS0075磁群2.6/P−1′，首GUI前登记 [坐标与两磁矩保留预期](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/axial-before-target.json)；VM06 §6.2.5/6.2.7与VM09 §9.1.1本轮只重读相关段落，不新增全文阅读声明。Remove symmetry→OK→新文件SaveAs实际得到磁群1.1/P1、两个独立Fe位点(.1,.2,.3)/(.9,.8,.7)，坐标展开符合，但VECTR与输入逐字相同，仅矢量1分配site1；第二个预期(−1,−2,−3)没有分配，**磁矩保留预期失败**。缺失矢量不是测得零磁矩。[11项证据核对及原科学失败](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/p1-crosscheck.json)。

结果后登记新进程重开对照：同一归属窗口两幅860×728原生PNG经目视，前2箭头、后1箭头，第二原子保持可见而箭头消失；选定ROI红像素840→0。[降对称前](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/before-main-ui.png) / [重开后](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/after-main-ui.png)。Vectors原图1034×333显示第1位点Vector1、第2位点空；wx行不在AX，由目视记录。[位点分配](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/after-vector-ui.png)。这是所测输入的持久化丢失行为，非完整3D渲染校准/一般P1转换验证。

原生OnRemoveSymmetry0x10003b0b4/184bytes在0x10003b0f0调用lib convertToP1 0x69c28/1748bytes；读取直接克隆XSite/TransformCoord路径，未隔离完整矢量丢失根因，不凭未见直接调用断言所有间接行为。两个归属GUI9852/10318正常0，51.365/102.995秒，均<180且无遗留。首分析脚本把真实9token STRUC行错设>9而返回空站点，[失败报告](../../output/vesta-study/20261108-113622/reviews/20261109-124500-s06-vector-parity/p1-crosscheck-initial.json)和初版脚本保留，改>=9后的新报告通过证据检查，原磁矩科学失败仍保留。一次CUA状态保护阻止旧动作，刷新AX后继续，不推断人为操作。

新增8至852Evidence，旧844/12问题头/75稳定样本/原件257与81历史manifest的11277文件不变。VF0095因新发现纳入规格，原VF0091/0177范围沿用；polar、非正交与实际展开数值磁矩仍待，未因坐标通过而关闭完整磁性/Study。254功能/32权威、10完成19未完成，Codex自验，dsh独立复审/S08未开始，无产品实现。


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


## R85 CLI全局样式与完整证据快照（2026-11-09）

新target前登记VECTS7、.125及7→.125双文件append三案；原版均正常255/完整watch，输出全局VECTS1、相数1/1/2，每相矢量(1,0,0)/Polar1/site1保留。静态cmdRead临时Scene读取后仅转移Crystal列表及文件名，未转移Scene全局XStyle，解释所测比例重置；未读取临时内存或验证所有样式字段/格式。[15项限定核对](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/style-crosscheck.json)。

锁定工具首次触发64MiB加载预算：原生export实际869/not_loaded2，磁盘871条均存在。两旧记录经原生按ID接口完整读取，与旧包/磁盘一致；[原始部分导出](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/native-evidence-final.json)保留，不改成全量；[完整核验派生快照871](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/verified-complete-evidence.json)明确为原生导出+原生ID解析的核验合并，[来源与身份](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/complete-evidence-provenance.json)、[预算重放](../../output/vesta-study/20261108-113622/reviews/20261109-131900-s07-cli-scene-style/method-load-budget-audit.json)可审。不得将wrapper0或导出记录数当完整性；后续须核not_loaded并按需补齐。

新增7至871原生来源记录，旧864/12头/77样本/原件257与84历史manifest/11713文件不变；raw导出869，完整核验871，两者须区分。未改工具二进制/锁定提交/工程历史记录，无产品实现。254功能/32权威、10完成19未完成，Codex自验，dsh独立复审/S08待。一般样式、非正交矢量及完整Study继续。


## R86 GUI样式保留、Cancel崩溃与数值隔离未成（2026-11-09）

首target前 [登记GUI5/传入7对照](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/gui-style-before-target.json)：当前一相X矢量(1,0,0)/全局5，传入新相Y矢量(0,1,0)/全局7，经短共享VESTA_PREF发送-i/-save。客户端0/exited，另核父进程仍活及新5347byte文件稳定；保存两相标题current_style_five/incoming_style_seven、全局VECTS5，前后原生AX比例5、后相下拉表含两名。所测“仅追加相数据、保留当前全局样式”得到GUI反例控制，非总重置1/非取传入7。未发送-refresh，未验第二相主模型渲染。

**原两项预期失败**：保存第二相rawY为.10000，预期1.00000，原始分量一致性失败；尚未证明重开后的物理磁矩损坏。另，父进程18317在Vectors的Cancel恢复预览时SIGSEGV11/−11，146.850秒<180且无残留，但正常退出/正常cleanup失败。先前对话称“Quit时”已被系统栈修正，不能把工具序列中的Quit当已证触发动作。[11项证据核对及原失败](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/gui-style-crosscheck.json)。

具体上下文：先在1相时打开/Cancel Vectors，再CLI追加为2相，重开Vectors查看相列表后Cancel。[系统报告原档（历史文件名含quit）](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/VESTA-18317-quit-crash.ips) 与 [地址/UUID/寄存器绑定](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/vector-cancel-crash-binding.json)：main UUID cd806142…，PC0x1000214e8执行ldp x1,x2,[x20]，x20/FAR=0x300；调用栈OnCancel→SetPreviewMode(false)→VectorBase赋值。还原循环以现有Scene相数为界，却索引backupScene+0x4a0的相数组，无旧长度检查，再加箭头向量偏移0x300；空源指针与报告吻合。旧备份实际长度未读内存，1→2相造成旧备份不匹配是有据推断，未宣称全部根因/所有Cancel场景。

崩溃后另登记两个无界面数值隔离对照；第一个Y-only在18秒idle后被SIGTERM终止（native code1/reason idle_timeout、wrapper0），无命令输出/无文件。研究脚本错误地先读预期文件而报FileNotFoundError；raw失败/脚本保留，第二个X/Y对照未调用，不能形成数值结论。[停止记录](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/headless-control-stop-record.json)。下轮先用新有界拥有实例检查是否有启动恢复提示；超时原因尚未确认，不原样盲重跑，也不重放已知两相Cancel崩溃。

新增7至878原生来源记录；raw原生导出876/not_loaded2，[完整核验派生快照878](../../output/vesta-study/20261108-113622/reviews/20261109-134100-s07-gui-style-retention/verified-complete-evidence.json) 保留旧871及12头、77样本、原件257、85历史manifest/11873文件。两个旧记录仍经原生ID解析补齐，无记录重写。254功能/32权威、10完成19未完成；Codex自验，dsh独立复审/S08待，无产品实现。

## S04.2–.4 入口归并与行为复核（2026-11-10）

本轮复用至 R132 的真实观察，仅新增一轮键盘/焦点探针。254 个 ID 保持原顺序，未重新编号；同一 ID 下的独立 child entry 保留完整路径和触发前提。当前 `functions.json` 为每个入口补 `entryConditions`（最低已知/文档条件、证据性质、复查卡）及 `s04Audit`；这些字段明确不代表完整运行 gate 或完整行为通过。原 `entry-observed / behavior-pending` 101 条中 8 条由旧已完成观察和本轮六种工具切换纠正为限定观察，现为 93 条；仍不以计数判断穷尽。

先核原生证据后，仅对新问题取证。菜单逐路径归并、69 份原始界面 surface、36 行快捷键、187 条手册候选和真正缺口分别见本轮证据包。187 中 11 条是数值表行误识别章节，176 条为文档章节族；原候选 JSON 不改。336 条 retained 菜单路径包含动态 recent-file 实例和调试副本菜单名，不等于 336 功能，更不等于只观察原件。OS Apple/Services/Window 及文本菜单按平台组明确归并，不执行 OS 服务，也不从重写范围删除相关用户能力。

VF-0017 误附的 R108/R109 体数据读取结论和引用已移回 VF-0009，R93/R94 输出文本事实仍在 VF-0017；错误前本、字段迁移和所有失败均保全。粉末窗口 File/Import Data 与 Export Graph 归于 VF-0068 的独立 child entry，Export Reflection Table 归 VF-0237；这是新登记的既有入口，不冒称导入/输出已通过。

S04.3 的初步行为规格覆盖选择、变换、属性、Undo/Redo、焦点与视图，逐项限定前提/动作/实际状态/证据。完整格式/科学数值/多相样本留 S05–S07。S04.2 仍未达到全部触发条件核实：键盘方向键/修饰键、动画 Click/Push/Random、Objects 表格内联/上下文与选择焦点组合仍留未勾选。S04.4 已形成版本差异与遗漏登记，但不会据此宣布 S04 或完整 Study 通过。

S04复核补正：粉末窗口 File/Export Data（无省略号）为VF0068下独立profile导出入口；主窗口Export Data...仍VF0014。R73已有.int实物/误标Save面板标题，不重跑。VM14 §14.11归VF0073–0075，§14.15归VF0081；不能按新增菜单后的序号算功能ID。

本轮 S04 触发补证：键盘18路径、Animation3模式、Objects部分行、选择/文本焦点、零原子晶面行和副体数据/粉末选择器，详见[矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-063443-s04-trigger-completion/S04-MATRIX.md)。93条入口级待核降至90仅表示三条状态有实际选定行为，不表示全部功能通过。

S04 Objects/focus补证：新增F2、Space、Control-click和相切换别名；K17归属由SaveOutputText纠正为Comment/Output，不增减254稳定ID。当前87条entry-only仅表示目录状态，不等于未运行数或全功能通过。详见[本轮矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-072741-s04-objects-focus/S04-MATRIX.md)。

## alpha运行时081848

本轮S04 alpha入口已有原件阳性：同PID激活后28×128浮窗与AXSlider192；诊断副本实测点击门槛/显示链。新增32原生至1826派生磁盘索引，254ID/78样本/12头/87entry-only保持。dsh复核pass、无硬阻断；S04.2按入口/条件登记范围接受，S04整卡收尾，Study14完成/15未完成。未进入S05或产品实现。 原件同PID：inactive时C右半点击后无屏上popup；访达Open既有应用激活后出现28×128/layer101浮窗，native AXSlider=192；再次CUA主窗观察后inactive且浮窗隐藏。诊断副本另实测mode2/column2/adjustedX25>16及alpha/Popup/Show链，不能冒充原件回调值。 原生unknown头未更新；截图not_found/首wrapper timeout/原件SIGTERM清理均保留。[证据](../../output/vesta-study/20261108-113622/reviews/20261110-081848-s04-alpha-runtime/runtime-report.json)。

当前dsh复核裁定：本轮S04 alpha入口已有原件阳性：同PID激活后28×128浮窗与AXSlider192；诊断副本实测点击门槛/显示链。新增32原生至1826派生磁盘索引，254ID/78样本/12头/87entry-only保持。dsh复核pass、无硬阻断；S04.2按入口/条件登记范围接受，S04整卡收尾，Study14完成/15未完成。未进入S05或产品实现。 只接受登记范围，保留全部失败与科学/平台未验。[实际复审](../../output/vesta-study/20261108-113622/reviews/20261110-081848-s04-alpha-runtime/review-result.json)。

## S05 格式专项核验（2026-11-10）

本轮仅扩展 VF-0009/0014/0253/0254，其余250条机器记录不变；254稳定功能ID不变，精确入口级待核仍87条。99名称分类不当99种兼容通过，11默认导出/10复读与轴序5导出5复读不当GUI全选项通过。[矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/S05-MATRIX.md)及[语义核对](../../output/vesta-study/20261108-113622/reviews/20261110-085833-s05-formats-audit/semantic-checks.json)保留PDB/AMCSD实际差异、m3d损失及所有旧失败。样本新增VS79，仍未满足S05.1–.3原标准。

## S05 本次继续执行结果（dsh复审通过）

稳定254 ID/87入口级待核保持。本轮仅VF0009 Open、VF0013 Save As、VF0014 Export及VF0253/0254格式族追加范围内证据，没有误把VF0007 New Structure当Open。48输入/23输出按[逐类矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/S05-MATRIX.md)链接独立预期/GUI/实物/重读与失败，研究覆盖不提高其它功能的领域或产品状态。


## S06领域规则索引

[S06_DOMAIN_RULES.json](S06_DOMAIN_RULES.json)逐一引用本唯一目录的稳定ID，列出206个领域相关条目的规则组、已证范围、明确未知及下一探针；其余条目仍在本目录，不因S06分组退出最终全部功能。相同ID可关联多个规则组，不新增或复用ID。未知字段不构成实现裁定；缺关键规则的模块在P1依赖安排前继续研究。仅VF72/73/74因本次真实运行推进入口状态，其余行为状态保持。

S06验收记录：本卡实际dsh独立复审pass，研究卡四项完成；已证/失败/未知的科学边界保持，完整Study和产品实现未完成。见[裁定](../../output/vesta-study/20261108-113622/reviews/20261110-135709-s06-domain-study/review-result.json)。本轮停止。

## S07 命令/外部合同承接

稳定254ID不新增，84条精确entry-observed / behavior-pending保持。functions.json为15个相关文件/科学/偏好条目追加s07Contract指针，并对VF65/66补真实TIDY子返回/失败；不改既有行为status或S06科学未知。官方16入口不是另造稳定功能ID或产品API，详见[CLI](CLI.md)和[S07矩阵](../../output/vesta-study/20261108-113622/reviews/20261110-150652-s07-contract-audit/S07-MATRIX.md)。

## S08 逐功能接手字段

唯一规格仍是 functions.json 的254条records。每条新增 `studyHandoff`：指向域规格/资料锚点、已有明确关联的稳定样本、实际输入计划定位、S06规则/未知/下一探针。61个功能有样本注册表的反向关联；其余193个不冒充已测试样本，其中7个为应用/OS入口无需领域文件，其余186个提供明确标为未验证的候选上下文，具体原实验输入仍由 evidenceRefs 内的计划/回执定位。候选不是新增稳定ID，也不是运行通过。所有原status、84条精确entry-observed / behavior-pending及5733个原证据引用保持；其它partial状态同样未全验。

新会话按 featureId 取record，先读entry/preconditions/entryConditions/pendingEntries，再读studyHandoff/specRefs与sourceIndexRefs，按registeredSampleIds在SAMPLES定位；若只有candidateContextSampleIds，先读取relatedEvidenceLocatorRefs，核是否足够，再按nextResearch补缺。所有科学未知见S06_DOMAIN_RULES、当前原生头见UNKNOWNS；无需依赖聊天猜输入。S08导航检查不是穷尽功能或全行为通过证明。

S08复审F1/F2澄清：VF-0011链接数据库原生unknown并给Stop/网络专门探针，VF-0019/0020改为历史分组/焦点/redo失效的未来探针。relatedEvidenceLocatorRefs仅是相关证据，不保证每项都是冻结输入计划；实际输入从所引原参数或注册样本定位，VF-0225显式以VS-0001/0002/0003为注册输入回退。没有新运行/样本或原行为状态变化。
