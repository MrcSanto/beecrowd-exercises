use std::io;

fn main() {
    let mut v1 : String = String::new();
    let mut v2 : String = String::new();
    io::stdin().read_line(&mut v1).expect("msg");
    io::stdin().read_line(&mut v2).expect("msg");

    let v1: i32 = v1.trim().parse().expect("msg");
    let v2: i32 = v2.trim().parse().expect("msg");

    let result : i32 = v1 + v2;
    println!("X = {}", result);
}
