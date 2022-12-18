pub fn solve (input: &str) -> String 
{
    let mut count = 0;
    let mut overlap = 0;
    for line in input.lines() {
        let split:Vec<&str> = line.split(',').collect();
        assert!(split.len() == 2);

        let x : Vec<&str> = split[0].trim().split('-').collect();
        let x_start: u32 = match x[0].trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        let x_end: u32 = match x[1].trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let y : Vec<&str> = split[1].trim().split('-').collect();
        let y_start: u32 = match y[0].trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        let y_end: u32 = match y[1].trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if x_start <= y_start && x_end >= y_end || x_start >= y_start && x_end <= y_end {
            count += 1;
        }

        if x_start >= y_start && x_start <= y_end || x_end >= y_start && x_end <= y_end || y_end >= x_start && y_end <= x_end || y_start >= x_start && y_start <= x_end {
            overlap += 1;
        }
    }

    format!("Inclusions: {}\nOverlaps: {}", count, overlap)
}

#[test]
fn test() {
    let input = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    assert_eq!(solve(input), "Inclusions: 2\nOverlaps: 4");
}

#[test]
fn test_biginput() {
    let input = super::input::get_input();

    assert_eq!(solve(input.as_str()), "Inclusions: 431\nOverlaps: 823");
}