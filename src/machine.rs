use std::collections::{HashMap, HashSet};

pub enum Direction {
    Left,
    Right,
    None,
}

pub struct Rule {
    pub init_state: u64,
    pub init_char: u64,
    pub final_state: u64,
    pub final_char: u64,
    pub dir: Direction,
}

impl Rule {
    pub fn from(s0: u64, c0: u64, s1: u64, c1: u64, dir: Direction) -> Rule {
        return Rule {
            init_state: s0,
            init_char: c0,
            final_state: s1,
            final_char: c1,
            dir: dir,
        };
    }
}

pub struct Machine {
    pub rules: Vec<Rule>,
    pub init_state: u64,
    pub init_loc: usize,
    pub blank_char: u64,
    pub halting_states: HashSet<u64>,
}

type Rulemap<'a> = HashMap<(u64, u64), &'a Rule>;

fn build_rulemap(rules: &Vec<Rule>) -> Rulemap {
    let mut rulemap = HashMap::new();
    for rule in rules {
        rulemap.insert((rule.init_state, rule.init_char), rule);
    }

    rulemap
}

fn apply_rule(
    state: u64,
    tape: &mut Vec<u64>,
    loc: usize,
    rulemap: &Rulemap,
    blank_char: u64,
) -> (u64, usize) {
    let (new_state, new_loc): (u64, usize);
    match rulemap.get(&(state, tape[loc])) {
        Some(rule) => {
            new_state = rule.final_state;
            tape[loc] = rule.final_char;

            // get the new location
            // negative indices are odd numbers (-1 = 1, -2 = 3...), positives are even
            // zero is zero
            match &rule.dir {
                Direction::Left => {
                    if loc % 2 == 1 {
                        // we're in negative indices; move right by two
                        new_loc = loc + 2;
                    } else if loc == 0 {
                        new_loc = 1;
                    } else {
                        // in positive (even) indices; move left by two
                        new_loc = loc - 2;
                    }
                }
                Direction::Right => {
                    if loc % 2 == 0 {
                        // zero or positives; move right by two
                        new_loc = loc + 2;
                    } else if loc == 1 {
                        // index 1 is tape -1, so go to index 0 (tape 0)
                        new_loc = 0;
                    } else {
                        // in negatives (3 or greater); move left by two
                        new_loc = loc - 2;
                    }
                }
                Direction::None => {
                    new_loc = loc;
                }
            }

            for _ in tape.len() - 1..new_loc {
                tape.push(blank_char);
            }
        }
        None => panic!("no rule applicable"),
    }

    (new_state, new_loc)
}

pub fn fold_idx(pos: usize, len: usize) -> usize {
    let center = len / 2;
    if pos < center {
        return (center - pos) * 2 - 1;
    } else {
        return (pos - center) * 2;
    }
}

pub fn unfold_idx(pos: usize, len: usize) -> usize {
    let center = len / 2;
    if pos % 2 == 0 {
        return pos / 2 + center;
    }
    return center - (pos + 1) / 2;
}

pub fn fold(tape: &Vec<u64>) -> Vec<u64> {
    let mut new_tape: Vec<u64> = Vec::new();
    for i in 0..tape.len() {
        new_tape.push(tape[unfold_idx(i, tape.len())])
    }

    new_tape
}

pub fn unfold(tape: &Vec<u64>) -> Vec<u64> {
    let mut new_tape: Vec<u64> = Vec::new();
    for i in 0..tape.len() {
        new_tape.push(tape[fold_idx(i, tape.len())]);
    }

    new_tape
}

pub fn print_state(state: u64, tape: &Vec<u64>, loc: usize) {
    let mut text = format!("\x1b[92m[{}]\x1b[0m", state);
    for i in 0..tape.len() {
        if i == loc {
            // string += f" \033[91m\033[1m{c}\033[0m"
            text = format!("{} \x1b[91m\x1b[1m{}\x1b[0m", text, tape[i]);
        } else {
            text = format!("{} {}", text, tape[i]);
        }
    }
    println!("{}", text);
}

pub fn run(machine: Machine, init_tape: Option<Vec<u64>>) {
    let rulemap = build_rulemap(&machine.rules);

    let mut state = machine.init_state;
    let mut tape: Vec<u64>;
    let mut loc: usize;

    match init_tape {
        Some(t) => {
            tape = fold(&t);
            loc = fold_idx(machine.init_loc, t.len());
        }
        None => {
            tape = Vec::new();
            // will fail if loc not zero
            loc = machine.init_loc
        }
    }

    // println!("[{}] {:?}", state, unfold(&tape));
    print_state(state, &unfold(&tape), unfold_idx(loc, tape.len()));
    loop {
        if machine.halting_states.contains(&state) {
            break;
        }

        (state, loc) = apply_rule(state, &mut tape, loc, &rulemap, machine.blank_char);
        print_state(state, &unfold(&tape), unfold_idx(loc, tape.len()));
    }
}
