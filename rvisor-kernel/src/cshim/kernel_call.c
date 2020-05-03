
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


// 这里用宏来将系统调用指针封装成函数，对C宏编程，参考：https://blog.csdn.net/gkzscs/article/details/82934054
SYSCALL_C_FUNC(open, long, const char * filename, int flags, unsigned short mode)
    (filename, flags, mode)
SYSCALL_C_FUNC_END

SYSCALL_C_FUNC(getpid, long, void)
    ()
SYSCALL_C_FUNC_END

SYSCALL_C_FUNC(openat, long, unsigned long f, const char * filename, int flags, unsigned short mode)
    (f, filename, flags, mode)
SYSCALL_C_FUNC_END


