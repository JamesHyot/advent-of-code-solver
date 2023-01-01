use std::collections::HashSet;

pub fn solve (input: &str) -> String 
{
    let mut instructions: Vec<Instruction> = vec![];
    let mut positions: HashSet<String> = HashSet::new();
    positions.insert(String::from("0,0"));


    for line in input.lines() {
        let direction = match &line.trim()[..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Invalid direction. Must either be L, R, U or D")
        };

        let length: u32 = line.trim()[2..].parse().unwrap_or(0);

        instructions.push(Instruction { direction, length });
    }
    
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    for instruction in instructions {
        for _ in 0..instruction.length {
            match instruction.direction {
                Direction::Right => {
                    head.x = head.x + 1;
                },
                Direction::Left => {
                    head.x = head.x - 1;
                },
                Direction::Up => {
                    head.y = head.y + 1;
                },
                Direction::Down => {
                    head.y = head.y - 1;
                }
            }
            
            if (head.x - tail.x).abs() >= 2 || (head.y - tail.y).abs() >= 2 {
                // Tail needs to catch up to head
                if (head.x - tail.x).abs() >= 2 {
                    if (head.y - tail.y).abs() == 1 {
                        // diagonal
                        if head.x > tail.x {
                            tail.x = tail.x + 1;
                        } else {
                            tail.x = tail.x - 1;
                        }

                        tail.y = head.y;
                    } else {
                        // simple follow
                        if head.x > tail.x {
                            tail.x = tail.x + 1;
                        } else {
                            tail.x = tail.x - 1;
                        }
                    }
                } else {
                    if (head.x - tail.x).abs() == 1 {
                        // diagonal
                        if head.y > tail.y {
                            tail.y = tail.y + 1;
                        } else {
                            tail.y = tail.y - 1;
                        }

                        tail.x = head.x;
                    } else {
                        // simple follow
                        if head.y > tail.y {
                            tail.y = tail.y + 1;
                        } else {
                            tail.y = tail.y - 1;
                        }
                    }
                }

                positions.insert(format!("{},{}", tail.x, tail.y));
            }
        }
    }

    format!("Part 1: {}, Part 2: {}", positions.len(), 0)
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: u32
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down    
}

struct Point {
    x: i32,
    y: i32
}

#[cfg(test)]
mod tests {
    use super::{*, super::input};

    #[test]
    fn day9_smallinput() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(solve(input), "Part 1: 13, Part 2: 0");
    }

    #[test]
    fn day9_biginput() {
        let input = input::get_input();

        assert_eq!(solve(input.as_str()), "Part 1: 5878, Part 2: 0");
    }
}