use time::OffsetDateTime;
use rand::prelude::*;

fn main() {
    let n = 80; // 计算第40个斐波那契数
    println!("Rust Fibonacci({}): {}", n, fibonacci(n));
    let now = OffsetDateTime::now_utc();
    println!("Now: {}", now);
    let local: OffsetDateTime = now.to_offset(now.offset());
    println!("Local: {}", local);
    let x: u8 = random();
    let mut rng=rand::thread_rng();
    let y: u8 = rng.gen_range(1..100);
    println!("x:{x} y:{y}");

}

fn fibonacci(n: u64) -> u64 {
    if n <= 0 {
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
