## Runrsc

Runrsc部分提供了在terminal中启动rvisor，并在其中创建进程/进行进程控制/输出相关信息的功能。

其使用方法为

```
./runrsc [command]
```

其具体命令包括：

- 在指定路径处创建容器  

  ```
  ./runrsc create [path]
  ```

- 输出当前容器中的全部进程信息  

  ```
  ./runrsc ps
  ```

- 关闭当前容器以及其中的所有进程  

  ```
  ./runrsc shutdown
  ```

- 运行执行指定路径处的程序 

  ```
  ./runrsc exec [path][option]
  ```

  - 可选择利用`-env [name=value]`的方式为该进程添加多个环境变量

  - 可选择在末尾添加`&`使该进程在后台运行而不阻塞terminal

Runrsc利用`insmod`载入rvisor内核，利用`mknod`创建rvisor设备节点，利用`ioctl()`控制rvisor。

rvisor部分提供的接口包括：

- 创建rvisor 

  ```
  ioctl(int fd, RVISOR_CREATE, char *path)
  ```

- 在rvisor中添加进程  

  ```
  ioctl(int fd, RVISOR_ADD_PROC, pid_t pid)
  ```

- 在rvisor中移除进程  

  ```
  ioctl(int fd, RVISOR_REMOVE_PROC, pid_t pid)
  ```

此外，runrsc利用cgroup对rvisor中运行的进程的内存资源进行了限制。

利用Runrsc创建容器后将首先完成基本的初始化，包括载入内核模块，创建设备节点，设置cgroup进程组等，并利用`ioctl(fd, RVISOR_CREATE, path)` 向rvisor部分发出请求。之后，该部分将利用socket接收后续使用Runrsc的相关进程发送的各类请求，完成进程间通信，并具体执行记录对应进程信息/输出当前容器内运行进程信息/关闭容器等任务。

利用Runrsc执行指定程序，将会将对应进程添加到cgroup控制组中，设置相应的环境变量，向主进程发送其信息，并利用`ioctl(fd, RVISOR_ADD_PROC, pid)`对rvisor进行控制。





