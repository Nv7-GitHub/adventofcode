const DIGITS: [(&str, usize); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

pub fn day1() {
    // Parse
    let inp: Vec<&str> = include_str!("day1.txt").split("\n").collect();
    let mut sum = 0;
    for (i, chk) in inp.into_iter().enumerate() {
        // Part 1
        /*let nums: Vec<i32> = chk
            .chars()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();
        println!("{:?}", nums);
        sum += nums[0] * 10;
        sum += nums[nums.len() - 1];*/

        // Part 2
        let mut start = 0;
        let mut i = 0;
        'start: while i < chk.len() {
            for digit in DIGITS {
                if i + digit.0.len() <= chk.len()
                    && chk.chars().skip(i).collect::<String>().starts_with(digit.0)
                {
                    start = digit.1;
                    break 'start;
                }
            }
            i += 1;
        }

        let mut end = 0;
        let mut i = chk.len() - 1;
        'end: while i > 0 {
            for digit in DIGITS {
                if i + digit.0.len() <= chk.len()
                    && chk.chars().skip(i).collect::<String>().starts_with(digit.0)
                {
                    end = digit.1;
                    break 'end;
                }
            }
            i -= 1;
        }

        println!("{}{}", start, end);
        sum += start * 10 + end;
    }

    println!("Result 1: {}", sum);
}
