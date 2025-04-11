#[allow(dead_code)]

mod sudoku;
mod sudoku_solver;

use sudoku::Sudoku;
use sudoku::example_sudokus;
use sudoku_solver::SudokuSolver;

use std::time::{Duration, Instant};


fn main() {
    //let sudoku = Sudoku::new_custom_sudoku(17).unwrap();
    let sudoku = example_sudokus::ambiguous_sudoku();
    let solver = SudokuSolver::new();

    println!("unsolved: \n{}", sudoku);
    
    let all_solutions = solver.find_all_solutions(&sudoku);

    println!("Found {} solutions", all_solutions.len());
    if all_solutions.len() < 50{
        for (i, solved_sudoku) in all_solutions.into_iter().enumerate(){
            println!("Solution {}\n{}", i+1, solved_sudoku);
        }
    }
    
}
