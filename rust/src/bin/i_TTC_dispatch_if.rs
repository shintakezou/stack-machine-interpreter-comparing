//! Porting Rust di i_TTC_dispatch_if.c
//! Token Threaded Code - dispatch implementato con if in cascata.

use stackmachine::test3::load_test3;
use stackmachine::vm::{Opcode, VM};

// ===== EXEC =====
fn exec(v: &mut VM) -> f64 {
    while v.pc < v.code_size {
        let op = v.code[v.pc as usize];
        v.pc += 1;

        if op == Opcode::OP_NOP {
            println!("pc={}: OP_NOP", v.pc - 1);
        }
        if op == Opcode::OP_RETURN {
            return v.stack[0];
        }
        if op == Opcode::OP_ADD {
            let a = v.pop();
            let b = v.pop();
            v.push(a + b);
        }
        if op == Opcode::OP_SUB {
            let a = v.pop();
            let b = v.pop();
            v.push(a - b);
        }
        if op == Opcode::OP_MUL {
            let a = v.pop();
            let b = v.pop();
            v.push(a * b);
        }
        if op == Opcode::OP_DIV {
            let a = v.pop();
            let b = v.pop();
            v.push(a / b);
        }
        if op == Opcode::OP_SIN {
            let a = v.pop();
            v.push(a.sin());
        }
        if op == Opcode::OP_COS {
            let a = v.pop();
            v.push(a.cos());
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
