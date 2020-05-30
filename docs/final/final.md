title: 结题报告
speaker: dnailz
plugins:
    - echarts

<slide class="bg-black-blue aligncenter" image="https://source.unsplash.com/C1HhAQrbykQ/ .dark">

# 结题报告 {.text-landing.text-shadow}

By x-chital {.text-intro}

[:fa-github: Github](https://github.com/ksky521/nodeppt){.button.ghost}





<slide :class="size-80">

:::column {.vertical-align}
### **系统调用替换的实现**

得到系统调用表后，编写一个简单的 C-Shim, 实现对系统调用的替换。   {.text-intro}

另外，由于 Linux 的内存保护机制，需要暂时禁止掉内存的写保护。

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