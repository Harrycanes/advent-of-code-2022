pub fn elf_iterator(input: &str) -> u32 {
    let ls = input.lines();
    let mut winners: Vec<u32> = vec![0, 0, 0];

    let mut elf_cal = 0;
    for l in ls {
        match l.parse::<u32>() {
            Ok(c) => elf_cal += c,
            Err(_) => {
                if elf_cal > winner.1 {
                    winner = elf_cal
                }
                elf_cal = 0;
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
