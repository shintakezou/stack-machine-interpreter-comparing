//! Porting Rust di i_DCT_dispatch_function_call.c
//! Direct Call Threading: il bytecode contiene direttamente i
//! puntatori alle funzioni da eseguire, l'interprete non ha tabella
//! di conversione (VM e opcode in src/dct.rs, dati generati in
//! src/dct_data.rs).

use stackmachine::dct::VM;
use stackmachine::dct_data::load_test3;

// ===== EXEC =====
fn exec(v: &mut VM) {
    while v.pc < v.code_size {
        let op = v.code[v.pc as usize];
        v.pc += 1;
        op(v);
    }
}

fn main() {
    let mut v = VM::new();

    for _ in 0..400000 {
        load_test3(&mut v);
        exec(&mut v);
    }

    if std::env::var_os("STACKVM_DEBUG").is_some() {
        println!("R: {}", v.stack[v.sp as usize - 1]);
    }
}
