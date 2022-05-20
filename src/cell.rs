#[derive(Debug, Copy, Clone)]
struct Cell {
    value: u8,
    possible_values: [u8; 9],
}


impl Cell {
    fn new() -> Cell {
        let value = 0;
        let possible_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        Cell { value, possible_values }
    }

    fn value(&self) -> &u8{
        &self.value
    }

    fn possible_values(&self) -> &[u8;9]{
        &self.possible_values
    }

    fn set_value(&mut self, value: u8) {
        self.value = value;
        for i in 0..9 {
            if i + 1 == value {
                self.possible_values[i as usize] = i + 1;
            } else {
                self.possible_values[i as usize] = 0;
            }
        }
    }

    fn remove_possible_value(&mut self, value: u8) {
        let pos: usize = (value - 1) as usize;
        self.possible_values[pos] = 0;
    }
}