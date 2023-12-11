pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 11 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 11 Part 2");
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
    get_galaxy_distance(input,1)
}

pub fn part2_implementation(input: &str) -> String {
    get_galaxy_distance(input,1000000)
}

fn get_galaxy_distance(input: &str,expansion : usize) -> String {
    let expansion = expansion -1;
    let mut sum = 0;
    let mut galaxies = Vec::new();
    let mut expanded_row = Vec::new();
    let mut row: usize = 0;
    for line in input.lines() {
        let number: Vec<usize> = line.char_indices().filter(|(index, value)| *value == '#').map(|(index, value)| index).collect();
        if number.len() == 0 {
            expanded_row.push(row);
            row += 1;
            continue;
        } else {
            for column in number {
                galaxies.push((row, column));
            }
        }
        row += 1;
    }

    let max_column = *galaxies.iter().map(|(row, column)| column).max().unwrap();
    let min_column = *galaxies.iter().map(|(row, column)| column).min().unwrap();
    let mut expanded_column = (min_column..max_column).collect::<Vec<usize>>().iter().filter(|empty| !galaxies.iter().any(|(_row, column)| column == *empty)).map(|column| *column).collect::<Vec<usize>>();
    //expand
    for galaxy in galaxies.iter_mut() {
        let row_to_add = expanded_row.iter().filter(|row| **row < galaxy.0).count();
        galaxy.0 += row_to_add * expansion;
        let column_to_add = expanded_column.iter().filter(|column| **column < galaxy.1).count();
        galaxy.1 += column_to_add * expansion;
    }
    //get min distance
    let min_distance = galaxies.iter().map(|&position| galaxies.iter().map(move |&other| get_distance(position, other))).flatten().sum::<usize>() / 2;

    min_distance.to_string()
}

fn get_distance(p0: (usize, usize), p1: (usize, usize)) -> usize {
    p0.1.abs_diff(p1.1) + p0.0.abs_diff(p1.0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 374.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(get_galaxy_distance(input,10), 1030.to_string());
        assert_eq!(get_galaxy_distance(input,100), 8410.to_string());
    }
}