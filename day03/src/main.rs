use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    let part2 = part2(input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

#[derive(Hash, PartialEq, Eq)]
struct PartNumber {
    line: usize,
    number_start: usize,
    number_end: usize,
}

fn part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let lines_count = lines.len();
    let columns_count = input.lines().next().unwrap().len();

    let mut grid = vec![vec![char::default(); columns_count]; lines_count];
    let mut sign_positions = vec![];
    for (i, line) in lines.iter().enumerate() {
        for (j, element) in line.chars().enumerate() {
            grid[i][j] = element;

            if !element.is_ascii_digit() && element != '.' {
                sign_positions.push((i, j));
            }
        }
    }

    let part_numbers = get_part_locations(sign_positions, &grid, columns_count);

    part_numbers
        .iter()
        .map(|x| {
            grid[x.line][x.number_start..=x.number_end]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn get_part_locations(
    sign_positions: Vec<(usize, usize)>,
    grid: &Vec<Vec<char>>,
    columns_count: usize,
) -> HashSet<PartNumber> {
    let mut part_numbers = HashSet::new();

    for (i, j) in sign_positions {
        for i_change in -1..=1 {
            for j_change in -1..=1 {
                if i_change == 0 && j_change == 0 {
                    continue;
                }
                let new_i = i.wrapping_add_signed(i_change);
                let new_j = j.wrapping_add_signed(j_change);
                if grid[new_i][new_j].is_ascii_digit() {
                    let mut number_start = Some(new_j);
                    let mut number_end = new_j;

                    loop {
                        if let Some(new_start) = number_start {
                            if !grid[new_i][new_start].is_ascii_digit() {
                                number_start = Some(new_start + 1);
                                break;
                            } else {
                                number_start = new_start.checked_sub(1);
                                if number_start.is_none() {
                                    number_start = Some(new_start);
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }

                    while number_end < columns_count {
                        if !grid[new_i][number_end].is_ascii_digit() {
                            number_end = number_end - 1;
                            break;
                        } else {
                            if number_end == columns_count - 1 {
                                break;
                            }
                            number_end = number_end + 1;
                        }
                    }

                    part_numbers.insert(PartNumber {
                        line: new_i,
                        number_start: number_start.unwrap(),
                        number_end,
                    });
                }
            }
        }
    }
    part_numbers
}

fn part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let lines_count = lines.len();
    let columns_count = input.lines().next().unwrap().len();

    let mut grid = vec![vec![char::default(); columns_count]; lines_count];
    let mut sign_positions = vec![];
    for (i, line) in lines.iter().enumerate() {
        for (j, element) in line.chars().enumerate() {
            grid[i][j] = element;

            if element == '*' {
                sign_positions.push((i, j));
            }
        }
    }

    let gear_ratios = get_gear_ratios(sign_positions, &grid, columns_count);
    gear_ratios.iter().sum()
}

fn get_gear_ratios(
    sign_positions: Vec<(usize, usize)>,
    grid: &Vec<Vec<char>>,
    columns_count: usize,
) -> Vec<usize> {
    let mut gear_ratios = vec![];

    for (i, j) in sign_positions {
        let mut adjacent_numbers = HashSet::new();
        for i_change in -1..=1 {
            for j_change in -1..=1 {
                if i_change == 0 && j_change == 0 {
                    continue;
                }
                let new_i = i.wrapping_add_signed(i_change);
                let new_j = j.wrapping_add_signed(j_change);
                if grid[new_i][new_j].is_ascii_digit() {
                    let mut number_start = Some(new_j);
                    let mut number_end = new_j;

                    loop {
                        if let Some(new_start) = number_start {
                            if !grid[new_i][new_start].is_ascii_digit() {
                                number_start = Some(new_start + 1);
                                break;
                            } else {
                                number_start = new_start.checked_sub(1);
                                if number_start.is_none() {
                                    number_start = Some(new_start);
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }

                    while number_end < columns_count {
                        if !grid[new_i][number_end].is_ascii_digit() {
                            number_end = number_end - 1;
                            break;
                        } else {
                            if number_end == columns_count - 1 {
                                break;
                            }
                            number_end = number_end + 1;
                        }
                    }

                    let number = grid[new_i][number_start.unwrap()..=number_end]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();

                    adjacent_numbers.insert(number);
                }
            }
        }
        if adjacent_numbers.len() == 2 {
            let mut iter = adjacent_numbers.iter();
            gear_ratios.push(iter.next().unwrap() * iter.next().unwrap());
        }
    }
    gear_ratios
}
