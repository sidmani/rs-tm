use crate::util::print_state;

pub struct BiInfiniteTape {
    left: Vec<u64>,
    right: Vec<u64>,
    head: isize,
}

impl BiInfiniteTape {
    pub fn create(init: Option<Vec<u64>>) -> BiInfiniteTape {
        let right = if let Some(t) = init { t } else { vec![0] };
        let left = Vec::new();
        BiInfiniteTape {
            left,
            right,
            head: 0,
        }
    }

    pub fn get(&self, index: isize) -> u64 {
        if index < 0 {
            self.left[(-index - 1) as usize]
        } else {
            self.right[index as usize]
        }
    }

    pub fn set(&mut self, index: isize, value: u64) {
        if index < 0 {
            self.left[(-index - 1) as usize] = value;
        } else {
            self.right[index as usize] = value;
        }
    }

    pub fn get_at_head(&self) -> u64 {
        self.get(self.head)
    }

    pub fn set_at_head(&mut self, value: u64) {
        self.set(self.head, value);
    }

    pub fn move_left(&mut self) {
        self.head -= 1;
        if self.head < 0 && -self.head as usize - 1 >= self.left.len() {
            self.left.push(0);
        }
    }

    pub fn move_right(&mut self) {
        self.head += 1;
        if self.head >= 0 && self.head as usize >= self.right.len() {
            self.right.push(0);
        }
    }

    pub fn print_state(&self, state: u64) {
        let joined_tape = [
            self.left.iter().copied().rev().collect(),
            self.right.clone(),
        ]
        .concat();
        let pos = self.left.len() as isize + self.head;
        print_state(state, &joined_tape, pos as usize);
    }
}
