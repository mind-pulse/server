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

## 编译

你也可以随时自行编译最新代码：

```bash
cargo build -r
```

如果觉得编译产物体积大，可以用`upx`压缩一下。

## 常见问题

### 1. 为什么量表硬编码？

- 心理学能够自评的又被心理学界认可的量表本来就不多，没必要使用数据库保存量表。
- 量表结构差异较大，硬编码没有反序列化的过程，方便维护。
