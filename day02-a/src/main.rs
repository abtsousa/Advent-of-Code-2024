fn check_safe(vec : &[i32]) -> bool {
    if vec.len() < 2 {return true;}
    let sign = (vec[1] - vec[0]).signum();
    let mut iter = vec.iter();
    let mut prev = iter.next().unwrap();

    let mut rst = true;

    for next in iter {
        let chk = next - prev;
        rst = match chk {
            1..=3 => sign == 1,
            -3..=-1 => sign == -1,
            _ => false
        };
        prev = next;
        if !rst {break};
    }
    rst
}

pub fn main() {
    let result : Vec<Vec<i32>> =
    include_str!("../input.txt") // Get the input
        .lines() // Split by lines
        .map(|line| line.split_whitespace() // Split by whitespaces
            .map(|x| x.parse::<i32>().unwrap()) // Parse to i32
            .collect()) // Collect to Vec<i32>
        .filter(|v: &Vec<i32>| check_safe(v.as_slice())) // Filter the safe ones
        .collect(); // Collect to Vec<Vec<i32>>
    dbg!(&result);
    println!("The result is {}",result.len())
}
