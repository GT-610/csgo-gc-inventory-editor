[简体中文](README_zh-CN.md)

# CSGO Inventory Editor

A cross-platform desktop application for editing [CS:GO GC](https://github.com/mikkokko/csgo_gc) inventory files.

## Features

- **Inventory Management**: View and edit CS:GO inventory items
- **Item Templates**: Create items using predefined templates
- **Rarity System**: Color-coded item rarity display (Consumer, Industrial, Mil-Spec, Restricted, Classified, Covert, Contraband)
- **Category Filtering**: Filter items by category (All, Equipped, Stickers & Graffiti, Cases & More, Collectibles)
- **Search**: Quick search for items
- **Multi-language Support**: English and Simplified Chinese
- **Cross-platform**: Works on Windows, macOS, and Linux

## Requirements

- **Graphics Driver**: 
  - Windows: DirectX 12 (DX12)
  - macOS: Metal
  - Linux: Vulkan

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

## Project Structure

```
csgo-gc-inventory-editor/
├── assets/
│   ├── fonts/              # Application fonts
│   └── languages/          # Localization files
│       ├── en-US.ftl       # English translations
│       └── zh-Hans.ftl     # Chinese translations
├── src/
│   ├── core/               # Core functionality (game directory)
│   ├── inventory/          # Inventory data structures and parsing
│   ├── ui/                 # UI components
│   ├── app.rs              # Main application logic
│   └── main.rs             # Application entry point
├── Cargo.toml              # Rust dependencies
└── README.md               # This file
```

## Dependencies

- [eframe](https://github.com/emilk/egui) - Cross-platform GUI library
- [egui](https://github.com/emilk/egui) - Immediate mode GUI library
- [egui-i18n](https://github.com/Areren/egui-i18n) - Internationalization for egui
- [serde](https://serde.rs/) - Serialization framework
- [regex](https://docs.rs/regex/) - Regular expressions

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
