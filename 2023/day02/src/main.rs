use regex::Regex;

fn main() {
    let input = include_str!("../input2.txt");
    // let input = include_str!("../test_input.txt");
    solve(input)
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;
const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

fn part01(line: &str) -> Option<u32> {
    let mut game_possible: Vec<bool> = Vec::new();

    let games: Vec<&str> = line.split(':').collect();
    games[1].split(';').for_each(|set| {
        let mut results: Vec<bool> = set
            .split(',')
            .map(|cube| {
                let cube_splited = cube.split_whitespace().collect::<Vec<&str>>();
                let number = cube_splited[0].parse::<u32>().unwrap();
                let color = cube_splited[1];

                if (RED == color) && (number <= MAX_RED) {
                    return true;
                }
                if (GREEN == color) && (number <= MAX_GREEN) {
                    return true;
                }
                if (BLUE == color) && (number <= MAX_BLUE) {
                    return true;
                }

                false
            })
            .collect();

        game_possible.append(&mut results)
    });

    if game_possible.iter().all(|&i| i) {
        let re = Regex::new(r"Game (\d+)").unwrap();

        let cap = re.captures(games[0]).unwrap();
        let game_number = &cap[1];
        Some(game_number.parse::<u32>().unwrap())
    } else {
        None
    }
}

fn part02(line: &str) -> Option<u32> {
    let games: Vec<&str> = line.split(':').collect();
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    games[1].split(';').for_each(|set| {
        set.split(',').for_each(|cube| {
            let cube_splited = cube.split_whitespace().collect::<Vec<&str>>();
            let number = cube_splited[0].parse::<u32>().unwrap();
            let color = cube_splited[1];

            if (RED == color) && (number >= min_red) {
                min_red = number;
            }
            if (GREEN == color) && (number >= min_green) {
                min_green = number;
            }
            if (BLUE == color) && (number >= min_blue) {
                min_blue = number;
            }
        })
    });

    Some(min_red * min_green * min_blue)
}

fn solve(input: &str) {
    let mut total = 0;
    input.lines().for_each(|line| {
        if let Some(game) = part02(line) {
            total += game
        }
    });

    println!("sum: {total}")
}
