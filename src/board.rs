use crate::cell::Cell;

#[derive(Clone)]
pub struct Board {
    cells: [[Cell; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        let mut cells = [[Cell::new(); 9]; 9];

        let mut b = Board { cells };
        b.set_cell_value(0, 3, 5);
        b.set_cell_value(0, 6, 3);
        b.set_cell_value(1, 2, 2);
        b.set_cell_value(1, 3, 8);
        b.set_cell_value(2, 2, 3);
        b.set_cell_value(2, 7, 9);
        b.set_cell_value(3, 1, 2);
        b.set_cell_value(3, 5, 9);
        b.set_cell_value(3, 8, 1);
        b.set_cell_value(4, 0, 6);
        b.set_cell_value(4, 1, 4);
        b.set_cell_value(4, 7, 5);

        b
    }

    pub fn set_cell_value(&mut self, x: usize, y: usize, value: u8) {
        self.cells[x][y].set_value(value);

        self.remove_value_from_row_and_col(x, y, value);
        self.remove_value_from_small_grid(x, y, value);
    }

    fn remove_value_from_row_and_col(&mut self, row: usize, col: usize, value: u8) {
        for i in 0..9 {
            self.cells[row][i].remove_possible_value(value);
            self.cells[i][col].remove_possible_value(value);
        }
    }

    fn remove_value_from_small_grid(&mut self, row: usize, col: usize, value: u8) {
        let r = row / 3;
        let c = col / 3;
        for i in 3 * r..3 * r + 3 {
            for j in 3 * c..3 * c + 3 {
                self.cells[i][j].remove_possible_value(value);
            }
        }
    }

    pub fn get_cell_with_least_possible_values(&self) -> (usize, usize, u8) {
        let mut min = 10;
        let mut x = 10;
        let mut y = 10;

        for i in 0..9 {
            for j in 0..9 {
                let count = self.cells[i][j].get_possible_values_count();
                if count > 1 && count < min {
                    min = count;
                    x = i;
                    y = j;
                }
            }
        }

        (x, y, min)
    }


    pub fn print(&self) {
        for i in 0..9 {
            for j in 0..9 {
                print!("{} ", self.cells[i][j].value());
                if j == 2 || j == 5 {
                    print!("| ");
                }
            }
            println!();
            if i == 2 || i == 5 {
                println!("---------------------");
            }
        }
        for i in 0..9 {
            for j in 0..9 {
                print!("{:?} ", self.cells[i][j].possible_values());
                if j == 2 || j == 5 {
                    print!("| ");
                }
            }
            println!();
            if i == 2 || i == 5 {
                println!("---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------");
            }
        }
    }

    pub fn is_solved(&self) -> bool{
        for r in self.cells{
            for c in r{
                if c.value() == &0{
                    return false
                }
            }
        }
        true
    }
}

