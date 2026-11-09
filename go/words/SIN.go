package words

import (
	"math"

	"stackmachine/vm"
)

func SIN(v *vm.VM) {
	a := v.Pop()
	v.Push(math.Sin(a))
}
