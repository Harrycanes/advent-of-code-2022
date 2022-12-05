fn map_letter_to_int(ch: char) -> u32 {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    match alphabet.find(ch) {
        Some(c) => c as u32 + 1,
        None => 0,
    }
}

pub fn rucksack_solver(input: &str) -> u32 {
    let ls = input.lines();
    let mut score = 0;
    let mut concatenated_slice: String = "".to_string();
    for (i, l) in ls.enumerate() {
        if (i + 1) % 3 == 0 {
            concatenated_slice.push_str(l);
            'inner: for ch in l.chars() {
                if concatenated_slice.matches(ch).count() > 2 {
                    dbg!(ch);
                    score += map_letter_to_int(ch);
                    break 'inner;
                }
            }
            dbg!(concatenated_slice);
            concatenated_slice = "".to_string();
        } else {
            concatenated_slice.push_str(l);
        }
    }
    score
}

fn main() {
    let input = include_str!("../input.txt");
    let score = rucksack_solver(input);
    println!("{:?}", score);
    ()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rucksack_solver() {
        let test_input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let score = rucksack_solver(test_input);
        assert_eq!(70, score);
        // let input = include_str!("../input.txt");
        // let score = rucksack_solver(input);
        // assert_eq!(7821, score);
    }
}
