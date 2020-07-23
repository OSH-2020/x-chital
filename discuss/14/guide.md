
目前我那边进展比较缓慢，导致这个项目也比较难做。因为只能使用一部分功能作为一个容器而言现在还太牵强。

这个东西目前还不是很急着搞，你尽可能先了解一下。（整个过程花不了多长时间）

总的来说，这一部分不很重要，只要说明我们做了就行了。而且也基本上都是抄/改别人的代码！

另外，我时间比较紧，你要是看的话要自行安装 go、docker 等你需要的。

## go 的使用

见动员文档

## 搭建镜像

可以用 docker hub 现成的镜像。也可以用 docker build + DOCKERFILE 自备镜像（自行参考https://github.com/docker-library/hello-world/tree/master/amd64/hello-world）

pull 镜像使用 `ctr pull <image name>:latest` 构建镜像用 `docker build` 然后放入 `ctr` 使用（docker和ctr(containerd) 的镜像是一样的）。

## 运行

ctr 的基本使用方法可以参照 https://zhuanlan.zhihu.com/p/111057726 中对wasm的做法。

需要添加 /etc/containerd/config.toml 配置文件，这个文件我已经给你准备好了。

```
sudo ctr run --rm --runtime io.containerd.rvisor.v1 docker.io/denverdino/hellowasm:latest
```

## 原理

参考runrsc/container.go文件，其中有这样几行(可以用搜索把它搜出来)：

```
	cmd := exec.Command("echo", args...)
	cmd := exec.Command("runrsc", args...)
```

大致来说，运行 ctr run就会执行上面的代码，通过修改上面的代码并查看相关的命令参数就可以了解其使用原理了。

