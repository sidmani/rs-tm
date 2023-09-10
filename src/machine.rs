use crate::util::print_state;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
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
            dir,
        };
    }
}

fn build_rulemap(rules: &Vec<Rule>) -> HashMap<(u64, u64), &Rule> {
    let mut rulemap = HashMap::new();
    for rule in rules {
        rulemap.insert((rule.init_state, rule.init_char), rule);
    }

    return rulemap;
}

fn apply_rule(
    state: u64,
    tape_l: &mut Vec<u64>,
    tape_r: &mut Vec<u64>,
    loc: (usize, bool),
    rulemap: &HashMap<(u64, u64), &Rule>,
) -> Option<(u64, (usize, bool))> {
    let curr_char: u64;
    if loc.1 {
        curr_char = tape_r[loc.0];
    } else {
        curr_char = tape_l[loc.0];
    }

    if let Some(rule) = rulemap.get(&(state, curr_char)) {
        let new_pos: usize;
        let new_is_right_tape: bool;
        let new_state = rule.final_state;

        if loc.1 {
            tape_r[loc.0] = rule.final_char;
        } else {
            tape_l[loc.0] = rule.final_char;
        }

        match &rule.dir {
            Direction::Left => {
                if loc.1 {
                    if loc.0 == 0 {
                        new_pos = 0;
                        new_is_right_tape = false;
                        if tape_l.len() == 0 {
                            tape_l.push(0);
                        }
                    } else {
                        new_pos = loc.0 - 1;
                        new_is_right_tape = true;
                    }
                } else {
                    new_pos = loc.0 + 1;
                    new_is_right_tape = false;
                    if new_pos >= tape_l.len() {
                        tape_l.push(0);
                    }
                }
            }
            Direction::Right => {
                if loc.1 {
                    new_pos = loc.0 + 1;
                    new_is_right_tape = true;
                    if new_pos >= tape_r.len() {
                        tape_r.push(0);
                    }
                } else {
                    if loc.0 == 0 {
                        new_pos = 0;
                        new_is_right_tape = true;
                    } else {
                        new_pos = loc.0 - 1;
                        new_is_right_tape = false;
                    }
                }
            }
            Direction::None => {
                new_pos = loc.0 ;
                new_is_right_tape = loc.1;
            },
        }

        return Some((new_state, (new_pos, new_is_right_tape)));
    }

    return None;
}


pub fn run(rules: &Vec<Rule>, init_tape: Option<Vec<u64>>, print_tape: bool) {
    let mut state = 0;
    let mut tape_l: Vec<u64> = Vec::new();
    let mut tape_r: Vec<u64>;
    let mut loc: (usize, bool) = (0, true);

    match init_tape {
        Some(t) => {
            tape_r = t;
        }
        None => {
            tape_r = Vec::new();
            tape_r.push(0);
        }
    }

    let rulemap = build_rulemap(rules);

    if print_tape {
        let joined_tape = [tape_l.iter().copied().rev().collect(), tape_r.clone()].concat();
        let mut pos = tape_l.len();
        if loc.1 {
            pos += loc.0;
        } else {
            pos -= loc.0 + 1;
        }
        print_state(state, &joined_tape, pos);
    }

    loop {
        match apply_rule(state, &mut tape_l, &mut tape_r, loc, &rulemap) {
            None => break,
            Some((new_state, new_loc)) => {
                state = new_state;
                loc = new_loc;
            }
        }

        if print_tape {
            let joined_tape = [tape_l.iter().copied().rev().collect(), tape_r.clone()].concat();
            let mut pos = tape_l.len();
            if loc.1 {
                pos += loc.0;
            } else {
                pos -= loc.0 + 1;
            }
            print_state(state, &joined_tape, pos);
        }
    }
}
