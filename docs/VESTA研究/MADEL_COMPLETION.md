# MADEL外部程序完成条件

锁定原包在当前macOS arm64主机上的MADEL启动问题已定位；它不是一个零能量或数值正确性结果，完整Study仍未完成。

## 原包程序与实际加载失败

原包`Contents/Resources/madel`为46176字节、x86_64 Mach-O、Developer ID签名/hardened runtime；SHA-256为`7ea860c45d9176399221dd2b2448f8e4fd2b4505228aed23627bf5114e9ab83b`。它已经包含在上轮原包257文件不变检查内，没有新装/替换程序。[身份](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-identity.json)、[签名只读结果](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-codesign.json)、[真实native概览](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-overview.stdout)。

[原始依赖表与本机路径检查](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-dependencies.json)确认它链接到：

| 绝对依赖 | 本机该路径 | 已观察范围 |
|---|---|---|
| /usr/local/gfortran/lib/libgfortran.5.dylib | 不存在 | 本轮dyld实际首个加载错误 |
| /usr/local/gfortran/lib/libgcc_s.1.dylib | 不存在 | 依赖/文件系统检查；未宣称dyld继续到此处 |
| /usr/local/gfortran/lib/libquadmath.0.dylib | 不存在 | 同上 |

把既有GUI生成的`temporary-C1-restored.pme`逐字复制到本轮工作目录，以原程序执行`madel 输入.pme 输出.madel`。未改原PME或原件，未安装GNU Fortran、建立全局链接、设置DYLD变量或改变安全设置。[输入派生](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/pme-input-derivation.json)、[原件受控执行](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/native-madel-signal.stdout)、[实际子进程结果](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-original-observation.json)。

原子进程返回-6/SIGABRT，0.13秒左右停止，stdout为空，没有.madel文件。[完整stderr](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-child.stderr)明确报`Library not loaded: /usr/local/gfortran/lib/libgfortran.5.dylib`和各搜索路径不存在。这是进入MADEL main/读取PME/科研计算之前的加载阻塞。直接CA执行记录归一化为1，[原回执](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/native-madel-direct.stdout)保留；监督器0只表示观察完成，实际子程序-6不隐藏。

## GUI调用与无结果分支

原主程序`VestaFrame::OnMADEL`入口0x10009b114：[真实dossier](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-handler-study.stdout)。它导出PME，构造带madel、.pme和.madel参数的命令；0x10009b6b8调用wxExecute，flags传1。返回x0随后未用于判断错误；接着尝试fopen结果文件，0x10009b744在结果为空时直接跳0x10009b7e0清理，不读取/输出计算结果。不是由“界面恢复可点击”证明外部计算成功。

本轮原件独立执行的加载失败，加上GUI调用/缺结果静默分支，解释当前环境中该功能的具体完成阻塞。历史GUI会话只捕获PME生成和界面恢复，当时子程序stderr没有保存在可用记录中；不伪称本轮dyld日志就是历史GUI的直接截获，也未重新点击旧GUI流程。

原生问题`unk_239d524d3a4b9d25`更新为revision2/resolved，[原生校验](../../output/vesta-study/20261108-113622/reviews/20261108-175342-s07-madel/madel-unknown-verify.stdout)valid=true。resolved含义为当前阻塞诊断已回答，未修复依赖或原件行为。

## 尚未验收的领域行为

既有PME的单C位点电荷字段为0，但程序未进入计算；本轮没有势、能量或科学正确性通过结论。MADEL具体算法、带电/电中性样例、收敛与边界、架构和依赖打包、完整外部执行错误契约仍待后续Study。没有借诊断启动产品实现或系统环境施工。

目前136条evidence、9问题（5 resolved/1 investigating/3 open）；254功能、37样本、10勾选/19未勾选保持。
