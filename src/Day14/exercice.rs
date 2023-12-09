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

pub fn part1_implementation(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let number = line.parse::<i32>().unwrap();
        sum += number;
    }
    sum.to_string()
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
        assert_eq!(part1_implementation(input), 1.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 0.to_string());
    }
}