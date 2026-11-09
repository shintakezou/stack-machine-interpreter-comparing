//! Supporto per la variante DCT (Direct Call Threading).
//!
//! Come in C, dove common.h viene ricompilato con
//! `typedef void (*Opcode)(VM *vm)`, qui la VM ha un tipo tutto suo:
//! il campo code contiene direttamente puntatori a funzione.

pub const STACK_MAX: usize = 2 * 1024;
pub const OPCODE_MAX: usize = 2 * 1024;

/// Equivalente di `typedef void (*Opcode)(VM *vm);`
pub type Opcode = fn(&mut VM);

pub struct VM {
    pub sp: u16,
    pub pc: u16,
    pub code_size: u16,
    pub stack: [f64; STACK_MAX],
    pub code: [Opcode; OPCODE_MAX],
}

impl VM {
    pub fn new() -> Self {
        VM {
            sp: 0,
            pc: 0,
            code_size: 0,
            stack: [0.0; STACK_MAX],
            code: [op_nop as Opcode; OPCODE_MAX],
        }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> f64 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    #[inline(always)]
    pub fn push(&mut self, x: f64) {
        self.stack[self.sp as usize] = x;
        self.sp += 1;
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

// ======= opcode =======
// variabili locali come nel C (questa variante non usa globali);
// anche l'ordine dei pop ricalca i_DCT_dispatch_function_call.c
// (ADD e DIV poppano b prima di a)

pub fn op_nop(v: &mut VM) {
    println!("\npc={}: OP_NOP-S", v.pc.wrapping_sub(1));
}

pub fn op_return(v: &mut VM) {
    v.pc = v.code_size; // termina VM
}

pub fn op_add(v: &mut VM) {
    let b = v.pop();
    let a = v.pop();
    v.push(a + b);
}

pub fn op_sub(v: &mut VM) {
    let a = v.pop();
    let b = v.pop();
    v.push(a - b);
}

pub fn op_mul(v: &mut VM) {
    let a = v.pop();
    let b = v.pop();
    v.push(a * b);
}

pub fn op_div(v: &mut VM) {
    let b = v.pop();
    let a = v.pop();
    v.push(a / b);
}

pub fn op_sin(v: &mut VM) {
    let a = v.pop();
    v.push(a.sin());
}

pub fn op_cos(v: &mut VM) {
    let a = v.pop();
    v.push(a.cos());
}
