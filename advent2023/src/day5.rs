pub fn day5() {
    // Parse maps
    let inp: Vec<&str> = include_str!("day5.txt").split("\n\n").collect();
    let mut maps = Vec::new();
    for (j, chk) in inp.iter().enumerate() {
        maps.push(Vec::new());
        for v in chk.split("\n").skip(1) {
            let nums: Vec<usize> = v.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
            maps[j].push((nums[1], nums[0], nums[2]));
        }
    }

    let mut seeds: Vec<usize> = "1347397244 12212989 2916488878 1034516675 2821376423 8776260 2240804122 368941186 824872000 124877531 1597965637 36057332 4091290431 159289722 1875817275 106230212 998513229 159131132 2671581775 4213184"
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // Part 2 start
    let mut newseeds = Vec::new();
    for i in 0..seeds.len() / 2 {
        for i in seeds[i * 2]..seeds[i * 2] + seeds[i * 2 + 1] {
            newseeds.push(i);
        }
    }
    seeds = newseeds;
    // Part 2 end

    let mut lowest = usize::MAX;
    for (i, seed) in seeds.iter().enumerate() {
        if i % 10000000 == 0 {
            println!("{}", i as f32 / seeds.len() as f32 * 100.0);
        }
        let mut seed = *seed;
        for map in maps.iter() {
            'a: for v in map {
                if v.0 <= seed && seed <= v.0 + v.2 {
                    seed = (seed - v.0) + v.1;
                    break 'a;
                }
            }
        }

        if seed < lowest {
            lowest = seed;
        }
    }
    println!("Result: {}", lowest);
}
