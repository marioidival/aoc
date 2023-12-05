use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    solve(input)
}
fn solve(input: &str) {
    part02(input)
}

fn part01(input: &str) {
    let mut ans = 0;
    input.lines().into_iter().for_each(|line| {
        let row: Vec<&str> = line.split(":").collect();
        let numbers: Vec<&str> = row[1].split("|").collect();
        let winners: HashSet<u32> = numbers[0]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let my_numbers: HashSet<u32> = numbers[1]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let intersection: Vec<u32> = winners.intersection(&my_numbers).copied().collect();
        if intersection.len() > 0 {
            ans += u32::pow(2, intersection.len() as u32 - 1)
        }
        println!("{intersection:?}");
    });

    println!("{ans}")
}

fn part02(input: &str) {
    let mut ans = 0;
    let mut played = vec![0; input.lines().count()];

    input.lines().into_iter().enumerate().for_each(|(i, line)| {
        played[i] += 1;

        let row: Vec<&str> = line.split(":").collect();
        let numbers: Vec<&str> = row[1].split("|").collect();
        let winners: HashSet<u32> = numbers[0]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let my_numbers: HashSet<u32> = numbers[1]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let intersection: Vec<u32> = winners.intersection(&my_numbers).copied().collect();
        if intersection.len() > 0 {
            for w in 0..intersection.len() {
                played[i + w + 1] += played[i];
            }
        }
    });

    println!("{}", played.into_iter().sum::<u32>())
}
