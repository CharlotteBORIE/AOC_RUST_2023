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
pub fn bench1(){
    let input = include_str!("Input.txt");
    part1_implementation(input);
}
pub fn bench2(){
    let input = include_str!("Input.txt");
    part2_implementation(input);
}

pub fn part1_implementation(input: &str) -> String {
    let mut product = 1;
    let times: Vec <u64> = input.lines().next().unwrap().strip_prefix("Time:").unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let distance: Vec <u64> = input.lines().nth(1).unwrap().strip_prefix("Distance:").unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    for index in 0..times.len() {
        let time = times[index];
        let distance = distance[index];
        product *= formula(time, distance);
    }
    product.to_string()
}

pub fn part2_implementation(input: &str) -> String {
    let mut product = 1;
    let time: u64 = input.lines().next().unwrap().strip_prefix("Time:").unwrap().split_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
    let distance: u64 = input.lines().nth(1).unwrap().strip_prefix("Distance:").unwrap().split_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
    formula(time, distance).to_string()
}

fn formula(time: u64, distance: u64) -> u64 {
    if 4 * distance < time * time {
        let delta = (time * time - 4 * distance) as f64;
        let mut x1 = ((time as f64 - delta.sqrt()) / 2.0);
        let mut x2 = ((time as f64 + delta.sqrt()) / 2.0);
        if x1.fract() == 0.0 {
            x1 += 1.0;
        }
        if x2.fract() == 0.0 {
            x2 -= 1.0;
        }
        return (x2.floor() - x1.ceil() ) as u64 + 1;
    }
    panic!("not implemented")
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