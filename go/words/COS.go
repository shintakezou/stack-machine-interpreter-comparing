package words

import (
	"math"

	"stackmachine/vm"
)

func COS(v *vm.VM) {
	a := v.Pop()
	v.Push(math.Cos(a))
}
