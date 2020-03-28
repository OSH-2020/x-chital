
# 调研报告

[TOC]

## 小组成员

* 丁垣天 dnailzb@outlook.com
* 叶之帆 yezhifan@mail.ustc.edu.cn
* 何灏迪 hardyhe@mail.ustc.edu.cn
* 郑在一

## 项目简介

rVisor 为一个基于用户空间（User Space）的通用安全沙箱环境。在gVisor的基础上， 提供更高层次的共享机制，避免 go 语言 GC 的开销，为程序提供轻量高效的运行环境。

rVisor 主要面向浏览器、移动端、嵌入式系统等对沙箱体积要求较高的小型系统，支持 Linux ELF 和 WASI 两种可执行文件，提供一个安全高效的跨平台环境。rVisor功能大体完善后也可考虑在服务端运行。

rVisor 初步可以考虑通过对Linux系统调用进行劫持的方法实现（ptrace系统调用），进一步可以考虑移植到Linux内核，作为一个Linux内核模块。rVisor 对


## 项目背景

### docker
 
#### 背景 
 
##### 虚拟化技术

虚拟化（Virtualization）是一种资源管理技术，通过将计算机中的各种实体资源进行抽象后呈现。这样一来，实体结构间的障碍被打破，底层物理硬件资源可以得到充分的利用。通过这种方式，用户可以以更好的方式利用底层资源，此外也保证了更高的安全性、可用性、可扩展性。

虚拟化技术可以在不同层次上实现，如指令集层、硬件层、操作系统层、应用程序层等。

###### 指令层

指令集层次的虚拟化，即通过主机的ISA直接翻译模拟另一给定系统的ISA，其基本的模拟方式是“代码解释”，一个软件翻译层的程序将源指令逐条翻译为目标指令，一条源指令可能会对应上百条目标指令来实现相同的功能。虽然之后进行了许多优化，但其实现效率仍是各种层次的虚拟化技术中最低的。

###### 硬件层

硬件层次的虚拟化，指在硬件上直接运行Hypervisor，有利于虚拟机的操作系统直接访问硬件资源。选择使用这种解决方案的有VMware ESXi 和 Xen server。

