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
    let valores : Vec<usize> = entrada
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("msg"))
        .collect();

    let p: u32 = valores[0] as u32;
    let n: usize = valores[1];

    let altura_canos : String = input();
    let alturas : Vec<u32> = altura_canos
        .split_whitespace()
        .take(n)
        .map(|s| s.parse::<u32>().expect("msg"))
        .collect();

    for i in 1..n{
        let cano_atual : u32 = alturas[i-1]; 
        let cano_a_pular : u32 = alturas[i];

        let diff : u32 = if cano_atual > cano_a_pular {
            cano_atual - cano_a_pular
        } else {
            cano_a_pular - cano_atual
        };

        if diff > p{
            println!("GAME OVER");
            return;
        }
    }
    
    println!("YOU WIN")
}