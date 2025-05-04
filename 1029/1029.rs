use std::io;

fn fibonacci(n : u32, num_calls : &mut u32) -> u32{
    *num_calls += 1;

    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1, num_calls) + fibonacci(n-2, num_calls),
    }
}

fn main(){
    let mut entrada : String = String::new();
    io::stdin().read_line(&mut entrada).expect("msg");

    let t : usize = entrada.trim().parse().expect("msg");

    for _ in 0..t{
        let mut entrada : String = String::new();
        io::stdin().read_line(&mut entrada).expect("msg");

        let x : u32 = entrada.trim().parse().expect("msg");

        let mut num_calls: u32 = 0;
        let res : u32 = fibonacci(x, &mut num_calls);

        println!("fib({}) = {} calls = {}", x, num_calls - 1, res);
    }
}