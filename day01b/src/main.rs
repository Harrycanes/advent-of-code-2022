pub fn elf_iterator(input: &str) -> u32 {
    let ls = input.lines();
    let mut winners = vec![0, 0, 0];

    let mut elf_cal = 0;
    for l in ls {
        winners.reverse_sort();
        match l.parse::<u32>() {
            Ok(c) => elf_cal += c,
            Err(_) => {
                let i = winners.partition_point(|&x| x < elf_cal);
                dbg!(i);
                winners.insert(i-1, elf_cal);
                winners.truncate(3);
                elf_cal = 0;
            }
        }
    }

    winners.iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let winner = elf_iterator(input);
    println!("{:?}", winner);
    ()
}
