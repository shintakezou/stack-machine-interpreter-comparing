//! Porting Rust di native_transpiled_run400k.c
//! esegue il programma transpilato 400k volte per il benchmark.

use stackmachine::transpiled;

fn main() {
    for _ in 0..400000 {
        transpiled::run();
    }
}
