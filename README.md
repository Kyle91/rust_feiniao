# 飞鸟快验 Rust Demo

使用 Rust + Tauri + Vue3 开发的飞鸟快验客户端 Demo。基本上都是AI写的，我不懂Rust，所以有问题问AI吧

## 加密流程

支持两种加密通信方式：

### 1. AES-192-CBC 固定密钥加密：

#### 请求加密流程：

- 构建请求数据（包含 Time、Status、Api 等字段）
- 使用 AES-192-CBC 加密请求数据
- 计算签名：MD5(加密后数据 + AES密钥)
- 发送请求：{"a": "加密后数据", "b": "签名"}

#### 响应解密流程：

- 验证响应签名：MD5(加密数据 + AES密钥)
- 使用 AES-192-CBC 解密响应数据
- 解析 JSON 响应

### 2. RSA + AES 动态密钥加密：

#### 请求加密流程：

- 构建请求数据（包含 Time、Status、Api 等字段）
- 随机生成 AES-192 密钥
- 使用 AES-192-CBC 加密请求数据
- 使用 RSA 公钥加密 AES 密钥
- 发送请求：{"a": "AES加密的数据", "b": "RSA加密的AES密钥"}

#### 响应解密流程：

- 获取加密的 AES 密钥（在响应的 b 字段）
- 使用 RSA 公钥解密得到 AES 密钥
- 使用解密后的 AES 密钥解密响应数据
- 解析 JSON 响应

## 开发环境配置

1. 安装 Rust

   ```bash
   # Windows
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   # 或访问 https://rustup.rs/ 下载安装程序

   # 验证安装
   rustc --version
   cargo --version
   ```

2. 安装 Node.js

   - 从 [Node.js 官网](https://nodejs.org/) 下载并安装 LTS 版本
   - 验证安装：`node --version` 和 `npm --version`

3. 安装系统依赖（Windows）

   - 安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - 选择安装 "Desktop development with C++"

4. 安装 OpenSSL（Windows）
   - 下载 Win64 OpenSSL v3.1.x: https://slproweb.com/products/Win32OpenSSL.html
   - 选择 "Win64 OpenSSL v3.1.x" 的 EXE 安装包
   - 安装时选择 "Copy OpenSSL DLLs to Windows system directory"
   - 设置环境变量：
     ```
     OPENSSL_DIR=C:\Program Files\OpenSSL-Win64
     Path 添加 C:\Program Files\OpenSSL-Win64\bin
     ```

## 项目设置

1. 克隆项目

   ```bash
   git clone <repository-url>
   cd rust_feiniao
   ```

2. 安装依赖

   ```bash
   # 安装前端依赖
   npm install

   # 安装 Rust 依赖（自动完成）
   cd src-tauri
   cargo build
   cd ..
   ```

3. 开发运行

   ```bash
   npm run tauri dev
   ```

4. 构建发布
   ```bash
   npm run tauri build
   ```

## 项目结构

```
├── src/ # 前端 Vue 代码
│ ├── components/ # Vue 组件
│ │ └── LoginRegister.vue # 登录注册组件
│ ├── router/ # 路由配置
│ │ └── index.ts # 路由定义
│ ├── App.vue # Vue 根组件
│ └── main.ts # 前端入口文件
├── src-tauri/ # Rust 后端代码
│ ├── src/
│ │ ├── main.rs # 主入口文件
│ │ ├── lib.rs # 库入口文件
│ │ ├── api.rs # API 通信封装
│ │ ├── crypto.rs # 加密工具类
│ │ └── constants.rs # 常量配置
│ ├── Cargo.toml # Rust 依赖配置
│ └── tauri.conf.json # Tauri 配置文件
├── .cargo/ # Cargo 配置
│ └── config.toml # Cargo 镜像配置
├── package.json # Node.js 依赖配置
├── vite.config.ts # Vite 构建配置
├── tsconfig.json # TypeScript 配置
├── tsconfig.app.json # TypeScript 应用配置
└── tsconfig.node.json # TypeScript Node 配置
```
