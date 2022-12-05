pub fn rock_paper_scissors(input: &str) -> u32 {
    let ls = input.lines();
    let mut score = 0;
    for l in ls {
        score += match l {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => {
                panic!("WHAT?!");
            }
        }
    }
    score
}

fn main() {
    let input = include_str!("../input.txt");
    let score = rock_paper_scissors(input);
    println!("{:?}", score);
    ()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rock_paper_scissors() {
        let test_input = r#"A Y
B X
C Z"#;
        let input = include_str!("../input.txt");
        let score = rock_paper_scissors(test_input);
        assert_eq!(15, score);
        let score = rock_paper_scissors(input);
        assert_eq!(13268, score);
    }
}
