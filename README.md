# pumpkin_db

A dynamic and generic Wasm-friendly database wrapper for PumpkinMC plugins.

## Usage
```rust
let db = PumpkinDb::<MyStruct>::open("plugins/data.json");
---

### Langkah 5: Cek Kesiapan Crate (*Dry Run*)
Sebelum benar-benar mempublikasikannya secara online, Anda bisa melakukan simulasi atau pengujian lokal untuk memastikan tidak ada file yang korup atau metadata yang kurang. Jalankan perintah ini di folder `pumpkin_db`:

```bash
cargo publish --dry-run
