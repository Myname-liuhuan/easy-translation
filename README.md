# Easy Translation - 快速中英文翻译工具

一个基于 Tauri + Vue + TypeScript 构建的桌面翻译应用，支持中英文互译和单词分析功能。

## 功能特性

- 🚀 **快速翻译**: 支持中英文实时互译
- 📖 **单词分析**: 提供详细的单词释义和例句
- 🎯 **全局快捷键**: 支持快速唤出翻译窗口
- 🌐 **多翻译引擎**: 支持百度、DeepL、Google等翻译引擎
- 💾 **本地词典**: 内置SQLite词典，支持离线查询

## 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust (Tauri)
- **数据库**: SQLite
- **构建工具**: Tauri CLI + Cargo

## 环境要求

### 开发环境
- Node.js 18+ 和 npm
- Rust 工具链 (rustc, cargo)
- Tauri CLI (`npm install -g @tauri-apps/cli`)

### 平台特定要求
- **macOS**: Xcode 命令行工具 (`xcode-select --install`)
- **Windows**: Microsoft Visual Studio C++ 构建工具 和 WiX Toolset
- **Linux**: 基础开发工具 (gcc, make 等) 和 libwebkit2gtk

## 安装与运行

### 1. 安装依赖
```bash
# 安装前端依赖
npm install

# 安装 Rust 依赖（自动通过 Cargo 管理）
```

### 2. 开发模式运行
```bash
# 启动开发服务器
npm run tauri dev
```

### 3. 构建前端
```bash
# 构建前端资源到 dist/ 目录
npm run build
```

## 打包与发布

### 基础打包命令

```bash
# 为当前平台构建应用
npm run tauri build
```

### 多平台打包指南

#### macOS 打包
```bash
# 为当前 macOS 架构打包
npm run tauri build

# 生成的文件位于:
# src-tauri/target/release/bundle/
#   - easy-translation.app (应用程序)
#   - easy-translation_0.1.0_x64.dmg (安装镜像)
```

**macOS 注意事项:**
- 需要 Xcode 命令行工具
- 如需发布到 App Store，需要开发者证书进行代码签名
- 可以使用 `--target universal-apple-darwin` 生成通用二进制文件

#### Windows 打包
```bash
# 在 Windows 系统上打包
npm run tauri build

# 生成的文件位于:
# src-tauri/target/release/bundle/
#   - easy-translation_0.1.0_x64-setup.exe (安装程序)
#   - easy-translation_0.1.0_x64.msi (MSI安装包)
```

**Windows 注意事项:**
- 需要安装 WiX Toolset 以生成 .msi 安装包
- 需要 Microsoft Visual Studio C++ 构建工具
- 可以使用 `--target x86_64-pc-windows-msvc` 指定目标架构

#### Linux 打包
```bash
# 在 Linux 系统上打包
npm run tauri build

# 生成的文件位于:
# src-tauri/target/release/bundle/
#   - easy-translation_0.1.0_amd64.deb (Debian/Ubuntu)
#   - easy-translation_0.1.0_x86_64.AppImage (通用Linux)
#   - easy-translation-0.1.0-1.x86_64.rpm (RHEL/Fedora)
```

**Linux 注意事项:**
- 需要安装相应的打包工具：`dpkg` (deb), `rpm` (rpm)
- 需要 libwebkit2gtk 库
- 可以使用 `--target x86_64-unknown-linux-gnu` 指定目标架构

### 交叉编译（在一个平台上为其他平台打包）

#### 在 macOS 上为其他平台打包
```bash
# 安装交叉编译工具链
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu

# 为 Windows 打包（需要安装 mingw-w64）
npm run tauri build -- --target x86_64-pc-windows-msvc

# 为 Linux 打包（需要安装相应工具链）
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

#### 在 Windows 上为其他平台打包
```bash
# 安装 Linux 交叉编译工具链
rustup target add x86_64-unknown-linux-gnu

# 为 Linux 打包
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### 高级打包选项

#### 1. 仅构建不打包
```bash
# 只构建 Rust 代码，不生成安装包
npm run tauri build -- --debug  # 调试模式
npm run tauri build -- --release  # 发布模式（默认）
```

#### 2. 自定义输出目录
```bash
# 指定输出目录
npm run tauri build -- --target-dir ./custom-target
```

#### 3. 跳过前端构建
```bash
# 如果已经构建了前端，可以跳过前端构建步骤
npm run tauri build -- --skip-frontend-build
```

### 打包输出文件说明

打包完成后，生成的文件位于 `src-tauri/target/release/bundle/` 目录：

- **macOS**:
  - `.app`: macOS 应用程序包
  - `.dmg`: 磁盘镜像安装包

- **Windows**:
  - `.exe`: Windows 安装程序
  - `.msi`: Windows 安装包（需要 WiX Toolset）

- **Linux**:
  - `.deb`: Debian/Ubuntu 安装包
  - `.AppImage`: 通用 Linux 应用程序
  - `.rpm`: RHEL/Fedora 安装包

## 故障排除

### 常见问题

1. **Rust 工具链问题**
   ```bash
   # 更新 Rust 工具链
   rustup update
   
   # 添加必要的目标平台
   rustup target add <target-triple>
   ```

2. **前端构建失败**
   ```bash
   # 清理 node_modules 并重新安装
   rm -rf node_modules
   npm install
   ```

3. **Tauri 构建错误**
   ```bash
   # 清理 Tauri 构建缓存
   cd src-tauri
   cargo clean
   cd ..
   ```

4. **平台特定问题**
   - **macOS**: 确保已安装 Xcode 命令行工具
   - **Windows**: 确保已安装 WiX Toolset 和 Visual Studio 构建工具
   - **Linux**: 确保已安装必要的开发库

### 获取帮助
- [Tauri 官方文档](https://tauri.app/)
- [Vue 3 文档](https://vuejs.org/)
- [Rust 文档](https://www.rust-lang.org/)

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 贡献指南

欢迎提交 Issue 和 Pull Request 来改进这个项目。

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request
