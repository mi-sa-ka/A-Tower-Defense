# Bevy UI + 窗口管理（Rust 软件工程项目）

这是一个可运行的 Rust 工程，覆盖文档要求中的 UI 与窗口管理核心能力：

- 窗口创建与配置
- 多窗口支持
- 运行时窗口属性切换
- 按钮交互
- 文本创建与动态更新（FPS）
- 布局（Flex / Grid 示例）

## 技术栈

- Rust 2021
- Bevy 0.15

## 项目结构

- `src/main.rs`：程序入口与插件注册
- `src/ui.rs`：UI 创建、按钮交互、FPS 文本更新
- `src/windowing.rs`：窗口管理（多窗口、VSync、窗口层级、鼠标抓取）
- `examples/flex_grid_demo.rs`：Flex + Grid 布局示例
- `examples/button_text_demo.rs`：按钮与文本基础示例

## 运行方式

1. 进入项目目录

```bash
cd bevy-ui-window-project
```

2. 运行主程序

```bash
cargo run
```

3. 运行示例

```bash
cargo run --example button_text_demo
cargo run --example flex_grid_demo
```

## 主程序交互说明

- `V`：切换 VSync（`AutoVsync` / `AutoNoVsync`）
- `T`：切换窗口层级（底层 / 普通 / 顶层）
- `C`：切换鼠标抓取模式（`None` / `Locked` / `Confined`）
- 鼠标悬停/点击按钮可看到 UI 状态变化

## 环境要求

- 安装 Rust（建议 stable）
- 能正常下载 crates 依赖（首次编译会较慢）

## Windows 常见错误修复（link.exe not found）

当出现 `link.exe not found` 时，说明没有可用的 MSVC C++ 构建工具。

1. 安装 Build Tools（管理员 PowerShell）

```bash
winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --source winget --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools"
```

2. 重启终端后，执行环境检查脚本

```bash
powershell -ExecutionPolicy Bypass -File .\scripts\setup-windows-msvc.ps1 -RunCheck
```

3. 如果仍有环境变量问题，使用开发者命令环境运行

```bash
.\scripts\run-after-vsdevcmd.bat
```

说明：VS Code 只是编辑器，Rust 的 `x86_64-pc-windows-msvc` 目标需要 Visual C++ 链接器（`link.exe`）。
