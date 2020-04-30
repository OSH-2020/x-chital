
use linux_kernel_module::file_operations as fops;
use linux_kernel_module::KernelResult;

pub struct IoDeviceFile;

impl fops::FileOperations for IoDeviceFile {
    const VTABLE: fops::FileOperationsVtable =
    fops::FileOperationsVtable::builder::<Self>()
            .ioctl()
            .build();

    fn open() -> KernelResult<Self> {
        return Ok(IoDeviceFile);
    }
}


static mut counter: i32 = 0;

impl fops::Ioctl for IoDeviceFile {
    fn ioctl(&self, cmd:u32, arg: u64) -> KernelResult<i64> {
        unsafe{
            counter += 1;
            info!(": cmd={}, arg={} counter={}", cmd, arg, counter);
        }
        return Ok(0);
    }
}