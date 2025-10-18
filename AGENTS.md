# Agent Guidelines for SuperSDG

## Build & Test Commands
- **Run locally (native)**: `cargo run`
- **Run in browser**: `trunk serve`
- **Build for web**: `trunk build`
- **Format code**: `cargo fmt`
- **Lint**: `cargo clippy`
- **Check compilation**: `cargo check`
- **Run tests**: `cargo test`
- **Run single test**: `cargo test test_name`

## Code Style
- **Language**: Rust Edition 2024, Bevy 0.17.2
- **Imports**: Group by `std`, `bevy`, external crates, then local modules
- **Formatting**: Use `cargo fmt` (rustfmt defaults)
- **Comments**: Minimal; code should be self-explanatory
- **Error Handling**: Use `Result` and `Option` idiomatically
- **Naming**: Snake_case for functions/variables, PascalCase for types/traits
- **Types**: Use Bevy's ECS components and resources; leverage type inference where clear
- **Patterns**: Use `let-else` and `if-let-chain` idioms (Edition 2024)
