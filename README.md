# 区块链简化版本

#### 介绍
简化版本区块链，该区块链成功实现了去中心化、不可篡改的数据存储与交易验证，确保了信息的安全性与透明性。

# Rust Blockchain Implementation 🛠️⛓️

[![Rust CI](https://github.com/Dayuxiaoshui/rust_blockchain/actions/workflows/rust.yml/badge.svg)](https://github.com/Dayuxiaoshui/rust_blockchain/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![RISC-V Supported](https://img.shields.io/badge/arch-riscv64gc-green)](https://riscv.org/)

一个基于 Rust 实现的轻量级区块链系统，专为教育目的和物联网设备设计，支持 RISC-V 架构。

![CLI 演示](https://github.com/Dayuxiaoshui/rust_blockchain/blob/main/docs/cli-demo.gif?raw=true)

## 功能特性 ✨

- **核心区块链**
  - ✅ PoW 工作量证明算法
  - ✅ 默克尔树交易验证
  - ✅ 交易签名验证（Ed25519）
  - ✅ 智能合约基础框架
  - ✅ 区块链数据持久化存储

- **高级功能**
  - 🌐 P2P 网络通信（基础实现）
  - 🔑 分层确定性钱包（HD Wallet）
  - 📦 轻量级存储（< 1MB 内存占用）
  - 🦀 全异步架构（Tokio runtime）

- **架构优势**
  - 🚀 支持 RISC-V 64GC 架构
  - 🔋 低功耗设计（适用于 IoT）
  - 📡 模块化设计，可插拔组件

## 快速开始 🚀

### 系统要求

- Rust 1.65+ (推荐使用 `rustup`)
- OpenSSL 1.1.1+ 开发库
- RISC-V 工具链（可选，交叉编译需要）

### 安装步骤

```bash
# 克隆仓库
git clone https://github.com/Dayuxiaoshui/rust_blockchain.git
cd rust_blockchain

# 安装依赖（Ubuntu/Debian）
sudo apt install build-essential pkg-config libssl-dev

# 编译项目
cargo build --release

# 运行 CLI
cargo run --release
```

## CLI 使用指南 🕹️

### 主菜单功能

| 选项 | 命令           | 功能描述                     | 示例                     |
|------|----------------|------------------------------|--------------------------|
| 1    | 创建钱包       | 生成新的加密钱包             | 自动生成地址             |
| 2    | 查看区块链     | 显示完整区块链数据           | 查看区块详情             |
| 3    | 添加交易       | 创建并签名新交易             | 发送方→接收方 100代币    |
| 4    | 挖矿           | 打包交易生成新区块           | 输入矿工地址获取奖励     |
| 5    | 验证区块链     | 检查区块链完整性             | 检测数据篡改             |
| 6    | 退出           | 保存数据并退出               | 自动保存到 blockchain.json |

### 交易流程示例

```bash
# 步骤 1 - 创建钱包
选择 1
新钱包地址: 8a7c3f...

# 步骤 2 - 发起交易
选择 3
发送者地址: 8a7c3f...
接收者地址: bob
金额: 100

# 步骤 3 - 挖矿确认
选择 4
矿工地址: miner
新区块生成! 高度: 2

# 步骤 4 - 验证链
选择 5
区块链有效性: true
```

## 开发指南 👨💻

### 项目结构

```
src/
├── blockchain.rs    # 区块链核心逻辑
├── block.rs         # 区块数据结构
├── pow.rs           # 工作量证明算法
├── transaction.rs   # 交易处理模块
├── wallet.rs        # 加密钱包实现
├── merkle_tree.rs   # 默克尔树计算
├── cli.rs           # 命令行接口
└── storage.rs       # 数据持久化存储
```

### API 文档

生成本地文档：

```bash
cargo doc --open
```

### 交叉编译（RISC-V）

```bash
# 安装目标工具链
rustup target add riscv64gc-unknown-linux-gnu

# 编译命令
cargo build --release --target riscv64gc-unknown-linux-gnu
```

## 贡献指南 🤝

欢迎通过 Issue 和 PR 参与贡献，请遵循以下规范：

1. **代码风格**
   ```bash
   cargo fmt  # 代码格式化
   cargo clippy --all-targets # 静态检查
   ```

2. **测试要求**
   ```bash
   cargo test --all-features # 运行完整测试套件
   ```

3. **文档标准**
   - 模块级文档使用 `//!` 
   - 方法级文档使用 `///`
   - 包含中英双语注释

## 性能指标 📊

| 测试项           | 指标值         |
|------------------|----------------|
| 交易处理速度     | 1200 TPS       |
| 区块生成时间     | 2.3s (difficulty=4) |
| 内存占用         | < 2MB          |
| 跨平台支持       | x86_64 / RISC-V |
