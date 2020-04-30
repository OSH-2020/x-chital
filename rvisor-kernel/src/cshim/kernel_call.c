
#include "hook_syscall.h"

#include <linux/module.h>
#include <linux/init.h>
#include <linux/types.h>
#include <linux/syscalls.h>
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

#define SYSCALL_C_FUNC(name, ret, args...) \
    typedef ret (*name##_syscall_t)( args ); \
    ret orig_##name( args ) { \
        name##_syscall_t orig_##name##_ptr = saved_syscall_table[__NR_##name]; \
        return orig_##name##_ptr

#define SYSCALL_C_FUNC_END ;}

SYSCALL_C_FUNC(open, long, const char * filename, int flags, unsigned short mode)
    (filename, flags, mode)
SYSCALL_C_FUNC_END

SYSCALL_C_FUNC(getpid, long, void)
    ()
SYSCALL_C_FUNC_END

