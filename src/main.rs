mod machine;
use machine::{Direction, Machine, Rule};

fn main() {
    // A, B, 1, 0, X, Y, Z
    // 0, 1, 2, 3, 4, 5, 6

    let rules = vec![
        // Q0
        Rule::from(0, 0, 0, 0, Direction::Right),
        Rule::from(0, 4, 0, 4, Direction::Right),
        Rule::from(0, 6, 0, 6, Direction::Right),
        Rule::from(0, 2, 1, 4, Direction::None),
        Rule::from(0, 5, 1, 6, Direction::None),
        Rule::from(0, 1, 3, 1, Direction::Left),
        // Q1
        Rule::from(1, 4, 1, 4, Direction::Left),
        Rule::from(1, 0, 1, 0, Direction::Left),
        Rule::from(1, 2, 1, 2, Direction::Left),
        Rule::from(1, 6, 1, 6, Direction::Left),
        Rule::from(1, 3, 2, 2, Direction::Right),
        // Q2
        Rule::from(2, 2, 2, 2, Direction::Right),
        Rule::from(2, 0, 0, 0, Direction::None),
        // Q3
        Rule::from(3, 6, 3, 5, Direction::Left),
        Rule::from(3, 4, 5, 5, Direction::Left),
        // Q4
        Rule::from(4, 4, 4, 2, Direction::Left),
        Rule::from(4, 0, 0, 0, Direction::None),
        // Q5
        Rule::from(5, 4, 4, 4, Direction::None),
        Rule::from(5, 0, 6, 0, Direction::None),
    ];

    let m = Machine::new(rules, 0, 0, 3);

    let init_tape = vec![0, 2, 2, 2, 1];

    machine::run(m, Option::Some(init_tape));
}
