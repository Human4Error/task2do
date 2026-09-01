
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


    // ── References & Borrowing ──
    let len = calculate_length(&s3);    // borrow, don't move
    println!("length of '{}': {}", s3, len);

    // ── Control Flow ──
    for i in 0..5 {
    // ── Pattern Matching ──
        match i {
            1 => println!("one"),
            2 | 3 => println!("two or three"),
            _ => println!("something else"),
        }
    }


    // ── Structs & Impl ──
    let user = User { name: String::from("Ali"), age: 25 };
    println!("{}", user.describe());

    // ── Enums ──
    let dir = Direction::North;
    move_player(dir);

    // ── Option<T> ──
    let maybe: Option<i32> = Some(42);
    match maybe {
        Some(v) => println!("got value: {}", v),
        None => println!("got nothing"),
    }
    // shorter way:
    if let Some(v) = maybe {
        println!("if let: {}", v);
    }

        // ── Result<T, E> ──
    match divide(10.00, 2.00) {
        Ok(v) => println!("10 / 2 = {}", v),
        Err(e) => println!("error: {}", e),
    }
    match divide(10.00, 0.00) {
        Ok(v) => println!("10 / 0 = {}", v),
        Err(e) => println!("error: {}", e),
    }

    // ── Vectors ──
    let mut nums: Vec<i32> = Vec::new();
    nums.push(1);
    nums.push(2);
    nums.push(3);
    for n in &nums {
        print!("{} ", n);
    }
    println!();

    // ── HashMap ──
    use std::collections::HashMap;
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Ali", 10);
    scores.insert("Sara", 20);
    if let Some(score) = scores.get("Ali") {
        println!("Ali's score: {}", score);
    }

    // ── Closures ──
    let add_one = |n: i32| -> i32 { n + 1 };
    println!("closure: {}", add_one(5));

    // ── Iterators ──
    let doubled: Vec<i32> = vec![1, 2, 3].iter().map(|&x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    let sum: i32 = vec![1, 2, 3, 4].iter().sum();
    println!("sum: {}", sum);

    // ── Traits ──
    let dog = Dog;
    dog.speak();

}

// ── Traits ──
trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// ── Structs ──
struct User {
    name: String,
    age: u32,
}

impl User {
    fn describe(&self) -> String {
        format!("User {} is {} years old", self.name, self.age)
    }
}

// ── Enums ──
enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::North => println!("moving north"),
        Direction::South => println!("moving south"),
        Direction::East  => println!("moving east"),
        Direction::West  => println!("moving west"),
    }
}