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
    for l in ls {
        let middle = l.len() / 2 as usize;
        let (left, right) = l.split_at(middle);
        'inner: for c in left.chars() {
            if right.contains(c) {
                score += map_letter_to_int(c);
                break 'inner;
            }
        }
    }
    score
}

fn main() {
    let input = include_str!("../input.txt");
    let winner = rucksack_solver(input);
    println!("{:?}", winner);
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
        assert_eq!(157, score);
        let input = include_str!("../input.txt");
        let score = rucksack_solver(input);
        assert_eq!(7821, score);
    }
}