![硬件虚拟化](https://img-blog.csdn.net/20180717140953793?watermark/2/text/aHR0cHM6Ly9ibG9nLmNzZG4ubmV0L2xpbnhpeWltZW5nMDA3/)

###### 操作系统层

操作系统层次的虚拟化，指在宿主机的操作系统上运行虚拟化部分，传统的虚拟化技术中，在虚拟层上仍需运行Guest OS。这种方案实现较简单，灵活性较强，但性能也相对硬件虚拟化较低。其典型例子为VMware workstation。

![操作系统虚拟化](https://img-blog.csdn.net/20180717140934421?watermark/2/text/aHR0cHM6Ly9ibG9nLmNzZG4ubmV0L2xpbnhpeWltZW5nMDA3/)

###### 应用程序层

应用程序层的虚拟化，是将一个应用程序虚拟化为一个虚拟机。最流行的方法是高级语言虚拟机，在这种情况下，虚拟化层作为一个应用程序处于操作系统之上，并且这一层抽象出一个虚拟机，其可以运行为特定的机器环境所编写和编译的程序。使用应用程序层次虚拟化的代表为JVM。

##### 容器技术

容器技术与传统的虚拟化技术不同，容器直接运行在操作系统内核之上的用户空间，并且在容器中不需要再包括Guest OS部分。这样的方式使得容器具有灵活、便捷的特点。在此基础上，容器也具备了虚拟化的基本特性，允许其中的应用跨平台的运行，而不用受制于Host OS的区别带来的麻烦。

#### Docker简介

Docker是一个基于go语言编写的开源的应用容器引擎。通过使用Docker开发者可以打包他们的应用以及依赖包到一个可移植的容器中,然后发布到任何流行的Linux机器或Windows机器上,也可以实现虚拟化,容器是完全使用沙箱机制,相互之间不会有任何接口。

作为容器技术，Docker相比传统的虚拟化技术具有以下优点：

- **更高效的利用系统资源**：由于容器不需要进行硬件虚拟以及运行完整操作系统等额外开销，Docker 对系统资源的利用率更高。无论是应用执行速度、内存损耗或者文件存储速度，都要比传统虚拟机技术更高效。因此，相比虚拟机技术，一个相同配置的主机，往往可以运行更多数量的应用。

- **更快速的启动时间**：传统的虚拟机技术启动应用服务往往需要数分钟，而Docker 容器应用，由于直接运行于宿主内核，无需启动完整的操作系统，因此可以做到秒级、甚至毫秒级的启动时间。大大的节约了开发、测试、部署的时间。

- **一致的运行环境**：开发过程中一个常见的问题是环境一致性问题。由于开发环境、测试环境、生产环境不一致，导致有些bug 并未在开发过程中被发现。而Docker 的镜像提供了除内核外完整的运行时环境，确保了应用运行环境一致性，从而不会再出现一段代码在不同机器上运行结果不同的问题。

- **持续交付和部署**：使用Docker 可以通过定制应用镜像来实现持续集成、持续交付、部署。开发人员可以通过Dockerfile 来进行镜像构建，并结合持续集成(Continuous Integration) 系统进行集成测试，而运维人员则可以直接在生产环境中快速部署该镜像，甚至结合持续部署(Continuous Delivery/Deployment) 系统进行自动部署。

- **更轻松的迁移**：Docker 使用的分层存储以及镜像的技术，使得应用重复部分的复用更为容易，也使得应用的维护更新更加简单，基于基础镜像进一步扩展镜像也变得非常简单。此外，Docker 团队同各个开源项目团队一起维护了一大批高质量的官方镜像，既可以直接在生产环境使用，又可以作为基础进一步定制，大大的降低了应用服务的镜像制作成本。使用Dockerfile 使镜像构建透明化，不仅仅开发团队可以理解应用运行环境，也方便运维团队理解应用运行所需条件，帮助更好的生产环境中部署该镜像。

#### Docker中的基本概念

##### 镜像 (Image)

镜像相当于一个特殊的文件系统，在Docker中，镜像为容器运行时提供需要的程序、库、资源、配置等文件，并会为之准备环境变量、匿名卷等配置参数。镜像不包含动态数据，在构建之后就不会被改变。
镜像，从认识上简单的来说，就是面向对象中的类，相当于一个模板。从本质上来说，镜像相当于一个文件系统。Docker 镜像是一个特殊的文件系统，除了提供容器运行时所需的程序、库、资源、配置等文件外，还包含了一些为运行时准备的一些配置参数（如匿名卷、环境变量、用户等）。镜像不包含任何动态数据，其内容在构建之后也不会被改变。

##### 容器 (Container)

容器是通过镜像创建出的实体，其本质是进程。但其运行于属于自己的命名空间，可以拥有自己的文件系统、网络配置、进程空间，甚至自己的用户ID空间。这样的特质使容器创造出一个隔离的环境，在容器内运行进程相比直接运行在Host OS上更加安全。

##### 仓库 (Repository)

仓库用于保存用户构建的镜像。镜像构建完成后，可以很容易的在当前宿主机上运行，但是，如果需要在其它服务器上使用这个镜像，我们就需要一个集中的存储、分发镜像的服务，Docker Registry 就是这样的服务。

#### Docker中的基本操作 

##### 启动与退出

- **启动Docker** `systemctl start docker`

- **停止Docker** `systemctl stop docker`

- **重启Docker** `systemctl restart docker`

##### 镜像相关操作

- **列出所有镜像**`docker images`

- **搜索镜像** `docker search [IMAGE]`

- **拉取镜像** `docker pull [OPTIONS] NAME [:TAG]`

- **推送镜像** `docker push NAME [:TAG]`

- **创建镜像** `docker commit [OPTIONS] CONTAINER [REPOSITORY[:TAG]]`

##### 容器相关操作

- **启动容器** `docker run IMAGE_NAME [COMMAND] [ARG…]`

- **列出容器** `docker ps`

- **查看容器** `docker inspect name | id`

- **重启停止的容器** `docker start [-i] 容器名`

### Kubernetes

#### Kubernetes简介

Kubernetes为支持自动化部署、大规模可伸缩、应用容器化管理。其具有以下一些特性：

- **自动包装** 根据资源需求和其他约束自动放置容器，同时不牺牲可用性。混合关键和尽力而为的工作负载，以提高利用率并节省更多资源。

- **自愈** 在节点死亡时重新启动失败的容器，替换和重新安排容器，杀死不响应用户定义的运行状况检查的容器，并且在它们准备好服务之前不会将它们通告给客户端。

- **水平缩放** 使用简单的命令，UI或基于CPU使用情况自动扩展和缩小应用程序。

- **服务发现和负载平衡** 无需修改应用程序即可使用不熟悉的服务发现机制。Kubernetes为容器提供了自己的IP地址和一个DNS名称，并且可以在它们之间进行负载平衡。

- **自动部署和回滚** Kubernetes逐步推出对您的应用程序或其配置的更改，同时监控应用程序运行状况以确保它不会同时终止您的所有实例。如果出现问题，Kubernetes支持回滚更改。

- **秘密和配置管理** 部署和更新机密和应用程序配置，无需重建映像，也不会在堆栈配置中暴露机密。

- **存储编排** 自动安装选择的存储系统，包括本地存储，公共云提供商（如GCP或AWS），网络存储系统（如NFS，iSCSI，Gluster，Ceph，Cinder或Flocker）等。

- **批量执行** 除服务外，Kubernetes还可以管理批处理和CI工作负载，如果需要，可以替换失败的容器。

#### Kubernetes核心概念

![Kubernetes架构](https://img-blog.csdnimg.cn/20200317095558914.png)

##### Pod

Pod是Kubernetes 的一个最小调度以及资源单元，包含一组容器和卷(Volume)。同一个Pod里的容器共享同一个网络命名空间，可以使用localhost互相通信。在Pod中可以定义容器所需要运行的方式。如运行容器的Command，运行容器的环境变量等。Pod与Pod之间相互隔离。

##### Service

Service是定义一系列Pod以及访问这些Pod的策略的一层抽象。Service通过Label找到Pod组。利用Service这一概念可以完成对Pod的抽象，用户可以不用关注于每一个Pod的具体情况，当Pod发生终止/重新连接时，用户也无需手动更新Pod的地址。借助Service这一概念，Kubernetes得以负载均衡的功能。

##### NameSpace

Namespace被用于进行一个集群中的逻辑隔离，包括鉴权、资源管理等功能。Kubernetes中的每一个资源都属于某一个Namespace。在一个Namespace中的资源要求命名的唯一性，而不同Namespace中的资源则可以重名。

##### Master节点

Kubernetes采用Master-Nodes架构，在其中创建的应用通常都在Nodes上运行，Master节点上主要负责进行资源的调度。其中包括：

- **etcd** 主要用于存放集群的状态。一般把所有的集群信息都存放到etcd当中，etcd不属于Kubernetes的某一个部分，而是单独集群部署的。

- **API Server** 提供操作资源的唯一入口。认证、授权、访问控制、注册或者发现等操作都是通过API Server来完成的。

- **Controller Manager** 负责维护集群的状态，例如Pod的故障检测、Pod的自动扩展、滚动更新等。

- **Scheduler** 负责整个集群的资源调度，按照默认或者指定的调度策略将Pod调度到符合要求的Node节点上运行。

#### Kubernetes的优缺点

使用Kubernetes提供的现有分布式系统架构，相关开发工作可以得到大幅简化。尤其是其中的服务调度和负载均衡的自动化处理，使开发人员无需考虑与应用本身无关的系统环境问题。其具有较好的移植性，利用之可以将现有的物理机环境无缝移植到公有云系统中。此外，Kubernetes具有较成熟稳定的管理系统以及健康机制。

相对于Docker Swarm，Kubernetes由于功能较多，配置较复杂，启动速度较慢。

### gVisor

#### 背景

##### 传统容器面临的安全挑战

##### 安全容器解决方案

#### gvisor简介

强调gVisor是一个小型Linux内核

### ptrace 系统调用

## 立项依据

gVisor 是一个尝试用 Go 语言编写的操作系统内核，希望劫持应用沙箱中的全部系统调用来保证沙箱内部的安全性，因而成为当今知名的安全容器实现。

但是，gVisor 具有一下两个问题，使之难以真正用于生产环境：

* gVisor 目前主要通过 Linux ptrace 系统调用实现，在系统调用监控的过程中，必然会出现一次上下文转换（Context Switch），这 gVisor 虽然保证了系统的安全，但是也造成了性能问题，gVisor 的性能与实际生产环境的要求还有较大差距。
* gVisor 作为一个操作系统内核，采用带有 GC 的 Go 语言编写。虽然 Go 语言的安全性确实保障了容器的安全，但现在绝大多数可用的操作系统内核都采用无 GC 的语言编写，用 Go 编写内核会有比较大的性能开销。

rVisor 为改进以上两点问题，提出一下的解决方案：

* rVisor 将会考虑使用 rust 语言编写，既考虑到安全容器对于安全的要求，又兼顾操作系统内核应有的效率。
* rVisor 可以在完成 ptrace 实现的基础上，通过修改 Linux 内核或者编写 Linux 内核模块的方式尝试将代码移植内核，避免一次上下文切换带来的开销。


## 重要性与前瞻性分析

### 安全容器

原本基于 KVM+QEMU 的传统虚拟化方法可以提供很好的隔离型，但是虚拟机整体而言不够轻量。而以 Docker 为代表的传统容器技术，实现了轻量、无痛的容器隔离，但随着容器技术的不断发展，传统容器隔离性不足的缺点逐渐暴露出来。

Docker 的架构大体如下，由 Linux Kernel、Namespace/Cgroups/Aufs、Seccomp-bpf、Libs、Language VM、User Code、Container(Docker) engine 这几个部件组成。

![](https://pic4.zhimg.com/v2-cb6a536fa0a0c92d53e7a961d5387b5b_b.jpg)

从攻击者的角度，这个架构比较复杂，因而暴露出的漏洞也比较多，攻击者既可以利用 Linux 内核进行攻击，又可以通过利用 Docker 容器自身的漏洞进行攻击，很容易实现容器逃逸。

为了提高容器的安全性，增强容器之间的隔离，安全容器的技术开始逐步出现：

* Kata 基于传统的容器技术，将传统容器采用比较轻量的方法实现，利用它自身优化和性能设计，也拥有与容器相媲美的敏捷性。
* gVisor 采用沙箱技术，它主要实现了大部分的system call。它运行在应用程序和内核之间，为它们提供隔离。

rVisor 基于 gVisor，力求实现更安全高效的安全容器。

### 轻量型的安全容器

对于 Kata 和 gVisor 这两种实现方法，我们对比如下：

![](https://pic1.zhimg.com/v2-03cbf5ecffb04a3654d56cd9c5b478d8_b.jpg)

最理想的实现方式，是利用 gVisor 这种进程级虚拟化，可以实现特别轻量的容器，但 gVisor 基于 ptrace 拦截系统调用，Sentry（哨兵进程）会不断与内核通信，这之间频繁的上下文切换是一个不小的开销。目前 gVisor 的实现仍然不再可接受的范围内。Kata 目前是一个比较可用的解决方案。

rVisor 计划通过将 gVisor 重写，然后移入 Linux 内核，以减少这一次上下文切换，实现更优秀的性能。

不过考虑到小组水平有限，可能暂时难以真正做到移入 Linux 内核，不过可以做一些理论工作，给出大体的实现方法。

### Rust 语言

当今容器技术的发展跟 Go 语言密不可分，作为一个容器的 gVisor 为什么要把它拿来用 rust 重写呢？

虽然 Go 和 Rust，都是当今流行的系统级编程语言，都具有安全高效的特征，但 Go 和 Rust 本身定位并不相同。

Go 并不适合操作系统领域的开发。Go 本身带有 GC 机制，虽然其语法比较像 C 语言，其目标是替代 Java、Python 的位置，做一个高性能且能够快速开发的语言。用 Go 开发操作系统[会带来一定的开销](https://pdos.csail.mit.edu/papers/biscuit.pdf)。而 Rust 没有 GC，效率上与 C/C++ 相进，适宜于操作系统、嵌入式的开发。

在容器领域，像 LXC 这种容器底层的部件往往都由 C 编写，Go 语言往往用于上层的调度工作。

Rust 在容器领域中也有一些初步的运用，比如经常与 gVisor 并列的 Kata 的核心部件：[Kata-Agent](https://github.com/kata-containers/kata-containers)，就从 Go 迁移到了 Rust，可见将容器领域的核心部件用 rust 重写有一定的价值。

另一方面，Google 想要逐渐将 gVisor 移植到 KVM 平台，将 gVisor 移植到 KVM 可以消除系统调用劫持过程中的开销。不过，gVisor 一旦作为一个独立的操作系统，就要对各个方面提供完整的实现，也要面对 Go 语言不适宜操作系统开发的问题。

rVisor 计划基于 rust 实现一个高性能的安全沙箱容器，在 gVisor 的基础上尽可能地提高性能。

## 相关工作

### gVisor

### WASI

### PRoot

https://github.com/proot-me/proot

## 参考文献

[Docker——入门实战](https://blog.csdn.net/bskfnvjtlyzmv867/article/details/81044217)

[Docker安装以及原理详解](https://blog.csdn.net/linxiyimeng007/article/details/81080223)

[虚拟化的层次与机制](https://blog.csdn.net/mayp1/article/details/51296682)

[Kubernetes综述](https://blog.csdn.net/qq_24095055/article/details/97624900)

[Kubernetes认识](https://blog.csdn.net/inthat/article/details/83055531)

[Kubernetes核心组件](https://blog.csdn.net/weixin_42438967/article/details/104580478)

[Kubernetes核心组件篇 (一) : Kubernetes核心组件组成](https://blog.csdn.net/BearStarX/article/details/104915170)

[Kubernetes](https://blog.csdn.net/liuj2511981/article/details/80442394)

[传统容器已死，安全容器将成为云原生标配](https://zhuanlan.zhihu.com/p/99182075)
