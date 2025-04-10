mod sudoku;
mod sudoku_solver;

use sudoku::Sudoku;

use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};
use sudoku_solver::SudokuSolver;
use std::collections::HashSet;

static EXAMPLE_SUDOKU: &str = "980010002007096800603070009078609410409001063500000000030000057005180390000537284";
static AMBIGUOUS_SUDOKU: &str = "500000070001003000000009002050040007000100000008000500000500900600010000090002000";

fn main() {
    let sudoku =Sudoku::from_string(EXAMPLE_SUDOKU).unwrap();
    let solver = SudokuSolver::new();

    let solved_sudoku = solver.has_unique_solution(&sudoku);

    //let solved_sudoku = solver.has_solution(&sudoku);

    //let solved_sudoku = solver.solve(&sudoku);

    println!("Unsovled: \n{}", sudoku);
    println!("Solved: \n{}", solved_sudoku);

}
