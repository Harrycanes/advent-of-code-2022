// An elf has an index and an assigned calorie count
type Elf = (u32, u32);

pub fn elf_iterator(input: &str) -> Elf {
    let ls = input.lines();
    let mut winner: Elf = (0, 0);

    let mut elf_index = 1;
    let mut elf_cal = 0;
    for l in ls {
        match l.parse::<u32>() {
            Ok(c) => elf_cal += c,
            Err(_) => {
                if elf_cal > winner.1 {
                    winner = (elf_index, elf_cal);
                }
                elf_cal = 0;
                elf_index += 1;
            }
        }
    }

    winner
}

fn main() {
    let input = include_str!("../input.txt");
    let winner = elf_iterator(input);
    println!("{:?}", winner);
    ()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_elf_iterator() {
        let input = include_str!("../input.txt");
        let winner = elf_iterator(input);
        assert_eq!((152, 74394), winner)
    }
}
