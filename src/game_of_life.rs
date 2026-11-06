// Game of Life, Week 12: grid, display, neighbor counts, the rule for one cell.
//
// TODO: remove the next line once every function below is used.
#![allow(dead_code)]

const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

// All cells dead
pub fn new_grid() -> Grid {
    [[false; COLS]; ROWS]
}
// '#' for a live cell, '.' for a dead cell, one line per row.
pub fn print_grid(grid: &Grid) {
    for row in grid {
        for &cell in row {
            if cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

// Live neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead. Careful: row - 1 on a usize of 0 panics.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count = 0;

    for dr in -1..=1 {
     for dc in -1..=1 {
        if dr == 0 && dc == 0 {
             continue;
            }

            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0
                && new_row < ROWS as isize
                && new_col >= 0
                && new_col < COLS as isize
            {
                if grid[new_row as usize][new_col as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}

// The rule for one cell: does it live in the next generation?
pub fn next_state(alive: bool, neighbors: u8) -> bool {
    match (alive, neighbors) {
        (true, 2..=3) => true,
        (false, 3) => true,
        _ => false,
    }
}

// Debug view: each cell's neighbor count as a digit, one line per row.
pub fn print_neighbor_counts(grid: &Grid) {
    for row in 0..ROWS {
        for col in 0..COLS {
            print!("{}", count_neighbors(grid, row, col));
        }
        println!();
    }
}

// Set a few cells alive (hardcoded, or entered as `row col` pairs until a blank line),
// print the grid, then the neighbor counts.
pub fn play() {
    let mut grid = new_grid();
    grid[4][10] = true;
    grid[5][10] = true;
    grid[6][10] = true;

    println!("Game of Life Grid:");
    print_grid(&grid);

    println!("\nNeighbor Counts:");
    print_neighbor_counts(&grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Supplied by the instructor.
    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

      #[test]
    fn live_4_dies() {
        assert!(!next_state(true, 4));
    }

    #[test]
    fn dead_2_stays_dead() {
        assert!(!next_state(false, 2));
    }

    #[test]
    fn live_0_dies() {
        assert!(!next_state(true, 0));
    }

    #[test]
    fn dead_8_stays_dead() {
        assert!(!next_state(false, 8));
    }
}
