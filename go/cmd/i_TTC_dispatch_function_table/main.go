// Porting Go di i_TTC_dispatch_function_table.c
// Token Threaded Code - dispatch implementato con tabella di funzioni.
package main

import (
	"fmt"
	"math"
	"os"

	"stackmachine/vm"
)

// variabili globali di supporto, come nel C (non thread-safe,
// commento originale: "forse converrebbe metterle nelle funzioni")
var a, b float64

var table [vm.OPCODE_COUNT]func(*vm.VM)

// ===== EXEC =====
func exec(v *vm.VM) float64 {
	for v.PC < v.CodeSize {
		op := v.Code[v.PC]
		v.PC++
		table[op](v)
	}
	return v.Stack[v.SP-1]
}

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
	v.Push(math.Sin(a))
}

func F_OP_COS(v *vm.VM) {
	a = v.Pop()
	v.Push(math.Cos(a))
}

func main() {
	table[vm.OP_NOP] = F_OP_NOP
	table[vm.OP_RETURN] = F_OP_RETURN
	table[vm.OP_ADD] = F_OP_ADD
	table[vm.OP_SUB] = F_OP_SUB
	table[vm.OP_MUL] = F_OP_MUL
	table[vm.OP_DIV] = F_OP_DIV
	table[vm.OP_SIN] = F_OP_SIN
	table[vm.OP_COS] = F_OP_COS

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
