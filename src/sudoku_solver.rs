use crate::sudoku::Sudoku;

pub struct SudokuSolver;

impl SudokuSolver{
    pub fn new() -> Self{
        Self
    }

    pub fn solve(&self, mut sudoku: Sudoku) -> Sudoku{
        self.backtrack(&mut sudoku, 0, false);
        sudoku
    }

    pub fn has_solution(&self, sudoku: &Sudoku) -> bool{
        let mut csudoku = sudoku.clone();

        let num_solutions = self.backtrack(&mut csudoku, 0, false);

        if num_solutions == 0{
            false
        }else{
            true
        }
    }

    pub fn has_unique_solution(&self, sudoku: &Sudoku) -> bool{
        let mut csudoku = sudoku.clone();

        let num_solutions = self.backtrack(&mut csudoku, 0, true);

        if num_solutions == 1{
            true
        }else{
            false
        }
    }

    pub fn backtrack(&self, sudoku: &mut Sudoku, pos:usize, check_for_uniqueness: bool) -> usize{
        let mut num_solutions = 0;
        if pos >= 81{
            if sudoku.is_solved(){
                return 1;
            }else{
                return 0;
            }
        }

        if sudoku.get_value(pos/9, pos%9).unwrap() != 0{
            return self.backtrack(sudoku, pos + 1, check_for_uniqueness);
        }else{
            for tile in 1..=9{
                if let Ok(()) = sudoku.set_value(pos / 9, pos % 9, tile){
                    num_solutions += self.backtrack(sudoku, pos + 1, check_for_uniqueness);
                    if check_for_uniqueness && num_solutions > 1{
                        return num_solutions;
                    }
                    if !check_for_uniqueness && num_solutions == 1 {
                        return num_solutions;
                    }
                    
                    sudoku.unset_value(pos/9, pos%9).unwrap();
                }
            }
            return num_solutions;
        }
    }

    pub fn find_all_solutions(sudoku: &Sudoku) -> Vec<Sudoku>{
        let mut csudoku = sudoku.clone();
        let mut solutions = Vec::new();
        let solver = SudokuSolver::new();

        solver.backtrack_find_all(&mut csudoku, 0, &mut solutions);

        solutions
    }

    pub fn backtrack_find_all(&self, sudoku: &mut Sudoku, pos:usize, solutions: &mut Vec<Sudoku>){
        let mut num_solutions = 0;
        if pos >= 81{
            if sudoku.is_solved(){
                solutions.push(sudoku.clone());
            }
        }

        if sudoku.get_value(pos/9, pos%9).unwrap() != 0{
            return self.backtrack_find_all(sudoku, pos + 1, solutions);
        }else{
            for tile in 1..=9{
                if let Ok(()) = sudoku.set_value(pos / 9, pos % 9, tile){
                    self.backtrack_find_all(sudoku, pos + 1, solutions);
                    
                    sudoku.unset_value(pos/9, pos%9).unwrap();
                }
            }
        }
    }
}