use grid::Grid;
use std::any::TypeId;
use std::cell::Cell;
use std::fmt::{Display, Formatter};

pub fn part1() {
    let input = include_str!("Input.txt");
    println!("Day 10 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2() {
    let input = include_str!("Input.txt");
    println!("Day 10 Part 2");
    println!("{:?}", part2_implementation(input));
}

pub fn bench() {
    let input = include_str!("Input.txt");
    part1_implementation(input);
    part2_implementation(input);
}
pub fn part1_implementation(input: &str) -> String {
    let cols = input.lines().next().unwrap().len();
    let mut grid: Grid<Pipe> = Grid::from_vec(
        input
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| get_pipe(&c))
            .collect(),
        cols,
    );
    let grid_distance: Grid<i32> = get_distance_grid(&grid);
    grid_distance.iter().max().unwrap().to_string()
}

pub fn part2_implementation(input: &str) -> String {
    let cols = input.lines().next().unwrap().len();
    let grid: Grid<Pipe> = Grid::from_vec(
        input
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| get_pipe(&c))
            .collect(),
        cols,
    );
    let grid = get_piped_grid(&grid);
    let internal_islands = get_internal_island(&grid, &grid);

    internal_islands
        .iter()
        .filter(|&c| check_is_in_chain(&grid, c.clone()))
        .map(|c| c.len())
        .sum::<usize>()
        .to_string()
}

fn get_distance_grid(grid: &Grid<Pipe>) -> Grid<i32> {
    //find start index and set start pipe
    let binding = grid
        .indexed_iter()
        .filter(|(index, &c)| c == Pipe::Start)
        .map(|(index, &c)| index)
        .collect::<Vec<(usize, usize)>>();
    let start_index = binding.first().unwrap();
    let start_pipe = get_start_pipe(grid, *start_index);
    let mut grid = grid.clone();
    let start_cell = grid.get_mut(start_index.0, start_index.1).unwrap();
    *start_cell = start_pipe;
    let mut grid = grid.clone();
    let mut new_grid = Grid::new(grid.rows(), grid.cols());
    let mut distance = 0;

    set(&mut new_grid, start_index.0, start_index.1, 1);

    let mut neighbours = get_neighbours(&grid, *start_index, None);
    let mut changed = true;

    while changed {
        distance += 1;

        changed = false;
        let mut temp_neighbours = vec![];
        for (neighbour_cell, previous) in neighbours {
            let distance_cell = new_grid
                .get_mut(neighbour_cell.0, neighbour_cell.1)
                .unwrap();
            if *distance_cell == 0 {
                set(&mut new_grid, neighbour_cell.0, neighbour_cell.1, distance);
                changed = true;
            }
            temp_neighbours.append(&mut get_neighbours(&grid, neighbour_cell, previous));
        }
        neighbours = temp_neighbours;
    }
    new_grid
}

fn get_piped_grid(grid: &Grid<Pipe>) -> Grid<Pipe> {
    //find start index and set start pipe
    let binding = grid
        .indexed_iter()
        .filter(|(index, &c)| c == Pipe::Start)
        .map(|(index, &c)| index)
        .collect::<Vec<(usize, usize)>>();
    let start_index = binding.first().unwrap();
    let start_pipe = get_start_pipe(grid, *start_index);
    let mut grid = grid.clone();
    let start_cell = grid.get_mut(start_index.0, start_index.1).unwrap();
    *start_cell = start_pipe;
    let mut new_grid = Grid::new(grid.rows(), grid.cols());

    let mut distance = 0;

    set(&mut new_grid, start_index.0, start_index.1, start_pipe);

    let mut neighbours = get_neighbours(&grid, *start_index, None);
    let mut changed = true;

    while changed {
        distance += 1;

        changed = false;
        let mut temp_neighbours = vec![];
        for (neighbour_cell, previous) in neighbours {
            let distance_cell = new_grid
                .get_mut(neighbour_cell.0, neighbour_cell.1)
                .unwrap();
            if *distance_cell == Pipe::Ground {
                set::<Pipe>(
                    &mut new_grid,
                    neighbour_cell.0,
                    neighbour_cell.1,
                    grid.get(neighbour_cell.0, neighbour_cell.1)
                        .unwrap()
                        .clone(),
                );
                changed = true;
            }
            temp_neighbours.append(&mut get_neighbours(&grid, neighbour_cell, previous));
        }
        neighbours = temp_neighbours;
    }
    new_grid
}

fn set<T>(grid: &mut Grid<T>, row: usize, column: usize, value: T) {
    let cell = grid.get_mut(row, column).unwrap();
    *cell = value;
}

