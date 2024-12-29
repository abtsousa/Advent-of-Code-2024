use regex::Regex;

pub fn main() {
    let re = Regex::new(r"mul\((?<d1>\d+),(?<d2>\d+)\)").unwrap();
    let input = include_str!("../input.txt"); // Get the input

    let caps: Vec<i32> = re.captures_iter(input)
        .flat_map(|cap| {
            let d1: i32 = cap.name("d1").and_then(|x| x.as_str().parse::<i32>().ok()).unwrap();
            let d2: i32 = cap.name("d2").and_then(|x| x.as_str().parse::<i32>().ok()).unwrap();
            vec![d1,d2]
        }).collect::<Vec<_>>();

    let mut result = 0;

    for chunk in caps.chunks(2) {
        match chunk {
            [i, j] => result += i * j,
            _ => continue
        }
    }

    println!("The result is: {}", result)
}
