use std::fmt;
use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};
use crate::sudoku_solver::SudokuSolver;
use std::collections::HashSet;


pub mod example_sudokus{
    use super::Sudoku;

    pub fn example_sudoku()-> Sudoku {
        Sudoku::from_string(EXAMPLE_SUDOKU).unwrap()
    }

    pub fn ambiguous_sudoku()->Sudoku {
        Sudoku::from_string(AMBIGUOUS_SUDOKU).unwrap()
    }

    pub fn extreme_sudoku() -> Sudoku {
        Sudoku::from_string(EXTREME_SUDOKU).unwrap()
    }

    pub fn ambiguous_easy() -> Sudoku{
        Sudoku::from_string(AMBIGUOUS_EASY).unwrap()
    }

    pub fn one_number_sudoku() -> Sudoku {
        Sudoku::from_string(ONE_NUMBER_SUDOKU).unwrap()
    }

    static EXAMPLE_SUDOKU: &str = "980010002007096800603070009078609410409001063500000000030000057005180390000537284";
    static AMBIGUOUS_SUDOKU: &str = "500000070001003000000009002050040007000100000008000500000500900600010000090002000";
    static EXTREME_SUDOKU: &str = "500400030000010600000080040000000000001340000073200009680000007000500020210060000";
    static AMBIGUOUS_EASY: &str = "145327698839654127672918543496085370218473956753096480367542819984761235521839764";
    static ONE_NUMBER_SUDOKU: &str = "000000000000000000000000000000000000000000000000000000000000000000000000000000005";
}

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
        Self::create_sudoku(difficulty)
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

    fn create_sudoku(difficulty: Difficulty) -> Sudoku{
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

        let mut filled_sudoku = Self::create_full_random_sudoku();

        let mut removed_tiles = 0;

        while 81 - removed_tiles > prefilled_cells{
            let mut idxs: Vec<usize> = (1..81).collect();
            idxs.shuffle(&mut rng);
            
            for idx in idxs.into_iter(){
                let tile_value = filled_sudoku.get_value(idx/9, idx%9).unwrap();
                filled_sudoku.unset_value(idx/9, idx%9).unwrap();

                if !solver.has_unique_solution(&filled_sudoku){
                    filled_sudoku.set_value(idx/9, idx%9, tile_value).unwrap();
                    continue;
                }

                removed_tiles += 1;

                if 81 - removed_tiles <= prefilled_cells{
                    break;
                }
            }
        }

        filled_sudoku
    }

    pub fn create_full_random_sudoku() -> Sudoku{
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
            if i % 3 == 0{
                writeln!(f, "-------------------------")?;
            }
            for j in 0..9 {
                if j % 3 == 0{
                    write!(f, "| {} ", self.grid[i * 9 + j])?;
                }
                else{
                    write!(f, "{} ", self.grid[i * 9 + j])?;
                }
            }
            write!(f, "|")?;
            writeln!(f)?;
        }
        writeln!(f, "-------------------------")?;
        Ok(())
    }
}