fn get_neighbours(
    grid: &Grid<Pipe>,
    cell: (usize, usize),
    previous: Option<(usize, usize)>,
) -> Vec<((usize, usize), Option<(usize, usize)>)> {
    let pipe = grid.get(cell.0, cell.1).unwrap();
    let top = if cell.0 == 0 {
        None
    } else {
        if previous.is_some() && previous.unwrap() == (cell.0 - 1, cell.1) {
            None
        } else {
            Some((cell.0 - 1, cell.1))
        }
    };
    let bottom = if previous.is_some() && previous.unwrap() == (cell.0 + 1, cell.1) {
        None
    } else {
        Some((cell.0 + 1, cell.1))
    };
    let left = if cell.1 == 0 {
        None
    } else {
        if previous.is_some() && previous.unwrap() == (cell.0, cell.1 - 1) {
            None
        } else {
            Some((cell.0, cell.1 - 1))
        }
    };
    let right = if previous.is_some() && previous.unwrap() == (cell.0, cell.1 + 1) {
        None
    } else {
        Some((cell.0, cell.1 + 1))
    };
    match pipe {
        Pipe::NorthSouth => get_valid_neighbour(vec![top, bottom], cell),
        Pipe::EastWest => get_valid_neighbour(vec![left, right], cell),
        Pipe::NorthEast => get_valid_neighbour(vec![top, right], cell),
        Pipe::NorthWest => get_valid_neighbour(vec![top, left], cell),
        Pipe::SouthEast => get_valid_neighbour(vec![bottom, right], cell),
        Pipe::SouthWest => get_valid_neighbour(vec![bottom, left], cell),
        Pipe::Start => panic!("Start"),
        Pipe::Ground => panic!("Ground"),
    }
}

fn get_valid_neighbour(
    list: Vec<Option<(usize, usize)>>,
    cell: (usize, usize),
) -> Vec<((usize, usize), Option<(usize, usize)>)> {
    list.iter()
        .filter(|&c| c.is_some())
        .map(|c| (c.unwrap(), Some(cell)))
        .collect::<Vec<((usize, usize), Option<(usize, usize)>)>>()
}

fn get_start_pipe(grid: &Grid<Pipe>, cell: (usize, usize)) -> Pipe {
    let top = if cell.0 == 0 {
        false
    } else {
        match grid.get(cell.0 - 1, cell.1).unwrap() {
            Pipe::NorthSouth => true,
            Pipe::SouthEast => true,
            Pipe::SouthWest => true,
            _ => false,
        }
    };
    let bottom = match grid.get(cell.0 + 1, cell.1).unwrap() {
        Pipe::NorthSouth => true,
        Pipe::NorthEast => true,
        Pipe::NorthWest => true,
        _ => false,
    };
    let left = if cell.1 == 0 {
        false
    } else {
        match grid.get(cell.0, cell.1 - 1).unwrap() {
            Pipe::EastWest => true,
            Pipe::SouthEast => true,
            Pipe::NorthEast => true,
            _ => false,
        }
    };
    let right = match grid.get(cell.0, cell.1 + 1).unwrap() {
        Pipe::EastWest => true,
        Pipe::SouthWest => true,
        Pipe::NorthWest => true,
        _ => false,
    };
    match (top, bottom, left, right) {
        (true, true, false, false) => Pipe::NorthSouth,
        (false, false, true, true) => Pipe::EastWest,
        (true, false, true, false) => Pipe::NorthWest,
        (true, false, false, true) => Pipe::NorthEast,
        (false, true, true, false) => Pipe::SouthWest,
        (false, true, false, true) => Pipe::SouthEast,
        _ => panic!("Invalid start pipe"),
    }
}

fn get_internal_island(grid: &Grid<Pipe>, grid_pipe: &Grid<Pipe>) -> Vec<Vec<(usize, usize)>> {
    let mut non_path_list = Vec::new();
    for ((row, column), cell) in grid.indexed_iter() {
        if *cell != Pipe::Ground {
            continue;
        }

        // try to find island blocks
        let mut new_island = vec![(row, column)];
        let mut temp_list = Vec::new();
        let mut island_option = non_path_list.pop();
        while island_option.is_some() {
            let mut connected = false;
            let mut island: Vec<(usize, usize)> = island_option.unwrap();
            for (row1, column1) in island.iter() {
                if (column == *column1 && (row + 1 == *row1 || (row - 1 == *row1)))
                    || (row == *row1 && (column + 1 == *column1 || (column - 1 == *column1)))
                {
                    connected = true;
                    break;
                }
            }

            if connected {
                new_island.append(&mut island);
            } else {
                temp_list.push(island);
            }
            island_option = non_path_list.pop();
        }

        temp_list.push(new_island);
        non_path_list = temp_list;
    }
    if non_path_list.len() == 1 {
        return vec![];
    } else if non_path_list.len() == 0 {
        panic!("No island found ");
    }

    let internal_island: Vec<Vec<(usize, usize)>> = non_path_list
        .iter()
        .filter(|&c| c.iter().all( |&(row, column)| row != 0 && column != 0 && row != grid.rows() - 1 && column != grid.cols() - 1))
        .map(|c| {
            let mut vec = vec![];
            for elem in c {
                vec.push(*elem);
            }
            vec
        })
        .collect();

    internal_island
}

