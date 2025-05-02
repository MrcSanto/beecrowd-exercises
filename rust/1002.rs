use std::io;

fn main() {
    let mut v1 : String = String::new();
    io::stdin().read_line(&mut v1).expect("msg");

    let v1 : f64 = v1.trim().parse().expect("msg");

    let res : f64 = v1 * v1 * 3.14159;
    println!("A={:.4}", res);
}
