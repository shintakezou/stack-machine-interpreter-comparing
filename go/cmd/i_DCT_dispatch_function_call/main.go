// Porting Go di i_DCT_dispatch_function_call.c
// Direct Call Threading: il bytecode contiene direttamente i valori
// funzione da eseguire, l'interprete non ha tabella di conversione.
//
// Come in C (dove common.h viene ricompilato con Opcode = puntatore a
// funzione), questa variante definisce la propria VM locale: il tipo
// del campo Code è diverso da quello del package vm.
package main

import (
	"fmt"
	"math"
	"os"
)

const (
	StackMax  = 2 * 1024
	OpcodeMax = 2 * 1024
)

// Opcode è un valore funzione: l'equivalente di
// typedef void (*Opcode)(VM *vm);
type Opcode func(*VM)

type VM struct {
	SP       uint16
	PC       uint16
	CodeSize uint16
	Stack    [StackMax]float64
	Code     [OpcodeMax]Opcode
}

func (v *VM) Pop() float64 {
	v.SP--
	return v.Stack[v.SP]
}

func (v *VM) Push(x float64) {
	v.Stack[v.SP] = x
	v.SP++
}

// ===== EXEC =====
func exec(v *VM) {
	for v.PC < v.CodeSize {
		op := v.Code[v.PC]
		v.PC++
		op(v)
	}
}

// ======= opcode =======
// variabili locali come nel C (questa variante non usa globali)

func opNOP(v *VM) {
	fmt.Printf("\npc=%d: OP_NOP-S\n", v.PC-1)
}

func opRETURN(v *VM) {
	v.PC = v.CodeSize // termina VM
}

func opADD(v *VM) {
	b := v.Pop()
	a := v.Pop()
	v.Push(a + b)
}

func opSUB(v *VM) {
	a := v.Pop()
	b := v.Pop()
	v.Push(a - b)
}

func opMUL(v *VM) {
	a := v.Pop()
	b := v.Pop()
	v.Push(a * b)
}

func opDIV(v *VM) {
	b := v.Pop()
	a := v.Pop()
	v.Push(a / b)
}

func opSIN(v *VM) {
	a := v.Pop()
	v.Push(math.Sin(a))
}

func opCOS(v *VM) {
	a := v.Pop()
	v.Push(math.Cos(a))
}

var _ = []Opcode{opNOP, opADD, opDIV, opSIN, opCOS} // referenziate come in C anche se il test non le usa

func main() {
	var v VM

	for times := 400000; times > 0; times-- {
		loadTest3(&v)
		exec(&v)
	}

	if os.Getenv("STACKVM_DEBUG") != "" {
		fmt.Println("R:", v.Stack[v.SP-1])
	}
}
