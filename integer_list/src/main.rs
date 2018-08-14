use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.trim().is_empty() {
            break;
        }

        let mut numbers: Vec<_> = line
            .split_whitespace()
            .map(|w| {
                w.parse::<u32>()
                    .expect(&format!("Cannot convert '{}' into number!", w))
            })
            .collect();
        numbers.sort_unstable();

        println!("Numbers: {:?}", numbers);
        println!("Mean: {}", mean(&numbers));
        println!("Median: {}", median(&numbers));
        println!("Mode: {}", mode(&numbers));
    }
}

fn mean(numbers: &[u32]) -> f64 {
    let sum = numbers.iter().fold(0u32, |sum, val| sum + val) as f64;
    sum / numbers.len() as f64
}

fn median(numbers: &[u32]) -> f64 {
    let middle = numbers.len() / 2;
    if numbers.len() % 2 == 1 {
        numbers[middle] as f64
    } else {
        mean(&numbers[(middle - 1)..=middle])
    }
}

fn mode(numbers: &[u32]) -> &u32 {
    let mut stats = HashMap::new();
    for n in numbers {
        let entry = stats.entry(n).or_insert(0);
        *entry += 1;
    }
    stats.iter().max_by_key(|e| e.1).unwrap().0
}
