# Blockchian 

本项目演示了区块链的简易构成部分，你可以在此创建区块，链以及钱包,proof of work 共识算法，以及交易过程等。

智能合约正在学习ing.....
稍后上线

## 运行区块链

可以参考network中的command来查看有哪些，欢迎大家进行代码改进
例如：
```bash
    {"Blocks":""}
    {"CreateWallet":""}
    {"Transaction":"addr":"amount"}
    .....
```

在此过程中，会从你的随机端口号生成一个peerId，每一个peerId可创一个钱包，
用户之间的交易可以通过
```
{"Transaction":"address":"amout"}
```
来完成。

## 运行代码

在运行代码之前，确保你已经安装了 Rust 编程语言以及 Cargo 包管理器。

### 构建可执行程序

在终端中使用以下命令构建可执行程序：

```bash
cargo build --release
```

这将会在项目目录下生成一个名为 \`target/release/main.exe\` 的可执行程序。

### 运行代码

在终端中使用以下命令运行代码：

```bash
cargo run
```

这将会编译并运行代码。请确保在代码中已经定义了 \`main\` 函数。

## 贡献

如果你希望贡献代码或改进本项目，请先进行以下操作：

1. Fork 本项目
2. 在你的本地克隆项目：\`git clone https://github.com/zhangzijie-pro/blockchian.git\`
3. 进入项目目录：\`cd 项目名称\`
4. 运行代码：\`cargo run\`

请确保你已经安装了 Rust 编程语言以及 Cargo 包管理器。

5. 在你的本地进行修改、添加新功能或修复错误
6. 将修改推送到你的 GitHub 仓库：\`git push origin master\`
7. 创建一个 Pull 请求，向本项目的 \`master\` 分支提交你的修改


## 联系我们

如果您有任何问题或建议，请提交问题或发送电子邮件至zzj01262022@163.com。
感谢您使用我们的服务!

## 许可证

本项目使用 [MIT 许可证](LICENSE)。
