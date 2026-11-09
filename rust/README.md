# Porting Rust della stack machine

Porting in Rust del benchmark C (stessa solfa del porting Go sul
branch go-lang): stessi modelli di interpretazione, transpilazione
compresa, più le due varianti con channel e thread. Solo std, nessuna
dipendenza esterna.

## Corrispondenza con la versione C

| C | Rust |
|---|---|
| `common.h` + `Opcode_enum.c` | `src/vm.rs` |
| `test.php` → `test3.c`, `test3_function_*.c` | `cargo run --bin gen` → `src/test3.rs`, `src/dct_data.rs`, `src/native_*_data.rs` |
| `i_TTC_dispatch_switch.c` | `src/bin/i_TTC_dispatch_switch.rs` (match) |
| `i_TTC_dispatch_if.c` | `src/bin/i_TTC_dispatch_if.rs` |
| `i_TTC_dispatch_function_table.c` | `src/bin/i_TTC_dispatch_function_table.rs` |
| `i_DCT_dispatch_function_call.c` | `src/bin/i_DCT_dispatch_function_call.rs` + `src/dct.rs` |
| `i_TTC_dispatch_computed_goto.c` | — (vedi sotto) |
| `i_TTC_dispatch_function_table_shared.c` | — (vedi sotto) |
| `native_*.c` | `src/bin/native_*.rs` |
| `transpiler.c` + `words/*.c` | `src/bin/transpiler.rs` + `src/words/` |
| `transpiled_main.c` + `native_transpiled_run400k.c` | `src/transpiled.rs` (generato) + `src/bin/native_transpiled_run400k.rs` |
| — | `src/bin/i_TTC_dispatch_channel.rs` (nuova) |
| — | `src/bin/i_TTC_dispatch_threads.rs` (nuova, = goroutines del porting Go) |

