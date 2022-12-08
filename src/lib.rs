mod utils;
use regex::Regex;
use std::collections::VecDeque;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn get_day1_result(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

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

#[wasm_bindgen]
pub fn get_day2_result(input: &str) -> String {
    let mut score: u64 = 0;

    for line in input.lines() {
        let elf_letter = &line.trim()[..1];
        let your_letter = &line.trim()[2..];
        
        let elf_move = match elf_letter {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            &_ => continue
        };
        
        // let your_move = match your_letter {
        //     "X" => Move::Rock,
        //     "Y" => Move::Paper,
        //     "Z" => Move::Scissors,
        //     &_ => continue
        // };

        let match_result = match your_letter {
            "X" => MatchResult::Loss,
            "Y" => MatchResult::Draw,
            "Z" => MatchResult::Win,
            &_ => continue
        };

        let your_move = get_move_needed_for_result(&elf_move, &match_result);
        let new_score = get_match_score(&your_move, &elf_move) + get_move_score(&your_move);
        println!("Match score {}", new_score);
        score += u64::from(new_score);
    }

    format!("Score total {}", score)
}

#[wasm_bindgen]
pub fn get_day3_result(input: &str) -> String {
    let lines : Vec<&str> = input.lines().collect();
    let mut total:u32 = 0;
    for line in &lines {
        let line_length = line.len();

        assert!(line_length%2 == 0);
        let first_container = &line[0.. line_length/2];
        let second_container = &line[line_length/2..];

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
        let first_elf = lines[3*i];
        let second_elf = lines[3*i + 1];
        let third_elf = lines[3*i + 2];

        for c in first_elf.chars() {
            if second_elf.contains(c) && third_elf.contains(c) {
                elf_total += get_score(c);
                break
            }
        }
    }
    
    format!("Result {}\nElf Result {}", total, elf_total)
}

#[wasm_bindgen]
pub fn get_day4_result(input: &str) -> String {
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

#[wasm_bindgen]
pub fn get_day5_result(input: &str) -> String {
    let mut stacks = vec![
        vec!('F','H','B','V','R','Q','D','P'),
        vec!('L','D','Z','Q','W','V'),
        vec!('H','L','Z','Q','G','R','P','C'),
        vec!('R','D','H','F','J','V','B'),
        vec!('Z','W','L','C'),
        vec!('J','R','P','N','T','G','V','M'),
        vec!('J','R','L','V','M','B','S'),
        vec!('D','P','J'),
        vec!('D','C','N','W','V')
        ];

    let mut stacks2 = Vec::clone(&stacks);
    
    let re = Regex::new(r"move ([0-9]*) from ([0-9]*) to ([0-9])*").unwrap();
    assert!(re.is_match(&input));
    
    for capture in re.captures_iter(&input) {
        let move_count: usize = match capture[1].parse() {
            Ok(num) => num,
            Err(_) => panic!("Parse move count")
        };

        let from: usize = match capture[2].parse() {
            Ok(num) => num,
            Err(_) => panic!("Parse from")
        };
        let to: usize = match capture[3].parse() {
            Ok(num) => num,
            Err(_) => panic!("Parse to")
        };
        
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..move_count {
            let c = match stacks[from - 1].pop() {
                Some(val) => val,
                None => panic!("Plus assez d'éléments")
            };

            stacks[to - 1].push(c);
            
            let c2 = match stacks2[from - 1].pop() {
                Some(val) => val,
                None => panic!("Plus assez d'éléments")
            };

            temp.push(c2);
        }

        for _ in 0..move_count {
            let c = match temp.pop() {
                Some(val) => val,
                None => panic!("Plus assez d'éléments")
            };

            stacks2[to - 1].push(c);
        }
    }


    let mut result = String::from("First crane ");
    for stack in stacks {
        result.push(match stack.last() {
            Some(c) => *c,
            None => panic!("Vide")
         });
    }
    
    result += " New crane ";

    for stack in stacks2 {
        result.push(match stack.last() {
            Some(c) => *c,
            None => panic!("Vide")
         });
    }

    result
}

#[wasm_bindgen]
pub fn get_day6_result(input: &str) -> String {
    let mut buffer: VecDeque<char> = VecDeque::new();
    let mut message_buffer: VecDeque<char> = VecDeque::new();
    let mut counter: usize = 0;
    let mut packet_counter: usize = 0;

    'main_loop: for c in input.chars() {
        counter += 1;

        buffer.push_back(c);
        message_buffer.push_back(c);

        if buffer.len() > 4 {
            buffer.pop_front();
        }

        if message_buffer.len() > 14 {
            message_buffer.pop_front();
        }

        if packet_counter == 0 && buffer.len() == 4 {
            if buffer[0] != buffer[1] && buffer[0] != buffer[2] && buffer[0] != buffer[3] && buffer[1] != buffer[2] && buffer[1] != buffer[3] && buffer[2] != buffer[3] {
                packet_counter = counter;
            }
        }

        if message_buffer.len() == 14 {
            for i in 0..13 {
                for j in i+1..14 {
                    if message_buffer[i] == message_buffer[j] {
                        continue 'main_loop
                    }
                }
            }

            break
        }
    }

    format!("Packet counter {}, Message counter {}", packet_counter, counter)
}

#[wasm_bindgen]
pub fn get_day7_result(input: &str) -> String {
    String::from(input)
}

#[wasm_bindgen]
pub fn get_day8_result(input: &str) -> String {
    String::from(input)
}

fn get_score(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u32 - 'a' as u32 + 1,
        false => c as u32 - 'A' as u32 + 1 + 26
    }
}

fn get_match_score(p1: &Move, p2: &Move) -> u8 {
    match get_match_result(p1, p2) {
        MatchResult::Win => 6,
        MatchResult::Draw => 3,
        MatchResult::Loss => 0
    }
}

fn get_move_score(m: &Move) -> u8 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }
}

fn get_move_needed_for_result (p1: &Move, result: &MatchResult) -> Move {
    match result {
        MatchResult::Draw => {
            match p1 {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors
            }
        }
        MatchResult::Win => {
            match p1 {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock
            }
        },
        MatchResult::Loss => {
            match p1 {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper
            }
        },
    }
}

fn get_match_result (p1: &Move, p2: &Move) -> MatchResult {
    if p1 == p2 { MatchResult::Draw }
    else {
        match p1 {
            Move::Rock => {
                if p2 == &Move::Paper {
                    MatchResult::Loss
                } else {
                    MatchResult::Win
                }
            },
            Move::Paper => {
                if p2 == &Move::Scissors {
                    MatchResult::Loss
                } else {
                    MatchResult::Win
                }
            },
            Move::Scissors => {
                if p2 == &Move::Rock {
                    MatchResult::Loss
                } else {
                    MatchResult::Win
                }
            },
        }
    }
}

enum MatchResult {
    Win,
    Loss,
    Draw
}

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}
