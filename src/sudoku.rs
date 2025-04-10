use std::fmt;
use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};
use crate::sudoku_solver::SudokuSolver;
use std::collections::HashSet;

enum Difficulty{
    VeryEasy,
    Easy,
    Medium,
    Hard,
    VeryHard
}

#[derive(Clone)]
pub struct Sudoku{
    grid: [u8; 81],
}

impl Sudoku{
    fn new(difficulty: Difficulty) -> Self{
        Sudoku{
            grid: Self::create_sudoku(difficulty),
        }
    }

    pub fn from_string(s: &str) -> Result<Sudoku, String>{
        let mut grid = [0; 81];

        let chars: Vec<char> = s.chars()
            .filter(|c| c.is_digit(10) || *c == '.' || *c == '0')
            .collect();

        if chars.len() != 81{
            return Err(format!("Invalid input length: got {}, expected 81", chars.len()));
        }

        for (i, c) in chars.iter().enumerate(){
            grid[i] = match c{
                '.' | '0' => 0,
                c if c.is_digit(10) => c.to_digit(10).unwrap() as u8,
                _ => return Err(format!("Invalid character: {}", c)),
            };
        }

        let s = Sudoku{grid};

        for (pos, &tile) in s.grid.iter().enumerate(){
            if  tile != 0 && !s.is_valid_placement(pos/9, pos%9, tile){
                return Err(format!("Invalid placement of tile: {} in row: {} in col: {}", tile, pos/9, pos%9));
            }
        }

        Ok(s)
    }

    pub fn new_very_easy() -> Self{
        Sudoku::new(Difficulty::VeryEasy)
    }

    pub fn new_easy() -> Self{
        Sudoku::new(Difficulty::Easy)
    }

    pub fn new_medium() -> Self{
        Sudoku::new(Difficulty::Medium)
    }

    pub fn new_hard() -> Self{
        Sudoku::new(Difficulty::Hard)
    }

    pub fn new_very_hard() -> Self{
        Sudoku::new(Difficulty::VeryHard)
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: u8) ->Result<(), String>{
        match Self::is_valid_placement(&self, row, col, value){
            true =>{
                self.grid[row * 9 + col] = value;
                Ok(())
            }, 
            false => {
                Err("Invalid placement".to_string())
            }
        }
    }

    pub fn get_value(&self, row: usize, col: usize) -> Result<u8, String>{
        if row > 8 || col > 8{
            return Err("Invalid input".to_string());
        }
        
        Ok(self.grid[row * 9 + col])
    }

    pub fn unset_value(&mut self, row: usize, col: usize) -> Result<(), String>{
        if row > 8 || col > 8{
            return Err("Invalid input".to_string());
        }

        self.grid[row * 9 + col] = 0;

        Ok(())
    }

    pub fn is_solved(&self) -> bool{
        let empty_tiles: Vec<&u8> = self.grid.iter().filter(|&&tile| tile == 0).collect();
        
        if !empty_tiles.is_empty(){
            return false;
        }

        for (pos, &tile) in self.grid.iter().enumerate(){
            if !self.is_valid_placement(pos / 9, pos % 9, tile){
                return false
            }
        }

        true
    }

    fn create_sudoku(difficulty: Difficulty) -> [u8; 81]{
        let mut rng = rand::rng();

        let prefilled_cells = match difficulty{
            Difficulty::VeryEasy => {
                rng.random_range(35..41)
            },
            Difficulty::Easy => {
                rng.random_range(30..35)
            },
            Difficulty::Medium => {
                rng.random_range(25..30)
            },
            Difficulty::Hard => {
                rng.random_range(20..25)
            },
            Difficulty::VeryHard => {
                rng.random_range(17..23)
            }
        };

        let solver = SudokuSolver::new();

        let mut solution = Self::create_full_random_sudoku();

        let mut idxs: Vec<usize> = (1..81).collect();
        idxs.shuffle(&mut rng);

        // change this depending on difficulty
        for idx in idxs{
            let value = solution.get_value(idx / 9, idx % 9).unwrap();
            solution.unset_value(idx / 9, idx % 9).unwrap();

            if !solver.has_unique_solution(&solution){
                solution.set_value(idx/9, idx%9, value).unwrap();
            }
        }

        solution.grid
    }

    fn create_full_random_sudoku() -> Sudoku{
        let mut sudoku = Sudoku{
            grid: [0; 81]
        };
        let solver = SudokuSolver::new();
        let mut rng = rand::rng();

        let mut placed_tiles = HashSet::new();

        let mut placed_numbers = 0;
        while placed_numbers < 81{
            let rand_num = rng.random_range(1..=9);

            let rand_pos = *(0..81)
                .filter(|n|{!placed_tiles.contains(n)})
                .collect::<Vec<usize>>()
                .choose(&mut rng)
                .unwrap();

            if let Err(_) = sudoku.set_value(rand_pos / 9 , rand_pos % 9, rand_num){
                continue;
            }

            if solver.has_solution(&sudoku){
                placed_tiles.insert(rand_pos);
                placed_numbers += 1;
            }else{
                sudoku.unset_value(rand_pos / 9, rand_pos % 9).unwrap();
            }
        }

        sudoku

    }

    fn is_valid_placement(&self, row: usize, col: usize, value: u8) -> bool{
        // Implement Sudoku validation logic here
        if value < 1 || value > 9 || row > 8 || col > 8{
            return false;
        }

        //check column
        for r in (0..9).filter(|&n|{n != row}){
            if self.grid[r * 9 + col] == value{
                return false;
            }
        }

        //check row
        for c in (0..9).filter(|&n|{n != col}){
            if self.grid[row * 9 + c] == value{
                return false;
            }
        }

        let sq_row_pos = row % 3;
        let sq_col_pos = col % 3;
        
        for r in 0..3{
            for c in 0..3{
                if r == sq_row_pos && c == sq_col_pos{
                    continue;
                }
                if self.grid[((row / 3) * 3 + r ) * 9 + (col / 3) * 3 + c] == value{
                    return false;
                }
            }
        }
        true

    }

}

impl fmt::Display for Sudoku{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..9 {
            for j in 0..9 {
                write!(f, "{} ", self.grid[i * 9 + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}