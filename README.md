# 阿里云图床上传工具

## 开发测试

```shell
cargo run "img/dog.jpg"
```

## 构建

```shell
cargo build --release
```

## 安装到当前系统

### 1. cargo install

> 暂时不放到https://crates.io/

```shell
cargo install --path .
```

![image-20250827144349440](https://cruder-figure-bed.oss-cn-beijing.aliyuncs.com/markdown/2025/08/27/14-43-49-755-09850106c4564ee5956bdbd968f8e43d.png)

### 2. 构建可执行程序，然后配置环境变量