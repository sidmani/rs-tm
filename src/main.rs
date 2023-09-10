mod machine;
mod verify;
use machine::{Direction, Rule};
use Direction::{Right, Left, None};

fn main() {
    // A, B, 1, 0, X, Y, Z
    // 3, 1, 2, 0, 4, 5, 6

    let rules = vec![
        // Q0
        Rule::from(0, 3, 0, 3, Right),
        Rule::from(0, 4, 0, 4, Right),
        Rule::from(0, 6, 0, 6, Right),
        Rule::from(0, 2, 1, 4, None),
        Rule::from(0, 5, 1, 6, None),
        Rule::from(0, 1, 3, 1, Left),
        // Q1
        Rule::from(1, 4, 1, 4, Left),
        Rule::from(1, 3, 1, 3, Left),
        Rule::from(1, 2, 1, 2, Left),
        Rule::from(1, 6, 1, 6, Left),
        Rule::from(1, 0, 2, 2, Right),
        // Q2
        Rule::from(2, 2, 2, 2, Right),
        Rule::from(2, 3, 0, 3, None),
        // Q3
        Rule::from(3, 6, 3, 5, Left),
        Rule::from(3, 4, 5, 5, Left),
        // Q4
        Rule::from(4, 4, 4, 2, Left),
        Rule::from(4, 3, 0, 3, None),
        // Q5
        Rule::from(5, 4, 4, 4, None),
        Rule::from(5, 3, 6, 3, None),
    ];

    let init_tape = vec![3, 2, 2, 2, 1];
    machine::run(&rules, Option::Some(init_tape), true);
}
