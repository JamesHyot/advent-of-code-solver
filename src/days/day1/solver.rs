pub fn solve (input: &str) -> String 
{
    let mut lines: Vec<&str> = input.lines().collect();

    // Add an extra empty line so that the final parse will break
    lines.push(&"");

    let mut best_elf = 0;
    let mut second_best_elf = 0;
    let mut third_best_elf = 0;
    let mut current_elf = 0;

    for line in lines {
        let calorie: u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if current_elf >= best_elf {
                    third_best_elf = second_best_elf;
                    second_best_elf = best_elf;
                    best_elf = current_elf;
                } else if current_elf >= second_best_elf {
                    third_best_elf = second_best_elf;
                    second_best_elf = current_elf;
                } else if current_elf > third_best_elf {
                    third_best_elf = current_elf;
                }

                current_elf = 0;

                continue
            },
        };

        current_elf += calorie;
    }

    format!("Best elf {}\nTop three elves total {}", best_elf, best_elf + second_best_elf + third_best_elf)
}


#[test]
fn test() {
    let input = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    assert_eq!(solve(input), String::from("Best elf 24000\nTop three elves total 45000"))
}

#[test]
fn test_biginput() {
    let input = super::input::get_input();
    assert_eq!(solve(input.as_str()), String::from("Best elf 73211\nTop three elves total 213958"))
}