# Bevy UI 模块开发（窗口管理基础之上）

本目录用于交付“UI 部分”的开发成果，内容覆盖：

- UI 系统基础概念
- 按钮创建与交互
- 文本创建与动态更新
- Flexbox 布局
- Grid 布局
- UI 样式与交互实践建议

## 目录说明

- `examples/button_text_demo.rs`：按钮 + 文本 + HUD 动态更新示例
- `examples/flex_grid_demo.rs`：Flex 与 Grid 综合布局示例

## 前置要求

- 已掌握 Bevy 快速入门
- 已掌握 ECS 基础
- 已掌握窗口管理基础（主窗口创建、属性设置）

## 快速运行

如果你还没有 Bevy 工程，可以先初始化：

```bash
cargo new bevy_ui_demo
cd bevy_ui_demo
cargo add bevy
```

然后将本目录示例文件复制到你的 `examples/` 目录，运行：

```bash
cargo run --example button_text_demo
cargo run --example flex_grid_demo
```

## 核心实践要点

1. 按钮交互
- 通过 `Interaction` 检测 `Pressed / Hovered / None`
- 联动修改 `BackgroundColor`、`BorderColor`、按钮文本

2. 文本更新
- 使用系统在 `Update` 阶段刷新文本
- 只更新必要文本，避免高频全量改写

3. 布局
- Flex 适合线性内容和响应式排列
- Grid 适合表格式、仪表盘式复杂布局

4. 样式
- 通过 `BackgroundColor`、`BorderColor`、`BorderRadius`、`TextColor` 增强可读性

5. 性能
- 控制 UI 节点数量
- 降低非必要文本刷新频率
- 减少复杂嵌套布局

## 与窗口管理的衔接

UI 系统依赖窗口进行显示。在窗口管理部分完成后，UI 常见工作流为：

1. 配置主窗口（标题、分辨率、主题）
2. 创建 `Camera2d`
3. 构建 UI 根节点（全屏 `Node`）
4. 挂载按钮、文本、布局容器
5. 在 `Update` 系统中处理交互与文本更新

以上即为本次 UI 部分文件交付内容。