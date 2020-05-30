//
// this file contains syscall hook logic.
//



#include <linux/module.h>
#include <linux/init.h>
#include <linux/types.h>
#include <linux/syscalls.h>
#include <linux/delay.h>
#include <linux/sched.h>
#include <linux/version.h>
#include <linux/uaccess.h>
#include <linux/kallsyms.h>
#include <linux/semaphore.h>
#include <asm/cacheflush.h>
#include <linux/bitops.h>
#include <linux/sizes.h>
#include <linux/byteorder/generic.h>
#include <linux/preempt.h>

MODULE_LICENSE("GPL"); // this is required for some reason.

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
    preempt_enable();
    // preempt_enable_no_resched(); // 释放内核控制
}



void **syscall_table = 0;

#define SYSCALL_TABLE_LENGTH 300
void * saved_syscall_table[SYSCALL_TABLE_LENGTH];

int replace_init() {
    unsigned long cr0;
    int i;

    syscall_table = (void **) kallsyms_lookup_name("sys_call_table");

    cr0 = disable_wp(); // disable write protection. you should use this for access to syscall_table
    for(i = 0; i < SYSCALL_TABLE_LENGTH; i++) {
        saved_syscall_table[i] = syscall_table[i]; // yes, copy the content.
        if(!saved_syscall_table[i]) {
            printk(KERN_WARNING "rvisor-kernel replace_init: miss sys_call_table at %d", i);
        }
    }
    restore_wp(cr0);
    return 0;
}

int replace_syscall(unsigned int syscall_num, long (*syscall_fn)(void)) { 
    unsigned long cr0;
    if (!syscall_table) {
        printk(KERN_ERR "rvisor-kernel replace_syscall: Cannot find the system call table address.\n");
        return -1;
    }
    if(syscall_num >= SYSCALL_TABLE_LENGTH) {
        printk(KERN_ERR "rvisor-kernel replace_syscall: syscall_num(%d) >= %d, which is not support\n", syscall_num, SYSCALL_TABLE_LENGTH);
        return -1;
    }
    if(!syscall_fn) {
        printk(KERN_ERR "rvisor-kernel replace_syscall: syscall_fn not found\n", syscall_num, SYSCALL_TABLE_LENGTH);
        return -1;
    }
    printk(KERN_DEBUG "rvisor-kernel replace_syscall: Found the sys_call_table at %16lx.\n", (unsigned long) syscall_table);

    cr0 = disable_wp(); // 关闭内存写保护
    syscall_table[syscall_num] = syscall_fn; // 替换相应的系统调用
    restore_wp(cr0); // 恢复内存写保护
    
    return 0;
}

int replace_clear() {
    unsigned long cr0;
    int i;

    if (!syscall_table) {
        printk(KERN_DEBUG "replace_syscall: Cannot find the system call table address.\n");
        return -1;
    }

    cr0 = disable_wp();
    for(i = 0; i < SYSCALL_TABLE_LENGTH; i++) {
        syscall_table[i] = saved_syscall_table[i];
    }
    restore_wp(cr0);
    return 0;
}

mm_segment_t  protect_fs() {\
    mm_segment_t oldfs;
    oldfs=get_fs();
    set_fs(KERNEL_DS);
    return oldfs;
}

void  release_fs(mm_segment_t oldfs) {
    set_fs(oldfs);
}

unsigned long user_max() {
    return user_addr_max();
}

int read_to(unsigned long p){
    char c;
    return __get_user(c , (const char *)p);
}

int strncpy_from_user2(char * dst, const char * src, unsigned long max) {
    return strncpy_from_user(dst, src, max);
}