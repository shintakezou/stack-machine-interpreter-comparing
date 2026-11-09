// i_TTC_dispatch_goroutines (solo Go, nessun equivalente C)
// Token Threaded Code - qui il parallelismo è messo dove è davvero
// sensato: non dentro la VM (una run è intrinsecamente sequenziale),
// ma tra le 400k run del benchmark, che sono indipendenti.
//
// Un channel distribuisce blocchi di iterazioni a un pool di
// goroutine (una per CPU); ogni worker ha la propria VM e usa il
// dispatch a switch, identico a i_TTC_dispatch_switch.
// È il confronto "quanto compra il parallelismo di Go" a parità di
// interprete.
package main

import (
	"fmt"
	"os"
	"runtime"
	"sync"

	"stackmachine/vm"
)

const (
	times = 400000
	chunk = 1000 // iterazioni per messaggio sul channel
)

// ===== EXEC ===== (identica a i_TTC_dispatch_switch, thread-safe:
// nessuna variabile globale)
func exec(v *vm.VM) float64 {
	var a, b float64

	for v.PC < v.CodeSize {
		op := v.Code[v.PC]
		v.PC++

		switch op {
		case vm.OP_RETURN:
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
		}
	}
	return 0
}

func main() {
	workers := runtime.GOMAXPROCS(0)
	jobs := make(chan int, workers)
	var wg sync.WaitGroup

	for w := 0; w < workers; w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			var v vm.VM // una VM per goroutine
			for n := range jobs {
				for i := 0; i < n; i++ {
					vm.LoadTest3(&v)
					exec(&v)
				}
			}
		}()
	}

	for sent := 0; sent < times; sent += chunk {
		n := chunk
		if times-sent < n {
			n = times - sent
		}
		jobs <- n
	}
	close(jobs)
	wg.Wait()

	if os.Getenv("STACKVM_DEBUG") != "" {
		var v vm.VM
		vm.LoadTest3(&v)
		fmt.Println("R:", exec(&v), "workers:", workers)
	}
}
