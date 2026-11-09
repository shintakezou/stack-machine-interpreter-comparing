pub fn COS(v: &mut VM) {
    let a = v.pop();
    v.push(a.cos());
}
