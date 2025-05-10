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
    let n : usize = entrada.trim().parse().expect("msg");

    let entrada_rpms : String = input();
    let rpms : Vec<u32> = entrada_rpms
        .split_whitespace()
        .take(n)
        .map(|s| s.parse::<u32>().expect("msg"))
        .collect();
    let mut ans: usize = 0;

    for i in 1..n{
        if rpms[i] < rpms[i-1]{
            ans = i + 1;
            break;
        }
    }
    
    println!("{}", ans)
}