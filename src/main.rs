mod sudoku;
mod sudoku_solver;

use sudoku::Sudoku;

use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};
use std::collections::HashSet;

fn main() {
    let mut rng = rand::rng();
    let mut placed_tiles:HashSet<usize> = HashSet::from([1, 5, 7, 9, 10, 43, 132, 34, 2]);
    println!("{}", *(0..81).filter(|n|{!placed_tiles.contains(n)}).collect::<Vec<usize>>().choose(&mut rng).unwrap());
    return;

    let mut sudoku = Sudoku::new_easy();

    sudoku.set_value(0, 0, 8).ok();
    sudoku.set_value(0, 1, 8).ok();

    println!("{}", sudoku);

}
