pub fn solve (input: &str) -> String 
{
    let mut score_part1: u64 = 0;
    let mut score_part2: u64 = 0;

    for line in input.lines() {
        let elf_letter = &line.trim()[..1];
        let your_letter = &line.trim()[2..];
        
        let elf_move = match elf_letter {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            &_ => continue
        };
        
        let your_move = match your_letter {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            &_ => continue
        };

        let match_result = match your_letter {
            "X" => MatchResult::Loss,
            "Y" => MatchResult::Draw,
            "Z" => MatchResult::Win,
            &_ => continue
        };

        let your_move_for_result = get_move_needed_for_result(&elf_move, &match_result);
        let score = get_match_score(&your_move, &elf_move) + get_move_score(&your_move);
        let new_score = get_match_score(&your_move_for_result, &elf_move) + get_move_score(&your_move_for_result);
        
        score_part1 += u64::from(score);
        score_part2 += u64::from(new_score);
    }

    format!("Score partie 1: {}\nScore partie 2: {}", score_part1, score_part2)
}

#[test]
fn test() {
    let input = "A Y
    B X
    C Z";

    assert_eq!(solve(input), "Score partie 1: 15\nScore partie 2: 12");
}

#[test]
fn test_biginput() {
    let input = super::input::get_input();

    assert_eq!(solve(input.as_str()), "Score partie 1: 13005\nScore partie 2: 11373");
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
