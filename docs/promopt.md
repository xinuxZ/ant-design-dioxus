
# 开发组件的提示词
项目背景查看这个文件 task.md 了解，然后按照任务规划，分析下一步需要实现的组件。实现组件功能器件需要遵守的项目规范 project_rules.md 。
要求至少实现5个组件，任务执行顺序如下：
1、分析ant-design 对应组件功能点，然后再实现。
2、实现所有组件功能以及正确的样式引入，并增加对应的使用示例到 components 目录
3、所有组件都实现成功后执：行语法检查，判断项目是否可以正常运行
4、检查错误，如有错误则修复错误；尝试多次无法解决的错误开启深度思考模式，分析问题根源。如果仍然无法解决，则跳过该错误继续修复其他错误。


1、去掉对 crate::components::ComponentProps; 有关的逻辑，因为没有这个文件。
2、每个组件独立成一个目录，参考 button 组件封装格式
3、补充每个组件的样式，参照 ant-design 对应组件的样式
4、前面3个步骤完成后，对比 src/components 目录下的组件，确保所有组件在 examples 目录下都有对应的示例。
5、执行 cargo check 命令，确保项目可以正常运行。
6、如果执行 cargo check 命令报错，根据错误提示修复错误。
7、如果修复错误后仍然无法解决问题，开启深度思考模式，分析问题根源。
8、如果仍然无法解决问题，跳过该错误继续修复其他错误。
9、启动深度思考模式，重复步骤 5-8，直到所有错误都被修复。


## 第二步：主题和样式系统
- 完善主题定制功能
- 实现暗色主题支持
- 实现紧凑主题
- CSS 变量系统
- 动画和过渡效果

## 第三步： 为每个组件添加基础使用示例
card
carousel
collapse
descriptions
image
list
message
modal
notification
popconfirm
popover
result
skeleton
statistic
steps
tabs
timeline
transfer
upload


## 第四步： 对比已经实现的组件是否完整复刻了ant-design的组件功能

## 第五步： css in rust 样式方案
