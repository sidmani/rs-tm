use crate::tape::BiInfiniteTape;
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
    for i in 0..=max_state {
        for j in 0..=max_sym {
            result.push(rulemap.get(&(i, j)).map(|r| (r.2, r.3, r.4.clone())));
        }
    }

    result
}

fn apply_rule(
    state: u64,
    tape: &mut BiInfiniteTape,
    rule_arr: &Vec<Option<(u64, u64, Direction)>>,
    num_symbols: u64,
) -> Option<u64> {
    let curr_char = tape.get_at_head();
    let rule_idx = (state * num_symbols + curr_char) as usize;
    if rule_idx >= rule_arr.len() {
        return None;
    }

    if let Some(rule) = &rule_arr[rule_idx] {
        let new_state = rule.0;
        tape.set_at_head(rule.1);
        match &rule.2 {
            Direction::Left => {
                tape.move_left();
            }
            Direction::Right => {
                tape.move_right();
            }
            Direction::None => {}
        }
        return Some(new_state);
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
    let mut tape = BiInfiniteTape::create(init_tape);

    tape.print_state(state);
    while let Some(new_state) = apply_rule(state, &mut tape, rule_arr, num_symbols) {
        state = new_state;
        tape.print_state(state);
    }
}
