use std::io::{self, BufRead};

fn digits_sum(n: u64, acc: u64) -> u64 {
    match n {
        0 => acc,
        _ => digits_sum(n / 10, n % 10 + acc),
    }
}

fn main() {
    let stdin = io::stdin();
    let numbers = stdin.lock().lines()
        .filter(Result::is_ok)
        .map(|rs| rs.unwrap().parse())
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .filter(|&n| n % digits_sum(n, 0) == 0)
        .collect::<Vec<u64>>();

    for n in numbers {
        println!("{}", n);
    }
}
