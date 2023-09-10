use crate::machine::Rule;
use std::collections::{HashMap, HashSet};

pub fn is_reversible(rules: &Vec<Rule>) -> bool {
    let mut rules_by_terminal_state: HashMap<u64, Vec<&Rule>> = HashMap::new();

    for rule in rules {
        if let Some(state_rules) = rules_by_terminal_state.get_mut(&rule.final_state) {
            state_rules.push(rule);
        } else {
            let mut state_rules: Vec<&Rule> = Vec::new();
            state_rules.push(rule);
            rules_by_terminal_state.insert(rule.final_state, state_rules);
        }
    }

    // check direction is the same for each state
    for state_rules in rules_by_terminal_state.values() {
        if state_rules.len() == 1 {
            continue;
        }

        let direction = &state_rules[0].dir;
        for rule in &state_rules[1..] {
            if rule.dir != *direction {
                return false;
            }
        }
    }

    // check that final characters are not repeated
    for state_rules in rules_by_terminal_state.values() {
        let mut final_chars: HashSet<u64> = HashSet::new();
        for rule in state_rules {
            if final_chars.contains(&rule.final_char) {
                return false;
            }

            final_chars.insert(rule.final_char);
        }
    }

    return true;
}
