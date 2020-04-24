#include <linux/bug.h>
#include <linux/printk.h>
#include <linux/uaccess.h>
#include <linux/version.h>

void a_kernel_func() {
    printk(KERN_INFO "a_kernel_func");
} 