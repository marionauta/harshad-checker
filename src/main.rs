use std::io::{self, Read};

fn digits_sum(n: u64, acc: u64) -> u64 {
    match n {
        0 => acc,
        _ => digits_sum(n / 10, n % 10 + acc),
    }
}

fn harshad(n: u64) -> bool {
    n % digits_sum(n, 0) == 0
}

fn read_stdin() -> Result<String, io::Error> {
    let mut res = String::new();

    match io::stdin().read_to_string(&mut res) {
        Ok(_) => Ok(res),
        Err(e) => Err(e),
    }
}

fn main() {
    let numbers: Vec<u64> = match read_stdin() {
        Ok(content) => content.split_whitespace()
            .map(str::parse)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect(),
        Err(_) => Vec::new(),
    };
            

    for n in numbers {
        if harshad(n) {
            println!("{}", n);
        }
    }
}
