pub fn parse(input: &str) -> usize {

    fn calculate(input : Vec<String>) -> usize {
        
        let matrix : Vec<Vec<char>> = input.into_iter()
            .map(|s| s.chars().collect()).collect();
        
        let num_rows = matrix.len();
        let num_cols = matrix[0].len();

        let mut result : usize = 0;

        // Search for As
        // 1.2
        // .A.
        // 3.4
        for c in 1..num_cols-1 {
            for r in 1..num_rows-1 {
                let a = matrix[r][c];
                let c1 = matrix[r-1][c-1];
                let c2 = matrix[r-1][c+1];
                let c3 = matrix[r+1][c-1];
                let c4 = matrix[r+1][c+1];
            
                if a == 'A'
                && ( (c1 == 'M' && c4 == 'S') || (c1 == 'S' && c4 == 'M') )
                && ( (c3 == 'M' && c2 == 'S') || (c3 == 'S' && c2 == 'M') ) {
                    result += 1;
                }
            }
        }
        result
    }

    let input_vec : Vec<String> = input.split("\n")
        .filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    calculate(input_vec)
}

fn main() {
    let input = include_str!("../input.txt"); // Get the input
    println!("TOTAL: {}", parse(input))
}
