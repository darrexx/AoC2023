use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let part1 = part1(&times, &distances);
    println!("part1: {}", part1);

    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse()
        .unwrap();

    let part2 = part2(time, distance);
    println!("part2: {}", part2);
}

fn part1(times: &Vec<usize>, distances: &Vec<usize>) -> usize {
    let mut result = 1;
    for (i, time) in times.iter().enumerate() {
        let distance_to_beat = distances[i];
        let (ways_to_win_start, ways_to_win_end) = find_ways_to_win(*time, distance_to_beat);
        result = result * (ways_to_win_start..=ways_to_win_end).count();
    }
    result
}

fn find_ways_to_win(time: usize, distance_to_beat: usize) -> (usize, usize) {
    let mut ways_to_win_start = 0;
    for speed in 0..time {
        let travel_time = time - speed;
        let distance = speed * travel_time;
        if distance > distance_to_beat {
            ways_to_win_start = speed;
            break;
        }
    }

    let mut ways_to_win_end = usize::MAX;
    for speed in (0..time).rev() {
        let travel_time = time - speed;
        let distance = speed * travel_time;
        if distance > distance_to_beat {
            ways_to_win_end = speed;
            break;
        }
    }
    (ways_to_win_start, ways_to_win_end)
}

fn part2(time: usize, distance: usize) -> usize {
    let (ways_to_win_start, ways_to_win_end) = find_ways_to_win(time, distance);

    (ways_to_win_start..=ways_to_win_end).count()
}
