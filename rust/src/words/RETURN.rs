pub fn RETURN(v: &mut VM) {
    v.pc = v.code_size; // termina VM
}
