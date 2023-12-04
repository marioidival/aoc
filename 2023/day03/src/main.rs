fn main() {
    let input = include_str!("../input2.txt");
    solve(input)
}

fn is_symbol(lines: &Vec<&str>, row: usize, col: usize) -> bool {
    if !(row < lines.len() && col < lines[0].len()) {
        return false;
    }

    let line = lines[row];
    let chars = line.chars().collect::<Vec<char>>();

    chars[col] != '.' && !chars[col].is_numeric()
}

fn collect_goods(
    lines: &Vec<&str>,
    row: usize,
    col: usize,
    goods: &mut Vec<Vec<Vec<u32>>>,
    n: u32,
) -> bool {
    if !(row < lines.len() && col < lines[0].len()) {
        return false;
    }

    let line = lines[row];
    let chars = line.chars().collect::<Vec<char>>();

    if chars[col] == '*' {
        goods[row][col].push(n)
    }

    chars[col] != '.' && !chars[col].is_numeric()
}

fn solve(input: &str) {
    part02(input)
}

fn part01(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let rows = lines.len();
    let columns = lines[0].len();
    let mut ans = 0;

    for (i, line) in lines.iter().enumerate() {
        let mut start = 0;
        let mut j = 0;

        while j < columns {
            start = j;
            let mut nums = String::new();

            let chars = line.chars().collect::<Vec<char>>();

            while j < columns && chars[j].is_numeric() {
                nums.push(chars[j]);
                j += 1;
            }

            if nums.is_empty() {
                j += 1;
                continue;
            }

            if is_symbol(&lines, i, if start == 0 { start } else { start - 1 })
                || is_symbol(&lines, i, j)
            {
                ans += nums.parse::<u32>().unwrap();
            }

            for k in if start == 0 { start } else { start - 1 }..j + 1 {
                if is_symbol(&lines, if i == 0 { i } else { i - 1 }, k)
                    || is_symbol(&lines, i + 1, k)
                {
                    ans += nums.parse::<u32>().unwrap();
                    break;
                }
            }
        }
    }

    println!("{ans}")
}

fn part02(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let rows = lines.len();
    let columns = lines[0].len();
    let mut goods: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; columns]; rows]; columns];

    for (i, line) in lines.iter().enumerate() {
        let mut start = 0;
        let mut j = 0;

        while j < columns {
            start = j;
            let mut nums = String::new();

            let chars = line.chars().collect::<Vec<char>>();

            while j < columns && chars[j].is_numeric() {
                nums.push(chars[j]);
                j += 1;
            }

            if nums.is_empty() {
                j += 1;
                continue;
            }

            let n = nums.parse::<u32>().unwrap();

            if collect_goods(
                &lines,
                i,
                if start == 0 { start } else { start - 1 },
                &mut goods,
                n,
            ) || collect_goods(&lines, i, j, &mut goods, n)
            {}

            for k in if start == 0 { start } else { start - 1 }..j + 1 {
                if collect_goods(&lines, if i == 0 { i } else { i - 1 }, k, &mut goods, n)
                    || collect_goods(&lines, i + 1, k, &mut goods, n)
                {
                    break;
                }
            }
        }
    }

    let mut ans = 0;

    for i in 0..rows {
        for j in 0..columns {
            let nums = &goods[i][j];
            let ns: Vec<u32> = nums
                .into_iter()
                .filter_map(|x| if *x != 0u32 { Some(*x) } else { None })
                .collect();
            if ns.len() == 2 {
                ans += ns[0] * ns[1];
                continue
            }
        }
    }

    println!("{ans}")
}
