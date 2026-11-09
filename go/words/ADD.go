package words

import "stackmachine/vm"

func ADD(v *vm.VM) {
	a := v.Pop()
	b := v.Pop()
	v.Push(a + b)
}
