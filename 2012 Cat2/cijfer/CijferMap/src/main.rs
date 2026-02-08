use std::io::{self, Read};

fn main() {
    // Leest volledige invoer
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    let t: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut k: u64 = it.next().unwrap().parse().unwrap();

        let mut digits: u64 = 1;
        let mut count: u64 = 9;
        let mut start: u64 = 1;

        // Zoekt het blok waarin het k-de cijfer zit
        while k > digits * count {
            k -= digits * count;
            digits += 1;
            count *= 10;
            start *= 10;
        }

        // Bepaalt het exacte getal
        let number = start + (k - 1) / digits;

        // Bepaalt de positie van het cijfer binnen het getal
        let index = ((k - 1) % digits) as usize;
        let result = number.to_string().as_bytes()[index] as char;

        println!("{}", result);
    }
}
