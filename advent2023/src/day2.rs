#[derive(Default)]
struct Game {
    red: usize,
    blue: usize,
    green: usize,
}

fn parse_game(v: &str) -> Game {
    let mut res = Game::default();
    for part in v.split(", ") {
        let vals: Vec<&str> = part.split(" ").collect();
        if vals[1].contains("blue") {
            res.blue = vals[0].parse::<usize>().unwrap();
        }
        if vals[1].contains("red") {
            res.red = vals[0].parse::<usize>().unwrap();
        }
        if vals[1].contains("green") {
            res.green = vals[0].parse::<usize>().unwrap();
        }
    }
    return res;
}

pub fn day2() {
    let inp: Vec<&str> = include_str!("day2.txt").split("\n").collect();
    let mut games: Vec<Vec<Game>> = Vec::new();
    for line in inp {
        let line = line.split(": ").skip(1).next().unwrap();
        let mut row = Vec::new();
        for part in line.split("; ") {
            row.push(parse_game(part));
        }
        games.push(row);
    }

    // Part 1
    let mut sum = 0;
    for (i, row) in games.iter().enumerate() {
        let mut works = true;
        for game in row {
            if game.red > 12 || game.green > 13 || game.blue > 14 {
                works = false;
                break;
            }
        }
        if works {
            sum += i + 1;
        }
    }
    println!("Part 1: {}", sum);

    // Part 2
    let mut sum = 0;
    for row in games.iter() {
        let mut min = Game::default();
        for g in row {
            if g.red > min.red {
                min.red = g.red;
            }
            if g.blue > min.blue {
                min.blue = g.blue;
            }
            if g.green > min.green {
                min.green = g.green;
            }
        }
        sum += min.red * min.blue * min.green;
    }
    println!("Part 2: {}", sum);
}
