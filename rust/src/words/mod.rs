//! Le primitive (words) dello stack language, un file per word come
//! in words/*.c. Gli include! appiattiscono tutto in questo modulo;
//! il transpiler scansiona la directory (mod.rs escluso) per sapere
//! quali primitive esistono.

#![allow(non_snake_case)]

use crate::vm::VM;

include!("ADD.rs");
include!("COS.rs");
include!("DIV.rs");
include!("MUL.rs");
include!("nop.rs");
include!("RETURN.rs");
include!("SIN.rs");
include!("SUB.rs");
