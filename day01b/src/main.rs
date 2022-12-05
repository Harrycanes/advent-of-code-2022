pub fn elf_iterator(input: &str) -> u32 {
    let ls = input.lines();
    let mut winners = vec![];

    let mut elf_cal = 0;
    for l in ls {
        match l.parse::<u32>() {
            Ok(c) => elf_cal += c,
            Err(_) => {
                winners.push(elf_cal);
                elf_cal = 0;
            }
        }
    }
    winners.push(elf_cal);
    winners.sort();
    winners.reverse();
    winners[..3].iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let winners = elf_iterator(input);
    println!("{:?}", winners);
    ()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_elf_iterator() {
        let input = include_str!("../input.txt");
        let winners = elf_iterator(input);
        assert_eq!(212836, winners)
    }
}
