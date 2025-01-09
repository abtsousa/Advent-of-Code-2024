pub fn parse(input: &str) -> usize {

    const TARGET : [&str; 2]= ["XMAS", "SAMX"];
    const TARGET_LEN : usize = TARGET.len();

    fn transpose(input : Vec<String>) -> Vec<String> {
        let num_cols = input.first().unwrap().len();
        let row_iter = input.into_iter().map(|s| s.chars().collect::<Vec<_>>());
        (0..num_cols).map(|col| row_iter.clone().map(|row| row[col]).collect::<String>()).collect()
    }

    fn down_right_diagonal(input : Vec<String>) -> Vec<String> {

        let matrix : Vec<Vec<char>> = input.into_iter()
            .map(|s| s.chars().collect()).collect();
        
        let num_rows = matrix.len();
        let num_cols = matrix[0].len();

        let mut result = Vec::new();

        // Iterate first row
        for start_col in 0..num_cols {
            let mut diagonal = String::new();
            let mut row = 0;
            let mut col = start_col;

            while row < num_rows && col < num_cols {
                diagonal.push(matrix[row][col]);
                row += 1;
                col += 1;
            }

            result.push(diagonal);
        }

        // Iterate first column (ignore (0,0))
        for start_row in 1..num_rows {
            
            let mut diagonal = String::new();
            let mut col = 0;
            let mut row = start_row;

            while row < num_rows && col < num_cols {
                diagonal.push(matrix[row][col]);
                row += 1;
                col += 1;
            }

            result.push(diagonal);
        }

        result
    }

    fn down_left_diagonal(input : Vec<String>) -> Vec<String> {

        let matrix : Vec<Vec<char>> = input.into_iter()
            .map(|s| s.chars().collect()).collect();
        
        let num_rows = matrix.len();
        let num_cols = matrix[0].len();

        let mut result = Vec::new();

        // Iterate first row
        for start_col in 0..num_cols {
            let mut diagonal = String::new();
            let mut row = 0;
            let mut col = start_col as i32;

            while row < num_rows && col >= 0 {
                diagonal.push(matrix[row][col as usize]);
                row += 1;
                col -= 1;
            }

            result.push(diagonal);
        }

        // Iterate last column (ignore (0,n))
        for start_row in 1..num_rows {
            
            let mut diagonal = String::new();
            let mut col = (num_cols - 1) as i32;
            let mut row = start_row;

            while row < num_rows && col >= 0 {
                diagonal.push(matrix[row][col as usize]);
                row += 1;
                col -= 1;
            }

            result.push(diagonal);
        }

        result
    }

    let input_vec : Vec<String> = input.split("\n")
        .filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    fn parse_horizontal(input: &str, targets: [&str; TARGET_LEN]) -> usize {
        let mut count = 0;
        for target in targets {
            count += input.matches(target).count();
        }
        count
    }

    fn calculate(input_vec: &[String]) -> usize {
        input_vec.iter().map(|x| parse_horizontal(x, TARGET)).sum()
    }

    let calc_h = calculate(&input_vec);
    println!("Found {} matches in horizontal search.", calc_h);
    let calc_v = calculate(&transpose(input_vec.clone()));
    println!("Found {} matches in vertical search.", calc_v);
    let calc_dr = calculate(&down_right_diagonal(input_vec.clone()));
    println!("Found {} matches in down-right search.", calc_dr);
    let calc_dl = calculate(&down_left_diagonal(input_vec.clone()));
    println!("Found {} matches in down-left search.", calc_dl);
    calc_h + calc_v + calc_dr + calc_dl
}

fn main() {
    let input = include_str!("../input.txt"); // Get the input
    println!("TOTAL: {}", parse(input))
}
