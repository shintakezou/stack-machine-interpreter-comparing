// Porting Go di i_TTC_dispatch_if.c
// Token Threaded Code - dispatch implementato con if in cascata.
package main

import (
	"fmt"
	"math"
	"os"

	"stackmachine/vm"
)

// ===== EXEC =====
func exec(v *vm.VM) float64 {
	var a, b float64

	for v.PC < v.CodeSize {
		op := v.Code[v.PC]
		v.PC++

		if op == vm.OP_NOP {
			fmt.Printf("pc=%d: OP_NOP\n", v.PC-1)
		}
		if op == vm.OP_RETURN {
			return v.Stack[0]
		}
		if op == vm.OP_ADD {
			a = v.Pop()
			b = v.Pop()
			v.Push(a + b)
		}
		if op == vm.OP_SUB {
			a = v.Pop()
			b = v.Pop()
			v.Push(a - b)
		}
		if op == vm.OP_MUL {
			a = v.Pop()
			b = v.Pop()
			v.Push(a * b)
		}
		if op == vm.OP_DIV {
			a = v.Pop()
			b = v.Pop()
			v.Push(a / b)
		}
		if op == vm.OP_SIN {
			a = v.Pop()
			v.Push(math.Sin(a))
		}
		if op == vm.OP_COS {
			a = v.Pop()
			v.Push(math.Cos(a))
		}
	}
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
