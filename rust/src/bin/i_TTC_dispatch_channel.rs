//! i_TTC_dispatch_channel (solo porting, nessun equivalente C)
//! Token Threaded Code - la VM spezzata in pipeline fetch → execute
//! su un channel, come nella variante Go.
//!
//! Differenze rispetto al Go, imposte (in meglio) dal modello Rust:
//!
//! - i thread OS non sono goroutine: spawnarne uno per run
//!   misurerebbe il costo di spawn, non quello del channel. Il
//!   fetcher è quindi UN thread persistente che possiede la propria
//!   copia del bytecode e il proprio PC; per ogni run riceve sul
//!   canale di controllo il SyncSender su cui spedire gli opcode.
//! - la race della versione Go (fetcher superstite che corre sul PC
//!   della run successiva) qui è strutturalmente impossibile: ogni
//!   run ha il suo canale opcode, la RETURN termina l'esecutore che
//!   DROPPA il receiver, la send del fetcher fallisce e la run è
//!   chiusa; le run sono serializzate dalla coda di controllo.
//!
//! Resta il punto della misura: una sincronizzazione per opcode.

use std::sync::mpsc::{channel, sync_channel, Sender, SyncSender};
use std::thread;

use stackmachine::test3::load_test3;
use stackmachine::vm::{Opcode, VM};

const CHAN_BUF: usize = 128;

// ===== EXEC ===== (stadio execute: possiede SP/Stack)
fn exec(v: &mut VM, ctrl_tx: &Sender<SyncSender<Opcode>>) -> f64 {
    let (ops_tx, ops_rx) = sync_channel::<Opcode>(CHAN_BUF);
    ctrl_tx.send(ops_tx).unwrap();

    for op in ops_rx {
        match op {
            Opcode::OP_NOP => {
                println!("pc: OP_NOP");
            }
            Opcode::OP_RETURN => {
                // uscendo, ops_rx viene droppato: la prossima send
                // del fetcher fallisce e la run si chiude
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
    // stadio fetch: thread persistente, possiede bytecode e PC
    let (ctrl_tx, ctrl_rx) = channel::<SyncSender<Opcode>>();
    let fetcher = thread::spawn(move || {
        let mut fv = VM::new();
        load_test3(&mut fv); // il programma è lo stesso per ogni run
        let code_size = fv.code_size;

        for ops_tx in ctrl_rx {
            let mut pc: u16 = 0;
            while pc < code_size {
                let op = fv.code[pc as usize];
                pc += 1;
                if ops_tx.send(op).is_err() {
                    break; // l'esecutore ha fatto RETURN
                }
            }
        }
    });

    let mut v = VM::new();
    let mut r = 0.0;

    for _ in 0..400000 {
        load_test3(&mut v); // stato dello stack per la run
        r = exec(&mut v, &ctrl_tx);
    }

    drop(ctrl_tx); // chiude il canale di controllo: il fetcher esce
    fetcher.join().unwrap();

    if std::env::var_os("STACKVM_DEBUG").is_some() {
        println!("R: {r}");
    }
}
