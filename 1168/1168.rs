use std::io;

fn input() -> String {
    let mut entrada : String = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a entrada!");
    entrada.to_string()
}

fn led_count(number: char) -> u32 {
    match number {
        '0' => 6, '1' => 2,
        '2' => 5, '3' => 5,
        '4' => 4, '5' => 5,
        '6' => 6, '7' => 3,
        '8' => 7, '9' => 6,
        _  => 0,
    }
}

fn main(){
    let entrada : String = input();
    let n : usize = entrada.trim().parse().expect("msg");

    for _  in 0..n{
        let entrada : String = input();
        let mut leds : u32 = 0;
        for ch in entrada.chars() {
            leds += led_count(ch)
        }
        println!("{} leds", leds)
    }
}