#[cfg(test)]
mod tests {
    use crate::sudoku_solver;
    use crate::Grid;
    #[test]
    fn should_return_correct_subgrid_index() {
        let mut grid = Grid::new();
        sudoku_solver(&mut grid, 0, 0);

        grid.draw();
    }
}

fn sudoku_solver(grid: &mut Grid, i: u8, j: u8) -> bool {
    if i == grid.length - 1 && j == grid.length {
        return true;
    }

    if j == grid.length {
        return sudoku_solver(grid, i + 1, 0);
    }

    if grid.board[i as usize][j as usize] > 0 {
        return sudoku_solver(grid, i, j + 1);
    }

    for num in 1..9 {
        if can_put_here(grid, i, j, num) {
            put(grid, i, j, num);
            return sudoku_solver(grid, i, j);
        }

        put(grid, i, j, 0);
    }

    return false;
}

fn put(grid: &mut Grid, i: u8, j: u8, num: u8) {
    grid.board[i as usize][j as usize] = num;
}

fn can_put_here(grid: &Grid, i: u8, j: u8, n: u8) -> bool {
    let sub_grid_index = Grid::get_current_subgrid_index(i, j);
    check_vertical(grid, j, n)
        && check_horizontal(grid, i, n)
        && can_put_in_subgrid(grid, sub_grid_index, n)
}

fn check_vertical(grid: &Grid, j: u8, n: u8) -> bool {
    for i in 0..9 {
        if grid.board[i as usize][j as usize] == n {
            return false;
        }
    }
    true
}

fn check_horizontal(grid: &Grid, i: u8, n: u8) -> bool {
    for index in 0..9 {
        if grid.board[i as usize][index] == n {
            return false;
        }
    }
    true
}

fn can_put_in_subgrid(grid: &Grid, grid_index: u8, n: u8) -> bool {
    let sub_grid = &grid.sub_grids[grid_index as usize];

    for i in sub_grid.board {
        let sub_array = i;
        for element in sub_array {
            if element == &n {
                return false;
            }
        }
    }

    true
}

pub struct SubGrid<'a> {
    pub board: [&'a [u8]; 3],
}

pub struct Grid<'a> {
    pub board: [[u8; 9]; 9],
    pub sub_grids: Vec<SubGrid<'a>>,
    pub length: u8,
}

impl<'a> Grid<'a> {
    pub fn get_current_subgrid_index(i: u8, j: u8) -> u8 {
        (i / 3) + (j / 3)
    }

    pub fn draw(&self) {
        for row in self.board {
            println!("{:?}", row);
        }
    }

    pub fn pre_fill_board(&self) {}

    fn new() -> Self {
        let board: [[u8; 9]; 9] = [[0; 9]; 9];

        let result = Self {
            board: board,
            sub_grids: vec![],
            length: 9,
        };

        result.make_sub_grids(0, 0, 9, 3, vec![]);
        result
    }

    fn make_sub_grids(
        &'a self,
        i: u8,
        j: u8,
        n: u8,
        step: u8,
        mut result: Vec<SubGrid<'a>>,
    ) -> Vec<SubGrid<'a>> {
        if i == n && j == n {
            return result;
        }

        if j == n {
            return Grid::make_sub_grids(&self, i + step, 0, n, step, result);
        } else {
            let j_usize = j as usize;
            let i_usize = i as usize;
            let sub_grid = SubGrid {
                board: [
                    &self.board[i_usize][j_usize..j_usize + 3],
                    &self.board[i_usize + 1][j_usize..j_usize + 3],
                    &self.board[i_usize + 2][j_usize..j_usize + 3],
                ],
            };

            result.push(sub_grid);
            return Grid::make_sub_grids(&self, i, j + step, step, step, result);
        }
    }
}
