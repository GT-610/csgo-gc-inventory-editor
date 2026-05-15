[简体中文](README_zh-CN.md)

# CSGO GC Editor

A desktop application for editing [CS:GO GC](https://github.com/mikkokko/csgo_gc) inventory and config.

Supports Windows and macOS. Linux support is planned for a future release.

## Features

<table>
  <tr>
    <td><img width="1548" height="854" alt="Main Menu" src="https://github.com/user-attachments/assets/44b9c8a3-58dd-4dbc-a28d-a7aad367334a" /></td>
    <td><img width="1548" height="854" alt="Item details and skin selection" src="https://github.com/user-attachments/assets/0e268720-5ea6-4386-bae5-78d4bfe26c71" /></td>
  </tr>
  <tr>
    <td><img width="1092" height="632" alt="Create from templates" src="https://github.com/user-attachments/assets/7ffbf75f-c6c6-44c5-8842-331a335d40e3" /></td>
    <td><img width="1092" height="575" alt="Automatically sync rarity" src="https://github.com/user-attachments/assets/bce41022-041d-4d8b-aaf7-a7597d4f6f8b" /></td>
  <tr>
    <td><img width="1548" height="854" alt="Player config editing" src="https://github.com/user-attachments/assets/98ada0ed-4b93-4816-912d-24ed204e6cc2" /></td>
    <td><img width="1548" height="854" alt="What you see is what you get" src="https://github.com/user-attachments/assets/e2461ae0-ba75-408a-ac33-05221f8aa820" /></td>
  </tr>
</table>

- **Inventory Management**: View and edit CS:GO inventory items
- **Item Templates**: Create items using predefined templates
- **Rarity System**: Color-coded item rarity display (Consumer, Industrial, Mil-Spec, Restricted, Classified, Covert, Contraband)
- **Category Filtering**: Filter items by category (All, Equipped, Stickers & Graffiti, Cases & More, Collectibles)
- **Search**: Quick search for items
- **Multi-language Support**: English and Simplified Chinese

## Requirements

- **Graphics Driver**: 
  - Windows: DirectX 12 (DX12)
  - macOS: Metal
  - Linux: Vulkan (planned for a future release)

## Building

```bash
# Clone the repository
git clone https://github.com/GT-610/csgo-gc-inventory-editor
cd csgo-gc-inventory-editor

# Build in development mode
cargo build

# Build in release mode
cargo build --release

# Run the application
cargo run
```

## License

This project is under [MIT License](LICENSE).

Font used in the application ([Fusion-JetBrainsMapleMono](https://github.com/SpaceTimee/Fusion-JetBrainsMapleMono)) is licensed under [OFL-1.1 license](https://github.com/SpaceTimee/Fusion-JetBrainsMapleMono/blob/main/OFL.txt).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
