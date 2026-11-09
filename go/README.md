# Porting Go della stack machine

Porting in Go del benchmark C, per confrontare le prestazioni dei due
linguaggi sugli stessi modelli di interpretazione — più due varianti
nuove che usano channel e goroutine, dove hanno senso.

## Corrispondenza con la versione C

| C | Go |
|---|---|
| `common.h` + `Opcode_enum.c` | package `vm` |
| `test.php` → `test3.c`, `test3_function_*.c` | `go run ./gen` → `vm/test3_data.go`, `cmd/*/test3_*_data.go` |
| `i_TTC_dispatch_switch.c` | `cmd/i_TTC_dispatch_switch` |
| `i_TTC_dispatch_if.c` | `cmd/i_TTC_dispatch_if` |
| `i_TTC_dispatch_function_table.c` | `cmd/i_TTC_dispatch_function_table` |
| `i_DCT_dispatch_function_call.c` | `cmd/i_DCT_dispatch_function_call` |
| `i_TTC_dispatch_computed_goto.c` | — (vedi sotto) |
| `i_TTC_dispatch_function_table_shared.c` + `vm_shared.c` | — (vedi sotto) |
| `native_*.c` | `cmd/native_*` |
| `transpiler.c` + `words/*.c` | `cmd/transpiler` + package `words` |
| `transpiled_main.c` + `native_transpiled_run400k.c` | package `transpiled` (generato) + `cmd/native_transpiled_run400k` |
| — | `cmd/i_TTC_dispatch_channel` (nuova) |
| — | `cmd/i_TTC_dispatch_goroutines` (nuova) |