fn check_is_in_chain(grid: &Grid<Pipe>, island: Vec<(usize, usize)>) -> bool {
    for (row, column) in island {
        //check top
        let top = check_row_for_trapped_island(grid, column, 0, row);

        //check bottom
        let bottom = check_row_for_trapped_island(grid, column, row + 1, grid.rows());

        //check left
        let left = check_column_for_trapped_island(grid, row, 0, column);

        //check right
        let right = check_column_for_trapped_island(grid, row, column + 1, grid.cols());

        if !top  || !bottom || !left  || !right  {
            return false;
        }
    }

    true
}

fn check_row_for_trapped_island(grid: &Grid<Pipe>, column: usize,start:usize, end : usize) -> bool {
    let mut straight:i8 = 0;
    let mut angle_east:i8 = 0;
    let mut angle_west:i8 = 0;


    for top_cell in start..end {
        let pipe = grid.get(top_cell, column).unwrap();
        match pipe {
            Pipe::EastWest => straight += 1,
            Pipe::NorthWest => angle_west += 1,
            Pipe::SouthWest => angle_west -= 1,
            Pipe::NorthEast => angle_east += 1,
            Pipe::SouthEast => angle_east -= 1,
            _ => (),
        }
    }
    if (angle_east % 2 == 1 || angle_west % 2 == 1) && angle_east + angle_west % 2 == 1 {
        return false
    }
    straight += (angle_east.abs() + angle_west.abs()) / 2;
    if straight % 2 == 0 {
        return false
    }
    true
}

fn check_column_for_trapped_island(grid: &Grid<Pipe>, row: usize,start:usize, end : usize) -> bool {
    let mut straight:i8 = 0;
    let mut angle_top:i8 = 0;
    let mut angle_down:i8 = 0;

    for cell in start..end {
        let pipe = grid.get(row, cell).unwrap();
        match pipe {
            Pipe::NorthSouth => straight += 1,
            Pipe::NorthWest => angle_top += 1,
            Pipe::NorthEast => angle_top -= 1,
            Pipe::SouthWest => angle_down += 1,
            Pipe::SouthEast => angle_down -= 1,
            _ => (),
        }
    }
    if (angle_top % 2 == 1 || angle_down % 2 == 1) && angle_top + angle_down % 2 == 1 {
        return false
    }
    straight += (angle_top.abs() + angle_down.abs()) / 2;
    if straight % 2 == 0 {
        return false
    }
    true
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    #[default]
    Ground,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Pipe::NorthSouth => '|',
            Pipe::EastWest => '-',
            Pipe::NorthEast => 'L',
            Pipe::NorthWest => 'J',
            Pipe::SouthEast => 'F',
            Pipe::SouthWest => '7',
            Pipe::Start => 'S',
            Pipe::Ground => '.',
        };
        write!(f, "{}", char)
    }
}

fn get_pipe(c: &char) -> Pipe {
    match c {
        'S' => Pipe::Start,
        '|' => Pipe::NorthSouth,
        '-' => Pipe::EastWest,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::Ground,
        char => panic!("Unknown char {}",char),
    }
}

fn display_grid(grid: &Grid<i32>) {
    for row in 0..grid.rows() {
        for column in 0..grid.cols() {
            let cell = grid.get(row, column).unwrap();
            if *cell >= 10 {
                print!("{} ", cell);
            } else {
                print!(" {} ", cell);
            }
        }
        println!();
    }
}

fn display_piped_grid(grid: &Grid<Pipe>) {
    for row in 0..grid.rows() {
        for column in 0..grid.cols() {
            let cell = grid.get(row, column).unwrap();
            print!("{}", cell);

        }
        println!();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 4.to_string());

        let input = include_str!("Example2.txt");
        assert_eq!(part1_implementation(input), 8.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example3.txt");
        assert_eq!(part2_implementation(input), 4.to_string());

        let input = include_str!("Example4.txt");
        assert_eq!(part2_implementation(input), 8.to_string());

        let input = include_str!("Example5.txt");
        assert_eq!(part2_implementation(input), 10.to_string());
    }
}
