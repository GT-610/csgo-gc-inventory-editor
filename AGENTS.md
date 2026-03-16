# Code Standards and Design Goals for LLM Agents

## Design Goals

### Item Identification System

The inventory system uses two distinct identifiers for each item:

1. **`id` (u64)** - Unique item identifier (root key)
   - Read from the root key in the VDF file (e.g., "3", "5", "7")
   - Used for server-side identification and client-side tracking
   - **Never reused** even after items are consumed
   - Must be unique across all items
   - When creating new items: `max(existing_ids) + 1`

2. **`inventory` (u64)** - Inventory position/status field
   - Dual-purpose field with bit 30 flag:
     - **Bit 30 = 1**: Unacknowledged item (0x40000000 mask)
       - Signals "new item" status requiring player acknowledgment
       - Values: 0x40000000 (Invalid), 0x40000001 (Dropped), 0x40000002 (Crafted), etc.
     - **Bit 30 = 0**: Acknowledged item position
       - Used for sorting and display ordering
       - Can be duplicated in modified versions
   - When creating new items: `max(existing_inventory) + 1`

### Why This Design Matters

- **Root keys are non-sequential**: Items can be consumed (opened in crates, applied stickers, etc.)
  - Example: If items have root keys "1 2 3 4 5" and item "4" is consumed, the correct result is "1 2 3 5"
  - Item "5" does NOT fill the gap left by "4"
  - Reusing root keys would cause game display corruption

- **Separation of concerns**: 
  - `id` = Unique identifier (never changes)
  - `inventory` = Display position/status (can change)

## Code Standards

### Development Workflow

After making any code changes:

1. **Run Clippy with strict warnings**:
   ```bash
   cargo clippy -- -D warnings
   ```
   - Fix all warnings before proceeding
   - This ensures code quality and catches potential issues early

2. **Format the code**:
   ```bash
   cargo fmt
   ```
   - Apply only after Clippy passes
   - Ensures consistent code style across the project

3. **Build and test**:
   ```bash
   cargo build
   ```
   - Don't build release versions unless required
   - Verify the build succeeds before considering the task complete

### Code Quality Guidelines

- **Add concise English comments** in appropriate places
  - Help developers quickly understand the code
  - Focus on "why" rather than "what" (code should be self-explanatory)
  - Keep comments brief and to the point

- **Keep code concise and efficient**
  - Avoid unnecessary complexity
  - Prefer idiomatic Rust patterns
  - Optimize for readability first, performance second

- **Use latest versions** of libraries, packages, and dependencies
  - Regularly update dependencies
  - Follow Rust 2024 Edition standards
  - Check for security updates

- **Follow Rust best practices**:
  - Use `Result` for error handling
  - Prefer `Option` over nullable values
  - Leverage Rust's ownership system
  - Use meaningful variable and function names

### Project-Specific Rules

- **Always use `item.id` as the unique identifier** for:
  - Opening item detail windows
  - Tracking edit states
  - Saving and deleting items
  - Caching display names

- **Never use `item.inventory` as a unique identifier**:
  - It is not unique (multiple items can share the same inventory value)
  - It represents position/status, not identity

- **When creating new items**:
  - Generate both `id` and `inventory` from their respective maximums + 1
  - Ensure both values are unique at creation time

## File Structure

- `src/inventory/` - Inventory data models and parsing
- `src/ui/` - UI components (item grid, detail windows, etc.)
- `src/app.rs` - Main application state and logic
- `src/main.rs` - Entry point and event handling

## Reference
`.trae\references` contains referenceable project code, source code of used libraries, documentation, etc.
