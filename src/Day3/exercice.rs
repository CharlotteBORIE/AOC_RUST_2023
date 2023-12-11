use grid::Grid;
use std::collections::HashMap;
use std::fmt::Error;

pub fn part1() {
    let input = include_str!("Input.txt");
    println!("Day 3 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2() {
    let input = include_str!("Input.txt");
    println!("Day 3 Part 2");
    println!("{:?}", part2_implementation(input));
}

pub fn bench(){
    let input = include_str!("Input.txt");
    part1_implementation(input);
    part2_implementation(input);
}

pub fn bench1(){
    let input = include_str!("Input.txt");
    part1_implementation(input);
}
pub fn bench2(){
    let input = include_str!("Input.txt");
    part2_implementation(input);
}
pub fn part1_implementation(input: &str) -> String {
    let cols = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::from_vec(input.chars().filter(|&c| c != '\n').collect(), cols);
    let mut sum = 0;
    let mut to_add = false;
    let mut number = 0;

    for ((row, column), cell) in grid.indexed_iter() {
        if cell.is_alphanumeric() {
            number = number * 10 + cell.to_digit(10).unwrap() as i32;
            if is_neighbour_symbol(&grid, row, column) {
                to_add = true;
            }
        } else {
            if to_add {
                sum += number;
            }
            number = 0;
            to_add = false;
        }
    }
    sum.to_string()
}

pub fn part2_implementation(input: &str) -> String {
    let cols = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::from_vec(input.chars().filter(|&c| c != '\n').collect(), cols);
    let mut sum = 0;
    let mut number = 0;
    let mut dictionnary: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut gear_position = (0, 0);

    for ((row, column), cell) in grid.indexed_iter() {
        if cell.is_alphanumeric() {
            number = number * 10 + cell.to_digit(10).unwrap() as i32;
            match is_neighbour_gear(&grid, row, column) {
                Ok((i, j)) => {
                    gear_position = (i, j);
                }
                Err(_) => {
                    continue;
                }
            }
        } else {
            if gear_position != (0, 0) {
                match dictionnary.get_mut(&gear_position) {
                    None => {
                        dictionnary.insert(gear_position, vec![number]);
                    }
                    Some(list) => {
                        list.push(number);
                    }
                }
            }
            number = 0;
            gear_position = (0, 0)
        }
    }
    for list in dictionnary.values() {
        if list.len() == 2 {
            sum += list[0] * list[1];
        }
    }
    sum.to_string()
}

fn is_neighbour_symbol(grid: &Grid<char>, row: usize, col: usize) -> bool {
    let start_row = match row < 1 {
        true => 0,
        false => row - 1,
    };
    let start_col = match col < 1 {
        true => 0,
        false => col - 1,
    };
    for i in start_row..=row + 1 {
        for j in start_col..=col + 1 {
            match grid.get(i, j) {
                Some(symbol) if !symbol.is_alphanumeric() && *symbol != '.' => {
                    return true;
                }
                Some(_) | None => {
                    continue;
                }
            }
        }
    }
    return false;
}

fn is_neighbour_gear(grid: &Grid<char>, row: usize, col: usize) -> Result<(usize, usize), Error> {
    let start_row = match row < 1 {
        true => 0,
        false => row - 1,
    };
    let start_col = match col < 1 {
        true => 0,
        false => col - 1,
    };
    for i in start_row..=row + 1 {
        for j in start_col..=col + 1 {
            match grid.get(i, j) {
                Some(symbol) if *symbol == '*' => {
                    return Ok((i, j));
                }
                Some(_) | None => {
                    continue;
                }
            }
        }
    }
    return Err(Error);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 4361.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 467835.to_string());
    }
}
