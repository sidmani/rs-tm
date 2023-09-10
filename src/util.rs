pub fn print_state(state: u64, tape: &Vec<u64>, loc: usize) {
    let mut text = format!("\x1b[92m[{}]\x1b[0m", state);
    for i in 0..tape.len() {
        if i == loc {
            text = format!("{} \x1b[91m\x1b[1m{}\x1b[0m", text, tape[i]);
        } else {
            text = format!("{} {}", text, tape[i]);
        }
    }
    println!("{}", text);
}
