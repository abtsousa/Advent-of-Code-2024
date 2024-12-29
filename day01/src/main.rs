use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn parse(path : &str) -> Result<(Vec<i32>,Vec<i32>), Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut vec1 : Vec<i32> = Vec::new();
    let mut vec2 : Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut words = line.split_whitespace();
        if let (Some(word1), Some(word2)) = (words.next(), words.next()) {
            vec1.push(word1.parse().unwrap());
            vec2.push(word2.parse().unwrap());
        }
    }

    Ok((vec1,vec2))
}

fn diff(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for (av, bv) in a.iter().zip(b) {
        let rv = av-bv;
        result.push(rv.abs())
    }
    result
}

fn sim(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for av in a.iter() {
        let rv = b.iter().filter(|&x| {*x == *av}).count() as i32;
        result.push(rv * av);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // dbg!(file_path);

    let (mut lst1, mut lst2) = parse(file_path).unwrap();
    lst1.sort(); lst2.sort();
    // dbg!(&lst1);
    // dbg!(&lst2);

    let result = diff(&lst1, &lst2);
    let rst : i32 = result.iter().sum();
    println!("The result is: {}", rst);

    let result2 = sim(&lst1, &lst2);
    let rst2 : i32 = result2.iter().sum();
    println!("The result is: {}", rst2);
}
