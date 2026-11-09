pub fn SIN(v: &mut VM) {
    let a = v.pop();
    v.push(a.sin());
}
