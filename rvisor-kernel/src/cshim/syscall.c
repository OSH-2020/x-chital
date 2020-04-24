
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


// replaced syscall (open)
long (*orig_sys_open)(const char *filename, int flags, umode_t mode);
extern long rvisor_open( const char *filename, int flags, umode_t mode);

// disable the protection of syscall
inline unsigned long disable_wp ( void ) {
    unsigned long cr0; 
    preempt_disable();
    barrier();

    cr0 = read_cr0();
    write_cr0(cr0 & ~X86_CR0_WP);
    return cr0;
}
// enable the protection of syscall
inline void restore_wp ( unsigned long cr0 ) {
    write_cr0(cr0);
    barrier();
    // preempt_enable_no_resched(); // 释放内核控制
}


void **syscall_table;

// 0xffffffffb5a00240 is your syscall address
// you should run `sudo cat /boot/System.map-$(uname -r) | grep sys_call_table`
int replace_init() {
    syscall_table = (void **) (unsigned long *) 0xffffffffb5a00240;
    return 0;
}

int replace_syscall() { 
    if (!syscall_table) {
        printk(KERN_DEBUG "replace_syscall: Cannot find the system call table address.\n");
        return -1;
    }

    printk(KERN_DEBUG "Found the sys_call_table at %16lx.\n", (unsigned long) syscall_table);

    unsigned long cr0;
    cr0 = disable_wp();
    printk(KERN_DEBUG "Houston! We have full write access to all pages. Proceeding...\n");
    orig_sys_open = syscall_table[__NR_open];
    printk(KERN_DEBUG "get orig open\n");
    syscall_table[__NR_open] = rvisor_open;
    restore_wp(cr0);
    return 0;
}

int replace_clear() {
    if (!syscall_table) {
        printk(KERN_DEBUG "replace_syscall: Cannot find the system call table address.\n");
        return -1;
    }

    unsigned long cr0;
    cr0 = disable_wp();
    syscall_table[__NR_open] = orig_sys_open;
    restore_wp(cr0);
    return 0;
}


