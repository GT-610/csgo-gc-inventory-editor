# Code Standards and Design Goals for LLM Agents

## Item Identification System

Two distinct identifiers per item:

1. **`id` (u64)** - Unique item identifier (root key from VDF)
   - **Never reused** after items are consumed
   - Must be unique across all items
   - When creating: `max(existing_ids) + 1`

2. **`inventory` (u64)** - Inventory position/status field
   - **Bit 30 = 1** (0x40000000): Unacknowledged item (new, needs player ack)
   - **Bit 30 = 0**: Acknowledged item position (for sorting/display, can duplicate)
   - When creating: `max(existing_inventory) + 1`

## Development Workflow

After code changes:

1. **Clippy**: `cargo clippy -- -D warnings` (fix all)
2. **Format**: `cargo fmt` (after Clippy passes)
3. **Build**: `cargo build` (no release unless required)

## Project-Specific Rules

- **Always use `item.id`** for unique identification (detail windows, edit states, save/delete, caching)
- **Never use `item.inventory`** as identifier - it's position/status, not identity
- **Creating new items**: generate both `id` and `inventory` from respective max + 1
- **Never run `cargo clean`** - deletes required external files (`csgo`, `csgo_gc`)
- **Always use latest library versions**, follow Rust 2024 Edition

## File Structure

- `src/inventory/` - Data models and parsing
- `src/ui/` - UI components
- `src/app.rs` - Main application state/logic
- `src/main.rs` - Entry point and event handling
