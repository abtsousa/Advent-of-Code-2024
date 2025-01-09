use regex::Regex;

pub fn main() {
    // TODO: capture mul() before first don't() and after last do()

    let re0 = Regex::new(r"(?s)do\(\).*?don't\(\)").unwrap();

    let re = Regex::new(r"mul\((?<d1>\d{1,3}),(?<d2>\d{1,3})\)").unwrap();

    let input = "do()".to_string() + include_str!("../input.txt") + "don't()"; // Get the input

    let caps = re0.find_iter(&input)
    .map(|c| c.as_str());

    let mut mul_vec : Vec<i32> = Vec::new();

    for i in caps {
        let mut res = re.captures_iter(i)
            .flat_map(|cap| {
            let d1: i32 = cap.name("d1").and_then(|x| x.as_str().parse::<i32>().ok()).unwrap();
            let d2: i32 = cap.name("d2").and_then(|x| x.as_str().parse::<i32>().ok()).unwrap();
            vec![d1, d2]
        })
        .collect::<Vec<_>>();

        mul_vec.append(&mut res);

    }

    println!("{:?}", mul_vec);

    let mut result = 0;

    for chunk in mul_vec.chunks(2) {
        match chunk {
            [i, j] => result += i * j,
            _ => continue,
        }
    }

    println!("The result is: {}", result)
}
