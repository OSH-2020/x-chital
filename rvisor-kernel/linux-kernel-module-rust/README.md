
# Linux Kernel Module Rust

This is a modified copy of github repo: [linux-kernel-module-rust](https://github.com/fishinabarrel/linux-kernel-module-rust).

We have to modify it to run our rvisor-kernel.

# How linux-kernel-module-rust build the kernel module?

## `build.rs`

`build.rs` is rust build script, it will be run in the target/ folder.

It use crate `cc` to build c files. like this:

```
    let mut builder = cc::Build::new();
    builder.compiler(env::var("CLANG").unwrap_or("clang".to_string()));
    builder.target(&target);
    builder.warnings(false);
    builder.file("src/helpers.c");
    for arg in shlex::split(std::str::from_utf8(&output.stdout).unwrap()).unwrap() {
        builder.flag(&arg);
    }
    builder.compile("helpers");
```

## kernel-cflags-finder

kernel-cflags-finder search for kernel build cflags.

As we don't know kernel build reference /lib/modules/$(uname -r)/build exactly (different linux kernel have different build script) , we need to search them.

then we can add the cflag to rust `cc::Build` as we saw before.

If you meet some problem in kernel-cflags-finder, you should test whether you could build a kernel module with your `clang`.

## Makefile & Kbuild

We build kernel module by Makefile, Makefile will use /lib/modules/$(uname -r)/build (we known as $(KDIR)). Then Kbuild file will be triggerd.

Kbuild should contains the infomation of where the compiled library (writen in rust) is, and how to convert them to object file.





