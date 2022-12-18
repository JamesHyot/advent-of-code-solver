use std::collections::VecDeque;

pub fn solve (input: &str) -> String 
{
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

    format!("Packet counter {}\nMessage counter {}", packet_counter, counter)
}

#[test]
fn test() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    assert_eq!(solve(input), "Packet counter 7\nMessage counter 19");
}

#[test]
fn test_biginput() {
    let input = super::input::get_input();

    assert_eq!(solve(input.as_str()), "Packet counter 1275\nMessage counter 3605");
}