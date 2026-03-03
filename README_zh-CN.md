[English](README.md)

# CSGO 库存编辑器

一款跨平台的桌面应用程序，用于编辑 [CS:GO GC](https://github.com/mikkokko/csgo_gc) 库存文件。

## 功能特性

- **库存管理**：查看和编辑 CS:GO 库存物品
- **物品模板**：使用预定义模板创建物品
- **稀有度系统**：按稀有度显示物品颜色（消费级、工业级军规级、受限级、保密级、隐秘级、违禁级）
- **分类筛选**：按分类筛选物品（全部、装备、印花与涂鸦、武器箱与更多、展示品）
- **搜索**：快速搜索物品
- **多语言支持**：英语和简体中文
- **跨平台**：支持 Windows、macOS 和 Linux

## 系统要求

- **图形驱动**：
  - Windows：DirectX 12 (DX12)
  - macOS：Metal
  - Linux：Vulkan

## 编译构建

```bash
# 克隆仓库
git clone https://github.com/GT-610/csgo-gc-inventory-editor
cd csgo-gc-inventory-editor

# 开发模式编译
cargo build

# 发布模式编译
cargo build --release

# 调试运行
cargo run
```

## 项目结构

```
csgo-gc-inventory-editor/
├── assets/
│   ├── fonts/              # 应用程序字体
│   └── languages/          # 本地化文件
│       ├── en-US.ftl       # 英语翻译
│       └── zh-Hans.ftl     # 中文翻译
├── src/
│   ├── core/               # 核心功能（游戏目录）
│   ├── inventory/          # 库存数据结构和解析
│   ├── ui/                 # UI 组件
│   ├── app.rs              # 主应用程序逻辑
│   └── main.rs             # 应用程序入口
├── Cargo.toml              # Rust 依赖
└── README.md               # 本文件
```

## 依赖库

- [eframe](https://github.com/emilk/egui) - 跨平台 GUI 库
- [egui](https://github.com/emilk/egui) - 即时模式 GUI 库
- [egui-i18n](https://github.com/Areren/egui-i18n) - egui 国际化支持
- [serde](https://serde.rs/) - 序列化框架
- [regex](https://docs.rs/regex/) - 正则表达式

## 许可证

[MIT 许可证](LICENSE)

## 贡献

欢迎贡献！请随时提交 Pull Request。
