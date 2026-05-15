[English](README.md)

# CSGO GC 编辑器

一款用于编辑 [CS:GO GC](https://github.com/mikkokko/csgo_gc) 库存和配置的桌面应用程序。

支持 Windows 和 macOS。Linux 版本计划在未来发布。

## 功能特性

<table>
  <tr>
    <td><img width="1548" height="854" alt="主界面" src="https://github.com/user-attachments/assets/44b9c8a3-58dd-4dbc-a28d-a7aad367334a" /></td>
    <td><img width="1548" height="854" alt="物品详情和皮肤选择" src="https://github.com/user-attachments/assets/0e268720-5ea6-4386-bae5-78d4bfe26c71" /></td>
  </tr>
  <tr>
    <td><img width="1092" height="632" alt="从模板创建" src="https://github.com/user-attachments/assets/7ffbf75f-c6c6-44c5-8842-331a335d40e3" /></td>
    <td><img width="1092" height="575" alt="自动同步稀有度" src="https://github.com/user-attachments/assets/bce41022-041d-4d8b-aaf7-a7597d4f6f8b" /></td>
  </tr>
  <tr>
    <td><img width="1548" height="854" alt="玩家配置编辑" src="https://github.com/user-attachments/assets/98ada0ed-4b93-4816-912d-24ed204e6cc2" /></td>
    <td><img width="1548" height="854" alt="所见即所得" src="https://github.com/user-attachments/assets/e2461ae0-ba75-408a-ac33-05221f8aa820" /></td>
  </tr>
</table>

- **库存管理**：查看和编辑 CS:GO 库存物品
- **物品模板**：使用预定义模板创建物品
- **稀有度系统**：按稀有度显示物品颜色（消费级、工业级、军规级、受限级、保密级、隐秘级、违禁级）
- **分类筛选**：按分类筛选物品（全部、装备、印花与涂鸦、武器箱与更多、展示品）
- **搜索**：快速搜索物品
- **多语言支持**：英语和简体中文

## 系统要求

- **图形驱动**：
  - Windows：DirectX 12 (DX12)
  - macOS：Metal
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

# 运行应用程序
cargo run
```

## 许可证

本项目采用 [MIT 许可证](LICENSE)。

应用程序中使用的字体（[Fusion-JetBrainsMapleMono](https://github.com/SpaceTimee/Fusion-JetBrainsMapleMono)）遵循 [OFL-1.1 许可证](https://github.com/SpaceTimee/Fusion-JetBrainsMapleMono/blob/main/OFL.txt)。

## 贡献

欢迎贡献！请随时提交 Pull Request。