Ogni binario ricalca il file C corrispondente: stessi nomi di
funzione, stesso ordine dei pop (in DCT, ADD e DIV poppano b prima di
a), stessi dettagli (`-(a)` per SIN/COS in `native_function_call`;
`a, b` globali — `static mut` — nella function table, non
thread-safe com'è nel C).

## Scelte di porting

- **VM**: struct identica alla C, array a dimensione fissa dentro la
  struct; `VM::new()` equivale a `VM vm = {0}`. `pop`/`push` sono
  `#[inline(always)]`, il gemello di `__attribute__((always_inline))`.
- **Opcode**: enum `#[repr(u8)]`, come il C con `-fshort-enums`. Il
  `match` su un enum così compila in una jump table come lo switch C,
  e in più è esaustivo: niente caso di default dimenticato.
- **computed_goto**: non portato, Rust non ha label-as-values (come
  Go); l'analogo è la function table.
- **function_table_shared / hotreload**: non portato (si farebbe con
  `libloading`/`dlopen`, ma vorrebbe dire uscire da std).
- **DCT**: come in C, dove `common.h` viene ricompilato con `Opcode =
  puntatore a funzione`, la variante ha la sua VM (`src/dct.rs`) con
  `code: [fn(&mut VM); 2048]`.
- I dati di test sono **srotolati** (2000+ statement) come i file
  generati C: l'init fa parte della misura. `cargo run --bin gen` li
  rigenera (è il `test.php` del porting).
- **Toolchain su questa macchina**: il rustc in uso ha la std
  compilata contro la glibc di Guix, quindi `.cargo/config.toml`
  imposta come linker `gcc` (il wrapper Guix) al posto del `cc` di
  sistema, altrimenti i binari non partono (version `GLIBC_2.3x` not
  found).

## Le varianti con channel e thread

**`i_TTC_dispatch_channel`** — la VM spezzata in pipeline
fetch → execute su un channel (`std::sync::mpsc`), come nella
variante Go, ma con due differenze imposte dal modello Rust, in
meglio:

1. i thread OS non sono goroutine: spawnarne uno per run
   misurerebbe lo spawn, non il channel. Il fetcher è un thread
   persistente che possiede la *propria copia* del bytecode e il
   proprio PC; a ogni run riceve sul canale di controllo il
   `SyncSender` su cui spedire gli opcode.
2. la race della versione Go (fetcher superstite che corre sul PC
   della run successiva — trovata davvero, al primo run del
   benchmark Go) qui è **strutturalmente impossibile**: ogni run ha
   il suo canale, la RETURN fa uscire l'esecutore che droppa il
   receiver, la send del fetcher fallisce e la run si chiude; le run
   sono serializzate dalla coda di controllo. Il borrow checker non
   permette comunque di condividere `&mut VM` tra i due thread: la
   separazione fetch/execute che in Go era una convenzione (PC al
   fetcher, stack all'esecutore) qui è imposta dai tipi.

**`i_TTC_dispatch_threads`** — il parallelismo dove è sensato: le
400k run indipendenti distribuite a blocchi, via channel, a un pool
di thread (uno per CPU), ognuno con la propria VM e dispatch a match.
È l'analogo di `i_TTC_dispatch_goroutines` del porting Go; il
`Receiver` mpsc non è condivisibile e viaggia in un `Arc<Mutex<_>>`
(un lock ogni 1000 run: irrilevante).

## Transpiler

`src/bin/transpiler.rs` è il porting a passata singola di
`transpiler.c`: numeri → `v.push(...)`, primitive → `words::NOME(v)`.
Le primitive sono i file di `src/words/` (uno per word, come
`words/*.c`), elencati in ordine alfabetico. Non viene generato
l'equivalente di `transpiled_words_used.c`: il codice non
referenziato lo elimina il linker. Il generato è
`src/transpiled.rs` (committato, come `transpiled_main.c`).

## Build e benchmark

```sh
cd rust
cargo run --release --bin gen                                  # rigenera i dati srotolati
cargo run --release --bin transpiler -- ../test3_function_call.f   # rigenera il transpilato
cargo build --release                                          # tutti i binari in target/release
```

Dalla root del repo: `./benchmark_rust.sh` fa due sweep (release
default e `-C target-cpu=native`, l'analogo di `-march=native`).

Con `STACKVM_DEBUG=1` ogni variante stampa il risultato finale a fine
benchmark (deve essere `-4000`), fuori dal loop misurato.

## Risultati

Misure sulla stessa macchina (12 CPU), una run per cella, indicative.
Rust 1.92 release (LTO, codegen-units=1); C = gcc 15.1.0 -O2; Go =
1.25.4 default (dal branch go-lang, stessa macchina).

| Implementazione | Rust (s) | C -O2 (s) | Go (s) |
|---|---:|---:|---:|
| `i_TTC_dispatch_switch` | 4.49 | 2.02 | 2.45 |
| `i_TTC_dispatch_if` | 4.48 | 2.03 | 2.49 |
| `i_TTC_dispatch_function_table` | 6.03 | 2.03 | 3.56 |
| `i_DCT_dispatch_function_call` | 6.18 | 2.10 | 3.27 |
| `i_TTC_dispatch_channel` | 29.48 | — | 205.51 |
| `i_TTC_dispatch_threads`/`goroutines` | 0.60 | — | 0.33 |
| `native_function_init` | 0.00 | 0.00 | 0.30 |
| `native_function_call` | 0.00 | 1.58 | 3.74 |
| `native_baseline` | 0.00 | 0.00 | 0.20 |
| `native_transpiled_run400k` | 0.00 | 0.00 | 4.24 |

`-C target-cpu=native` non sposta nulla (switch 4.70: rumore).

## Analisi dei risultati

**L'interprete fedele è il più lento dei tre linguaggi, e il motivo
sono i bounds check sugli indici u16.** Non era scontato: in Go
costavano il 4%. Il micro-esperimento in `analysis/mb.rs` isola la
causa con tre varianti dello stesso exec:

| variante | tempo |
|---|---:|
| a: sp/pc `u16` + indicizzazione sicura (il porting fedele) | 5.10 |
| b: come a, con `get_unchecked` (niente check) | 2.66 |
| c: come a, con sp/pc `usize` | 3.06 |

Con sp `u16` il decremento in release è wrapping: dopo `sp -= 1`
LLVM non può dedurre `sp < 2048`, quindi ogni `stack[sp as usize]`
porta un confronto più un salto al panic path, e quel ramo — essendo
un side effect osservabile — ingessa anche lo scheduling della
catena di SUB. Basta passare a `usize` (variante c) perché più check
diventino dimostrabilmente superflui, e togliendoli del tutto
(variante b) si arriva a ridosso del C. Il porting resta fedele al
`uint16_t` del C: un interprete Rust scritto per andare veloce
userebbe `usize`, o un `get_unchecked` mirato e giustificato.

**function_table e DCT (6.0–6.2 s)**: chiamate indirette attraverso
`fn(&mut VM)`, mai inlinabili, più i bounds check di cui sopra dentro
ogni funzione; il doppio svantaggio le rende le varianti interpretate
più lente anche qui, come in Go e al contrario del C.

**Sui nativi LLVM gioca nella categoria di gcc, non in quella di
gc.** baseline, init, function_call e il transpilato Rust valgono
tutti 0.00: dead store elimination e constant folding totali,
attraverso 2000 chiamate srotolate e perfino i globali `static mut`
(function_call in C -O2 resta a 1.58 perché lì gcc non piega i
globali; LLVM con LTO sì). È l'immagine speculare del porting Go,
dove nulla veniva eliminato: nel confronto "nativo" Rust e C stanno
insieme, Go dietro. E vale il punto già fatto per il C: metà del
vantaggio sui micro-benchmark è esecuzione evitata, non più veloce.

**Channel: 29.5 s, ~37 ns per opcode — 7× meglio del Go (205 s), ma
il design è diverso ed è la vera lezione.** Qui niente spawn per run
(thread OS ≠ goroutine): un fetcher persistente, un canale opcode
per run, niente select per-opcode — la terminazione viaggia col drop
del receiver. In più la race che in Go andava trovata e chiusa a
mano col join è qui impossibile per costruzione: il borrow checker
non lascia condividere la VM, e la separazione PC-al-fetcher /
stack-all'esecutore è imposta dai tipi. Resta il verdetto di merito:
6.6× lo switch — una sincronizzazione per opcode non si ammortizza
nemmeno con mpsc al meglio.

**threads: 0.60 s, 7.5× lo switch Rust** — lo scaling sulle CPU è
pieno, identico al Go (7.4×); in assoluto resta dietro ai goroutines
Go (0.33) solo perché l'interprete sottostante è 2× più lento. Il
parallelismo tra run indipendenti paga in entrambi i linguaggi, in
proporzione identica.

**In sintesi**: C vince sull'interprete puro; Rust lo pareggerebbe
rinunciando ai check (o agli indici u16) e lo batte sui nativi grazie
a LLVM+LTO; Go tiene l'interprete vicino al C ma non elimina mai
lavoro. I channel non vanno nel ciclo di dispatch in nessuno dei due
linguaggi che li hanno; i thread/goroutine sulle run indipendenti
danno lo stesso identico fattore di scala. E una nota di metodo: lo
stesso benchmark, portato in tre linguaggi, misura tre cose diverse —
in C il dispatch, in Go l'inliner, in Rust i bounds check.
