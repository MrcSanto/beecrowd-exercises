use std::io;

fn main(){
    let mut entrada : String = String::new();
    io::stdin().read_line(&mut entrada).expect("msg");

    let line : Vec<&str> = entrada.trim().split_whitespace().collect();
    let a : f32 = line[0].parse().expect("msg");
    let b : f32 = line[1].parse().expect("msg");
    let c : f32 = line[2].parse().expect("msg");

    if a < b + c && b < a + c && c < a + b {
        println!("Perimetro = {:.1}", a + b + c);
    }
    else {
        println!("Area = {:.1}", ((a + b) * c) / 2.0);
    }
}