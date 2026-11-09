// i_TTC_dispatch_channel (solo Go, nessun equivalente C)
// Token Threaded Code - la VM è spezzata in due stadi collegati da un
// channel, come una pipeline hardware fetch → execute:
//
//   - una goroutine "fetch" legge il bytecode, avanza il PC e spedisce
//     gli opcode su un channel bufferizzato;
//   - l'esecutore riceve gli opcode dal channel e li applica allo stack.
//
// Non c'è parallelismo utile dentro una singola run (ogni istruzione
// dipende dallo stato lasciato dalla precedente): il channel compra
// solo il disaccoppiamento fetch/execute, al prezzo di una
// sincronizzazione per opcode. Serve a misurare proprio quel prezzo.
//
// RETURN termina l'esecutore; il canale done ferma il fetcher, che
// altrimenti resterebbe bloccato sulla send (nel C equivale a
// vm->pc = vm->code_size).
package main

import (
	"fmt"
	"math"
	"os"

	"stackmachine/vm"
)

const chanBuf = 128

// ===== EXEC =====
func exec(v *vm.VM) float64 {
	ops := make(chan vm.Opcode, chanBuf)
	done := make(chan struct{})
	fetchExited := make(chan struct{})

	// stadio fetch: unico proprietario di PC
	go func() {
		defer close(fetchExited)
		defer close(ops)
		for v.PC < v.CodeSize {
			op := v.Code[v.PC]
			v.PC++
			select {
			case ops <- op:
			case <-done:
				return
			}
		}
	}()

	// stadio execute: unico proprietario di SP/Stack
	var a, b float64
	for op := range ops {
		switch op {
		case vm.OP_NOP:
			fmt.Printf("pc: OP_NOP\n")

		case vm.OP_RETURN:
			// ferma il fetcher e ASPETTA che sia uscito: senza il
			// join, il fetcher di questa run può sopravvivere fino
			// alla run successiva e incrementare PC in concorrenza
			// con il nuovo fetcher, corrompendo lo stream di opcode
			close(done)
			<-fetchExited
			return v.Stack[0]

		case vm.OP_ADD:
			a = v.Pop()
			b = v.Pop()
			v.Push(a + b)

		case vm.OP_SUB:
			a = v.Pop()
			b = v.Pop()
			v.Push(a - b)

		case vm.OP_MUL:
			a = v.Pop()
			b = v.Pop()
			v.Push(a * b)

		case vm.OP_DIV:
			a = v.Pop()
			b = v.Pop()
			v.Push(a / b)

		case vm.OP_SIN:
			a = v.Pop()
			v.Push(math.Sin(a))

		case vm.OP_COS:
			a = v.Pop()
			v.Push(math.Cos(a))
		}
	}
	close(done)
	return 0
}

func main() {
	var v vm.VM
	var r float64

	for times := 400000; times > 0; times-- {
		vm.LoadTest3(&v)
		r = exec(&v)
	}

	if os.Getenv("STACKVM_DEBUG") != "" {
		fmt.Println("R:", r)
	}
}
