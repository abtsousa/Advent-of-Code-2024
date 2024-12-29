fn check_safe(vec : &[i32], failable : bool) -> bool {
    if vec.len() < 2 {return true;}
    let sign = (vec.last().unwrap() - vec.first().unwrap()).signum();
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
        if rst {
            prev = next;
        } else if failable {
            rst = subsets_popped_one(vec).iter().any(|v| check_safe(&v.iter().map(|&&x| x).collect::<Vec<_>>(), false));
            break;
        } else { break };
    }
    rst
}

fn subsets_popped_one<T>(v: &[T]) -> Vec<Vec<&T>> {
    let mut result = Vec::new();
    for i in 0..v.len() {
        let mut subset = Vec::new();
        for (j, item) in v.iter().enumerate() {
            if j != i {
                subset.push(item);
            }
        }
        result.push(subset);
    }
    result
}

pub fn main() {
    let result : Vec<Vec<i32>> =
    include_str!("../input.txt") // Get the input
        .lines() // Split by lines
        .map(|line| line.split_whitespace() // Split by whitespaces
            .map(|x| x.parse::<i32>().unwrap()) // Parse to i32
            .collect()) // Collect to Vec<i32>
        .filter(|v: &Vec<i32>| check_safe(v.as_slice(), true)) // Filter the safe ones
        .collect(); // Collect to Vec<Vec<i32>>
    dbg!(&result);
    println!("The result is {}",result.len())
}
