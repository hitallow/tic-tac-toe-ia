#[derive(Copy, Clone, Debug)]
pub struct Move {
    row: i8,
    column: i8,
}

impl Move {
    pub fn new(row: i8, column: i8) -> Move {
        return Move { row, column };
    }
    pub fn set_column(&mut self, column: i8) {
        self.column = column;
    }

    pub fn get_column(&self) -> i8 {
        return self.column;
    }

    pub fn set_row(&mut self, row: i8) {
        self.row = row;
    }

    pub fn get_row(&self) -> i8 {
        return self.row;
    }
}
