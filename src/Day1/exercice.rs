pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 1 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day1 Part 2");
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
        let numbers = line.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
        println!("{:?}", numbers);
        let mut a_string = String::from("");
        a_string.push(*numbers.first().unwrap());
        a_string.push(*numbers.last().unwrap());
        sum += a_string.parse::<u128>().unwrap();
    }
    sum.to_string()
}

pub fn part2_implementation(input: &str) -> String {
    let words = vec![String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six"), String::from("seven"), String::from("eight"), String::from("nine"),String::from("zero"),
    String::from("1"),String::from("2"), String::from("3"),String::from("4"),String::from("5"),String::from("6"),String::from("7"),String::from("8"),String::from("9"),String::from("0"),];
    let mut sum = 0;
    for line in input.lines() {
        let string = line.parse::<String>().unwrap();
        let mut first_index=string.len();
        let mut first_number =0 ;
        let mut last_index= 0;
        let mut last_number =0 ;
        for word in words.iter() {
            let new_index_last =string.rfind(word);
            if new_index_last.is_some() {
                if last_index==0 || new_index_last.unwrap() > last_index {
                    last_index = new_index_last.unwrap();
                    last_number = get_number(word);
                }
            }
            let new_index_first =string.find(word);
           if new_index_first.is_some() {
            if new_index_first.unwrap() < first_index {
                first_index = new_index_first.unwrap();
                first_number = get_number(word);
            }
           }
        }

        sum+= first_number *10 + last_number;
    }
    sum.to_string()
}

fn get_number(word: &str) -> u32 {
    match word {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" |"8" => 8,
        "nine" | "9" => 9,
        "zero" | "0" => 0,
        _=> panic!("Not a number")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 142.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example2.txt");
        assert_eq!(part2_implementation(input), 281.to_string());
    }
}