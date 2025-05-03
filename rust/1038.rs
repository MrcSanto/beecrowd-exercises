use std::io;

fn main() {
    let mut entrada : String = String::new();

    io::stdin().read_line(&mut entrada).expect("msg");
    let line : Vec<&str> = entrada.trim().split_whitespace().collect();

    let cod : u32 = line[0].parse().expect("msg");
    let quantidade : f32 = line[1].parse().expect("msg");
    let res : f32;

    if cod == 1 {
        res = 4.00 * quantidade; 
    }
    else if cod == 2 {
        res = 4.50 * quantidade;
    }
    else if cod == 3 {
        res = 5.00 * quantidade;
    }
    else if cod == 4 {
        res = 2.00 * quantidade;
    }
    else {
        res = 1.50 * quantidade;
    }

    println!("Total: R$ {:.2}", res);
}
