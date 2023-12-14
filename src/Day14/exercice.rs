use std::iter::StepBy;
use std::panic::panic_any;
use std::slice::Iter;
use grid::{Grid, GridColIter};

pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 14 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 14 Part 2");
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
    grid = tilt_grid_north(&grid);
    let mut sum = 0;
    for column in grid.iter_cols() {
        let number = north_pressure(column);
        sum += number;
    }
    sum.to_string()
}

fn north_pressure(column: StepBy<Iter<char>>) -> usize {
    let mut sum = 0;
    for (index,&row) in column.clone().enumerate(){
        match row {
            '.' => {},
            '#' => {
            }
            'O' => {
                sum += column.len() - index;
            }
            _ => panic!("Invalid char"),
        }
    }
    sum
}

pub fn part2_implementation(input: &str) -> String {
    let cols = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::from_vec(input.chars().filter(|&c| c != '\n').collect(), cols);
    grid = spin_grid(&grid ,1000000000);
    let mut sum = 0;
    for column in grid.iter_cols() {
        let number = north_pressure(column);
        sum += number;
    }
    sum.to_string()
}

fn spin_grid(grid: &Grid<char>, number_of_cycles: usize) -> Grid<char> {
    let mut list_of_grid = vec![grid.clone()];
    let mut new_grid = grid.clone();
    for index in 0..number_of_cycles {
        let mut sum = 0;
        for column in list_of_grid[index].iter_cols() {
            let number = north_pressure(column);
            sum += number;
        }

        new_grid = spin_grid_once(&new_grid);
        if list_of_grid.contains(&new_grid) {
            //we found a loop
            let loop_start = list_of_grid.iter().position(|x| *x == new_grid).unwrap();
            let loop_size = index + 1 - loop_start;
            let offset = (number_of_cycles - loop_start) % loop_size;

            return list_of_grid.get(loop_start + offset).unwrap().clone();
        }
        list_of_grid.push( new_grid.clone());
    }
    new_grid
}

fn spin_grid_once(grid: &Grid<char>) -> Grid<char> {
    let mut grid = grid.clone();
    //north
    grid = tilt_grid_north(&grid);

    //west
    grid =tilt_grid_west(&grid);

    //south
    grid = tilt_grid_south(&mut grid);

    //east
    tilt_grid_east(&mut grid)
}

fn tilt_grid_east(grid: &Grid<char>) -> Grid<char> {
    let mut temp_grid: Grid<char> = Grid::new(0, 0);

    for (index, row) in grid.iter_rows().enumerate() {
        let mut new_row = tilt_west(row.rev().collect::<Vec<&char>>());
        new_row.reverse();
        temp_grid.push_row(new_row);
    }
    temp_grid
}

fn tilt_grid_south(grid:&Grid<char>)  -> Grid<char> {
    let mut temp_grid: Grid<char> = Grid::new(0, 0);

    for (index, col) in grid.iter_cols().enumerate() {
        let mut new_col = tilt_north(col.rev().collect::<Vec<&char>>());
        new_col.reverse();
        temp_grid.push_col(new_col);
    }
    temp_grid
}

fn tilt_grid_west(grid:&Grid<char>)  -> Grid<char> {
    let mut temp_grid: Grid<char> = Grid::new(0, 0);
    for (index, row) in grid.iter_rows().enumerate() {
        temp_grid.push_row(tilt_west(row.collect::<Vec<&char>>()));
    }
    temp_grid
}

fn tilt_grid_north(grid:&Grid<char>)  -> Grid<char>{
    let mut temp_grid: Grid<char> = Grid::new(0, 0);
    for (index, col) in grid.iter_cols().enumerate() {
        temp_grid.push_col(tilt_north(col.collect::<Vec<&char>>()));
    }
    temp_grid
}

fn display_grid(grid:&Grid<char>) {
    for row in grid.iter_rows() {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

fn tilt_north( column: Vec<&char>) -> Vec<char>{
    let mut new_column :Vec<char>= vec!['.';column.len()];
    let mut count = 0;
    let mut block_index = 0;
    let mut current_index = 0;
    for (current_index,&row) in column.iter().enumerate(){
        match row {
            '.' => {},
            '#' => {
                //println!("block: {} {} {}", block_index, count,current_index);
                for i in block_index..block_index+count{
                    new_column[i] = 'O';
                }
                new_column[current_index] = '#';
                block_index = current_index+1;
                count = 0 ;
            }
            'O' => {
                new_column[current_index] = '.';
                count+= 1;
            }
            _ => panic!("Invalid char"),
        }

        //println!("sum: {}", sum);
    }
    for i in block_index..block_index+count{
        new_column[i] = 'O';
    }

    new_column
}

fn tilt_west( row: Vec<&char>) -> Vec<char>{
    let mut new_row :Vec<char>= vec!['.';row.len()];
    let mut count = 0;
    let mut block_index = 0;
    let mut current_index = 0;
    for (current_index,&column) in row.iter().enumerate(){
        match column {
            '.' => {},
            '#' => {
                //println!("block: {} {} {}", block_index, count,current_index);
                for i in block_index..block_index+count{
                    new_row[i] = 'O';
                }
                new_row[current_index] = '#';
                block_index = current_index+1;
                count = 0 ;
            }
            'O' => {
                new_row[current_index] = '.';
                count+= 1;
            }
            _ => panic!("Invalid char"),
        }

        //println!("sum: {}", sum);
    }
    for i in block_index..block_index+count{
        new_row[i] = 'O';
    }

    new_row
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 136.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 64.to_string());
    }
}