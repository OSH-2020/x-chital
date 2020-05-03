use linux_kernel_module as lkm;
use lkm::bindings


struct task_struct {
    pid : bindings::pid_t,
}