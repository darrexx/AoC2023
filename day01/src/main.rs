use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let part1: u32 = part1(input);
    let part2: u32 = part2(input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| x.chars())
        .map(|x| x.filter(|y| y.is_ascii_digit()))
        .map(|x| format!("{}{}", x.clone().next().unwrap(), x.last().unwrap()))
        .map(|x| x.parse::<u32>().unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(get_numbers_of_line)
        .map(|x| format!("{}{}", x.first().unwrap(), x.last().unwrap()))
        .map(|x| x.parse::<u32>().unwrap())
        .sum()
}

fn get_numbers_of_line(line: &str) -> Vec<String> {
    let mut text_to_number_hashmap = HashMap::<&str, &str>::new();
    text_to_number_hashmap.insert("zero", "0");
    text_to_number_hashmap.insert("one", "1");
    text_to_number_hashmap.insert("two", "2");
    text_to_number_hashmap.insert("three", "3");
    text_to_number_hashmap.insert("four", "4");
    text_to_number_hashmap.insert("five", "5");
    text_to_number_hashmap.insert("six", "6");
    text_to_number_hashmap.insert("seven", "7");
    text_to_number_hashmap.insert("eight", "8");
    text_to_number_hashmap.insert("nine", "9");

    let mut numbers: Vec<String> = Vec::new();

    for i in 0..line.len() {
        if line.chars().nth(i).is_some_and(|x| x.is_ascii_digit()) {
            numbers.push(line.chars().nth(i).unwrap().into());
        } else {
            for (key, value) in text_to_number_hashmap.iter() {
                if line[i..].starts_with(key) {
                    numbers.push((*value).into())
                }
            }
        }
    }
    numbers
}
