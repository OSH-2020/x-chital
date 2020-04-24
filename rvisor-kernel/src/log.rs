

/// this marco used for debug
/// 
/// we highly recommand use it everywhere
#[macro_export]
macro_rules! info{
    () => ({
        use linux_kernel_module::printk
    });
    ($fmt:expr) => ({
        linux_kernel_module::println!(concat!("rvisor-kernel ",$fmt));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        linux_kernel_module::println!(concat!("rvisor-kernel ",$fmt), $($arg)*);
    });
}