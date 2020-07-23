
### 搭建 Rust + Linux 内核模块开发环境
---

* 借助 fjw 助教提供的 VLab KVM 虚拟机远程共同开发。 {.animated.fadeInUp}
* 使用 Github 项目 [linux-kernel-module-rust](https://github.com/fishinabarrel/linux-kernel-module-rust) 进行 LKM + Rust 开发   {.animated.fadeInUp.delay-400}
* 借助 RLS（Rust Language Server）搭建开发环境 {.animated.fadeInUp.delay-800}

<slide :class="size-80">

:::column {.vertical-align}
### 系统调用替换的实现
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
### 实现内核与用户的交互
# 
#
#
#

实现内核与用户的交互的其他方式：系统调用方式（`seccomp`）、文件系统方式（`cgroup`）

在系统中添加一个虚拟设备节点类型 `rvisor`，使用 `mknod` 创建节点后，可以通过 `ioctl` 系统调用进行交互。

命令列表如下

> * `create` 新建一个容器环境
> * `addproc` 增加一个进程
> * `remove` 删除一个进程

----

```rust
    /// 对用户空间的iotcl调用做出反应
    /// * create 命令新建一个容器环境
    /// * addproc 增加一个进程
    /// * remove 删除一个进程
    fn ioctl(&self, cmd:u32, arg: u64) -> KernelResult<i64> {
        info!("ioctl cmd={} arg={}", cmd, arg);
        let mut container = Container::get_container();
        let cmd = IoctlCmd::try_from(cmd)?;
        match cmd {
            IoctlCmd::Create => {
                let path_str = string::read_from_user(arg, kernel::PATH_MAX)?;
                container.init(path_str)?;
                Ok(0)
            }
            IoctlCmd::AddProc => {
                container.add_task(arg as i32)?;
                Ok(0)
            }
            IoctlCmd::Remove => {
                container.remove_task(arg as i32)?;
                Ok(0)
            }
        }
    }
```
:::

<slide :class="size-80">

### 在 Rust 中实现内核同步
---

* 原子操作：rust 的引用计数智能指针 `Arc` 提供的相关操作是原子的，故可以直接使用。
* 自旋锁、自旋读写锁： rust no_std 本身没有提供，但是可以通过外部库来实现（这个库本身使用不到任何内核模块相关的东西，可以放心使用）。
* Rcu 锁：这个使用内核的 Rcu 锁，并封装成一个 Rust 的 struct 为 `RcuLock`。

<slide :class="size-80">

### 模拟 Linux 虚拟文件系统
# 
#
#
#
# 
#
#
#

:::column {.vertical-align}

```c

struct inode {         
        unsigned long           i_ino;
        atomic_t                i_count;
        umode_t                 i_mode;
        uid_t                   i_uid;
        gid_t                   i_gid;
        kdev_t                  i_rdev;
        loff_t                  i_size;
        spinlock_t              i_lock;
        struct file_operations  *i_fop;
        ......
};

struct dentry {
        atomic_t                 d_count;      
        spinlock_t               d_lock;       
        struct inode             *d_inode;     
        struct list_head         d_child;      
        struct dentry_operations *d_op;        
        struct rcu_head          d_rcu;        
        .....
};
```

----

```rust
#[derive(Debug, Clone)]
pub struct INode {
    pub ino : u64,
    pub mode : u64,
    pub uid : u64,
    pub gid : u64,
}

#[derive(Debug, Clone)]
pub struct DEntry {
    pub name : String,
    pub inode : INode,
    pub parent : Weak<RcuLock<DEntry>>,
    pub child : LinkedList<Arc<RcuLock<DEntry>>>,
    pub fops : Option<Rc<dyn FileOperations>>,
}
```
:::


<slide :class="size-80">

### 文件系统的读写

未完成，计划本周完成。

<slide :class="size-80">

### 进程管理系统

未完成

<slide :class="size-80">

### proc 文件系统

基于之前编写的虚拟文件系统，实现一个 proc 文件系统。增加对 `top` 指令的支持。可以查看当前真是的内核信息。
