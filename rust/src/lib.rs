//! Porting Rust della stack machine (vedi rust/README.md).
//!
//! I moduli *_data e transpiled sono generati:
//!   cargo run --release --bin gen          (equivalente di test.php)
//!   cargo run --release --bin transpiler   (equivalente di transpiler.c)

pub mod dct;
pub mod dct_data;
pub mod native_call;
pub mod native_call_data;
pub mod native_init_data;
pub mod test3;
pub mod transpiled;
pub mod vm;
pub mod words;
