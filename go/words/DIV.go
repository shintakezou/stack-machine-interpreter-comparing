package words

import "stackmachine/vm"

func DIV(v *vm.VM) {
	a := v.Pop()
	b := v.Pop()
	v.Push(a / b)
}
