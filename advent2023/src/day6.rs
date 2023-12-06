pub fn day6() {
    let times: Vec<usize> = "53837288"
        .split(" ")
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let distance: Vec<usize> = "333163512891532"
        .split(" ")
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut res = 1;
    for i in 0..times.len() {
        let mut sum = 0;
        for p in 0..times[i] {
            let d = p * (times[i] - p);
            if d > distance[i] {
                sum += 1;
            }
        }
        println!("{}", sum);
        res *= sum;
    }
    println!("Result: {}", res);
}
