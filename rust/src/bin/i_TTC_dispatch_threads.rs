//! i_TTC_dispatch_threads (solo porting, nessun equivalente C;
//! è l'analogo di i_TTC_dispatch_goroutines del porting Go)
//! Token Threaded Code - il parallelismo messo dove è sensato: non
//! dentro la VM (una run è sequenziale), ma tra le 400k run
//! indipendenti del benchmark.
//!
//! Un channel distribuisce blocchi di iterazioni a un pool di thread
//! (uno per CPU); ogni worker ha la propria VM e usa il dispatch a
//! match, identico a i_TTC_dispatch_switch. Il Receiver di mpsc non
//! è condivisibile: i worker lo estraggono da un Arc<Mutex<_>> (la
//! contesa è irrilevante: un lock ogni 1000 run).

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

use stackmachine::test3::load_test3;
use stackmachine::vm::{Opcode, VM};

const TIMES: usize = 400000;
const CHUNK: usize = 1000; // iterazioni per messaggio sul channel

// ===== EXEC ===== (identica a i_TTC_dispatch_switch, thread-safe:
// nessuna variabile globale)
fn exec(v: &mut VM) -> f64 {
    while v.pc < v.code_size {
        let op = v.code[v.pc as usize];
        v.pc += 1;

        match op {
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
            _ => {}
        }
    }
    0.0
}

fn main() {
    let workers = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let (tx, rx) = channel::<usize>();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = Vec::with_capacity(workers);
    for _ in 0..workers {
        let rx = Arc::clone(&rx);
        handles.push(thread::spawn(move || {
            let mut v = VM::new(); // una VM per thread
            loop {
                let n = match rx.lock().unwrap().recv() {
                    Ok(n) => n,
                    Err(_) => break, // canale chiuso: fine lavoro
                };
                for _ in 0..n {
                    load_test3(&mut v);
                    exec(&mut v);
                }
            }
        }));
    }

    let mut sent = 0;
    while sent < TIMES {
        let n = CHUNK.min(TIMES - sent);
        tx.send(n).unwrap();
        sent += n;
    }
    drop(tx);

    for h in handles {
        h.join().unwrap();
    }

    if std::env::var_os("STACKVM_DEBUG").is_some() {
        let mut v = VM::new();
        load_test3(&mut v);
        println!("R: {} workers: {workers}", exec(&mut v));
    }
}
