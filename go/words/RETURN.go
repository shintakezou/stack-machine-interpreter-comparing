package words

import "stackmachine/vm"

func RETURN(v *vm.VM) {
	v.PC = v.CodeSize // termina VM
}
