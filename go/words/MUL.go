package words

import "stackmachine/vm"

func MUL(v *vm.VM) {
	a := v.Pop()
	b := v.Pop()
	v.Push(a * b)
}
