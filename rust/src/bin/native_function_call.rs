//! Porting Rust di native_function_call.c
//! esegue il codice nativo tramite le stesse funzioni di
//! i_TTC_dispatch_function_table, ma senza interprete: le chiamate
//! sono srotolate nel sorgente generato (native_call_data.rs).
//! Le funzioni opcode (con i globali a,b e -(a) per SIN/COS, come
//! nel C) sono in src/native_call.rs.

use stackmachine::native_call_data::load_and_call;
use stackmachine::vm::VM;

fn run() -> f64 {
    let mut v = VM::new(); // VM vm = {0};
    load_and_call(&mut v);
    v.stack[0]
}

fn main() {
    let mut r = 0.0;
    for _ in 0..400000 {
        r = run();
    }

    if std::env::var_os("STACKVM_DEBUG").is_some() {
        println!("R: {r}");
    }
}
