//! Porting Rust di native_function_init.c
//! misura la sola inizializzazione nativa della macchina a stack
//! (VM azzerata + stack riempito, nessuna esecuzione).

use stackmachine::native_init_data::load_init;
use stackmachine::vm::VM;

fn run() {
    let mut v = VM::new(); // VM vm = {0};
    load_init(&mut v);
}

fn main() {
    for _ in 0..400000 {
        run();
    }

    if std::env::var_os("STACKVM_DEBUG").is_some() {
        println!("init only");
    }
}
