use crate::cell::Cell;

#[derive(Debug, Clone)]
pub struct Board {
    cells: [[Cell; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        let mut cells = [[Cell::new(); 9]; 9];

        let mut b = Board { cells };
        b.set_cell_value(0, 0, 3);
        b.set_cell_value(0, 2, 7);
        b.set_cell_value(0, 4, 4);
        b.set_cell_value(1, 1, 5);
        b.set_cell_value(1, 6, 1);
        b.set_cell_value(1, 8, 4);
        b.set_cell_value(2, 2, 4);
        b.set_cell_value(2, 3, 2);
        b.set_cell_value(2, 4, 3);
        b.set_cell_value(2, 6, 6);
        b.set_cell_value(3, 0, 4);
        b.set_cell_value(3, 1, 2);
        b.set_cell_value(3, 2, 9);
        b.set_cell_value(3, 4, 5);
        b.set_cell_value(3, 5, 3);
        b.set_cell_value(4, 1, 6);
        b.set_cell_value(4, 2, 3);
        b.set_cell_value(4, 5, 2);
        b.set_cell_value(4, 6, 4);
        b.set_cell_value(4, 7, 8);
        b.set_cell_value(1, 3, 7);
        b.set_cell_value(5, 5, 9);
        b.set_cell_value(6, 2, 5);
        b.set_cell_value(6, 3, 9);
        b.set_cell_value(6, 6, 8);
        b.set_cell_value(6, 8, 6);
        b.set_cell_value(7, 0, 1);
        b.set_cell_value(7, 1, 4);
        /* b.set_cell_value(7, 2, 6);
        b.set_cell_value(7, 8, 2);
        b.set_cell_value(8, 0, 2);
        b.set_cell_value(8, 3, 6);
        b.set_cell_value(8, 5, 7);
        b.set_cell_value(8, 8, 3);*/

        b.autofill_cells();
        b
    }

    pub fn get_cells(&self) -> &[[Cell; 9]; 9] {
        &self.cells
    }
    pub fn set_cell_value(&mut self, x: usize, y: usize, value: u8) {
        self.cells[x][y].set_value(value);

        self.remove_value_from_row_and_col(x, y, value);
        self.remove_value_from_small_grid(x, y, value);
        self.autofill_cells();
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

    pub fn get_empty_cell_with_least_possible_values(&self) -> (usize, usize, u8) {
        let mut min = 10;
        let mut x = 10;
        let mut y = 10;

        for i in 0..9 {
            for j in 0..9 {
                if !self.cells[i][j].is_empty() {
                    continue;
                }
                let count = self.cells[i][j].get_possible_values_count();
                if count < min {
                    min = count;
                    x = i;
                    y = j;
                }
            }
        }

        (x, y, min)
    }

    pub fn get_all_possible_values(&self, x: usize, y: usize) -> Vec<u8> {
        self.cells[x][y].get_all_possible_values()
    }

    fn autofill_cells(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                if self.cells[i][j].is_empty() && self.cells[i][j].get_possible_values_count() == 1
                {
                    self.set_cell_value(i, j, self.cells[i][j].get_all_possible_values()[0]);
                    return;
                }
            }
        }
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
        /*for i in 0..9 {
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
        }*/
    }

    pub fn is_solved(&self) -> bool {
        for r in self.cells {
            for c in r {
                if c.is_empty() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_solvable(&self) -> bool {
        for r in self.cells {
            for _ in r {
                if self.get_empty_cell_with_least_possible_values().2 == 0 {
                    return false;
                }
            }
        }
        true
    }
}
