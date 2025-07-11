use std::io;

fn input() -> String {
    let mut entrada : String = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a entrada!");
    entrada.trim().to_string()
}

fn main() {
    let entrada : String = input();
    let n : usize = entrada.trim().parse().expect("msg");

    for _ in 0..n{
        let input_linha: Vec<u32> = input()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let (r1, r2) = (input_linha[0], input_linha[1]);

        println!("{}", r1 + r2);
    }
}