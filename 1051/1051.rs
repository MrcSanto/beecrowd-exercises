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
    let mut renda : f32 = entrada.trim().parse().expect("msg");

    let salarios_corte : [f32; 3] = [4500.00, 3000.00, 2000.00];
    let aliquotas : [f32; 3] = [0.28, 0.18, 0.08];
    let mut taxa: f32 = 0.00;
    
    for i in 0..3{
        let resto : f32 = renda - salarios_corte[i];
        if resto > 0.00 {
            taxa += resto * aliquotas[i];
            renda -= resto;
        }
    }

    if taxa == 0.00 {
        println!("Isento");
    } else {
        println!("R$ {:.2}", taxa)
    }

}