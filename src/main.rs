mod sudoku;
mod sudoku_solver;

use sudoku::Sudoku;

use sudoku_solver::SudokuSolver;

static EXAMPLE_SUDOKU: &str = "980010002007096800603070009078609410409001063500000000030000057005180390000537284";
static AMBIGUOUS_SUDOKU: &str = "500000070001003000000009002050040007000100000008000500000500900600010000090002000";
static EXTREME_SUDOKU: &str = "500400030000010600000080040000000000001340000073200009680000007000500020210060000";
static AMBIGUOUS_EASY: &str = "145327698839654127672918543496085370218473956753096480367542819984761235521839764";

fn main() {
    let sudoku =Sudoku::from_string(AMBIGUOUS_SUDOKU).unwrap();
    let solver = SudokuSolver::new();

    let has_solution = solver.has_solution(&sudoku);
    let is_unique = solver.has_unique_solution(&sudoku);
    let solved_sudoku = solver.solve(&sudoku);

    println!("Original Sudoku: \n{}", sudoku);
    println!("Has solution: {}", has_solution);
    println!("Is unique: {}", is_unique);
    println!("Solved Sudoku: \n{}", solved_sudoku);
}
