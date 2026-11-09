pub fn MUL(v: &mut VM) {
    let a = v.pop();
    let b = v.pop();
    v.push(a * b);
}
