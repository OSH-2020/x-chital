
#include "hook_syscall.h"

#include <linux/module.h>
#include <linux/init.h>
#include <linux/types.h>
// #include <linux/syscalls.h>
#include <linux/delay.h>
#include <linux/sched.h>
#include <linux/version.h>
#include <linux/kallsyms.h>
#include <linux/semaphore.h>
#include <asm/cacheflush.h>
#include <linux/bitops.h>
#include <linux/sizes.h>
#include <linux/byteorder/generic.h>
#include <linux/preempt.h>

// 系统调用名，返回值，系统调用参数表
#define SYSCALL_C_FUNC(name, ret, args...) \
    typedef ret (*name##_syscall_t)( args ); \
    ret orig_##name( args ) { \
        name##_syscall_t orig_##name##_ptr = saved_syscall_table[__NR_##name]; \
        return orig_##name##_ptr

#define SYSCALL_C_FUNC_END ;}


#define SYSCALL_EXPORT0(name) \
    SYSCALL_C_FUNC(name, long, void) \
        () \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT1(name, T1, V1) \
    SYSCALL_C_FUNC(name, long, T1 V1) \
        (V1) \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT2(name, T1, V1, T2, V2) \
    SYSCALL_C_FUNC(name, long, T1 V1, T2 V2) \
        (V1, V2) \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT3(name, T1, V1, T2, V2, T3, V3) \
    SYSCALL_C_FUNC(name, long, T1 V1, T2 V2, T3 V3) \
        (V1, V2, V3) \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT4(name, T1, V1, T2, V2, T3, V3, T4, V4) \
    SYSCALL_C_FUNC(name, long, T1 V1, T2 V2, T3 V3, T4 V4) \
        (V1, V2, V3, V4) \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT5(name, T1, V1, T2, V2, T3, V3, T4, V4, T5, V5) \
    SYSCALL_C_FUNC(name, long, T1 V1, T2 V2, T3 V3, T4 V4, T5 V5) \
        (V1, V2, V3, V4, V5) \
    SYSCALL_C_FUNC_END

// 这里用宏来将系统调用指针封装成函数，对C宏编程，参考：https://blog.csdn.net/gkzscs/article/details/82934054
SYSCALL_EXPORT3(open, const char *, filename, int,flags, unsigned short,mode)

SYSCALL_EXPORT0(getpid)


SYSCALL_EXPORT4(openat, unsigned long, f, const char *, filename, int, flags, unsigned short, mode)

SYSCALL_EXPORT5(clone, unsigned long, clone_flags, unsigned long, newsp,
		 int __user *, parent_tidptr,
		 int __user *, child_tidptr,
		 unsigned long, tls)

SYSCALL_EXPORT0(fork)

SYSCALL_EXPORT0(vfork)


SYSCALL_EXPORT3(execve,
		const char __user *, filename,
		const char __user *const __user *, argv,
		const char __user *const __user *, envp)

SYSCALL_EXPORT5(execveat,
		int, fd, const char __user *, filename,
		const char __user *const __user *, argv,
		const char __user *const __user *, envp,
		int, flags)


SYSCALL_EXPORT1(chdir, const char __user *, filename)
SYSCALL_EXPORT2(getcwd, char __user *, buf, unsigned long, size)

SYSCALL_EXPORT4(mknodat, int, dfd, const char __user *, filename, umode_t, mode, unsigned int, dev)
SYSCALL_EXPORT3(mknod, const char __user *, filename, umode_t, mode, unsigned, dev)
SYSCALL_EXPORT3(mkdirat, int, dfd, const char __user *, pathname, umode_t, mode)
SYSCALL_EXPORT2(mkdir, const char __user *, pathname, umode_t, mode)
SYSCALL_EXPORT1(rmdir, const char __user *, pathname)
SYSCALL_EXPORT2(stat, const char __user *, filename, struct __old_kernel_stat __user *, statbuf)
SYSCALL_EXPORT2(lstat, const char __user *, filename, struct __old_kernel_stat __user *, statbuf)