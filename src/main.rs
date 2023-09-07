extern crate regex;
use core::num;

use rand::prelude::*;
use regex::Regex;
use time::OffsetDateTime;

fn main() {
    let single_precision: f32 = 1.23456789012345678901234567890;
    let double_precision: f64 = 1.23456789012345678901234567890;

    println!("Single Precision: {}", single_precision);
    println!("Double Precision: {}", double_precision);
    // 元组
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("a: {} b: {} c: {} d: {}", a, b, c, d);
    println!("0:{} 1:{} 2:{} 3:{}", tuple.0, tuple.1, tuple.2, tuple.3);
    // 数组
    let a = [1, 2, 3, 4, 5];
    println!(
        "a has {} elements,first val:{} second val:{}",
        a.len(),
        a[0],
        a[1]
    );
    // 函数方法
    another_function(24, 'k');
    // 表达式
    expressions();
    // 返回值
    println!(
        "The value of return_value: {} value1: {}",
        return_value(),
        return_value1()
    );
    // 控制流
    condition();
    // 循环
    loop1();
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function value: {} unit_label:{}", x, unit_label);
}

fn expressions() {
    let x = 5;
    let y = {
        let z = 3;
        z + 1
    };
    println!("The values of x:{} y: {}", x, y);
}

fn return_value() -> i32 {
    5 + 3
}

fn return_value1() -> i32 {
    return 666;
}

fn condition() {
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }
    if number != 0 {
        println!("number was three");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3,or 2");
    }
}

fn loop1() {
    let mut counter = 0;
    // loop
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The loop result is {}", result);
    // while
    while counter > 0 {
        println!("while counter:{}", counter);
        counter -= 1;
    }
    // for
    let a = [10, 20, 30, 40, 50];
    for ele in a {
        println!("the val is {ele}");
        counter = ele;
        println!("the counter is {counter}");
    }
    // for range
    for number in (1..4).rev() {
        println!("{} !!!", number);
    }

    'counting_up: loop {
        println!("counting_one {}", counter);
        loop {
            println!("counting_two {}", counter);
            counter += 1;
            if counter == 55 {
                println!("counting_two counter:{}", counter);
                break;
            } else if counter == 60 {
                println!("counting_one counter:{}", counter);
                break 'counting_up;
            }
        }
    }
}

fn main1() {
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
