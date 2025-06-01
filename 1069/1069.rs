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

    for _ in 0..n{
        let mut pilha : Vec<char> = Vec::new();
        let mut diamantes : u32 = 0;

        let areia_diamantes : String = input();
        for c in areia_diamantes.chars(){
            if c == '<' {pilha.push(c);}
            else if c == '>' && !pilha.is_empty() {
                diamantes += 1;
                pilha.pop();
            }
        }

        println!("{}", diamantes)
    }
}