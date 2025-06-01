use std::io;

fn input() -> String {
    let mut entrada : String = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a entrada!");
    entrada.to_string()
}

fn main() {
    let entrada : String = input();
    let n : usize = entrada.trim().parse().expect("msg");

    // valores mutaveis
    let mut min : i32;
    let mut pos : usize = 0; 

    let valores : String = input();
    let valores_arr : Vec<i32> = valores
        .split_whitespace()
        .take(n)
        .map(|s| s.parse::<i32>().expect("msg"))
        .collect();

    min = valores_arr[0];
    for i in 0..n{
        if valores_arr[i] < min{
            min = valores_arr[i];
            pos = i;
        }
    }

    println!("Menor valor: {}", min);
    println!("Posicao: {}", pos);
}