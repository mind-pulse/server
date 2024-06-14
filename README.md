# 心灵脉冲服务器

小微型网站专用。

## 使用

[release](https://github.com/mind-pulse/server/releases/latest) 中下载最新版本，上传到你自己的服务器即可使用。

在服务器中解压文件：

```
tar zxvf mind-pulse-server-linux-x64-*.tar.gz
```

你会得到一个`server`文件，可以直接运行：

```bash
./server
```

根据此服务使用的 host 和 port 配置反向代理，即可在你自己的网站中使用。

```
host：127.0.0.1
port：9999
```
