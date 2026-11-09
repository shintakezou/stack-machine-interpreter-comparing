package words

import "stackmachine/vm"

// Nop: in Go il nome esportato di una word minuscola ha l'iniziale
// maiuscola (il transpiler fa la stessa conversione).
func Nop(v *vm.VM) {
	_ = v
}
