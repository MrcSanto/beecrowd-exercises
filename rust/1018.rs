use std::io;

fn main() {
    let cedules_arr : [u32; 7] = [100, 50, 20, 10, 5, 2, 1];

    let mut value : String = String::new();
    io::stdin().read_line(&mut value).expect("msg");
    print!("{}", value);

    let mut value : u32 = value.trim().parse().expect("msg");
    for cedule in cedules_arr.iter(){
        let quantity : u32 = value / cedule;
        value %= cedule;
        println!("{} nota(s) de R$ {:.2},00", quantity, *cedule);
    }
}
