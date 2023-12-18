use grid::Grid;

pub fn part1() {
    let input = include_str!("Input.txt");
    println!("Day 18 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2() {
    let input = include_str!("Input.txt");
    println!("Day 18 Part 2");
    println!("{:?}", part2_implementation(input));
}

pub fn bench() {
    let input = include_str!("Input.txt");
    part1_implementation(input);
    part2_implementation(input);
}

pub fn part1_implementation(input: &str) -> String {
    let mut sum = 0;

    // get the number of rows and columns
    let mut col_min = 0;
    let mut col_max = 0;
    let mut current_col = 0;
    let mut row_min = 0;
    let mut row_max = 0;
    let mut current_row = 0;

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let direction = split[0];
        let value = split[1].parse::<i32>().unwrap();
        match direction {
            "L" => {
                current_col -= value;
                if current_col < col_min {
                    col_min = current_col;
                }
            }
            "R" => {
                current_col += value;
                if current_col > col_max {
                    col_max = current_col;
                }
            }
            "U" => {
                current_row -= value;
                if current_row < row_min {
                    row_min = current_row;
                }
            }
            "D" => {
                current_row += value;
                if current_row > row_max {
                    row_max = current_row;
                }
            }
            _ => panic!("Unknown direction"),
        }
    }

    // create trench
    let cols = (col_max - col_min + 1) as usize;
    let rows = (row_max - row_min + 1) as usize;
    let mut grid = Grid::new(rows, cols);
    let mut position = (-row_min as usize, -col_min as usize);
    set_dug(&mut grid, position);

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let direction = split[0];
        let number = split[1].parse::<i32>().unwrap();
        match direction {
            "L" => {
                for _ in 0..number {
                    position.1 -= 1;
                    set_dug(&mut grid, position);
                }
            }
            "R" => {
                for _ in 0..number {
                    position.1 += 1;
                    set_dug(&mut grid, position);
                }
            }
            "U" => {
                for _ in 0..number {
                    position.0 -= 1;
                    set_dug(&mut grid, position);
                }
            }
            "D" => {
                for _ in 0..number {
                    position.0 += 1;
                    set_dug(&mut grid, position);
                }
            }
            _ => panic!("Unknown direction"),
        }
    }
    display_grid(&grid);

    // fill trench
    let first_col = grid.iter_row(0).position(|&c| c).unwrap();
    let first_corner = (0,first_col);
    let mut filled = grid.clone();
    let mut previous = Adjacent::Left;
    let mut current_position = (0,first_col+1);
    while current_position != first_corner {
        let neighbor = get_neighbor(current_position, &grid, previous);
        if (previous == Adjacent::Down && neighbor.1 !=Adjacent::Left)
            || (previous == Adjacent::Left && neighbor.1 ==Adjacent::Down)
        {
            // set cell on row to dig
            let mut column = current_position.1 + 1;
            let mut count = 0;
            // find first cell to dig
            while *grid.get(current_position.0,column).unwrap(){
                column+=1;
            }
            loop
            {
                let option = grid.get(current_position.0,column);
                if option.is_none() || *option.unwrap() {
                    break;
                }
                set_dug(&mut filled, (current_position.0,column));
                column += 1;
            }
        }

       current_position = neighbor.0;
        previous = neighbor.1;

    }
    //display_grid(&filled);

    let sum = filled.iter().filter(|&c| *c).count();
    sum.to_string()
}

fn get_neighbor(position: (usize, usize), grid: &Grid<bool>, previous_direction: Adjacent) -> ((usize, usize), Adjacent) {
    let mut result = Vec::new();
    let mut directions = match previous_direction {
        Adjacent::Down => vec![Adjacent::Down, Adjacent::Left, Adjacent::Right],
        Adjacent::Up => vec![Adjacent::Up, Adjacent::Left, Adjacent::Right],
        Adjacent::Left => vec![Adjacent::Down, Adjacent::Up, Adjacent::Left],
        Adjacent::Right => vec![Adjacent::Down, Adjacent::Up, Adjacent::Right],
    };

    for direction in directions {
        match direction {
            Adjacent::Down => {
                if position.0 == 0 {
                    continue;
                }
                if *grid.get(position.0 - 1, position.1).unwrap() {
                    result.push(((position.0 - 1, position.1),direction));
                }
            }
            Adjacent::Up => {
                if position.0 == grid.rows() - 1 {
                    continue;
                }
                if *grid.get(position.0 + 1, position.1).unwrap() {
                    result.push(((position.0 + 1, position.1),direction));
                }
            }
            Adjacent::Left => {
                if position.1 == grid.cols() - 1 {
                    continue;
                }
                if *grid.get(position.0, position.1 + 1).unwrap() {
                    result.push(((position.0, position.1 + 1),direction));
                }
            }
            Adjacent::Right => {
                if position.1 == 0 {
                    continue;
                }
                if *grid.get(position.0, position.1 - 1).unwrap() {
                    result.push(((position.0, position.1 - 1),direction));
                }
            }
        }
    }

    if result.len() == 1 {
        return result[0];
    }
    else {
        panic!("Too many neighbors");
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Adjacent {
    Down,
    Up,
    Left,
    Right,
}

fn display_grid(grid: &Grid<bool>) {
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let cell = grid.get(y, x).unwrap();
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn set_dug(p0: &mut Grid<bool>, p1: (usize, usize)) {
    let cell = p0.get_mut(p1.0, p1.1).unwrap();
    *cell = true;
}

pub fn part2_implementation(input: &str) -> String {
    let mut product = 0;
    for line in input.lines() {
        let number = line.parse::<i32>().unwrap();
        product *= number;
    }
    product.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 62.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 0.to_string());
    }
}
