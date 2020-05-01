//! A Device for User Interact.
//! 
//! 用户空间与系统空间的交互，常常通过ioctl系统调用完成，ioctl可以向一个设备文件发送设备定义的命令和参数，实现用户与设备的交互。
//! 
//! 这里编写一个设备文件用于io。可以使用 `ioctl(open("/dev/rvisor"), command, args)` 与外部应用程序做交互
//! 
//! c中的实现方法是，实现方法是构造一个 [file_operations](https://docs.huihoo.com/doxygen/linux/kernel/3.7/structfile__operations.html) (本身作为一个c struct)。
//! 
//! 它现在与rust中的`linux_kernel_module::file_operations::FileOperationsVtable` 相对应。
//! 
use linux_kernel_module::file_operations as fops;
use linux_kernel_module::KernelResult;

/// 输入输出文件，
pub struct IoDeviceFile;


/// 实现 IoDeviceFile trait，
impl fops::FileOperations for IoDeviceFile {
    const VTABLE: fops::FileOperationsVtable =
    fops::FileOperationsVtable::builder::<Self>()
            .ioctl()
            .build();

    fn open() -> KernelResult<Self> {
        return Ok(IoDeviceFile);
    }
}

/// 用于测试的静态变量
static mut counter: i32 = 0;

/// 对用户空间的iotcl调用做出反应
impl fops::Ioctl for IoDeviceFile {
    fn ioctl(&self, cmd:u32, arg: u64) -> KernelResult<i64> {
        unsafe{
            counter += 1;
            info!(": cmd={}, arg={} counter={}", cmd, arg, counter);
        }
        return Ok(0);
    }
}