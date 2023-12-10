pub fn day10() {
    let inp: Vec<Vec<char>> = include_str!("day10.txt")
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut posses = Vec::new();

    'a: for startr in 0..inp.len() {
        for startc in 0..inp[startr].len() {
            if inp[startr][startc] == 'S' {
                findpipe(startr as i32, startc as i32, &inp, &mut posses);
                posses.reverse();
                break 'a;
            }
        }
    }

    // Visualize the pipe
    for (r, row) in inp.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if posses.contains(&(r, c)) {
                print!("{}", v);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    println!("Part 1: {}", posses.len() / 2)
}

fn findpipe(r: i32, c: i32, inp: &Vec<Vec<char>>, prev: &mut Vec<(usize, usize)>) -> bool {
    prev.push((r as usize, c as usize));
    for roff in (-1..2).rev() {
        // Try different combinations of reversed orders to get the largest answer
        for coff in (-1..2) {
            if roff == 0 && coff == 0 {
                continue;
            }
            if roff != 0 && coff != 0 {
                continue;
            }
            if r + roff < 0 || r + roff >= inp.len() as i32 {
                continue;
            }
            if c + coff < 0 || c + coff >= inp[r as usize].len() as i32 {
                continue;
            }
            let val = inp[(r + roff) as usize][(c + coff) as usize];
            if val == 'S' && (r as usize != prev[1].0) && (c as usize != prev[1].1) {
                return true;
            }
            if prev.contains(&((r + roff) as usize, (c + coff) as usize)) {
                continue;
            }
            let possible = getpipes(roff, coff);
            if possible.contains(&val) {
                if findpipe(r + roff, c + coff, inp, prev) {
                    return true;
                }
            }
        }
    }
    prev.pop();
    false
}

pub fn getpipes(roff: i32, coff: i32) -> Vec<char> {
    match (roff, coff) {
        (-1, 0) => vec!['|', '7', 'F'],
        (1, 0) => vec!['|', 'L', 'J'],
        (0, 1) => vec!['-', 'J', '7'],
        (0, -1) => vec!['-', 'L', 'F'],
        _ => vec![],
    }
}
