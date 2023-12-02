fn main() {
    let input = include_str!("../input.txt");
    day01(input)
}

fn day01(input: &str) {
    let mut total = 0;

    input.split("\n").into_iter().for_each(|line| {
        let mut index = 0;
        let line = std::iter::from_fn(move || {
            let reduced = &line[index..];

            let number = if reduced.starts_with("one") {
                Some('1')
            } else if reduced.starts_with("two") {
                Some('2')
            } else if reduced.starts_with("three") {
                Some('3')
            } else if reduced.starts_with("four") {
                Some('4')
            } else if reduced.starts_with("five") {
                Some('5')
            } else if reduced.starts_with("six") {
                Some('6')
            } else if reduced.starts_with("seven") {
               Some('7') 
            } else if reduced.starts_with("eight") {
                Some('8')
            } else if reduced.starts_with("nine") {
                Some('9')
            } else {
                reduced.chars().next()
            };

            index += 1;
            number
        });

        let strnums: Vec<i32> = line
            .filter_map(|x| x.to_string().parse::<i32>().ok())
            .collect();

        if strnums.len() == 0 {
            total +=  0;
            return;
        }

        let first = strnums.first().unwrap();
        let last = strnums.last().unwrap();
        let num = format!("{first}{last}");
        total += num.parse::<i32>().unwrap();
    });

    println!("{total}")
}
