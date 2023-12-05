#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

pub fn run() {
    let input = include_str!("input.txt");

    let mut total_power: usize = 0;


    input.lines().for_each(|line| {
        let game = line
            .split_whitespace()
            .into_iter()
            .skip(2)
            .collect::<Vec<&str>>()
            .join(" ");

        let sets_to_check: Vec<&str> = game.split(';').map(|set| set.trim()).collect();

        let mut game_set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        sets_to_check.iter().for_each(|set| {
            let colors = set.split(", ").collect::<Vec<&str>>();

            colors.into_iter().for_each(|color| {
                let color = color.split_whitespace().collect::<Vec<&str>>();

                match color[1] {
                    "red" => {
                        let value_to_check: usize = color[0].parse::<usize>().unwrap();
                        if game_set.red < value_to_check {
                            game_set.red = value_to_check;
                        }
                    },
                    "green" => {
                        let value_to_check: usize = color[0].parse::<usize>().unwrap();
                        if game_set.green < value_to_check {
                            game_set.green = value_to_check;
                        }
                    }
                    "blue" => {
                        let value_to_check:usize = color[0].parse::<usize>().unwrap();
                        if game_set.blue < value_to_check {
                            game_set.blue = value_to_check;
                        }
                    }
                    _ => {}
                };
            });
        });

        let power: usize  = game_set.red * game_set.green * game_set.blue;

        total_power += power;

    });

    println!("Total power sum: {total_power}");
}

pub fn run_a() {
    let input = include_str!("input_short.txt");

    let mut valid_games: usize = 0;

    input.lines().for_each(|line| {
        let game = line
            .split_whitespace()
            .into_iter()
            .skip(2)
            .collect::<Vec<&str>>()
            .join(" ");

        let game_number = line
            .split_whitespace()
            .into_iter()
            .nth(1)
            .unwrap()
            .replace(":", "")
            .parse::<usize>()
            .unwrap();

        let mut valid_game = true;

        let sets_to_check: Vec<&str> = game.split(';').map(|set| set.trim()).collect();

        sets_to_check.iter().for_each(|set| {
            if !valid_game {
                return;
            }

            let colors = set.split(", ").collect::<Vec<&str>>();

            let mut set = Set {
                red: 0,
                green: 0,
                blue: 0,
            };

            colors.into_iter().for_each(|color| {
                let color = color.split_whitespace().collect::<Vec<&str>>();

                match color[1] {
                    "red" => set.red += color[0].parse::<usize>().unwrap(),
                    "green" => set.green += color[0].parse::<usize>().unwrap(),
                    "blue" => set.blue += color[0].parse::<usize>().unwrap(),
                    _ => {}
                };
            });

            if !validate_set(&set) {
                valid_game = false;
            }
        });
        if valid_game {
            valid_games += game_number;
        }
    });

    println!("Valid games sum: {}", valid_games);
}

fn validate_set(set: &Set) -> bool {
    let mut valid = true;

    if set.red > 12 || set.green > 13 || set.blue > 14 {
        valid = false;
    }

    valid
}
