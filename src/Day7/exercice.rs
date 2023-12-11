use itertools::Itertools;
pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 7 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 7 Part 2");
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
    get_points(input,true)
}
pub fn part2_implementation(input: &str) -> String {
    get_points(input,false)
}

fn get_points(input: &str,part1 :bool) -> String {
    let mut sum = 0;
    let mut plays: Vec<(&str, i64)> = vec![];
    for line in input.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let hand = split[0];
        let bid = split[1].parse::<i64>().unwrap();
        plays.push((hand, bid));
    }
    plays.sort_by(|(hand_a, _), (hand_b, _)| compare_hands(hand_b, hand_a, part1));
    plays.iter().enumerate().for_each(|(index, (hand, bid))| {
        sum += bid * (index as i64 + 1);
    });
    sum.to_string()
}

fn compare_hands(hand_a: &str, hand_b: &str,part1 : bool) -> std::cmp::Ordering {
    let type_a = get_hand_type(hand_a,part1);
    let type_b = get_hand_type(hand_b,part1);
    match type_a.cmp(&type_b) {
        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => {  },
    }

    //compare cards
    for (index,char) in hand_a.chars().enumerate() {
        let card_b = hand_b.chars().nth(index).unwrap();
        match compare_card(char,card_b,part1){
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {  },
        }

    }
    println!("Should never happen");
    return std::cmp::Ordering::Equal;
}

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    Pair,
    HighCard,
}

fn get_hand_type(hand: &str, part1 : bool) -> HandType {
    let mut ordered_hand = hand.chars().collect::<Vec<char>>();
    ordered_hand.sort_by(|a,b| b.cmp(a));
    let mut count =vec![];
    for elem in ordered_hand.iter().unique(){
        count.push(ordered_hand.iter().filter(|&n| n == elem && *n!='J').count());
    }
    count.sort_by(|a,b| b.cmp(a));
    if !part1{
    //only part 2
    let jokers = ordered_hand.iter().filter(|&n| *n == 'J').count();
    if jokers == 5 {
        return HandType::FiveOfAKind;
    }
    count[0] += jokers;
    //
    }

    return match count.iter().max() {
        Some(5) => HandType::FiveOfAKind,
        Some(4) => HandType::FourOfAKind,
        Some(3) => {
            if count.iter().filter(|&n| *n == 2).count() == 1 {
                return HandType::FullHouse;
            }
            HandType::ThreeOfAKind
        },
        Some(2) => {
            if count.iter().filter(|&n| *n == 2).count() == 2 {
                return HandType::TwoPairs;
            }
            HandType::Pair
        },
        _ => HandType::HighCard,
    }
}

fn compare_card(card_a: char, card_b: char, part1 : bool) -> std::cmp::Ordering {
    let order = if part1{
         "AKQJT98765432"
    }
    else{
        "AKQT98765432J"
    };
    let index_a = order.find(card_a).unwrap();
    let index_b = order.find(card_b).unwrap();
    index_a.cmp(&index_b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 6440.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 5905.to_string());
    }
}