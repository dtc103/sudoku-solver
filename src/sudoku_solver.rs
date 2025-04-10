use crate::sudoku::Sudoku;

pub struct SudokuSolver;

impl SudokuSolver{
    pub fn new() -> Self{
        Self
    }

    pub fn solve(&self, sudoku: &Sudoku) -> Sudoku{
        let mut ssudoku = sudoku.clone();
        self.backtrack_has_solution(&mut ssudoku, 0);

        ssudoku
    }

    pub fn has_solution(&self, sudoku: &Sudoku) -> bool {
        let mut ssudoku = sudoku.clone();
        
        self.backtrack_has_solution(&mut ssudoku, 0)
    }

    pub fn backtrack_has_solution(&self, sudoku: &mut Sudoku, pos: usize) -> bool{
        if pos >= 81{
            if sudoku.is_solved(){
                return true;
            }else{
                return false;
            }
        }

        if sudoku.get_value(pos/9, pos%9).unwrap() != 0{
            return self.backtrack_has_solution(sudoku, pos + 1);
        }else{
            for tile in 1..=9{
                if let Ok(()) = sudoku.set_value(pos / 9, pos % 9, tile){
                    if self.backtrack_has_solution(sudoku, pos + 1){
                        return true;
                    }
                    sudoku.unset_value(pos/9, pos%9).unwrap();
                }
            }
            return false;
        }
    }

    pub fn has_unique_solution(&self, sudoku: &Sudoku) -> bool{
        let mut ssudoku = sudoku.clone();
        // TODO maybe do somethign with that later
        let mut solutions: Vec<Sudoku> = Vec::new();
        let num_solutions = self.backtrack_has_unique_solution(&mut ssudoku, 0, &mut solutions);

        if num_solutions > 1 || num_solutions == 0 {
            false
        }else {
            true
        }

    }

    pub fn backtrack_has_unique_solution(&self, sudoku: &mut Sudoku, pos:usize, ssolutions: &mut Vec<Sudoku>) -> usize{
        let mut num_solutions = 0;
        if pos >= 81{
            if sudoku.is_solved(){
                ssolutions.push(sudoku.clone());
                return 1;
            }else{
                return 0;
            }
        }

        if sudoku.get_value(pos/9, pos%9).unwrap() != 0{
            return self.backtrack_has_unique_solution(sudoku, pos + 1, ssolutions);
        }else{
            for tile in 1..=9{
                if let Ok(()) = sudoku.set_value(pos / 9, pos % 9, tile){
                    num_solutions += self.backtrack_has_unique_solution(sudoku, pos + 1, ssolutions);
                    
                    sudoku.unset_value(pos/9, pos%9).unwrap();
                }
            }
            return num_solutions;
        }
    }
}