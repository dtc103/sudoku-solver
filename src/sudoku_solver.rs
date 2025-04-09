use crate::sudoku::Sudoku;

pub struct SudokuSolver;

impl SudokuSolver{
    pub fn new() -> Self{
        Self
    }

    pub fn solve(&self, sudoku: &Sudoku) -> Sudoku{
        unimplemented!()
    }

    pub fn has_solution(&self, sudoku: &Sudoku) -> bool{
        unimplemented!()
    }

    pub fn has_unique_solution(&self, sudoku: &Sudoku) -> bool{
        unimplemented!()
    }
}