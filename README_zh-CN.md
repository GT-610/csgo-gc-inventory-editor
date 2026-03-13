[English](README.md)

# CSGO GC 编辑器

一款用于编辑 [CS:GO GC](https://github.com/mikkokko/csgo_gc) 库存和配置的桌面应用程序。

目前只支持 Windows，但 macOS 和 Linux 版本计划在未来发布。

## 功能特性

- **库存管理**：查看和编辑 CS:GO 库存物品
- **物品模板**：使用预定义模板创建物品
- **稀有度系统**：按稀有度显示物品颜色（消费级、工业级军规级、受限级、保密级、隐秘级、违禁级）
- **分类筛选**：按分类筛选物品（全部、装备、印花与涂鸦、武器箱与更多、展示品）
- **搜索**：快速搜索物品
- **多语言支持**：英语和简体中文

## 系统要求

- **图形驱动**：
  - Windows：DirectX 12 (DX12)
  - macOS：Metal（计划未来发布）
  - Linux：Vulkan（计划未来发布）

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

## 许可证

本项目采用 [MIT 许可证](LICENSE)。

应用程序中使用的字体（[Fusion-JetBrainsMapleMono](https://github.com/SpaceTimee/Fusion-JetBrainsMapleMono)）遵循 [OFL-1.1 许可证](https://github.com/SpaceTimee/Fusion-JetBrainsMapleMono/blob/main/OFL.txt)。


## 贡献

欢迎贡献！请随时提交 Pull Request。
