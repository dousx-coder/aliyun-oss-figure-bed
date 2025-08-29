# 阿里云图床上传工具

## 环境变量

需要配置以下环境变量

- ALIYUN_KEY_ID
- ALIYUN_KEY_SECRET
- ALIYUN_ENDPOINT
- ALIYUN_BUCKET

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

## 参数说明

新增`md`参数，输出不同的路径结果

![image-20250829162532249](https://cruder-figure-bed.oss-cn-beijing.aliyuncs.com/markdown/2025/08/29/16-25-32-847-66bf9bcdfdc6468c959cee9f4351a027.png)
