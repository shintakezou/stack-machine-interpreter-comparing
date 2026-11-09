//! Le funzioni opcode di native_function_call.c: le stesse della
//! function table, ma chiamate direttamente dal codice srotolato
//! generato (native_call_data.rs), senza interprete.
//!
//! Note di fedeltà al C:
//! - a, b sono globali (static mut): non thread-safe, com'è
//!   nell'originale ("forse converrebbe metterle nelle funzioni");
//! - F_OP_SIN/F_OP_COS usano -(a) al posto di sin/cos, com'è nel
//!   sorgente C.

#![allow(non_snake_case)]

use crate::vm::VM;

pub static mut A: f64 = 0.0;
pub static mut B: f64 = 0.0;

// ======= opcode =======

pub fn F_OP_NOP(v: &mut VM) {
    println!("pc={}: OP_NOP", v.pc.wrapping_sub(1));
}

pub fn F_OP_RETURN(v: &mut VM) {
    v.pc = v.code_size; // termina VM
}

pub fn F_OP_ADD(v: &mut VM) {
    unsafe {
        A = v.pop();
        B = v.pop();
        v.push(A + B);
    }
}

pub fn F_OP_SUB(v: &mut VM) {
    unsafe {
        A = v.pop();
        B = v.pop();
        v.push(A - B);
    }
}

pub fn F_OP_MUL(v: &mut VM) {
    unsafe {
        A = v.pop();
        B = v.pop();
        v.push(A * B);
    }
}

pub fn F_OP_DIV(v: &mut VM) {
    unsafe {
        A = v.pop();
        B = v.pop();
        v.push(A / B);
    }
}

pub fn F_OP_SIN(v: &mut VM) {
    unsafe {
        A = v.pop();
        v.push(-A);
    }
}

pub fn F_OP_COS(v: &mut VM) {
    unsafe {
        A = v.pop();
        v.push(-A);
    }
}
