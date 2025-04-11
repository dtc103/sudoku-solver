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
    grid: [u8; Sudoku::SUDOKU_TILES],
}

impl Sudoku{
    const SUDOKU_TILES: usize = 81;
    pub const EMPTY_TILE: u8 = 0;

    fn new(difficulty: Difficulty, prefilled_cells:Option<usize>) -> Self{
        Self::create_sudoku(difficulty, prefilled_cells)
    }

    pub fn from_string(s: &str) -> Result<Self, String>{
        let mut grid = [Self::EMPTY_TILE; Self::SUDOKU_TILES];

        let s = s.trim().replace(char::is_whitespace, "");

        let chars: Vec<char> = s.chars()
            .filter(|c| c.is_digit(10) || *c == '.' || *c == '0' || *c == '-' || *c == 'x')
            .collect();

        if chars.len() != Self::SUDOKU_TILES{
            return Err(format!("Invalid input length: got {}, expected 81", chars.len()));
        }

        for (i, c) in chars.iter().enumerate(){
            grid[i] = match c{
                '.' | '0' | '-' | 'x' => Self::EMPTY_TILE,
                c if c.is_digit(10) => c.to_digit(10).unwrap() as u8,
                _ => return Err(format!("Invalid character: {}", c)),
            };
        }

        let s = Self{grid};

        for (pos, &tile) in s.grid.iter().enumerate(){
            if  tile != Self::EMPTY_TILE && !s.is_valid_placement(pos/9, pos%9, tile){
                return Err(format!("Invalid placement of tile: {} in row: {} in col: {}", tile, pos/9, pos%9));
            }
        }

        Ok(s)
    }

    pub fn new_custom_sudoku(prefilled_cells: usize) -> Result<Self, String>{
        //difficulty does not matter here
        if prefilled_cells < 17{
            return Err(format!("Cell count can not be smaller than 17. Current value: {}", prefilled_cells));
        }
        Ok(Self::new(Difficulty::Easy, Some(prefilled_cells)))
    }

    pub fn new_very_easy() -> Self{
        Self::new(Difficulty::VeryEasy, None)
    }

    pub fn new_easy() -> Self{
        Self::new(Difficulty::Easy, None)
    }

    pub fn new_medium() -> Self{
        Self::new(Difficulty::Medium, None)
    }

    pub fn new_hard() -> Self{
        Self::new(Difficulty::Hard, None)
    }

    pub fn new_very_hard() -> Self{
        Self::new(Difficulty::VeryHard, None)
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: u8) ->Result<(), String>{
        if Self::is_valid_placement(&self, row, col, value){
            self.grid[row * 9 + col] = value;
            Ok(())
        }
        else{
            Err(format!("Invalid placement in row: {}, col: {}, given value: {}", row, col, value))
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

        self.grid[row * 9 + col] = Self::EMPTY_TILE;

        Ok(())
    }

    pub fn is_solved(&self) -> bool{
        let empty_tiles: Vec<&u8> = self.grid.iter().filter(|&&tile| tile == Self::EMPTY_TILE).collect();
        
        if !empty_tiles.is_empty(){
            return false;
        }
        //we don't need to check here, if all tiles are placed correctly, since we do this with each tile in the is_valid_placement function already

        true
    }

    fn create_sudoku(difficulty: Difficulty, prefilled_cells: Option<usize>) -> Self{
        let mut rng = rand::rng();

        let prefilled_cells = if let Some(n) = prefilled_cells{
            n
        }else{
            match difficulty{
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
            }
        };

        let solver = SudokuSolver::new();

        let mut filled_sudoku = Self::create_full_random_sudoku();

        let mut removed_tiles = 0;

        while Self::SUDOKU_TILES - removed_tiles > prefilled_cells{
            let mut idxs: Vec<usize> = (1..Self::SUDOKU_TILES).collect();
            idxs.shuffle(&mut rng);
            
            for idx in idxs.into_iter(){
                let tile_value = filled_sudoku.get_value(idx/9, idx%9).unwrap();
                filled_sudoku.unset_value(idx/9, idx%9).unwrap();

                if !solver.has_unique_solution(&filled_sudoku){
                    filled_sudoku.set_value(idx/9, idx%9, tile_value).unwrap();
                    continue;
                }

                removed_tiles += 1;

                if Self::SUDOKU_TILES - removed_tiles <= prefilled_cells{
                    break;
                }
            }
        }

        filled_sudoku
    }

    pub fn create_full_random_sudoku() -> Self{
        let mut sudoku = Self{
            grid: [Self::EMPTY_TILE; 81]
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

        if value == self.get_value(row, col).unwrap(){
            return true;
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