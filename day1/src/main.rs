fn main() {
    let inputs = include_str!("../input.txt");

    let inputs: Vec<&str> = inputs.split("\n\n").collect();

    let inputs: Vec<Vec<u32>> = inputs
        .iter()
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut sums: Vec<u32> = inputs
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect();
    
    sums.sort();
    sums.reverse();

    println!("{:?}", &sums[..3].iter().sum::<u32>());
    
}
