
use std::fmt;
fn main() {

    // ── Data Types ──
    let a: i32 = -42;
    let b: u64 = 100;
    let f: f64 = 3.14159;
    let is_true: bool = true;
    let ch: char = '🦀';
    let tuple: (i32, &str) = (1, "hello");
    let arr: [i32; 3] = [1, 2, 3];
    println!("{} {} {} {} {} {:?} {:?}", a, b, f, is_true, ch, tuple, arr);

    // ── Strings: &str vs String ──
    let slice: &str = "literal";       // borrowed, immutable
    let owned: String = String::from("owned");  // heap, mutable
    let combined = format!("{} - {}", slice, owned);
    println!("{}", combined);

    // ── Ownership ──

    let s1 = String::from("hello");
    let s2 = s1;                        // s1 moved to s2
    // println!("{}", s1);              // ERROR: s1 no longer valid
    let s3 = s2.clone();                // explicit deep copy
    println!("s2={}, s3={}", s2, s3);



}
