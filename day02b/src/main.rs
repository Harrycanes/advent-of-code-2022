const LOSS: u32 = 0; // X
const DRAW: u32 = 3; // Y
const WIN: u32 = 6; // Z
const ROCK: u32 = 1; // A
const PAPER: u32 = 2; // B
const SCISSORS: u32 = 3; // C

pub fn rock_paper_scissors(input: &str) -> u32 {
    let ls = input.lines();
    let mut score = 0;
    for l in ls {
        score += match l {
            "A X" => LOSS + SCISSORS,
            "A Y" => DRAW + ROCK,
            "A Z" => WIN + PAPER,
            "B X" => LOSS + ROCK,
            "B Y" => DRAW + PAPER,
            "B Z" => WIN + SCISSORS,
            "C X" => LOSS + PAPER,
            "C Y" => DRAW + SCISSORS,
            "C Z" => WIN + ROCK,
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
        assert_eq!(12, score);
        let score = rock_paper_scissors(input);
        assert_eq!(15508, score);
    }
}
