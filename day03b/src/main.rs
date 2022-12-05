use std::collections::HashSet;
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
    let mut rucksacks: Vec<&str> = vec![];
    for (i, l) in ls.enumerate() {
        if (i + 1) % 3 == 0 {
            let set_a: HashSet<_> = rucksacks[0].chars().collect();
            let set_b: HashSet<_> = rucksacks[1].chars().collect();
            let set_c: HashSet<_> = rucksacks[2].chars().collect();
            let set_ab = set_a
                .intersection(&set_b)
                .collect::<HashSet<_>>()
                .intersection(&set_c);
            let ch = rucksacks.clear();
        } else {
            rucksacks.push(l);
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
