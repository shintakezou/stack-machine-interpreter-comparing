// Package vm è il porting Go di common.h + Opcode_enum.c:
// la struttura dati condivisa dalle implementazioni TTC e native.
//
// Come in C, Stack e Code sono array a dimensione fissa dentro la
// struct (non slice), così l'inizializzazione `var v VM` equivale a
// `VM vm = {0}` e l'accesso non passa per un'indirezione.
//
// Opcode è uint8: corrisponde alla variante C compilata con
// -fshort-enums (l'enum C di default è un int a 4 byte).
package vm

const (
	StackMax  = 2 * 1024
	OpcodeMax = 2 * 1024
)

type Opcode uint8

const (
	OP_NOP Opcode = iota
	OP_RETURN
	OP_ADD
	OP_SUB
	OP_MUL
	OP_DIV
	OP_SIN
	OP_COS

	OPCODE_COUNT
)

type VM struct {
	SP       uint16
	PC       uint16
	CodeSize uint16
	Stack    [StackMax]float64
	Code     [OpcodeMax]Opcode
}

// Pop e Push: equivalenti di stack_pop/stack_push (static inline in C;
// il compilatore Go li inlinea perché sono foglie banali).

func (v *VM) Pop() float64 {
	v.SP--
	return v.Stack[v.SP]
}

func (v *VM) Push(x float64) {
	v.Stack[v.SP] = x
	v.SP++
}
