pub fn day9() {
    let inp: Vec<&str> = include_str!("day9.txt").split("\n").collect();
    let mut p1 = 0;
    let mut p2 = 0;
    for line in inp {
        let mut rows = vec![line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()];
        let mut valid = false;
        while !valid {
            valid = true;
            let mut diffs = Vec::new();
            let r = &rows[rows.len() - 1];
            for i in 1..r.len() {
                diffs.push(r[i] - r[i - 1]);
                if r[i] - r[i - 1] != 0 {
                    valid = false;
                }
            }
            rows.push(diffs);
        }

        let mut lastp1 = 0;
        let mut lastp2 = 0;
        for i in (0..rows.len() - 1).rev() {
            lastp1 = rows[i][rows[i].len() - 1] + lastp1;
            lastp2 = rows[i][0] - lastp2;
        }
        p1 += lastp1;
        p2 += lastp2;
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
