use std::collections::HashMap;

pub fn day8() {
    let inp: Vec<&str> = include_str!("day8.txt").split("\n").collect();
    let inds: Vec<usize> = "LRRRLRRLRLRRRLRLLLLRRRLRLRRLRLRLRRLRRRLRRLRRRLRLLLRRLRRLRRLRRLRRLLLLLRLRLRRRLRLLRRLRLRRRLRRLRRRLLLRRLRRLRRLRRRLRLRLRRLLRRRLRRLRRRLRRRLRRRLRLRRLRRLRRLRRRLRLRRLRRLLRRRLRRLRRLRRRLRLRLRRLLRRRLLRRLRRRLRRRLRLRRLLRRRLRLRRLLRRLRLRRRLRLRRRLRRLRRLRRLRRRLRRRLRLLRRLRRLLRRLRRRLRRLRLRLRRRLLLRRLRLRRLRRLRLRLLRLRRLRLRLRRRR".chars().map(|x| if x == 'R' { 1 } else { 0 }).collect();
    let mut paths = HashMap::new();
    for line in inp {
        let parts = line.split(" = ").collect::<Vec<&str>>();
        let end = parts[1].split(", ").collect::<Vec<&str>>();
        paths.insert(
            parts[0].to_string(),
            (end[0].replace("(", ""), end[1].replace(")", "")),
        );
    }

    // Part 1
    /*let mut iters = 0;
    let mut curr: String = "AAA".to_string();
    while curr != "ZZZ" {
        for v in inds.iter() {
            if *v == 0 {
                curr = paths[&curr].0.clone();
            } else {
                curr = paths[&curr].1.clone();
            }
        }
        iters += 1;

        if iters % 100000 == 0 {
            println!("{}", iters);
        }
    }
    println!("Part 1: {}", iters * inds.len());*/

    // Part 2
    let mut iters = 0;
    let mut curr = Vec::new();
    for key in paths.keys() {
        if key.ends_with("A") {
            curr.push(key.clone());
        }
    }
    let mut done = false;
    while !done {
        done = true;
        for i in 0..curr.len() {
            for v in inds.iter() {
                if *v == 0 {
                    curr[i] = paths[&curr[i]].0.clone();
                } else {
                    curr[i] = paths[&curr[i]].1.clone();
                }
            }
            if !curr[i].ends_with("Z") {
                done = false;
            }
        }
        if iters % 10000 == 0 {
            println!("{}", iters);
        }
        iters += 1;
    }
    println!("Part 2: {}", iters * inds.len());
}
