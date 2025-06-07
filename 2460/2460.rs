use std::io;
use std::collections::HashMap;


fn input() -> String {
    let mut entrada : String = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a entrada!");
    entrada.to_string()
}

fn main() {
    let mut first : bool = true;

    let n : usize = input().trim().parse().expect("msg");

    let mut fila_ingressos: HashMap<u32, bool> = HashMap::new();

    let pessoas_fila : String = input();
    let pessoas_fila : Vec<u32> = pessoas_fila
        .split_whitespace()
        .take(n)
        .map(|s| s.parse::<u32>().expect("msg"))
        .collect();

    for &pessoa in &pessoas_fila{
        fila_ingressos.insert(pessoa, true);
    } 

    let m : usize = input().trim().parse().expect("msg");
    let removidos : Vec<u32> = input()
        .split_whitespace()
        .take(m)
        .map(|s| s.parse::<u32>().expect("msg"))
        .collect();

    for &rem in &removidos{
        fila_ingressos.insert(rem, false);
    }
       for &pessoa in &pessoas_fila {
            if *fila_ingressos.get(&pessoa).unwrap_or(&false) {
                if !first {
                    print!(" ");
                }
                print!("{}", pessoa);
                first = false;
            }
       }

    println!();
}