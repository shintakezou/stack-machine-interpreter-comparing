//! Porting Rust di i_TTC_dispatch_switch.c
//! Token Threaded Code - dispatch implementato con match (lo switch
//! di Rust: su un enum #[repr(u8)] compila in una jump table come lo
//! switch C).

use stackmachine::test3::load_test3;
use stackmachine::vm::{Opcode, VM};

// ===== EXEC =====
fn exec(v: &mut VM) -> f64 {
    while v.pc < v.code_size {
        let op = v.code[v.pc as usize];
        v.pc += 1;

        match op {
            Opcode::OP_NOP => {
                println!("pc={}: OP_NOP", v.pc - 1);
            }
            Opcode::OP_RETURN => {
                return v.stack[0];
            }
            Opcode::OP_ADD => {
                let a = v.pop();
                let b = v.pop();
                v.push(a + b);
            }
            Opcode::OP_SUB => {
                let a = v.pop();
                let b = v.pop();
                v.push(a - b);
            }
            Opcode::OP_MUL => {
                let a = v.pop();
                let b = v.pop();
                v.push(a * b);
            }
            Opcode::OP_DIV => {
                let a = v.pop();
                let b = v.pop();
                v.push(a / b);
            }
            Opcode::OP_SIN => {
                let a = v.pop();
                v.push(a.sin());
            }
            Opcode::OP_COS => {
                let a = v.pop();
                v.push(a.cos());
            }
        }
    }
    0.0
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
