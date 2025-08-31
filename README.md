# 阿里云图床上传工具

> 1. 支持多个图片上传
>
> 2. 支持`md`参数控制输出结果

## 1. 环境变量

需要配置以下环境变量

- ALIYUN_KEY_ID
- ALIYUN_KEY_SECRET
- ALIYUN_ENDPOINT
- ALIYUN_BUCKET

## 2. 开发测试

1. 输出`Typora`自定义命令格式

```shell
cargo run "img/dog.jpg" "img/dog2.jpg"
```

2. 输出`md`可访问格式

```shell
cargo run md "img/dog.jpg" "img/dog2.jpg"
```

运行效果示例
![2025/08/31/21-08-36-897-6464d12bab254e388a455045c317ad46.png](https://cruder-figure-bed.oss-cn-beijing.aliyuncs.com/markdown/2025/08/31/21-08-36-897-6464d12bab254e388a455045c317ad46.png)

## 3. 构建

### 3.1 使用cargo install安装

> 暂时不放到https://crates.io/

```shell
cargo install --path .
```

![image-20250827144349440](https://cruder-figure-bed.oss-cn-beijing.aliyuncs.com/markdown/2025/08/27/14-43-49-755-09850106c4564ee5956bdbd968f8e43d.png)

### 3.2 构建可执行程序，将可执行文件放到指定目录然后配置环境变量

```shell
cargo build --release
```



## 4. 额外说明

### 4.1 mac需要注意

1. `install` 之后，`zsh`能执行，但是`Typora`如果不配置绝对路径会找不到命令。
2. zsh配置环境变量之后，无法读不到环境变量（原因待排查），这种情况暂时先在代码中配置变量(仅在当前进程中生效)。

![image-20250831214024084](https://cruder-figure-bed.oss-cn-beijing.aliyuncs.com/markdown/2025/08/31/21-40-24-119-5f5a665162c7436ea7dceac1b17f2b2b.png)

![image-20250831214406637](https://cruder-figure-bed.oss-cn-beijing.aliyuncs.com/markdown/2025/08/31/21-44-06-682-28b063097c1f4b8da55d6db234231694.png)