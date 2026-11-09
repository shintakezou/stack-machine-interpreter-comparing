//! Porting Rust di common.h + Opcode_enum.c: la struttura dati
//! condivisa dalle implementazioni TTC e native.
//!
//! Come in C, stack e code sono array a dimensione fissa dentro la
//! struct (niente Vec): `VM::new()` equivale a `VM vm = {0}` e
//! l'accesso non passa per indirezioni.
//!
//! Opcode è un enum #[repr(u8)]: corrisponde alla variante C
//! compilata con -fshort-enums (l'enum C di default è un int).

pub const STACK_MAX: usize = 2 * 1024;
pub const OPCODE_MAX: usize = 2 * 1024;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    OP_NOP = 0,
    OP_RETURN,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_SIN,
    OP_COS,
}

pub const OPCODE_COUNT: usize = 8;

pub struct VM {
    pub sp: u16,
    pub pc: u16,
    pub code_size: u16,
    pub stack: [f64; STACK_MAX],
    pub code: [Opcode; OPCODE_MAX],
}

impl VM {
    /// Equivalente di `VM vm = {0};`
    pub fn new() -> Self {
        VM {
            sp: 0,
            pc: 0,
            code_size: 0,
            stack: [0.0; STACK_MAX],
            code: [Opcode::OP_NOP; OPCODE_MAX],
        }
    }

    // pop e push: equivalenti di stack_pop/stack_push (static inline
    // in C); #[inline(always)] replica __attribute__((always_inline))

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
