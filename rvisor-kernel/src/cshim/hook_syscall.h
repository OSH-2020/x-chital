#pragma once

extern void **syscall_table;

#define SYSCALL_TABLE_LENGTH 300
extern void * saved_syscall_table[SYSCALL_TABLE_LENGTH];