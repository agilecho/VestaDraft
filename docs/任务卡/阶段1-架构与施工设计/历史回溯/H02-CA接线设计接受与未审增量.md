# P1-H02 · CA接线设计接受与未审增量（历史回溯卡）

> 当前任务组织（2026-11-18）：P0研究完成、P1设计已在rev66独审通过并于rev67登记收口。本页是历史补档；下文“当前剩余/partial/下一卡”仅属原补档时点，不再作为P1待施工。旧被审范围不追认，后续实现见[总任务入口](/Users/agilecho/code/Draft/docs/任务卡/README.md)。

**事后整理：2026-11-14。** 本卡在原执行之后补建，不代表当时已有本目录或该编号。状态：P1.3 C4设计级已接受；C5有条件复审；C6增量未审。

## 给你看的

工具接线的C4设计接受已经成立，后来的C5、C6有各自增量身份。运行验证继续另行授权；没有因尚未运行撤销C4，也没有把后续原卡复审扩成C6接受。

补档时点剩余：C6沿原卡增量待审入口由用户手动转dsh，不另开C4施工；[P1-05](../05-构建资源与外算合同.md)消费其设计与资格边界，[P1-06](../06-全功能实施计划与P1收口.md)收口实施安排。[M1–M8](../非阻塞续办.md)不作为01前置；真实运行留待另行授权卡。

## 给执行者看的

### 原执行时点、任务与授权依据

原P1在2026-11-10交源码级草稿；2026-11-14 R2形成C4，R3按用户带回dsh及授权回写接受；R3/C5有真实有条件独审，R4形成C6，至R9/rev35仍未审C6增量。

原P1设计授权与DEC-P1-03见[总体计划](../../../superpowers/plans/2026-11-08-vesta-full-rewrite.md)及[既定裁定](../决策确认单.md)；[接受回写及整改原话归档](../../../../output/vesta-study/20261108-113622/reviews/20261114-120337-p1-design-r3/user-instruction.json)；SHA256 `9b38382620c7ea32463210468bb4f5551cb0b7182ff3bb2a2c32862e05492e60`明确按设计级接受、P1.2/P1.4/整体partial及不进产品。

### 实际交付与原验收断言

P1.3交prepare锁定构建/调用合同、正式Study派生迁入、target/request/stack/backend/kind绑定、唯一Build launcher、worker/inbox/checks-asks/round0及失败恢复设计。C5细化浏览器制品绑定；C6补buildRoot/获准来源/receipt等设计，仍只是设计稿。

### 原验收、复审及被审版本

[用户带回真实R2报告](../../../../output/vesta-study/20261108-113622/reviews/20261114-120337-p1-design-r3/inbound/dsh-r2-review.md)；SHA256 `d59c076f9bc6cad69635d0ab31e72e133b94a19a4e3e36f3f389e91d07e54b14`在回写前保留P1_3_complete=false原字段；设计接受由dsh设计判断加用户明确授权、独立接受记录承载，不用作者转录冒充原结论。

[继承接受记录](../../../../output/vesta-study/20261108-113622/reviews/20261114-152228-p1-design-r7-errata/P1.3-design-acceptance.json)；SHA256 `fd952af6b540edc26d27fbe3f8d738fe6b07f8104031569041a0e2389e679c61`记C4已接受/运行未授权。[真实C5条件复审](../../../../output/vesta-study/20261108-113622/reviews/20261114-122114-p1-design-r3-dsh-review/review-result.json)；SHA256 `aca02c847f55d5f5661efa4983ae04e48359aa377f03e052806019a2616d0556`列N3/N6及R3-R1–R3残留，P1仍2/2；C6没有实际增量接受。

| 版本 | 身份及结论 |
|---|---|
| C4 | `51af2f4d5418bc6542f9f120d6bfa5fc59f558edd278cb52a537eae3ea4f8cda`，R2/after保留原稿，设计级接受 |
| C5 | `11c17fb68293a01ec69aa6a5be8179f030eea29f9ebca1ff055e93af1d7a1be3`，真实R3复审有条件通过，不能由C4覆盖增量 |
| C6 | `a688ef07e3d6a4115ecf456ba2a0ab2f27d9ad1c3a5da065f5159f7835df47c9`，R4起当前稿，增量未审；R5–R9明示unchanged-not-reviewed |

确切副本、接受源/用户指令与SHA的本次核验见证据目录reviewed-version-bindings.json。

### 失败、限制及未验证项

C6生产浏览器获准来源表为空、合同不是真实安装/launcher/worker行为。真实new_project/copy/prepare/build/worker/Check/launcher、故意失败及恢复都未授权/未验证。旧accepted_design.currentDeltaReview仍带R4标签是M3，表示pending而非已审；不修改旧接受原件。

### 补档时点归属与边界

C6沿原卡增量待审入口由用户手动转dsh，不另开C4施工；[P1-05](../05-构建资源与外算合同.md)消费其设计与资格边界，[P1-06](../06-全功能实施计划与P1收口.md)收口实施安排。[M1–M8](../非阻塞续办.md)不作为01前置；真实运行留待另行授权卡。

本卡只作历史归档，不是新的开工授权，不重做已接受工作。原技术文件仍是技术依据；当前施工须从[任务总览](../../README.md)及相应剩余卡进入。预算、累计费用、次数和停报状态不重置；完整金额账目未取得，不能报零成本。本次补档的原始核验与限制见[证据入口](../../../../output/vesta-history/20261114-173923-task-backfill/README.md)。
