// Micro-esperimento di analisi: perché l'interprete Rust è più lento
// di C e Go? Tre varianti dello stesso exec a switch:
//   a = porting fedele (sp/pc u16 + indicizzazione sicura)
//   b = come a, ma con get_unchecked (niente bounds check)
//   c = come a, ma sp/pc usize (niente cast, check eliminabili)
//
// build && run (il linker gcc serve solo su questa macchina, vedi README):
//   rustc -C opt-level=3 -C codegen-units=1 -C linker=gcc -o mb mb.rs
//   ./mb a; ./mb b; ./mb c
//
// micro-esperimento: individuare il collo di bottiglia dell'interprete Rust
// varianti: A = porting fedele (u16 + indicizzazione sicura)
//           B = come A ma indicizzazione unchecked (niente bounds check)
//           C = come A ma sp/pc usize (niente cast u16->usize)
use std::env;

const STACK_MAX: usize = 2048;
const OPCODE_MAX: usize = 2048;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Op { Nop = 0, Return, Add, Sub, Mul, Div }

struct VM { sp: u16, pc: u16, code_size: u16, stack: [f64; STACK_MAX], code: [Op; OPCODE_MAX] }

impl VM {
    fn new() -> Self { VM { sp:0, pc:0, code_size:0, stack:[0.0;STACK_MAX], code:[Op::Nop;OPCODE_MAX] } }
    #[inline(always)] fn pop(&mut self) -> f64 { self.sp -= 1; self.stack[self.sp as usize] }
    #[inline(always)] fn push(&mut self, x: f64) { self.stack[self.sp as usize] = x; self.sp += 1; }
    #[inline(always)] unsafe fn pop_u(&mut self) -> f64 { self.sp -= 1; *self.stack.get_unchecked(self.sp as usize) }
    #[inline(always)] unsafe fn push_u(&mut self, x: f64) { *self.stack.get_unchecked_mut(self.sp as usize) = x; self.sp += 1; }
}

fn load(v: &mut VM) {
    v.pc = 0; v.sp = 0;
    v.stack[0] = 2.0;
    for i in 1..=2000 { v.stack[i] = 1.0; }
    v.stack[2001] = 0.0;
    v.sp = 2002;
    v.code[2001] = Op::Return;
    v.code[2000] = Op::Mul;
    for i in (0..2000).rev() { v.code[i] = Op::Sub; }
    v.code_size = 2002;
}

fn exec_a(v: &mut VM) -> f64 {
    while v.pc < v.code_size {
        let op = v.code[v.pc as usize]; v.pc += 1;
        match op {
            Op::Return => return v.stack[0],
            Op::Add => { let a=v.pop(); let b=v.pop(); v.push(a+b); }
            Op::Sub => { let a=v.pop(); let b=v.pop(); v.push(a-b); }
            Op::Mul => { let a=v.pop(); let b=v.pop(); v.push(a*b); }
            Op::Div => { let a=v.pop(); let b=v.pop(); v.push(a/b); }
            Op::Nop => { println!("nop"); }
        }
    }
    0.0
}

fn exec_b(v: &mut VM) -> f64 {
    while v.pc < v.code_size {
        let op = unsafe { *v.code.get_unchecked(v.pc as usize) }; v.pc += 1;
        match op {
            Op::Return => return v.stack[0],
            Op::Add => unsafe { let a=v.pop_u(); let b=v.pop_u(); v.push_u(a+b); }
            Op::Sub => unsafe { let a=v.pop_u(); let b=v.pop_u(); v.push_u(a-b); }
            Op::Mul => unsafe { let a=v.pop_u(); let b=v.pop_u(); v.push_u(a*b); }
            Op::Div => unsafe { let a=v.pop_u(); let b=v.pop_u(); v.push_u(a/b); }
            Op::Nop => { println!("nop"); }
        }
    }
    0.0
}

struct VMU { sp: usize, pc: usize, code_size: usize, stack: [f64; STACK_MAX], code: [Op; OPCODE_MAX] }
impl VMU {
    fn new() -> Self { VMU { sp:0, pc:0, code_size:0, stack:[0.0;STACK_MAX], code:[Op::Nop;OPCODE_MAX] } }
    #[inline(always)] fn pop(&mut self) -> f64 { self.sp -= 1; self.stack[self.sp] }
    #[inline(always)] fn push(&mut self, x: f64) { self.stack[self.sp] = x; self.sp += 1; }
}
fn load_u(v: &mut VMU) {
    v.pc = 0; v.sp = 0;
    v.stack[0] = 2.0;
    for i in 1..=2000 { v.stack[i] = 1.0; }
    v.stack[2001] = 0.0;
    v.sp = 2002;
    v.code[2001] = Op::Return;
    v.code[2000] = Op::Mul;
    for i in (0..2000).rev() { v.code[i] = Op::Sub; }
    v.code_size = 2002;
}
fn exec_c(v: &mut VMU) -> f64 {
    while v.pc < v.code_size {
        let op = v.code[v.pc]; v.pc += 1;
        match op {
            Op::Return => return v.stack[0],
            Op::Add => { let a=v.pop(); let b=v.pop(); v.push(a+b); }
            Op::Sub => { let a=v.pop(); let b=v.pop(); v.push(a-b); }
            Op::Mul => { let a=v.pop(); let b=v.pop(); v.push(a*b); }
            Op::Div => { let a=v.pop(); let b=v.pop(); v.push(a/b); }
            Op::Nop => { println!("nop"); }
        }
    }
    0.0
}

fn main() {
    let which = env::args().nth(1).unwrap_or_default();
    let mut r = 0.0;
    match which.as_str() {
        "a" => { let mut v = VM::new();  for _ in 0..400000 { load(&mut v);  r = exec_a(&mut v); } }
        "b" => { let mut v = VM::new();  for _ in 0..400000 { load(&mut v);  r = exec_b(&mut v); } }
        "c" => { let mut v = VMU::new(); for _ in 0..400000 { load_u(&mut v); r = exec_c(&mut v); } }
        _ => { eprintln!("uso: mb a|b|c"); return; }
    }
    println!("R: {r}");
}
