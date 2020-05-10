
首先，确保对 `insmod` `dmesg` 的使用有一定的了解。

编写一个便于搭建容器的client

可以考虑使用rust + clap 来做subcommand，iotcl 可以去 nix 库里面去找。

也可以使用python或者C，语言方面随便（python 可以用 argparse 做subcommand，C 的话要自己折腾，不推荐）。

可以参考 rvisor-kernel/tests/iotcl.c 作为例子参考。

暂时先只实现最基本的功能，后面这里还有些事情可以做。
