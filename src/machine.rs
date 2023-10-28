use crate::util::print_state;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone)]
pub enum Direction {
    Left,
    Right,
    None,
}

pub fn build_rule_arr(
    rules: &Vec<(u64, u64, u64, u64, Direction)>,
) -> Vec<Option<(u64, u64, Direction)>> {
    let mut rulemap = HashMap::new();
    let mut max_state = 0;
    let mut max_sym = 0;
    for rule in rules {
        max_state = max_state.max(rule.0);
        max_sym = max_sym.max(rule.1);
        rulemap.insert((rule.0, rule.1), rule);
    }

    let mut result = Vec::new();
    for i in 0..max_state + 1 {
        for j in 0..max_sym + 1 {
            result.push(if let Some(rule) = rulemap.get(&(i, j)) {
                Some((rule.2, rule.3, rule.4.clone()))
            } else {
                None
            });
        }
    }

    result
}

fn apply_rule(
    state: u64,
    tape_l: &mut Vec<u64>,
    tape_r: &mut Vec<u64>,
    loc: (usize, bool),
    rule_arr: &Vec<Option<(u64, u64, Direction)>>,
    num_symbols: u64,
) -> Option<(u64, (usize, bool))> {
    let curr_char = if loc.1 { tape_r[loc.0] } else { tape_l[loc.0] };
    let rule_idx = (state * num_symbols + curr_char) as usize;
    if rule_idx >= rule_arr.len() {
        return None;
    }

    if let Some(rule) = &rule_arr[rule_idx] {
        let new_pos: usize;
        let new_is_right_tape: bool;
        let new_state = rule.0;

        if loc.1 {
            tape_r[loc.0] = rule.1;
        } else {
            tape_l[loc.0] = rule.1;
        }
        match (&rule.2, loc.0, loc.1) {
            (Direction::Left, 0, true) => {
                // moving left from edge of right tape
                new_pos = 0;
                new_is_right_tape = false;
                if tape_l.len() == 0 {
                    tape_l.push(0);
                }
            }
            (Direction::Left, k, true) => {
                // moving left on right tape
                new_pos = k - 1;
                new_is_right_tape = true;
            }
            (Direction::Left, k, false) => {
                new_pos = k + 1;
                new_is_right_tape = false;
                if tape_l.len() == new_pos {
                    tape_l.push(0);
                }
            }
            (Direction::Right, 0, false) => {
                // moving right from edge of left tape
                new_pos = 0;
                new_is_right_tape = true;
            }
            (Direction::Right, k, false) => {
                // moving right on left tape
                new_pos = k - 1;
                new_is_right_tape = false;
            }
            (Direction::Right, k, true) => {
                // moving right on right tape
                new_pos = k + 1;
                new_is_right_tape = true;
                if tape_r.len() == new_pos {
                    tape_r.push(0);
                }
            }
            (Direction::None, k, r) => {
                new_pos = k;
                new_is_right_tape = r;
            }
        }

        return Some((new_state, (new_pos, new_is_right_tape)));
    }

    None
}

pub fn run(
    rule_arr: &Vec<Option<(u64, u64, Direction)>>,
    init_tape: Option<Vec<u64>>,
    num_symbols: u64,
    print_tape: bool,
) {
    let mut state = 0;
    let mut loc: (usize, bool) = (0, true);
    let mut tape_l: Vec<u64> = Vec::new();
    let mut tape_r = match init_tape {
        Some(t) => t,
        None => vec![0],
    };

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

    while let Some((new_state, new_loc)) =
        apply_rule(state, &mut tape_l, &mut tape_r, loc, &rule_arr, num_symbols)
    {
        state = new_state;
        loc = new_loc;

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
