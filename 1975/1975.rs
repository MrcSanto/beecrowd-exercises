use std::io;
use std::collections::{HashMap, HashSet};


fn input() -> String {
    let mut entrada : String = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a entrada!");
    entrada.trim().to_string()
}

fn main() {
    loop {
        let entrada: Vec<u32> = input()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let (p, a, r) = (entrada[0], entrada[1], entrada[2]);

        if p == 0 && a == 0 && r == 0 {
            break;
        }

        let mut perolas : HashSet<String> = HashSet::new(); // tp o unordered_set c++

        // adicionando valores ao set
        for _ in 0..p{
            let perola : String = input();
            perolas.insert(perola);
        }

        let mut contagem_erros : HashMap<String, u32> = HashMap::new(); // tipo o map do c++
        let mut max_erros : u32 = 0; // var para coletar a qtd máxima de erros

        for _ in 0..a{
            let nome_aluno : String = input();
            let mut qtd : u32 = 0; // quantidade de erros do aluno

            for _ in 0..r{
                let resposta : String = input();
                // verificando se a resposta está no set
                if perolas.contains(&resposta){
                    qtd += 1;
                }
            }
            contagem_erros.insert(nome_aluno, qtd);

            // verificando a maior quantidade de erros (de todos os alunos)
            if qtd > max_erros{
                max_erros = qtd;
            }
        }

        // filtra os alunos que possuem o maior numeros de pérolas
        let mut perdedores : Vec<String> = contagem_erros
            .iter()
            .filter(|(_, v)| **v == max_erros)
            .map(|(k, _)| k.clone())
            .collect();
        perdedores.sort();
        println!("{}", perdedores.join(", "));
    }
}