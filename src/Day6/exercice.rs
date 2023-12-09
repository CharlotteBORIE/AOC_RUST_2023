pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 6 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 6 Part 2");
    println!("{:?}", part2_implementation(input));
}

pub fn bench(){
    let input = include_str!("Input.txt");
    part1_implementation(input);
    part2_implementation(input);
}

pub fn part1_implementation(input: &str) -> String {
    let mut product = 1;
    let times: Vec <i32> = input.lines().next().unwrap().strip_prefix("Time:").unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let distance: Vec <i32> = input.lines().nth(1).unwrap().strip_prefix("Distance:").unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    for index in 0..times.len() {
        let time = times[index];
        let distance = distance[index];
        let mut count = 0;
        for time_pressed in 0..time{
            if time_pressed* (time - time_pressed) > distance {
                count += 1;
            }
        }
        product *= count;
    }
    product.to_string()
}

pub fn part2_implementation(input: &str) -> String {
    let mut product = 1;
    let time: u64 = input.lines().next().unwrap().strip_prefix("Time:").unwrap().split_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
    let distance: u64 = input.lines().nth(1).unwrap().strip_prefix("Distance:").unwrap().split_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
        let mut count = 0;
        for time_pressed in 0..time{
            if time_pressed* (time - time_pressed) > distance {
                count += 1;
            }
        }
    product *= count;
    product.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 288.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 71503.to_string());
    }
}