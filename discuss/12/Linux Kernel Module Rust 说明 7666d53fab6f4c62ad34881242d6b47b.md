# Linux Kernel Module Rust 说明

使用该模块完成一些基本的内核功能。

1. 引用该模块是统一使用简称 `lkm`（在 `Cargo.toml` 中设置简称）。
2. 用户数据和内核数据的交换统一使用 `user_ptr` 完成。
3. 使用系统调用可以使用 syscall 模块完成，注意，当输入的指针是用户指针的时候，使用 `lkm::syscall::user::*` ，而输入的指针已经被转移到内核中之后，使用`lkm::syscall::kern::*` ，两者都是 unsafe ，需要小心。
4. 记得编写一些 log，多写一些 `trace!("description {}", a)`