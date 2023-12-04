pub fn day4() {
    let inp: Vec<&str> = include_str!("day4.txt").split("\n").collect();
    let mut intersections = Vec::new();
    for line in inp {
        let line = line.split(": ").skip(1).next().unwrap();
        let parts: Vec<&str> = line.split(" | ").collect();
        let real: Vec<usize> = parts[0]
            .split(" ")
            .filter(|x| x.len() > 0 && x != &" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let got: Vec<usize> = parts[1]
            .split(" ")
            .filter(|x| x.len() > 0 && x != &" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let cnt: Vec<usize> = got.into_iter().filter(|x| real.contains(x)).collect();
        intersections.push(cnt);
    }

    // Part 1

    let mut sum = 0;
    for int in intersections.iter() {
        if int.len() > 0 {
            sum += (2 as usize).pow((int.len() as u32) - 1);
        }
    }
    println!("Part 1: {}", sum);

    // Part 2
    let mut copies_new = Vec::new();
    let mut copies_old: Vec<usize> = (0..intersections.len()).collect();
    let mut copies = 0;
    while copies_old.len() > 0 {
        // Make copies
        for v in copies_old.iter() {
            copies_new
                .append(&mut (*v + 1..(v + intersections[*v].len() + 1)).collect::<Vec<usize>>());
        }

        // Move everything
        copies += copies_old.len();
        copies_old = copies_new;
        copies_new = Vec::new();
    }
    println!("Part 2: {}", copies);
}

pub fn day4p2() {}
