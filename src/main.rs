extern crate regex;
use core::{num, slice};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};
use std::{fmt::Debug, ops::Add};

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
    // String
    let str = "hello"; // str 是一个不可变的字符串切片
    let mut s = String::from("hello"); // s 是一个可变的字符串
                                       // push_str() 将字符串切片附加到String
    s.push_str(" ,world!");

    println!("s:{}", s);
    let s2 = s.clone();
    // let s2 = s; // s的所有权已经转移到s2,所以s不能再使用
    println!("s2:{}", s2);
    // 引用,不会获取所有权
    // 切片是引用，所以不会获取所有权
    let len = s.len();
    let hello = &s[0..5]; // &s[..5]
    let world = &s[7..len]; // &s[7..]
    let slice = &s[..]; // &s[0..len]
    let first_word = slice_world(&s);
    // s.clear(); // clear 需要操作 s 的可变引用，而 first_word 是不可变引用，所以不能同时使用
    println!(
        "hello:{},world:{} slice:{} first_world:{}",
        hello, world, slice, first_word
    );
    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[..3];
    assert_eq!(slice, &[1, 2, 3]);
    // 结构体
    let mut user1 = User {
        username: String::from("yy13003"),
        email: String::from("13003@qq.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("13004@qq.com");
    println!("user1:{}", user1.email);
    // 结构体更新语法
    let user2 = build_user(String::from("13005@qq.com"), String::from("yy13005"));
    println!("user2:{}", user2.email);
    let user3 = User {
        email: String::from("13006@qq.com"),
        ..user2
    };
    println!("user2:{} user3:{}", user2.email, user3.email);
    // 元组结构体
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(1920, 1080);
    println!(
        "black:{} white:{} origin:{}x{}",
        black.0, white.0, origin.0, origin.1
    );
    // 单元结构
    let unit = UnitStruct;
    println!("unit:{:?}", unit);
    // 结构体和方法
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "rect area:{} rect1:{} rect2:{}  \nrect can hold rect1:{} \nrect can hold rect2:{}",
        rect.area(),
        rect1.area(),
        rect2.area(),
        rect.can_hold(&rect1),
        rect.can_hold(&rect2),
    );
    // 关联函数
    let sq = Rectangle::square(3);
    println!("sq:{}", sq.area());
    // 枚举
    let four = IpAddrKind::IPV4;
    let six = IpAddrKind::IPV6;
    route(four);
    route(six);
    // match 匹配其他
    let dice_roll = 6;
    match dice_roll {
        3 => println!("You rolled a three!"),
        4 => println!("You rolled a four!"),
        _ => println!("You rolled something else!"),
    }
    // if let 匹配,只匹配一个分支
    let dice_roll = 7;
    if let 7 = dice_roll {
        println!("You rolled a there!");
    } else {
        println!("You rolled something else!");
    }
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v:{}", v[0]);
    let v = vec![1, 2, 3];
    println!("v0:{} v1:{} v2:{}", v[0], v[1], v[2]);
    let two = &v[1];
    println!("two:{}", two);
    let three = v.get(2);
    match three {
        Some(three) => println!("three:{}", three),
        None => println!("None"),
    }
    for i in &v {
        println!("i:{}", i);
    }

    let mut v2 = vec![100, 200, 300];
    for i in &mut v2 {
        *i += 50;
        println!("i:{}", i);
    }
    v2.push(400);
    println!("v2:{}", v2[3]);
    // 字符串
    let s = String::new();
    println!("empty s:{}", s);
    let data = "initial contents";
    println!("data:{}", data);
    let s = data.to_string();
    println!("s1:{}", s);
    let s = "initial contents".to_string();
    println!("s2:{}", s);
    let s = String::from("initial contents");
    println!("s3:{}", s);
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    println!("hello:{}", hello);
    let s = String::from("Start");
    let s2 = "End";
    // let ss = s.add(&s2); // add() 方法获取一个字符串，这里s 失去了所有权，而 s2 传递的是一个引用，所以 s 将失效，s2 仍然有效
    let s3 = s + "_" + &s2;
    // println!("s3:{} s:{} s2{}", s3, s, s2); // s2 仍然有效,因为 s2 是 &str 类型,不是 &String 类型
    println!("s3:{} s2:{}", s3, s2); // s2 仍然有效,因为 s2 是 &str 类型,不是 &String 类型
    let s4 = format!("{}-{}", s2, s3); // format! 宏不会获取任何参数的所有权
    println!("s4:{} s3:{} s2:{}", s4, s3, s2); // 所以 s3和s2 仍然有效
    let hola = String::from("Hola");
    // let a = &hola[0];// 这里会报错，因为 Rust 不允许我们索引 String
    let a = &hola[..1]; // 这里使用字符串切片，非常明确，所以不会报错
    println!("a:{}", a);
    let chars = hola.chars();
    for c in chars {
        println!("c:{}", c);
    }
    let bytes = hola.bytes();
    for b in bytes {
        println!("b:{}", b);
    }
    // 字符串的一些处理方法
    assert_eq!(hola.contains("H"), true);
    let replace_str = hola.replace("H", "h");
    println!("replace_str:{}", replace_str);
    // HashMap
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 15);
    for (k, v) in scores.iter() {
        println!("k:{} v:{}", k, v);
    }
    // 获取值
    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    println!("blue_score:{}", blue_score);
    // 更新值
    scores.insert("Blue", 25);
    println!("scores:{:?}", scores);
    // 仅当键没有对应值时插入
    scores.entry("Yellow").or_insert(50);
    scores.entry("Blue").or_insert(50);
    println!("scores:{:?}", scores);
    // 根据旧值更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map:{:?}", map);
    // 错误和异常
    // panic!("crash and burn"); // panic! 宏会打印一个错误信息，展开并清理调用栈，然后退出程序
    let get_file_result = File::open("hello.txt");
    let get_file = match get_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("File created successfully: {:?}", fc);
                    fc
                }
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            _ => panic!("There was a problem opening the file: {:?}", error),
        },
    };
    let username = read_username_from_file().unwrap();
    let username = read_username_from_file2().unwrap();
    println!("username:{}", username);
    let l = last_char("hello").unwrap();
    println!("l:{}", l);
}

