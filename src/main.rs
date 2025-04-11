#[allow(dead_code)]

mod sudoku;
mod sudoku_solver;

use sudoku::Sudoku;
use sudoku_solver::SudokuSolver;


fn main() {
    let sudoku = Sudoku::new_easy();
    let solver = SudokuSolver::new();

    println!("unsolved: \n{}", sudoku);

    let sudoku = solver.solve(sudoku);

    println!("solved: \n{}", sudoku);
}
