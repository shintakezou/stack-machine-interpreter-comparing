//! Porting Rust di native_baseline.c
//! baseline number crunching di test3: stessa elaborazione espressa
//! in modo idiomatico, per stimare quanto il compilatore riesce a
//! semplificare (gcc a -Ofast la azzera; gc no; vediamo LLVM).

fn run() {
    let mut r = 0.0;
    for _ in 0..2000 {
        r -= 1.0;
    }
    r *= 2.0;
    let _ = r;
}

fn main() {
    for _ in 0..400000 {
        run();
    }
}
