// Porting Go di native_function_init.c
// misura la sola inizializzazione nativa della macchina a stack
// (VM azzerata + stack riempito, nessuna esecuzione).
package main

import (
	"fmt"
	"os"

	"stackmachine/vm"
)

// variabili globali di supporto, come nel C
var a, b float64

// ======= opcode ======= (presenti come nel C, qui inutilizzate)
func F_OP_RETURN(v *vm.VM) {
	v.PC = v.CodeSize
}

func F_OP_SUB(v *vm.VM) {
	a = v.Pop()
	b = v.Pop()
	v.Push(a - b)
}

func F_OP_MUL(v *vm.VM) {
	a = v.Pop()
	b = v.Pop()
	v.Push(a * b)
}

var _ = []func(*vm.VM){F_OP_RETURN, F_OP_SUB, F_OP_MUL}

func run() {
	var v vm.VM // VM vm = {0};
	loadInit(&v)
}

func main() {
	for times := 400000; times > 0; times-- {
		run()
	}

	if os.Getenv("STACKVM_DEBUG") != "" {
		fmt.Println("init only")
	}
}
