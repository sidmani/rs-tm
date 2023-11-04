pub struct BiInfiniteTape {
    left: Vec<u64>,
    right: Vec<u64>,
    head: isize,
}

impl BiInfiniteTape {
    pub fn new(init: Option<Vec<u64>>) -> BiInfiniteTape {
        BiInfiniteTape {
            left: Vec::new(),
            right: if let Some(t) = init { t } else { vec![0] },
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
}

impl ToString for BiInfiniteTape {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for i in (0..self.left.len()).rev() {
            let text;
            if self.head < 0 && i == -self.head as usize - 1 {
                text = format!("\x1b[91m\x1b[1m{}\x1b[0m ", self.left[i]);
            } else {
                text = format!("{} ", self.left[i]);
            }
            result.push_str(&text);
        }
        for i in 0..self.right.len() {
            let text;
            if self.head >= 0 && i == self.head as usize {
                text = format!("\x1b[91m\x1b[1m{}\x1b[0m ", self.right[i]);
            } else {
                text = format!("{} ", self.right[i]);
            }
            result.push_str(&text)
        }

        result
    }
}
