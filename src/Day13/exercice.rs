use grid::Grid;

pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 13 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 13 Part 2");
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
    let mut sum = 0;
    let mut cols = 0;
    let mut vector: Vec<char> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            let mut grid: Grid<char> = Grid::from_vec(vector.clone(), cols);
            display_grid(&grid);
            sum += find_symmetry(&grid);
            vector.clear();
            continue;
        }
        cols = line.chars().count();
        vector.append(&mut line.chars().collect());


    }

    // do last
    let mut grid: Grid<char> = Grid::from_vec(vector.clone(), cols);
    display_grid(&grid);
    sum += find_symmetry(&grid);


    sum.to_string()
}

fn find_symmetry(grid: &Grid<char>) -> usize {
    // try column

    // find first row symetries
    let row = 0;
    let mut col = 0;
    let mut symmetries :Vec<usize> = Vec::new();
    while col < grid.cols() -1 {
        let sym = check_symmetry_column(grid, row, col);
        if sym {
            symmetries.push(col);
        }
        col += 1;
    }

    // check rest of the rows
    let mut possible_symmetries = Vec::new();
    for &col in symmetries.iter(){
        let mut sym = true;
        for row in 1..grid.rows(){
            sym = check_symmetry_column(grid, row, col);
            if !sym {
                break;
            }
        }
        if sym{
            possible_symmetries.push(col);
        }
    }
    if possible_symmetries.len() > 1 {
        panic!("Multiple symmetries found")
    }
    if possible_symmetries.len() == 1 {
        return possible_symmetries[0] + 1;
    }

    // try row
    let mut row = 0;
    let col = 0;
    let mut symmetries :Vec<usize> = Vec::new();
    while row < grid.rows() - 1 {
        let sym = check_symmetry_row(grid, row, col);
        if sym {
            symmetries.push(row);
        }
        row += 1;
    }

    println!("First Symmetry found {:?}", symmetries);

    // check rest of the rows
    let mut possible_symmetries = Vec::new();
    for &row in symmetries.iter(){
        let mut sym=true;
        for col in 1..grid.cols(){
            sym = check_symmetry_row(grid, row, col);
            if !sym {
                break;
            }
        }
        if sym{
            possible_symmetries.push(row);
        }
    }
    if possible_symmetries.len() > 1 {
        panic!("Multiple symmetries found")
    }
    if possible_symmetries.len() == 0 {
        panic!("No symmetries found")
    }
    println!("Symmetry found {:?}", possible_symmetries);

    (possible_symmetries[0]+1) * 100
}

fn check_symmetry_column(grid: &Grid<char>, row: usize, col: usize) -> bool {
    let mut sym = true;
    for offset in 0..col+1 {
        if col + offset + 1 >= grid.cols() {
            break;
        }
        if grid[(row, col + offset+1)] != grid[(row, col - offset)] {
            sym = false;
            break;
        }
    }
    sym
}

fn check_symmetry_row(grid: &Grid<char>, row: usize, col: usize) -> bool {
    let mut sym = true;
    for offset in 0..row+1 {
        if row + offset + 1 >= grid.rows() {
            break;
        }
        if grid[(row + offset + 1, col)] != grid[(row - offset, col )] {
            sym = false;
            break;
        }
    }
    sym
}

fn display_grid(p0: &Grid<char>) {
    for row in p0.iter_rows() {
        for elm in row {
            print!("{}", elm);
        }
        println!(" ");
    }
    println!(" ");
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
        assert_eq!(part1_implementation(input), 405.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 0.to_string());
    }
}