// Porting Go di native_function_call.c
// esegue il codice nativo tramite le stesse funzioni di
// i_TTC_dispatch_function_table, ma senza interprete: le chiamate
// sono srotolate nel sorgente generato (test3_call_data.go).
//
// Nota: come nel C, F_OP_SIN/F_OP_COS qui usano -(a) al posto di
// sin/cos (così era nel sorgente originale).
package main

import (
	"fmt"
	"os"

	"stackmachine/vm"
)

// variabili globali di supporto, come nel C
var a, b float64

// ======= opcode =======
func F_OP_NOP(v *vm.VM) {
	fmt.Printf("pc=%d: OP_NOP\n", v.PC-1)
}

func F_OP_RETURN(v *vm.VM) {
	v.PC = v.CodeSize // termina VM
}

func F_OP_ADD(v *vm.VM) {
	a = v.Pop()
	b = v.Pop()
	v.Push(a + b)
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

func F_OP_DIV(v *vm.VM) {
	a = v.Pop()
	b = v.Pop()
	v.Push(a / b)
}

func F_OP_SIN(v *vm.VM) {
	a = v.Pop()
	v.Push(-(a))
}

func F_OP_COS(v *vm.VM) {
	a = v.Pop()
	v.Push(-(a))
}

var _ = []func(*vm.VM){F_OP_NOP, F_OP_ADD, F_OP_DIV, F_OP_SIN, F_OP_COS}

func run() float64 {
	var v vm.VM // VM vm = {0};
	loadAndCall(&v)
	return v.Stack[0]
}

func main() {
	var r float64
	for times := 400000; times > 0; times-- {
		r = run()
	}

	if os.Getenv("STACKVM_DEBUG") != "" {
		fmt.Println("R:", r)
	}
}
