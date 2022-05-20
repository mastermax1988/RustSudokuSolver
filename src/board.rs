use std::collections::HashSet;


#[derive(Debug, Clone)]
pub struct Cell {
    value: u8,
    possible_values: HashSet<u8>,
}

impl Cell{
    pub fn new() -> Cell{
        let value = 0;
        let mut possible_values: HashSet<u8> = HashSet::new();
        for i in 1..10{
            possible_values.insert(i);
        }
        Cell{value, possible_values}
    }

    pub fn remove_possible_value(&mut self, value: u8){
        self.possible_values.remove(&value);
    }

}