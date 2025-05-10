use std::io;

fn input() -> String {
    let mut entrada : String = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a entrada!");
    entrada.to_string()
}

fn main(){
    let entrada : String = input();
    let p1 : Vec<&str> = entrada.trim().split_whitespace().collect();

    let entrada : String = input();  
    let p2 : Vec<&str> = entrada.trim().split_whitespace().collect();

    let x1: f64 = p1[0].parse().expect("msg");
    let x2: f64 = p2[0].parse().expect("msg");
    let y1: f64 = p1[1].parse().expect("msg");
    let y2: f64 = p2[1].parse().expect("msg");

    // powf(0.5) mesma coisa que extrair a raiz
    let res : f64 = ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).powf(0.5);

    println!("{:.4}", res)
}