pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 15 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 15 Part 2");
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
    let lines = input.split(",").collect::<Vec<&str>>();
    for line in lines {
        let mut number = 0;
        for char in line.chars() {
            number = update_number(number, char);
        }

        sum += number;
    }
    sum.to_string()
}

fn update_number(number: i32, char: char) -> i32 {
    let mut new_number = number + char as i32;
    new_number *= 17;
    new_number %= 256;
    new_number
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
        assert_eq!(part1_implementation(input), 1320.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 0.to_string());
    }
}