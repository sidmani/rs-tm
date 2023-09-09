use std::collections::{HashMap, HashSet};

struct Rule {
    init_state: u64,
    init_char: u64,
    final_state: u64,
    final_char: u64,
    dir: i8,
}

struct Machine {
    rules: Vec<Rule>,
    init_state: u64,
    init_loc: usize,
    halting_states: HashSet<u64>,
}

type Rulemap<'a> = HashMap<(u64, u64), &'a Rule>;

fn main() {
    println!("Hello, world!");
    // A, B, 1, 0, X, Y, Z
    // 0, 1, 2, 3, 4, 5, 6

    let rules = vec![
        Rule {
            init_state: 0,
            init_char: 0,
            final_state: 0,
            final_char: 0,
            dir: 0
        }
    ];
}

fn build_rulemap(rules: &Vec<Rule>) -> Rulemap {
    let mut rulemap = HashMap::new();
    for rule in rules {
        rulemap.insert((rule.init_state, rule.init_char), rule);
    }

    rulemap
}

fn apply_rule(state: u64, tape: &mut Vec<u64>, loc: usize, rulemap: &Rulemap) -> (u64, usize) {
    let (mut new_state, mut new_loc): (u64, usize);
    match rulemap.get(&(state, tape[loc])) {
        Some(rule) => {
            new_state = rule.final_state;
            tape[loc] = rule.final_char;

            // get the new location
            // negative indices are odd numbers (-1 = 1, -2 = 3...), positives are even
            // zero is zero
            if rule.dir == -1 {
                if loc % 2 == 1 {
                    // we're in negative indices; move right by two
                    new_loc = loc + 2;
                } else if loc == 0 {
                    new_loc = 1;
                } else {
                    // in positive (even) indices; move left by two
                    new_loc = loc - 2;
                }
            } else if rule.dir == 1 {
                if loc % 2 == 0 {
                    // zero or positives; move right by two
                    new_loc = loc + 2;
                } else if loc == 1 {
                    // index 1 is tape -1, so go to index 2 (tape 1)
                    new_loc = 2
                } else {
                    // in negatives (3 or greater); move left by two
                    new_loc = loc - 2
                }
            } else {
                new_loc = loc;
            }

            for _ in tape.len()..loc {
                tape.push(0);
            }
        }
        None => panic!("no rule applicable"),
    }

    (new_state, new_loc)
}

fn run(machine: Machine) {
    let rulemap = build_rulemap(&machine.rules);

    let mut state = machine.init_state;
    let mut loc = machine.init_loc;
    let mut tape: Vec<u64> = Vec::new();

    loop {
        if machine.halting_states.contains(&state) {
            break;
        }

        (state, loc) = apply_rule(state, &mut tape, loc, &rulemap);
    }
}
