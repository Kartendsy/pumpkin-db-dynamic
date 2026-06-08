# pumpkin_db

A dynamic and generic Wasm-friendly database wrapper for PumpkinMC plugins.

## Usage
```rust
let db = PumpkinDb::<MyStruct>::open("plugins/data.json");
---