Ogni `main.go` ricalca il file C corrispondente: stessi nomi di
funzione, stesso ordine di pop, stessi dettagli (per esempio
`native_function_call` usa `-(a)` per SIN/COS, com'è nel C; la
variante function_table tiene `a, b` globali, non thread-safe,
com'è nel C).

## Scelte di porting

- **VM**: struct identica alla C, con array a dimensione fissa
  (`[2048]float64`, `[2048]Opcode`) dentro la struct; `var v VM`
  equivale a `VM vm = {0}`. `Pop`/`Push` sono metodi foglia che il
  compilatore inlinea (come le `static inline` C).
- **Opcode = uint8**: corrisponde al C compilato con `-fshort-enums`
  (l'enum C di default è un int da 4 byte).
- **computed_goto**: non portato. Go non ha label-as-values né `goto`
  calcolato; l'analogo più vicino è la function table.
- **function_table_shared / hotreload**: non portato. Il
  corrispettivo Go sarebbe il package `plugin` (buildmode=plugin);
  possibile estensione futura.
- **DCT**: in C lo stesso `common.h` viene ricompilato con
  `Opcode = puntatore a funzione`; in Go la variante definisce la
  propria VM locale con `type Opcode func(*VM)`.
- I file di dati di test sono **srotolati** (2000+ statement) come i
  file generati C: il costo di inizializzazione fa parte di ciò che
  il benchmark misura. `go run ./gen` li rigenera (è il `test.php`
  del porting).

## Le varianti con channel e goroutine

**`i_TTC_dispatch_channel`** — la VM spezzata in una pipeline
fetch → execute: una goroutine legge il bytecode e spedisce gli
opcode su un channel bufferizzato, l'esecutore li applica allo stack.
Dentro una singola run non c'è parallelismo estraibile (ogni
istruzione dipende dalla precedente), quindi questa variante misura
esattamente il costo della sincronizzazione per-opcode. Nota di
implementazione: alla RETURN l'esecutore deve fare *join* del fetcher
prima di ritornare, altrimenti il fetcher superstite corre contro
quello della run successiva su `PC` (race trovata col primo run del
benchmark).

**`i_TTC_dispatch_goroutines`** — il parallelismo dove è sensato: le
400k run del benchmark sono indipendenti, quindi un channel
distribuisce blocchi di iterazioni a un pool di goroutine (una per
CPU), ognuna con la propria VM e dispatch a switch. Confrontata con
`i_TTC_dispatch_switch` dice quanto compra il parallelismo di Go a
parità di interprete.

## Transpiler

`cmd/transpiler` è il porting a passata singola di `transpiler.c`:
numeri → `v.Push(...)`, primitive → `words.NOME(v)`. Le primitive
sono i file di `go/words/` (come `words/*.c`), caricati in ordine
alfabetico (`os.ReadDir` è deterministico di suo, al contrario di
`readdir(3)`). Non viene generato l'equivalente di
`transpiled_words_used.c`: il linker Go elimina da solo il codice non
referenziato. Il generato è `go/transpiled/transpiled_main.go`
(committato, come `transpiled_main.c`).

## Build e benchmark

```sh
make go-all        # compila tutto in build/go/ (transpilato incluso)
make go-gen        # rigenera i file di dati srotolati
make benchmark-go  # sweep completa (./benchmark_go.sh)
```

Go non ha i livelli `-O` di gcc: il benchmark fa due sweep, con
`GOAMD64` di default e `GOAMD64=v3` (l'analogo di `-march=native`).

Con `STACKVM_DEBUG=1` ogni variante stampa il risultato finale a fine
benchmark (deve essere `-4000`, cioè `(2-...-1)*2` — il check sta
fuori dal loop e non costa nulla).

## Primi numeri

Misure sulla stessa macchina (12 CPU): Go 1.25.4 con GOAMD64 default,
C con gcc 15.1.0 a -O2. Una sola run per cella, indicativa (per la
sweep completa: `make benchmark-go` e `./benchmark.sh`).

| Implementazione | Go (s) | C -O2 (s) |
|---|---:|---:|
| `i_TTC_dispatch_switch` | 2.45 | 2.02 |
| `i_TTC_dispatch_if` | 2.49 | 2.03 |
| `i_TTC_dispatch_function_table` | 3.56 | 2.03 |
| `i_DCT_dispatch_function_call` | 3.27 | 2.10 |
| `i_TTC_dispatch_channel` | 205.51 | — |
| `i_TTC_dispatch_goroutines` | 0.33 | — |
| `native_function_init` | 0.30 | 0.00 |
| `native_function_call` | 3.74 | 1.58 |
| `native_baseline` | 0.20 | 0.00 |
| `native_transpiled_run400k` | 4.24 | 0.00 |

## Analisi dei risultati

**Dispatch a switch/if: Go a ~20% dal C.** È il terreno dove gc regge
meglio: `Pop`/`Push` vengono inlinate (verificato con `-gcflags=-m`)
e il loop compila in codice molto simile a quello di gcc. I bounds
check sugli array, il sospetto ovvio, pesano poco: ricompilando con
`-gcflags=all=-B` (controlli disattivati) lo switch passa da 2.45 a
2.34 s, ~4%. Il resto del divario è qualità del codice del loop
(allocazione dei registri, forma dei salti), non le verifiche di
sicurezza. Che if in cascata e switch si equivalgano, qui come in C,
dice anche un'altra cosa: con 8 opcode e un programma che è SUB×2000,
il branch predictor copre qualunque meccanismo di dispatch.

**Le varianti a chiamata pagano 1.6–1.7×, e il motivo in Go è
diverso.** In C function_table è veloce quanto switch; in Go
function_table (3.56) e DCT (3.27) staccano. Due cause: gc non
inlinea mai una chiamata attraverso un valore-funzione, e ogni
chiamata Go porta il prologo di controllo crescita dello stack della
goroutine, mentre in C la chiamata indiretta è nuda. Curioso il
sorpasso interno: in Go DCT batte function_table (in C è il
contrario) — tolta la tabella sparisce un load indiretto, e quello
che resta è dominato dal costo di chiamata comunque.

**native_function_call (3.74 vs 1.58 C) smaschera un limite preciso
dell'inliner di gc.** Qui le chiamate sono dirette e srotolate:
`F_OP_SUB` è inlinabile (costo 44, sotto il budget di 80), ma
`loadAndCall` — 4000 righe generate, costo 132092 — viene
classificata "big" e per le funzioni grandi gc *riduce* il budget
degli inlinee: risultato, 2000 chiamate vere (`-gcflags='-m -m'`:
"function considered 'big'; reducing max cost of inlinees"). gcc
nella stessa situazione inlinea tutto e tiene lo stack nei registri.
Non sono i bounds check (senza: 3.65 s) e non è colpa delle globali
`a, b`: è il codice generato-gigante che a gc non piace.

**gcc elimina il lavoro, gc lo esegue.** `native_baseline` e
`native_function_init` in C a -O2 valgono 0.00: dead store
elimination — il risultato non viene letto, gcc cancella l'intera
elaborazione. Su questa macchina gcc 15 azzera perfino il transpilato
(0.00 s!): il programma è interamente constant-foldable. gc non fa
nulla di tutto questo: baseline 0.20 (il loop gira davvero), init
0.30 (azzeramento VM da ~34 KB + 2000 store, ~750 ns a run), e il
transpilato Go è anzi il più lento (4.24) perché somma init
srotolato, chiamate non inlinate (stesso effetto "big function" di
cui sopra) e zeroing. Morale: nel confronto C-vs-Go su questi micro
benchmark, metà del vantaggio C non è "esecuzione più veloce" ma
"esecuzione evitata".

**Channel nel ciclo interno: due ordini di grandezza.** 205.5 s per
800M di opcode = ~256 ns l'uno, contro ~3 ns del dispatch a switch:
una coppia send/recv su channel bufferizzato più una select a due vie
costa ~85× l'intero lavoro che sincronizza. La pipeline
fetch→execute, che in hardware è gratis, in software ha senso solo se
lo stadio fa abbastanza lavoro da ammortizzare la sincronizzazione —
un opcode non lo è mai. I channel vanno ai confini di unità di lavoro
grosse, non dentro il dispatch.

**Goroutine tra run indipendenti: 7.4×.** Stesso interprete dello
switch, 0.33 s contro 2.45: scaling quasi pieno sui core fisici (12
thread hw), perché qui il channel trasporta un messaggio ogni 1000
run — granularità giusta — e ogni worker ha la sua VM (niente
condivisione, niente false sharing). È l'unica variante Go che batte
*qualunque* tempo C a parità di macchina: il parallelismo è il dove
Go restituisce ciò che i channel tolgono nel ciclo interno.

**In sintesi**: la gerarchia C (table ≈ switch > call) in Go si
ribalta: per un interprete in Go il modello giusto è il TTC con
switch, le chiamate indirette costano, i channel nel dispatch sono
proibitivi, e il guadagno vero sta nel parallelizzare le esecuzioni
indipendenti.
