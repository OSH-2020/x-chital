
# 调研报告

[TOC]

## 小组成员

* 丁垣天 dnailzb@outlook.com
* 叶之帆 yezhifan@mail.ustc.edu.cn
* 何灏迪
* 郑在一

## 项目简介

rVisor 为一个基于用户空间（User Space）的通用安全沙箱环境。在gVisor的基础上， 提供更高层次的共享机制，避免 go 语言 GC 的开销，为程序提供轻量高效的运行环境。

rVisor 主要面向浏览器、移动端、嵌入式系统等对沙箱体积要求较高的小型系统，支持 Linux ELF 和 WASI 两种可执行文件，提供一个安全高效的跨平台环境。rVisor功能大体完善后也可考虑在服务端运行。

rVisor 初步可以考虑通过对Linux系统调用进行劫持的方法实现（ptrace系统调用），进一步可以考虑移植到Linux内核，作为一个Linux内核模块。rVisor 对


## 项目背景

### docker

### Kubernetes

### gVisor

[传统容器已死，安全容器将成为云原生标配](https://zhuanlan.zhihu.com/p/99182075)

强调gVisor是一个小型Linux内核

### ptrace 系统调用


## 重要性与前瞻性分析

### 面向小型系统的通用安全沙箱环境

#### 移动端和嵌入式系统

#### 浏览器

### 服务端使用的可能性

rVisor 在移入Linux内核后，也可以考虑在服务端使用。

鉴于 Google 正在考虑将 gVisor 移入 KVM，但 gVisor 必定会受到 go 语言GC机制的局限，其性能无法得到充分的发挥。rVisor 使用在操作系统领域常用的开发语言 rust，作为这样的一个独立运行的操作系统内核更为合适。

不过考虑到本小组能力有限，rVisor在服务端的可能性可能将会无法验证。

## 相关工作

### gVisor

### WASI

### PRoot

https://github.com/proot-me/proot

## 参考文献

