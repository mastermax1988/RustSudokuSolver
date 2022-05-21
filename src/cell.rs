#[derive(Debug, Copy, Clone)]
pub struct Cell {
    value: u8,
    possible_values: [u8; 9],
}

impl Cell {
    pub fn new() -> Cell {
        let value = 0;
        let possible_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        Cell {
            value,
            possible_values,
        }
    }

    pub fn value(&self) -> &u8 {
        &self.value
    }

    pub fn possible_values(&self) -> &[u8; 9] {
        &self.possible_values
    }

    pub fn set_value(&mut self, value: u8) {
        if self.possible_values[(value - 1) as usize] == 0 {
            panic!("Not solvable")
        }
        self.value = value;
        for i in 0..9 {
            if i + 1 == value {
                self.possible_values[i as usize] = i + 1;
            } else {
                self.possible_values[i as usize] = 0;
            }
        }
    }

    pub fn remove_possible_value(&mut self, value: u8) {
        if self.value != value {
            let pos: usize = (value - 1) as usize;
            self.possible_values[pos] = 0;
        }
    }

    pub fn get_possible_values_count(&self) -> u8 {
        let mut ret = 0;
        for i in 0..9 {
            if self.possible_values[i] > 0 {
                ret += 1;
            }
        }
        ret
    }

    pub fn get_all_possible_values(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        for i in 0..9 {
            if self.possible_values[i] != 0 {
                v.push(self.possible_values[i]);
            }
        }
        v
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }
}
