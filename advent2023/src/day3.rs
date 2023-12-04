pub fn day3() {
    let inp: Vec<Vec<char>> = include_str!("day3.txt")
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut sum = 0;
    for (r, line) in inp.iter().enumerate() {
        let mut c = 0;
        while c < line.len() {
            if line[c].is_ascii_digit() {
                let mut num = "".to_string();
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
                }
            }
            c += 1;
        }
    }
    println!("Part 1: {}", sum);
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
