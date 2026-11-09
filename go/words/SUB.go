package words

import "stackmachine/vm"

func SUB(v *vm.VM) {
	a := v.Pop()
	b := v.Pop()
	v.Push(a - b)
}
