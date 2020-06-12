title: 结题报告
speaker: dnailz
plugins:
    - echarts

<slide class="bg-black-blue aligncenter" image="https://source.unsplash.com/C1HhAQrbykQ/ .dark">

# 结题报告 {.text-landing.text-shadow}

By x-chital {.text-intro}

[:fa-github: Github](https://github.com/OSH-2020/x-chital){.button.ghost}


<slide :class="size-80">

### 搭建 Rust + Linux 内核模块开发环境
---

* 借助 fjw 助教提供的 VLab KVM 虚拟机远程共同开发。 {.animated.fadeInUp}
* 使用 Github 项目 [linux-kernel-module-rust](https://github.com/fishinabarrel/linux-kernel-module-rust) 进行 LKM + Rust 开发   {.animated.fadeInUp.delay-400}
* 借助 RLS（Rust Language Server）搭建开发环境 {.animated.fadeInUp.delay-800}

<slide :class="size-80">

:::column {.vertical-align}
### 系统调用替换的实现
# 
#

使用 `kallsyms_lookup_name` 得到系统调用表后，编写一个简单的 C-Shim, 实现对系统调用的替换。

由于 Linux 的内存保护机制，需要暂时禁止掉内存的写保护。

----

```c
int replace_syscall(unsigned int syscall_num, long (*syscall_fn)(void)) { 
    .......

    cr0 = disable_wp(); // 关闭内存写保护
    syscall_table[syscall_num] = syscall_fn; // 替换相应的系统调用
    restore_wp(cr0); // 恢复内存写保护
    
    return 0;
}
```
:::

<slide :class="size-80">

:::column {.vertical-align}
### 构造虚拟设备节点实现内核
# 
#


使用 `kallsyms_lookup_name` 得到系统调用表后，编写一个简单的 C-Shim, 实现对系统调用的替换。

由于 Linux 的内存保护机制，需要暂时禁止掉内存的写保护。

----

```c
int replace_syscall(unsigned int syscall_num, long (*syscall_fn)(void)) { 
    .......

    cr0 = disable_wp(); // 关闭内存写保护
    syscall_table[syscall_num] = syscall_fn; // 替换相应的系统调用
    restore_wp(cr0); // 恢复内存写保护
    
    return 0;
}
```
:::


