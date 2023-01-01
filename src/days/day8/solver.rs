use std::collections::HashSet;

pub fn solve (input: &str) -> String 
{
    // This is far from pretty. There's actually no reason to have both columns and rows as we're iterating over indexes directly.
    // Todo : refacto to remove one or the 

    let lines : Vec<&str> = input.lines().collect();
    let mut result: HashSet<String> = HashSet::new();
    let mut scores: Vec<u32> = vec![];

    let forest_height = lines.len();
    let forest_width = lines.first().expect("If we don't have any input on the first line ... we can't do anything").len();

    let mut rows: Vec<Vec<u32>> = vec![vec![0; forest_width]; forest_height];

    // Parse the input and put it into rows and columns which we'll then be able to use for the solve
    for height in 0..forest_height {
        let current_line = lines[height];

        let mut width = 0;
        for c in current_line.chars() {       
            let digit: u32 = c.to_digit(10).expect("Input should only contains digits, which can be converted to u8");
            
            rows[height][width] = digit;

            width  = width + 1;
        }
    }

    // x ----------------->
    // y
    // |
    // |
    // |
    // |
    // v

    // Go through rows from left to right
    for i in 0..forest_height {
        let mut max_height = rows[i][0];
        result.insert(format!("{},{}", 0, i));

        // Only iterate on "inside" elements so reduce range by 1 at start and 1 at end
        for j in 1..forest_width - 1 {
            if rows[i][j] > max_height {
                max_height = rows[i][j];
                result.insert(format!("{},{}", j, i));
            }
        }
    }

    // Go through rows from right to left
    for i in 0..forest_height {
        let mut max_height = rows[i][forest_width - 1];
        result.insert(format!("{},{}", forest_width - 1, i));

        for j in (1..forest_width - 1).rev() {
            if rows[i][j] > max_height {
                max_height = rows[i][j];
                result.insert(format!("{},{}", j, i));
            }
        }
    }

    // Go through column from top to bottom
    for i in 0..forest_width {
        let mut max_height = rows[0][i];
        result.insert(format!("{},{}", i, 0));
        
        for j in 1..forest_height - 1 {
            if rows[j][i] > max_height {
                max_height = rows[j][i];
                result.insert(format!("{},{}", i, j));
            }
        }
    }

    // Go through column from bottom to top
    for i in 0..forest_height {
        let mut max_height = rows[forest_height - 1][i];
        result.insert(format!("{},{}", i, forest_height - 1));

        for j in (1..forest_width - 1).rev() {
            if rows[j][i] > max_height {
                max_height = rows[j][i];
                result.insert(format!("{},{}", i, j));
            }
        }
    }

    // Part 2 - no need to check for trees on the outside as their score will be 0
    for x in 1..forest_width - 1 {
        for y in 1..forest_height - 1 {
            let tree = rows[y][x];
            let mut right:u32 = 0;
            let mut left:u32 = 0;
            let mut down:u32 = 0;
            let mut up:u32 = 0;

            // Go right
            for i in x + 1..forest_width {
                right = right + 1;

                if tree <= rows[y][i] {
                    break;
                }
            }

            // Go left
            for i in (0..x).rev() {
                left = left + 1;

                if tree <= rows[y][i] {
                    break;
                }
            }

            // Go down
            for i in y + 1..forest_height {
                down = down + 1;

                if tree <= rows[i][x] {
                    break;
                }
            }

            // Go up
            for i in (0..y).rev() {
                up = up + 1;

                if tree <= rows[i][x] {
                    break;
                }
            }

            scores.push(right * left * down * up);
        }
    }

    format!("Part 1: {}, Part 2: {}", result.len(), scores.iter().max().expect("We'll always have at least one tree inside"))
}

#[cfg(test)]
mod tests {
    use super::{*, super::input};

    #[test]
    fn day8_smallinput() {
        let input = "30373
25512
65332
33549
35390";

        assert_eq!(solve(input), "Part 1: 21, Part 2: 8");
    }

    #[test]
    fn day8_biginput() {
        let input = input::get_input();

        assert_eq!(solve(input.as_str()), "Part 1: 1816, Part 2: 383520");
    }
}