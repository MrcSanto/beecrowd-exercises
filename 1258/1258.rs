use std::io;

fn input() -> String {
    let mut entrada : String = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a entrada!");
    entrada.to_string()
}

struct Camiseta {
    nome : String,
    cor : String,
    tamanho : String,
}

fn main() {
    let mut first : bool = true;

    loop{
        let entrada : String = input();
        let n : u32 = entrada.trim().parse().expect("msg");
        if n == 0 {
            break;
        }
        
        if first {
            first = false;
        } else {
            println!();
        }

        let mut camisetas : Vec<Camiseta> = Vec::new();

        for _ in 0..n{
            let nome: String = input().trim().to_string();
            let linha: Vec<String> = input()
                .trim()
                .split_whitespace()
                .map(str::to_string)
                .collect();
            let cor: String = linha[0].clone();
            let tamanho: String = linha[1].clone();

            camisetas.push(Camiseta { nome, cor, tamanho });
        }

        camisetas.sort_by(|a, b| {
            if a.cor != b.cor {
                a.cor.cmp(&b.cor) // cor em ordem crescente
            } else if a.tamanho != b.tamanho {
                b.tamanho.cmp(&a.tamanho) // tamanho em ordem decrescente
            } else {
                a.nome.cmp(&b.nome) // nome em ordem crescente
            }
        });

        for camiseta in camisetas {
            println!("{} {} {}", camiseta.cor, camiseta.tamanho, camiseta.nome);
        }
    }
}