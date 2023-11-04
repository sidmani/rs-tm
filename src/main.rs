mod machine;
mod tape;
// mod verify;
use machine::{build_rule_arr, Direction};
use Direction::{Left, None, Right};

fn main() {
    // A, B, 1, 0, X, Y, Z
    // 3, 1, 2, 0, 4, 5, 6
    let rules = vec![
        // Q0
        (0, 3, 0, 3, Right),
        (0, 4, 0, 4, Right),
        (0, 6, 0, 6, Right),
        (0, 2, 1, 4, None),
        (0, 5, 1, 6, None),
        (0, 1, 3, 1, Left),
        // Q1
        (1, 4, 1, 4, Left),
        (1, 3, 1, 3, Left),
        (1, 2, 1, 2, Left),
        (1, 6, 1, 6, Left),
        (1, 0, 2, 2, Right),
        // Q2
        (2, 2, 2, 2, Right),
        (2, 3, 0, 3, None),
        // Q3
        (3, 6, 3, 5, Left),
        (3, 4, 5, 5, Left),
        // Q4
        (4, 4, 4, 2, Left),
        (4, 3, 0, 3, None),
        // Q5
        (5, 4, 4, 4, None),
        (5, 3, 6, 3, None),
    ];

    let init_tape = vec![3, 2, 2, 2, 1];
    let rule_arr = build_rule_arr(&rules);
    machine::run(&rule_arr, Some(init_tape), 7, true);
}
