pub fn run() {
    let input = include_str!("input.txt");
    // Only for part b
    let numbers = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let mut sum = 0;
    input.lines().for_each(|line| {
        // Shadow line only for part b
        let line = numbers.iter().fold(line.to_owned(), |acc, (word, digit)| {
            acc.replace(word, digit)
        });

        let mut digits = ('0', '0');
        line.chars().for_each(|c| {
            if digits.0 != '0' {
                return;
            }
            update_digit(&mut digits.0, c);
        });

        line.chars().rev().for_each(|c| {
            if digits.1 != '0' {
                return;
            }
            update_digit(&mut digits.1, c);
        });

        let num = format!("{}{}", digits.0, digits.1);

        sum += num.parse::<i32>().unwrap();
    });

    println!("Sum: {sum}");
}

fn update_digit(digit: &mut char, c: char) {
    if *digit != '0' {
        return;
    };

    match c.to_digit(10) {
        Some(_) => {
            *digit = c;
        }
        None => {}
    }
}