fn read_username_from_file() -> Result<String, Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

fn last_char(s: &str) -> Option<char> {
    s.lines().next()?.chars().last()
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

// 切片单词
fn slice_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // 返回第一个单词也可以写成 &s[0..i]
        }
    }
    return &s[..]; // 返回整个字符串也可以写成 &s[..]
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
// 用户结构体
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
// 构建用户结构体对象
fn build_user(email: String, username: String) -> User {
    User {
        username, // 字段初始化简写语法 username: username
        email,    // 字段初始化简写语法 email: email
        active: true,
        sign_in_count: 1,
    }
}
// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32);
// 单元结构
#[derive(Debug)]
struct UnitStruct;

// 结构体和方法
struct Rectangle {
    width: u32,
    height: u32,
}

// 实现
impl Rectangle {
    // 关联函数,计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数,检查是否可以容纳另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数,创建一个正方形
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
// 枚举
enum IpAddrKind {
    IPV4,
    IPV6,
}
// 枚举1
enum IpAddrKind1 {
    IPV4(String),
    IPV6(String),
}

// 枚举2
enum IpAddrKind2 {
    IPV4(u8, u8, u8, u8),
    IPV6(String),
}
// match 匹配枚举
fn route(ip_type: IpAddrKind) {
    match ip_type {
        IpAddrKind::IPV4 => println!("route: IPV4"),
        IpAddrKind::IPV6 => println!("route: IPV6"),
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(fibonacci(80), 23416728348467685);
    }
}
