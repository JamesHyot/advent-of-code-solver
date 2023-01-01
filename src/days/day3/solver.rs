pub fn solve (input: &str) -> String 
{
    let lines : Vec<&str> = input.lines().collect();
    let mut total:u32 = 0;
    for line in &lines {
        let trimmed_line = line.trim();
        let line_length = trimmed_line.len();

        assert!(line_length%2 == 0);
        let first_container = &trimmed_line[0.. line_length/2];
        let second_container = &trimmed_line[line_length/2..];

        for c in first_container.chars() {
            if second_container.contains(c) {
                total += get_score(c);
                break
            }
        }
    }

    assert!(&lines.len() % 3 == 0);

    let mut elf_total = 0;
    for i in 0..&lines.len()/3 {
        let first_elf = lines[3*i].trim();
        let second_elf = lines[3*i + 1].trim();
        let third_elf = lines[3*i + 2].trim();

        for c in first_elf.chars() {
            if second_elf.contains(c) && third_elf.contains(c) {
                elf_total += get_score(c);
                break
            }
        }
    }
    
    format!("Result: {}\nElf Result: {}", total, elf_total)
}

fn get_score(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u32 - 'a' as u32 + 1,
        false => c as u32 - 'A' as u32 + 1 + 26
    }
}

#[cfg(test)]
mod tests {
    use super::{*, super::input};

    #[test]
    fn day3_smallinput() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(solve(input), "Result: 157\nElf Result: 70");
    }

    #[test]
    fn day3_biginput() {
        let input = input::get_input();

        assert_eq!(solve(input.as_str()), "Result: 7997\nElf Result: 2545");
    }
}