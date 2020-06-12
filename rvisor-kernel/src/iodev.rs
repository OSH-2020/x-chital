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
use linux_kernel_module::bindings;
use linux_kernel_module::Error;
use crate::container::Container;

use alloc::borrow::ToOwned;
use alloc::string::String;
use crate::kernel;
use crate::string;

#[repr(u32)]
enum IoctlCmd {
    Create = 0,
    AddProc = 1,
    Remove = 2,
}

impl IoctlCmd {
    fn try_from(i : u32) -> KernelResult<IoctlCmd> {
        match i {
            0 => Ok(IoctlCmd::Create),
            1 => Ok(IoctlCmd::AddProc),
            2 => Ok(IoctlCmd::Remove),
            _ => Err(Error::EINVAL),
        }
    }
}

/// 输入输出文件，
pub struct IoDeviceFile {}


/// 实现 IoDeviceFile trait，
impl fops::FileOperations for IoDeviceFile {
    /// 原本的 `struct * file_operations`
    const VTABLE: fops::FileOperationsVtable =
    fops::FileOperationsVtable::builder::<Self>()
            .ioctl()
            .build();

    fn open() -> KernelResult<Self> {
        info!("open called");
        return Ok(IoDeviceFile{});
    }
}

/// 对用户空间的iotcl调用做出反应
impl fops::Ioctl for IoDeviceFile {
    fn ioctl(&self, cmd:u32, arg: u64) -> KernelResult<i64> {
        info!("ioctl cmd={} arg={}", cmd, arg);
        let mut container = Container::get_container();
        let cmd = IoctlCmd::try_from(cmd)?;
        match cmd {
            // create 命令新建一个容器环境
            IoctlCmd::Create => {
                let path_str = string::read_from_user(arg, kernel::PATH_MAX)?;
                container.init(path_str)?;
                Ok(0)
            }
            // addproc 增加一个进程
            IoctlCmd::AddProc => {
                container.add_task(arg as i32)?;
                Ok(0)
            }
            // remove 删除一个进程
            IoctlCmd::Remove => {
                container.remove_task(arg as i32)?;
                Ok(0)
            }
        }

    }
}