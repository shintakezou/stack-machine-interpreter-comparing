//! Porting Rust di i_TTC_dispatch_function_table.c
//! Token Threaded Code - dispatch implementato con tabella di funzioni.
//!
//! Fedeltà al C: a, b sono globali (static mut, non thread-safe —
//! commento originale: "forse converrebbe metterle nelle funzioni").
//! Unica differenza: la tabella è una static immutabile invece di
//! essere riempita a runtime in main (semantica identica).

#![allow(non_snake_case)]

use stackmachine::test3::load_test3;
use stackmachine::vm::{OPCODE_COUNT, VM};

static mut A: f64 = 0.0;
static mut B: f64 = 0.0;

type OpFn = fn(&mut VM);

static TABLE: [OpFn; OPCODE_COUNT] = [
    F_OP_NOP,
    F_OP_RETURN,
    F_OP_ADD,
    F_OP_SUB,
    F_OP_MUL,
    F_OP_DIV,
    F_OP_SIN,
    F_OP_COS,
];

// ===== EXEC =====
fn exec(v: &mut VM) -> f64 {
    while v.pc < v.code_size {
        let op = v.code[v.pc as usize];
        v.pc += 1;
        TABLE[op as usize](v);
    }
    v.stack[v.sp as usize - 1]
}

// ======= opcode =======
fn F_OP_NOP(v: &mut VM) {
    println!("pc={}: OP_NOP", v.pc.wrapping_sub(1));
}

fn F_OP_RETURN(v: &mut VM) {
    v.pc = v.code_size; // termina VM
}

fn F_OP_ADD(v: &mut VM) {
    unsafe {
        A = v.pop();
        B = v.pop();
        v.push(A + B);
    }
}

fn F_OP_SUB(v: &mut VM) {
    unsafe {
        A = v.pop();
        B = v.pop();
        v.push(A - B);
    }
}

fn F_OP_MUL(v: &mut VM) {
    unsafe {
        A = v.pop();
        B = v.pop();
        v.push(A * B);
    }
}

fn F_OP_DIV(v: &mut VM) {
    unsafe {
        A = v.pop();
        B = v.pop();
        v.push(A / B);
    }
}

fn F_OP_SIN(v: &mut VM) {
    unsafe {
        A = v.pop();
        v.push(A.sin());
    }
}

fn F_OP_COS(v: &mut VM) {
    unsafe {
        A = v.pop();
        v.push(A.cos());
    }
}

fn main() {
    let mut v = VM::new();
    let mut r = 0.0;

    for _ in 0..400000 {
        load_test3(&mut v);
        r = exec(&mut v);
    }

    if std::env::var_os("STACKVM_DEBUG").is_some() {
        println!("R: {r}");
    }
}
