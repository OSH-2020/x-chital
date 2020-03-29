## WA的虚拟机

### 其他语言->WA的运行机制

![wasm1](C:%5CUsers%5Cdonpa%5CDesktop%5CFeverBasketballOpenSource%5Cwasm1.png)

从看到的资料来说, 我对webassembly的了解主要是"它统一了JS, WEB的生态", "它可以(用LLVM等)把C等语言编译成统一的机器码". 总之这是一种工具, 但是现在对它的组件支持不完善(比如调试不是很方便), 所以我觉得选题可以从这些方面入手.

### 实现一个WebAssembly虚拟机

现阶段，WebAssembly 主要还是以Web应用为主，执行的容器大多基于主流的浏览器，并且通过javascript与外部通信，但是它的基于自定义内存和沙盒的特性，也使得WebAssembly 可以很好的适用于一些轻量级的场景，如作为执行区块链智能合约的虚拟机。

WebAssembly 是基于栈式的虚拟机，指令的执行都是在栈内完成的：

[![wasm4](C:%5CUsers%5Cdonpa%5CDesktop%5CFeverBasketballOpenSource%5Cwasm4.png)](https://github.com/ontio/ontology-wasm/blob/master/doc/images/wasm4.png)

webAssembly 指令集参考：[webAssembly bianry code](https://github.com/ontio/ontology-wasm/blob/master/doc/wasm_binarycode.md )

WebAssembly 只支持4种基本类型：

- int32
- int64
- float32
- float64

所以函数的参数和返回值也只能是这四种类型，并且每个函数只能有一个返回值。

如果想要使用复杂的类型，比如 string，就需要额外对内存进行操作。
[这里是关于WASM更完整的介绍](https://github.com/ontio/ontology-wasm/blob/master/doc/wasmvm_introduction.md)

## 分布式计算和AI

查找资料后, 公网的延迟确实不足以支撑分布式的参数更新迭代. 同时关于Intel和GPU的AI框架已经比较成熟了. 之前有学长是基于intel的SIMD指令集做优化, ARM架构上可以考虑用NEON指令集, 再自己写下MAR-REDUCE. 先用C或者其他语言写一个框架, 然后特定的目标看看底层的优化.

[NEON指令集的介绍](https://github.com/xylcbd/blogs_code/tree/master/arm_neon/2016_04_16_10_56_arm_neon_introduction/msvc)

[NEON实现矩阵运算的简单示例](https://blog.csdn.net/fuwenyan/article/details/78794526)

### 关于具体的框架

我们可以先从map-reduce做起. 下面是一个更好的模型(Parameter Server)的介绍, 如果难度合适我觉得可以移植到树莓派集群上.[Parameter Server](https://zhuanlan.zhihu.com/p/82116922)

## 选题的一些想法

1. 用rust写一个wa的轻量的虚拟机, 方便一些特定的功能(譬如动态逆向分析)

   优点:

   + 目标比较直接, 可借鉴的多
   + 语言相对好学

   缺点:

   + 代码量大(有人用python写这种virtual machine, 大概是一个人肝二十天的工作量)

2. 在树莓派上用NEON写一个小的, 分布式的矩阵运算框架; 矩阵分割以及一些特殊情况的处理.

   + 这个很大程度上取决于什么时候返校(不过手机也是ARM的话, 可以现在手机上调试)
   + 需要一定汇编能力