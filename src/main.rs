extern crate regex;
use rand::prelude::*;
use regex::Regex;
use time::OffsetDateTime;

fn main() {
    let n = 80; // 计算第40个斐波那契数
    println!("Rust Fibonacci({}): {}", n, fibonacci(n));
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    println!("Now: {}", now);
    let local: OffsetDateTime = now.to_offset(now.offset());
    println!("Local: {}", local);
    let x: u8 = random();
    let mut rng = rand::thread_rng();
    let y: u8 = rng.gen_range(1..100);
    println!("x:{x} y:{y}");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(fibonacci(80), 23416728348467685);
    }
}
