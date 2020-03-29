# RVisor

## 大体思路

[google gVisor](https://github.com/google/gvisor) **gVisor** is a user-space kernel, written in Go, that implements a substantial portion of the Linux system surface. It includes an [Open Container Initiative (OCI)](https://www.opencontainers.org/) runtime called `runsc` that provides an isolation boundary between the application and the host kernel. The `runsc` runtime integrates with Docker and Kubernetes, making it simple to run sandboxed containers.



> 轻量级的虚拟化本身就是一个解决虚拟化痛点的一个很大的方向。不但是AWS，谷歌给出了一个完全不同的方向。Firecracker的思路是既然虚拟机重量级，那么我就让虚拟机轻量，但是还是用虚拟机的思想来做到更好的隔离性，但是谷歌的gVisor不这么认为。gVisor认为既然隔离性不够，那我就从隔离性本身下手，让隔离性更好。所以谷歌也是在容器的后端下功夫，但是下功夫的方式是将所有的系统调用截断，在gVisor中用用户程序来实现系统调用的API，从而将内核的逻辑耦合变成了一个一个的独立的进程。相当于在用户空间实现了内核的功能，给Docker使用的时候，Docker看到的也是一个内核，但是实际上是一个gVisor自己实现的模拟内核。区别于真的把内核跑在用户空间的UML项目，gVisor极其轻量，隔离性却也达到了操作系统能带来的隔离程度。而且其实现强度极其让人发指，就连Linux的网络协议栈都在用户空间实现一遍。几乎实现了所有的Linux系统调用。
>
> 作者：刘叔
> 链接：https://zhuanlan.zhihu.com/p/55603422
> 来源：知乎



> “Hyper 非常高兴看到 gVisor 这样全新的提高容器隔离性的方法。行业需要一个强大的安全容器技术生态系统，我们期待通过与 gVisor 的合作让安全容器成为主流。“
>
>  -- Xu Wang，Kata 技术指导委员会成员，Hyper.sh CTO

解释：gVisor是新一代安全容器框架，其优势在于gVisor更为轻量（将所有的系统调用截断，在gVisor中用用户程序来实现系统调用的API，从而将内核的逻辑耦合变成了一个一个的独立的进程。）本身与WASI比较像（感觉）。

我目前的思路就是用rust重写gVisor，我们不妨称之为rVisor。

## 立项依据

* [传统容器已死，安全容器将成为云原生标配](https://zhuanlan.zhihu.com/p/99182075) ：gVisor的优势

* [为什么主流容器引擎都是用Go而不是C来写的？](https://www.zhihu.com/question/366520262/answer/975642782)：rust在容器基础设施中，或是go的一种更好的选择。**（此处作者没有给理由）**（所以我现在比较推崇RUST，虽然RUST中了语言设计的毒，狂加语法糖和函数式编程的玩意，另外就是众所周知的很多功能还不太稳定的问题（需要开发产品的时候用nightly版本可不是说着玩的），但是如果忍住那些令人心烦的玩意，RUST是取代GO的不二选择。）

* 相比WASI：显然，gVisor比WASI更快、更小巧，更易于移植。

* 可以考虑同时支持Linux ELF和WASI，实现安全高效的跨平台（LinuxELF在windows上以WSL形式运行）

* 可以更进一步考虑将rust代码转为wasm移入浏览器
* 也可以考虑载入Linux内核模块（体现我Rust可以入内核的优势

## 问题

* 用rust来写一个容器型设施是否有必要？（毕竟正常情况下都用go）
* 实现难度etc