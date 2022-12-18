use regex::Regex;

pub fn solve (input: &str) -> String 
{
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
    
    result += "\nNew crane ";

    for stack in stacks2 {
        result.push(match stack.last() {
            Some(c) => *c,
            None => panic!("Vide")
         });
    }

    result
}