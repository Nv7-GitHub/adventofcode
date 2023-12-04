pub fn day3() {
    let inp: Vec<Vec<char>> = include_str!("day3.txt")
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut sum = 0;
    let mut ratios = vec![vec![Vec::new(); inp[0].len()]; inp.len()];
    for (r, line) in inp.iter().enumerate() {
        let mut c = 0;
        while c < line.len() {
            if line[c].is_ascii_digit() {
                let mut num = "".to_string();
                /*
                // Part 1
                let mut adjacent = false;
                while c < line.len() && line[c].is_ascii_digit() {
                    num += (line[c] as char).to_string().as_str();
                    if !adjacent && is_adjacent(r as i32, c as i32, &inp) {
                        adjacent = true;
                    }
                    c += 1;
                }
                if adjacent {
                    sum += num.parse::<usize>().unwrap();
                }*/

                // Part 2
                let mut adjacent = Vec::new();
                while c < line.len() && line[c].is_ascii_digit() {
                    num += (line[c] as char).to_string().as_str();
                    let res = is_adjacent_gear(r as i32, c as i32, &inp);
                    if res.is_some() && !adjacent.contains(&res.unwrap()) {
                        adjacent.push(res.unwrap());
                    }
                    c += 1;
                }
                for v in adjacent {
                    ratios[v.0][v.1].push(num.parse::<usize>().unwrap());
                }
            }
            c += 1;
        }
    }
    for row in ratios {
        for v in row {
            if v.len() == 2 {
                sum += v[0] * v[1];
            }
        }
    }
    println!("Result: {}", sum);
}

pub fn is_adjacent(row: i32, col: i32, inp: &Vec<Vec<char>>) -> bool {
    for roff in -1..2 {
        for coff in -1..2 {
            if roff == 0 && coff == 0 {
                continue;
            }
            if row + roff < 0 || row + roff >= inp.len() as i32 {
                continue;
            }
            if col + coff < 0 || col + coff >= inp[0].len() as i32 {
                continue;
            }

            let v = inp[(row + roff) as usize][(col + coff) as usize];
            if v != '.' && !v.is_ascii_digit() {
                return true;
            }
        }
    }
    false
}

pub fn is_adjacent_gear(row: i32, col: i32, inp: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for roff in -1..2 {
        for coff in -1..2 {
            if roff == 0 && coff == 0 {
                continue;
            }
            if row + roff < 0 || row + roff >= inp.len() as i32 {
                continue;
            }
            if col + coff < 0 || col + coff >= inp[0].len() as i32 {
                continue;
            }

            let v = inp[(row + roff) as usize][(col + coff) as usize];
            if v == '*' {
                return Some(((row + roff) as usize, (col + coff) as usize));
            }
        }
    }
    return None;
}
