[简体中文](README_zh-CN.md)

# CSGO GC Editor

A desktop application for editing [CS:GO GC](https://github.com/mikkokko/csgo_gc) inventory and config.

Currently only supports Windows, but macOS and Linux versions are planned for future releases.

## Features

- **Inventory Management**: View and edit CS:GO inventory items
- **Item Templates**: Create items using predefined templates
- **Rarity System**: Color-coded item rarity display (Consumer, Industrial, Mil-Spec, Restricted, Classified, Covert, Contraband)
- **Category Filtering**: Filter items by category (All, Equipped, Stickers & Graffiti, Cases & More, Collectibles)
- **Search**: Quick search for items
- **Multi-language Support**: English and Simplified Chinese

## Requirements

- **Graphics Driver**: 
  - Windows: DirectX 12 (DX12)
  - macOS: Metal (planned for future release)
  - Linux: Vulkan (planned for future release)

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
